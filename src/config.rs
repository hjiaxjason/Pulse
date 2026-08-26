/// Reads and parses user's settings from config dir and defines what nudges exist and how often
/// they should run.
use anyhow::{anyhow, Context, Result};
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::fs::File;
use chrono::TimeDelta;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_nudges")]
    pub nudges: Vec<Nudge>,
    #[serde(default)]
    pub events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Nudge {
    pub name: String,
    #[serde(default = "default_interval")]
    pub interval: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub name: String,
    pub event_type: EventType,
    pub start_time: String,
    pub duration: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    OneTime,
    Recurring,
}


fn default_interval() -> String {
    "1h".to_string()
}

fn default_nudges() -> Vec<Nudge> {
    vec![]
}

impl Default for Config {
    fn default() -> Self {
        Self {
            nudges: default_nudges(),
            events: Vec::new(),
        }
    }
}

impl Config {
    fn modify_and_save<F>(path_override: Option<&Path>, mutate: F) -> Result<()>
        where
            F: FnOnce(&mut Config),
        {
            let mut cfg = load(path_override)?;

            mutate(&mut cfg);

            let path = config_path(path_override)?;
            let toml_str = toml::to_string_pretty(&cfg)?;
            std::fs::write(path, toml_str)?;

            Ok(())
        }
}

pub fn save_nudges(nudges: Vec<Nudge>, path_override: Option<&Path>) -> Result<()> {
    Config::modify_and_save(path_override, |cfg| {
        cfg.nudges = nudges;
    })
}

pub fn add_event(event: Event, path_override: Option<&Path>) -> Result<()> {
    Config::modify_and_save(path_override, |cfg| {
        cfg.events.push(event);
    })
}

fn config_path(path_override: Option<&Path>) -> Result<PathBuf> {
    if let Some(p) = path_override {
        return Ok(p.to_path_buf());
    }

    // ~/.config/pulse/config.toml
    let dir = dirs::config_dir()
        .context("could not determine config directory")?
        .join("pulse");
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join("config.toml"))
}

pub fn load(path_override: Option<&Path>) -> Result<Config> {
    let path = config_path(path_override)?;
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn parse_duration(duration: &str) -> Result<TimeDelta> {
    let duration = duration.trim();
    
    let (suffix, builder): (&str, fn(i64) -> TimeDelta) = if duration.ends_with("mins") {
        ("mins", TimeDelta::minutes)
    } else if duration.ends_with("min") {
        ("min", TimeDelta::minutes)
    } else if duration.ends_with("hr") {
        ("hr", TimeDelta::hours)
    } else if duration.ends_with('h') {
        ("h", TimeDelta::hours)
    } else if duration.ends_with('m') {
        ("m", TimeDelta::minutes)
    } else {
        return Err(anyhow!("Unsupported duration unit in: '{duration}'"));
    };

    let num_str = duration.strip_suffix(suffix).unwrap_or("").trim();
    let num = num_str
        .parse::<i64>()
        .context(format!("Failed to parse number in '{duration}'"))?;

    Ok(builder(num))
}
            

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_path_returns_ok() {
        let result = config_path(None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_path_ends_with_pulse_conflig_toml() {
        let result = config_path(None).unwrap();
        assert!(result.ends_with("pulse/config.toml"));
    }

    #[test]
    fn test_save_returns_ok() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();

        let nudges = vec![
            Nudge {
                name: String::from("water"),
                interval: String::from("1h"),
            },
            Nudge {
                name: String::from("stretch"),
                interval: String::from("30m"),
            }
        ];
        
        let result = save_nudges(nudges, Some(temp_file.path()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_save_updates_config_toml_and_load_works() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();

        let nudges = vec![
            Nudge {
                name: String::from("water"),
                interval: String::from("90m"),
            },
            Nudge {
                name: String::from("stretch"),
                interval: String::from("30m"),
            }
        ];
        let _ = save_nudges(nudges, Some(temp_file.path()));
        
        let cfg = load(Some(temp_file.path())).unwrap();
        assert_eq!(cfg.nudges.get(0).unwrap().name, String::from("water"));
        assert_eq!(cfg.nudges.get(0).unwrap().interval, String::from("90m"));
        assert_eq!(cfg.nudges.get(1).unwrap().name, String::from("stretch"));
        assert_eq!(cfg.nudges.get(1).unwrap().interval, String::from("30m"));
    }

    #[test]
    fn test_load_default_works() {
        let cfg = load(None).unwrap();
        assert!(cfg.nudges.is_empty());
    }
}






