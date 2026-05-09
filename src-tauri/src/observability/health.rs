use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthState {
    Healthy,
    Degraded,
    Unhealthy,
}

impl std::fmt::Display for HealthState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthState::Healthy => write!(f, "healthy"),
            HealthState::Degraded => write!(f, "degraded"),
            HealthState::Unhealthy => write!(f, "unhealthy"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub name: String,
    pub status: HealthState,
    pub message: Option<String>,
    pub latency_ms: Option<u64>,
}

impl ComponentHealth {
    pub fn healthy(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: HealthState::Healthy,
            message: None,
            latency_ms: None,
        }
    }

    pub fn degraded(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: HealthState::Degraded,
            message: Some(message.into()),
            latency_ms: None,
        }
    }

    pub fn unhealthy(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: HealthState::Unhealthy,
            message: Some(message.into()),
            latency_ms: None,
        }
    }

    #[inline]
    pub fn is_healthy(&self) -> bool {
        self.status == HealthState::Healthy
    }

    #[inline]
    pub fn with_latency(mut self, latency_ms: u64) -> Self {
        self.latency_ms = Some(latency_ms);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: HealthState,
    pub components: HashMap<String, ComponentHealth>,
    pub timestamp: DateTime<Utc>,
}

impl HealthStatus {
    pub fn new() -> Self {
        Self {
            status: HealthState::Healthy,
            components: HashMap::new(),
            timestamp: Utc::now(),
        }
    }

    pub fn add_component(&mut self, component: ComponentHealth) {
        if component.status == HealthState::Unhealthy {
            self.status = HealthState::Unhealthy;
        } else if component.status == HealthState::Degraded && self.status == HealthState::Healthy {
            self.status = HealthState::Degraded;
        }
        self.components.insert(component.name.clone(), component);
    }

    #[inline]
    pub fn is_healthy(&self) -> bool {
        self.status == HealthState::Healthy
    }

    #[inline]
    pub fn is_degraded(&self) -> bool {
        self.status == HealthState::Degraded
    }

    #[inline]
    pub fn is_unhealthy(&self) -> bool {
        self.status == HealthState::Unhealthy
    }
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
pub trait HealthCheck: Send + Sync {
    fn name(&self) -> &str;
    async fn check(&self) -> ComponentHealth;
}

#[derive(Debug)]
pub struct HealthChecker {
    checks: Vec<Arc<dyn HealthCheck>>,
}

impl HealthChecker {
    pub fn new() -> Self {
        Self {
            checks: Vec::new(),
        }
    }

    pub fn register<H: HealthCheck + 'static>(&mut self, check: H) {
        self.checks.push(Arc::new(check));
    }

    pub fn register_boxed(&mut self, check: Arc<dyn HealthCheck>) {
        self.checks.push(check);
    }

    pub async fn check_all(&self) -> HealthStatus {
        let mut status = HealthStatus::new();

        for check in &self.checks {
            let component_health = check.check().await;
            status.add_component(component_health);
        }

        status
    }

    pub fn list_checks(&self) -> Vec<String> {
        self.checks.iter().map(|c| c.name().to_string()).collect()
    }

    pub fn unregister(&mut self, name: &str) -> bool {
        let original_len = self.checks.len();
        self.checks.retain(|c| c.name() != name);
        self.checks.len() < original_len
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct LivenessChecker {
    startup_time: std::time::Instant,
}

impl LivenessChecker {
    pub fn new() -> Self {
        Self {
            startup_time: std::time::Instant::now(),
        }
    }
}

impl HealthCheck for LivenessChecker {
    fn name(&self) -> &str {
        "liveness"
    }

    async fn check(&self) -> ComponentHealth {
        ComponentHealth::healthy("process").with_latency(0)
    }
}

impl Default for LivenessChecker {
    fn default() -> Self {
        Self::new()
    }
}
