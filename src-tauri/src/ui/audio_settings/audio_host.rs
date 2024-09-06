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