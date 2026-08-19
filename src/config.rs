use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_nudges")]
    pub nudge: Vec<Nudge>,
}

#[derive(Deserialize, Debug, Clone)]
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

fn config_path() -> Result<PathBuf> {
    // ~/.config/pulse/config.toml
    let dir = dirs::config_dir()
        .context("could not determine config directory")?
        .join("pulse");
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join("config.toml"))
}

pub fn load() -> Result<Config> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(Config { nudge: default_nudges() });
    }
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
