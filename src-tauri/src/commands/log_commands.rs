use crate::services::LogService;
use tauri::State;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
pub struct WriteLogParams {
    pub data: String,
    pub with_timestamp: Option<bool>,
}

#[tauri::command]
pub async fn write_log(
    params: WriteLogParams,
    service: State<'_, LogService>,
) -> Result<CommandResult<()>, String> {
    let with_timestamp = params.with_timestamp.unwrap_or(false);

    let result: std::io::Result<()> = if with_timestamp {
        service.write_with_timestamp(&params.data).await
    } else {
        service.write_line(&params.data).await
    };

    match result {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::<()>::err(e.to_string())),
    }
}

#[derive(Debug, Deserialize)]
pub struct SaveDataParams {
    pub data: String,
    pub source: String,
    pub format: String,
}

#[tauri::command]
pub async fn save_debug_data(
    params: SaveDataParams,
    service: State<'_, LogService>,
) -> Result<CommandResult<()>, String> {
    let log_entry = format!(
        "[{}] [{}] {}",
        params.source,
        params.format.to_uppercase(),
        params.data
    );

    match service.write_with_timestamp(&log_entry).await {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::<()>::err(e.to_string())),
    }
}

#[derive(Debug, Serialize)]
pub struct LogStats {
    pub log_dir: String,
    pub rotation_size: usize,
}

#[tauri::command]
pub async fn get_log_stats(
    _service: State<'_, LogService>,
) -> Result<CommandResult<LogStats>, String> {
    Ok(CommandResult::ok(LogStats {
        log_dir: "./logs".to_string(),
        rotation_size: 10 * 1024 * 1024,
    }))
}

#[tauri::command]
pub async fn export_logs(
    output_path: String,
    _service: State<'_, LogService>,
) -> Result<CommandResult<String>, String> {
    let log_dir = PathBuf::from(&output_path);
    
    match tokio::fs::read_dir(&log_dir).await {
        Ok(mut entries) => {
            let mut count = 0;
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".log") {
                        count += 1;
                    }
                }
            }
            Ok(CommandResult::ok(format!("Found {} log files in {}", count, output_path)))
        }
        Err(e) => Ok(CommandResult::<String>::err(format!("Failed to read log directory: {}", e))),
    }
}
