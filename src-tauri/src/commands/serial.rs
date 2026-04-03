//! Serial port command handlers

use crate::commands::{into_response, ApiResponse};
use crate::devices::serial::{SerialConfig, SerialPortInfo};
use crate::state::AppState;
use crate::AppResult;
use serde::{Deserialize, Serialize};
use tauri::State;

/// List available serial ports
#[tauri::command]
pub async fn list_serial_ports() -> ApiResponse<Vec<SerialPortInfo>> {
    into_response(crate::devices::serial::SerialManager::list_ports())
}

/// Auto-detect OrangePi serial port
#[tauri::command]
pub async fn auto_detect_serial() -> ApiResponse<Option<String>> {
    into_response(crate::devices::serial::SerialManager::auto_detect())
}

/// Connect to serial port
#[tauri::command]
pub async fn connect_serial(
    config: SerialConfig,
    state: State<'_, AppState>,
) -> ApiResponse<()> {
    into_response(async {
        let mut serial = state.serial_manager.lock();
        serial.connect(config, None).await
    }.await)
}

/// Disconnect from serial port
#[tauri::command]
pub async fn disconnect_serial(state: State<'_, AppState>) -> ApiResponse<()> {
    into_response(async {
        let mut serial = state.serial_manager.lock();
        serial.disconnect().await
    }.await)
}

/// Write data to serial port
#[tauri::command]
pub async fn write_serial(
    data: Vec<u8>,
    state: State<'_, AppState>,
) -> ApiResponse<usize> {
    into_response(async {
        let mut serial = state.serial_manager.lock();
        serial.write(&data).await
    }.await)
}

/// Write string to serial port
#[tauri::command]
pub async fn write_serial_string(
    data: String,
    state: State<'_, AppState>,
) -> ApiResponse<usize> {
    into_response(async {
        let mut serial = state.serial_manager.lock();
        serial.write(data.as_bytes()).await
    }.await)
}

/// Get serial connection status
#[tauri::command]
pub async fn get_serial_status(state: State<'_, AppState>) -> ApiResponse<SerialStatus> {
    let serial = state.serial_manager.lock();
    let config = serial.get_config().cloned();
    let is_connected = serial.is_connected();
    
    ApiResponse::success(SerialStatus {
        connected: is_connected,
        config,
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
    timeout_ms: u64,
    state: State<'_, AppState>,
) -> ApiResponse<String> {
    into_response(async {
        let mut serial = state.serial_manager.lock();
        
        // Send command
        let cmd = if command.ends_with('\n') {
            command.into_bytes()
        } else {
            format!("{}\n", command).into_bytes()
        };
        
        serial.write(&cmd).await?;
        
        // Wait for response (simplified - would need proper response handling)
        tokio::time::sleep(tokio::time::Duration::from_millis(timeout_ms.min(5000))).await;
        
        Ok("Command sent".to_string())
    }.await)
}