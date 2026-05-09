pub mod audit;
pub mod permissions;
pub mod masking;

pub use audit::{AuditLogger, AuditEvent, AuditLevel};
pub use permissions::{PermissionChecker, Permission, DangerousOperations};
pub use masking::{DataMasker, SensitiveData};

use std::sync::Arc;
use parking_lot::RwLock;

pub struct SecurityManager {
    pub audit_logger: Arc<AuditLogger>,
    pub permission_checker: Arc<PermissionChecker>,
    pub data_masker: Arc<DataMasker>,
    pub enable_audit: bool,
    pub enable_permission_check: bool,
    pub enable_masking: bool,
}

impl SecurityManager {
    pub fn new() -> Self {
        Self {
            audit_logger: Arc::new(AuditLogger::new()),
            permission_checker: Arc::new(PermissionChecker::new()),
            data_masker: Arc::new(DataMasker::new()),
            enable_audit: true,
            enable_permission_check: true,
            enable_masking: true,
        }
    }

    pub fn with_audit_disabled(mut self) -> Self {
        self.enable_audit = false;
        self
    }

    pub fn with_permissions_disabled(mut self) -> Self {
        self.enable_permission_check = false;
        self
    }

    pub fn with_masking_disabled(mut self) -> Self {
        self.enable_masking = false;
        self
    }

    pub fn log_operation(&self, event: AuditEvent) {
        if self.enable_audit {
            self.audit_logger.log(event);
        }
    }

    pub fn check_permission(&self, operation: &str, user: Option<&str>) -> bool {
        if self.enable_permission_check {
            self.permission_checker.check(operation, user)
        } else {
            true
        }
    }

    pub fn is_dangerous_operation(&self, operation: &str) -> bool {
        DangerousOperations::is_dangerous(operation)
    }

    pub fn mask_sensitive_data(&self, data: &str, data_type: &str) -> String {
        if self.enable_masking {
            self.data_masker.mask(data, data_type)
        } else {
            data.to_string()
        }
    }
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new()
    }
}
