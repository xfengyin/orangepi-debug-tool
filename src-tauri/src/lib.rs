//! OrangePi Debug Tool - Core Library
//! 
//! A comprehensive debugging utility for OrangePi devices with support for:
//! - Serial communication
//! - GPIO control
//! - PWM output
//! - Data logging and visualization

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

pub mod commands;
pub mod devices;
pub mod error;
pub mod state;
pub mod utils;

pub use error::{AppError, AppResult};
pub use state::AppState;

use tauri::Manager;
use tracing::info;

/// Initialize the application state and logging
pub async fn initialize_app(app: &tauri::App) -> AppResult<()> {
    // Initialize logging
    utils::logging::init_logging()?;
    
    info!("OrangePi Debug Tool v2.0.0 starting...");
    
    // Initialize application state
    let state = AppState::new(app).await?;
    app.manage(state);
    
    info!("Application initialized successfully");
    Ok(())
}

/// Run cleanup tasks before application exit
pub async fn cleanup_app(app: &tauri::AppHandle) -> AppResult<()> {
    info!("Application shutting down...");
    
    if let Some(state) = app.try_state::<AppState>() {
        state.cleanup().await?;
    }
    
    info!("Cleanup completed");
    Ok(())
}