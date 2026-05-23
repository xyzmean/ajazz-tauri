//! `ajazz-helperd` — headless background helper.
//!
//! Streams live content to the keyboard backlight independent of the GUI:
//!   * `screen` — mirrors the primary display (Desktop Duplication via `scrap`, Windows only)
//!   * `gif`    — loops a GIF file
//! Both feed the realtime RGB stream (SET_REALTIME_RGB / cmd 0x32): the firmware arms a 50-tick
//! watchdog on each packet and falls back to the system effect if no refresh arrives. On-device
//! animation storage is not supported on this firmware, so a host-side stream is the only path.
//!
//! It reads its instructions from the shared `helper.json` config (written by the GUI), so it
//! keeps running after the GUI is closed. On Windows it can register itself to autostart at logon
//! (`--install` / `--uninstall`).
//!
//! Backlight matrix: 16 columns × 6 rows, LED index = row*16 + col (hardware-confirmed). Capture is
//! sampled on that uniform grid; cells without a physical LED simply do nothing.

// The helper is headless and must NOT pull in `tauri` (and its GTK/WebKit system deps): it depends
// only on the pure-Rust protocol + config modules, included directly rather than via the GUI lib.
#[allow(dead_code)]
#[path = "../protocol.rs"]
mod protocol;
#[allow(dead_code)]
#[path = "../helper_config.rs"]
mod helper_config;

use hidapi::{HidApi, HidDevice};
use protocol::LedColor;
use std::time::Duration;

const COLS: usize = 16;
const ROWS: usize = 6;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--install") {
        autostart::install();
        return;
    }
    if args.iter().any(|a| a == "--uninstall") {
        autostart::uninstall();
        return;
    }
    run();
}

fn run() {
    let api = match HidApi::new() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("[helperd] hidapi init failed: {e}");
            return;
        }
    };
    eprintln!("[helperd] started; polling {}", helper_config::config_path().display());
    loop {
        let cfg = helper_config::load();
        match cfg.mode.as_str() {
            "screen" => {
                if let Err(e) = run_screen(&api, &cfg) {
                    eprintln!("[helperd] screen: {e}");
                    std::thread::sleep(Duration::from_millis(500));
                }
            }
            "gif" => {
                if let Err(e) = run_gif(&api, &cfg) {
                    eprintln!("[helperd] gif: {e}");
                    std::thread::sleep(Duration::from_millis(500));
                }
            }
            _ => std::thread::sleep(Duration::from_millis(400)), // idle
        }
    }
}

/// Open the vendor HID interface (usage page 0xFF67/0xFF68), preferring `want` if given.
fn open_keyboard(api: &HidApi, want: &Option<String>) -> Option<HidDevice> {
    for d in api.device_list() {
        if d.usage_page() != 0xFF67 && d.usage_page() != 0xFF68 {
            continue;
        }
        if let Some(w) = want {
            if d.path().to_string_lossy() != w.as_str() {
                continue;
            }
        }
        if let Ok(dev) = api.open_path(d.path()) {
            return Some(dev);
        }
    }
    None
}

/// Sample a frame buffer on the 16×6 LED grid. `bgra` selects byte order (true for `scrap`'s BGRA,
/// false for RGBA from the GIF decoder).
fn sample_grid(buf: &[u8], w: usize, h: usize, stride: usize, bgra: bool) -> Vec<LedColor> {
    let mut out = Vec::with_capacity(COLS * ROWS);
    for row in 0..ROWS {
        for col in 0..COLS {
            let x = ((((col as f32) + 0.5) / COLS as f32) * w as f32) as usize;
            let y = ((((row as f32) + 0.5) / ROWS as f32) * h as f32) as usize;
            let i = y.min(h - 1) * stride + x.min(w - 1) * 4;
            if i + 2 >= buf.len() {
                continue;
            }
            let (r, g, b) = if bgra {
                (buf[i + 2], buf[i + 1], buf[i])
            } else {
                (buf[i], buf[i + 1], buf[i + 2])
            };
            out.push(LedColor {
                idx: (row * COLS + col) as u8,
                r,
                g,
                b,
            });
        }
    }
    out
}

// ── Screen mirror (Windows: Desktop Duplication via scrap) ───────────────────────────────────

#[cfg(windows)]
fn run_screen(api: &HidApi, cfg: &helper_config::HelperConfig) -> Result<(), String> {
    use scrap::{Capturer, Display};
    use std::time::Instant;

    let device = open_keyboard(api, &cfg.device_path).ok_or("no vendor keyboard found")?;
    protocol::set_custom_lighting_mode(&device)?;

    let display = Display::primary().map_err(|e| format!("no display: {e}"))?;
    let mut capturer = Capturer::new(display).map_err(|e| format!("capturer: {e}"))?;
    let (w, h) = (capturer.width(), capturer.height());
    let frame_dt = Duration::from_millis((1000 / cfg.fps.max(1)) as u64);
    let mut last: Option<Vec<LedColor>> = None;

    loop {
        if helper_config::load().mode != "screen" {
            return Ok(()); // mode changed — hand control back to the dispatcher
        }
        let started = Instant::now();
        match capturer.frame() {
            Ok(frame) => {
                let stride = frame.len() / h;
                let leds = sample_grid(&frame, w, h, stride, true);
                if last.as_ref() != Some(&leds) {
                    protocol::stream_led_frame(&device, leds.clone())?;
                    last = Some(leds);
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {} // frame not ready yet
            Err(e) => return Err(format!("capture failed: {e}")),
        }
        if let Some(rem) = frame_dt.checked_sub(started.elapsed()) {
            std::thread::sleep(rem);
        }
    }
}

#[cfg(not(windows))]
fn run_screen(_api: &HidApi, _cfg: &helper_config::HelperConfig) -> Result<(), String> {
    Err("screen mirror is only implemented on Windows".to_string())
}

// ── GIF playback (cross-platform) ────────────────────────────────────────────────────────────

/// Decode a GIF into fully-composited RGBA frames + per-frame delay (ms).
fn load_gif(path: &str) -> Result<(usize, usize, Vec<(Vec<u8>, u64)>), String> {
    let mut opts = gif::DecodeOptions::new();
    opts.set_color_output(gif::ColorOutput::RGBA);
    let file = std::fs::File::open(path).map_err(|e| format!("open gif: {e}"))?;
    let mut decoder = opts.read_info(file).map_err(|e| format!("decode gif: {e}"))?;
    let w = decoder.width() as usize;
    let h = decoder.height() as usize;
    let mut canvas = vec![0u8; w * h * 4];
    let mut frames = Vec::new();

    while let Some(frame) = decoder.read_next_frame().map_err(|e| format!("gif frame: {e}"))? {
        // Composite this (possibly partial) frame onto the running canvas.
        let (fw, fh) = (frame.width as usize, frame.height as usize);
        let (left, top) = (frame.left as usize, frame.top as usize);
        for fy in 0..fh {
            for fx in 0..fw {
                let (cx, cy) = (left + fx, top + fy);
                if cx >= w || cy >= h {
                    continue;
                }
                let src = (fy * fw + fx) * 4;
                if frame.buffer[src + 3] == 0 {
                    continue; // transparent pixel — keep canvas
                }
                let dst = (cy * w + cx) * 4;
                canvas[dst..dst + 4].copy_from_slice(&frame.buffer[src..src + 4]);
            }
        }
        let delay_ms = (frame.delay as u64 * 10).max(20); // GIF delay is in 1/100 s
        frames.push((canvas.clone(), delay_ms));
    }
    if frames.is_empty() {
        return Err("gif has no frames".to_string());
    }
    Ok((w, h, frames))
}

fn run_gif(api: &HidApi, cfg: &helper_config::HelperConfig) -> Result<(), String> {
    let path = cfg.gif_path.clone().ok_or("no gif_path set")?;
    let device = open_keyboard(api, &cfg.device_path).ok_or("no vendor keyboard found")?;
    protocol::set_custom_lighting_mode(&device)?;

    let (w, h, frames) = load_gif(&path)?;
    let stride = w * 4;
    loop {
        for (canvas, delay) in &frames {
            if helper_config::load().mode != "gif" {
                return Ok(());
            }
            let leds = sample_grid(canvas, w, h, stride, false);
            let _ = protocol::stream_led_frame(&device, leds);
            std::thread::sleep(Duration::from_millis(*delay));
        }
    }
}

// ── Autostart at logon ──────────────────────────────────────────────────────────────────────

#[cfg(windows)]
mod autostart {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
    use winreg::RegKey;

    const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
    const VALUE_NAME: &str = "AjazzKeyboardHelper";

    pub fn install() {
        let exe = match std::env::current_exe() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("[helperd] current_exe failed: {e}");
                return;
            }
        };
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        match hkcu.create_subkey(RUN_KEY) {
            Ok((run, _)) => {
                let cmd = format!("\"{}\"", exe.display());
                if let Err(e) = run.set_value(VALUE_NAME, &cmd) {
                    eprintln!("[helperd] set Run value failed: {e}");
                } else {
                    println!("[helperd] autostart installed: {cmd}");
                }
            }
            Err(e) => eprintln!("[helperd] open Run key failed: {e}"),
        }
    }

    pub fn uninstall() {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(run) = hkcu.open_subkey_with_flags(RUN_KEY, KEY_WRITE) {
            let _ = run.delete_value(VALUE_NAME);
        }
        println!("[helperd] autostart uninstalled");
    }
}

#[cfg(not(windows))]
mod autostart {
    pub fn install() {
        eprintln!("[helperd] autostart is only supported on Windows");
    }
    pub fn uninstall() {
        eprintln!("[helperd] autostart is only supported on Windows");
    }
}
