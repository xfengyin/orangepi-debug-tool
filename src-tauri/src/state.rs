//! Application state management module

use crate::{AppError, AppResult};
use dashmap::DashMap;
use parking_lot::{Mutex, RwLock};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, info};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Theme setting (light/dark/system)
    pub theme: String,
    /// Language setting
    pub language: String,
    /// Auto-save interval in seconds
    pub auto_save_interval: u64,
    /// Serial buffer size
    pub serial_buffer_size: usize,
    /// Max log entries to keep in memory
    pub max_log_entries: usize,
    /// Enable hardware acceleration
    pub hardware_acceleration: bool,
}

impl Default for AppConfig {
    #[inline]
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            language: "zh-CN".to_string(),
            auto_save_interval: 30,
            serial_buffer_size: 65536,
            max_log_entries: 10000,
            hardware_acceleration: true,
        }
    }
}

/// Serial port connection state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialState {
    /// Connected port name
    pub port_name: Option<String>,
    /// Current baud rate
    pub baud_rate: u32,
    /// Data bits
    pub data_bits: u8,
    /// Parity
    pub parity: String,
    /// Stop bits
    pub stop_bits: u8,
    /// Is currently connected
    pub is_connected: bool,
    /// Is monitoring enabled
    pub is_monitoring: bool,
    /// Received bytes count
    pub rx_bytes: u64,
    /// Transmitted bytes count
    pub tx_bytes: u64,
}

impl Default for SerialState {
    #[inline]
    fn default() -> Self {
        Self {
            port_name: None,
            baud_rate: 115200,
            data_bits: 8,
            parity: "None".to_string(),
            stop_bits: 1,
            is_connected: false,
            is_monitoring: false,
            rx_bytes: 0,
            tx_bytes: 0,
        }
    }
}

/// GPIO state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpioState {
    /// Pin configurations
    pub pins: DashMap<u32, GpioPinState>,
    /// Interrupt monitoring enabled
    pub interrupt_monitoring: bool,
}

/// GPIO pin state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpioPinState {
    /// Pin number
    pub pin: u32,
    /// Direction (in/out)
    pub direction: String,
    /// Current value (0/1)
    pub value: u8,
    /// Pull-up/pull-down configuration
    pub pull: String,
    /// Is interrupt enabled
    pub interrupt_enabled: bool,
    /// Interrupt trigger type
    pub interrupt_trigger: Option<String>,
}

/// PWM state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwmState {
    /// Channel configurations
    pub channels: DashMap<u32, PwmChannelState>,
}

/// PWM channel state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwmChannelState {
    /// Channel number
    pub channel: u32,
    /// Enabled state
    pub enabled: bool,
    /// Frequency in Hz
    pub frequency: f64,
    /// Duty cycle (0-100)
    pub duty_cycle: f64,
    /// Polarity
    pub polarity: String,
}

/// Application state container
pub struct AppState {
    /// Application configuration
    pub config: RwLock<AppConfig>,
    /// Serial port state
    pub serial: RwLock<SerialState>,
    /// GPIO state
    pub gpio: RwLock<GpioState>,
    /// PWM state
    pub pwm: RwLock<PwmState>,
    /// Database connection pool
    pub db_pool: Option<sqlx::SqlitePool>,
    /// Event sender for UI updates
    pub event_tx: mpsc::Sender<AppEvent>,
    /// Device manager
    pub device_manager: Arc<Mutex<crate::devices::DeviceManager>>,
    /// Serial port manager
    pub serial_manager: Arc<Mutex<crate::devices::serial::SerialManager>>,
}

/// Application events for UI updates
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum AppEvent {
    /// Serial data received
    SerialData(Vec<u8>),
    /// Serial connection status changed
    SerialStatus(bool),
    /// GPIO pin value changed
    GpioChange { pin: u32, value: u8 },
    /// PWM value changed
    PwmChange { channel: u32, duty_cycle: f64 },
    /// Log message
    Log { level: String, message: String },
    /// Error occurred
    Error(String),
}

impl AppState {
    /// Create new application state
    pub async fn new(app: &tauri::App) -> AppResult<Self> {
        info!("Initializing application state...");
        
        let config = Self::load_config(app).await?;
        let (event_tx, _event_rx) = mpsc::channel(1000);
        
        let db_pool = Self::init_database(app).await.ok();
        
        let state = Self {
            config: RwLock::new(config),
            serial: RwLock::new(SerialState::default()),
            gpio: RwLock::new(GpioState {
                pins: DashMap::new(),
                interrupt_monitoring: false,
            }),
            pwm: RwLock::new(PwmState {
                channels: DashMap::new(),
            }),
            db_pool,
            event_tx,
            device_manager: Arc::new(Mutex::new(crate::devices::DeviceManager::new())),
            serial_manager: Arc::new(Mutex::new(crate::devices::serial::SerialManager::new())),
        };
        
        info!("Application state initialized");
        Ok(state)
    }
    
    /// Load configuration from file
    async fn load_config(_app: &tauri::App) -> AppResult<AppConfig> {
        // Try to load from config file, fallback to defaults
        Ok(AppConfig::default())
    }
    
    /// Initialize database
    async fn init_database(app: &tauri::App) -> AppResult<sqlx::SqlitePool> {
        let app_dir = app
            .path_resolver()
            .app_data_dir()
            .ok_or_else(|| AppError::Config("Failed to get app data directory".to_string()))?;
        
        std::fs::create_dir_all(&app_dir)?;
        
        let db_path = app_dir.join("debug_tool.db");
        let db_url = format!("sqlite:{}", db_path.display());
        
        let pool = sqlx::SqlitePool::connect(&db_url).await?;
        
        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        info!("Database initialized at: {}", db_path.display());
        Ok(pool)
    }
    
    /// Get configuration
    #[inline]
    pub fn get_config(&self) -> AppConfig {
        self.config.read().clone()
    }
    
    /// Update configuration
    #[inline]
    pub fn update_config(&self, config: AppConfig) {
        *self.config.write() = config;
    }
    
    /// Get serial state
    #[inline]
    pub fn get_serial_state(&self) -> SerialState {
        self.serial.read().clone()
    }
    
    /// Update serial state
    #[inline]
    pub fn update_serial_state<F>(&self, f: F)
    where
        F: FnOnce(&mut SerialState),
    {
        f(&mut *self.serial.write());
    }
    
    /// Send event to UI
    #[inline]
    pub async fn send_event(&self, event: AppEvent) {
        let _ = self.event_tx.send(event).await;
    }
    
    /// Cleanup resources before exit
    pub async fn cleanup(&self) -> AppResult<()> {
        debug!("Cleaning up application state...");
        
        // Disconnect serial port
        if let Ok(mut serial) = self.serial_manager.try_lock() {
            serial.disconnect().await?;
        }
        
        // Close database connections
        if let Some(pool) = &self.db_pool {
            pool.close().await;
        }
        
        info!("Application state cleaned up");
        Ok(())
    }
}