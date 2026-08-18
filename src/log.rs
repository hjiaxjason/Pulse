use anyhow::{Context, Result};
use chrono::{Duration, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::io::{BufRead, BufReader, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub message: String,
    pub kind: String, 
    pub source: String, // "cli" | "mcp" | "nudge"
}

fn log_path(override_path: Option<&Path>) -> Result<PathBuf> {
    if let Some(p) = override_path {
        return Ok(p.to_path_buf());
    }

    let dir = dirs::data_local_dir()
        .context("Failed to find data local dir")?
        .join("pulse");
    let _ = std::fs::create_dir_all(&dir);

    let mut path = dir.join("log");
    path.set_extension("jsonl");
    Ok(path)
}

pub fn append_entry(message: &str, kind: &str, source: &str, path_override: Option<&Path>) -> Result<()> {
    // Open file in append and create-if-missing mode
    let dir = log_path(path_override)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir)?;

    // Create entry
    let entry = LogEntry {
        timestamp: Utc::now(),
        message: message.to_string(),
        kind: kind.to_string(),
        source: source.to_string(),
    };

    let json_string = serde_json::to_string(&entry)?;

    writeln!(file, "{}", json_string)?;

    Ok(())
}

fn read_all(path_override: Option<&Path>) -> Result<Vec<LogEntry>> {
    let path = log_path(path_override)?;
    if !path.exists() {
        return Ok(vec![]);
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let stream = serde_json::Deserializer::from_reader(reader).into_iter::<LogEntry>();

    let entries: Vec<LogEntry> = stream
        .filter_map(|result| match result {
            Ok(entry) => Some(entry),
            Err(e) => {
                eprintln!("Warning: Skipping corrupted log entry: {}", e);
                None
            }
        })
        .collect();

    Ok(entries)
}

fn entries_since(cutoff: DateTime<Utc>, path_override: Option<&Path>) -> Result<Vec<LogEntry>> {
    let path = log_path(path_override)?;
    if !path.exists() {
        return Ok(vec![]);
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let stream = serde_json::Deserializer::from_reader(reader).into_iter::<LogEntry>();

    Ok(filter_logs_by_cutoff(stream, cutoff))
}

pub fn entries_today(path_override: Option<&Path>) -> Result<Vec<LogEntry>> {
    entries_since(Utc::now() - Duration::hours(24), path_override)
}

pub fn entries_this_week(path_override: Option<&Path>) -> Result<Vec<LogEntry>> {
    entries_since(Utc::now() - Duration::days(7), path_override)
}

pub fn filter_logs_by_cutoff(entries_stream: impl Iterator<Item = Result<LogEntry, serde_json::Error>>, cutoff: DateTime<Utc>) -> Vec<LogEntry> {
    entries_stream
        .filter_map(|result| match result {
            Ok(entry) => Some(entry),
            Err(e) => {
                eprintln!("Warning: Skipping corrupted log entry: {}", e);
                None
            }
        })
        .filter(|entry| entry.timestamp >= cutoff)
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_path_returns_ok() {
        let path = log_path(None);
        assert!(path.is_ok());
    }

    #[test]
    fn test_log_path_ends_with_pulse_log_jsonl() {
        let path = log_path(None).unwrap();
        assert!(path.ends_with("pulse/log.jsonl"));
    }

    #[test]
    fn test_append_entry_returns_ok() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        
        let result = append_entry("test message", "test", "cli", Some(temp_file.path()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_append_entry_updates_log() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let _ = append_entry("test message 1", "test", "cli", Some(temp_file.path()));
        let binding = read_all(Some(temp_file.path())).unwrap();
        let appended = binding.get(0);

        assert_eq!(appended.unwrap().message, String::from("test message 1"));
        assert_eq!(appended.unwrap().kind, String::from("test"));
        assert_eq!(appended.unwrap().source, String::from("cli"));
    }

    #[test]
    fn test_read_all_returns_ok() {
        let entries = read_all(None);
        assert!(entries.is_ok());
    }

    #[test]
    fn test_read_all_works_for_multiple_entries() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let _ = append_entry("test message 1", "test", "cli", Some(temp_file.path()));
        let _ = append_entry("test message 2", "test", "cli", Some(temp_file.path()));

        let entries = read_all(Some(temp_file.path())).unwrap();
        assert_eq!(entries.get(0).unwrap().message, String::from("test message 1"));
        assert_eq!(entries.get(0).unwrap().kind, String::from("test"));
        assert_eq!(entries.get(0).unwrap().source, String::from("cli"));
        assert_eq!(entries.get(1).unwrap().message, String::from("test message 2"));
        assert_eq!(entries.get(1).unwrap().kind, String::from("test"));
        assert_eq!(entries.get(1).unwrap().source, String::from("cli"));
    }

    #[test]
    fn test_chrono_timestamp_filtering() {
        let now = Utc::now();
        let cutoff = now - Duration::days(7);

        let mock_entries = vec![
            // Both entries within cutoff, should keep
            Ok(LogEntry { timestamp: now, message: String::from("test message 1"), kind: String::from("test"), source: String::from("cli") }),
            Ok(LogEntry { timestamp: now - Duration::days(3), message: String::from("test message 2"), kind: String::from("test"), source: String::from("cli") }),
            
            // Exactly on cutoff boundary, should keep
            Ok(LogEntry { timestamp: cutoff, message: String::from("test message 3"), kind: String::from("test"), source: String::from("cli") }),

            // Just outside cutoff by an hour, should filter out
            Ok(LogEntry { timestamp: cutoff - Duration::hours(1), message: String::from("test message 4"), kind: String::from("test"), source: String::from("cli") }),

            // 12 days ago, filter out
            Ok(LogEntry { timestamp: now - Duration::days(12), message: String::from("test message 5"), kind: String::from("test"), source: String::from("cli") }),

            // Corrupted log line simulation, should filter out
            Err(serde_json::from_str::<LogEntry>("Malformed JSON string").unwrap_err()),
        ];
        
        let stream = mock_entries.into_iter();
        let result = filter_logs_by_cutoff(stream, cutoff);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].message, String::from("test message 1"));
        assert_eq!(result[1].message, String::from("test message 2"));
        assert_eq!(result[2].message, String::from("test message 3"));
    }
}


