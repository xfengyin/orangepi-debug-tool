use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, info};

use crate::error::{AppError, AppResult};
use crate::adapters::{DeviceInfo, DeviceCapability};

#[derive(Debug, Clone)]
pub struct DeviceState {
    pub id: String,
    pub device_type: DeviceType,
    pub connected: bool,
    pub last_activity: Option<chrono::DateTime<chrono::Utc>>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Serial,
    Gpio,
    Pwm,
    I2C,
    Spi,
}

#[derive(Debug, Clone)]
pub enum DeviceEvent {
    Connected { device_id: String },
    Disconnected { device_id: String },
    Error { device_id: String, error: String },
    DataReceived { device_id: String, data: Vec<u8> },
    DataTransmitted { device_id: String, data: Vec<u8> },
    StateChanged { device_id: String, state: HashMap<String, String> },
}

pub struct DeviceStore {
    devices: Arc<RwLock<HashMap<String, DeviceState>>>,
    event_sender: mpsc::Sender<DeviceEvent>,
    event_receiver: Arc<RwLock<Option<mpsc::Receiver<DeviceEvent>>>>,
    capabilities: Arc<RwLock<HashSet<DeviceCapability>>>,
}

impl DeviceStore {
    pub fn new() -> Self {
        let (event_tx, event_rx) = mpsc::channel(1000);
        
        Self {
            devices: Arc::new(RwLock::new(HashMap::new())),
            event_sender: event_tx,
            event_receiver: Arc::new(RwLock::new(Some(event_rx))),
            capabilities: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    pub fn register_device(&self, info: DeviceInfo) {
        let state = DeviceState {
            id: info.id.clone(),
            device_type: Self::infer_device_type(&info),
            connected: false,
            last_activity: None,
            metadata: HashMap::new(),
        };
        
        self.devices.write().insert(info.id.clone(), state);
        info!("Device registered: {}", info.id);
    }

    pub fn unregister_device(&self, device_id: &str) -> bool {
        let removed = self.devices.write().remove(device_id).is_some();
        if removed {
            info!("Device unregistered: {}", device_id);
        }
        removed
    }

    pub fn set_connected(&self, device_id: &str, connected: bool) -> AppResult<()> {
        let mut devices = self.devices.write();
        
        if let Some(device) = devices.get_mut(device_id) {
            device.connected = connected;
            device.last_activity = Some(chrono::Utc::now());
            
            let event = if connected {
                DeviceEvent::Connected { device_id: device_id.to_string() }
            } else {
                DeviceEvent::Disconnected { device_id: device_id.to_string() }
            };
            
            drop(devices);
            let _ = self.event_sender.try_send(event);
            Ok(())
        } else {
            Err(AppError::NotFound(format!("Device {} not found", device_id)))
        }
    }

    pub fn update_activity(&self, device_id: &str) -> AppResult<()> {
        let mut devices = self.devices.write();
        
        if let Some(device) = devices.get_mut(device_id) {
            device.last_activity = Some(chrono::Utc::now());
            Ok(())
        } else {
            Err(AppError::NotFound(format!("Device {} not found", device_id)))
        }
    }

    pub fn get_device(&self, device_id: &str) -> Option<DeviceState> {
        self.devices.read().get(device_id).cloned()
    }

    pub fn get_all_devices(&self) -> Vec<DeviceState> {
        self.devices.read().values().cloned().collect()
    }

    pub fn get_connected_devices(&self) -> Vec<DeviceState> {
        self.devices.read()
            .values()
            .filter(|d| d.connected)
            .cloned()
            .collect()
    }

    pub fn get_devices_by_type(&self, device_type: DeviceType) -> Vec<DeviceState> {
        self.devices.read()
            .values()
            .filter(|d| d.device_type == device_type)
            .cloned()
            .collect()
    }

    pub fn subscribe(&self) -> mpsc::Receiver<DeviceEvent> {
        let (_, rx) = mpsc::channel(1000);
        rx
    }

    pub fn get_event_sender(&self) -> mpsc::Sender<DeviceEvent> {
        self.event_sender.clone()
    }

    pub fn set_capabilities(&self, capabilities: HashSet<DeviceCapability>) {
        *self.capabilities.write() = capabilities;
    }

    pub fn get_capabilities(&self) -> HashSet<DeviceCapability> {
        self.capabilities.read().clone()
    }

    pub fn has_capability(&self, capability: DeviceCapability) -> bool {
        self.capabilities.read().contains(&capability)
    }

    pub fn clear(&self) {
        self.devices.write().clear();
        debug!("All devices cleared from store");
    }

    fn infer_device_type(info: &DeviceInfo) -> DeviceType {
        if info.capabilities.contains(&DeviceCapability::Serial) {
            DeviceType::Serial
        } else if info.capabilities.contains(&DeviceCapability::Gpio) {
            DeviceType::Gpio
        } else if info.capabilities.contains(&DeviceCapability::Pwm) {
            DeviceType::Pwm
        } else if info.capabilities.contains(&DeviceCapability::I2C) {
            DeviceType::I2C
        } else if info.capabilities.contains(&DeviceCapability::Spi) {
            DeviceType::Spi
        } else {
            DeviceType::Serial
        }
    }
}

impl Default for DeviceStore {
    fn default() -> Self {
        Self::new()
    }
}

use std::collections::HashSet;
