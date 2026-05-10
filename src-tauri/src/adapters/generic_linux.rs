use async_trait::async_trait;
use std::collections::HashSet;
use std::path::Path;
use tokio::time::Instant;
use tracing::{debug, warn};

use crate::error::{AppError, AppResult};

use super::traits::*;
use crate::observability::health::HealthStatus;

#[derive(Debug)]
pub struct GenericLinuxAdapter {
    sysfs_gpio_path: String,
    sysfs_pwm_path: String,
}

impl GenericLinuxAdapter {
    pub fn new() -> Self {
        Self {
            sysfs_gpio_path: "/sys/class/gpio".to_string(),
            sysfs_pwm_path: "/sys/class/pwm".to_string(),
        }
    }

    pub fn with_paths(mut self, gpio_path: &str, pwm_path: &str) -> Self {
        self.sysfs_gpio_path = gpio_path.to_string();
        self.sysfs_pwm_path = pwm_path.to_string();
        self
    }

    fn detect_gpio_driver(&self) -> Option<String> {
        if Path::new("/dev/gpiochip0").exists() {
            Some("gpio-cdev".to_string());
        } else if Path::new(&self.sysfs_gpio_path).exists() {
            Some("sysfs".to_string());
        } else {
            None
        }
    }

    fn detect_pwm_driver(&self) -> Option<String> {
        if Path::new(&self.sysfs_pwm_path).exists() {
            Some("sysfs".to_string());
        } else {
            None
        }
    }
}

impl Default for GenericLinuxAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DeviceAdapter for GenericLinuxAdapter {
    fn id(&self) -> &'static str {
        "generic_linux"
    }

    fn name(&self) -> &str {
        "Generic Linux Adapter"
    }

    fn capabilities(&self) -> HashSet<DeviceCapability> {
        let mut caps = HashSet::new();
        caps.insert(DeviceCapability::Serial);
        
        if self.detect_gpio_driver().is_some() {
            caps.insert(DeviceCapability::Gpio);
        }
        
        if self.detect_pwm_driver().is_some() {
            caps.insert(DeviceCapability::Pwm);
        }
        
        caps
    }

    async fn health_check(&self) -> AppResult<HealthStatus> {
        let start = Instant::now();
        
        let gpio_driver = self.detect_gpio_driver();
        let pwm_driver = self.detect_pwm_driver();
        
        let message = match (&gpio_driver, &pwm_driver) {
            (Some(g), Some(p)) => format!("GPIO: {}, PWM: {}", g, p),
            (Some(g), None) => format!("GPIO: {} (PWM not available)", g),
            (None, Some(p)) => format!("PWM: {} (GPIO not available)", p),
            (None, None) => {
                return Ok(HealthStatus::unhealthy(self.id(), "No hardware devices detected")
                    .with_latency(start.elapsed().as_millis() as u64));
            }
        };
        
        let has_serial = tokio_serial::available_ports()
            .map(|ports| !ports.is_empty())
            .unwrap_or(false);
        
        if has_serial {
            Ok(HealthStatus::healthy(self.id())
                .with_latency(start.elapsed().as_millis() as u64))
        } else {
            Ok(HealthStatus::degraded(self.id(), "Serial ports not available")
                .with_latency(start.elapsed().as_millis() as u64))
        }
    }
}

#[cfg(feature = "hardware-support")]
mod hardware {
    use super::*;
    use std::fs;
    use std::io::{Read, Write};

    pub fn sysfs_gpio_export(pin: u32) -> AppResult<()> {
        let export_path = "/sys/class/gpio/export";
        let gpio_path = format!("/sys/class/gpio/gpio{}", pin);
        
        if !Path::new(&gpio_path).exists() {
            fs::write(export_path, pin.to_string())
                .map_err(|e| AppError::Gpio(format!("Failed to export pin {}: {}", pin, e)))?;
        }
        
        Ok(())
    }

    pub fn sysfs_gpio_unexport(pin: u32) -> AppResult<()> {
        let unexport_path = "/sys/class/gpio/unexport";
        fs::write(unexport_path, pin.to_string())
            .map_err(|e| AppError::Gpio(format!("Failed to unexport pin {}: {}", pin, e)))?;
        Ok(())
    }

    pub fn sysfs_gpio_set_direction(pin: u32, direction: &str) -> AppResult<()> {
        let dir_path = format!("/sys/class/gpio/gpio{}/direction", pin);
        fs::write(&dir_path, direction)
            .map_err(|e| AppError::Gpio(format!("Failed to set direction: {}", e)))?;
        Ok(())
    }

    pub fn sysfs_gpio_read(pin: u32) -> AppResult<u8> {
        let value_path = format!("/sys/class/gpio/gpio{}/value", pin);
        let mut file = fs::File::open(&value_path)
            .map_err(|e| AppError::Gpio(format!("Failed to open value file: {}", e)))?;
        
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| AppError::Gpio(format!("Failed to read value: {}", e)))?;
        
        contents.trim()
            .parse::<u8>()
            .map_err(|e| AppError::Gpio(format!("Invalid value: {}", e)))
    }

    pub fn sysfs_gpio_write(pin: u32, value: u8) -> AppResult<()> {
        let value_path = format!("/sys/class/gpio/gpio{}/value", pin);
        let mut file = fs::File::create(&value_path)
            .map_err(|e| AppError::Gpio(format!("Failed to open value file: {}", e)))?;
        
        file.write_all(if value == 0 { b"0" } else { b"1" })
            .map_err(|e| AppError::Gpio(format!("Failed to write value: {}", e)))?;
        
        Ok(())
    }

    pub fn sysfs_pwm_enable(channel: u32) -> AppResult<()> {
        let enable_path = format!("/sys/class/pwm/pwmchip0/pwm{}/enable", channel);
        fs::write(&enable_path, "1")
            .map_err(|e| AppError::Pwm(format!("Failed to enable PWM channel {}: {}", channel, e)))?;
        Ok(())
    }

    pub fn sysfs_pwm_disable(channel: u32) -> AppResult<()> {
        let enable_path = format!("/sys/class/pwm/pwmchip0/pwm{}/enable", channel);
        fs::write(&enable_path, "0")
            .map_err(|e| AppError::Pwm(format!("Failed to disable PWM channel {}: {}", channel, e)))?;
        Ok(())
    }

    pub fn sysfs_pwm_set_period(channel: u32, period_ns: u32) -> AppResult<()> {
        let period_path = format!("/sys/class/pwm/pwmchip0/pwm{}/period", channel);
        fs::write(&period_path, period_ns.to_string())
            .map_err(|e| AppError::Pwm(format!("Failed to set period: {}", e)))?;
        Ok(())
    }

    pub fn sysfs_pwm_set_duty_cycle(channel: u32, duty_ns: u32) -> AppResult<()> {
        let duty_path = format!("/sys/class/pwm/pwmchip0/pwm{}/duty_cycle", channel);
        fs::write(&duty_path, duty_ns.to_string())
            .map_err(|e| AppError::Pwm(format!("Failed to set duty cycle: {}", e)))?;
        Ok(())
    }
}
