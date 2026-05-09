pub mod serial_service;
pub mod gpio_service;
pub mod pwm_service;

pub use serial_service::SerialService;
pub use gpio_service::GpioService;
pub use pwm_service::PwmService;

use crate::adapters::DeviceAdapterRegistry;
use crate::config::DeviceConfigSection;
use crate::error::AppResult;
use std::sync::Arc;
use parking_lot::Mutex;

pub struct ServiceManager {
    pub serial: SerialService,
    pub gpio: GpioService,
    pub pwm: PwmService,
}

impl ServiceManager {
    pub fn new(registry: Arc<Mutex<DeviceAdapterRegistry>>, config: &DeviceConfigSection) -> Self {
        Self {
            serial: SerialService::new(registry.clone(), &config.serial),
            gpio: GpioService::new(registry.clone(), &config.gpio),
            pwm: PwmService::new(registry.clone(), &config.pwm),
        }
    }

    pub async fn initialize(&self) -> AppResult<()> {
        self.serial.initialize().await?;
        self.gpio.initialize().await?;
        self.pwm.initialize().await?;
        Ok(())
    }

    pub async fn shutdown(&self) -> AppResult<()> {
        self.serial.shutdown().await?;
        self.gpio.shutdown().await?;
        self.pwm.shutdown().await?;
        Ok(())
    }
}
