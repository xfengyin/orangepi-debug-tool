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
    
    pub fn register(&self, adapter: Arc<dyn DeviceAdapter>) -> AppResult<()> {
        let id = adapter.id();
        
        if self.adapters.contains_key(id) {
            return Err(AppError::InvalidState(format!(
                "Adapter '{}' already registered",
                id
            )));
        }
        
        self.adapters.insert(id.to_string(), adapter.clone());
        
        if let Some(serial) = Arc::downcast::<dyn SerialAdapter>(adapter.clone()).ok() {
            self.serial_adapters.insert(id.to_string(), serial);
            if self.default_serial.is_none() {
                self.default_serial = Some(id.to_string());
            }
        }
        
        if let Some(gpio) = Arc::downcast::<dyn GpioAdapter>(adapter.clone()).ok() {
            self.gpio_adapters.insert(id.to_string(), gpio);
            if self.default_gpio.is_none() {
                self.default_gpio = Some(id.to_string());
            }
        }
        
        if let Some(pwm) = Arc::downcast::<dyn PwmAdapter>(adapter.clone()).ok() {
            self.pwm_adapters.insert(id.to_string(), pwm);
            if self.default_pwm.is_none() {
                self.default_pwm = Some(id.to_string());
            }
        }
        
        info!("Registered adapter: {}", id);
        Ok(())
    }
    
    pub fn unregister(&self, id: &str) -> AppResult<()> {
        if !self.adapters.contains_key(id) {
            return Err(AppError::NotFound(format!("Adapter '{}' not found", id)));
        }
        
        self.adapters.remove(id);
        self.serial_adapters.remove(id);
        self.gpio_adapters.remove(id);
        self.pwm_adapters.remove(id);
        
        if self.default_serial.as_deref() == Some(id) {
            self.default_serial = self.serial_adapters.iter().next().map(|s| s.key().clone());
        }
        if self.default_gpio.as_deref() == Some(id) {
            self.default_gpio = self.gpio_adapters.iter().next().map(|s| s.key().clone());
        }
        if self.default_pwm.as_deref() == Some(id) {
            self.default_pwm = self.pwm_adapters.iter().next().map(|s| s.key().clone());
        }
        
        info!("Unregistered adapter: {}", id);
        Ok(())
    }
    
    pub fn get_adapter(&self, id: &str) -> Option<Arc<dyn DeviceAdapter>> {
        self.adapters.get(id).map(|entry| entry.value().clone())
    }
    
    pub fn get_serial_adapter(&self, id: &str) -> Option<Arc<dyn SerialAdapter>> {
        self.serial_adapters.get(id).map(|entry| entry.value().clone())
    }
    
    pub fn get_gpio_adapter(&self, id: &str) -> Option<Arc<dyn GpioAdapter>> {
        self.gpio_adapters.get(id).map(|entry| entry.value().clone())
    }
    
    pub fn get_pwm_adapter(&self, id: &str) -> Option<Arc<dyn PwmAdapter>> {
        self.pwm_adapters.get(id).map(|entry| entry.value().clone())
    }
    
    pub fn get_default_serial(&self) -> Option<Arc<dyn SerialAdapter>> {
        self.default_serial.as_ref().and_then(|id| {
            self.serial_adapters.get(id).map(|entry| entry.value().clone())
        })
    }
    
    pub fn get_default_gpio(&self) -> Option<Arc<dyn GpioAdapter>> {
        self.default_gpio.as_ref().and_then(|id| {
            self.gpio_adapters.get(id).map(|entry| entry.value().clone())
        })
    }
    
    pub fn get_default_pwm(&self) -> Option<Arc<dyn PwmAdapter>> {
        self.default_pwm.as_ref().and_then(|id| {
            self.pwm_adapters.get(id).map(|entry| entry.value().clone())
        })
    }
    
    pub fn set_default_serial(&mut self, id: &str) -> AppResult<()> {
        if !self.serial_adapters.contains_key(id) {
            return Err(AppError::NotFound(format!("Serial adapter '{}' not found", id)));
        }
        self.default_serial = Some(id.to_string());
        Ok(())
    }
    
    pub fn set_default_gpio(&mut self, id: &str) -> AppResult<()> {
        if !self.gpio_adapters.contains_key(id) {
            return Err(AppError::NotFound(format!("GPIO adapter '{}' not found", id)));
        }
        self.default_gpio = Some(id.to_string());
        Ok(())
    }
    
    pub fn set_default_pwm(&mut self, id: &str) -> AppResult<()> {
        if !self.pwm_adapters.contains_key(id) {
            return Err(AppError::NotFound(format!("PWM adapter '{}' not found", id)));
        }
        self.default_pwm = Some(id.to_string());
        Ok(())
    }
    
    pub fn list_adapters(&self) -> Vec<String> {
        self.adapters.iter().map(|entry| entry.key().clone()).collect()
    }
    
    pub fn list_serial_adapters(&self) -> Vec<String> {
        self.serial_adapters.iter().map(|entry| entry.key().clone()).collect()
    }
    
    pub fn list_gpio_adapters(&self) -> Vec<String> {
        self.gpio_adapters.iter().map(|entry| entry.key().clone()).collect()
    }
    
    pub fn list_pwm_adapters(&self) -> Vec<String> {
        self.pwm_adapters.iter().map(|entry| entry.key().clone()).collect()
    }
}
