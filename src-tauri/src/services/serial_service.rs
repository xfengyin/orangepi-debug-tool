use crate::adapters::serial::{SerialAdapter, SerialConfig, SerialHandle, SerialPortInfo};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{mpsc, RwLock};

#[derive(Error, Debug)]
pub enum SerialServiceError {
    #[error("Not connected")]
    NotConnected,
    #[error("Send failed: {0}")]
    SendFailed(String),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
}

pub type SerialServiceResult<T> = Result<T, SerialServiceError>;

#[derive(Debug, Clone)]
pub struct SerialStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub errors: u64,
}

struct SerialConnection {
    handle: SerialHandle,
    receiver: mpsc::Receiver<Vec<u8>>,
}

pub struct SerialService {
    adapter: Arc<dyn SerialAdapter>,
    connections: Arc<RwLock<HashMap<String, SerialConnection>>>,
    stats: Arc<RwLock<HashMap<String, SerialStats>>>,
    timing_senders: Arc<RwLock<HashMap<String, tokio::sync::oneshot::Sender<()>>>>,
}

impl SerialService {
    pub fn new(adapter: Arc<dyn SerialAdapter>) -> Self {
        Self {
            adapter,
            connections: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(HashMap::new())),
            timing_senders: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn list_ports(&self) -> SerialServiceResult<Vec<SerialPortInfo>> {
        self.adapter
            .list_ports()
            .await
            .map_err(|e| SerialServiceError::ConnectionFailed(e.to_string()))
    }

    pub async fn connect(&self, _port_name: String, config: SerialConfig) -> SerialServiceResult<String> {
        let handle = self
            .adapter
            .connect(&config)
            .await
            .map_err(|e| SerialServiceError::ConnectionFailed(e.to_string()))?;

        let id = handle.id.clone();
        let (_tx, rx) = mpsc::channel(100);

        self.connections.write().await.insert(
            id.clone(),
            SerialConnection {
                handle,
                receiver: rx,
            },
        );

        self.stats.write().await.insert(
            id.clone(),
            SerialStats {
                bytes_sent: 0,
                bytes_received: 0,
                packets_sent: 0,
                packets_received: 0,
                errors: 0,
            },
        );

        Ok(id)
    }

    pub async fn disconnect(&self, id: &str) -> SerialServiceResult<()> {
        if let Some(stop_tx) = self.timing_senders.write().await.remove(id) {
            let _ = stop_tx.send(());
        }

        if let Some(conn) = self.connections.write().await.remove(id) {
            self.adapter
                .disconnect(&conn.handle)
                .await
                .map_err(|e| SerialServiceError::ConnectionFailed(e.to_string()))?;
        }

        self.stats.write().await.remove(id);
        Ok(())
    }

    pub async fn send(&self, id: &str, data: &[u8]) -> SerialServiceResult<()> {
        let guard = self.connections.read().await;
        let conn = guard.get(id).ok_or(SerialServiceError::NotConnected)?;
        let handle = conn.handle.clone();
        drop(guard);

        self.adapter
            .write(&handle, data)
            .await
            .map_err(|e| SerialServiceError::SendFailed(e.to_string()))?;

        let mut stats = self.stats.write().await;
        if let Some(s) = stats.get_mut(id) {
            s.bytes_sent += data.len() as u64;
            s.packets_sent += 1;
        }

        Ok(())
    }

    pub async fn read(&self, id: &str, buffer: &mut [u8]) -> SerialServiceResult<usize> {
        let guard = self.connections.read().await;
        let conn = guard.get(id).ok_or(SerialServiceError::NotConnected)?;
        let handle = conn.handle.clone();
        drop(guard);

        let n = self
            .adapter
            .read(&handle, buffer)
            .await
            .map_err(|e| SerialServiceError::SendFailed(e.to_string()))?;

        let mut stats = self.stats.write().await;
        if let Some(s) = stats.get_mut(id) {
            s.bytes_received += n as u64;
            s.packets_received += 1;
        }

        Ok(n)
    }

    pub async fn start_timing_send(
        &self,
        id: &str,
        data: Vec<u8>,
        interval_ms: u64,
    ) -> SerialServiceResult<()> {
        if self.connections.read().await.get(id).is_none() {
            return Err(SerialServiceError::NotConnected);
        }

        let (stop_tx, mut stop_rx) = tokio::sync::oneshot::channel();

        let service = self.clone();
        let id_clone = id.to_string();
        let data_clone = data.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(interval_ms));

            loop {
                tokio::select! {
                    _ = &mut stop_rx => break,
                    _ = interval.tick() => {
                        if let Err(e) = service.send(&id_clone, &data_clone).await {
                            tracing::error!("Timing send error: {}", e);
                        }
                    }
                }
            }
        });

        self.timing_senders
            .write()
            .await
            .insert(id.to_string(), stop_tx);
        Ok(())
    }

    pub async fn stop_timing_send(&self, id: &str) -> SerialServiceResult<()> {
        if let Some(stop_tx) = self.timing_senders.write().await.remove(id) {
            let _ = stop_tx.send(());
        }
        Ok(())
    }

    pub async fn get_stats(&self, id: &str) -> SerialServiceResult<SerialStats> {
        self.stats
            .read()
            .await
            .get(id)
            .cloned()
            .ok_or(SerialServiceError::NotConnected)
    }

    pub async fn is_connected(&self, id: &str) -> bool {
        self.connections.read().await.contains_key(id)
    }
}

impl Clone for SerialService {
    fn clone(&self) -> Self {
        Self {
            adapter: self.adapter.clone(),
            connections: self.connections.clone(),
            stats: self.stats.clone(),
            timing_senders: self.timing_senders.clone(),
        }
    }
}
