//! GPIO command handlers

use crate::commands::{into_response, ApiResponse};
use crate::devices::gpio::{GpioConfig, GpioPinInfo};
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

/// List available GPIO pins
#[tauri::command]
pub async fn list_gpio_pins(state: State<'_, AppState>) -> ApiResponse<Vec<GpioPinInfo>> {
    let gpio = state.device_manager.lock();
    // This would need to be implemented properly in DeviceManager
    into_response(Ok(Vec::new()))
}

/// Configure a GPIO pin
#[tauri::command]
pub async fn configure_gpio(
    config: GpioConfig,
    state: State<'_, AppState>,
) -> ApiResponse<()> {
    into_response(async {
        let mut gpio = state.gpio.write();
        gpio.configure_pin(config)
    }())
}

/// Read GPIO pin value
#[tauri::command]
pub async fn read_gpio(pin: u32, state: State<'_, AppState>) -> ApiResponse<u8> {
    into_response(async {
        let gpio = state.gpio.read();
        gpio.read_pin(pin)
    }())
}

/// Write GPIO pin value
#[tauri::command]
pub async fn write_gpio(
    pin: u32,
    value: u8,
    state: State<'_, AppState>,
) -> ApiResponse<()> {
    into_response(async {
        let mut gpio = state.gpio.write();
        gpio.write_pin(pin, value)
    }())
}

/// Toggle GPIO pin value
#[tauri::command]
pub async fn toggle_gpio(pin: u32, state: State<'_, AppState>) -> ApiResponse<u8> {
    into_response(async {
        let mut gpio = state.gpio.write();
        gpio.toggle_pin(pin)
    }())
}

/// Batch configure GPIO pins
#[tauri::command]
pub async fn batch_configure_gpio(
    configs: Vec<GpioConfig>,
    state: State<'_, AppState>,
) -> ApiResponse<Vec<ApiResponse<()>>> {
    let mut gpio = state.gpio.write();
    let results = gpio.batch_configure(configs);
    
    ApiResponse::success(results.into_iter().map(into_response).collect())
}

/// Unconfigure a GPIO pin
#[tauri::command]
pub async fn unconfigure_gpio(pin: u32, state: State<'_, AppState>) -> ApiResponse<()> {
    into_response(async {
        let mut gpio = state.gpio.write();
        gpio.unconfigure_pin(pin)
    }())
}