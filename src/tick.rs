
pub fn run_tick() -> Result<()> {
    let mut app_state = state::load()?;
    let cfg = config::load()?;
    for nudge in &cfg.nudges {
        if app_state.is_due(&nudge.name, nudge.interval) {
            notify::fire(&nudge.name)?;
            log::append_entry(&nudge.name, &format!("{} nudge", nudge.name), "nudge")?;
            app_state.mark_fired(&nudge.name);
        }
    }
    state::save(&app_state)?;
    Ok(())
}
