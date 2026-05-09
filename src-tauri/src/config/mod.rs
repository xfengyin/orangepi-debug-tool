pub mod schema;
pub mod loader;
pub mod validator;

pub use schema::*;
pub use loader::*;
pub use validator::*;

use std::path::PathBuf;

#[derive(Debug)]
pub struct ConfigPaths {
    pub config_dir: PathBuf,
    pub main_config: PathBuf,
    pub device_config: PathBuf,
    pub plugin_config: PathBuf,
}

impl ConfigPaths {
    pub fn new(app_name: &str) -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(app_name);
        
        Self {
            config_dir: config_dir.clone(),
            main_config: config_dir.join("config.yaml"),
            device_config: config_dir.join("devices.yaml"),
            plugin_config: config_dir.join("plugins.yaml"),
        }
    }

    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.config_dir)
    }
}
