use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use parking_lot::Mutex;
use tokio::time::{timeout, Duration};
use tracing::{debug, error, info, warn};

use crate::adapters::{DeviceAdapterRegistry, SerialAdapter, SerialConfig, SerialPortInfo};
use crate::config::SerialDeviceConfig;
use crate::error::{AppError, AppResult};
use crate::observability::{MetricsCollector, SerialMetric};

#[derive(Debug, Clone)]
pub struct SerialConnection {
    pub id: String,
    pub config: SerialConfig,
    pub port_info: SerialPortInfo,
    pub connected_at: chrono::DateTime<chrono::Utc>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

pub struct SerialService {
    registry: Arc<Mutex<DeviceAdapterRegistry>>,
    config: SerialDeviceConfig,
    connections: Arc<RwLock<HashMap<String, SerialConnection>>>,
    metrics: Arc<MetricsCollector>,
    data_callbacks: Arc<RwLock<Vec<mpsc::Sender<SerialData>>>>,
}

#[derive(Debug, Clone)]
pub struct SerialData {
    pub connection_id: String,
    pub data: Vec<u8>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl SerialService {
    pub fn new(registry: Arc<Mutex<DeviceAdapterRegistry>>, config: &SerialDeviceConfig) -> Self {
        let metrics = Arc::new(MetricsCollector::new());
        metrics.register_counter("serial_bytes_received", "Total bytes received");
        metrics.register_counter("serial_bytes_transmitted", "Total bytes transmitted");
        metrics.register_counter("serial_errors", "Total serial errors");
        metrics.register_histogram("serial_read_latency_ms", "Serial read latency in milliseconds");

        Self {
            registry,
            config: config.clone(),
            connections: Arc::new(RwLock::new(HashMap::new())),
            metrics,
            data_callbacks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn initialize(&self) -> AppResult<()> {
        info!("Initializing SerialService with config: {:?}", self.config);
        
        if let Some(adapter) = self.registry.lock().get_default_serial() {
            let ports = adapter.list_ports().await
                .map_err(|e| AppError::Serial(format!("Failed to list ports: {}", e)))?;
            info!("Found {} serial ports", ports.len());
        }
        
        Ok(())
    }

    pub async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down SerialService");
        
        let mut connections = self.connections.write().await;
        for (id, _) in connections.iter() {
            info!("Closing connection: {}", id);
        }
        connections.clear();
        
        Ok(())
    }

    pub async fn list_ports(&self) -> AppResult<Vec<SerialPortInfo>> {
        let adapter = self.registry.lock().get_default_serial()
            .ok_or_else(|| AppError::NotFound("No serial adapter available".to_string()))?;
        
        adapter.list_ports().await
            .map_err(|e| AppError::Serial(format!("Failed to list ports: {}", e)))
    }

    pub async fn connect(&self, port_name: &str, baud_rate: u32) -> AppResult<String> {
        if !self.config.supported_baudrates.contains(&baud_rate) {
            return Err(AppError::InvalidArgument(
                format!("Unsupported baud rate: {}. Supported: {:?}", baud_rate, self.config.supported_baudrates)
            ));
        }

        let adapter = self.registry.lock().get_default_serial()
            .ok_or_else(|| AppError::NotFound("No serial adapter available".to_string()))?;

        let ports = adapter.list_ports().await
            .map_err(|e| AppError::Serial(format!("Failed to list ports: {}", e)))?;
        
        let port_info = ports.into_iter()
            .find(|p| p.port_name == port_name)
            .ok_or_else(|| AppError::NotFound(format!("Port not found: {}", port_name)))?;

        let config = SerialConfig {
            port_name: port_name.to_string(),
            baud_rate,
            data_bits: 8,
            parity: "none".to_string(),
            stop_bits: 1,
            flow_control: "none".to_string(),
            read_timeout_ms: self.config.read_timeout_ms,
            write_timeout_ms: self.config.write_timeout_ms,
        };

        let connection_id = uuid::Uuid::new_v4().to_string();
        
        let connection = SerialConnection {
            id: connection_id.clone(),
            config: config.clone(),
            port_info: port_info.clone(),
            connected_at: chrono::Utc::now(),
            bytes_sent: 0,
            bytes_received: 0,
        };

        self.connections.write().await.insert(connection_id.clone(), connection);
        
        self.metrics.increment("serial_connections");
        info!("Connected to {} at {} baud", port_name, baud_rate);
        
        Ok(connection_id)
    }

    pub async fn disconnect(&self, connection_id: &str) -> AppResult<()> {
        let removed = self.connections.write().await.remove(connection_id);
        
        if removed.is_some() {
            self.metrics.increment("serial_disconnections");
            info!("Disconnected: {}", connection_id);
            Ok(())
        } else {
            Err(AppError::NotFound(format!("Connection not found: {}", connection_id)))
        }
    }

    pub async fn send(&self, connection_id: &str, data: &[u8]) -> AppResult<usize> {
        let adapter = self.registry.lock().get_default_serial()
            .ok_or_else(|| AppError::NotFound("No serial adapter available".to_string()))?;

        let mut connections = self.connections.write().await;
        let connection = connections.get_mut(connection_id)
            .ok_or_else(|| AppError::NotFound(format!("Connection not found: {}", connection_id)))?;

        let start = std::time::Instant::now();
        
        let bytes_written = adapter.write_internal(&connection.config, data).await
            .map_err(|e| {
                self.metrics.increment("serial_errors");
                AppError::Serial(format!("Write failed: {}", e))
            })?;

        let elapsed = start.elapsed().as_millis() as f64;
        self.metrics.observe_histogram("serial_write_latency_ms", elapsed);
        self.metrics.add_counter("serial_bytes_transmitted", bytes_written as u64);
        
        connection.bytes_sent += bytes_written as u64;
        debug!("Sent {} bytes to {}", bytes_written, connection_id);
        
        Ok(bytes_written)
    }

    pub async fn receive(&self, connection_id: &str, buffer: &mut [u8], timeout_ms: u64) -> AppResult<usize> {
        let adapter = self.registry.lock().get_default_serial()
            .ok_or_else(|| AppError::NotFound("No serial adapter available".to_string()))?;

        let mut connections = self.connections.write().await;
        let connection = connections.get_mut(connection_id)
            .ok_or_else(|| AppError::NotFound(format!("Connection not found: {}", connection_id)))?;

        let start = std::time::Instant::now();
        
        let result = timeout(
            Duration::from_millis(timeout_ms),
            adapter.read_internal(&connection.config, buffer)
        ).await;

        match result {
            Ok(Ok(bytes_read)) => {
                let elapsed = start.elapsed().as_millis() as f64;
                self.metrics.observe_histogram("serial_read_latency_ms", elapsed);
                self.metrics.add_counter("serial_bytes_received", bytes_read as u64);
                
                connection.bytes_received += bytes_read as u64;
                debug!("Received {} bytes from {}", bytes_read, connection_id);
                
                Ok(bytes_read)
            }
            Ok(Err(e)) => {
                self.metrics.increment("serial_errors");
                Err(AppError::Serial(format!("Read failed: {}", e)))
            }
            Err(_) => {
                Ok(0)
            }
        }
    }

    pub async fn set_baudrate(&self, connection_id: &str, baud_rate: u32) -> AppResult<()> {
        if !self.config.supported_baudrates.contains(&baud_rate) {
            return Err(AppError::InvalidArgument(
                format!("Unsupported baud rate: {}", baud_rate)
            ));
        }

        let adapter = self.registry.lock().get_default_serial()
            .ok_or_else(|| AppError::NotFound("No serial adapter available".to_string()))?;

        let mut connections = self.connections.write().await;
        let connection = connections.get_mut(connection_id)
            .ok_or_else(|| AppError::NotFound(format!("Connection not found: {}", connection_id)))?;

        adapter.set_baudrate_internal(&connection.config, baud_rate).await
            .map_err(|e| AppError::Serial(format!("Failed to set baudrate: {}", e)))?;
        
        connection.config.baud_rate = baud_rate;
        info!("Changed baudrate to {} for {}", baud_rate, connection_id);
        
        Ok(())
    }

    pub async fn get_connection(&self, connection_id: &str) -> Option<SerialConnection> {
        self.connections.read().await.get(connection_id).cloned()
    }

    pub async fn list_connections(&self) -> Vec<SerialConnection> {
        self.connections.read().await.values().cloned().collect()
    }

    pub fn get_metrics(&self) -> SerialMetrics {
        SerialMetrics {
            active_connections: self.connections.blocking_read().len() as u64,
        }
    }

    pub async fn subscribe_data<F>(&self, callback: F) -> AppResult<()>
    where
        F: Fn(SerialData) + Send + 'static,
    {
        let (tx, mut rx) = mpsc::channel(1000);
        
        self.data_callbacks.write().await.push(tx);
        
        tokio::spawn(async move {
            while let Some(data) = rx.recv().await {
                callback(data);
            }
        });
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SerialMetrics {
    pub active_connections: u64,
}

#[async_trait]
trait SerialAdapterExt: Send + Sync {
    async fn write_internal(&self, config: &SerialConfig, data: &[u8]) -> AppResult<usize>;
    async fn read_internal(&self, config: &SerialConfig, buffer: &mut [u8]) -> AppResult<usize>;
    async fn set_baudrate_internal(&self, config: &SerialConfig, baudrate: u32) -> AppResult<()>;
}

#[async_trait]
impl SerialAdapterExt for Arc<dyn SerialAdapter> {
    async fn write_internal(&self, config: &SerialConfig, data: &[u8]) -> AppResult<usize> {
        let handle = crate::adapters::SerialHandle {
            port_name: config.port_name.clone(),
            config: config.clone(),
            #[cfg(feature = "hardware-support")]
            stream: unsafe { std::mem::zeroed() },
        };
        (**self).write(&handle, data).await
    }

    async fn read_internal(&self, config: &SerialConfig, buffer: &mut [u8]) -> AppResult<usize> {
        let handle = crate::adapters::SerialHandle {
            port_name: config.port_name.clone(),
            config: config.clone(),
            #[cfg(feature = "hardware-support")]
            stream: unsafe { std::mem::zeroed() },
        };
        (**self).read(&handle, buffer).await
    }

    async fn set_baudrate_internal(&self, config: &SerialConfig, baudrate: u32) -> AppResult<()> {
        let handle = crate::adapters::SerialHandle {
            port_name: config.port_name.clone(),
            config: config.clone(),
            #[cfg(feature = "hardware-support")]
            stream: unsafe { std::mem::zeroed() },
        };
        (**self).set_baudrate(&handle, baudrate).await
    }
}
