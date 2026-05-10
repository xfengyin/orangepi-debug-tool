use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use crate::error::{AppError, AppResult};
use crate::adapters::{DeviceAdapterRegistry, SerialAdapter, GpioAdapter, PwmAdapter};
use crate::config::{AppConfiguration, ConfigLoader};
use crate::services::{ServiceManager, SerialService, GpioService, PwmService};
use crate::observability::{HealthChecker, MetricsCollector, AppTracer};
use crate::devices::{SerialDevice, GpioDevice, PwmDevice};
use tauri::Manager;
use tracing::{info, error};

pub mod config_store;
pub mod device_store;
pub mod session_store;

pub use config_store::ConfigStore;
pub use device_store::DeviceStore;
pub use session_store::{SessionStore, Session, Operation};

pub struct AppState {
    pub config: Arc<RwLock<AppConfiguration>>,
    pub device_registry: Arc<DeviceAdapterRegistry>,
    pub service_manager: Arc<RwLock<Option<ServiceManager>>>,
    pub health_checker: Arc<HealthChecker>,
    pub metrics: Arc<MetricsCollector>,
    pub tracer: Arc<AppTracer>,
    pub serial_manager: Arc<RwLock<Option<SerialDevice>>>,
    pub gpio: Arc<RwLock<GpioDevice>>,
    pub pwm: Arc<RwLock<PwmDevice>>,
    pub device_manager: Arc<RwLock<Option<ServiceManager>>>,
}

impl AppState {
    pub async fn new(app: &tauri::App) -> AppResult<Self> {
        info!("Initializing application state");
        
        let config = ConfigLoader::load()?;
        let device_registry = DeviceAdapterRegistry::new();
        
        let health_checker = Arc::new(HealthChecker::new());
        let metrics = Arc::new(MetricsCollector::new());
        let tracer = Arc::new(AppTracer::new());
        
        let serial_manager = Arc::new(RwLock::new(None));
        let gpio = Arc::new(RwLock::new(GpioDevice::new()));
        let pwm = Arc::new(RwLock::new(PwmDevice::new()));
        let device_manager = Arc::new(RwLock::new(None));
        
        info!("Application state initialized successfully");
        
        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            device_registry: Arc::new(device_registry),
            service_manager: Arc::new(RwLock::new(None)),
            health_checker,
            metrics,
            tracer,
            serial_manager,
            gpio,
            pwm,
            device_manager,
        })
    }
    
    pub async fn cleanup(&self) -> AppResult<()> {
        info!("Cleaning up application state");
        
        if let Some(manager) = self.service_manager.write().take() {
            manager.shutdown().await?;
        }
        
        Ok(())
    }
}
