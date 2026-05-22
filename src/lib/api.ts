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

/** List connected Ajazz keyboards visible to the OS HID layer. */
export const listDevices = () => invoke<DeviceSummary[]>("list_devices");

/** Read device info from a keyboard by its HID path, via our Rust protocol port. */
export const getDeviceInfo = (path: string) => invoke<DeviceInfo>("get_device_info", { path });
