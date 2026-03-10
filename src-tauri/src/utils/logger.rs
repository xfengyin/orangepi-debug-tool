//! 日志工具

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// 初始化日志系统
pub fn init_logger() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}
