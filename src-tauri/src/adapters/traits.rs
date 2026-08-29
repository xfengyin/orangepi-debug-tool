use async_trait::async_trait;
use std::collections::HashSet;
use std::fmt::Debug;

use crate::error::{AppError, AppResult};
use crate::observability::health::{ComponentHealth, HealthState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceCapability {
    Serial,
    Gpio,
    Pwm,
    I2C,
    Spi,
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub capabilities: HashSet<DeviceCapability>,
    pub board_model: Option<String>,
    pub firmware_version: Option<String>,
}

#[async_trait]
pub trait DeviceAdapter: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &str;
    fn capabilities(&self) -> HashSet<DeviceCapability>;
    
    async fn health_check(&self) -> AppResult<ComponentHealth> {
        Ok(ComponentHealth::healthy(self.id()))
    }
    
    async fn initialize(&self) -> AppResult<()> {
        Ok(())
    }
    
    async fn shutdown(&self) -> AppResult<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioDirection {
    Input,
    Output,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioPull {
    None,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioTrigger {
    Rising,
    Falling,
    Both,
    High,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialConfig {
    pub port_name: String,
    pub baud_rate: u32,
    pub data_bits: u8,
    pub parity: String,
    pub stop_bits: u8,
    pub flow_control: String,
    pub read_timeout_ms: u64,
    pub write_timeout_ms: u64,
}

impl Default for SerialConfig {
    fn default() -> Self {
        Self {
            port_name: String::new(),
            baud_rate: 115200,
            data_bits: 8,
            parity: "none".to_string(),
            stop_bits: 1,
            flow_control: "none".to_string(),
            read_timeout_ms: 1000,
            write_timeout_ms: 1000,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SerialPortInfo {
    pub port_name: String,
    pub port_type: String,
}

#[derive(Debug, Clone)]
pub struct GpioPinInfo {
    pub pin: u32,
    pub name: String,
    pub modes: Vec<String>,
    pub current_mode: Option<String>,
    pub is_exported: bool,
}

#[derive(Debug, Clone)]
pub struct PwmChannelInfo {
    pub channel: u32,
    pub name: String,
    pub enabled: bool,
    pub frequency_hz: Option<u32>,
    pub duty_cycle: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwmConfig {
    pub chip: u32,
    pub channel: u32,
    pub pin: u32,
    pub frequency: f64,
    pub duty_cycle: f64,
    pub enabled: bool,
}

impl Default for PwmConfig {
    fn default() -> Self {
        Self {
            chip: 0,
            channel: 0,
            pin: 0,
            frequency: 1000.0,
            duty_cycle: 50.0,
            enabled: false,
        }
    }
}

#[derive(Debug)]
pub struct SerialHandle {
    pub port_name: String,
    pub config: SerialConfig,
    #[cfg(feature = "hardware-support")]
    pub stream: tokio_serial::SerialStream,
}

use serde::{Deserialize, Serialize};

#[async_trait]
pub trait SerialAdapter: DeviceAdapter {
    async fn list_ports(&self) -> AppResult<Vec<SerialPortInfo>>;
    
    async fn connect(&self, config: SerialConfig) -> AppResult<SerialHandle>;
    
    async fn disconnect(&self, handle: SerialHandle) -> AppResult<()>;
    
    async fn read(&self, handle: &SerialHandle, buffer: &mut [u8]) -> AppResult<usize>;
    
    async fn write(&self, handle: &SerialHandle, data: &[u8]) -> AppResult<usize>;
    
    async fn set_baudrate(&self, handle: &SerialHandle, baudrate: u32) -> AppResult<()>;
}

#[async_trait]
pub trait GpioAdapter: DeviceAdapter {
    async fn list_pins(&self) -> AppResult<Vec<GpioPinInfo>>;
    
    async fn export_pin(&self, pin: u32) -> AppResult<()>;
    
    async fn unexport_pin(&self, pin: u32) -> AppResult<()>;
    
    async fn set_direction(&self, pin: u32, direction: GpioDirection) -> AppResult<()>;
    
    async fn set_pull(&self, pin: u32, pull: GpioPull) -> AppResult<()>;
    
    async fn read_pin(&self, pin: u32) -> AppResult<u8>;
    
    async fn write_pin(&self, pin: u32, value: u8) -> AppResult<()>;
    
    async fn enable_interrupt(&self, pin: u32, trigger: GpioTrigger) -> AppResult<()>;
    
    async fn disable_interrupt(&self, pin: u32) -> AppResult<()>;
}

#[async_trait]
pub trait PwmAdapter: DeviceAdapter {
    async fn list_channels(&self) -> AppResult<Vec<PwmChannelInfo>>;
    
    async fn configure(&self, config: PwmConfig) -> AppResult<()>;
    
    async fn enable_channel(&self, chip: u32, channel: u32, enabled: bool) -> AppResult<()>;
    
    async fn set_frequency(&self, chip: u32, channel: u32, frequency: f64) -> AppResult<()>;
    
    async fn set_duty_cycle(&self, chip: u32, channel: u32, duty_cycle: f64) -> AppResult<()>;
}
