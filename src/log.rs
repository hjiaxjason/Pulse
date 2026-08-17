use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::io::{BufRead, BufReader, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub message: String,
    pub kind: String, 
}

fn log_path() -> Result<PathBuf> {
    let dir = dirs::data_local_dir()
        .context("Failed to find data local dir")?
        .join("pulse");
    let _ = std::fs::create_dir_all(&dir);

    let mut path = dir.join("log");
    path.set_extension("jsonl");
    Ok(path)
}

fn append_entry(message: &str, kind: &str) -> Result<()> {
    // Create entry
    let entry = LogEntry {
        timestamp: Utc::now(),
        message: message.to_string(),
        kind: kind.to_string(),
    };

    let json_string = serde_json::to_string(&entry)?;

    // Open file in append and create-if-missing mode
    let dir = log_path()?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir)?;

    writeln!(file, "{}", json_string)?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_path_returns_ok() {
        let path = log_path();
        assert!(path.is_ok());
    }

    #[test]
    fn test_log_path_ends_with_pulse_log_jsonl() {
        let path = log_path().unwrap();
        assert!(path.ends_with("pulse/log.jsonl"));
    }

    #[test]
    fn test_append_entry_returns_ok() {
        assert!(append_entry("Finish debugging PR#45", "work").is_ok());
    }

}


