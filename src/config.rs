/// Reads and parses user's settings from config dir and defines what nudges exist and how often
/// they should run.
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_nudges")]
    pub nudge: Vec<Nudge>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Nudge {
    pub name: String,
    #[serde(default = "default_interval")]
    pub interval: String,
}

fn default_interval() -> String {
    "45m".to_string()
}

fn default_nudges() -> Vec<Nudge> {
    vec![
        Nudge { name: "water".to_string(), interval: "1hr".to_string() },
        Nudge { name: "stretch".to_string(), interval: "30m".to_string() },
    ]
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
        return Ok(Config { nudge: default_nudges() });
    }
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn save(nudges: Vec<Nudge>, path_override: Option<&Path>) -> Result<()> {
    let cfg = Config { nudge: nudges };

    let path = config_path(path_override)?;
    let toml_str = toml::to_string_pretty(&cfg)?;

    std::fs::write(path, toml_str)?;

    Ok(())
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
        
        let result = save(nudges, Some(temp_file.path()));
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
        let _ = save(nudges, Some(temp_file.path()));
        
        let cfg = load(Some(temp_file.path())).unwrap();
        assert_eq!(cfg.nudge.get(0).unwrap().name, String::from("water"));
        assert_eq!(cfg.nudge.get(0).unwrap().interval, String::from("90m"));
        assert_eq!(cfg.nudge.get(1).unwrap().name, String::from("stretch"));
        assert_eq!(cfg.nudge.get(1).unwrap().interval, String::from("30m"));
    }

    #[test]
    fn test_load_default_works() {
        let cfg = load(None).unwrap();
        assert_eq!(cfg.nudge.get(0).unwrap().name, String::from("water"));
        assert_eq!(cfg.nudge.get(0).unwrap().interval, String::from("1h"));
        assert_eq!(cfg.nudge.get(1).unwrap().name, String::from("stretch"));
        assert_eq!(cfg.nudge.get(1).unwrap().interval, String::from("30m"));
    }
}






