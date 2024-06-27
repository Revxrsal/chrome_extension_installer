use std::io;

#[cfg(target_os = "windows")]
use windows as platform;

#[cfg(target_os = "windows")]
pub(crate) mod windows;

#[cfg(target_os = "macos")]
use macos as platform;

#[cfg(target_os = "macos")]
pub(crate) mod macos;

#[cfg(target_os = "linux")]
use linux as platform;

#[cfg(target_os = "linux")]
pub(crate) mod linux;

pub fn install_extension(extension_id: &str) -> io::Result<()> {
    platform::install_extension(extension_id)
}
