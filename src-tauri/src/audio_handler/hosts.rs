use serde::{Deserialize, Serialize};

pub trait Host {
    fn from_cpal_host_id(host_id: cpal::HostId) -> Self;
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
}
