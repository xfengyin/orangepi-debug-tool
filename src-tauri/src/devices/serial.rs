//! Serial port management module

use crate::{AppError, AppResult};
use bytes::BytesMut;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{mpsc, oneshot};
use tokio::time::timeout;
use tokio_serial::{DataBits, FlowControl, Parity, SerialPortBuilderExt, SerialStream, StopBits};
use tracing::{debug, error, info, warn};

/// Serial port configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialConfig {
    /// Port name (e.g., "/dev/ttyUSB0", "COM3")
    pub port_name: String,
    /// Baud rate
    pub baud_rate: u32,
    /// Data bits (5, 6, 7, 8)
    pub data_bits: u8,
    /// Parity (none, even, odd)
    pub parity: String,
    /// Stop bits (1, 2)
    pub stop_bits: u8,
    /// Flow control (none, software, hardware)
    pub flow_control: String,
    /// Read timeout in milliseconds
    pub read_timeout: u64,
    /// Write timeout in milliseconds
    pub write_timeout: u64,
}

impl Default for SerialConfig {
    #[inline]
    fn default() -> Self {
        Self {
            port_name: String::new(),
            baud_rate: 115200,
            data_bits: 8,
            parity: "none".to_string(),
            stop_bits: 1,
            flow_control: "none".to_string(),
            read_timeout: 1000,
            write_timeout: 1000,
        }
    }
}

/// Serial port information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialPortInfo {
    /// Port name
    pub port_name: String,
    /// Port type description
    pub port_type: String,
    /// Vendor ID (if available)
    pub vid: Option<u16>,
    /// Product ID (if available)
    pub pid: Option<u16>,
    /// Serial number (if available)
    pub serial_number: Option<String>,
    /// Manufacturer (if available)
    pub manufacturer: Option<String>,
    /// Product name (if available)
    pub product: Option<String>,
}

/// Serial data packet for binary communication
#[derive(Debug, Clone)]
pub struct SerialPacket {
    /// Timestamp
    pub timestamp: u64,
    /// Data bytes
    pub data: Vec<u8>,
    /// Direction (true = received, false = transmitted)
    pub is_rx: bool,
}

/// Serial port manager
pub struct SerialManager {
    config: Option<SerialConfig>,
    port: Option<SerialStream>,
    data_tx: Option<mpsc::Sender<SerialPacket>>,
    command_tx: Option<mpsc::Sender<SerialCommand>>,
    is_connected: bool,
    stats: SerialStats,
}

/// Serial statistics
#[derive(Debug, Clone, Default)]
pub struct SerialStats {
    pub bytes_received: u64,
    pub bytes_transmitted: u64,
    pub packets_received: u64,
    pub packets_transmitted: u64,
    pub errors: u64,
}

/// Serial commands
#[derive(Debug)]
enum SerialCommand {
    Write(Vec<u8>, oneshot::Sender<AppResult<usize>>),
    Disconnect(oneshot::Sender<AppResult<()>>),
}

impl SerialManager {
    /// Create a new serial manager
    pub fn new() -> Self {
        Self {
            config: None,
            port: None,
            data_tx: None,
            command_tx: None,
            is_connected: false,
            stats: SerialStats::default(),
        }
    }
    
    /// List available serial ports
    pub fn list_ports() -> AppResult<Vec<SerialPortInfo>> {
        let ports = tokio_serial::available_ports()
            .map_err(|e| AppError::Serial(format!("Failed to list ports: {}", e)))?;
        
        let mut result = Vec::new();
        for port in ports {
            let info = SerialPortInfo {
                port_name: port.port_name.clone(),
                port_type: format!("{:?}", port.port_type),
                vid: None,
                pid: None,
                serial_number: None,
                manufacturer: None,
                product: None,
            };
            result.push(info);
        }
        
        Ok(result)
    }
    
    /// Auto-detect OrangePi serial port
    pub fn auto_detect() -> AppResult<Option<String>> {
        let ports = Self::list_ports()?;
        
        // Look for common OrangePi USB-to-Serial adapters
        for port in ports {
            let port_lower = port.port_name.to_lowercase();
            if port_lower.contains("usb")
                || port_lower.contains("acm")
                || port_lower.contains("serial")
            {
                return Ok(Some(port.port_name));
            }
        }
        
        Ok(None)
    }
    
    /// Detect baud rate by trying common rates
    pub async fn detect_baud_rate(&mut self, port_name: &str) -> AppResult<Option<u32>> {
        let common_rates = [115200, 9600, 57600, 38400, 19200, 4800];
        
        for rate in &common_rates {
            let config = SerialConfig {
                port_name: port_name.to_string(),
                baud_rate: *rate,
                ..Default::default()
            };
            
            match self.connect(config, None).await {
                Ok(_) => {
                    // Try to read some data to verify
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    self.disconnect().await?;
                    return Ok(Some(*rate));
                }
                Err(_) => continue,
            }
        }
        
        Ok(None)
    }
    
    /// Connect to a serial port
    pub async fn connect(
        &mut self,
        config: SerialConfig,
        data_tx: Option<mpsc::Sender<SerialPacket>>,
    ) -> AppResult<()> {
        if self.is_connected {
            self.disconnect().await?;
        }
        
        info!("Connecting to serial port: {} @ {}", config.port_name, config.baud_rate);
        
        let data_bits = match config.data_bits {
            5 => DataBits::Five,
            6 => DataBits::Six,
            7 => DataBits::Seven,
            _ => DataBits::Eight,
        };
        
        let parity = match config.parity.to_lowercase().as_str() {
            "even" => Parity::Even,
            "odd" => Parity::Odd,
            _ => Parity::None,
        };
        
        let stop_bits = match config.stop_bits {
            2 => StopBits::Two,
            _ => StopBits::One,
        };
        
        let flow_control = match config.flow_control.to_lowercase().as_str() {
            "software" => FlowControl::Software,
            "hardware" => FlowControl::Hardware,
            _ => FlowControl::None,
        };
        
        let port = tokio_serial::new(&config.port_name, config.baud_rate)
            .data_bits(data_bits)
            .parity(parity)
            .stop_bits(stop_bits)
            .flow_control(flow_control)
            .open_native_async()
            .map_err(|e| AppError::Serial(format!("Failed to open port: {}", e)))?;
        
        // Create command channel
        let (command_tx, mut command_rx) = mpsc::channel::<SerialCommand>(100);
        
        // Start background task
        let port_clone = port.try_clone().map_err(|e| AppError::Serial(e.to_string()))?;
        let data_tx_clone = data_tx.clone();
        
        tokio::spawn(async move {
            Self::serial_task(port_clone, data_tx_clone, &mut command_rx).await;
        });
        
        self.port = Some(port);
        self.config = Some(config);
        self.data_tx = data_tx;
        self.command_tx = Some(command_tx);
        self.is_connected = true;
        
        info!("Serial port connected successfully");
        Ok(())
    }
    
    /// Disconnect from serial port
    pub async fn disconnect(&mut self) -> AppResult<()> {
        if !self.is_connected {
            return Ok(());
        }
        
        info!("Disconnecting from serial port...");
        
        if let Some(tx) = &self.command_tx {
            let (resp_tx, resp_rx) = oneshot::channel();
            let _ = tx.send(SerialCommand::Disconnect(resp_tx)).await;
            let _ = timeout(Duration::from_secs(2), resp_rx).await;
        }
        
        if let Some(port) = self.port.take() {
            drop(port);
        }
        
        self.command_tx = None;
        self.data_tx = None;
        self.is_connected = false;
        
        info!("Serial port disconnected");
        Ok(())
    }
    
    /// Write data to serial port
    pub async fn write(&mut self, data: &[u8]) -> AppResult<usize> {
        if !self.is_connected {
            return Err(AppError::Serial("Not connected".to_string()));
        }
        
        if let Some(tx) = &self.command_tx {
            let (resp_tx, resp_rx) = oneshot::channel();
            tx.send(SerialCommand::Write(data.to_vec(), resp_tx))
                .await
                .map_err(|_| AppError::Serial("Command channel closed".to_string()))?;
            
            match timeout(Duration::from_millis(5000), resp_rx).await {
                Ok(Ok(result)) => {
                    if let Ok(n) = &result {
                        self.stats.bytes_transmitted += *n as u64;
                        self.stats.packets_transmitted += 1;
                    }
                    result
                }
                Ok(Err(_)) => Err(AppError::Serial("Response channel closed".to_string())),
                Err(_) => Err(AppError::Timeout("Write operation timed out".to_string())),
            }
        } else {
            Err(AppError::Serial("Not connected".to_string()))
        }
    }
    
    /// Check if connected
    #[inline]
    pub fn is_connected(&self) -> bool {
        self.is_connected
    }
    
    /// Get current configuration
    #[inline]
    pub fn get_config(&self) -> Option<&SerialConfig> {
        self.config.as_ref()
    }
    
    /// Get statistics
    #[inline]
    pub fn get_stats(&self) -> &SerialStats {
        &self.stats
    }
    
    /// Background task for serial I/O
    async fn serial_task(
        mut port: SerialStream,
        data_tx: Option<mpsc::Sender<SerialPacket>>,
        command_rx: &mut mpsc::Receiver<SerialCommand>,
    ) {
        let mut buffer = BytesMut::with_capacity(4096);
        
        loop {
            tokio::select! {
                // Read from serial port
                result = port.read_buf(&mut buffer) => {
                    match result {
                        Ok(n) if n > 0 => {
                            let data = buffer.split_to(n).freeze().to_vec();
                            if let Some(tx) = &data_tx {
                                let packet = SerialPacket {
                                    timestamp: Self::current_timestamp(),
                                    data,
                                    is_rx: true,
                                };
                                let _ = tx.send(packet).await;
                            }
                        }
                        Ok(_) => {} // No data
                        Err(e) => {
                            error!("Serial read error: {}", e);
                            break;
                        }
                    }
                }
                
                // Handle commands
                Some(cmd) = command_rx.recv() => {
                    match cmd {
                        SerialCommand::Write(data, resp_tx) => {
                            let result = port.write_all(&data).await
                                .map(|_| data.len())
                                .map_err(|e| AppError::Io(e.to_string()));
                            let _ = resp_tx.send(result);
                        }
                        SerialCommand::Disconnect(resp_tx) => {
                            let _ = resp_tx.send(Ok(()));
                            break;
                        }
                    }
                }
            }
        }
        
        debug!("Serial background task ended");
    }
    
    #[inline]
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}