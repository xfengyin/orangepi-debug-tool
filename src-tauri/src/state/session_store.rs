use parking_lot::RwLock;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::{debug, info};

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub started_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub user_id: Option<String>,
    pub metadata: HashMap<String, String>,
    pub operation_count: u64,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub id: String,
    pub session_id: String,
    pub command: String,
    pub timestamp: DateTime<Utc>,
    pub duration_ms: u64,
    pub success: bool,
    pub error: Option<String>,
    pub result_preview: Option<String>,
}

pub struct SessionStore {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
    operations: Arc<RwLock<HashMap<String, VecDeque<Operation>>>>,
    current_session: Arc<RwLock<Option<String>>>,
    max_operations_per_session: usize,
    undo_stack: Arc<RwLock<HashMap<String, VecDeque<Operation>>>>,
    redo_stack: Arc<RwLock<HashMap<String, VecDeque<Operation>>>>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            operations: Arc::new(RwLock::new(HashMap::new())),
            current_session: Arc::new(RwLock::new(None)),
            max_operations_per_session: 1000,
            undo_stack: Arc::new(RwLock::new(HashMap::new())),
            redo_stack: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn create_session(&self, user_id: Option<String>) -> String {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let session = Session {
            id: id.clone(),
            started_at: now,
            last_activity: now,
            user_id,
            metadata: HashMap::new(),
            operation_count: 0,
        };
        
        self.sessions.write().insert(id.clone(), session);
        self.operations.write().insert(id.clone(), VecDeque::new());
        self.undo_stack.write().insert(id.clone(), VecDeque::new());
        self.redo_stack.write().insert(id.clone(), VecDeque::new());
        
        *self.current_session.write() = Some(id.clone());
        
        info!("Session created: {}", id);
        id
    }

    pub fn set_current_session(&self, session_id: &str) -> bool {
        if self.sessions.read().contains_key(session_id) {
            *self.current_session.write() = Some(session_id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_session(&self) -> Option<Session> {
        let current = self.current_session.read().clone();
        current.and_then(|id| self.sessions.read().get(&id).cloned())
    }

    pub fn get_session(&self, session_id: &str) -> Option<Session> {
        self.sessions.read().get(session_id).cloned()
    }

    pub fn end_session(&self, session_id: &str) -> bool {
        let mut sessions = self.sessions.write();
        if let Some(session) = sessions.get_mut(session_id) {
            session.last_activity = Utc::now();
            info!("Session ended: {}", session_id);
            return true;
        }
        false
    }

    pub fn record_operation(&self, command: String, duration_ms: u64, success: bool, error: Option<String>, result_preview: Option<String>) -> String {
        let current = self.current_session.read().clone();
        let session_id = match current {
            Some(id) => id,
            None => return String::new(),
        };
        
        let operation_id = Uuid::new_v4().to_string();
        
        let operation = Operation {
            id: operation_id.clone(),
            session_id: session_id.clone(),
            command,
            timestamp: Utc::now(),
            duration_ms,
            success,
            error,
            result_preview,
        };
        
        let mut operations = self.operations.write();
        if let Some(ops) = operations.get_mut(&session_id) {
            if ops.len() >= self.max_operations_per_session {
                ops.pop_front();
            }
            ops.push_back(operation.clone());
        }
        
        let mut sessions = self.sessions.write();
        if let Some(session) = sessions.get_mut(&session_id) {
            session.last_activity = Utc::now();
            session.operation_count += 1;
        }
        
        if success {
            drop(operations);
            drop(sessions);
            let mut undo_stack = self.undo_stack.write();
            if let Some(stack) = undo_stack.get_mut(&session_id) {
                if stack.len() >= self.max_operations_per_session {
                    stack.pop_front();
                }
                stack.push_back(operation);
            }
            let mut redo_stack = self.redo_stack.write();
            redo_stack.get_mut(&session_id).map(|s| s.clear());
        }
        
        debug!("Operation recorded for session {}: {}", session_id, operation_id);
        operation_id
    }

    pub fn get_operations(&self, session_id: &str, limit: Option<usize>) -> Vec<Operation> {
        let operations = self.operations.read();
        let ops = match operations.get(session_id) {
            Some(ops) => ops,
            None => return Vec::new(),
        };
        
        let limit = limit.unwrap_or(ops.len());
        ops.iter().rev().take(limit).cloned().collect()
    }

    pub fn undo(&self) -> Option<Operation> {
        let current = self.current_session.read().clone();
        let session_id = match current {
            Some(id) => id,
            None => return None,
        };
        
        let mut undo_stack = self.undo_stack.write();
        let operation = undo_stack.get_mut(&session_id)?.pop_back()?;
        
        let mut redo_stack = self.redo_stack.write();
        redo_stack.get_mut(&session_id)?.push_back(operation.clone());
        
        info!("Undo operation: {} in session {}", operation.id, session_id);
        Some(operation)
    }

    pub fn redo(&self) -> Option<Operation> {
        let current = self.current_session.read().clone();
        let session_id = match current {
            Some(id) => id,
            None => return None,
        };
        
        let mut redo_stack = self.redo_stack.write();
        let operation = redo_stack.get_mut(&session_id)?.pop_back()?;
        
        let mut undo_stack = self.undo_stack.write();
        undo_stack.get_mut(&session_id)?.push_back(operation.clone());
        
        info!("Redo operation: {} in session {}", operation.id, session_id);
        Some(operation)
    }

    pub fn can_undo(&self) -> bool {
        let current = self.current_session.read().clone();
        let session_id = match current {
            Some(id) => id,
            None => return false,
        };
        
        !self.undo_stack.read().get(&session_id).map(|s| s.is_empty()).unwrap_or(true)
    }

    pub fn can_redo(&self) -> bool {
        let current = self.current_session.read().clone();
        let session_id = match current {
            Some(id) => id,
            None => return false,
        };
        
        !self.redo_stack.read().get(&session_id).map(|s| s.is_empty()).unwrap_or(true)
    }

    pub fn get_all_sessions(&self) -> Vec<Session> {
        self.sessions.read().values().cloned().collect()
    }

    pub fn clear_session(&self, session_id: &str) {
        self.operations.write().remove(session_id);
        self.undo_stack.write().remove(session_id);
        self.redo_stack.write().remove(session_id);
        debug!("Session {} cleared", session_id);
    }

    pub fn get_session_stats(&self, session_id: &str) -> Option<SessionStats> {
        let session = self.sessions.read().get(session_id)?;
        let operations = self.operations.read().get(session_id)?;
        
        let total_duration: u64 = operations.iter().map(|op| op.duration_ms).sum();
        let successful = operations.iter().filter(|op| op.success).count() as u64;
        let failed = operations.len() as u64 - successful;
        
        Some(SessionStats {
            session_id: session_id.to_string(),
            operation_count: operations.len() as u64,
            successful_operations: successful,
            failed_operations: failed,
            total_duration_ms: total_duration,
            average_duration_ms: if operations.is_empty() { 0 } else { total_duration / operations.len() as u64 },
            session_duration_ms: (Utc::now() - session.started_at).num_milliseconds() as u64,
        })
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct SessionStats {
    pub session_id: String,
    pub operation_count: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub total_duration_ms: u64,
    pub average_duration_ms: u64,
    pub session_duration_ms: u64,
}
