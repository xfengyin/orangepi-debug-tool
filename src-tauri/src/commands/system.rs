//! System command handlers

use crate::commands::ApiResponse;
use crate::state::{AppConfig, AppState};
use serde::{Deserialize, Serialize};
use tauri::State;

/// Get application configuration
#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> ApiResponse<AppConfig> {
    ApiResponse::success(state.get_config())
}

/// Update application configuration
#[tauri::command]
pub async fn update_config(
    config: AppConfig,
    state: State<'_, AppState>,
) -> ApiResponse<()> {
    state.update_config(config);
    ApiResponse::success(())
}

/// Get system information
#[tauri::command]
pub async fn get_system_info() -> ApiResponse<SystemInfo> {
    let info = SystemInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        platform: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
    };
    ApiResponse::success(info)
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
pub async fn check_orangepi() -> ApiResponse<bool> {
    // Check for OrangePi specific files
    let is_orangepi = std::path::Path::new("/sys/firmware/devicetree/base/model")
        .exists() && {
        if let Ok(content) = std::fs::read_to_string("/sys/firmware/devicetree/base/model") {
            content.to_lowercase().contains("orange")
        } else {
            false
        }
    };
    
    ApiResponse::success(is_orangepi)
}

/// Open external link
#[tauri::command]
pub async fn open_link(url: String) -> ApiResponse<()> {
    match open::that(&url) {
        Ok(_) => ApiResponse::success(()),
        Err(e) => ApiResponse::error(format!("Failed to open link: {}", e), None),
    }
}

/// Save log data
#[tauri::command]
pub async fn save_log(
    filename: String,
    data: String,
) -> ApiResponse<()> {
    use std::io::Write;
    
    let path = std::path::Path::new(&filename);
    match std::fs::File::create(path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(data.as_bytes()) {
                ApiResponse::error(format!("Failed to write log: {}", e), None)
            } else {
                ApiResponse::success(())
            }
        }
        Err(e) => ApiResponse::error(format!("Failed to create file: {}", e), None),
    }
}