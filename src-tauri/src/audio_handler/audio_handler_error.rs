use tauri::ipc::InvokeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioHandlerError {
    #[error("Failed to switch host: {0}")]
    HostSwitchError(String),

    #[error("Failed to get output devices: {0}")]
    OutputDevicesError(#[from] cpal::DevicesError),

    #[error("Output device '{0}' not found")]
    DeviceNotFound(String),

    #[error("Failed to get default output config: {0}")]
    DefaultConfigError(#[from] cpal::DefaultStreamConfigError),

    #[error("CPAL error: {0}")]
    CpalError(#[from] cpal::BuildStreamError),
}

impl Into<InvokeError> for AudioHandlerError {
    fn into(self) -> InvokeError {
        InvokeError::from(self.to_string())
    }
}