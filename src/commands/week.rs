use anyhow::Result;
use crate::log;

pub fn execute() -> Result<()> {
    let entries_this_week: Vec<log::LogEntry> = log::entries_this_week(None)?;
    println!("This week's logs:");
    for entry in &entries_this_week {
        println!("{:#?}", entry);
    }
    Ok(())
}
