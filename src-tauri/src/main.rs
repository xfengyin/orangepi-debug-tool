//! OrangePi Debug Tool - Main Entry Point
//! 
//! A comprehensive debugging utility for OrangePi devices

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use orangepi_debug_tool::{cleanup_app, initialize_app};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tracing::info;

fn main() {
    // Initialize system tray
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "显示窗口"))
        .add_item(CustomMenuItem::new("hide", "隐藏窗口"))
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "退出"));
    
    let system_tray = SystemTray::new().with_menu(tray_menu);
    
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
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "quit" => {
                    let app_handle = app.clone();
                    tauri::async_runtime::block_on(async move {
                        let _ = cleanup_app(&app_handle).await;
                    });
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
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