//! System command handlers

use crate::config::AppConfiguration;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

/// Get application configuration
#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<AppConfiguration, String> {
    Ok(state.get_config())
}

/// Update application configuration
#[tauri::command]
pub async fn update_config(
    config: AppConfiguration,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.update_config(config);
    Ok(())
}

/// Get system information
#[tauri::command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    let info = SystemInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        platform: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
    };
    Ok(info)
}

/// System information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// Application version
    pub version: String,
    /// Platform (OS)
    pub platform: String,
    /// Architecture
    pub arch: String,
}

/// Check if running on OrangePi
#[tauri::command]
pub async fn check_orangepi() -> Result<bool, String> {
    let is_orangepi = std::path::Path::new("/sys/firmware/devicetree/base/model")
        .exists() && {
        if let Ok(content) = std::fs::read_to_string("/sys/firmware/devicetree/base/model") {
            content.to_lowercase().contains("orange")
        } else {
            false
        }
    };
    
    Ok(is_orangepi)
}

/// Open external link
#[tauri::command]
pub async fn open_link(url: String) -> Result<(), String> {
    open::that(&url).map_err(|e| format!("Failed to open link: {}", e))
}

/// Save log data
#[tauri::command]
pub async fn save_log(
    filename: String,
    data: String,
) -> Result<(), String> {
    use std::io::Write;
    
    let path = std::path::Path::new(&filename);
    let mut file = std::fs::File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;
    file.write_all(data.as_bytes()).map_err(|e| format!("Failed to write log: {}", e))?;
    Ok(())
}