/// Persisted nudge state and state managed that is written to the disk
use std::fs::File;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use chrono::{Duration, DateTime, Utc};
use crate::config;
use serde::{Serialize, Deserialize};
use std::io::{BufReader, BufWriter};
use anyhow::{Context, Result};

#[derive(Default, Serialize, Deserialize)]
pub struct AppState {
    pub nudges: HashMap<String, DateTime<Utc>>,   // key: "water" || "stretch" || "event::meeting", value: some Utc timestamp
    pub paused_at: Option<DateTime<Utc>>,
}

impl AppState {
    pub fn is_due(&self, nudge_name: &str, interval: Duration) -> bool {
        match self.nudges.get(nudge_name) {
            Some(last) => Utc::now() - *last >= interval,
            None => true,
        }
    }

    pub fn mark_fired(&mut self, nudge_name: &str) -> Result<()> {
        self.nudges.insert(String::from(nudge_name), Utc::now());
        Ok(())
    }

    pub fn resume(&mut self) {
        let pause_duration = Utc::now() - self.paused_at;
        
        for last_fired in self.nudges.values_mut() {
            *last_fired += pause_duration;
        }

        paused_at = None;
    }

    pub fn load(path_override: Option<&Path>) -> Result<Self> {
        let path = state_path(path_override)?;

        if !path.exists() {
            return Ok(AppState {
                nudges: HashMap::new(),
            });
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let loaded_nudges: HashMap<String, DateTime<Utc>> = serde_json::from_reader(reader)?;

        Ok(AppState {
            nudges: loaded_nudges,
        })
    }

    pub fn save(&self, path_override: Option<&Path>) -> Result<()> {
        let dir = state_path(path_override)?;
        let file = File::create(dir)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &self.nudges)?;

        Ok(())
    }
}

fn state_path(path_override: Option<&Path>) -> Result<PathBuf> { // path_override is purely for unit testing
    if let Some(p) = path_override {
        return Ok(p.to_path_buf());
    }

    let dir = dirs::data_local_dir()
        .context("could not determine local data directory")?
        .join("pulse");
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join("state.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_path_returns_ok() {
        let path = state_path(None);
        assert!(path.is_ok());
    }

    #[test]
    fn test_state_path_ends_with_pulse_state_jsonl() {
        let path = state_path(None).unwrap();
        assert!(path.ends_with("pulse/state.json"));
    }

    #[test]
    fn test_save_returns_ok() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();

        let state = AppState {
            nudges: HashMap::from([(String::from("water"), Utc::now()), (String::from("stretch"), Utc::now())]),
        };
        
        let result = state.save(Some(temp_file.path()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_save_updates_state_json() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let now = Utc::now();

        // First save old state
        let mut state = AppState {
            nudges: HashMap::from([(String::from("water"), now), (String::from("stretch"), now)]),
        };
        let _ = state.save(Some(temp_file.path()));

        // Update with new state
        let mut new_state = AppState {
            nudges: HashMap::from([(String::from("water"), now-Duration::hours(2)), (String::from("stretch"), now-Duration::hours(2))]),
        };
        let _ = new_state.save(Some(temp_file.path()));

        let result_state = AppState::load(Some(temp_file.path())).unwrap();

        assert_eq!(result_state.nudges.get("water"), Some(&(now-Duration::hours(2))));
        assert_eq!(result_state.nudges.get("stretch"), Some(&(now-Duration::hours(2))));
    }

    #[test]
    fn test_load_returns_ok() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let mut state = AppState {
            nudges: HashMap::from([(String::from("water"), Utc::now()), (String::from("stretch"), Utc::now())]),
        };
        let _ = state.save(Some(temp_file.path()));

        let result = AppState::load(None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mark_fired_returns_ok() {
        let now = Utc::now();
        let mut state = AppState {
            nudges: HashMap::from([(String::from("water"), now-Duration::hours(2)), (String::from("stretch"), now-Duration::hours(2))]),
        };
        
        let result = state.mark_fired("water");
        assert!(result.is_ok());
    }

    #[test]
    fn test_mark_fired_correctly_updates_timestamp() {
        let now = Utc::now();
        let mut state = AppState {
            nudges: HashMap::from([(String::from("water"), now-Duration::hours(2)), (String::from("stretch"), now-Duration::hours(2))]),
        };

        let _ = state.mark_fired("water");
        assert!(state.nudges.get("water") > Some(&(now-Duration::hours(2))));
    }

    #[test]
    fn test_is_due_return_correct_bool() {
        let now = Utc::now();
        let mut state = AppState {
            nudges: HashMap::from([(String::from("water"), now-Duration::hours(1)), (String::from("stretch"), now-Duration::hours(1))]),
        };

        let due_water = state.is_due("water", Duration::minutes(59));
        let due_stretch = state.is_due("stretch", Duration::minutes(120));
        assert!(due_water, "Expected due_water to be true, but it was false");
        assert!(!due_stretch, "Expected due_stretch to be false, but it was true");
    }
}
