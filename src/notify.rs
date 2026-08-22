/// OS Notification Driver
use notify_rust::{Notification, Timeout};
use anyhow::Result;

fn build_notification_body(nudge:name: &str) -> &'static str {
    let body = match nudge_name {
        "water" => "It is time for your water break!",
        "stretch" => "It is time for your stretch break!",
        other => {
            eprintln!("Warning: unknown nudge type '{other}', firing generic notification");
            "Time for your break!"
        }
    }
}


fn fire(nudge_name: &str) -> Result<()> {
    let body = build_notification_body(nudge_name);

    Notification::new() 
        .summary("Pulse Notification")
        .body(body)
        .icon("dialog-information")
        .appname("Pulse")
        .timeout(Timeout::Never)
        .show()?;

    Ok(())
}
        
