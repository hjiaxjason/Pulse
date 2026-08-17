use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::io::{BufRead, BufReader, Write};
use std::fs::OpenOptions;

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
    std::fs::create_dir_all(dir);
    Ok(dir.join("log").set_extension("jsonl"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_log_path_returns_ok() {
        let path = log_path();
        assert!(path.is_ok());
    }

    #[test]
    fn test_log_path_ends_with_pulse_log_jsonl() {
        let path = log_path().unwrap();
        assert!(path.ends_with("pulse/log.jsonl");
    }
}


