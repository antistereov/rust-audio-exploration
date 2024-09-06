#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ui;
mod synth;
mod audio_handler;

fn main() {

  tauri::Builder::default()
        .manage(std::sync::Mutex::new(audio_handler::audio_handler::AudioHandler::new()))
        .invoke_handler(tauri::generate_handler![
            ui::audio_settings::get_current_host,
            ui::audio_settings::switch_host,
            ui::audio_settings::get_current_device_name,
            ui::audio_settings::select_output_device,
            ui::audio_settings::list_available_output_devices,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
