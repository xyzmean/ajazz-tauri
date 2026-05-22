//! Tauri commands — the IPC surface the Vue frontend calls via invoke().

use crate::{models, protocol};
use hidapi::HidApi;
use std::collections::HashSet;
use std::sync::Mutex;

pub struct AppState {
    pub api: Mutex<HidApi>,
    pub cached_device: Mutex<Option<(String, hidapi::HidDevice)>>,
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

/// Helper function to execute an operation on an opened device path with connection caching.
/// If an error occurs during the operation, the cache is cleared so the next command can reconnect cleanly.
fn with_device<F, R>(state: &tauri::State<AppState>, path: &str, f: F) -> Result<R, String>
where
    F: FnOnce(&hidapi::HidDevice) -> Result<R, String>,
{
    let mut cached = state.cached_device.lock().map_err(|e| e.to_string())?;
    let has_matching_cache = match &*cached {
        Some((cached_path, _)) => cached_path == path,
        None => false,
    };
    if !has_matching_cache {
        *cached = None;
        let api = state.api.lock().map_err(|e| e.to_string())?;
        let cpath = std::ffi::CString::new(path).map_err(|e| e.to_string())?;
        let device = api.open_path(&cpath).map_err(|e| format!("open failed: {e}"))?;
        *cached = Some((path.to_string(), device));
    }
    let device_ref = &cached.as_ref().unwrap().1;
    let result = f(device_ref);
    if result.is_err() {
        *cached = None; // Invalidate cache on I/O failure to trigger self-healing reconnect
    }
    result
}

/// List connected keyboards on our HID usage page (or any known model).
/// Performs an active handshake check to filter out non-writable duplicate interfaces on Windows.
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
        // Under Windows, we often get multiple entries for the same keyboard.
        // The correct vendor custom control collection always has usage == 1.
        #[cfg(target_os = "windows")]
        if d.usage() != 1 {
            continue;
        }
        let path = d.path().to_string_lossy().into_owned();
        if !seen.insert(path.clone()) {
            continue;
        }

        // Active Handshake Validation:
        // We open the device and perform a test query to verify it is writable and responsive.
        // If the device is currently already in our connection cache, we skip reopening it.
        let is_cached = {
            if let Ok(cached) = state.cached_device.lock() {
                if let Some((cached_path, _)) = &*cached {
                    cached_path == &path
                } else {
                    false
                }
            } else {
                false
            }
        };

        if !is_cached {
            let cpath = match std::ffi::CString::new(path.clone()) {
                Ok(cp) => cp,
                Err(_) => continue,
            };
            let is_valid = match api.open_path(&cpath) {
                Ok(device) => protocol::get_device_info(&device).is_ok(),
                Err(_) => false,
            };
            if !is_valid {
                continue;
            }
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
    with_device(&state, &path, |device| {
        protocol::get_device_info(device)
    })
}

/// Read game mode / performance settings.
#[tauri::command]
pub fn get_game_mode(
    state: tauri::State<AppState>,
    path: String,
    frame_version: u8,
) -> Result<protocol::GameMode, String> {
    with_device(&state, &path, |device| {
        protocol::get_game_mode(device, frame_version)
    })
}

/// Save game mode / performance settings.
#[tauri::command]
pub fn set_game_mode(
    state: tauri::State<AppState>,
    path: String,
    config: protocol::GameMode,
    frame_version: u8,
) -> Result<(), String> {
    with_device(&state, &path, |device| {
        protocol::set_game_mode(device, &config, frame_version)
    })
}

/// Read current lighting / LED effect settings.
#[tauri::command]
pub fn get_led_effect(
    state: tauri::State<AppState>,
    path: String,
    frame_version: u8,
) -> Result<protocol::LedEffect, String> {
    with_device(&state, &path, |device| {
        protocol::get_led_effect(device, frame_version)
    })
}

/// Save lighting / LED effect settings.
#[tauri::command]
pub fn set_led_effect(
    state: tauri::State<AppState>,
    path: String,
    config: protocol::LedEffect,
) -> Result<(), String> {
    with_device(&state, &path, |device| {
        protocol::set_led_effect(device, &config)
    })
}

/// Execute a factory reset or calibration clear.
#[tauri::command]
pub fn factory_reset(
    state: tauri::State<AppState>,
    path: String,
    reset_type: u8,
) -> Result<(), String> {
    with_device(&state, &path, |device| {
        protocol::factory_reset(device, reset_type)
    })
}

/// Stream real-time RGB frames to the backlight key matrix (GET_LED_DATA = cmd 50).
#[tauri::command]
pub fn stream_led_frame(
    state: tauri::State<AppState>,
    path: String,
    frame: Vec<protocol::LedColor>,
) -> Result<(), String> {
    with_device(&state, &path, |device| {
        protocol::stream_led_frame(device, frame)
    })
}

/// Write native loopback equalizer amplitudes (SET_MUSIC_DATA = cmd 53).
#[tauri::command]
pub fn send_music_data(
    state: tauri::State<AppState>,
    path: String,
    mode: u8,
    speed: u8,
    brightness: u8,
    amplitudes: Vec<u8>,
) -> Result<(), String> {
    with_device(&state, &path, |device| {
        protocol::send_music_data(device, mode, speed, brightness, &amplitudes)
    })
}

/// Upload animated gif or frame buffer sequences to the LCD screen storage (for future-proofing).
#[tauri::command]
pub fn upload_lcd_animation(
    state: tauri::State<AppState>,
    path: String,
    delays: Vec<u8>,
    rgb565_buffer: Vec<u8>,
) -> Result<(), String> {
    with_device(&state, &path, |device| {
        protocol::upload_lcd_animation(device, delays, rgb565_buffer)
    })
}
