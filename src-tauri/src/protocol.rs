//! Reverse-engineered Ajazz HID protocol — Rust port of the `reverse` branch's src/protocol/core.ts.
//!
//! Wire format (see core.ts for the full spec):
//!   Request packet (32 bytes, sent with report id 0):
//!     [0] 0xAA  [1] cmd  [2] len  [3..4] addr (LE)  [6] last-packet flag  [8..] payload
//!   Response packet:
//!     [0] 0x55  [1] cmd  [2] lenOrType  [3..4] addr (LE)  [8..] payload
//!
//! HID transport is done here in Rust (hidapi) because Tauri webviews have no WebHID.

use hidapi::HidDevice;
use serde::{Deserialize, Serialize};

pub const REPORT_ID: u8 = 0;
pub const REQUEST_HEADER: u8 = 0xAA;
pub const RESPONSE_HEADER: u8 = 0x55;
pub const HEADER_SIZE: usize = 8;
// AK980 MAX (and the vendor 0xFF68 interface generally) exposes a 64-byte output report
// (descriptor reportCount=0x40). Confirmed against live hardware: at 32 bytes the per-packet
// payload (24) mismatches the device's expected chunking; 64 gives a 56-byte payload.
pub const REPORT_SIZE: usize = 64;

/// Lighting effect mode that displays the custom per-key buffer (自定义).
pub const LED_MODE_CUSTOM: u8 = 128;

/// Max LED updates packed into one CMD_REALTIME_RGB (0x32) packet: (64 − 8) / 4 = 14.
pub const REALTIME_RGB_PER_PACKET: usize = 14;

/// Command opcodes (subset of CMD from core.ts; extend as features are ported).
#[allow(dead_code)] // several opcodes are placeholders for not-yet-ported features
pub mod cmd {
    pub const GET_DEVICE_INFO: u8 = 16;
    pub const GET_GAME_MODE: u8 = 17;
    pub const GET_KEY: u8 = 18;
    pub const GET_LED_EFFECT: u8 = 19;
    pub const SET_GAME_MODE: u8 = 33;
    pub const SET_LED_EFFECT: u8 = 35;
    /// Real-time per-LED RGB stream. Firmware sets `*0x2000047d=1` and arms a 50-tick
    /// watchdog: without a refresh packet within that window the system effect resumes.
    /// Format: header (8) + N × [led_idx, R, G, B], N ≤ 14. No flash writes (no wear).
    pub const SET_REALTIME_RGB: u8 = 0x32;
    pub const SET_FACTORY_RESET: u8 = 15;
}

/// Build one 32-byte request packet (port of `buildPacket` / minified `P`).
pub fn build_packet(command: u8, len: u8, addr: u16, payload: Option<&[u8]>, last: bool) -> [u8; REPORT_SIZE] {
    let mut buf = [0u8; REPORT_SIZE];
    buf[0] = REQUEST_HEADER;
    buf[1] = command;
    buf[2] = len;
    buf[3] = (addr & 0xFF) as u8;
    buf[4] = ((addr >> 8) & 0xFF) as u8;
    buf[6] = if last { 1 } else { 0 };
    if let Some(p) = payload {
        let n = p.len().min(REPORT_SIZE - HEADER_SIZE);
        buf[HEADER_SIZE..HEADER_SIZE + n].copy_from_slice(&p[..n]);
    }
    buf
}

/// Write one packet (prefixed with report id 0, as hidapi requires).
fn send(device: &HidDevice, packet: &[u8; REPORT_SIZE]) -> Result<(), String> {
    let mut out = [0u8; REPORT_SIZE + 1];
    out[0] = REPORT_ID;
    out[1..].copy_from_slice(packet);
    device.write(&out).map_err(|e| format!("write failed: {e}"))?;
    Ok(())
}

/// Read a response packet matching `expected_cmd`, retrying reads until timeout.
fn recv(device: &HidDevice, expected_cmd: u8, timeout_ms: i32) -> Result<[u8; REPORT_SIZE], String> {
    let mut buf = [0u8; REPORT_SIZE];
    // A few reads to skip unrelated input reports (e.g. notifications).
    for _ in 0..8 {
        let n = device
            .read_timeout(&mut buf, timeout_ms)
            .map_err(|e| format!("read failed: {e}"))?;
        if n == 0 {
            return Err(format!("command 0x{expected_cmd:02x} response timeout"));
        }
        if buf[0] == RESPONSE_HEADER && buf[1] == expected_cmd {
            return Ok(buf);
        }
    }
    Err(format!("command 0x{expected_cmd:02x}: no matching response"))
}

/// Chunked read transport (port of `transfer`/`readDataChunks`/minified `C`).
/// Returns `content_size` reassembled payload bytes.
pub fn read_data(device: &HidDevice, command: u8, content_size: usize, timeout_ms: i32) -> Result<Vec<u8>, String> {
    let per_packet = REPORT_SIZE - HEADER_SIZE; // 24
    let packet_count = content_size.div_ceil(per_packet);
    let mut out: Vec<u8> = Vec::with_capacity(content_size);

    for i in 0..packet_count {
        let addr = (i * per_packet) as u16;
        let remaining = content_size - i * per_packet;
        let len = remaining.min(per_packet) as u8;
        let last = i == packet_count - 1;

        let packet = build_packet(command, len, addr, None, last);
        send(device, &packet)?;
        let resp = recv(device, command, timeout_ms)?;
        out.extend_from_slice(&resp[HEADER_SIZE..REPORT_SIZE]);
    }
    out.truncate(content_size);
    Ok(out)
}

/// Decoded GET_DEVICE_INFO payload (port of `getDeviceInfo` / minified `Ce`).
#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub rom_size: u8,
    pub macro_space_size: u16,
    pub vendor_id: u16,
    pub product_id: u16,
    pub version: f32,
    pub sensor: u16,
    pub work_mode: u8,
    pub battery_level: u8,
    pub charge_status: u8,
    pub current_profile: u8,
    pub rt_precision: u8,
    pub frame_version: u8,
    pub lighting_version: u8,
}

pub fn parse_device_info(e: &[u8]) -> DeviceInfo {
    let u16le = |a: usize| e[a] as u16 | ((e[a + 1] as u16) << 8);
    let version = (((e[8] & 0x0F) as f32) + (((e[8] & 0xF0) >> 4) as f32) * 10.0 + (e[9] as f32) * 100.0) / 100.0;
    DeviceInfo {
        rom_size: e[0],
        macro_space_size: u16le(2),
        vendor_id: u16le(4),
        product_id: u16le(6),
        version: (version * 100.0).round() / 100.0,
        sensor: u16le(10),
        work_mode: e[16],
        battery_level: e[17],
        charge_status: e[18],
        current_profile: e[19],
        rt_precision: e[29],
        frame_version: e[30],
        lighting_version: e[31],
    }
}

/// Read & decode device info from an opened device.
pub fn get_device_info(device: &HidDevice) -> Result<DeviceInfo, String> {
    let payload = read_data(device, cmd::GET_DEVICE_INFO, 48, 500)?;
    Ok(parse_device_info(&payload))
}

/// Chunked write transport. Sends `content_size` payload bytes from `data` split across packets,
/// waiting for a response for each packet.
pub fn write_data(
    device: &HidDevice,
    command: u8,
    data: &[u8],
    timeout_ms: i32,
) -> Result<(), String> {
    let per_packet = REPORT_SIZE - HEADER_SIZE; // 24
    let content_size = data.len();
    let packet_count = content_size.div_ceil(per_packet);

    for i in 0..packet_count {
        let addr = (i * per_packet) as u16;
        let from = i * per_packet;
        let to = (from + per_packet).min(content_size);
        let chunk = &data[from..to];
        let len = chunk.len() as u8;
        let last = i == packet_count - 1;

        let packet = build_packet(command, len, addr, Some(chunk), last);
        send(device, &packet)?;
        // Wait for response matching command
        let _resp = recv(device, command, timeout_ms)?;
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameMode {
    pub game_mode: u8,
    pub fn_switch: u8,
    pub sleep_time: u8,
    pub key_delay: u8,
    pub report_rate: u8,
    pub system_mode: u8,
    pub tft_display_time: u8,
    pub top_dead_zone: f32,
    pub bottom_dead_zone: f32,
    pub stability_mode: u8,
    pub auto_calibration: u8,
    pub single_key_wakeup: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LedEffect {
    pub mode: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub driver_setting: u8,
    pub secondary_red: u8,
    pub secondary_green: u8,
    pub secondary_blue: u8,
    pub color_mode: u8,
    pub brightness: u8,
    pub speed: u8,
    pub direction: u8,
    pub effect_mode_type: u8,
    pub check_code_l: u8,
    pub check_code_h: u8,
}

pub fn parse_game_mode(o: &[u8]) -> GameMode {
    GameMode {
        game_mode: o[1],
        fn_switch: o[2],
        sleep_time: o[3],
        key_delay: o[4],
        report_rate: o[5],
        system_mode: o[6],
        tft_display_time: o[7],
        top_dead_zone: o[8] as f32 / 100.0,
        bottom_dead_zone: o[9] as f32 / 100.0,
        stability_mode: o[11],
        auto_calibration: o[14],
        single_key_wakeup: o[15],
    }
}

pub fn encode_game_mode(v: &GameMode) -> Vec<u8> {
    let mut e = vec![0u8; 56];
    e[1] = v.game_mode;
    e[2] = v.fn_switch;
    e[3] = v.sleep_time;
    e[4] = v.key_delay;
    e[5] = v.report_rate;
    e[6] = v.system_mode;
    e[7] = v.tft_display_time;
    e[8] = (v.top_dead_zone * 100.0).round() as u8;
    e[9] = (v.bottom_dead_zone * 100.0).round() as u8;
    e[11] = v.stability_mode;
    e[14] = v.auto_calibration;
    e[15] = v.single_key_wakeup;
    e
}

pub fn parse_led_effect(o: &[u8]) -> LedEffect {
    LedEffect {
        mode: o[0],
        red: o[1],
        green: o[2],
        blue: o[3],
        driver_setting: o[4],
        secondary_red: o[5],
        secondary_green: o[6],
        secondary_blue: o[7],
        color_mode: o[8],
        brightness: o[9],
        speed: o[10],
        direction: o[11],
        effect_mode_type: o[12],
        check_code_l: o[14],
        check_code_h: o[15],
    }
}

pub fn encode_led_effect(v: &LedEffect) -> Vec<u8> {
    let mut e = vec![0u8; 16];
    e[0] = v.mode;
    e[1] = v.red;
    e[2] = v.green;
    e[3] = v.blue;
    e[4] = 255; // driverSetting forced to 255 by upstream
    e[5] = v.secondary_red;
    e[6] = v.secondary_green;
    e[7] = v.secondary_blue;
    e[8] = v.color_mode;
    e[9] = v.brightness;
    e[10] = v.speed;
    e[11] = v.direction;
    e[12] = v.effect_mode_type;
    e[14] = 170; // 0xAA validation check code L
    e[15] = 85;  // 0x55 validation check code H
    e
}

pub fn get_game_mode(device: &HidDevice, frame_version: u8) -> Result<GameMode, String> {
    let timeout = if frame_version == 1 { 2000 } else { 500 };
    let payload = read_data(device, cmd::GET_GAME_MODE, 56, timeout)?;
    Ok(parse_game_mode(&payload))
}

pub fn set_game_mode(device: &HidDevice, config: &GameMode, frame_version: u8) -> Result<(), String> {
    let timeout = if frame_version == 1 { 2000 } else { 500 };
    let payload = encode_game_mode(config);
    write_data(device, cmd::SET_GAME_MODE, &payload, timeout)
}

pub fn get_led_effect(device: &HidDevice, frame_version: u8) -> Result<LedEffect, String> {
    let timeout = if frame_version == 1 { 2000 } else { 500 };
    let payload = read_data(device, cmd::GET_LED_EFFECT, 16, timeout)?;
    Ok(parse_led_effect(&payload))
}

pub fn set_led_effect(device: &HidDevice, config: &LedEffect) -> Result<(), String> {
    let payload = encode_led_effect(config);
    write_data(device, cmd::SET_LED_EFFECT, &payload, 500)
}

pub fn factory_reset(device: &HidDevice, reset_type: u8) -> Result<(), String> {
    let packet = build_packet(cmd::SET_FACTORY_RESET, reset_type, 0, None, true);
    send(device, &packet)?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    Ok(())
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LedColor {
    pub idx: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Stream a frame of per-LED colours via SET_REALTIME_RGB (cmd 0x32). Each packet carries up to
/// 14 LED updates ([idx, R, G, B]). The firmware holds the realtime override for ~50 ticks after
/// the last 0x32 packet; if the caller stops sending, system effects resume on their own.
///
/// Background: the previous code used cmd 0x24 (= CMD_WRITE_BINDINGS, flash sector 0x9A00) — that
/// corrupts key bindings and only "lit" a few LEDs as a side-effect. Firmware dump confirms 0x32 is
/// the proper realtime path (translates idx → physical LED through internal table; no flash wear).
pub fn stream_led_frame(device: &HidDevice, frame: Vec<LedColor>) -> Result<(), String> {
    if frame.is_empty() {
        return Ok(());
    }
    for chunk in frame.chunks(REALTIME_RGB_PER_PACKET) {
        let mut payload = [0u8; REPORT_SIZE - HEADER_SIZE];
        for (i, led) in chunk.iter().enumerate() {
            let o = i * 4;
            payload[o] = led.idx;
            payload[o + 1] = led.r;
            payload[o + 2] = led.g;
            payload[o + 3] = led.b;
        }
        let len = (chunk.len() * 4) as u8;
        let packet = build_packet(cmd::SET_REALTIME_RGB, len, 0, Some(&payload), true);
        send(device, &packet)?;
    }
    Ok(())
}

/// Put the keyboard into the explicit custom-buffer lighting mode (mode 128 / 自定义). Optional for
/// the streaming path — cmd 0x32 auto-takes-over via the realtime flag — but kept for callers that
/// want a sticky custom backdrop.
pub fn set_custom_lighting_mode(device: &HidDevice) -> Result<(), String> {
    let mut e = vec![0u8; 16];
    e[0] = LED_MODE_CUSTOM;
    e[4] = 255; // driverSetting (forced by upstream)
    e[9] = 100; // brightness
    e[14] = 0xAA; // validation check code L
    e[15] = 0x55; // validation check code H
    write_data(device, cmd::SET_LED_EFFECT, &e, 500)
}

pub fn send_music_data(
    device: &HidDevice,
    mode: u8,
    speed: u8,
    brightness: u8,
    amplitudes: &[u8],
) -> Result<(), String> {
    let mut buf = [0u8; 32];
    buf[0] = 0xAA;
    buf[1] = 53; // SET_MUSIC_DATA
    buf[2] = 0;  // Wired USB marker
    buf[3] = mode;
    buf[4] = speed;
    buf[5] = brightness;
    
    let n = amplitudes.len().min(21);
    buf[6..6 + n].copy_from_slice(&amplitudes[..n]);
    
    let mut sum: u32 = 0;
    for i in 0..31 {
        sum += buf[i] as u32;
    }
    buf[31] = (sum & 0xFF) as u8;
    
    let mut out = [0u8; 33];
    out[0] = REPORT_ID;
    out[1..].copy_from_slice(&buf);
    
    device.write(&out).map_err(|e| format!("music write failed: {e}"))?;
    Ok(())
}

pub fn upload_lcd_animation(
    device: &HidDevice,
    delays: Vec<u8>,
    rgb565_buffer: Vec<u8>,
) -> Result<(), String> {
    let num_frames = delays.len();
    if num_frames == 0 {
        return Err("No frames to upload".to_string());
    }
    
    let mut header = [255u8; 256];
    header[0] = num_frames as u8;
    for i in 0..(num_frames - 1).min(255) {
        header[i + 1] = delays[i].saturating_mul(5);
    }
    header[num_frames] = 0;
    
    let mut full_buffer = Vec::with_capacity(256 + rgb565_buffer.len());
    full_buffer.extend_from_slice(&header);
    full_buffer.extend_from_slice(&rgb565_buffer);
    
    let chunk_size = 4096;
    let content_size = full_buffer.len();
    let total_chunks = content_size.div_ceil(chunk_size) as u16;
    
    for i in 0..total_chunks {
        let from = i as usize * chunk_size;
        let to = (from + chunk_size).min(content_size);
        let chunk = &full_buffer[from..to];
        
        let mut large_report = vec![0u8; 8 + chunk_size];
        large_report[0] = 170;
        large_report[1] = 80; // SET_TFT_USER_ANIMATION
        large_report[2] = (i & 0xFF) as u8;
        large_report[3] = ((i >> 8) & 0xFF) as u8;
        large_report[4] = (total_chunks & 0xFF) as u8;
        large_report[5] = ((total_chunks >> 8) & 0xFF) as u8;
        large_report[6] = 0x50; // address offset low: 0x50
        large_report[7] = 0x06; // address offset high: 0x06
        
        large_report[8..8 + chunk.len()].copy_from_slice(chunk);
        
        let mut write_buf = vec![0u8; 8 + chunk_size + 1];
        write_buf[0] = REPORT_ID;
        write_buf[1..].copy_from_slice(&large_report);
        
        device.write(&write_buf).map_err(|e| format!("TFT write failed at chunk {i}: {e}"))?;
        let _resp = recv(device, 65, 2000)?;
    }
    Ok(())
}

