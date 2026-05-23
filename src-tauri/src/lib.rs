mod commands;
pub mod helper_config;
mod models;
pub mod protocol;

use commands::{AppState, LedStreamDaemon};
use hidapi::HidApi;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let api = HidApi::new().expect("failed to initialize hidapi");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
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
            commands::enter_custom_mode,
            commands::is_windows_host,
            commands::list_displays,
            commands::pick_gif_file,
            commands::get_helper_config,
            commands::set_helper_config,
            commands::install_helper_autostart,
            commands::uninstall_helper_autostart,
            commands::get_magnetic_axis_rt,
            commands::set_magnetic_axis_rt,
            commands::calibration_start,
            commands::calibration_finish,
            commands::poll_calibration_sample
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
