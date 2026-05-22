mod commands;
mod models;
mod protocol;

use commands::{AppState, LedStreamDaemon};
use hidapi::HidApi;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let api = HidApi::new().expect("failed to initialize hidapi");

    tauri::Builder::default()
        .manage(AppState {
            api: Mutex::new(api),
            cached_device: Mutex::new(None),
            led_stream: LedStreamDaemon::default(),
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_devices,
            commands::get_device_info,
            commands::get_game_mode,
            commands::set_game_mode,
            commands::get_led_effect,
            commands::set_led_effect,
            commands::factory_reset,
            commands::stream_led_frame,
            commands::send_music_data,
            commands::upload_lcd_animation,
            commands::start_led_stream,
            commands::push_led_frame,
            commands::stop_led_stream,
            commands::enter_custom_mode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
