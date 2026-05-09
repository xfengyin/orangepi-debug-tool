use parking_lot::RwLock;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::error::{AppError, AppResult};

use super::schema::*;

#[derive(Debug)]
pub enum ConfigError {
    Io(String),
    Parse(String),
    Validation(String),
    NotFound(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(msg) => write!(f, "IO error: {}", msg),
            ConfigError::Parse(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::Validation(msg) => write!(f, "Validation error: {}", msg),
            ConfigError::NotFound(msg) => write!(f, "Config not found: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::Io(err.to_string())
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(err: serde_yaml::Error) -> Self {
        ConfigError::Parse(err.to_string())
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::Parse(err.to_string())
    }
}

pub struct ConfigLoader {
    config_path: PathBuf,
    hot_reload_enabled: bool,
    cache: RwLock<AppConfiguration>,
    change_sender: Option<mpsc::Sender<ConfigChangeEvent>>,
}

#[derive(Debug, Clone)]
pub enum ConfigChangeEvent {
    Reloaded(AppConfiguration),
    Error(ConfigError),
}

impl ConfigLoader {
    pub fn new<P: AsRef<Path>>(config_path: P) -> Self {
        Self {
            config_path: config_path.as_ref().to_path_buf(),
            hot_reload_enabled: false,
            cache: RwLock::new(AppConfiguration::default()),
            change_sender: None,
        }
    }

    pub fn with_hot_reload(mut self, enabled: bool) -> Self {
        self.hot_reload_enabled = enabled;
        self
    }

    pub async fn load(&self) -> Result<AppConfiguration, ConfigError> {
        if !self.config_path.exists() {
            info!("Config file not found, using defaults: {}", self.config_path.display());
            let default_config = AppConfiguration::default();
            *self.cache.write() = default_config.clone();
            return Ok(default_config);
        }

        let content = tokio::fs::read_to_string(&self.config_path).await?;

        let extension = self.config_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("yaml");

        let config: AppConfiguration = match extension {
            "json" => serde_json::from_str(&content)?,
            "yaml" | "yml" => serde_yaml::from_str(&content)?,
            _ => serde_yaml::from_str(&content)?,
        };

        *self.cache.write() = config.clone();
        info!("Configuration loaded successfully from: {}", self.config_path.display());
        Ok(config)
    }

    pub async fn save(&self, config: &AppConfiguration) -> Result<(), ConfigError> {
        let extension = self.config_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("yaml");

        let content: String = match extension {
            "json" => serde_json::to_string_pretty(config)?,
            "yaml" | "yml" => serde_yaml::to_string(config)?,
            _ => serde_yaml::to_string(config)?,
        };

        if let Some(parent) = self.config_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::write(&self.config_path, content).await?;
        *self.cache.write() = config.clone();
        info!("Configuration saved to: {}", self.config_path.display());
        Ok(())
    }

    pub fn get_config(&self) -> AppConfiguration {
        self.cache.read().clone()
    }

    pub fn get_cached(&self) -> Arc<AppConfiguration> {
        Arc::new(self.cache.read().clone())
    }

    pub fn watch<F>(&mut self, callback: F) -> Result<(), ConfigError>
    where
        F: Fn(AppConfiguration) + Send + Sync + 'static,
    {
        if !self.hot_reload_enabled {
            warn!("Hot reload is not enabled");
            return Ok(());
        }

        let config_path = self.config_path.clone();
        let cache = Arc::new(RwLock::new(self.cache.read().clone()));

        std::thread::spawn(move || {
            let mut watcher = match notify::recommended_watcher(move |res: Result<_, notify::Error>| {
                if let Ok(event) = res {
                    if event.kind.is_modify() {
                        debug!("Config file changed: {:?}", event);
                    }
                }
            }) {
                Ok(w) => w,
                Err(e) => {
                    error!("Failed to create config watcher: {}", e);
                    return;
                }
            };

            if let Err(e) = watcher.watch(&config_path, notify::RecursiveMode::NonRecursive) {
                error!("Failed to watch config file: {}", e);
            }

            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });

        Ok(())
    }

    pub fn merge_with_env(&self, config: &mut AppConfiguration) {
        if let Ok(log_level) = std::env::var("LOG_LEVEL") {
            if let Ok(level) = log_level.parse() {
                config.system.log_level = level;
            }
        }

        if let Ok(max_tasks) = std::env::var("MAX_CONCURRENT_TASKS") {
            if let Ok(tasks) = max_tasks.parse() {
                config.system.max_concurrent_tasks = tasks;
            }
        }

        if let Ok(timeout) = std::env::var("TASK_TIMEOUT_SECONDS") {
            if let Ok(secs) = timeout.parse() {
                config.system.task_timeout_seconds = secs;
            }
        }

        if let Ok(adapter) = std::env::var("SERIAL_DEFAULT_ADAPTER") {
            config.devices.serial.default_adapter = adapter;
        }

        if let Ok(baudrate) = std::env::var("SERIAL_DEFAULT_BAUDRATE") {
            if let Ok(rate) = baudrate.parse() {
                config.devices.serial.auto_detect.default_baudrate = rate;
            }
        }

        debug!("Configuration merged with environment variables");
    }

    pub fn get_config_path(&self) -> &Path {
        &self.config_path
    }
}

pub struct ConfigWatcher {
    path: PathBuf,
    last_modified: std::time::SystemTime,
}

impl ConfigWatcher {
    pub fn new(path: PathBuf) -> std::io::Result<Self> {
        let last_modified = std::fs::metadata(&path)?
            .modified()?;
        Ok(Self { path, last_modified })
    }

    pub fn has_changed(&self) -> bool {
        if let Ok(metadata) = std::fs::metadata(&self.path) {
            if let Ok(modified) = metadata.modified() {
                return modified > self.last_modified;
            }
        }
        false
    }

    pub fn update(&mut self) {
        if let Ok(metadata) = std::fs::metadata(&self.path) {
            if let Ok(modified) = metadata.modified() {
                self.last_modified = modified;
            }
        }
    }
}
