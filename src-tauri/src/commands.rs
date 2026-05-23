//! Tauri commands — the IPC surface the Vue frontend calls via invoke().

use crate::helper_config::{self, HelperConfig};
use crate::{models, protocol};
use hidapi::HidApi;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct AppState {
    pub api: Mutex<HidApi>,
    pub cached_device: Mutex<Option<(String, hidapi::HidDevice)>>,
    pub led_stream: LedStreamDaemon,
}

/// Background LED-frame streaming daemon.
///
/// Both screen-mirror and GIF playback are just *frame producers*: the frontend pushes the
/// latest frame into `latest` (an O(1) store, no device I/O on the IPC thread), and a single
/// background thread owns the device handle and writes frames at a steady cadence, independent
/// of the webview's timers (which the OS throttles when the window is backgrounded). Identical
/// consecutive frames are skipped, so a static screen / paused GIF produces zero USB traffic.
#[derive(Default)]
pub struct LedStreamDaemon {
    running: Arc<AtomicBool>,
    latest: Arc<Mutex<Option<Vec<protocol::LedColor>>>>,
    handle: Mutex<Option<std::thread::JoinHandle<()>>>,
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

/// One enumerated vendor HID interface (before grouping into physical keyboards).
struct Candidate {
    path: String,
    vid: u16,
    pid: u16,
    usage_page: u16,
    usage: u16,
    product: Option<String>,
    manufacturer: Option<String>,
}

/// List connected keyboards on our vendor HID usage page.
///
/// On Windows a single keyboard exposes several HID interfaces, so we de-duplicate by
/// (vendorId, productId) — one entry per physical board. Within each group we *prefer* the
/// interface that answers our protocol handshake, but if none answers (device busy, or the
/// collection layout differs from what we expect) we still surface the first candidate as a
/// fallback. This is deliberate: an over-strict filter previously dropped every device.
#[tauri::command]
pub fn list_devices(state: tauri::State<AppState>) -> Result<Vec<DeviceSummary>, String> {
    let mut api = state.api.lock().map_err(|e| e.to_string())?;
    api.refresh_devices().map_err(|e| e.to_string())?;

    // Pass 1: collect every interface on the vendor usage page (0xFF67 / 0xFF68).
    let mut seen = HashSet::new();
    let mut cands: Vec<Candidate> = Vec::new();
    for d in api.device_list() {
        let is_custom_hid = d.usage_page() == 0xFF67 || d.usage_page() == 0xFF68;
        if !is_custom_hid {
            continue;
        }
        let path = d.path().to_string_lossy().into_owned();
        if !seen.insert(path.clone()) {
            continue;
        }
        cands.push(Candidate {
            path,
            vid: d.vendor_id(),
            pid: d.product_id(),
            usage_page: d.usage_page(),
            usage: d.usage(),
            product: d.product_string().map(str::to_owned),
            manufacturer: d.manufacturer_string().map(str::to_owned),
        });
    }

    eprintln!(
        "[list_devices] {} candidate interface(s) on vendor usage page",
        cands.len()
    );
    for c in &cands {
        eprintln!(
            "  vid={:04x} pid={:04x} usage_page={:04x} usage={:#x} path={}",
            c.vid, c.pid, c.usage_page, c.usage, c.path
        );
    }

    // Pass 2: group by (vid, pid) = one physical keyboard, then pick the best interface.
    let cached_path = state
        .cached_device
        .lock()
        .ok()
        .and_then(|c| c.as_ref().map(|(p, _)| p.clone()));

    let mut groups: Vec<(u16, u16)> = Vec::new();
    for c in &cands {
        if !groups.contains(&(c.vid, c.pid)) {
            groups.push((c.vid, c.pid));
        }
    }

    let mut out = Vec::new();
    for (vid, pid) in groups {
        let group: Vec<&Candidate> =
            cands.iter().filter(|c| c.vid == vid && c.pid == pid).collect();

        // Prefer: (a) the interface we already hold open, then (b) one that answers the handshake.
        let mut chosen: Option<&Candidate> = group
            .iter()
            .find(|c| cached_path.as_deref() == Some(c.path.as_str()))
            .copied();

        if chosen.is_none() {
            for c in &group {
                if let Ok(cp) = std::ffi::CString::new(c.path.clone()) {
                    if let Ok(device) = api.open_path(&cp) {
                        if protocol::get_device_info(&device).is_ok() {
                            chosen = Some(c);
                            break;
                        }
                    }
                }
            }
        }

        // Fallback: never drop a keyboard just because the handshake didn't land.
        let pick = chosen.unwrap_or(group[0]);
        if chosen.is_none() {
            eprintln!(
                "[list_devices] vid={vid:04x} pid={pid:04x}: no interface answered handshake; \
                 surfacing {} as fallback",
                pick.path
            );
        }

        out.push(DeviceSummary {
            path: pick.path.clone(),
            vendor_id: vid,
            product_id: pid,
            product: pick.product.clone(),
            manufacturer: pick.manufacturer.clone(),
            model_name: models::find(vid, pid),
        });
    }

    eprintln!("[list_devices] returning {} keyboard(s)", out.len());
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

/// Start the background LED-frame streaming daemon for `path`.
/// Idempotent: a second call while running is a no-op. The daemon owns its own device handle
/// and writes the most recently pushed frame at a steady cadence until `stop_led_stream`.
#[tauri::command]
pub fn start_led_stream(state: tauri::State<AppState>, path: String) -> Result<(), String> {
    // Claim the running flag; if it was already set, a daemon is live — nothing to do.
    if state.led_stream.running.swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    // Open a dedicated handle for the daemon thread (reusing the single shared HidApi).
    let device = {
        let api = state.api.lock().map_err(|e| e.to_string())?;
        let cpath = std::ffi::CString::new(path).map_err(|e| e.to_string())?;
        match api.open_path(&cpath) {
            Ok(d) => d,
            Err(e) => {
                state.led_stream.running.store(false, Ordering::SeqCst);
                return Err(format!("open failed: {e}"));
            }
        }
    };

    // Enter custom-buffer mode once so the streamed frames are actually displayed.
    let _ = protocol::set_custom_lighting_mode(&device);

    let running = state.led_stream.running.clone();
    let latest = state.led_stream.latest.clone();
    let handle = std::thread::spawn(move || {
        let cadence = Duration::from_millis(16); // ~60 Hz write ceiling
        let mut last_sent: Option<Vec<protocol::LedColor>> = None;
        while running.load(Ordering::SeqCst) {
            let started = Instant::now();
            let frame = latest.lock().ok().and_then(|g| g.clone());
            if let Some(frame) = frame {
                // Optimization: only touch the wire when the frame actually changed.
                if last_sent.as_ref() != Some(&frame) {
                    if protocol::stream_led_frame(&device, frame.clone()).is_err() {
                        break; // device went away — let the daemon exit
                    }
                    last_sent = Some(frame);
                }
            }
            if let Some(rem) = cadence.checked_sub(started.elapsed()) {
                std::thread::sleep(rem);
            }
        }
        running.store(false, Ordering::SeqCst);
    });
    *state.led_stream.handle.lock().map_err(|e| e.to_string())? = Some(handle);
    Ok(())
}

/// Push the latest frame for the daemon to stream. O(1) store — no device I/O here.
#[tauri::command]
pub fn push_led_frame(
    state: tauri::State<AppState>,
    frame: Vec<protocol::LedColor>,
) -> Result<(), String> {
    *state.led_stream.latest.lock().map_err(|e| e.to_string())? = Some(frame);
    Ok(())
}

/// Put the keyboard into custom per-key lighting mode (mode 128). Needed before custom colours
/// (static fill or a single streamed frame) are displayed.
#[tauri::command]
pub fn enter_custom_mode(state: tauri::State<AppState>, path: String) -> Result<(), String> {
    with_device(&state, &path, protocol::set_custom_lighting_mode)
}

/// Stop the LED-frame daemon and clear the pending frame.
#[tauri::command]
pub fn stop_led_stream(state: tauri::State<AppState>) -> Result<(), String> {
    state.led_stream.running.store(false, Ordering::SeqCst);
    if let Some(h) = state.led_stream.handle.lock().map_err(|e| e.to_string())?.take() {
        let _ = h.join();
    }
    *state.led_stream.latest.lock().map_err(|e| e.to_string())? = None;
    Ok(())
}

// ── Background helper (ajazz-helperd) control ────────────────────────────────────────────────

/// Read the current background-helper config.
#[tauri::command]
pub fn get_helper_config() -> HelperConfig {
    helper_config::load()
}

/// Write the background-helper config (mode = off|screen|gif, gif path, device, fps).
/// The running helper polls this file and switches behaviour accordingly.
#[tauri::command]
pub fn set_helper_config(config: HelperConfig) -> Result<(), String> {
    helper_config::save(&config).map_err(|e| e.to_string())
}

/// Locate the `ajazz-helperd` binary that ships alongside the GUI executable.
fn helperd_path() -> Option<std::path::PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?;
    let name = if cfg!(windows) { "ajazz-helperd.exe" } else { "ajazz-helperd" };
    Some(dir.join(name))
}

/// Register the helper to start at logon and launch it now.
#[tauri::command]
pub fn install_helper_autostart() -> Result<(), String> {
    let p = helperd_path().ok_or("helper binary not found next to the app")?;
    std::process::Command::new(&p)
        .arg("--install")
        .status()
        .map_err(|e| format!("--install failed: {e}"))?;
    std::process::Command::new(&p)
        .spawn()
        .map_err(|e| format!("launch failed: {e}"))?;
    Ok(())
}

/// Unregister the helper from logon autostart.
#[tauri::command]
pub fn uninstall_helper_autostart() -> Result<(), String> {
    let p = helperd_path().ok_or("helper binary not found next to the app")?;
    std::process::Command::new(&p)
        .arg("--uninstall")
        .status()
        .map_err(|e| format!("--uninstall failed: {e}"))?;
    Ok(())
}

// ── Magnetic-axis Rapid Trigger + calibration ────────────────────────────────────────────────

#[tauri::command]
pub fn get_magnetic_axis_rt(
    state: tauri::State<AppState>,
    path: String,
    rt_precision: u8,
    frame_version: u8,
) -> Result<Vec<protocol::MagneticAxisRT>, String> {
    with_device(&state, &path, |device| {
        protocol::get_magnetic_axis_rt(device, rt_precision, frame_version)
    })
}

#[tauri::command]
pub fn set_magnetic_axis_rt(
    state: tauri::State<AppState>,
    path: String,
    keys: Vec<protocol::MagneticAxisRT>,
    rt_precision: u8,
    frame_version: u8,
) -> Result<(), String> {
    with_device(&state, &path, |device| {
        protocol::set_magnetic_axis_rt(device, &keys, rt_precision, frame_version)
    })
}

#[tauri::command]
pub fn calibration_start(state: tauri::State<AppState>, path: String) -> Result<(), String> {
    with_device(&state, &path, protocol::calibration_start)
}

#[tauri::command]
pub fn calibration_finish(state: tauri::State<AppState>, path: String) -> Result<(), String> {
    with_device(&state, &path, protocol::calibration_finish)
}

/// Frontend poll for live calibration samples. Returns the freshest sample (or none) after a short
/// blocking read. Designed to be called in a loop while the calibration session is active.
#[tauri::command]
pub fn poll_calibration_sample(
    state: tauri::State<AppState>,
    path: String,
) -> Result<Option<protocol::CalibrationSample>, String> {
    with_device(&state, &path, |device| protocol::poll_calibration_sample(device, 50))
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
