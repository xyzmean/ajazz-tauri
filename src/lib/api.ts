/**
 * Frontend ↔ Rust bridge. All HID communication happens in Rust (hidapi),
 * because Tauri webviews do not expose WebHID — the UI only calls invoke().
 */
import { invoke } from "@tauri-apps/api/core";

export interface DeviceSummary {
  path: string;
  vendorId: number;
  productId: number;
  product: string | null;
  manufacturer: string | null;
  /** Resolved keyboard model name from our table, if known. */
  modelName: string | null;
}

export interface DeviceInfo {
  romSize: number;
  macroSpaceSize: number;
  vendorId: number;
  productId: number;
  version: number;
  sensor: number;
  workMode: number;
  batteryLevel: number;
  chargeStatus: number;
  currentProfile: number;
  rtPrecision: number;
  frameVersion: number;
  lightingVersion: number;
}

export interface GameMode {
  gameMode: number;
  fnSwitch: number;
  sleepTime: number;
  keyDelay: number;
  reportRate: number;
  systemMode: number;
  tftDisplayTime: number;
  topDeadZone: number;
  bottomDeadZone: number;
  stabilityMode: number;
  autoCalibration: number;
  singleKeyWakeup: number;
}

export interface LedEffect {
  mode: number;
  red: number;
  green: number;
  blue: number;
  driverSetting: number;
  secondaryRed: number;
  secondaryGreen: number;
  secondaryBlue: number;
  colorMode: number;
  brightness: number;
  speed: number;
  direction: number;
  effectModeType: number;
  checkCodeL: number;
  checkCodeH: number;
}

/** List connected Ajazz keyboards visible to the OS HID layer. */
export const listDevices = () => invoke<DeviceSummary[]>("list_devices");

/** Read device info from a keyboard by its HID path, via our Rust protocol port. */
export const getDeviceInfo = (path: string) => invoke<DeviceInfo>("get_device_info", { path });

/** Read game mode / performance settings. */
export const getGameMode = (path: string, frameVersion: number) =>
  invoke<GameMode>("get_game_mode", { path, frameVersion });

/** Save game mode / performance settings. */
export const setGameMode = (path: string, config: GameMode, frameVersion: number) =>
  invoke<void>("set_game_mode", { path, config, frameVersion });

/** Read current lighting / LED effect settings. */
export const getLedEffect = (path: string, frameVersion: number) =>
  invoke<LedEffect>("get_led_effect", { path, frameVersion });

/** Save lighting / LED effect settings. */
export const setLedEffect = (path: string, config: LedEffect) =>
  invoke<void>("set_led_effect", { path, config });

/** Trigger factory reset or clear calibration. */
export const factoryReset = (path: string, resetType: number) =>
  invoke<void>("factory_reset", { path, resetType });

export interface LedColor {
  idx: number;
  r: number;
  g: number;
  b: number;
}

/** Stream a single real-time RGB frame to the backlight key matrix (one-shot). */
export const streamLedFrame = (path: string, frame: LedColor[]) =>
  invoke<void>("stream_led_frame", { path, frame });

/** Put the keyboard into custom per-key lighting mode (mode 128) so custom frames are shown. */
export const enterCustomMode = (path: string) =>
  invoke<void>("enter_custom_mode", { path });

// --- Background LED-frame daemon (owns the device, writes at a steady cadence) ---
/** Start the background streaming daemon for `path` (idempotent). */
export const startLedStream = (path: string) =>
  invoke<void>("start_led_stream", { path });
/** Push the latest frame for the daemon to stream (O(1) store, no device I/O). */
export const pushLedFrame = (frame: LedColor[]) =>
  invoke<void>("push_led_frame", { frame });
/** Stop the background streaming daemon and clear the pending frame. */
export const stopLedStream = () => invoke<void>("stop_led_stream");

/** Send native real-time audio spectrum amplitude packets. */
export const sendMusicData = (
  path: string,
  mode: number,
  speed: number,
  brightness: number,
  amplitudes: number[]
) => invoke<void>("send_music_data", { path, mode, speed, brightness, amplitudes });

// --- Background helper (ajazz-helperd): runs independent of the GUI, autostarts at logon ---
export interface HelperConfig {
  mode: string; // "off" | "screen" | "gif"
  devicePath: string | null;
  gifPath: string | null;
  fps: number;
}
/** Read the background helper's current config. */
export const getHelperConfig = () => invoke<HelperConfig>("get_helper_config");
/** Write the background helper's config (it polls this and switches behaviour). */
export const setHelperConfig = (config: HelperConfig) =>
  invoke<void>("set_helper_config", { config });
/** Register the helper to start at logon and launch it now. */
export const installHelperAutostart = () => invoke<void>("install_helper_autostart");
/** Remove the helper from logon autostart. */
export const uninstallHelperAutostart = () => invoke<void>("uninstall_helper_autostart");

/** Upload animated gif delays and frame buffers to LCD screen storage. */
export const uploadLcdAnimation = (
  path: string,
  delays: number[],
  rgb565Buffer: number[]
) => invoke<void>("upload_lcd_animation", { path, delays, rgb565Buffer });


