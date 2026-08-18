use anyhow::Result;
use crate::log;

pub fn execute() -> Result<()> {
    let entries_today: Vec<log::LogEntry> = log::entries_today(None)?;
    println!("Todays logs:");
    for entry in &entries_today {
        println!("{:#?}", entry);
    }
    Ok(())
}


