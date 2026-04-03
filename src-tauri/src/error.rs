//! Error handling module for OrangePi Debug Tool

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type alias with AppError
pub type AppResult<T> = Result<T, AppError>;

/// Main error type for the application
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "message")]
pub enum AppError {
    /// Serial communication errors
    #[error("Serial error: {0}")]
    Serial(String),
    
    /// GPIO related errors
    #[error("GPIO error: {0}")]
    Gpio(String),
    
    /// PWM related errors
    #[error("PWM error: {0}")]
    Pwm(String),
    
    /// Device detection errors
    #[error("Device error: {0}")]
    Device(String),
    
    /// Database errors
    #[error("Database error: {0}")]
    Database(String),
    
    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),
    
    /// I/O errors
    #[error("I/O error: {0}")]
    Io(String),
    
    /// Invalid argument errors
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    /// Not found errors
    #[error("Not found: {0}")]
    NotFound(String),
    
    /// Permission denied errors
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// Timeout errors
    #[error("Timeout: {0}")]
    Timeout(String),
    
    /// Internal errors
    #[error("Internal error: {0}")]
    Internal(String),
}

impl AppError {
    /// Convert error to user-friendly message
    #[inline]
    pub fn to_user_message(&self) -> String {
        match self {
            AppError::Serial(msg) => format!("串口通信错误: {}", msg),
            AppError::Gpio(msg) => format!("GPIO错误: {}", msg),
            AppError::Pwm(msg) => format!("PWM错误: {}", msg),
            AppError::Device(msg) => format!("设备错误: {}", msg),
            AppError::Database(msg) => format!("数据库错误: {}", msg),
            AppError::Config(msg) => format!("配置错误: {}", msg),
            AppError::Io(msg) => format!("I/O错误: {}", msg),
            AppError::InvalidArgument(msg) => format!("参数错误: {}", msg),
            AppError::NotFound(msg) => format!("未找到: {}", msg),
            AppError::PermissionDenied(msg) => format!("权限不足: {}", msg),
            AppError::Timeout(msg) => format!("操作超时: {}", msg),
            AppError::Internal(msg) => format!("内部错误: {}", msg),
        }
    }
    
    /// Get error code for frontend handling
    #[inline]
    pub fn code(&self) -> &'static str {
        match self {
            AppError::Serial(_) => "SERIAL_ERROR",
            AppError::Gpio(_) => "GPIO_ERROR",
            AppError::Pwm(_) => "PWM_ERROR",
            AppError::Device(_) => "DEVICE_ERROR",
            AppError::Database(_) => "DB_ERROR",
            AppError::Config(_) => "CONFIG_ERROR",
            AppError::Io(_) => "IO_ERROR",
            AppError::InvalidArgument(_) => "INVALID_ARG",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::PermissionDenied(_) => "PERMISSION_DENIED",
            AppError::Timeout(_) => "TIMEOUT",
            AppError::Internal(_) => "INTERNAL_ERROR",
        }
    }
}

impl From<std::io::Error> for AppError {
    #[inline]
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    #[inline]
    fn from(err: serde_json::Error) -> Self {
        AppError::Internal(format!("JSON serialization error: {}", err))
    }
}

impl From<sqlx::Error> for AppError {
    #[inline]
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

#[cfg(feature = "hardware-support")]
impl From<gpio_cdev::Error> for AppError {
    #[inline]
    fn from(err: gpio_cdev::Error) -> Self {
        AppError::Gpio(err.to_string())
    }
}