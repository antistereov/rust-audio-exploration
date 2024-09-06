use crate::audio_handler::{audio_handler::AudioHandler, audio_handler_error::AudioHandlerError, hosts::WindowsHost};
use std::sync::Mutex;

#[tauri::command]
pub fn get_current_host(audio_handler: tauri::State<'_, Mutex<AudioHandler>>) -> Result<WindowsHost, AudioHandlerError> {
    Ok(audio_handler.lock().unwrap().get_current_host())
}

#[tauri::command]
pub fn switch_host(audio_handler: tauri::State<'_, Mutex<AudioHandler>>, host: String) -> Result<(), AudioHandlerError> {
    let windows_host = WindowsHost::from_string(&host)?;
    audio_handler.lock().unwrap().switch_host(windows_host)
}

#[tauri::command]
pub fn get_current_device_name(audio_handler: tauri::State<'_, Mutex<AudioHandler>>) -> Option<String> {
    audio_handler.lock().unwrap().get_current_device_name()
}

#[tauri::command]
pub fn list_available_output_devices(audio_handler: tauri::State<'_, Mutex<AudioHandler>>) -> Vec<String> {
    audio_handler.lock().unwrap().list_available_output_devices()
}

#[tauri::command]
pub fn select_output_device(audio_handler: tauri::State<'_, Mutex<AudioHandler>>, device_name: String) -> Result<(), AudioHandlerError> {
    audio_handler.lock().unwrap().select_output_device(&device_name)
}

