use crate::adapters::serial::{SerialError, SerialResult, SerialHandle, SerialConfig, SerialPortInfo, SerialAdapter};
use async_trait::async_trait;
use parking_lot::Mutex;
use serialport::{DataBits, Parity, SerialPortType, StopBits, SerialPort};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, warn};

const BUFFER_SIZE: usize = 4096;

pub struct NativeSerialAdapter {
    connections: Arc<Mutex<HashMap<String, Arc<Mutex<Option<Box<dyn SerialPort + Send>>>>>>>,
    reconnect_enabled: Arc<Mutex<bool>>,
}

impl NativeSerialAdapter {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
            reconnect_enabled: Arc::new(Mutex::new(false)),
        }
    }

    pub fn enable_reconnect(&self, enabled: bool) {
        let mut reconnect = self.reconnect_enabled.lock();
        *reconnect = enabled;
    }

    fn convert_data_bits(bits: u8) -> DataBits {
        match bits {
            5 => DataBits::Five,
            6 => DataBits::Six,
            7 => DataBits::Seven,
            _ => DataBits::Eight,
        }
    }

    fn convert_stop_bits(bits: u8) -> StopBits {
        match bits {
            1 => StopBits::One,
            2 => StopBits::Two,
            _ => StopBits::One,
        }
    }

    fn convert_parity(parity: &str) -> Parity {
        match parity.to_lowercase().as_str() {
            "odd" => Parity::Odd,
            "even" => Parity::Even,
            _ => Parity::None,
        }
    }
}

impl Default for NativeSerialAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SerialAdapter for NativeSerialAdapter {
    fn id(&self) -> &'static str {
        "native-serial"
    }

    async fn list_ports(&self) -> SerialResult<Vec<SerialPortInfo>> {
        let ports = serialport::available_ports()
            .map_err(|e| SerialError::ConnectionFailed(e.to_string()))?;

        let port_infos: Vec<SerialPortInfo> = ports
            .into_iter()
            .map(|port| {
                let port_type = match port.port_type {
                    SerialPortType::UsbPort(_) => "USB".to_string(),
                    SerialPortType::PciPort => "PCI".to_string(),
                    SerialPortType::BluetoothPort => "Bluetooth".to_string(),
                    SerialPortType::Unknown => "Unknown".to_string(),
                };

                SerialPortInfo {
                    name: port.port_name,
                    port_type,
                    baudrate: 115200,
                }
            })
            .collect();

        info!("Found {} serial ports", port_infos.len());
        Ok(port_infos)
    }

    async fn connect(&self, config: &SerialConfig) -> SerialResult<SerialHandle> {
        let handle_id = uuid::Uuid::new_v4().to_string();
        info!("Connecting to serial port: {} at {} baud", config.port_name, config.baudrate);

        let port = serialport::new(&config.port_name, config.baudrate)
            .data_bits(Self::convert_data_bits(config.data_bits))
            .stop_bits(Self::convert_stop_bits(config.stop_bits))
            .parity(Self::convert_parity(&config.parity))
            .timeout(Duration::from_millis(100))
            .open()
            .map_err(|e| SerialError::ConnectionFailed(e.to_string()))?;

        let mut connections = self.connections.lock();
        connections.insert(handle_id.clone(), Arc::new(Mutex::new(Some(port))));
        drop(connections);

        let handle = SerialHandle::new(handle_id, config.clone());
        info!("Successfully connected to serial port");
        Ok(handle)
    }

    async fn disconnect(&self, handle: &SerialHandle) -> SerialResult<()> {
        let mut connections = self.connections.lock();
        if let Some(conn) = connections.remove(&handle.id) {
            let mut guard = conn.lock();
            *guard = None;
            info!("Disconnected from serial port: {}", handle.config.port_name);
        }
        Ok(())
    }

    async fn read(&self, handle: &SerialHandle, buffer: &mut [u8]) -> SerialResult<usize> {
        use std::io::Read;

        let connections = self.connections.lock();
        let conn = connections
            .get(&handle.id)
            .ok_or(SerialError::NotConnected)?;

        let mut guard = conn.lock();
        let port = guard
            .as_mut()
            .ok_or(SerialError::NotConnected)?;

        let bytes_read = port
            .read(buffer)
            .map_err(|e| SerialError::ReadError(e.to_string()))?;

        drop(guard);
        drop(connections);

        Ok(bytes_read)
    }

    async fn write(&self, handle: &SerialHandle, data: &[u8]) -> SerialResult<usize> {
        use std::io::Write;

        let connections = self.connections.lock();
        let conn = connections
            .get(&handle.id)
            .ok_or(SerialError::NotConnected)?;

        let mut guard = conn.lock();
        let port = guard
            .as_mut()
            .ok_or(SerialError::NotConnected)?;

        let bytes_written = port
            .write(data)
            .map_err(|e| SerialError::WriteError(e.to_string()))?;

        port.flush()
            .map_err(|e| SerialError::WriteError(e.to_string()))?;

        drop(guard);
        drop(connections);

        Ok(bytes_written)
    }

    async fn set_baudrate(&self, _handle: &SerialHandle, _baudrate: u32) -> SerialResult<()> {
        warn!("set_baudrate not fully implemented for native adapter");
        Ok(())
    }
}

pub struct CircularBuffer {
    buffer: Vec<u8>,
    read_pos: usize,
    write_pos: usize,
    size: usize,
}

impl CircularBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![0u8; capacity],
            read_pos: 0,
            write_pos: 0,
            size: 0,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        for &byte in data {
            if self.size < self.buffer.len() {
                self.buffer[self.write_pos] = byte;
                self.write_pos = (self.write_pos + 1) % self.buffer.len();
                self.size += 1;
                written += 1;
            } else {
                break;
            }
        }
        written
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> usize {
        let mut bytes_read = 0;
        let to_read = std::cmp::min(buffer.len(), self.size);

        for i in 0..to_read {
            buffer[i] = self.buffer[self.read_pos];
            self.read_pos = (self.read_pos + 1) % self.buffer.len();
            self.size -= 1;
            bytes_read += 1;
        }

        bytes_read
    }

    pub fn available(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    pub fn clear(&mut self) {
        self.read_pos = 0;
        self.write_pos = 0;
        self.size = 0;
    }
}

impl Default for CircularBuffer {
    fn default() -> Self {
        Self::new(BUFFER_SIZE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_buffer_write_read() {
        let mut buffer = CircularBuffer::new(10);
        
        let data = [1, 2, 3, 4, 5];
        let written = buffer.write(&data);
        assert_eq!(written, 5);
        assert_eq!(buffer.available(), 5);

        let mut read_buffer = [0u8; 5];
        let read = buffer.read(&mut read_buffer);
        assert_eq!(read, 5);
        assert_eq!(&read_buffer[..], &[1, 2, 3, 4, 5]);
        assert_eq!(buffer.available(), 0);
    }

    #[test]
    fn test_circular_buffer_overflow() {
        let mut buffer = CircularBuffer::new(5);
        
        let data = [1, 2, 3, 4, 5, 6, 7];
        let written = buffer.write(&data);
        assert_eq!(written, 5);
        assert_eq!(buffer.available(), 5);
    }

    #[test]
    fn test_circular_buffer_wrap() {
        let mut buffer = CircularBuffer::new(5);
        
        buffer.write(&[1, 2, 3]);
        let mut read_buffer = [0u8; 2];
        buffer.read(&mut read_buffer);
        
        buffer.write(&[4, 5, 6]);
        
        let mut read_buffer = [0u8; 3];
        let read = buffer.read(&mut read_buffer);
        assert_eq!(read, 3);
    }
}
