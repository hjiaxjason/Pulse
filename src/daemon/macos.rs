use anyhow::Result;
use anyhow::Context;
use std::fs;
use std::path::PathBuf;

fn plist_path() -> Result<PathBuf> {
    let dir = dirs::home_dir()
        .context("could not determine home directory")?
        .join("Library/LaunchAgents");

    std::fs::create_dir_all(&dir)?;
    Ok(dir.join("com.pulse.tick.plist"))
}

fn generate_plist() -> Result<String> {
    let binary_path = std::env::current_exe()?
        .to_str()
        .context("binary path is not valid UTF-8")?
        .to_string();

    let interval_seconds = 60;

    let plist = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.pulse.tick</string>
    <key>ProgramArguments</key>
    <array>
        <string>{binary_path}</string>
        <string>tick</string>
    </array>
    <key>StartInterval</key>
    <integer>{interval_seconds}</integer>
    <key>RunAtLoad</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/pulse-tick.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/pulse-tick.err</string>
</dict>
</plist>
"#
    );

    Ok(plist)
}

pub fn install() -> Result<()> {
    let plist: String = generate_plist()?;
    let path = plist_path()?;
    
    fs::write(path, plist)?;

    Ok(())
}

pub fn uninstall() -> Result<()> {
    let path = plist_path()?;

    fs::remove_file(path)?;

    Ok(())
}
