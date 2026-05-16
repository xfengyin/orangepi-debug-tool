use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

impl HealthStatus {
    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)
    }
}

#[derive(Debug, Clone)]
pub struct HealthReport {
    pub status: HealthStatus,
    pub checks: Vec<HealthCheckResult>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl HealthReport {
    pub fn new() -> Self {
        Self {
            status: HealthStatus::Healthy,
            checks: Vec::new(),
            timestamp: chrono::Utc::now(),
        }
    }
    
    pub fn with_check(mut self, result: HealthCheckResult) -> Self {
        if !result.is_healthy() && self.status == HealthStatus::Healthy {
            self.status = HealthStatus::Degraded;
        }
        self.checks.push(result);
        self
    }
}

impl Default for HealthReport {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    pub name: String,
    pub status: HealthStatus,
    pub message: Option<String>,
    pub duration_ms: u64,
}

impl HealthCheckResult {
    pub fn healthy(name: &str, duration_ms: u64) -> Self {
        Self {
            name: name.to_string(),
            status: HealthStatus::Healthy,
            message: None,
            duration_ms,
        }
    }
    
    pub fn unhealthy(name: &str, message: &str, duration_ms: u64) -> Self {
        Self {
            name: name.to_string(),
            status: HealthStatus::Unhealthy,
            message: Some(message.to_string()),
            duration_ms,
        }
    }
    
    pub fn degraded(name: &str, message: &str, duration_ms: u64) -> Self {
        Self {
            name: name.to_string(),
            status: HealthStatus::Degraded,
            message: Some(message.to_string()),
            duration_ms,
        }
    }
    
    pub fn is_healthy(&self) -> bool {
        self.status == HealthStatus::Healthy
    }
}

#[async_trait::async_trait]
pub trait HealthCheck: Send + Sync {
    fn name(&self) -> &str;
    async fn check(&self) -> HealthCheckResult;
}

pub struct HealthChecker {
    checks: RwLock<HashMap<String, Arc<dyn HealthCheck>>>,
}

impl HealthChecker {
    pub fn new() -> Self {
        Self {
            checks: RwLock::new(HashMap::new()),
        }
    }
    
    pub fn register<C: HealthCheck + 'static>(&self, check: C) {
        self.checks.write().insert(check.name().to_string(), Arc::new(check));
    }
    
    pub async fn check_all(&self) -> HealthReport {
        let mut report = HealthReport::new();
        
        let checks = self.checks.read().clone();
        for (_, check) in checks {
            let start = std::time::Instant::now();
            let result = check.check().await;
            let duration = start.elapsed().as_millis() as u64;
            
            let result = HealthCheckResult {
                name: result.name,
                status: result.status,
                message: result.message,
                duration_ms: duration,
            };
            
            report = report.with_check(result);
        }
        
        report
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeResponse {
    pub status: HealthStatus,
    pub message: String,
}

impl From<HealthReport> for ProbeResponse {
    fn from(report: HealthReport) -> Self {
        match report.status {
            HealthStatus::Healthy => Self {
                status: report.status,
                message: "All checks passed".to_string(),
            },
            HealthStatus::Degraded => Self {
                status: report.status,
                message: format!("{} checks degraded", report.checks.iter().filter(|c| c.status == HealthStatus::Degraded).count()),
            },
            HealthStatus::Unhealthy => Self {
                status: report.status,
                message: format!("{} checks failed", report.checks.iter().filter(|c| c.status == HealthStatus::Unhealthy).count()),
            },
        }
    }
}
