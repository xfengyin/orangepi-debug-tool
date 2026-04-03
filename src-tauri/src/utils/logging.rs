//! Logging utilities

use crate::AppResult;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

/// Initialize application logging
pub fn init_logging() -> AppResult<()> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .compact();
    
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .try_init()
        .ok();
    
    Ok(())
}