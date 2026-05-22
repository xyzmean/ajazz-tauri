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

