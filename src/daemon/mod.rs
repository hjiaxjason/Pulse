use anyhow::Result;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub fn install() -> Result<()> {
    #[cfg(target_os = "macos")]
    return macos::install();

    #[cfg(target_os = "linux")]
    return linux::install();

    #[cfg(target_os = "windows")]
    return windows::install();
}

pub fn uninstall() -> Result<()> {
    #[cfg(target_os = "macos")]
    return macos::uninstall();

    #[cfg(target_os = "linux")]
    return linux::uninstall();

    #[cfg(target_os = "windows")]
    return windows::uninstall();
}
