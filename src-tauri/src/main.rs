#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ui;
mod synth;
mod audio_handler;

fn main() {

  tauri::Builder::default()
      .manage(std::sync::Mutex::new(audio_handler::audio_handler::AudioHandler::new()))
      .invoke_handler(tauri::generate_handler![
          ui::tauri_interface::greet,
          ui::tauri_interface::switch_host,
          ui::tauri_interface::select_output_device,
          ui::tauri_interface::list_output_devices,
      ])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
