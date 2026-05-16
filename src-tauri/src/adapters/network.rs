use std::net::SocketAddr;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Not connected")]
    NotConnected,
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Send failed: {0}")]
    SendFailed(String),
    #[error("Receive failed: {0}")]
    ReceiveFailed(String),
    #[error("Accept failed: {0}")]
    AcceptFailed(String),
}

pub type DeviceResult<T> = Result<T, DeviceError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpServerConfig {
    pub port: u16,
    pub max_connections: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpHandle {
    pub id: String,
    pub addr: SocketAddr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UdpHandle {
    pub id: String,
    pub local_addr: SocketAddr,
    pub remote_addr: SocketAddr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpServerHandle {
    pub id: String,
    pub local_addr: SocketAddr,
}

#[async_trait]
pub trait NetworkAdapter: Send + Sync {
    async fn create_tcp_server(&self, config: TcpServerConfig) -> DeviceResult<TcpServerHandle>;
    async fn close_tcp_server(&self, handle: &TcpServerHandle) -> DeviceResult<()>;
    async fn accept_tcp_connection(&self, handle: &TcpServerHandle) -> DeviceResult<TcpHandle>;
    
    async fn connect_tcp(&self, addr: &str, port: u16) -> DeviceResult<TcpHandle>;
    async fn disconnect_tcp(&self, handle: &TcpHandle) -> DeviceResult<()>;
    async fn send_tcp(&self, handle: &TcpHandle, data: &[u8]) -> DeviceResult<()>;
    async fn receive_tcp(&self, handle: &TcpHandle, buffer: &mut [u8]) -> DeviceResult<usize>;
    
    async fn connect_udp(&self, local_port: u16, remote_addr: &str, remote_port: u16) -> DeviceResult<UdpHandle>;
    async fn disconnect_udp(&self, handle: &UdpHandle) -> DeviceResult<()>;
    async fn send_udp(&self, handle: &UdpHandle, data: &[u8], addr: &str, port: u16) -> DeviceResult<()>;
    async fn receive_udp(&self, handle: &UdpHandle, buffer: &mut [u8]) -> DeviceResult<(usize, String, u16)>;
}
