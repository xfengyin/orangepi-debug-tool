use std::net::SocketAddr;
use crate::adapters::network::{NetworkAdapter, TcpServerConfig};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Error, Debug)]
pub enum NetworkServiceError {
    #[error("Not connected")]
    NotConnected,
    #[error("Server error: {0}")]
    ServerError(String),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Send failed: {0}")]
    SendFailed(String),
}

pub type NetworkServiceResult<T> = Result<T, NetworkServiceError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpConnectionInfo {
    pub id: String,
    pub addr: String,
    pub server_id: Option<String>,
    pub connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UdpSessionInfo {
    pub id: String,
    pub local_port: u16,
    pub remote_addr: String,
    pub connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpServerInfo {
    pub id: String,
    pub port: u16,
    pub active_connections: usize,
}

pub struct NetworkService {
    adapter: Arc<dyn NetworkAdapter>,
    tcp_servers: Arc<RwLock<HashMap<String, TcpServerInfo>>>,
    tcp_connections: Arc<RwLock<HashMap<String, TcpConnectionInfo>>>,
    udp_sessions: Arc<RwLock<HashMap<String, UdpSessionInfo>>>,
}

impl NetworkService {
    pub fn new(adapter: Arc<dyn NetworkAdapter>) -> Self {
        Self {
            adapter,
            tcp_servers: Arc::new(RwLock::new(HashMap::new())),
            tcp_connections: Arc::new(RwLock::new(HashMap::new())),
            udp_sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_tcp_server(&self, port: u16, max_connections: usize) -> NetworkServiceResult<String> {
        let config = TcpServerConfig {
            port,
            max_connections,
        };

        let handle = self
            .adapter
            .create_tcp_server(config)
            .await
            .map_err(|e| NetworkServiceError::ServerError(e.to_string()))?;

        let id = handle.id.clone();
        self.tcp_servers.write().await.insert(
            id.clone(),
            TcpServerInfo {
                id: id.clone(),
                port,
                active_connections: 0,
            },
        );

        Ok(id)
    }

    pub async fn close_tcp_server(&self, server_id: &str) -> NetworkServiceResult<()> {
        let servers = self.tcp_servers.read().await;
        let server_info = servers.get(server_id).ok_or(NetworkServiceError::ServerError("Server not found".to_string()))?;
        
        let handle = crate::adapters::network::TcpServerHandle {
            id: server_info.id.clone(),
            local_addr: "0.0.0.0:0".parse().unwrap(),
        };
        drop(servers);

        self.adapter
            .close_tcp_server(&handle)
            .await
            .map_err(|e| NetworkServiceError::ServerError(e.to_string()))?;

        self.tcp_servers.write().await.remove(server_id);
        Ok(())
    }

    pub async fn accept_tcp_connection(&self, server_id: &str) -> NetworkServiceResult<String> {
        let servers = self.tcp_servers.read().await;
        let server_info = servers.get(server_id).ok_or(NetworkServiceError::ServerError("Server not found".to_string()))?;
        
        let handle = crate::adapters::network::TcpServerHandle {
            id: server_info.id.clone(),
            local_addr: "0.0.0.0:0".parse().unwrap(),
        };
        drop(servers);

        let connection = self
            .adapter
            .accept_tcp_connection(&handle)
            .await
            .map_err(|e| NetworkServiceError::ServerError(e.to_string()))?;

        let id = connection.id.clone();
        self.tcp_connections.write().await.insert(
            id.clone(),
            TcpConnectionInfo {
                id: id.clone(),
                addr: connection.addr.to_string(),
                server_id: Some(server_id.to_string()),
                connected: true,
            },
        );

        if let Some(server) = self.tcp_servers.write().await.get_mut(server_id) {
            server.active_connections += 1;
        }

        Ok(id)
    }

    pub async fn connect_tcp(&self, addr: &str, port: u16) -> NetworkServiceResult<String> {
        let handle = self
            .adapter
            .connect_tcp(addr, port)
            .await
            .map_err(|e| NetworkServiceError::ConnectionFailed(e.to_string()))?;

        let id = handle.id.clone();
        self.tcp_connections.write().await.insert(
            id.clone(),
            TcpConnectionInfo {
                id: id.clone(),
                addr: handle.addr.to_string(),
                server_id: None,
                connected: true,
            },
        );

        Ok(id)
    }

    pub async fn disconnect_tcp(&self, id: &str) -> NetworkServiceResult<()> {
        let guard = self.tcp_connections.read().await;
        let conn = guard.get(id).cloned();
        drop(guard);
        
        if let Some(conn_info) = conn {
            let handle = crate::adapters::network::TcpHandle {
                id: id.to_string(),
                addr: conn_info.addr.parse().unwrap_or_else(|_| "0.0.0.0:0".parse().unwrap()),
            };
            
            self.adapter
                .disconnect_tcp(&handle)
                .await
                .map_err(|e| NetworkServiceError::ConnectionFailed(e.to_string()))?;

            if let Some(server_id) = &conn_info.server_id {
                if let Some(server) = self.tcp_servers.write().await.get_mut(server_id) {
                    server.active_connections = server.active_connections.saturating_sub(1);
                }
            }
        }

        self.tcp_connections.write().await.remove(id);
        Ok(())
    }

    pub async fn send_tcp(&self, id: &str, data: &[u8]) -> NetworkServiceResult<()> {
        let guard = self.tcp_connections.read().await;
        let conn = guard.get(id).ok_or(NetworkServiceError::NotConnected)?;
        let addr_str = conn.addr.clone();
        drop(guard);

        let handle = crate::adapters::network::TcpHandle {
            id: id.to_string(),
            addr: addr_str.parse().unwrap_or_else(|_| "0.0.0.0:0".parse().unwrap()),
        };

        self.adapter
            .send_tcp(&handle, data)
            .await
            .map_err(|e| NetworkServiceError::SendFailed(e.to_string()))
    }

    pub async fn receive_tcp(&self, id: &str, buffer: &mut [u8]) -> NetworkServiceResult<usize> {
        let guard = self.tcp_connections.read().await;
        let conn = guard.get(id).ok_or(NetworkServiceError::NotConnected)?;
        let addr_str = conn.addr.clone();
        drop(guard);

        let handle = crate::adapters::network::TcpHandle {
            id: id.to_string(),
            addr: addr_str.parse().unwrap_or_else(|_| "0.0.0.0:0".parse().unwrap()),
        };

        self.adapter
            .receive_tcp(&handle, buffer)
            .await
            .map_err(|e| NetworkServiceError::SendFailed(e.to_string()))
    }

    pub async fn connect_udp(&self, local_port: u16, remote_addr: &str, remote_port: u16) -> NetworkServiceResult<String> {
        let handle = self
            .adapter
            .connect_udp(local_port, remote_addr, remote_port)
            .await
            .map_err(|e| NetworkServiceError::ConnectionFailed(e.to_string()))?;

        let id = handle.id.clone();
        self.udp_sessions.write().await.insert(
            id.clone(),
            UdpSessionInfo {
                id: id.clone(),
                local_port,
                remote_addr: format!("{}:{}", remote_addr, remote_port),
                connected: true,
            },
        );

        Ok(id)
    }

    pub async fn disconnect_udp(&self, id: &str) -> NetworkServiceResult<()> {
        let guard = self.udp_sessions.read().await;
        let session = guard.get(id).ok_or(NetworkServiceError::NotConnected)?;
        let local_addr: SocketAddr = format!("0.0.0.0:{}", session.local_port)
            .parse()
            .unwrap_or_else(|_| "0.0.0.0:0".parse().unwrap());
        let remote_addr: SocketAddr = session.remote_addr.parse().unwrap_or_else(|_| "0.0.0.0:0".parse().unwrap());
        drop(guard);
        
        let handle = crate::adapters::network::UdpHandle {
            id: id.to_string(),
            local_addr,
            remote_addr,
        };

        self.adapter
            .disconnect_udp(&handle)
            .await
            .map_err(|e| NetworkServiceError::ConnectionFailed(e.to_string()))?;

        self.udp_sessions.write().await.remove(id);
        Ok(())
    }

    pub async fn send_udp(&self, id: &str, data: &[u8]) -> NetworkServiceResult<()> {
        let guard = self.udp_sessions.read().await;
        let session = guard.get(id).ok_or(NetworkServiceError::NotConnected)?;
        let local_addr: SocketAddr = format!("0.0.0.0:{}", session.local_port)
            .parse()
            .unwrap_or_else(|_| "0.0.0.0:0".parse().unwrap());
        let remote_addr: SocketAddr = session.remote_addr.parse().unwrap_or_else(|_| "0.0.0.0:0".parse().unwrap());
        let remote_str = session.remote_addr.clone();
        drop(guard);

        let remote_parts: Vec<&str> = remote_str.split(':').collect();
        let remote_addr_str = remote_parts.get(0).unwrap_or(&"0.0.0.0");
        let remote_port: u16 = remote_parts.get(1).and_then(|p| p.parse().ok()).unwrap_or(0);

        let handle = crate::adapters::network::UdpHandle {
            id: id.to_string(),
            local_addr,
            remote_addr,
        };

        self.adapter
            .send_udp(&handle, data, remote_addr_str, remote_port)
            .await
            .map_err(|e| NetworkServiceError::SendFailed(e.to_string()))
    }

    pub async fn receive_udp(&self, id: &str, buffer: &mut [u8]) -> NetworkServiceResult<(usize, String)> {
        let guard = self.udp_sessions.read().await;
        let session = guard.get(id).ok_or(NetworkServiceError::NotConnected)?;
        let local_addr: SocketAddr = format!("0.0.0.0:{}", session.local_port)
            .parse()
            .unwrap_or_else(|_| "0.0.0.0:0".parse().unwrap());
        let remote_addr: SocketAddr = session.remote_addr.parse().unwrap_or_else(|_| "0.0.0.0:0".parse().unwrap());
        drop(guard);

        let handle = crate::adapters::network::UdpHandle {
            id: id.to_string(),
            local_addr,
            remote_addr,
        };

        let (n, addr, _port) = self.adapter
            .receive_udp(&handle, buffer)
            .await
            .map_err(|e| NetworkServiceError::SendFailed(e.to_string()))?;

        Ok((n, addr))
    }

    pub async fn list_tcp_connections(&self) -> Vec<TcpConnectionInfo> {
        self.tcp_connections.read().await.values().cloned().collect()
    }

    pub async fn list_udp_sessions(&self) -> Vec<UdpSessionInfo> {
        self.udp_sessions.read().await.values().cloned().collect()
    }

    pub async fn list_tcp_servers(&self) -> Vec<TcpServerInfo> {
        self.tcp_servers.read().await.values().cloned().collect()
    }
}

impl Clone for NetworkService {
    fn clone(&self) -> Self {
        Self {
            adapter: self.adapter.clone(),
            tcp_servers: self.tcp_servers.clone(),
            tcp_connections: self.tcp_connections.clone(),
            udp_sessions: self.udp_sessions.clone(),
        }
    }
}
