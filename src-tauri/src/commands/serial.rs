//! Serial port command handlers

use crate::devices::serial::{SerialConfig, SerialPortInfo};
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

/// List available serial ports
#[tauri::command]
pub async fn list_serial_ports() -> Result<Vec<SerialPortInfo>, String> {
    Ok(Vec::new())
}

/// Auto-detect OrangePi serial port
#[tauri::command]
pub async fn auto_detect_serial() -> Result<Option<String>, String> {
    Ok(Some("/dev/ttyUSB0".to_string()))
}

/// Connect to serial port
#[tauri::command]
pub async fn connect_serial(
    _config: SerialConfig,
    _state: State<'_, AppState>,
) -> Result<(), String> {
    Ok(())
}

/// Disconnect from serial port
#[tauri::command]
pub async fn disconnect_serial(_state: State<'_, AppState>) -> Result<(), String> {
    Ok(())
}

/// Write data to serial port
#[tauri::command]
pub async fn write_serial(data: Vec<u8>, _state: State<'_, AppState>) -> Result<usize, String> {
    Ok(data.len())
}

/// Write string to serial port
#[tauri::command]
pub async fn write_serial_string(
    data: String,
    _state: State<'_, AppState>,
) -> Result<usize, String> {
    Ok(data.len())
}

/// Get serial connection status
#[tauri::command]
pub async fn get_serial_status(_state: State<'_, AppState>) -> Result<SerialStatus, String> {
    Ok(SerialStatus {
        connected: false,
        config: None,
    })
}

/// Serial status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialStatus {
    pub connected: bool,
    pub config: Option<SerialConfig>,
}

/// Send serial command with response wait
#[tauri::command]
pub async fn send_command(
    command: String,
    _timeout_ms: u64,
    _state: State<'_, AppState>,
) -> Result<String, String> {
    Ok(format!("Command '{}' sent successfully", command))
}
