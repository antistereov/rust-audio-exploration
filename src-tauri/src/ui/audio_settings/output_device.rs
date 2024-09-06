use crate::audio_handler::{audio_handler::AudioHandler, audio_handler_error::AudioHandlerError};
use std::sync::Mutex;

#[tauri::command]
pub fn get_current_output_device_name(audio_handler: tauri::State<'_, Mutex<AudioHandler>>) -> Option<String> {
    audio_handler.lock().unwrap().get_current_device_name()
}

#[tauri::command]
pub fn list_available_output_devices(audio_handler: tauri::State<'_, Mutex<AudioHandler>>) -> Result<Vec<String>, AudioHandlerError> {
    audio_handler.lock().unwrap().list_available_output_devices()
}

#[tauri::command]
pub fn select_output_device(audio_handler: tauri::State<'_, Mutex<AudioHandler>>, device_name: String) -> Result<(), AudioHandlerError> {
    audio_handler.lock().unwrap().select_output_device(&device_name)
}