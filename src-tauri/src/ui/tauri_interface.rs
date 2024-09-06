use crate::audio_handler::audio_handler::{AudioHandler, AudioHandlerError, Host};
use std::sync::Mutex;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}", name)
}

#[tauri::command]
pub fn switch_host(audio_handler: tauri::State<'_, Mutex<AudioHandler>>, host: String) -> Result<(), String> {
    let host_id = match host.as_str() {
        "ASIO" => Ok(Host::ASIO),
        "WASAPI" => Ok(Host::WASAPI),
        _ => Err(AudioHandlerError::HostSwitchError(host).to_string())
    }?;
    audio_handler.lock().unwrap().switch_host(host_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn select_output_device(audio_handler: tauri::State<'_, Mutex<AudioHandler>>, device_name: String) -> Result<(), String> {
    audio_handler.lock().unwrap().select_output_device(&device_name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_output_devices(audio_handler: tauri::State<'_, Mutex<AudioHandler>>) -> Result<Vec<String>, String> {
    Ok(audio_handler.lock().unwrap().list_output_devices())
}