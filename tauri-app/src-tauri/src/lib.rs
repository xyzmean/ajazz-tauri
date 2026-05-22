mod commands;
mod models;
mod protocol;

use commands::AppState;
use hidapi::HidApi;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let api = HidApi::new().expect("failed to initialize hidapi");

    tauri::Builder::default()
        .manage(AppState { api: Mutex::new(api) })
        .invoke_handler(tauri::generate_handler![
            commands::list_devices,
            commands::get_device_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
