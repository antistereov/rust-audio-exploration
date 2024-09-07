#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ui;
mod synth;
mod audio_handler;
use chrono::prelude::*;

use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
            ])
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{} - {:<8} - {:<50} - {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
                    record.level(),
                    record.target(),
                    message
                ))
            })
            .build()
        )
        .setup(|app| {
            let audio_handler = std::sync::Mutex::new(audio_handler::audio_handler::AudioHandler::new());
            app.manage(audio_handler);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ui::audio_settings::audio_host::get_current_host,
            ui::audio_settings::audio_host::switch_host,
            ui::audio_settings::output_device::get_current_output_device_name,
            ui::audio_settings::output_device::select_output_device,
            ui::audio_settings::output_device::list_available_output_devices,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
