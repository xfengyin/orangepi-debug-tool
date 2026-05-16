#[cfg(feature = "native-serial")]
pub mod native;

#[cfg(feature = "native-serial")]
pub use native::NativeSerialAdapter;

pub mod buffer;
pub use buffer::CircularBuffer;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SerialError {
    #[error("Not connected")]
    NotConnected,
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Read error: {0}")]
    ReadError(String),
    #[error("Write error: {0}")]
    WriteError(String),
}

pub type SerialResult<T> = Result<T, SerialError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialPortInfo {
    pub name: String,
    pub port_type: String,
    pub manufacturer: Option<String>,
    pub serial_number: Option<String>,
    pub product_identifier: Option<u16>,
    pub vendor_identifier: Option<u16>,
    pub baudrate: Option<u32>,
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

impl SerialHandle {
    pub fn new(id: String, config: SerialConfig) -> Self {
        Self {
            id,
            port_name: config.port_name.clone(),
            config,
        }
    }
}

#[async_trait]
pub trait SerialAdapter: Send + Sync {
    fn id(&self) -> &'static str;
    async fn list_ports(&self) -> SerialResult<Vec<SerialPortInfo>>;
    async fn connect(&self, config: &SerialConfig) -> SerialResult<SerialHandle>;
    async fn disconnect(&self, handle: &SerialHandle) -> SerialResult<()>;
    async fn read(&self, handle: &SerialHandle, buffer: &mut [u8]) -> SerialResult<usize>;
    async fn write(&self, handle: &SerialHandle, data: &[u8]) -> SerialResult<usize>;
}
