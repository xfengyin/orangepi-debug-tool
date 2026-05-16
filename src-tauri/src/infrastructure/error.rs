use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    // 串口错误 (1000-1999)
    #[error("Serial port not found: {0}")]
    SerialPortNotFound(String),
    #[error("Serial connection failed: {0}")]
    SerialConnectionFailed(String),
    #[error("Serial read error: {0}")]
    SerialReadError(String),
    #[error("Serial write error: {0}")]
    SerialWriteError(String),
    
    // 网络错误 (2000-2999)
    #[error("Network connection refused: {0}")]
    NetworkConnectionRefused(String),
    #[error("Network connection closed")]
    NetworkConnectionClosed,
    #[error("Network timeout")]
    NetworkTimeout,
    #[error("Network send error: {0}")]
    NetworkSendError(String),
    
    // 配置错误 (3000-3999)
    #[error("Config not found: {0}")]
    ConfigNotFound(String),
    #[error("Config parse error: {0}")]
    ConfigParseError(String),
    #[error("Config validation failed: {0}")]
    ConfigValidationFailed(String),
    
    // 通用错误 (9000-9999)
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

impl AppError {
    pub fn error_code(&self) -> u32 {
        match self {
            AppError::SerialPortNotFound(_) => 1001,
            AppError::SerialConnectionFailed(_) => 1002,
            AppError::SerialReadError(_) => 1003,
            AppError::SerialWriteError(_) => 1004,
            
            AppError::NetworkConnectionRefused(_) => 2001,
            AppError::NetworkConnectionClosed => 2002,
            AppError::NetworkTimeout => 2003,
            AppError::NetworkSendError(_) => 2004,
            
            AppError::ConfigNotFound(_) => 3001,
            AppError::ConfigParseError(_) => 3002,
            AppError::ConfigValidationFailed(_) => 3003,
            
            AppError::InternalError(_) => 9001,
            AppError::NotImplemented(_) => 9002,
            AppError::InvalidParameter(_) => 9003,
        }
    }
    
    pub fn error_category(&self) -> &'static str {
        match self {
            AppError::SerialPortNotFound(_) |
            AppError::SerialConnectionFailed(_) |
            AppError::SerialReadError(_) |
            AppError::SerialWriteError(_) => "serial",
            
            AppError::NetworkConnectionRefused(_) |
            AppError::NetworkConnectionClosed |
            AppError::NetworkTimeout |
            AppError::NetworkSendError(_) => "network",
            
            AppError::ConfigNotFound(_) |
            AppError::ConfigParseError(_) |
            AppError::ConfigValidationFailed(_) => "config",
            
            AppError::InternalError(_) |
            AppError::NotImplemented(_) |
            AppError::InvalidParameter(_) => "general",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u32,
    pub category: String,
    pub message: String,
    pub details: Option<String>,
}

impl From<AppError> for ErrorResponse {
    fn from(err: AppError) -> Self {
        Self {
            code: err.error_code(),
            category: err.error_category().to_string(),
            message: err.to_string(),
            details: None,
        }
    }
}
