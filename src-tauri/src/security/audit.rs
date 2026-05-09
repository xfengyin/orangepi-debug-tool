use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use tracing::{debug, error, info};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}

impl std::fmt::Display for AuditLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditLevel::Trace => write!(f, "TRACE"),
            AuditLevel::Debug => write!(f, "DEBUG"),
            AuditLevel::Info => write!(f, "INFO"),
            AuditLevel::Warn => write!(f, "WARN"),
            AuditLevel::Error => write!(f, "ERROR"),
            AuditLevel::Critical => write!(f, "CRITICAL"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub level: AuditLevel,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub operation: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub details: HashMap<String, String>,
    pub result: OperationResult,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperationResult {
    Success,
    Failure,
    Denied,
    Pending,
}

impl std::fmt::Display for OperationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationResult::Success => write!(f, "SUCCESS"),
            OperationResult::Failure => write!(f, "FAILURE"),
            OperationResult::Denied => write!(f, "DENIED"),
            OperationResult::Pending => write!(f, "PENDING"),
        }
    }
}

impl AuditEvent {
    pub fn new(
        operation: impl Into<String>,
        resource_type: impl Into<String>,
        result: OperationResult,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            level: AuditLevel::Info,
            user_id: None,
            session_id: None,
            operation: operation.into(),
            resource_type: resource_type.into(),
            resource_id: None,
            details: HashMap::new(),
            result,
            ip_address: None,
            user_agent: None,
        }
    }

    pub fn with_user(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn with_session(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    pub fn with_resource(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into());
        self
    }

    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.details.insert(key.into(), value.into());
        self
    }

    pub fn with_level(mut self, level: AuditLevel) -> Self {
        self.level = level;
        self
    }

    pub fn with_ip(mut self, ip: impl Into<String>) -> Self {
        self.ip_address = Some(ip.into());
        self
    }

    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }
}

pub struct AuditLogger {
    log_path: Option<PathBuf>,
    events: Arc<RwLock<Vec<AuditEvent>>>,
    max_events: usize,
    min_level: AuditLevel,
}

impl AuditLogger {
    pub fn new() -> Self {
        Self {
            log_path: None,
            events: Arc::new(RwLock::new(Vec::new())),
            max_events: 10000,
            min_level: AuditLevel::Info,
        }
    }

    pub fn with_path(mut self, path: PathBuf) -> Self {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        self.log_path = Some(path);
        self
    }

    pub fn with_max_events(mut self, max: usize) -> Self {
        self.max_events = max;
        self
    }

    pub fn with_min_level(mut self, level: AuditLevel) -> Self {
        self.min_level = level;
        self
    }

    pub fn log(&self, event: AuditEvent) {
        if !self.should_log(&event.level) {
            return;
        }

        {
            let mut events = self.events.write();
            events.push(event.clone());
            if events.len() > self.max_events {
                events.drain(0..events.len() / 2);
            }
        }

        if let Some(ref path) = self.log_path {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
            {
                let json = event.to_json();
                let line = format!("{}\n", json);
                if let Err(e) = file.write_all(line.as_bytes()) {
                    error!("Failed to write audit log: {}", e);
                }
            }
        }

        info!(
            "Audit: {} {} - {} ({})",
            event.level,
            event.operation,
            event.resource_type,
            event.result
        );
    }

    fn should_log(&self, level: &AuditLevel) -> bool {
        *level as u8 >= self.min_level as u8
    }

    pub fn get_events(&self, limit: Option<usize>) -> Vec<AuditEvent> {
        let events = self.events.read();
        let limit = limit.unwrap_or(events.len());
        events.iter().rev().take(limit).cloned().collect()
    }

    pub fn get_events_by_user(&self, user_id: &str) -> Vec<AuditEvent> {
        self.events.read()
            .iter()
            .filter(|e| e.user_id.as_deref() == Some(user_id))
            .cloned()
            .collect()
    }

    pub fn get_events_by_operation(&self, operation: &str) -> Vec<AuditEvent> {
        self.events.read()
            .iter()
            .filter(|e| e.operation == operation)
            .cloned()
            .collect()
    }

    pub fn get_failed_events(&self) -> Vec<AuditEvent> {
        self.events.read()
            .iter()
            .filter(|e| e.result == OperationResult::Failure || e.result == OperationResult::Denied)
            .cloned()
            .collect()
    }

    pub fn get_events_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<AuditEvent> {
        self.events.read()
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .cloned()
            .collect()
    }

    pub fn clear(&self) {
        self.events.write().clear();
    }

    pub fn export_csv(&self) -> String {
        let mut csv = String::from("ID,Timestamp,Level,User,Session,Operation,Resource,Result\n");
        
        for event in self.events.read().iter() {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                event.id,
                event.timestamp,
                event.level,
                event.user_id.as_deref().unwrap_or("-"),
                event.session_id.as_deref().unwrap_or("-"),
                event.operation,
                event.resource_type,
                event.result
            ));
        }
        
        csv
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! audit_log {
    ($logger:expr, $operation:expr, $resource:expr, $result:expr) => {{
        use $crate::security::AuditEvent;
        let event = AuditEvent::new($operation, $resource, $result);
        $logger.log(event);
    }};
    ($logger:expr, $operation:expr, $resource:expr, $result:expr, $($key:expr => $value:expr),*) => {{
        use $crate::security::AuditEvent;
        let event = AuditEvent::new($operation, $resource, $result)
            $(.with_detail($key, $value))*;
        $logger.log(event);
    }};
}
