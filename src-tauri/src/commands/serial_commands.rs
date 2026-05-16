use crate::services::SerialService;
use crate::adapters::serial::{SerialConfig, SerialPortInfo};
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

#[tauri::command]
pub async fn list_serial_ports(
    service: State<'_, SerialService>,
) -> Result<CommandResult<Vec<SerialPortInfo>>, String> {
    match service.list_ports().await {
        Ok(ports) => Ok(CommandResult::ok(ports)),
        Err(e) => Ok(CommandResult::<Vec<SerialPortInfo>>::err(e.to_string())),
    }
}

#[derive(Debug, Deserialize)]
pub struct ConnectSerialParams {
    pub port_name: String,
    pub baudrate: u32,
    pub data_bits: u8,
    pub stop_bits: u8,
    pub parity: String,
}

#[tauri::command]
pub async fn connect_serial(
    params: ConnectSerialParams,
    service: State<'_, SerialService>,
) -> Result<CommandResult<String>, String> {
    let config = SerialConfig {
        port_name: params.port_name.clone(),
        baudrate: params.baudrate,
        data_bits: params.data_bits,
        stop_bits: params.stop_bits,
        parity: params.parity,
    };

    match service.connect(params.port_name, config).await {
        Ok(id) => Ok(CommandResult::ok(id)),
        Err(e) => Ok(CommandResult::<String>::err(e.to_string())),
    }
}

#[tauri::command]
pub async fn disconnect_serial(
    id: String,
    service: State<'_, SerialService>,
) -> Result<CommandResult<()>, String> {
    match service.disconnect(&id).await {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::<()>::err(e.to_string())),
    }
}

#[derive(Debug, Deserialize)]
pub struct SendSerialParams {
    pub id: String,
    pub data: String,
    pub format: String,
}

#[tauri::command]
pub async fn send_serial(
    params: SendSerialParams,
    service: State<'_, SerialService>,
) -> Result<CommandResult<()>, String> {
    let bytes = match params.format.as_str() {
        "hex" => parse_hex_string(&params.data),
        "ascii" => params.data.as_bytes().to_vec(),
        _ => return Ok(CommandResult::<()>::err("Invalid format. Use 'hex' or 'ascii'".to_string())),
    };

    match service.send(&params.id, &bytes).await {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::<()>::err(e.to_string())),
    }
}

#[derive(Debug, Deserialize)]
pub struct TimingSendParams {
    pub id: String,
    pub data: String,
    pub format: String,
    pub interval_ms: u64,
}

#[tauri::command]
pub async fn start_timing_send(
    params: TimingSendParams,
    service: State<'_, SerialService>,
) -> Result<CommandResult<()>, String> {
    let bytes = match params.format.as_str() {
        "hex" => parse_hex_string(&params.data),
        "ascii" => params.data.as_bytes().to_vec(),
        _ => return Ok(CommandResult::<()>::err("Invalid format. Use 'hex' or 'ascii'".to_string())),
    };

    match service.start_timing_send(&params.id, bytes, params.interval_ms).await {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::<()>::err(e.to_string())),
    }
}

#[tauri::command]
pub async fn stop_timing_send(
    id: String,
    service: State<'_, SerialService>,
) -> Result<CommandResult<()>, String> {
    match service.stop_timing_send(&id).await {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::<()>::err(e.to_string())),
    }
}

#[tauri::command]
pub async fn get_serial_stats(
    id: String,
    service: State<'_, SerialService>,
) -> Result<CommandResult<crate::services::SerialStats>, String> {
    match service.get_stats(&id).await {
        Ok(stats) => Ok(CommandResult::ok(stats)),
        Err(e) => Ok(CommandResult::<crate::services::SerialStats>::err(e.to_string())),
    }
}

fn parse_hex_string(s: &str) -> Vec<u8> {
    s.split_whitespace()
        .filter_map(|part| u8::from_str_radix(part, 16).ok())
        .collect()
}
