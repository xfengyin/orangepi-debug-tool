use parking_lot::RwLock;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, info};

use crate::config::{AppConfiguration, ConfigLoader, ConfigValidator};
use crate::error::{AppError, AppResult};

pub struct ConfigStore {
    config: Arc<RwLock<AppConfiguration>>,
    loader: Arc<ConfigLoader>,
    validator: ConfigValidator,
    change_sender: Option<mpsc::Sender<ConfigChangeEvent>>,
}

#[derive(Debug, Clone)]
pub enum ConfigChangeEvent {
    Changed(AppConfiguration),
    Error(String),
}

impl ConfigStore {
    pub fn new(config_path: Option<&std::path::Path>) -> Self {
        let path = config_path.unwrap_or_else(|| std::path::Path::new("config.yaml"));
        let loader = Arc::new(ConfigLoader::new(path).with_hot_reload(true));
        
        Self {
            config: Arc::new(RwLock::new(AppConfiguration::default())),
            loader,
            validator: ConfigValidator::new(),
            change_sender: None,
        }
    }

    pub async fn load(&self) -> AppResult<AppConfiguration> {
        let config = self.loader.load().await
            .map_err(|e| AppError::Config(e.to_string()))?;
        
        if let Err(e) = self.validator.validate(&config) {
            info!("Config validation warning: {}", e);
        }
        
        *self.config.write() = config.clone();
        Ok(config)
    }

    pub async fn save(&self, config: &AppConfiguration) -> AppResult<()> {
        if let Err(e) = self.validator.validate(config) {
            return Err(AppError::Config(format!("Validation failed: {}", e)));
        }
        
        self.loader.save(config).await
            .map_err(|e| AppError::Config(e.to_string()))?;
        
        *self.config.write() = config.clone();
        
        if let Some(ref sender) = self.change_sender {
            let _ = sender.send(ConfigChangeEvent::Changed(config.clone())).await;
        }
        
        Ok(())
    }

    pub fn get_config(&self) -> AppConfiguration {
        self.config.read().clone()
    }

    pub fn get_cached(&self) -> Arc<AppConfiguration> {
        Arc::new(self.config.read().clone())
    }

    pub fn update_config<F>(&self, f: F) -> AppResult<()>
    where
        F: FnOnce(&mut AppConfiguration),
    {
        let mut config = self.config.write();
        f(&mut config);
        debug!("Configuration updated");
        Ok(())
    }

    pub fn subscribe(&mut self) -> mpsc::Receiver<ConfigChangeEvent> {
        let (tx, rx) = mpsc::channel(100);
        self.change_sender = Some(tx);
        rx
    }

    pub fn merge_with_env(&self) {
        let mut config = self.config.write();
        self.loader.merge_with_env(&mut config);
    }

    pub fn get_system_config(&self) -> crate::config::SystemConfig {
        self.config.read().system.clone()
    }

    pub fn get_device_config(&self) -> crate::config::DeviceConfigSection {
        self.config.read().devices.clone()
    }

    pub fn get_security_config(&self) -> crate::config::SecurityConfig {
        self.config.read().security.clone()
    }

    pub fn get_observability_config(&self) -> crate::config::ObservabilityConfig {
        self.config.read().observability.clone()
    }
}

impl Default for ConfigStore {
    fn default() -> Self {
        Self::new(None)
    }
}
