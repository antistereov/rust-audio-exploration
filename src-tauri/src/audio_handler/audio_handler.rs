use cpal::traits::{DeviceTrait, HostTrait};
use super::{audio_handler_error::AudioHandlerError, hosts::WindowsHost, hosts::Host};

pub struct AudioHandler {
    current_host: cpal::Host,
    output_device: Option<cpal::Device>,
    stream_config: Option<cpal::StreamConfig>,
}

impl AudioHandler {
    pub fn new() -> Self {
        AudioHandler {
            current_host: cpal::default_host(),
            output_device: cpal::default_host().default_output_device(),
            stream_config: None,
        }
    }

    pub fn get_current_device_name(&self) -> Option<String> {
        self.output_device.as_ref().and_then(|d| d.name().ok())
    }

    pub fn list_available_output_devices(&self) -> Vec<String> {
        self.current_host
            .output_devices()
            .unwrap()
            .filter_map(|device| device.name().ok())
            .collect()
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

    pub fn get_current_host(&self) -> WindowsHost {
        WindowsHost::from_cpal_host_id(self.current_host.id())
    }

    pub fn switch_host(&mut self, host: WindowsHost) -> Result<(), AudioHandlerError> {
        let host_id = match host {
            WindowsHost::ASIO => cpal::HostId::Asio,
            WindowsHost::WASAPI => cpal::HostId::Wasapi
        };
        self.current_host = cpal::host_from_id(host_id)
            .map_err(|e| AudioHandlerError::HostSwitchError(e.to_string()))?;
        self.output_device = None;
        self.stream_config = None;
        Ok(())
    }

}