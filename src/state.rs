use std::fs::File;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use chrono::{DateTime};
use crate::config;
use serde::{Serialize, Deserialize, Default};
use std::io::{BufReader, BufWriter};
use anyhow::{Context, Result};

#[derive(Default, Serialize, Deserialize)]
pub struct AppState {
    pub nudges: HashMap<String, DateTime<Utc>>,   // key: "water" || "stretch", value: some Utc for last
                                              // nudge time 
}

impl AppState {
    pub fn is_due(&self, nudge_name: &str, interval: chrono::Duration) -> bool {
        match self.nudges.get(nudge_name) {
            Some(last) => Utc::now() - *last >= interval,
            None => true,
        }
    }

    pub fn mark_fired(&self, nudge_name: &str) -> Result<()> {
        self.nudges.insert(String::from(nudge_name), Utc::now());
        Ok(())
    }

    pub fn load(path_override: Option<&Path>) -> Result<()> {
        let path = state_path(path_override)?;
        if !path.exists() {
            return Ok(vec![]);
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let loaded_nudges: HashMap<String, DateTime<Utc>> = serde_json::from_reader(reader)?;

        self.nudges = loaded_nudges;

        Ok(())
    }

    pub fn save(&self, path_override: Option<&Path>) -> Result<()> {
        let dir = state_path(path_override)?;
        let file = File::create(dir)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &self.nudges)?;

        Ok(())
    }
}

fn state_path(path_override: Option<&Path>) -> Result<PathBuf> { // path_override is purely for unit testing, set to
    if let Some(p) = path_override {
        return Ok(p.to_path_buf());
    {

    let dir = dirs::data_local_dir()
        .context("could not determine local data directory")?
        .join("pulse");
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join("state.json"))
}

