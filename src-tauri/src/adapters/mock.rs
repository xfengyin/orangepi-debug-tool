use std::collections::VecDeque;
use std::sync::Arc;
use async_trait::async_trait;
use parking_lot::Mutex;
use uuid::Uuid;

use crate::adapters::serial::{SerialAdapter, SerialConfig, SerialHandle, SerialPortInfo};

pub struct MockSerialAdapter {
    connections: Arc<Mutex<Vec<(SerialHandle, VecDeque<u8>)>>>,
    ports: Arc<Mutex<Vec<SerialPortInfo>>>,
}

impl MockSerialAdapter {
    pub fn new() -> Self {
        let default_ports = vec![
            SerialPortInfo {
                name: "/dev/ttyUSB0".to_string(),
                port_type: "USB".to_string(),
                manufacturer: Some("FTDI".to_string()),
                serial_number: Some("FT123456".to_string()),
                product_identifier: Some(0x6001),
                vendor_identifier: Some(0x0403),
                baudrate: Some(115200),
            },
            SerialPortInfo {
                name: "/dev/ttyUSB1".to_string(),
                port_type: "USB".to_string(),
                manufacturer: Some("Silicon Labs".to_string()),
                serial_number: Some("SIL123456".to_string()),
                product_identifier: Some(0xea60),
                vendor_identifier: Some(0x10c4),
                baudrate: Some(115200),
            },
            SerialPortInfo {
                name: "/dev/ttyACM0".to_string(),
                port_type: "USB".to_string(),
                manufacturer: Some("Arduino".to_string()),
                serial_number: Some("ARDUINO123".to_string()),
                product_identifier: Some(0x0043),
                vendor_identifier: Some(0x2341),
                baudrate: Some(9600),
            },
        ];

        Self {
            connections: Arc::new(Mutex::new(Vec::new())),
            ports: Arc::new(Mutex::new(default_ports)),
        }
    }

    pub fn with_ports(self, ports: Vec<SerialPortInfo>) -> Self {
        *self.ports.lock() = ports;
        self
    }

    pub fn add_virtual_port(&self, port_name: &str) {
        self.ports.lock().push(SerialPortInfo {
            name: port_name.to_string(),
            port_type: "Virtual".to_string(),
            manufacturer: Some("Virtual".to_string()),
            serial_number: None,
            product_identifier: None,
            vendor_identifier: None,
            baudrate: Some(115200),
        });
    }
}

impl Default for MockSerialAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SerialAdapter for MockSerialAdapter {
    fn id(&self) -> &'static str {
        "mock-serial"
    }

    async fn list_ports(&self) -> crate::adapters::serial::SerialResult<Vec<SerialPortInfo>> {
        Ok(self.ports.lock().clone())
    }

    async fn connect(&self, config: &SerialConfig) -> crate::adapters::serial::SerialResult<SerialHandle> {
        let handle_id = Uuid::new_v4().to_string();
        let handle = SerialHandle::new(handle_id, config.clone());
        
        self.connections.lock().push((handle.clone(), VecDeque::new()));
        
        Ok(handle)
    }

    async fn disconnect(&self, handle: &SerialHandle) -> crate::adapters::serial::SerialResult<()> {
        let mut connections = self.connections.lock();
        connections.retain(|(h, _)| h.id != handle.id);
        Ok(())
    }

    async fn read(&self, handle: &SerialHandle, buffer: &mut [u8]) -> crate::adapters::serial::SerialResult<usize> {
        let mut connections = self.connections.lock();
        
        if let Some((_, rx_buffer)) = connections.iter_mut().find(|(h, _)| h.id == handle.id) {
            if rx_buffer.is_empty() {
                return Ok(0);
            }
            
            let mut count = 0;
            for byte in buffer.iter_mut() {
                if let Some(b) = rx_buffer.pop_front() {
                    *byte = b;
                    count += 1;
                } else {
                    break;
                }
            }
            return Ok(count);
        }
        
        Err(crate::adapters::serial::SerialError::NotConnected)
    }

    async fn write(&self, handle: &SerialHandle, data: &[u8]) -> crate::adapters::serial::SerialResult<usize> {
        let mut connections = self.connections.lock();
        
        if let Some((_, rx_buffer)) = connections.iter_mut().find(|(h, _)| h.id == handle.id) {
            for &byte in data {
                rx_buffer.push_back(byte);
            }
            return Ok(data.len());
        }
        
        Err(crate::adapters::serial::SerialError::NotConnected)
    }
}
