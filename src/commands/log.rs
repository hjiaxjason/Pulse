use anyhow::Result;
use crate::log;

pub fn execute(message: String, kind: Option<String>) -> Result<()> {
    let kind = kind.unwrap_or_else(|| "work".to_string());
    log::append_entry(&message, &kind, "cli", None)?;
    println!("logged: {message}");
    Ok(())
}


