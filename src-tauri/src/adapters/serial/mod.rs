pub mod native;

pub use native::NativeSerialAdapter;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::adapters::network::DeviceError;
use crate::adapters::network::DeviceResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialPortInfo {
    pub name: String,
    pub port_type: String,
    pub manufacturer: Option<String>,
    pub serial_number: Option<String>,
    pub product_identifier: Option<u16>,
    pub vendor_identifier: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialConfig {
    pub port_name: String,
    pub baudrate: u32,
    pub data_bits: u8,
    pub stop_bits: u8,
    pub parity: String,
}

impl Default for SerialConfig {
    fn default() -> Self {
        Self {
            port_name: String::new(),
            baudrate: 115200,
            data_bits: 8,
            stop_bits: 1,
            parity: "none".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SerialHandle {
    pub id: String,
    pub port_name: String,
    pub config: SerialConfig,
}

#[async_trait]
pub trait SerialAdapter: Send + Sync {
    async fn list_ports(&self) -> DeviceResult<Vec<SerialPortInfo>>;
    async fn connect(&self, config: &SerialConfig) -> DeviceResult<SerialHandle>;
    async fn disconnect(&self, handle: &SerialHandle) -> DeviceResult<()>;
    async fn read(&self, handle: &SerialHandle, buffer: &mut [u8]) -> DeviceResult<usize>;
    async fn write(&self, handle: &SerialHandle, data: &[u8]) -> DeviceResult<usize>;
}
