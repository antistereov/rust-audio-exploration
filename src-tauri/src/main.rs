#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ui;
mod synth;
mod audio_handler;

fn main() {

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default()
            .target(tauri_plugin_log::Target::new(
                tauri_plugin_log::TargetKind::Stdout,
            ))
            .build()
        )
        .manage(std::sync::Mutex::new(audio_handler::audio_handler::AudioHandler::new()))
        .invoke_handler(tauri::generate_handler![
            ui::audio_settings::audio_host::get_current_host,
            ui::audio_settings::audio_host::switch_host,
            ui::audio_settings::output_device::get_current_output_device_name,
            ui::audio_settings::output_device::select_output_device,
            ui::audio_settings::output_device::list_available_output_devices,
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");

    log::info!("Application started");
}
