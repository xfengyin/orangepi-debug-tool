use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, error, info};

use crate::error::{AppError, AppResult};

use super::traits::*;

#[derive(Debug)]
pub enum RegistryError {
    AlreadyRegistered(String),
    NotFound(String),
    NoHealthyAdapter,
    CapabilityNotSupported(String),
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryError::AlreadyRegistered(id) => write!(f, "Adapter '{}' already registered", id),
            RegistryError::NotFound(id) => write!(f, "Adapter '{}' not found", id),
            RegistryError::NoHealthyAdapter => write!(f, "No healthy adapter available"),
            RegistryError::CapabilityNotSupported(cap) => write!(f, "Capability '{}' not supported", cap),
        }
    }
}

impl std::error::Error for RegistryError {}

#[derive(Debug, Clone)]
pub struct RegistryStats {
    pub total_adapters: usize,
    pub healthy_adapters: usize,
    pub adapter_states: HashMap<String, bool>,
}

pub struct DeviceAdapterRegistry {
    adapters: DashMap<String, Arc<dyn DeviceAdapter>>,
    serial_adapters: DashMap<String, Arc<dyn SerialAdapter>>,
    gpio_adapters: DashMap<String, Arc<dyn GpioAdapter>>,
    pwm_adapters: DashMap<String, Arc<dyn PwmAdapter>>,
    default_serial: Option<String>,
    default_gpio: Option<String>,
    default_pwm: Option<String>,
}

impl Default for DeviceAdapterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceAdapterRegistry {
    pub fn new() -> Self {
        Self {
            adapters: DashMap::new(),
            serial_adapters: DashMap::new(),
            gpio_adapters: DashMap::new(),
            pwm_adapters: DashMap::new(),
            default_serial: None,
            default_gpio: None,
            default_pwm: None,
        }
    }

    pub fn register<A: DeviceAdapter + 'static>(&mut self, adapter: A) -> Result<(), RegistryError> {
        let id = adapter.id();
        
        if self.adapters.contains_key(id) {
            return Err(RegistryError::AlreadyRegistered(id));
        }
        
        let adapter = Arc::new(adapter);
        
        if adapter.capabilities().contains(&DeviceCapability::Serial) {
            if let Some(serial) = adapter.clone().downcast::<dyn SerialAdapter>() {
                self.serial_adapters.insert(id.to_string(), serial);
                if self.default_serial.is_none() {
                    self.default_serial = Some(id.to_string());
                }
            }
        }
        
        if adapter.capabilities().contains(&DeviceCapability::Gpio) {
            if let Some(gpio) = adapter.clone().downcast::<dyn GpioAdapter>() {
                self.gpio_adapters.insert(id.to_string(), gpio);
                if self.default_gpio.is_none() {
                    self.default_gpio = Some(id.to_string());
                }
            }
        }
        
        if adapter.capabilities().contains(&DeviceCapability::Pwm) {
            if let Some(pwm) = adapter.clone().downcast::<dyn PwmAdapter>() {
                self.pwm_adapters.insert(id.to_string(), pwm);
                if self.default_pwm.is_none() {
                    self.default_pwm = Some(id.to_string());
                }
            }
        }
        
        self.adapters.insert(id.to_string(), adapter);
        
        debug!("Registered adapter: {}", id);
        Ok(())
    }

    pub fn unregister(&mut self, id: &str) -> bool {
        self.adapters.remove(id);
        self.serial_adapters.remove(id);
        self.gpio_adapters.remove(id);
        self.pwm_adapters.remove(id);
        
        if self.default_serial.as_deref() == Some(id) {
            self.default_serial = self.serial_adapters.keys().next().map(|s| s.clone());
        }
        if self.default_gpio.as_deref() == Some(id) {
            self.default_gpio = self.gpio_adapters.keys().next().map(|s| s.clone());
        }
        if self.default_pwm.as_deref() == Some(id) {
            self.default_pwm = self.pwm_adapters.keys().next().map(|s| s.clone());
        }
        
        debug!("Unregistered adapter: {}", id);
        true
    }

    pub fn get(&self, id: &str) -> Option<Arc<dyn DeviceAdapter>> {
        self.adapters.get(id).map(|r| r.clone())
    }

    pub fn get_all(&self) -> Vec<Arc<dyn DeviceAdapter>> {
        self.adapters.iter().map(|r| r.value().clone()).collect()
    }

    pub fn has_adapter(&self, id: &str) -> bool {
        self.adapters.contains_key(id)
    }

    pub fn get_default_serial(&self) -> Option<Arc<dyn SerialAdapter>> {
        self.default_serial
            .as_ref()
            .and_then(|id| self.serial_adapters.get(id).map(|r| r.clone()))
    }

    pub fn get_default_gpio(&self) -> Option<Arc<dyn GpioAdapter>> {
        self.default_gpio
            .as_ref()
            .and_then(|id| self.gpio_adapters.get(id).map(|r| r.clone()))
    }

    pub fn get_default_pwm(&self) -> Option<Arc<dyn PwmAdapter>> {
        self.default_pwm
            .as_ref()
            .and_then(|id| self.pwm_adapters.get(id).map(|r| r.clone()))
    }

    pub fn set_default_serial(&mut self, id: &str) -> Result<(), RegistryError> {
        if !self.serial_adapters.contains_key(id) {
            return Err(RegistryError::NotFound(id.to_string()));
        }
        self.default_serial = Some(id.to_string());
        Ok(())
    }

    pub fn set_default_gpio(&mut self, id: &str) -> Result<(), RegistryError> {
        if !self.gpio_adapters.contains_key(id) {
            return Err(RegistryError::NotFound(id.to_string()));
        }
        self.default_gpio = Some(id.to_string());
        Ok(())
    }

    pub fn set_default_pwm(&mut self, id: &str) -> Result<(), RegistryError> {
        if !self.pwm_adapters.contains_key(id) {
            return Err(RegistryError::NotFound(id.to_string()));
        }
        self.default_pwm = Some(id.to_string());
        Ok(())
    }

    pub async fn auto_detect(&self) -> Result<DeviceInfo, RegistryError> {
        for adapter in self.adapters.iter() {
            let adapter = adapter.value();
            match adapter.health_check().await {
                Ok(status) if status.is_healthy() => {
                    info!("Auto-detected healthy adapter: {}", adapter.id());
                    return Ok(DeviceInfo {
                        id: adapter.id().to_string(),
                        name: adapter.name().to_string(),
                        capabilities: adapter.capabilities(),
                        board_model: None,
                        firmware_version: None,
                    });
                }
                Ok(status) => {
                    debug!("Adapter {} reported status: {:?}", adapter.id(), status.state);
                }
                Err(e) => {
                    debug!("Adapter {} health check failed: {}", adapter.id(), e);
                }
            }
        }
        Err(RegistryError::NoHealthyAdapter)
    }

    pub async fn auto_detect_serial(&self) -> Result<SerialPortInfo, RegistryError> {
        let adapter = self.get_default_serial()
            .ok_or_else(|| RegistryError::CapabilityNotSupported("Serial".to_string()))?;
        
        let ports = adapter.list_ports().await
            .map_err(|e| RegistryError::CapabilityNotSupported(e.to_string()))?;
        
        ports.into_iter()
            .next()
            .ok_or_else(|| RegistryError::NotFound("No serial ports found".to_string()))
    }

    pub fn get_stats(&self) -> RegistryStats {
        let mut adapter_states = HashMap::new();
        let mut healthy_count = 0;
        
        for adapter in self.adapters.iter() {
            let state = futures::executor::block_on(adapter.health_check())
                .map(|h| h.is_healthy())
                .unwrap_or(false);
            adapter_states.insert(adapter.key().clone(), state);
            if state {
                healthy_count += 1;
            }
        }
        
        RegistryStats {
            total_adapters: self.adapters.len(),
            healthy_adapters: healthy_count,
            adapter_states,
        }
    }

    pub fn list_serial_adapters(&self) -> Vec<String> {
        self.serial_adapters.keys().map(|s| s.clone()).collect()
    }

    pub fn list_gpio_adapters(&self) -> Vec<String> {
        self.gpio_adapters.keys().map(|s| s.clone()).collect()
    }

    pub fn list_pwm_adapters(&self) -> Vec<String> {
        self.pwm_adapters.keys().map(|s| s.clone()).collect()
    }
}
