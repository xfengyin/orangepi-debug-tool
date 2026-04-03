//! Device management module

pub mod gpio;
pub mod pwm;
pub mod serial;

use crate::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Device identifier
    pub id: String,
    /// Device name
    pub name: String,
    /// Device type
    pub device_type: DeviceType,
    /// Connection status
    pub connected: bool,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Device type enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    /// Serial port device
    Serial,
    /// GPIO device
    Gpio,
    /// PWM device
    Pwm,
    /// I2C device
    I2c,
    /// SPI device
    Spi,
}

/// Device manager for handling all connected devices
pub struct DeviceManager {
    devices: HashMap<String, DeviceInfo>,
}

impl DeviceManager {
    /// Create a new device manager
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }
    
    /// Register a device
    #[inline]
    pub fn register_device(&mut self, device: DeviceInfo) {
        self.devices.insert(device.id.clone(), device);
    }
    
    /// Unregister a device
    #[inline]
    pub fn unregister_device(&mut self, device_id: &str) {
        self.devices.remove(device_id);
    }
    
    /// Get device by ID
    #[inline]
    pub fn get_device(&self, device_id: &str) -> Option<&DeviceInfo> {
        self.devices.get(device_id)
    }
    
    /// Get all devices
    #[inline]
    pub fn get_all_devices(&self) -> Vec<&DeviceInfo> {
        self.devices.values().collect()
    }
    
    /// Get devices by type
    #[inline]
    pub fn get_devices_by_type(&self, device_type: DeviceType) -> Vec<&DeviceInfo> {
        self.devices
            .values()
            .filter(|d| d.device_type == device_type)
            .collect()
    }
    
    /// Update device connection status
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