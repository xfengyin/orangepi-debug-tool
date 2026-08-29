use async_trait::async_trait;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::time::Instant;
use tracing::{debug, info};

use crate::error::{AppError, AppResult};

use super::traits::*;
use crate::observability::health::ComponentHealth;

#[derive(Debug)]
pub struct OrangePiZero3Adapter {
    board_model: String,
    pin_definitions: Vec<PinDefinition>,
}

#[derive(Debug, Clone)]
pub struct PinDefinition {
    pub pin: u32,
    pub name: String,
    pub gpio_number: Option<u32>,
    pub modes: Vec<String>,
}

impl OrangePiZero3Adapter {
    pub fn new() -> Self {
        Self {
            board_model: "OrangePi Zero3".to_string(),
            pin_definitions: Self::create_pin_definitions(),
        }
    }

    fn create_pin_definitions() -> Vec<PinDefinition> {
        vec![
            PinDefinition { pin: 3, name: "PA12".to_string(), gpio_number: Some(12), modes: vec!["i2c0".to_string(), "gpio".to_string()] },
            PinDefinition { pin: 5, name: "PA11".to_string(), gpio_number: Some(11), modes: vec!["i2c0".to_string(), "gpio".to_string()] },
            PinDefinition { pin: 7, name: "PA6".to_string(), gpio_number: Some(6), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 8, name: "PG8".to_string(), gpio_number: Some(200), modes: vec!["uart2".to_string(), "gpio".to_string()] },
            PinDefinition { pin: 10, name: "PG9".to_string(), gpio_number: Some(201), modes: vec!["uart2".to_string(), "gpio".to_string()] },
            PinDefinition { pin: 11, name: "PA1".to_string(), gpio_number: Some(1), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 12, name: "PA7".to_string(), gpio_number: Some(7), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 13, name: "PA0".to_string(), gpio_number: Some(0), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 15, name: "PA3".to_string(), gpio_number: Some(3), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 16, name: "PA15".to_string(), gpio_number: Some(15), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 18, name: "PA16".to_string(), gpio_number: Some(16), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 19, name: "PA14".to_string(), gpio_number: Some(14), modes: vec!["spi1".to_string(), "gpio".to_string()] },
            PinDefinition { pin: 21, name: "PA13".to_string(), gpio_number: Some(13), modes: vec!["spi1".to_string(), "gpio".to_string()] },
            PinDefinition { pin: 22, name: "PA2".to_string(), gpio_number: Some(2), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 23, name: "PA10".to_string(), gpio_number: Some(10), modes: vec!["spi1".to_string(), "gpio".to_string()] },
            PinDefinition { pin: 24, name: "PA8".to_string(), gpio_number: Some(8), modes: vec!["spi1".to_string(), "gpio".to_string()] },
            PinDefinition { pin: 26, name: "PA9".to_string(), gpio_number: Some(9), modes: vec!["spi1".to_string(), "gpio".to_string()] },
            PinDefinition { pin: 27, name: "PA19".to_string(), gpio_number: Some(19), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 28, name: "PA18".to_string(), gpio_number: Some(18), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 29, name: "PA21".to_string(), gpio_number: Some(21), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 31, name: "PA20".to_string(), gpio_number: Some(20), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 32, name: "PA10".to_string(), gpio_number: Some(10), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 33, name: "PA9".to_string(), gpio_number: Some(9), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 35, name: "PA19".to_string(), gpio_number: Some(19), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 36, name: "PA18".to_string(), gpio_number: Some(18), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 37, name: "PA21".to_string(), gpio_number: Some(21), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 38, name: "PA20".to_string(), gpio_number: Some(20), modes: vec!["gpio".to_string()] },
            PinDefinition { pin: 40, name: "PA22".to_string(), gpio_number: Some(22), modes: vec!["gpio".to_string()] },
        ]
    }
}

impl Default for OrangePiZero3Adapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DeviceAdapter for OrangePiZero3Adapter {
    fn id(&self) -> &'static str {
        "orangepi_zero3"
    }

    fn name(&self) -> &str {
        "OrangePi Zero3 Adapter"
    }

    fn capabilities(&self) -> HashSet<DeviceCapability> {
        let mut caps = HashSet::new();
        caps.insert(DeviceCapability::Serial);
        caps.insert(DeviceCapability::Gpio);
        caps.insert(DeviceCapability::Pwm);
        caps
    }

    async fn health_check(&self) -> AppResult<ComponentHealth> {
        let start = Instant::now();
        
        #[cfg(feature = "hardware-support")]
        {
            let gpio_path = "/dev/gpiochip0";
            if !std::path::Path::new(gpio_path).exists() {
                return Ok(ComponentHealth::degraded(self.id(), "GPIO device not found")
                    .with_latency(start.elapsed().as_millis() as u64));
            }
            
            let pwm_path = "/sys/class/pwm/pwmchip0";
            if !std::path::Path::new(pwm_path).exists() {
                return Ok(ComponentHealth::degraded(self.id(), "PWM device not found")
                    .with_latency(start.elapsed().as_millis() as u64));
            }
        }
        
        Ok(ComponentHealth::healthy(self.id())
            .with_latency(start.elapsed().as_millis() as u64))
    }

    async fn initialize(&self) -> AppResult<()> {
        info!("Initializing OrangePi Zero3 adapter for {}", self.board_model);
        
        #[cfg(feature = "hardware-support")]
        {
            if !std::path::Path::new("/dev/gpiochip0").exists() {
                return Err(AppError::Device("GPIO device not available".to_string()));
            }
        }
        
        Ok(())
    }
}

#[cfg(feature = "hardware-support")]
mod hardware {
    use super::*;
    use std::fs;
    use std::path::Path;
    use gpio_cdev::{Chip, LineRequestFlags};
    use std::sync::Mutex;

    pub struct GpioHardwareState {
        exported_pins: HashSet<u32>,
    }

    impl Default for GpioHardwareState {
        fn default() -> Self {
            Self {
                exported_pins: HashSet::new(),
            }
        }
    }
}
