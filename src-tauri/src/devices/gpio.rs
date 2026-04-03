//! GPIO management module for OrangePi devices

use crate::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, info, warn};

/// GPIO pin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpioConfig {
    /// Pin number (physical pin or GPIO number)
    pub pin: u32,
    /// Pin direction
    pub direction: GpioDirection,
    /// Pull-up/pull-down configuration
    pub pull: GpioPull,
    /// Initial value (for output pins)
    pub initial_value: u8,
    /// Interrupt configuration
    pub interrupt: Option<GpioInterruptConfig>,
}

/// GPIO direction
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GpioDirection {
    /// Input mode
    Input,
    /// Output mode
    Output,
}

/// GPIO pull configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GpioPull {
    /// No pull
    None,
    /// Pull-up
    Up,
    /// Pull-down
    Down,
}

/// GPIO interrupt configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpioInterruptConfig {
    /// Trigger type
    pub trigger: GpioTrigger,
    /// Debounce time in milliseconds
    pub debounce_ms: u64,
}

/// GPIO trigger type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GpioTrigger {
    /// Rising edge
    Rising,
    /// Falling edge
    Falling,
    /// Both edges
    Both,
    /// High level
    High,
    /// Low level
    Low,
}

/// GPIO pin information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpioPinInfo {
    /// Pin number
    pub pin: u32,
    /// Pin name
    pub name: String,
    /// Available modes
    pub modes: Vec<String>,
    /// Current mode
    pub current_mode: Option<String>,
    /// Is pin exported
    pub is_exported: bool,
}

/// GPIO manager
pub struct GpioManager {
    exported_pins: HashMap<u32, GpioPin>,
    event_tx: Option<tokio::sync::mpsc::Sender<GpioEvent>>,
}

/// GPIO pin handle
struct GpioPin {
    config: GpioConfig,
    value_fd: Option<std::fs::File>,
}

/// GPIO event
#[derive(Debug, Clone, Serialize)]
pub struct GpioEvent {
    pub pin: u32,
    pub value: u8,
    pub timestamp: u64,
}

impl GpioManager {
    /// Create a new GPIO manager
    pub fn new() -> Self {
        Self {
            exported_pins: HashMap::new(),
            event_tx: None,
        }
    }
    
    /// List available GPIO pins for the current board
    pub fn list_pins(&self) -> AppResult<Vec<GpioPinInfo>> {
        let mut pins = Vec::new();
        
        // OrangePi Zero 2 pin definitions
        let pin_definitions = vec![
            (3, "PA12", vec!["i2c", "gpio"]),
            (5, "PA11", vec!["i2c", "gpio"]),
            (7, "PA6", vec!["gpio"]),
            (8, "PG8", vec!["uart", "gpio"]),
            (10, "PG9", vec!["uart", "gpio"]),
            (11, "PA1", vec!["gpio"]),
            (12, "PA7", vec!["gpio"]),
            (13, "PA0", vec!["gpio"]),
            (15, "PA3", vec!["gpio"]),
            (16, "PA15", vec!["gpio"]),
            (18, "PA16", vec!["gpio"]),
            (19, "PA14", vec!["spi", "gpio"]),
            (21, "PA13", vec!["spi", "gpio"]),
            (22, "PA2", vec!["gpio"]),
            (23, "PA10", vec!["spi", "gpio"]),
            (24, "PA8", vec!["spi", "gpio"]),
        ];
        
        for (pin, name, modes) in pin_definitions {
            let info = GpioPinInfo {
                pin,
                name: name.to_string(),
                modes: modes.iter().map(|s| s.to_string()).collect(),
                current_mode: None,
                is_exported: self.exported_pins.contains_key(&pin),
            };
            pins.push(info);
        }
        
        Ok(pins)
    }
    
    /// Configure a GPIO pin
    pub fn configure_pin(&mut self, config: GpioConfig) -> AppResult<()> {
        info!("Configuring GPIO pin {} as {:?}", config.pin, config.direction);
        
        #[cfg(feature = "hardware-support")]
        {
            self.export_pin(config.pin)?;
            self.set_direction(config.pin, config.direction)?;
            self.set_pull(config.pin, config.pull)?;
            
            if config.direction == GpioDirection::Output {
                self.write_pin(config.pin, config.initial_value)?;
            }
        }
        
        let pin = GpioPin {
            config: config.clone(),
            value_fd: None,
        };
        
        self.exported_pins.insert(config.pin, pin);
        
        debug!("GPIO pin {} configured successfully", config.pin);
        Ok(())
    }
    
    /// Read pin value
    pub fn read_pin(&self, pin: u32) -> AppResult<u8> {
        let _pin = self
            .exported_pins
            .get(&pin)
            .ok_or_else(|| AppError::Gpio(format!("Pin {} not configured", pin)))?;
        
        #[cfg(feature = "hardware-support")]
        {
            let gpio_path = format!("/sys/class/gpio/gpio{}/value", pin);
            let value = std::fs::read_to_string(&gpio_path)
                .map_err(|e| AppError::Gpio(format!("Failed to read pin {}: {}", pin, e)))?;
            
            return value
                .trim()
                .parse::<u8>()
                .map_err(|e| AppError::Gpio(format!("Invalid pin value: {}", e)));
        }
        
        #[cfg(not(feature = "hardware-support"))]
        {
            Ok(0)
        }
    }
    
    /// Write pin value
    pub fn write_pin(&mut self, pin: u32, value: u8) -> AppResult<()> {
        let _pin = self
            .exported_pins
            .get_mut(&pin)
            .ok_or_else(|| AppError::Gpio(format!("Pin {} not configured", pin)))?;
        
        #[cfg(feature = "hardware-support")]
        {
            let gpio_path = format!("/sys/class/gpio/gpio{}/value", pin);
            std::fs::write(&gpio_path, value.to_string())
                .map_err(|e| AppError::Gpio(format!("Failed to write pin {}: {}", pin, e)))?;
        }
        
        debug!("GPIO pin {} set to {}", pin, value);
        Ok(())
    }
    
    /// Toggle pin value
    pub fn toggle_pin(&mut self, pin: u32) -> AppResult<u8> {
        let current = self.read_pin(pin)?;
        let new_value = if current == 0 { 1 } else { 0 };
        self.write_pin(pin, new_value)?;
        Ok(new_value)
    }
    
    /// Unconfigure a pin
    pub fn unconfigure_pin(&mut self, pin: u32) -> AppResult<()> {
        if self.exported_pins.remove(&pin).is_some() {
            #[cfg(feature = "hardware-support")]
            {
                self.unexport_pin(pin)?;
            }
            info!("GPIO pin {} unconfigured", pin);
        }
        Ok(())
    }
    
    /// Batch configure pins
    pub fn batch_configure(&mut self, configs: Vec<GpioConfig>) -> AppResult<Vec<AppResult<()>>> {
        configs.into_iter().map(|c| self.configure_pin(c)).collect()
    }
    
    /// Get pin configuration
    pub fn get_pin_config(&self, pin: u32) -> Option<&GpioConfig> {
        self.exported_pins.get(&pin).map(|p| &p.config)
    }
    
    /// Set event sender for interrupt notifications
    pub fn set_event_sender(&mut self, tx: tokio::sync::mpsc::Sender<GpioEvent>) {
        self.event_tx = Some(tx);
    }
    
    #[cfg(feature = "hardware-support")]
    fn export_pin(&self, pin: u32) -> AppResult<()> {
        let export_path = "/sys/class/gpio/export";
        if !std::path::Path::new(&format!("/sys/class/gpio/gpio{}", pin)).exists() {
            std::fs::write(export_path, pin.to_string())
                .map_err(|e| AppError::Gpio(format!("Failed to export pin {}: {}", pin, e)))?;
        }
        Ok(())
    }
    
    #[cfg(feature = "hardware-support")]
    fn unexport_pin(&self, pin: u32) -> AppResult<()> {
        let unexport_path = "/sys/class/gpio/unexport";
        std::fs::write(unexport_path, pin.to_string())
            .map_err(|e| AppError::Gpio(format!("Failed to unexport pin {}: {}", pin, e)))?;
        Ok(())
    }
    
    #[cfg(feature = "hardware-support")]
    fn set_direction(&self, pin: u32, direction: GpioDirection) -> AppResult<()> {
        let direction_path = format!("/sys/class/gpio/gpio{}/direction", pin);
        let direction_str = match direction {
            GpioDirection::Input => "in",
            GpioDirection::Output => "out",
        };
        std::fs::write(&direction_path, direction_str)
            .map_err(|e| AppError::Gpio(format!("Failed to set direction: {}", e)))?;
        Ok(())
    }
    
    #[cfg(feature = "hardware-support")]
    fn set_pull(&self, pin: u32, pull: GpioPull) -> AppResult<()> {
        // Pull configuration may vary by board
        // This is a simplified implementation
        let _ = pull;
        let _ = pin;
        Ok(())
    }
}

impl Default for GpioManager {
    fn default() -> Self {
        Self::new()
    }
}