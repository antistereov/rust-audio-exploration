use serde::{Deserialize, Serialize};

use super::audio_handler_error::AudioHandlerError;

pub trait Host {
    fn from_cpal_host_id(host_id: cpal::HostId) -> Self;
    fn to_cpal_host(&self) -> cpal::HostId;
}

#[derive(Serialize, Deserialize)]
pub enum WindowsHost {
    ASIO, WASAPI
}

impl Host for WindowsHost {
    fn from_cpal_host_id(host_id: cpal::HostId) -> Self {
        match host_id {
            cpal::HostId::Asio => WindowsHost::ASIO,
            cpal::HostId::Wasapi => WindowsHost::WASAPI,
        }
    }

    fn to_cpal_host(&self) -> cpal::HostId {
        match self {
            WindowsHost::ASIO => cpal::HostId::Asio,
            WindowsHost::WASAPI => cpal::HostId::Wasapi,
        }
    }
}

impl WindowsHost {
    pub fn from_string(string: &str) -> Result<Self, AudioHandlerError> {
        match string {
            "ASIO" => Ok(WindowsHost::ASIO),
            "WASAPI" => Ok(WindowsHost::WASAPI),
            _ => Err(AudioHandlerError::NoSuchWindowsHost(string.to_string()))
        }
    }
}
