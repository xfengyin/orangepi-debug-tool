#[cfg(test)]
mod tests {
    use crate::security::{
        AuditLogger, AuditEvent, AuditLevel, OperationResult,
        PermissionChecker, Permission, DangerousOperations, DangerLevel,
        DataMasker, SensitiveData
    };

    #[test]
    fn test_audit_event_creation() {
        let event = AuditEvent::new("serial:connect", "serial", OperationResult::Success);
        
        assert_eq!(event.operation, "serial:connect");
        assert_eq!(event.resource_type, "serial");
        assert_eq!(event.result, OperationResult::Success);
        assert!(event.id.len() > 0);
    }

    #[test]
    fn test_audit_event_builder() {
        let event = AuditEvent::new("gpio:write", "gpio", OperationResult::Success)
            .with_user("admin")
            .with_session("session123")
            .with_resource("pin7")
            .with_detail("value", "1")
            .with_level(AuditLevel::Warn);
        
        assert_eq!(event.user_id, Some("admin".to_string()));
        assert_eq!(event.session_id, Some("session123".to_string()));
        assert_eq!(event.resource_id, Some("pin7".to_string()));
        assert_eq!(event.details.get("value"), Some(&"1".to_string()));
        assert_eq!(event.level, AuditLevel::Warn);
    }

    #[test]
    fn test_audit_logger_event_recording() {
        let logger = AuditLogger::new();
        
        logger.log(AuditEvent::new("test", "test", OperationResult::Success));
        logger.log(AuditEvent::new("test", "test", OperationResult::Failure));
        
        let events = logger.get_events(None);
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn test_audit_logger_events_by_operation() {
        let logger = AuditLogger::new();
        
        logger.log(AuditEvent::new("serial:read", "serial", OperationResult::Success));
        logger.log(AuditEvent::new("serial:write", "serial", OperationResult::Success));
        logger.log(AuditEvent::new("serial:read", "serial", OperationResult::Success));
        
        let read_events = logger.get_events_by_operation("serial:read");
        assert_eq!(read_events.len(), 2);
    }

    #[test]
    fn test_audit_logger_failed_events() {
        let logger = AuditLogger::new();
        
        logger.log(AuditEvent::new("test", "test", OperationResult::Success));
        logger.log(AuditEvent::new("test", "test", OperationResult::Failure));
        logger.log(AuditEvent::new("test", "test", OperationResult::Denied));
        
        let failed_events = logger.get_failed_events();
        assert_eq!(failed_events.len(), 2);
    }

    #[test]
    fn test_permission_checker_default_permissions() {
        let checker = PermissionChecker::new();
        
        assert!(checker.check("serial:read", None));
        assert!(checker.check("gpio:write", None));
        assert!(checker.check("pwm:control", None));
    }

    #[test]
    fn test_permission_checker_grant_revoke() {
        let checker = PermissionChecker::new();
        
        checker.grant_permission("user1", Permission::SystemAdmin);
        assert!(checker.check_permission(Permission::SystemAdmin, Some("user1")));
        
        checker.revoke_permission("user1", Permission::SystemAdmin);
        assert!(!checker.check_permission(Permission::SystemAdmin, Some("user1")));
    }

    #[test]
    fn test_permission_checker_user_permissions() {
        let checker = PermissionChecker::new();
        
        checker.grant_permission("user1", Permission::SerialWrite);
        checker.grant_permission("user1", Permission::GpioWrite);
        
        let perms = checker.get_user_permissions("user1");
        assert!(perms.contains(&Permission::SerialWrite));
        assert!(perms.contains(&Permission::GpioWrite));
    }

    #[test]
    fn test_dangerous_operations_detection() {
        assert!(DangerousOperations::is_dangerous("gpio:export_all"));
        assert!(DangerousOperations::is_dangerous("system:factory_reset"));
        assert!(!DangerousOperations::is_dangerous("serial:read"));
    }

    #[test]
    fn test_dangerous_operations_requires_confirmation() {
        assert!(DangerousOperations::requires_confirmation("gpio:export_all"));
        assert!(DangerousOperations::requires_confirmation("system:shutdown"));
        assert!(!DangerousOperations::requires_confirmation("serial:read"));
    }

    #[test]
    fn test_danger_level() {
        assert_eq!(
            DangerousOperations::get_danger_level("system:factory_reset"),
            DangerLevel::Critical
        );
        assert_eq!(
            DangerousOperations::get_danger_level("config:reset"),
            DangerLevel::High
        );
        assert_eq!(
            DangerousOperations::get_danger_level("gpio:write_all"),
            DangerLevel::Medium
        );
    }

    #[test]
    fn test_data_masker_password() {
        let masker = DataMasker::new();
        
        let masked = masker.mask("secret123", "password");
        assert_eq!(masked, "********");
    }

    #[test]
    fn test_data_masker_email() {
        let masker = DataMasker::new();
        
        let masked = masker.mask("test@example.com", "email");
        assert!(masked.contains("***"));
        assert!(masked.contains("@example.com"));
    }

    #[test]
    fn test_data_masker_ip_address() {
        let masker = DataMasker::new();
        
        let masked = masker.mask("192.168.1.100", "ip");
        assert_eq!(masked, "192.*.*.*");
    }

    #[test]
    fn test_data_masker_phone_number() {
        let masker = DataMasker::new();
        
        let masked = masker.mask("13812345678", "phone");
        assert_eq!(masked, "********5678");
    }

    #[test]
    fn test_data_masker_partial() {
        let masked = DataMasker::mask_partial("ABCDEFGH", 2, 2, '*');
        assert_eq!(masked, "AB****GH");
    }

    #[test]
    fn test_data_masker_ip() {
        let masked = DataMasker::mask_ip("192.168.1.100");
        assert_eq!(masked, "192.*.*.*");
    }

    #[test]
    fn test_data_masker_email_static() {
        let masked = DataMasker::mask_email("test@example.com");
        assert_eq!(masked, "t***t@example.com");
    }

    #[test]
    fn test_data_masker_detect() {
        let masker = DataMasker::new();
        
        let detected = masker.detect_sensitive_data("test@example.com");
        assert!(detected.contains(&SensitiveData::Email));
        
        let detected_ip = masker.detect_sensitive_data("192.168.1.1");
        assert!(detected_ip.contains(&SensitiveData::IpAddress));
    }

    #[test]
    fn test_data_masker_token() {
        let masker = DataMasker::new();
        
        let masked = masker.mask("token12345678", "token");
        assert_eq!(masked, "********");
    }

    #[test]
    fn test_audit_event_json() {
        let event = AuditEvent::new("test", "test", OperationResult::Success);
        let json = event.to_json();
        
        assert!(json.contains("test"));
        assert!(json.contains("SUCCESS"));
    }

    #[test]
    fn test_audit_logger_clear() {
        let logger = AuditLogger::new();
        
        logger.log(AuditEvent::new("test", "test", OperationResult::Success));
        assert_eq!(logger.get_events(None).len(), 1);
        
        logger.clear();
        assert_eq!(logger.get_events(None).len(), 0);
    }
}
