mod config_store;
mod device_store;
mod session_store;

pub use config_store::ConfigStore;
pub use device_store::{DeviceStore, DeviceEvent, DeviceState, DeviceType};
pub use session_store::{SessionStore, Session, Operation, SessionStats};

use crate::{AppError, AppResult};
use crate::adapters::DeviceAdapterRegistry;
use crate::observability::{HealthChecker, MetricsCollector, AppTracer};
use parking_lot::{Mutex, RwLock};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: String,
    pub language: String,
    pub auto_save_interval: u64,
    pub serial_buffer_size: usize,
    pub max_log_entries: usize,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialState {
    pub port_name: Option<String>,
    pub baud_rate: u32,
    pub data_bits: u8,
    pub parity: String,
    pub stop_bits: u8,
    pub is_connected: bool,
    pub is_monitoring: bool,
    pub rx_bytes: u64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpioState {
    pub pins: std::collections::HashMap<u32, GpioPinState>,
    pub interrupt_monitoring: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpioPinState {
    pub pin: u32,
    pub direction: String,
    pub value: u8,
    pub pull: String,
    pub interrupt_enabled: bool,
    pub interrupt_trigger: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwmState {
    pub channels: std::collections::HashMap<u32, PwmChannelState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwmChannelState {
    pub channel: u32,
    pub enabled: bool,
    pub frequency: f64,
    pub duty_cycle: f64,
    pub polarity: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum AppEvent {
    SerialData(Vec<u8>),
    SerialStatus(bool),
    GpioChange { pin: u32, value: u8 },
    PwmChange { channel: u32, duty_cycle: f64 },
    Log { level: String, message: String },
    Error(String),
}

pub struct AppState {
    pub config: RwLock<AppConfig>,
    pub serial: RwLock<SerialState>,
    pub gpio: RwLock<GpioState>,
    pub pwm: RwLock<PwmState>,
    pub db_pool: Option<sqlx::SqlitePool>,
    pub event_tx: mpsc::Sender<AppEvent>,
    
    pub config_store: ConfigStore,
    pub device_store: DeviceStore,
    pub session_store: SessionStore,
    
    pub adapter_registry: Arc<Mutex<DeviceAdapterRegistry>>,
    pub health_checker: Arc<HealthChecker>,
    pub metrics: Arc<MetricsCollector>,
    pub tracer: Arc<AppTracer>,
}

impl AppState {
    pub async fn new(app: &tauri::App) -> AppResult<Self> {
        info!("Initializing application state...");
        
        let (event_tx, _event_rx) = mpsc::channel(1000);
        
        let config_store = ConfigStore::new(None);
        let device_store = DeviceStore::new();
        let session_store = SessionStore::new();
        
        let adapter_registry = Arc::new(Mutex::new(DeviceAdapterRegistry::new()));
        
        let health_checker = Arc::new(HealthChecker::new());
        let metrics = Arc::new(MetricsCollector::new());
        let tracer = Arc::new(AppTracer::new());
        
        let db_pool = Self::init_database(app).await.ok();
        
        let state = Self {
            config: RwLock::new(AppConfig::default()),
            serial: RwLock::new(SerialState::default()),
            gpio: RwLock::new(GpioState {
                pins: std::collections::HashMap::new(),
                interrupt_monitoring: false,
            }),
            pwm: RwLock::new(PwmState {
                channels: std::collections::HashMap::new(),
            }),
            db_pool,
            event_tx,
            config_store,
            device_store,
            session_store,
            adapter_registry,
            health_checker,
            metrics,
            tracer,
        };
        
        state.session_store.create_session(None);
        
        info!("Application state initialized");
        Ok(state)
    }
    
    async fn init_database(app: &tauri::App) -> AppResult<sqlx::SqlitePool> {
        let app_dir = app
            .path_resolver()
            .app_data_dir()
            .ok_or_else(|| AppError::Config("Failed to get app data directory".to_string()))?;
        
        std::fs::create_dir_all(&app_dir)?;
        
        let db_path = app_dir.join("debug_tool.db");
        let db_url = format!("sqlite:{}", db_path.display());
        
        let pool = sqlx::SqlitePool::connect(&db_url).await?;
        
        info!("Database initialized at: {}", db_path.display());
        Ok(pool)
    }
    
    #[inline]
    pub fn get_config(&self) -> AppConfig {
        self.config.read().clone()
    }
    
    #[inline]
    pub fn update_config(&self, config: AppConfig) {
        *self.config.write() = config;
    }
    
    #[inline]
    pub fn get_serial_state(&self) -> SerialState {
        self.serial.read().clone()
    }
    
    #[inline]
    pub fn update_serial_state<F>(&self, f: F)
    where
        F: FnOnce(&mut SerialState),
    {
        f(&mut *self.serial.write());
    }
    
    #[inline]
    pub async fn send_event(&self, event: AppEvent) {
        let _ = self.event_tx.send(event).await;
    }
    
    pub async fn cleanup(&self) -> AppResult<()> {
        debug!("Cleaning up application state...");
        
        if let Some(pool) = &self.db_pool {
            pool.close().await;
        }
        
        info!("Application state cleaned up");
        Ok(())
    }
    
    pub fn record_operation(&self, command: String, duration_ms: u64, success: bool, error: Option<String>, result_preview: Option<String>) {
        self.session_store.record_operation(command, duration_ms, success, error, result_preview);
    }
    
    pub fn get_current_session(&self) -> Option<Session> {
        self.session_store.get_current_session()
    }
    
    pub fn get_device_store(&self) -> &DeviceStore {
        &self.device_store
    }
    
    pub fn get_config_store(&self) -> &ConfigStore {
        &self.config_store
    }
}
