//! OrangePi Debug Tool - Main Entry Point
//! 
//! A comprehensive debugging utility for OrangePi devices

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use orangepi_debug_tool::{cleanup_app, initialize_app};
use tauri::Manager;
use tracing::info;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize application
            let app_handle = app.handle();
            tauri::async_runtime::block_on(async move {
                if let Err(e) = initialize_app(&app_handle).await {
                    eprintln!("Failed to initialize app: {}", e);
                }
            });
            Ok(())
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            // Serial commands
            orangepi_debug_tool::commands::serial::list_serial_ports,
            orangepi_debug_tool::commands::serial::auto_detect_serial,
            orangepi_debug_tool::commands::serial::connect_serial,
            orangepi_debug_tool::commands::serial::disconnect_serial,
            orangepi_debug_tool::commands::serial::write_serial,
            orangepi_debug_tool::commands::serial::write_serial_string,
            orangepi_debug_tool::commands::serial::get_serial_status,
            orangepi_debug_tool::commands::serial::send_command,
            // GPIO commands
            orangepi_debug_tool::commands::gpio::list_gpio_pins,
            orangepi_debug_tool::commands::gpio::configure_gpio,
            orangepi_debug_tool::commands::gpio::read_gpio,
            orangepi_debug_tool::commands::gpio::write_gpio,
            orangepi_debug_tool::commands::gpio::toggle_gpio,
            orangepi_debug_tool::commands::gpio::batch_configure_gpio,
            orangepi_debug_tool::commands::gpio::unconfigure_gpio,
            // PWM commands
            orangepi_debug_tool::commands::pwm::list_pwm_channels,
            orangepi_debug_tool::commands::pwm::configure_pwm,
            orangepi_debug_tool::commands::pwm::set_pwm_frequency,
            orangepi_debug_tool::commands::pwm::set_pwm_duty_cycle,
            orangepi_debug_tool::commands::pwm::set_pwm_enabled,
            orangepi_debug_tool::commands::pwm::get_pwm_info,
            orangepi_debug_tool::commands::pwm::play_pwm_waveform,
            orangepi_debug_tool::commands::pwm::unconfigure_pwm,
            // System commands
            orangepi_debug_tool::commands::system::get_config,
            orangepi_debug_tool::commands::system::update_config,
            orangepi_debug_tool::commands::system::get_system_info,
            orangepi_debug_tool::commands::system::check_orangepi,
            orangepi_debug_tool::commands::system::open_link,
            orangepi_debug_tool::commands::system::save_log,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}