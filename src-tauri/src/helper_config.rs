//! Shared configuration between the GUI and the background helper daemon (`ajazz-helperd`).
//!
//! The GUI writes this JSON file; the helper polls it and acts on `mode`. Using a file (rather
//! than a socket) keeps the two processes decoupled — the helper survives the GUI being closed,
//! which is the whole point of the background daemon.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// What the background helper should be doing.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HelperConfig {
    /// "off" | "screen" | "gif"
    pub mode: String,
    /// HID path of the target keyboard; if None the helper picks the first vendor device.
    pub device_path: Option<String>,
    /// Path to the GIF file to loop when `mode == "gif"`.
    pub gif_path: Option<String>,
    /// Capture / playback cadence ceiling, frames per second.
    pub fps: u32,
}

impl Default for HelperConfig {
    fn default() -> Self {
        Self {
            mode: "off".to_string(),
            device_path: None,
            gif_path: None,
            fps: 30,
        }
    }
}

/// Directory holding the helper config: `%APPDATA%/ajazz-driver` on Windows,
/// `$XDG_CONFIG_HOME|$HOME/.config/ajazz-driver` elsewhere.
pub fn config_dir() -> PathBuf {
    #[cfg(windows)]
    let base = std::env::var_os("APPDATA").map(PathBuf::from);
    #[cfg(not(windows))]
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")));

    base.unwrap_or_else(|| PathBuf::from(".")).join("ajazz-driver")
}

pub fn config_path() -> PathBuf {
    config_dir().join("helper.json")
}

/// Load the config, falling back to defaults if missing or unparsable.
pub fn load() -> HelperConfig {
    std::fs::read_to_string(config_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

/// Persist the config (creates the directory if needed).
pub fn save(cfg: &HelperConfig) -> std::io::Result<()> {
    std::fs::create_dir_all(config_dir())?;
    let json = serde_json::to_string_pretty(cfg).map_err(std::io::Error::other)?;
    std::fs::write(config_path(), json)
}
