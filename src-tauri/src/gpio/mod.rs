//! GPIO 控制模块

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::State;
use tokio::sync::Mutex;
use std::sync::Arc;

/// GPIO 引脚状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GPIOPinState {
    High,
    Low,
    Input,
}

/// GPIO 引脚信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPIOPin {
    pin: u8,
    state: GPIOPinState,
    mode: String, // "output" | "input"
}

/// GPIO 状态管理
pub struct GPIOState {
    pins: HashMap<u8, GPIOPin>,
}

impl GPIOState {
    pub fn new() -> Self {
        let mut pins = HashMap::new();
        
        // 初始化常见 GPIO 引脚 (OrangePi Zero 2)
        for pin in &[17, 27, 22, 23, 24, 25, 5, 6] {
            pins.insert(*pin, GPIOPin {
                pin: *pin,
                state: GPIOPinState::Low,
                mode: "output".to_string(),
            });
        }
        
        Self { pins }
    }
}

/// 应用状态 (从 main.rs 导入)
pub struct AppState {
    pub gpio_state: Mutex<GPIOState>,
}

/// GPIO 命令响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GPIOResponse {
    success: bool,
    message: String,
    pin: Option<u8>,
    state: Option<String>,
}

/// 列出所有 GPIO 引脚
#[tauri::command]
pub async fn list_pins(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<GPIOPin>, String> {
    tracing::info!("📋 列出 GPIO 引脚");
    
    let gpio_state = state.gpio_state.lock().await;
    let pins: Vec<GPIOPin> = gpio_state.pins.values().cloned().collect();
    
    Ok(pins)
}

/// 设置 GPIO 引脚状态
#[tauri::command]
pub async fn set_gpio(
    pin: u8,
    value: String,
    state: State<'_, Arc<AppState>>,
) -> Result<GPIOResponse, String> {
    tracing::info!("🔌 设置 GPIO{} = {}", pin, value);
    
    let mut gpio_state = state.gpio_state.lock().await;
    
    if let Some(gpio_pin) = gpio_state.pins.get_mut(&pin) {
        gpio_pin.state = match value.as_str() {
            "high" => GPIOPinState::High,
            "low" => GPIOPinState::Low,
            "input" => GPIOPinState::Input,
            _ => return Err("无效的值，必须是 'high', 'low' 或 'input'".to_string()),
        };
        
        // TODO: 实际 GPIO 操作
        // 使用 rppal::gpio::Gpio
        
        Ok(GPIOResponse {
            success: true,
            message: format!("GPIO{} 设置为 {}", pin, value),
            pin: Some(pin),
            state: Some(value),
        })
    } else {
        Err(format!("GPIO{} 不存在", pin))
    }
}

/// 获取 GPIO 引脚状态
#[tauri::command]
pub async fn get_gpio(
    pin: u8,
    state: State<'_, Arc<AppState>>,
) -> Result<GPIOResponse, String> {
    tracing::info!("📖 读取 GPIO{}", pin);
    
    let gpio_state = state.gpio_state.lock().await;
    
    if let Some(gpio_pin) = gpio_state.pins.get(&pin) {
        let state_str = match gpio_pin.state {
            GPIOPinState::High => "high",
            GPIOPinState::Low => "low",
            GPIOPinState::Input => "input",
        };
        
        Ok(GPIOResponse {
            success: true,
            message: format!("GPIO{} = {}", pin, state_str),
            pin: Some(pin),
            state: Some(state_str.to_string()),
        })
    } else {
        Err(format!("GPIO{} 不存在", pin))
    }
}
