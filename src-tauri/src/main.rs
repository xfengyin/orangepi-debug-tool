#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod adapters;
mod services;
mod commands;

use std::sync::Arc;

fn main() {
    tauri::Builder::default()
        .manage(services::SerialService::new(Arc::new(
            adapters::serial::NativeSerialAdapter::new(),
        )))
        .manage(services::NetworkService::new(Arc::new(
            adapters::network::TokioNetworkAdapter::new(),
        )))
        .manage(services::LogService::new(std::path::PathBuf::from("./logs")))
        .invoke_handler(tauri::generate_handler![
            commands::list_serial_ports,
            commands::connect_serial,
            commands::disconnect_serial,
            commands::send_serial,
            commands::start_timing_send,
            commands::stop_timing_send,
            commands::get_serial_stats,
            commands::create_tcp_server,
            commands::close_tcp_server,
            commands::accept_tcp_connection,
            commands::connect_tcp,
            commands::disconnect_tcp,
            commands::send_tcp,
            commands::connect_udp,
            commands::disconnect_udp,
            commands::send_udp,
            commands::list_tcp_connections,
            commands::list_udp_sessions,
            commands::list_tcp_servers,
            commands::write_log,
            commands::save_debug_data,
            commands::get_log_stats,
            commands::export_logs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
