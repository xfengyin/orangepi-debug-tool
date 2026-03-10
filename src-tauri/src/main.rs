// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod serial;
mod gpio;
mod pwm;
mod utils;
mod ui;

use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::sync::Mutex;
use std::sync::Arc;

/// 应用状态
struct AppState {
    serial_port: Mutex<Option<serial::SerialPort>>,
    gpio_state: Mutex<gpio::GPIOState>,
}

/// 串口命令
#[derive(Debug, Serialize, Deserialize, Clone)]
struct SerialCommand {
    port: String,
    baudrate: u32,
    action: String, // "connect" | "disconnect" | "send"
    data: Option<String>,
    hex_mode: Option<bool>,
}

/// GPIO 命令
#[derive(Debug, Serialize, Deserialize, Clone)]
struct GPIOCommand {
    pin: u8,
    value: String, // "high" | "low" | "input"
}

/// PWM 命令
#[derive(Debug, Serialize, Deserialize, Clone)]
struct PWMCommand {
    channel: u8,
    frequency: f64,
    duty_cycle: f64,
}

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("orangepi_debug_tool=debug".parse().unwrap()),
        )
        .init();

    tracing::info!("🚀 OrangePi Debug Tool v2.0 启动");

    let app_state = AppState {
        serial_port: Mutex::new(None),
        gpio_state: Mutex::new(gpio::GPIOState::new()),
    };

    tauri::Builder::default()
        .manage(Arc::new(app_state))
        .invoke_handler(tauri::generate_handler![
            serial::connect_serial,
            serial::disconnect_serial,
            serial::send_data,
            serial::list_ports,
            gpio::set_gpio,
            gpio::get_gpio,
            gpio::list_pins,
            pwm::set_pwm,
            pwm::stop_pwm,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
