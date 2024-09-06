use cpal::traits::{DeviceTrait, HostTrait};
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

pub struct AudioHandler {
    current_host: cpal::Host,
    output_device: Option<cpal::Device>,
    stream_config: Option<cpal::StreamConfig>,
}

pub enum Host {
    ASIO, WASAPI
}

impl AudioHandler {
    pub fn new() -> Self {
        let host = cpal::default_host();
        AudioHandler {
            current_host: host,
            output_device: None,
            stream_config: None,
        }
    }

    pub fn list_output_devices(&self) -> Vec<String> {
        self.current_host
            .output_devices()
            .unwrap()
            .filter_map(|device| device.name().ok())
            .collect()
    }

    pub fn switch_host(&mut self, host:Host) -> Result<(), AudioHandlerError> {
        let host_id = match host {
            Host::ASIO => cpal::HostId::Asio,
            Host::WASAPI => cpal::HostId::Wasapi
        };
        self.current_host = cpal::host_from_id(host_id)
            .map_err(|e| AudioHandlerError::HostSwitchError(e.to_string()))?;
        self.output_device = None;
        self.stream_config = None;
        Ok(())
    }

    pub fn select_output_device(&mut self, device_name: &str) -> Result<(), AudioHandlerError> {
        let device = self.current_host
            .output_devices()
            .unwrap()
            .find(|device| device.name().map(|name| name == device_name).unwrap_or(false))
            .ok_or_else(|| AudioHandlerError::DeviceNotFound(device_name.to_string()))?;

        let config = device.default_output_config()?;
        self.stream_config = Some(config.into());
        self.output_device = Some(device);
        Ok(())
    }

    pub fn get_current_device_name(&self) -> Option<String> {
        self.output_device.as_ref().and_then(|d| d.name().ok())
    }
}