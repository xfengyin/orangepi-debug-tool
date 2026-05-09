use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use parking_lot::Mutex;
use tracing::{debug, error, info, warn};

use crate::adapters::{DeviceAdapterRegistry, GpioAdapter, GpioDirection, GpioPull, GpioTrigger, GpioPinInfo};
use crate::config::GpioDeviceConfig;
use crate::error::{AppError, AppResult};
use crate::observability::{MetricsCollector, GpioMetric};

#[derive(Debug, Clone)]
pub struct GpioPin {
    pub pin: u32,
    pub direction: GpioDirection,
    pub value: u8,
    pub pull: GpioPull,
    pub interrupt_enabled: bool,
}

pub struct GpioService {
    registry: Arc<Mutex<DeviceAdapterRegistry>>,
    config: GpioDeviceConfig,
    pins: Arc<RwLock<HashMap<u32, GpioPin>>>,
    metrics: Arc<MetricsCollector>,
    interrupt_callbacks: Arc<RwLock<Vec<mpsc::Sender<GpioInterrupt>>>>,
}

#[derive(Debug, Clone)]
pub struct GpioInterrupt {
    pub pin: u32,
    pub value: u8,
    pub trigger: GpioTrigger,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl GpioService {
    pub fn new(registry: Arc<Mutex<DeviceAdapterRegistry>>, config: &GpioDeviceConfig) -> Self {
        let metrics = Arc::new(MetricsCollector::new());
        metrics.register_counter("gpio_pin_reads", "Total GPIO pin reads");
        metrics.register_counter("gpio_pin_writes", "Total GPIO pin writes");
        metrics.register_counter("gpio_interrupts", "Total GPIO interrupts");
        metrics.register_histogram("gpio_read_latency_ms", "GPIO read latency in milliseconds");
        metrics.register_histogram("gpio_write_latency_ms", "GPIO write latency in milliseconds");

        Self {
            registry,
            config: config.clone(),
            pins: Arc::new(RwLock::new(HashMap::new())),
            metrics,
            interrupt_callbacks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn initialize(&self) -> AppResult<()> {
        info!("Initializing GpioService with config: {:?}", self.config);
        
        if let Some(adapter) = self.registry.lock().get_default_gpio() {
            let pins = adapter.list_pins().await
                .map_err(|e| AppError::Gpio(format!("Failed to list pins: {}", e)))?;
            info!("Found {} GPIO pins", pins.len());
        }
        
        Ok(())
    }

    pub async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down GpioService");
        
        let pins = self.pins.read().await;
        for (pin, _) in pins.iter() {
            info!("Unexporting pin: {}", pin);
        }
        
        Ok(())
    }

    pub async fn list_pins(&self) -> AppResult<Vec<GpioPinInfo>> {
        let adapter = self.registry.lock().get_default_gpio()
            .ok_or_else(|| AppError::NotFound("No GPIO adapter available".to_string()))?;
        
        adapter.list_pins().await
            .map_err(|e| AppError::Gpio(format!("Failed to list pins: {}", e)))
    }

    pub async fn export_pin(&self, pin: u32) -> AppResult<()> {
        let adapter = self.registry.lock().get_default_gpio()
            .ok_or_else(|| AppError::NotFound("No GPIO adapter available".to_string()))?;

        adapter.export_pin(pin).await
            .map_err(|e| AppError::Gpio(format!("Failed to export pin {}: {}", pin, e)))?;

        let mut pins = self.pins.write().await;
        pins.insert(pin, GpioPin {
            pin,
            direction: GpioDirection::Input,
            value: 0,
            pull: GpioPull::None,
            interrupt_enabled: false,
        });

        info!("Exported GPIO pin {}", pin);
        Ok(())
    }

    pub async fn unexport_pin(&self, pin: u32) -> AppResult<()> {
        let adapter = self.registry.lock().get_default_gpio()
            .ok_or_else(|| AppError::NotFound("No GPIO adapter available".to_string()))?;

        adapter.unexport_pin(pin).await
            .map_err(|e| AppError::Gpio(format!("Failed to unexport pin {}: {}", pin, e)))?;

        self.pins.write().await.remove(&pin);

        info!("Unexported GPIO pin {}", pin);
        Ok(())
    }

    pub async fn set_direction(&self, pin: u32, direction: GpioDirection) -> AppResult<()> {
        let adapter = self.registry.lock().get_default_gpio()
            .ok_or_else(|| AppError::NotFound("No GPIO adapter available".to_string()))?;

        adapter.set_direction(pin, direction).await
            .map_err(|e| AppError::Gpio(format!("Failed to set direction for pin {}: {}", pin, e)))?;

        if let Some(pin_state) = self.pins.write().await.get_mut(&pin) {
            pin_state.direction = direction;
        }

        debug!("Set GPIO pin {} direction to {:?}", pin, direction);
        Ok(())
    }

    pub async fn set_pull(&self, pin: u32, pull: GpioPull) -> AppResult<()> {
        let adapter = self.registry.lock().get_default_gpio()
            .ok_or_else(|| AppError::NotFound("No GPIO adapter available".to_string()))?;

        adapter.set_pull(pin, pull).await
            .map_err(|e| AppError::Gpio(format!("Failed to set pull for pin {}: {}", pin, e)))?;

        if let Some(pin_state) = self.pins.write().await.get_mut(&pin) {
            pin_state.pull = pull;
        }

        debug!("Set GPIO pin {} pull to {:?}", pin, pull);
        Ok(())
    }

    pub async fn read_pin(&self, pin: u32) -> AppResult<u8> {
        let adapter = self.registry.lock().get_default_gpio()
            .ok_or_else(|| AppError::NotFound("No GPIO adapter available".to_string()))?;

        let start = std::time::Instant::now();
        
        let value = adapter.read_pin(pin).await
            .map_err(|e| AppError::Gpio(format!("Failed to read pin {}: {}", pin, e)))?;

        let elapsed = start.elapsed().as_millis() as f64;
        self.metrics.increment("gpio_pin_reads");
        self.metrics.observe_histogram("gpio_read_latency_ms", elapsed);

        if let Some(pin_state) = self.pins.write().await.get_mut(&pin) {
            pin_state.value = value;
        }

        debug!("Read GPIO pin {} value: {}", pin, value);
        Ok(value)
    }

    pub async fn write_pin(&self, pin: u32, value: u8) -> AppResult<()> {
        if value > 1 {
            return Err(AppError::InvalidArgument("GPIO value must be 0 or 1".to_string()));
        }

        let adapter = self.registry.lock().get_default_gpio()
            .ok_or_else(|| AppError::NotFound("No GPIO adapter available".to_string()))?;

        let start = std::time::Instant::now();
        
        adapter.write_pin(pin, value).await
            .map_err(|e| AppError::Gpio(format!("Failed to write pin {}: {}", pin, e)))?;

        let elapsed = start.elapsed().as_millis() as f64;
        self.metrics.increment("gpio_pin_writes");
        self.metrics.observe_histogram("gpio_write_latency_ms", elapsed);

        if let Some(pin_state) = self.pins.write().await.get_mut(&pin) {
            pin_state.value = value;
        }

        debug!("Wrote GPIO pin {} value: {}", pin, value);
        Ok(())
    }

    pub async fn enable_interrupt(&self, pin: u32, trigger: GpioTrigger) -> AppResult<()> {
        let adapter = self.registry.lock().get_default_gpio()
            .ok_or_else(|| AppError::NotFound("No GPIO adapter available".to_string()))?;

        adapter.enable_interrupt(pin, trigger).await
            .map_err(|e| AppError::Gpio(format!("Failed to enable interrupt on pin {}: {}", pin, e)))?;

        if let Some(pin_state) = self.pins.write().await.get_mut(&pin) {
            pin_state.interrupt_enabled = true;
        }

        self.metrics.increment("gpio_interrupts");
        info!("Enabled interrupt on GPIO pin {} with trigger {:?}", pin, trigger);
        Ok(())
    }

    pub async fn disable_interrupt(&self, pin: u32) -> AppResult<()> {
        let adapter = self.registry.lock().get_default_gpio()
            .ok_or_else(|| AppError::NotFound("No GPIO adapter available".to_string()))?;

        adapter.disable_interrupt(pin).await
            .map_err(|e| AppError::Gpio(format!("Failed to disable interrupt on pin {}: {}", pin, e)))?;

        if let Some(pin_state) = self.pins.write().await.get_mut(&pin) {
            pin_state.interrupt_enabled = false;
        }

        debug!("Disabled interrupt on GPIO pin {}", pin);
        Ok(())
    }

    pub async fn batch_read(&self, pins: &[u32]) -> AppResult<HashMap<u32, u8>> {
        let adapter = self.registry.lock().get_default_gpio()
            .ok_or_else(|| AppError::NotFound("No GPIO adapter available".to_string()))?;

        let mut results = HashMap::new();
        let start = std::time::Instant::now();

        for &pin in pins {
            match adapter.read_pin(pin).await {
                Ok(value) => {
                    results.insert(pin, value);
                    if let Some(pin_state) = self.pins.write().await.get_mut(&pin) {
                        pin_state.value = value;
                    }
                }
                Err(e) => {
                    warn!("Failed to read pin {}: {}", pin, e);
                }
            }
        }

        let elapsed = start.elapsed().as_millis() as f64;
        self.metrics.add_counter("gpio_pin_reads", pins.len() as u64);
        self.metrics.observe_histogram("gpio_read_latency_ms", elapsed / pins.len() as f64);

        debug!("Batch read {} GPIO pins", results.len());
        Ok(results)
    }

    pub async fn batch_write(&self, values: &[(u32, u8)]) -> AppResult<()> {
        let adapter = self.registry.lock().get_default_gpio()
            .ok_or_else(|| AppError::NotFound("No GPIO adapter available".to_string()))?;

        let start = std::time::Instant::now();

        for (pin, value) in values {
            if *value > 1 {
                return Err(AppError::InvalidArgument("GPIO value must be 0 or 1".to_string()));
            }

            if let Err(e) = adapter.write_pin(*pin, *value).await {
                warn!("Failed to write pin {}: {}", pin, e);
            } else if let Some(pin_state) = self.pins.write().await.get_mut(pin) {
                pin_state.value = *value;
            }
        }

        let elapsed = start.elapsed().as_millis() as f64;
        self.metrics.add_counter("gpio_pin_writes", values.len() as u64);
        self.metrics.observe_histogram("gpio_write_latency_ms", elapsed / values.len() as f64);

        debug!("Batch write {} GPIO pins", values.len());
        Ok(())
    }

    pub async fn get_pin_state(&self, pin: u32) -> Option<GpioPin> {
        self.pins.read().await.get(&pin).cloned()
    }

    pub async fn get_all_pins(&self) -> Vec<GpioPin> {
        self.pins.read().await.values().cloned().collect()
    }

    pub fn get_metrics(&self) -> GpioMetrics {
        GpioMetrics {
            exported_pins: self.pins.blocking_read().len() as u64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GpioMetrics {
    pub exported_pins: u64,
}
