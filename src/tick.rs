/// Scheduler for nudges
use anyhow::Result;
use chrono::Utc;
use crate::{config, notify, log, state};

pub fn run_tick() -> Result<()> {
    let mut app_state = state::AppState::load(None)?; // Correct
    let cfg = config::load(None)?; // Correct
    for nudge in &cfg.nudges { // Correct
        let interval_delta = config::parse_duration(&nudge.interval)?;
        if app_state.is_due(&nudge.name, interval_delta) { // Correct
            notify::fire(&nudge.name)?; // Correct
            log::append_entry(&nudge.name, &format!("{} nudge", nudge.name), "nudge", None)?;
            app_state.mark_fired(&nudge.name); // Correct
        }
    }
    app_state.save(None)?; // Correct
    Ok(())
}
