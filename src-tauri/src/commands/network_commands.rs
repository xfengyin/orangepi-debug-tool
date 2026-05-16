use crate::services::NetworkService;
use crate::services::{TcpConnectionInfo, UdpSessionInfo, TcpServerInfo};
use tauri::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CommandResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> CommandResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(msg: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTcpServerParams {
    pub port: u16,
    pub max_connections: Option<usize>,
}

#[tauri::command]
pub async fn create_tcp_server(
    params: CreateTcpServerParams,
    service: State<'_, NetworkService>,
) -> Result<CommandResult<String>, String> {
    let max_connections = params.max_connections.unwrap_or(10);

    match service.create_tcp_server(params.port, max_connections).await {
        Ok(id) => Ok(CommandResult::ok(id)),
        Err(e) => Ok(CommandResult::<String>::err(e.to_string())),
    }
}

#[tauri::command]
pub async fn close_tcp_server(
    server_id: String,
    service: State<'_, NetworkService>,
) -> Result<CommandResult<()>, String> {
    match service.close_tcp_server(&server_id).await {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::<()>::err(e.to_string())),
    }
}

#[tauri::command]
pub async fn accept_tcp_connection(
    server_id: String,
    service: State<'_, NetworkService>,
) -> Result<CommandResult<String>, String> {
    match service.accept_tcp_connection(&server_id).await {
        Ok(id) => Ok(CommandResult::ok(id)),
        Err(e) => Ok(CommandResult::<String>::err(e.to_string())),
    }
}

#[derive(Debug, Deserialize)]
pub struct ConnectTcpParams {
    pub addr: String,
    pub port: u16,
}

#[tauri::command]
pub async fn connect_tcp(
    params: ConnectTcpParams,
    service: State<'_, NetworkService>,
) -> Result<CommandResult<String>, String> {
    match service.connect_tcp(&params.addr, params.port).await {
        Ok(id) => Ok(CommandResult::ok(id)),
        Err(e) => Ok(CommandResult::<String>::err(e.to_string())),
    }
}

#[tauri::command]
pub async fn disconnect_tcp(
    id: String,
    service: State<'_, NetworkService>,
) -> Result<CommandResult<()>, String> {
    match service.disconnect_tcp(&id).await {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::<()>::err(e.to_string())),
    }
}

#[derive(Debug, Deserialize)]
pub struct SendTcpParams {
    pub id: String,
    pub data: String,
    pub format: String,
}

#[tauri::command]
pub async fn send_tcp(
    params: SendTcpParams,
    service: State<'_, NetworkService>,
) -> Result<CommandResult<()>, String> {
    let bytes = match params.format.as_str() {
        "hex" => parse_hex_string(&params.data),
        "ascii" => params.data.as_bytes().to_vec(),
        _ => return Ok(CommandResult::<()>::err("Invalid format. Use 'hex' or 'ascii'".to_string())),
    };

    match service.send_tcp(&params.id, &bytes).await {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::<()>::err(e.to_string())),
    }
}

#[derive(Debug, Deserialize)]
pub struct ConnectUdpParams {
    pub local_port: u16,
    pub remote_addr: String,
    pub remote_port: u16,
}

#[tauri::command]
pub async fn connect_udp(
    params: ConnectUdpParams,
    service: State<'_, NetworkService>,
) -> Result<CommandResult<String>, String> {
    match service.connect_udp(params.local_port, &params.remote_addr, params.remote_port).await {
        Ok(id) => Ok(CommandResult::ok(id)),
        Err(e) => Ok(CommandResult::<String>::err(e.to_string())),
    }
}

#[tauri::command]
pub async fn disconnect_udp(
    id: String,
    service: State<'_, NetworkService>,
) -> Result<CommandResult<()>, String> {
    match service.disconnect_udp(&id).await {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::<()>::err(e.to_string())),
    }
}

#[derive(Debug, Deserialize)]
pub struct SendUdpParams {
    pub id: String,
    pub data: String,
    pub format: String,
}

#[tauri::command]
pub async fn send_udp(
    params: SendUdpParams,
    service: State<'_, NetworkService>,
) -> Result<CommandResult<()>, String> {
    let bytes = match params.format.as_str() {
        "hex" => parse_hex_string(&params.data),
        "ascii" => params.data.as_bytes().to_vec(),
        _ => return Ok(CommandResult::<()>::err("Invalid format. Use 'hex' or 'ascii'".to_string())),
    };

    match service.send_udp(&params.id, &bytes).await {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::<()>::err(e.to_string())),
    }
}

#[tauri::command]
pub async fn list_tcp_connections(
    service: State<'_, NetworkService>,
) -> Result<CommandResult<Vec<TcpConnectionInfo>>, String> {
    let connections = service.list_tcp_connections().await;
    Ok(CommandResult::ok(connections))
}

#[tauri::command]
pub async fn list_udp_sessions(
    service: State<'_, NetworkService>,
) -> Result<CommandResult<Vec<UdpSessionInfo>>, String> {
    let sessions = service.list_udp_sessions().await;
    Ok(CommandResult::ok(sessions))
}

#[tauri::command]
pub async fn list_tcp_servers(
    service: State<'_, NetworkService>,
) -> Result<CommandResult<Vec<TcpServerInfo>>, String> {
    let servers = service.list_tcp_servers().await;
    Ok(CommandResult::ok(servers))
}

fn parse_hex_string(s: &str) -> Vec<u8> {
    s.split_whitespace()
        .filter_map(|part| u8::from_str_radix(part, 16).ok())
        .collect()
}
