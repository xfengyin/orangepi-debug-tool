//! Error handling module for OrangePi Debug Tool

use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use std::time::{Duration, Instant};

/// Result type alias with AppError
pub type AppResult<T> = Result<T, AppError>;

/// Main error type for the application with error codes
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "message")]
pub enum AppError {
    /// Serial communication errors (code: 1001-1999)
    #[error("Serial error: {0}")]
    Serial(String),
    
    /// GPIO related errors (code: 2001-2999)
    #[error("GPIO error: {0}")]
    Gpio(String),
    
    /// PWM related errors (code: 3001-3999)
    #[error("PWM error: {0}")]
    Pwm(String),
    
    /// Device detection errors (code: 4001-4999)
    #[error("Device error: {0}")]
    Device(String),
    
    /// Database errors (code: 5001-5999)
    #[error("Database error: {0}")]
    Database(String),
    
    /// Configuration errors (code: 6001-6999)
    #[error("Configuration error: {0}")]
    Config(String),
    
    /// I/O errors (code: 7001-7999)
    #[error("I/O error: {0}")]
    Io(String),
    
    /// Invalid argument errors (code: 8001-8999)
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    /// Not found errors (code: 9001-9999)
    #[error("Not found: {0}")]
    NotFound(String),
    
    /// Permission denied errors (code: 10001-10999)
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// Timeout errors (code: 11001-11999)
    #[error("Timeout: {0}")]
    Timeout(String),
    
    /// Circuit breaker open errors (code: 12001-12999)
    #[error("Circuit breaker open: {0}")]
    CircuitBreakerOpen(String),
    
    /// Invalid state errors (code: 13001-13999)
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    /// Internal errors (code: 99999)
    #[error("Internal error: {0}")]
    Internal(String),
}

impl AppError {
    /// Convert error to user-friendly message in Chinese
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
            AppError::CircuitBreakerOpen(msg) => format!("服务熔断: {}", msg),
            AppError::InvalidState(msg) => format!("状态异常: {}", msg),
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
            AppError::CircuitBreakerOpen(_) => "CIRCUIT_BREAKER_OPEN",
            AppError::InvalidState(_) => "INVALID_STATE",
            AppError::Internal(_) => "INTERNAL_ERROR",
        }
    }

    /// Get numeric error code for logging
    #[inline]
    pub fn numeric_code(&self) -> u32 {
        match self {
            AppError::Serial(_) => 1001,
            AppError::Gpio(_) => 2001,
            AppError::Pwm(_) => 3001,
            AppError::Device(_) => 4001,
            AppError::Database(_) => 5001,
            AppError::Config(_) => 6001,
            AppError::Io(_) => 7001,
            AppError::InvalidArgument(_) => 8001,
            AppError::NotFound(_) => 9001,
            AppError::PermissionDenied(_) => 10001,
            AppError::Timeout(_) => 11001,
            AppError::CircuitBreakerOpen(_) => 12001,
            AppError::InvalidState(_) => 13001,
            AppError::Internal(_) => 99999,
        }
    }

    /// Check if error is retryable
    #[inline]
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            AppError::Timeout(_) | 
            AppError::Io(_) | 
            AppError::CircuitBreakerOpen(_)
        )
    }

    /// Check if error is critical (should trigger circuit breaker)
    #[inline]
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            AppError::Device(_) | 
            AppError::CircuitBreakerOpen(_)
        )
    }

    /// Get recovery suggestion in Chinese
    pub fn recovery_suggestion(&self) -> Option<String> {
        match self {
            AppError::Serial(msg) => Some(format!(
                "请检查串口连接是否正常，波特率设置是否正确。原始错误: {}", msg
            )),
            AppError::Gpio(msg) => Some(format!(
                "请检查GPIO引脚是否被其他程序占用，是否有权限访问。原始错误: {}", msg
            )),
            AppError::Pwm(msg) => Some(format!(
                "请检查PWM通道是否可用，频率设置是否在有效范围内。原始错误: {}", msg
            )),
            AppError::Timeout(_) => Some("操作超时，请检查设备响应或网络连接".to_string()),
            AppError::CircuitBreakerOpen(_) => Some("服务暂时不可用，请稍后重试".to_string()),
            _ => None,
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

/// Error recovery strategies
#[derive(Debug, Clone)]
pub enum RecoveryStrategy {
    Retry { max_attempts: u32, delay_ms: u64 },
    Fallback { fallback_value: String },
    CircuitBreak,
    Degrade { degraded_functionality: String },
}

/// Circuit breaker states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}

/// Circuit breaker for preventing cascading failures
pub struct CircuitBreaker {
    name: String,
    state: CircuitBreakerState,
    failure_count: u32,
    success_count: u32,
    last_failure_time: Option<Instant>,
    config: CircuitBreakerConfig,
}

#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub success_threshold: u32,
    pub timeout: Duration,
    pub half_open_max_calls: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(60),
            half_open_max_calls: 3,
        }
    }
}

impl CircuitBreaker {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
            config: CircuitBreakerConfig::default(),
        }
    }

    pub fn with_config(mut self, config: CircuitBreakerConfig) -> Self {
        self.config = config;
        self
    }

    pub fn state(&self) -> CircuitBreakerState {
        if self.state == CircuitBreakerState::Open {
            if let Some(last_failure) = self.last_failure_time {
                if last_failure.elapsed() >= self.config.timeout {
                    return CircuitBreakerState::HalfOpen;
                }
            }
        }
        self.state
    }

    pub fn is_open(&self) -> bool {
        self.state() == CircuitBreakerState::Open
    }

    pub fn record_success(&mut self) {
        match self.state {
            CircuitBreakerState::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= self.config.success_threshold {
                    self.state = CircuitBreakerState::Closed;
                    self.failure_count = 0;
                    self.success_count = 0;
                }
            }
            CircuitBreakerState::Closed => {
                self.failure_count = self.failure_count.saturating_sub(1);
            }
            _ => {}
        }
    }

    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(Instant::now());

        match self.state {
            CircuitBreakerState::HalfOpen => {
                self.state = CircuitBreakerState::Open;
                self.success_count = 0;
            }
            CircuitBreakerState::Closed => {
                if self.failure_count >= self.config.failure_threshold {
                    self.state = CircuitBreakerState::Open;
                }
            }
            _ => {}
        }
    }

    pub fn reset(&mut self) {
        self.state = CircuitBreakerState::Closed;
        self.failure_count = 0;
        self.success_count = 0;
        self.last_failure_time = None;
    }
}

/// Error statistics collector
pub struct ErrorCollector {
    errors: Arc<RwLock<HashMap<String, ErrorStats>>>,
}

#[derive(Debug, Clone, Default)]
pub struct ErrorStats {
    pub count: u64,
    pub last_occurrence: Option<Instant>,
    pub first_occurrence: Option<Instant>,
    pub error_messages: Vec<String>,
}

impl ErrorCollector {
    pub fn new() -> Self {
        Self {
            errors: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn record(&self, error_code: &str, message: &str) {
        let mut errors = self.errors.write();
        let stats = errors.entry(error_code.to_string()).or_default();
        stats.count += 1;
        let now = Instant::now();
        stats.last_occurrence = Some(now);
        if stats.first_occurrence.is_none() {
            stats.first_occurrence = Some(now);
        }
        if stats.error_messages.len() < 10 {
            stats.error_messages.push(message.to_string());
        }
    }

    pub fn get_stats(&self, error_code: &str) -> Option<ErrorStats> {
        self.errors.read().get(error_code).cloned()
    }

    pub fn get_all_stats(&self) -> HashMap<String, ErrorStats> {
        self.errors.read().clone()
    }

    pub fn get_top_errors(&self, limit: usize) -> Vec<(String, ErrorStats)> {
        let mut errors: Vec<_> = self.errors.read()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        errors.sort_by(|a, b| b.1.count.cmp(&a.1.count));
        errors.into_iter().take(limit).collect()
    }

    pub fn clear(&self) {
        self.errors.write().clear();
    }
}

/// Retry handler with exponential backoff
pub struct RetryHandler {
    max_attempts: u32,
    initial_delay_ms: u64,
    max_delay_ms: u64,
    backoff_multiplier: f64,
}

impl Default for RetryHandler {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
        }
    }
}

impl RetryHandler {
    pub fn new(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            ..Default::default()
        }
    }

    pub async fn execute<F, T, E>(&self, operation: F) -> Result<T, E>
    where
        F: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Debug,
    {
        let mut operation = std::pin::pin!(operation);
        let mut delay = self.initial_delay_ms;
        
        for attempt in 1..=self.max_attempts {
            match operation.as_mut().await {
                Ok(result) => return Ok(result),
                Err(e) if attempt < self.max_attempts => {
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                    delay = (delay as f64 * self.backoff_multiplier) as u64;
                    delay = delay.min(self.max_delay_ms);
                }
                Err(e) => return Err(e),
            }
        }
        unreachable!()
    }
}
