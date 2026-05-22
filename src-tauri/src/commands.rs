//! Tauri commands — the IPC surface the Vue frontend calls via invoke().

use crate::{models, protocol};
use hidapi::HidApi;
use std::collections::HashSet;
use std::sync::Mutex;

pub struct AppState {
    pub api: Mutex<HidApi>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSummary {
    pub path: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub product: Option<String>,
    pub manufacturer: Option<String>,
    pub model_name: Option<String>,
}

/// List connected keyboards on our HID usage page (or any known model).
#[tauri::command]
pub fn list_devices(state: tauri::State<AppState>) -> Result<Vec<DeviceSummary>, String> {
    let mut api = state.api.lock().map_err(|e| e.to_string())?;
    api.refresh_devices().map_err(|e| e.to_string())?;

    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for d in api.device_list() {
        let vid = d.vendor_id();
        let pid = d.product_id();
        let model = models::find(vid, pid);
        // Only target the vendor-defined custom HID interface (UsagePage 0xFF67 or 0xFF68)
        let is_custom_hid = d.usage_page() == 0xFF67 || d.usage_page() == 0xFF68;
        if !is_custom_hid {
            continue;
        }
        let path = d.path().to_string_lossy().into_owned();
        if !seen.insert(path.clone()) {
            continue;
        }
        out.push(DeviceSummary {
            path,
            vendor_id: vid,
            product_id: pid,
            product: d.product_string().map(str::to_owned),
            manufacturer: d.manufacturer_string().map(str::to_owned),
            model_name: model,
        });
    }
    Ok(out)
}

/// Read device info from a keyboard by HID path, via our Rust protocol port.
#[tauri::command]
pub fn get_device_info(
    state: tauri::State<AppState>,
    path: String,
) -> Result<protocol::DeviceInfo, String> {
    let api = state.api.lock().map_err(|e| e.to_string())?;
    let cpath = std::ffi::CString::new(path).map_err(|e| e.to_string())?;
    let device = api.open_path(&cpath).map_err(|e| format!("open failed: {e}"))?;
    protocol::get_device_info(&device)
}

/// Read game mode / performance settings.
#[tauri::command]
pub fn get_game_mode(
    state: tauri::State<AppState>,
    path: String,
    frame_version: u8,
) -> Result<protocol::GameMode, String> {
    let api = state.api.lock().map_err(|e| e.to_string())?;
    let cpath = std::ffi::CString::new(path).map_err(|e| e.to_string())?;
    let device = api.open_path(&cpath).map_err(|e| format!("open failed: {e}"))?;
    protocol::get_game_mode(&device, frame_version)
}

/// Save game mode / performance settings.
#[tauri::command]
pub fn set_game_mode(
    state: tauri::State<AppState>,
    path: String,
    config: protocol::GameMode,
    frame_version: u8,
) -> Result<(), String> {
    let api = state.api.lock().map_err(|e| e.to_string())?;
    let cpath = std::ffi::CString::new(path).map_err(|e| e.to_string())?;
    let device = api.open_path(&cpath).map_err(|e| format!("open failed: {e}"))?;
    protocol::set_game_mode(&device, &config, frame_version)
}

/// Read current lighting / LED effect settings.
#[tauri::command]
pub fn get_led_effect(
    state: tauri::State<AppState>,
    path: String,
    frame_version: u8,
) -> Result<protocol::LedEffect, String> {
    let api = state.api.lock().map_err(|e| e.to_string())?;
    let cpath = std::ffi::CString::new(path).map_err(|e| e.to_string())?;
    let device = api.open_path(&cpath).map_err(|e| format!("open failed: {e}"))?;
    protocol::get_led_effect(&device, frame_version)
}

/// Save lighting / LED effect settings.
#[tauri::command]
pub fn set_led_effect(
    state: tauri::State<AppState>,
    path: String,
    config: protocol::LedEffect,
) -> Result<(), String> {
    let api = state.api.lock().map_err(|e| e.to_string())?;
    let cpath = std::ffi::CString::new(path).map_err(|e| e.to_string())?;
    let device = api.open_path(&cpath).map_err(|e| format!("open failed: {e}"))?;
    protocol::set_led_effect(&device, &config)
}

/// Execute a factory reset or calibration clear.
#[tauri::command]
pub fn factory_reset(
    state: tauri::State<AppState>,
    path: String,
    reset_type: u8,
) -> Result<(), String> {
    let api = state.api.lock().map_err(|e| e.to_string())?;
    let cpath = std::ffi::CString::new(path).map_err(|e| e.to_string())?;
    let device = api.open_path(&cpath).map_err(|e| format!("open failed: {e}"))?;
    protocol::factory_reset(&device, reset_type)
}

