use notify_rust::{Notification, Timeout};
use std::error::Error;

fn fire(nudge_name: &str) -> Result<()> {
    let body = match nudge_name {
        "water" => "It is time for your water break!",
        "stretch" => "It is time for you stretch break!",
        other => {
            eprintln!("Warning: unknown nudge type '{other}', firing generic notification");
            "Time for your break!"
        }
    };

    Notification::new() {
        .summary("Pulse Notification")
        .body(body)
        .icon("dialog-information")
        .appname("Pulse")
        .timeout(Timeout::Never)
        .show()?;

    Ok(())
}
        
