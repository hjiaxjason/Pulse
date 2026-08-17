use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::io::{BufRead, BufReader, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub message: String,
    pub kind: String, 
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

fn append_entry(message: &str, kind: &str, path_override: Option<&Path>) -> Result<()> {
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

fn entries_today(path_override: Option<&Path>) -> Result<Vec<LogEntry>> {
    let path = log_path(path_override)?;
    if !path.exists() {
        return Ok(vec![]);
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let stream = serde_json::Deserializer::from_reader(reader).into_iter::<LogEntry>();
    let cutoff = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_sec() - 24 * 60 * 60;
    let entries: Vec<LogEntry> = stream
        .filter_map(|result| match result {
            Ok(entry) => Some(entry),
            Err(e) => {
                eprintln!("Warning: Skipping corrupted log entry: {}", e);
                None
            }
        })
        .filter(|entry| entry.timestamp >= cutoff)
        .collect();

    Ok(entries)
}

fn entries_this_week(path_override: Option<&Path>) -> Result<Vec<LogEntry>> {
    let path = log_path(path_override)?;
    if !path.exists() {
        return Ok(vec![]);
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let stream = serde_json::Deserializer::from_reader(reader).into_iter::<LogEntry>();
    let cutoff = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_sec() - 7 * 24 * 60 * 60;
    let entries: Vec<LogEntry> = stream
        .filter_map(|result| match result {
            Ok(entry) => Some(entry),
            Err(e) => {
                eprintln!("Warning: Skipping corrupted log entry: {}", e);
                None
            }
        })
        .filter(|entry| entry.timestamp >= cutoff)
        .collect();

    Ok(entries)
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
        
        let result = append_entry("test message", "test", Some(temp_file.path()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_append_entry_updates_log() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let _ = append_entry("test message 1", "test", Some(temp_file.path()));
        let binding = read_all(Some(temp_file.path())).unwrap();
        let appended = binding.get(0);

        assert_eq!(appended.unwrap().message, String::from("test message 1"));
        assert_eq!(appended.unwrap().kind, String::from("test"));
    }

    #[test]
    fn test_read_all_returns_ok() {
        let entries = read_all(None);
        assert!(entries.is_ok());
    }

    #[test]
    fn test_read_all_works_for_multiple_entries() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let _ = append_entry("test message 1", "test", Some(temp_file.path()));
        let _ = append_entry("test message 2", "test", Some(temp_file.path()));

        let entries = read_all(Some(temp_file.path())).unwrap();
        assert_eq!(entries.get(0).unwrap().message, String::from("test message 1"));
        assert_eq!(entries.get(0).unwrap().kind, String::from("test"));
        assert_eq!(entries.get(1).unwrap().message, String::from("test message 2"));
        assert_eq!(entries.get(1).unwrap().kind, String::from("test"));
    }
}


