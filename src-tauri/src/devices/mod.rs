//! Device management module

pub mod gpio;
pub mod pwm;
pub mod serial;

use crate::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use gpio::GpioDevice;
pub use pwm::PwmDevice;
pub use serial::SerialDevice;

/// Device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub connected: bool,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    Serial,
    Gpio,
    Pwm,
    I2c,
    Spi,
}

pub struct DeviceManager {
    devices: HashMap<String, DeviceInfo>,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }
    
    #[inline]
    pub fn register_device(&mut self, device: DeviceInfo) {
        self.devices.insert(device.id.clone(), device);
    }
    
    #[inline]
    pub fn unregister_device(&mut self, device_id: &str) {
        self.devices.remove(device_id);
    }
    
    #[inline]
    pub fn get_device(&self, device_id: &str) -> Option<&DeviceInfo> {
        self.devices.get(device_id)
    }
    
    #[inline]
    pub fn get_all_devices(&self) -> Vec<&DeviceInfo> {
        self.devices.values().collect()
    }
    
    #[inline]
    pub fn get_devices_by_type(&self, device_type: DeviceType) -> Vec<&DeviceInfo> {
        self.devices
            .values()
            .filter(|d| d.device_type == device_type)
            .collect()
    }
    
    #[inline]
    pub fn update_connection_status(&mut self, device_id: &str, connected: bool) -> AppResult<()> {
        let device = self
            .devices
            .get_mut(device_id)
            .ok_or_else(|| AppError::NotFound(format!("Device {}", device_id)))?;
        device.connected = connected;
        Ok(())
    }
}
