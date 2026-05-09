use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    SerialRead,
    SerialWrite,
    SerialConnect,
    GpioRead,
    GpioWrite,
    GpioExport,
    PwmControl,
    ConfigRead,
    ConfigWrite,
    SystemAdmin,
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Permission::SerialRead => write!(f, "serial:read"),
            Permission::SerialWrite => write!(f, "serial:write"),
            Permission::SerialConnect => write!(f, "serial:connect"),
            Permission::GpioRead => write!(f, "gpio:read"),
            Permission::GpioWrite => write!(f, "gpio:write"),
            Permission::GpioExport => write!(f, "gpio:export"),
            Permission::PwmControl => write!(f, "pwm:control"),
            Permission::ConfigRead => write!(f, "config:read"),
            Permission::ConfigWrite => write!(f, "config:write"),
            Permission::SystemAdmin => write!(f, "system:admin"),
        }
    }
}

pub struct PermissionChecker {
    user_permissions: Arc<RwLock<HashMap<String, Vec<Permission>>>>,
    default_permissions: Vec<Permission>,
}

impl PermissionChecker {
    pub fn new() -> Self {
        Self {
            user_permissions: Arc::new(RwLock::new(HashMap::new())),
            default_permissions: vec![
                Permission::SerialRead,
                Permission::SerialWrite,
                Permission::SerialConnect,
                Permission::GpioRead,
                Permission::GpioWrite,
                Permission::GpioExport,
                Permission::PwmControl,
                Permission::ConfigRead,
            ],
        }
    }

    pub fn grant_permission(&self, user_id: &str, permission: Permission) {
        let mut perms = self.user_permissions.write();
        perms.entry(user_id.to_string())
            .or_insert_with(Vec::new)
            .push(permission);
        debug!("Granted {:?} to user {}", permission, user_id);
    }

    pub fn revoke_permission(&self, user_id: &str, permission: Permission) {
        let mut perms = self.user_permissions.write();
        if let Some(user_perms) = perms.get_mut(user_id) {
            user_perms.retain(|&p| p != permission);
        }
        debug!("Revoked {:?} from user {}", permission, user_id);
    }

    pub fn check(&self, operation: &str, user: Option<&str>) -> bool {
        let required_perm = self.operation_to_permission(operation);
        
        let user_id = user.unwrap_or("anonymous");
        let perms = self.user_permissions.read();
        
        if let Some(user_perms) = perms.get(user_id) {
            user_perms.contains(&required_perm)
        } else {
            self.default_permissions.contains(&required_perm)
        }
    }

    pub fn check_permission(&self, permission: Permission, user: Option<&str>) -> bool {
        let user_id = user.unwrap_or("anonymous");
        let perms = self.user_permissions.read();
        
        if let Some(user_perms) = perms.get(user_id) {
            user_perms.contains(&permission) || user_perms.contains(&Permission::SystemAdmin)
        } else {
            self.default_permissions.contains(&permission) || permission == Permission::SystemAdmin
        }
    }

    pub fn get_user_permissions(&self, user_id: &str) -> Vec<Permission> {
        let perms = self.user_permissions.read();
        perms.get(user_id)
            .cloned()
            .unwrap_or_else(|| self.default_permissions.clone())
    }

    fn operation_to_permission(&self, operation: &str) -> Permission {
        match operation {
            "serial:read" => Permission::SerialRead,
            "serial:write" => Permission::SerialWrite,
            "serial:connect" => Permission::SerialConnect,
            "gpio:read" => Permission::GpioRead,
            "gpio:write" => Permission::GpioWrite,
            "gpio:export" => Permission::GpioExport,
            "pwm:control" => Permission::PwmControl,
            "config:read" => Permission::ConfigRead,
            "config:write" => Permission::ConfigWrite,
            "system:admin" => Permission::SystemAdmin,
            _ => Permission::SystemAdmin,
        }
    }
}

impl Default for PermissionChecker {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DangerousOperations;

impl DangerousOperations {
    pub fn is_dangerous(operation: &str) -> bool {
        matches!(
            operation,
            "gpio:export_all" |
            "gpio:write_all" |
            "config:reset" |
            "config:delete" |
            "system:factory_reset" |
            "system:shutdown" |
            "system:restart"
        )
    }

    pub fn requires_confirmation(operation: &str) -> bool {
        matches!(
            operation,
            "gpio:export_all" |
            "gpio:write_all" |
            "gpio:unexport_all" |
            "config:reset" |
            "config:delete" |
            "system:factory_reset" |
            "system:shutdown" |
            "system:restart"
        )
    }

    pub fn get_danger_level(operation: &str) -> DangerLevel {
        match operation {
            "system:factory_reset" | "system:shutdown" => DangerLevel::Critical,
            "config:reset" | "config:delete" | "gpio:export_all" => DangerLevel::High,
            "gpio:write_all" | "gpio:unexport_all" | "system:restart" => DangerLevel::Medium,
            _ => DangerLevel::Low,
        }
    }

    pub fn all_dangerous_operations() -> Vec<&'static str> {
        vec![
            "gpio:export_all",
            "gpio:write_all",
            "gpio:unexport_all",
            "config:reset",
            "config:delete",
            "system:factory_reset",
            "system:shutdown",
            "system:restart",
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DangerLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for DangerLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DangerLevel::Low => write!(f, "低风险"),
            DangerLevel::Medium => write!(f, "中等风险"),
            DangerLevel::High => write!(f, "高风险"),
            DangerLevel::Critical => write!(f, "极高风险"),
        }
    }
}
