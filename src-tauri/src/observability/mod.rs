pub mod health;
pub mod metrics;
pub mod tracing;

pub use health::*;
pub use metrics::*;
pub use tracing::*;

use std::sync::Arc;
use parking_lot::RwLock;
use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct ObservabilityManager {
    health_checker: Arc<HealthChecker>,
    metrics: Arc<MetricsCollector>,
    tracer: Arc<AppTracer>,
}

impl ObservabilityManager {
    pub fn new() -> Self {
        Self {
            health_checker: Arc::new(HealthChecker::new()),
            metrics: Arc::new(MetricsCollector::new()),
            tracer: Arc::new(AppTracer::new()),
        }
    }

    pub fn health_checker(&self) -> Arc<HealthChecker> {
        self.health_checker.clone()
    }

    pub fn metrics(&self) -> Arc<MetricsCollector> {
        self.metrics.clone()
    }

    pub fn tracer(&self) -> Arc<AppTracer> {
        self.tracer.clone()
    }
}

impl Default for ObservabilityManager {
    fn default() -> Self {
        Self::new()
    }
}
