use std::collections::HashMap;
use std::net::{SocketAddr,IpAddr};
use std::sync::Arc;
use async_trait::async_trait;
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::adapters::network::{
    DeviceError, DeviceResult, NetworkAdapter, TcpServerConfig, TcpHandle, 
    UdpHandle, TcpServerHandle,
};
use uuid::Uuid;
use tokio::sync::Mutex as AsyncMutex;

pub struct TokioNetworkAdapter {
    tcp_connections: Arc<AsyncMutex<HashMap<String, Arc<AsyncMutex<Option<TcpStream>>>>>>,
    tcp_servers: Arc<AsyncMutex<HashMap<String, Arc<AsyncMutex<TcpListener>>>>>,
    udp_sockets: Arc<AsyncMutex<HashMap<String, Arc<AsyncMutex<Option<UdpSocket>>>>>>,
}

impl TokioNetworkAdapter {
    pub fn new() -> Self {
        Self {
            tcp_connections: Arc::new(AsyncMutex::new(HashMap::new())),
            tcp_servers: Arc::new(AsyncMutex::new(HashMap::new())),
            udp_sockets: Arc::new(AsyncMutex::new(HashMap::new())),
        }
    }
}

impl Default for TokioNetworkAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NetworkAdapter for TokioNetworkAdapter {
    async fn create_tcp_server(&self, config: TcpServerConfig) -> DeviceResult<TcpServerHandle> {
        let addr: SocketAddr = format!("0.0.0.0:{}", config.port)
            .parse()
            .map_err(|e: std::net::AddrParseError| DeviceError::ConnectionFailed(e.to_string()))?;

        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| DeviceError::ConnectionFailed(e.to_string()))?;

        let id = Uuid::new_v4().to_string();
        
        self.tcp_servers.lock().await.insert(id.clone(), Arc::new(AsyncMutex::new(listener)));

        Ok(TcpServerHandle {
            id,
            local_addr: addr,
        })
    }

    async fn close_tcp_server(&self, handle: &TcpServerHandle) -> DeviceResult<()> {
        self.tcp_servers.lock().await.remove(&handle.id);
        Ok(())
    }

    async fn accept_tcp_connection(&self, handle: &TcpServerHandle) -> DeviceResult<TcpHandle> {
        let listener_arc = {
            let servers = self.tcp_servers.lock().await;
            servers.get(&handle.id).cloned()
        }.ok_or(DeviceError::NotConnected)?;

        let listener = listener_arc.lock().await;
        let (stream, addr) = listener.accept()
            .await
            .map_err(|e| DeviceError::AcceptFailed(e.to_string()))?;
        drop(listener);

        let id = Uuid::new_v4().to_string();
        self.tcp_connections.lock().await.insert(id.clone(), Arc::new(AsyncMutex::new(Some(stream))));

        Ok(TcpHandle {
            id,
            addr,
        })
    }

    async fn connect_tcp(&self, addr: &str, port: u16) -> DeviceResult<TcpHandle> {
        let socket_addr: SocketAddr = format!("{}:{}", addr, port)
            .parse()
            .map_err(|e: std::net::AddrParseError| DeviceError::ConnectionFailed(e.to_string()))?;

        let stream = TcpStream::connect(socket_addr)
            .await
            .map_err(|e| DeviceError::ConnectionFailed(e.to_string()))?;

        let local_addr = stream.local_addr()
            .map_err(|e| DeviceError::ConnectionFailed(e.to_string()))?;

        let id = Uuid::new_v4().to_string();
        self.tcp_connections.lock().await.insert(id.clone(), Arc::new(AsyncMutex::new(Some(stream))));

        Ok(TcpHandle {
            id,
            addr: local_addr,
        })
    }

    async fn disconnect_tcp(&self, handle: &TcpHandle) -> DeviceResult<()> {
        self.tcp_connections.lock().await.remove(&handle.id);
        Ok(())
    }

    async fn send_tcp(&self, handle: &TcpHandle, data: &[u8]) -> DeviceResult<()> {
        let stream_arc = {
            let connections = self.tcp_connections.lock().await;
            connections.get(&handle.id).cloned()
        }.ok_or(DeviceError::NotConnected)?;

        let mut stream_guard = stream_arc.lock().await;
        if let Some(ref mut stream) = *stream_guard {
            stream.write_all(data).await
                .map_err(|e| DeviceError::SendFailed(e.to_string()))?;
        }
        Ok(())
    }

    async fn receive_tcp(&self, handle: &TcpHandle, buffer: &mut [u8]) -> DeviceResult<usize> {
        let stream_arc = {
            let connections = self.tcp_connections.lock().await;
            connections.get(&handle.id).cloned()
        }.ok_or(DeviceError::NotConnected)?;

        let mut stream_guard = stream_arc.lock().await;
        if let Some(ref mut stream) = *stream_guard {
            let n = stream.read(buffer).await
                .map_err(|e| DeviceError::ReceiveFailed(e.to_string()))?;
            return Ok(n);
        }
        Err(DeviceError::NotConnected)
    }

    async fn connect_udp(&self, local_port: u16, remote_addr: &str, remote_port: u16) -> DeviceResult<UdpHandle> {
        let local_socket_addr: SocketAddr = SocketAddr::new(IpAddr::from([0, 0, 0, 0]), local_port);

        let remote_socket_addr: SocketAddr = format!("{}:{}", remote_addr, remote_port)
            .parse()
            .map_err(|e: std::net::AddrParseError| DeviceError::ConnectionFailed(e.to_string()))?;

        let socket = UdpSocket::bind(local_socket_addr)
            .await
            .map_err(|e| DeviceError::ConnectionFailed(e.to_string()))?;

        let id = Uuid::new_v4().to_string();
        self.udp_sockets.lock().await.insert(id.clone(), Arc::new(AsyncMutex::new(Some(socket))));

        Ok(UdpHandle {
            id,
            local_addr: local_socket_addr,
            remote_addr: remote_socket_addr,
        })
    }

    async fn disconnect_udp(&self, handle: &UdpHandle) -> DeviceResult<()> {
        self.udp_sockets.lock().await.remove(&handle.id);
        Ok(())
    }

    async fn send_udp(&self, handle: &UdpHandle, data: &[u8], addr: &str, port: u16) -> DeviceResult<()> {
        let socket_arc = {
            let sockets = self.udp_sockets.lock().await;
            sockets.get(&handle.id).cloned()
        }.ok_or(DeviceError::NotConnected)?;

        let remote_addr: SocketAddr = format!("{}:{}", addr, port)
            .parse()
            .map_err(|e: std::net::AddrParseError| DeviceError::SendFailed(e.to_string()))?;

        let mut socket_guard = socket_arc.lock().await;
        if let Some(ref mut socket) = *socket_guard {
            socket.send_to(data, remote_addr).await
                .map_err(|e| DeviceError::SendFailed(e.to_string()))?;
        }
        Ok(())
    }

    async fn receive_udp(&self, handle: &UdpHandle, buffer: &mut [u8]) -> DeviceResult<(usize, String, u16)> {
        let socket_arc = {
            let sockets = self.udp_sockets.lock().await;
            sockets.get(&handle.id).cloned()
        }.ok_or(DeviceError::NotConnected)?;

        let mut socket_guard = socket_arc.lock().await;
        if let Some(ref mut socket) = *socket_guard {
            let (n, from_addr) = socket.recv_from(buffer).await
                .map_err(|e| DeviceError::ReceiveFailed(e.to_string()))?;

            let from_str = from_addr.ip().to_string();
            let from_port = from_addr.port();

            return Ok((n, from_str, from_port));
        }
        Err(DeviceError::NotConnected)
    }
}
