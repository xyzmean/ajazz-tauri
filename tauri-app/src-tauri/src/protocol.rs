//! Reverse-engineered Ajazz HID protocol — Rust port of reverse-source/src/protocol/core.ts.
//!
//! Wire format (see core.ts for the full spec):
//!   Request packet (32 bytes, sent with report id 0):
//!     [0] 0xAA  [1] cmd  [2] len  [3..4] addr (LE)  [6] last-packet flag  [8..] payload
//!   Response packet:
//!     [0] 0x55  [1] cmd  [2] lenOrType  [3..4] addr (LE)  [8..] payload
//!
//! HID transport is done here in Rust (hidapi) because Tauri webviews have no WebHID.

use hidapi::HidDevice;

pub const REPORT_ID: u8 = 0;
pub const REQUEST_HEADER: u8 = 0xAA;
pub const RESPONSE_HEADER: u8 = 0x55;
pub const HEADER_SIZE: usize = 8;
pub const REPORT_SIZE: usize = 32;
pub const USAGE_PAGE: u16 = 0xFF67;

/// Command opcodes (subset of CMD from core.ts; extend as features are ported).
#[allow(dead_code)] // several opcodes are placeholders for not-yet-ported features
pub mod cmd {
    pub const GET_DEVICE_INFO: u8 = 16;
    pub const GET_GAME_MODE: u8 = 17;
    pub const GET_KEY: u8 = 18;
    pub const GET_LED_EFFECT: u8 = 19;
    pub const SET_GAME_MODE: u8 = 33;
    pub const SET_LED_EFFECT: u8 = 35;
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
