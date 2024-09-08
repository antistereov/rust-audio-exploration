use cpal::traits::{DeviceTrait, HostTrait};

use super::{audio_handler_error::AudioHandlerError, hosts::WindowsHost, hosts::Host};

pub struct AudioHandler {
    current_host: cpal::Host,
    output_device: Option<cpal::Device>,
    stream_config: Option<cpal::StreamConfig>,
}

impl AudioHandler {
    pub fn new() -> Self {
        let default_host = cpal::default_host();
        let output_device = default_host.default_output_device();

        log::info!("Setting up audio host: {:?}",default_host.id());
        log::info!("  Default output device: {}",
            output_device.as_ref()
                .and_then(|dev| dev.name().ok())
                .unwrap_or_else(|| "No device".to_string())
        );

        AudioHandler {
            current_host: default_host,
            output_device: output_device,
            stream_config: None,
        }
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

    pub fn get_current_output_device_name(&self) -> Option<String> {
        self.output_device.as_ref().and_then(|d| d.name().ok())
    }

    pub fn list_available_output_devices(&self) -> Result<Vec<String>, AudioHandlerError> {
        match self.current_host.output_devices() {
            Ok(devices) => {
                let output_devices: Vec<String> = devices
                    .filter_map(|device| device.name().ok())
                    .collect();

                log::debug!("Available output devices:");
                output_devices.iter().enumerate().for_each( |(index, device)| {
                    log::debug!("  {}: {}", index + 1, device);
            });

                Ok(output_devices)
            }
            Err(err) => {
                log::error!("Failed to get output devices: {}", err);
                Err(AudioHandlerError::OutputDevicesError(err))
            }
        }
    }

    pub fn select_output_device(&mut self, device_name: &str) -> Result<(), AudioHandlerError> {
        match self.current_host
            .output_devices()
            .unwrap()
            .find(|device| device.name().map(|name| name == device_name).unwrap_or(false)) {
            Some(device) => {
                match device.default_output_config() {
                    Ok(config) => {
                        self.stream_config = Some(config.into());
                        self.output_device = Some(device);
                        log::info!("Successfully selected output device: {}", device_name);
                        Ok(())
                    },
                    Err(e) => {
                        let error = AudioHandlerError::from(e);
                        log::error!("Failed to get default output config for device {}: {:?}", device_name, error);
                        Err(error)
                    }
                }
            },
            None => {
                let error = AudioHandlerError::DeviceNotFound(device_name.to_string());
                log::error!("Output device not found: {}", device_name);
                Err(error)
            }
        }
    }

}