//! Tauri command handlers

pub mod gpio;
pub mod pwm;
pub mod serial;
pub mod system;

use crate::AppResult;
use serde::{Deserialize, Serialize};

/// Standard API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Success flag
    pub success: bool,
    /// Response data
    pub data: Option<T>,
    /// Error message if failed
    pub error: Option<String>,
    /// Error code
    pub error_code: Option<String>,
}

impl<T> ApiResponse<T> {
    /// Create success response
    #[inline]
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            error_code: None,
        }
    }
    
    /// Create error response
    #[inline]
    pub fn error<E: Into<String>>(message: E, code: Option<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
            error_code: code,
        }
    }
}

/// Convert AppResult to ApiResponse
#[inline]
pub fn into_response<T>(result: AppResult<T>) -> ApiResponse<T> {
    match result {
        Ok(data) => ApiResponse::success(data),
        Err(e) => ApiResponse::error(e.to_user_message(), Some(e.code().to_string())),
    }
}