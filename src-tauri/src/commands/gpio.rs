//! GPIO command handlers

use crate::commands::{into_response, ApiResponse};
use crate::devices::gpio::{GpioConfig, GpioPinInfo};
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

/// List available GPIO pins
#[tauri::command]
pub async fn list_gpio_pins(state: State<'_, AppState>) -> Result<Vec<GpioPinInfo>, String> {
    let gpio = state.gpio.read();
    Ok(gpio.list_pins())
}

/// Configure a GPIO pin
#[tauri::command]
pub async fn configure_gpio(
    config: GpioConfig,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut gpio = state.gpio.write();
    gpio.configure_pin(config).map_err(|e| e.to_string())
}

/// Read GPIO pin value
#[tauri::command]
pub async fn read_gpio(pin: u32, state: State<'_, AppState>) -> Result<u8, String> {
    let gpio = state.gpio.read();
    gpio.read_pin(pin).map_err(|e| e.to_string())
}

/// Write GPIO pin value
#[tauri::command]
pub async fn write_gpio(
    pin: u32,
    value: u8,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut gpio = state.gpio.write();
    gpio.write_pin(pin, value).map_err(|e| e.to_string())
}

/// Toggle GPIO pin value
#[tauri::command]
pub async fn toggle_gpio(pin: u32, state: State<'_, AppState>) -> Result<u8, String> {
    let mut gpio = state.gpio.write();
    gpio.toggle_pin(pin).map_err(|e| e.to_string())
}

/// Batch configure GPIO pins
#[tauri::command]
pub async fn batch_configure_gpio(
    configs: Vec<GpioConfig>,
    state: State<'_, AppState>,
) -> Result<Vec<()>, String> {
    let mut gpio = state.gpio.write();
    let mut results = Vec::new();
    for config in configs {
        match gpio.configure_pin(config) {
            Ok(_) => results.push(()),
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(results)
}

/// Unconfigure a GPIO pin
#[tauri::command]
pub async fn unconfigure_gpio(pin: u32, state: State<'_, AppState>) -> Result<(), String> {
    let mut gpio = state.gpio.write();
    gpio.unconfigure_pin(pin).map_err(|e| e.to_string())
}
