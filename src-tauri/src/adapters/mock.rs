use async_trait::async_trait;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use crate::error::{AppError, AppResult};

use super::traits::*;
use super::HealthStatus;

#[derive(Debug)]
pub struct MockAdapter {
    config: MockConfig,
    state: Arc<Mutex<MockState>>,
}

#[derive(Debug, Clone)]
pub struct MockConfig {
    pub simulate_delay_ms: u64,
    pub inject_errors: bool,
    pub error_rate: f32,
    pub data_pattern: MockDataPattern,
}

impl Default for MockConfig {
    fn default() -> Self {
        Self {
            simulate_delay_ms: 10,
            inject_errors: false,
            error_rate: 0.0,
            data_pattern: MockDataPattern::Incrementing,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MockDataPattern {
    Incrementing,
    Random,
    Fixed,
    Alternating,
}

#[derive(Debug)]
struct MockState {
    serial_connected: bool,
    serial_buffer: Vec<u8>,
    gpio_exported: HashSet<u32>,
    gpio_values: HashMap<u32, u8>,
    pwm_enabled: HashSet<u32>,
    pwm_frequencies: HashMap<u32, u32>,
    pwm_duty_cycles: HashMap<u32, f64>,
    call_counts: MockCallCounts,
}

#[derive(Debug, Default)]
struct MockCallCounts {
    serial_read: u64,
    serial_write: u64,
    gpio_read: u64,
    gpio_write: u64,
    pwm_set_frequency: u64,
    pwm_set_duty_cycle: u64,
}

impl MockAdapter {
    pub fn new() -> Self {
        Self::with_config(MockConfig::default())
    }

    pub fn with_config(config: MockConfig) -> Self {
        Self {
            config,
            state: Arc::new(Mutex::new(MockState::default())),
        }
    }

    pub fn with_delay(mut self, delay_ms: u64) -> Self {
        self.config.simulate_delay_ms = delay_ms;
        self
    }

    pub fn with_error_injection(mut self, rate: f32) -> Self {
        self.config.inject_errors = true;
        self.config.error_rate = rate;
        self
    }

    async fn simulate_delay(&self) {
        if self.config.simulate_delay_ms > 0 {
            sleep(Duration::from_millis(self.config.simulate_delay_ms)).await;
        }
    }

    fn should_inject_error(&self) -> bool {
        if self.config.inject_errors && self.config.error_rate > 0.0 {
            use std::time::{SystemTime, UNIX_EPOCH};
            let seed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos();
            (seed as f32 / u32::MAX as f32) < self.config.error_rate
        } else {
            false
        }
    }

    fn generate_mock_data(&self, size: usize) -> Vec<u8> {
        match self.config.data_pattern {
            MockDataPattern::Incrementing => (0..size as u8).collect(),
            MockDataPattern::Random => {
                use std::time::{SystemTime, UNIX_EPOCH};
                let seed = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos();
                (0..size).map(|i| ((seed + i as u128) % 256) as u8).collect()
            }
            MockDataPattern::Fixed => vec![0xAA; size],
            MockDataPattern::Alternating => (0..size).map(|i| if i % 2 == 0 { 0x55 } else { 0xAA }).collect(),
        }
    }
}

impl Default for MockState {
    fn default() -> Self {
        Self {
            serial_connected: false,
            serial_buffer: Vec::new(),
            gpio_exported: HashSet::new(),
            gpio_values: HashMap::new(),
            pwm_enabled: HashSet::new(),
            pwm_frequencies: HashMap::new(),
            pwm_duty_cycles: HashMap::new(),
            call_counts: MockCallCounts::default(),
        }
    }
}

impl Default for MockAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DeviceAdapter for MockAdapter {
    fn id(&self) -> &'static str {
        "mock"
    }

    fn name(&self) -> &str {
        "Mock Adapter (Testing)"
    }

    fn capabilities(&self) -> HashSet<DeviceCapability> {
        let mut caps = HashSet::new();
        caps.insert(DeviceCapability::Serial);
        caps.insert(DeviceCapability::Gpio);
        caps.insert(DeviceCapability::Pwm);
        caps
    }

    async fn health_check(&self) -> AppResult<HealthStatus> {
        self.simulate_delay().await;
        Ok(HealthStatus::healthy(self.id()))
    }
}

#[async_trait]
impl SerialAdapter for MockAdapter {
    async fn list_ports(&self) -> AppResult<Vec<SerialPortInfo>> {
        self.simulate_delay().await;
        
        Ok(vec![
            SerialPortInfo {
                port_name: "/dev/ttyUSB0".to_string(),
                port_type: "USB-Serial".to_string(),
                vid: Some(0x1a86),
                pid: Some(0x7523),
                serial_number: Some("MOCK001".to_string()),
                manufacturer: Some("Mock Manufacturer".to_string()),
                product: Some("Mock USB-UART".to_string()),
            },
            SerialPortInfo {
                port_name: "/dev/ttyUSB1".to_string(),
                port_type: "USB-Serial".to_string(),
                vid: Some(0x067b),
                pid: Some(0x2303),
                serial_number: Some("MOCK002".to_string()),
                manufacturer: Some("Mock Manufacturer".to_string()),
                product: Some("Mock Serial Port".to_string()),
            },
        ])
    }

    async fn connect(&self, config: SerialConfig) -> AppResult<SerialHandle> {
        self.simulate_delay().await;
        
        if self.should_inject_error() {
            return Err(AppError::Serial("Mock connection error".to_string()));
        }
        
        let mut state = self.state.lock().await;
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
        
        let mut state = self.state.lock().await;
        state.serial_connected = false;
        state.serial_buffer.clear();
        
        Ok(())
    }

    async fn read(&self, _handle: &SerialHandle, buffer: &mut [u8]) -> AppResult<usize> {
        self.simulate_delay().await;
        
        let mut state = self.state.lock().await;
        state.call_counts.serial_read += 1;
        
        if self.should_inject_error() {
            return Err(AppError::Serial("Mock read error".to_string()));
        }
        
        if state.serial_buffer.is_empty() {
            let mock_data = self.generate_mock_data(buffer.len().min(64));
            let len = mock_data.len().min(buffer.len());
            buffer[..len].copy_from_slice(&mock_data[..len]);
            Ok(len)
        } else {
            let len = state.serial_buffer.len().min(buffer.len());
            buffer[..len].copy_from_slice(&state.serial_buffer[..len]);
            state.serial_buffer.drain(..len);
            Ok(len)
        }
    }

    async fn write(&self, _handle: &SerialHandle, data: &[u8]) -> AppResult<usize> {
        self.simulate_delay().await;
        
        let mut state = self.state.lock().await;
        state.call_counts.serial_write += 1;
        
        if self.should_inject_error() {
            return Err(AppError::Serial("Mock write error".to_string()));
        }
        
        state.serial_buffer.extend_from_slice(data);
        Ok(data.len())
    }

    async fn set_baudrate(&self, _handle: &SerialHandle, _baudrate: u32) -> AppResult<()> {
        self.simulate_delay().await;
        Ok(())
    }
}

#[async_trait]
impl GpioAdapter for MockAdapter {
    async fn list_pins(&self) -> AppResult<Vec<GpioPinInfo>> {
        self.simulate_delay().await;
        
        Ok(vec![
            GpioPinInfo {
                pin: 3,
                name: "GPIO3".to_string(),
                modes: vec!["gpio".to_string(), "i2c".to_string()],
                current_mode: None,
                is_exported: false,
            },
            GpioPinInfo {
                pin: 5,
                name: "GPIO5".to_string(),
                modes: vec!["gpio".to_string(), "i2c".to_string()],
                current_mode: None,
                is_exported: false,
            },
            GpioPinInfo {
                pin: 7,
                name: "GPIO7".to_string(),
                modes: vec!["gpio".to_string()],
                current_mode: None,
                is_exported: false,
            },
            GpioPinInfo {
                pin: 11,
                name: "GPIO11".to_string(),
                modes: vec!["gpio".to_string(), "spi".to_string()],
                current_mode: None,
                is_exported: false,
            },
        ])
    }

    async fn export_pin(&self, pin: u32) -> AppResult<()> {
        self.simulate_delay().await;
        
        if self.should_inject_error() {
            return Err(AppError::Gpio(format!("Mock export error for pin {}", pin)));
        }
        
        let mut state = self.state.lock().await;
        state.gpio_exported.insert(pin);
        state.gpio_values.insert(pin, 0);
        
        Ok(())
    }

    async fn unexport_pin(&self, pin: u32) -> AppResult<()> {
        self.simulate_delay().await;
        
        let mut state = self.state.lock().await;
        state.gpio_exported.remove(&pin);
        state.gpio_values.remove(&pin);
        
        Ok(())
    }

    async fn set_direction(&self, _pin: u32, _direction: GpioDirection) -> AppResult<()> {
        self.simulate_delay().await;
        Ok(())
    }

    async fn set_pull(&self, _pin: u32, _pull: GpioPull) -> AppResult<()> {
        self.simulate_delay().await;
        Ok(())
    }

    async fn read_pin(&self, pin: u32) -> AppResult<u8> {
        self.simulate_delay().await;
        
        let mut state = self.state.lock().await;
        state.call_counts.gpio_read += 1;
        
        if self.should_inject_error() {
            return Err(AppError::Gpio(format!("Mock read error for pin {}", pin)));
        }
        
        state.gpio_values.get(&pin).copied().ok_or_else(|| {
            AppError::Gpio(format!("Pin {} not exported", pin))
        })
    }

    async fn write_pin(&self, pin: u32, value: u8) -> AppResult<()> {
        self.simulate_delay().await;
        
        let mut state = self.state.lock().await;
        state.call_counts.gpio_write += 1;
        
        if self.should_inject_error() {
            return Err(AppError::Gpio(format!("Mock write error for pin {}", pin)));
        }
        
        if !state.gpio_exported.contains(&pin) {
            return Err(AppError::Gpio(format!("Pin {} not exported", pin)));
        }
        
        state.gpio_values.insert(pin, value);
        Ok(())
    }

    async fn enable_interrupt(&self, _pin: u32, _trigger: GpioTrigger) -> AppResult<()> {
        self.simulate_delay().await;
        Ok(())
    }

    async fn disable_interrupt(&self, _pin: u32) -> AppResult<()> {
        self.simulate_delay().await;
        Ok(())
    }
}

#[async_trait]
impl PwmAdapter for MockAdapter {
    async fn list_channels(&self) -> AppResult<Vec<PwmChannelInfo>> {
        self.simulate_delay().await;
        
        Ok(vec![
            PwmChannelInfo {
                channel: 0,
                name: "PWM0".to_string(),
                enabled: false,
                frequency_hz: None,
                duty_cycle: None,
            },
            PwmChannelInfo {
                channel: 1,
                name: "PWM1".to_string(),
                enabled: false,
                frequency_hz: None,
                duty_cycle: None,
            },
        ])
    }

    async fn enable_channel(&self, channel: u32) -> AppResult<()> {
        self.simulate_delay().await;
        
        if self.should_inject_error() {
            return Err(AppError::Pwm(format!("Mock enable error for channel {}", channel)));
        }
        
        let mut state = self.state.lock().await;
        state.pwm_enabled.insert(channel);
        state.pwm_frequencies.entry(channel).or_insert(1000);
        state.pwm_duty_cycles.entry(channel).or_insert(50.0);
        
        Ok(())
    }

    async fn disable_channel(&self, channel: u32) -> AppResult<()> {
        self.simulate_delay().await;
        
        let mut state = self.state.lock().await;
        state.pwm_enabled.remove(&channel);
        
        Ok(())
    }

    async fn set_frequency(&self, channel: u32, frequency_hz: u32) -> AppResult<()> {
        self.simulate_delay().await;
        
        let mut state = self.state.lock().await;
        state.call_counts.pwm_set_frequency += 1;
        
        if self.should_inject_error() {
            return Err(AppError::Pwm(format!("Mock frequency error for channel {}", channel)));
        }
        
        if !state.pwm_enabled.contains(&channel) {
            return Err(AppError::Pwm(format!("Channel {} not enabled", channel)));
        }
        
        state.pwm_frequencies.insert(channel, frequency_hz);
        Ok(())
    }

    async fn set_duty_cycle(&self, channel: u32, duty_percent: f64) -> AppResult<()> {
        self.simulate_delay().await;
        
        let mut state = self.state.lock().await;
        state.call_counts.pwm_set_duty_cycle += 1;
        
        if self.should_inject_error() {
            return Err(AppError::Pwm(format!("Mock duty cycle error for channel {}", channel)));
        }
        
        if !state.pwm_enabled.contains(&channel) {
            return Err(AppError::Pwm(format!("Channel {} not enabled", channel)));
        }
        
        if !(0.0..=100.0).contains(&duty_percent) {
            return Err(AppError::InvalidArgument("Duty cycle must be between 0 and 100".to_string()));
        }
        
        state.pwm_duty_cycles.insert(channel, duty_percent);
        Ok(())
    }
}

impl MockAdapter {
    pub async fn get_call_counts(&self) -> MockCallCounts {
        let state = self.state.lock().await;
        state.call_counts.clone()
    }

    pub async fn get_gpio_values(&self) -> std::collections::HashMap<u32, u8> {
        let state = self.state.lock().await;
        state.gpio_values.clone()
    }

    pub async fn get_pwm_state(&self) -> (HashSet<u32>, HashMap<u32, u32>, HashMap<u32, f64>) {
        let state = self.state.lock().await;
        (
            state.pwm_enabled.clone(),
            state.pwm_frequencies.clone(),
            state.pwm_duty_cycles.clone(),
        )
    }

    pub async fn reset(&self) {
        let mut state = self.state.lock().await;
        *state = MockState::default();
    }
}

use std::collections::HashMap;
