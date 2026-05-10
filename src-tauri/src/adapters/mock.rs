use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use parking_lot::Mutex;
use crate::error::{AppError, AppResult};
use super::traits::{SerialAdapter, SerialConfig, SerialHandle};

pub struct MockAdapter {
    pub delay_ms: u64,
    pub error_rate: f32,
    state: Arc<Mutex<MockState>>,
}

#[derive(Default)]
pub struct MockState {
    serial_connected: bool,
    serial_buffer: Vec<u8>,
    gpio_states: HashMap<u32, bool>,
    pwm_states: HashMap<u32, (f64, f64, bool)>,
}

impl MockAdapter {
    pub fn new(delay_ms: u64, error_rate: f32) -> Self {
        Self {
            delay_ms,
            error_rate,
            state: Arc::new(Mutex::new(MockState::default())),
        }
    }
    
    async fn simulate_delay(&self) {
        if self.delay_ms > 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(self.delay_ms)).await;
        }
    }
    
    fn should_inject_error(&self) -> bool {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() as f32;
        (nanos / u32::MAX as f32) < self.error_rate
    }
}

#[async_trait]
impl SerialAdapter for MockAdapter {
    fn list_ports() -> Vec<String> {
        vec![
            "/dev/ttyUSB0".to_string(),
            "/dev/ttyUSB1".to_string(),
            "/dev/ttyACM0".to_string(),
        ]
    }
    
    async fn connect(&self, config: SerialConfig) -> AppResult<SerialHandle> {
        self.simulate_delay().await;
        
        if self.should_inject_error() {
            return Err(AppError::Serial("Mock connection error".to_string()));
        }
        
        let mut state = self.state.lock();
        state.serial_connected = true;
        state.serial_buffer.clear();
        
        Ok(SerialHandle {
            port_name: config.port_name,
            config,
            #[cfg(feature = "hardware-support")]
            stream: unsafe { std::mem::zeroed() },
        })
    }
    
    async fn disconnect(&self, _handle: SerialHandle) -> AppResult<()> {
        self.simulate_delay().await;
        
        let mut state = self.state.lock();
        state.serial_connected = false;
        state.serial_buffer.clear();
        
        Ok(())
    }
    
    async fn read(&self, _handle: &SerialHandle, buffer: &mut [u8]) -> AppResult<usize> {
        self.simulate_delay().await;
        
        let mut state = self.state.lock();
        if state.serial_buffer.is_empty() {
            return Ok(0);
        }
        
        let len = std::cmp::min(buffer.len(), state.serial_buffer.len());
        buffer[..len].copy_from_slice(&state.serial_buffer[..len]);
        state.serial_buffer.drain(..len);
        Ok(len)
    }
    
    async fn write(&self, _handle: &SerialHandle, data: &[u8]) -> AppResult<usize> {
        self.simulate_delay().await;
        
        let mut state = self.state.lock();
        state.serial_buffer.extend_from_slice(data);
        Ok(data.len())
    }
    
    async fn set_baudrate(&self, _handle: &SerialHandle, _baudrate: u32) -> AppResult<()> {
        self.simulate_delay().await;
        Ok(())
    }
    
    fn is_connected(&self) -> bool {
        self.state.lock().serial_connected
    }
}
