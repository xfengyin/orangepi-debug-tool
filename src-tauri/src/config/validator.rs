use std::collections::HashSet;

use crate::error::{AppError, AppResult};

use super::schema::*;

#[derive(Debug)]
pub struct ConfigValidator;

impl ConfigValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, config: &AppConfiguration) -> Result<(), ConfigValidationError> {
        self.validate_system_config(&config.system)?;
        self.validate_device_config(&config.devices)?;
        self.validate_security_config(&config.security)?;
        self.validate_observability_config(&config.observability)?;
        Ok(())
    }

    pub fn validate_system_config(&self, config: &SystemConfig) -> Result<(), ConfigValidationError> {
        if config.max_concurrent_tasks == 0 {
            return Err(ConfigValidationError::InvalidValue {
                field: "system.max_concurrent_tasks".to_string(),
                message: "must be greater than 0".to_string(),
            });
        }

        if config.task_timeout_seconds == 0 {
            return Err(ConfigValidationError::InvalidValue {
                field: "system.task_timeout_seconds".to_string(),
                message: "must be greater than 0".to_string(),
            });
        }

        self.validate_retry_policy(&config.retry_policy)?;
        self.validate_circuit_breaker(&config.circuit_breaker)?;

        Ok(())
    }

    pub fn validate_retry_policy(&self, policy: &RetryPolicy) -> Result<(), ConfigValidationError> {
        if policy.max_attempts == 0 {
            return Err(ConfigValidationError::InvalidValue {
                field: "system.retry_policy.max_attempts".to_string(),
                message: "must be greater than 0".to_string(),
            });
        }

        if policy.initial_delay_ms == 0 {
            return Err(ConfigValidationError::InvalidValue {
                field: "system.retry_policy.initial_delay_ms".to_string(),
                message: "must be greater than 0".to_string(),
            });
        }

        if policy.backoff_multiplier < 1.0 {
            return Err(ConfigValidationError::InvalidValue {
                field: "system.retry_policy.backoff_multiplier".to_string(),
                message: "must be at least 1.0".to_string(),
            });
        }

        Ok(())
    }

    pub fn validate_circuit_breaker(&self, config: &CircuitBreakerConfig) -> Result<(), ConfigValidationError> {
        if config.enabled {
            if config.failure_threshold == 0 {
                return Err(ConfigValidationError::InvalidValue {
                    field: "system.circuit_breaker.failure_threshold".to_string(),
                    message: "must be greater than 0 when enabled".to_string(),
                });
            }

            if config.timeout_seconds == 0 {
                return Err(ConfigValidationError::InvalidValue {
                    field: "system.circuit_breaker.timeout_seconds".to_string(),
                    message: "must be greater than 0".to_string(),
                });
            }
        }

        Ok(())
    }

    pub fn validate_device_config(&self, config: &DeviceConfigSection) -> Result<(), ConfigValidationError> {
        self.validate_serial_config(&config.serial)?;
        self.validate_gpio_config(&config.gpio)?;
        self.validate_pwm_config(&config.pwm)?;
        Ok(())
    }

    pub fn validate_serial_config(&self, config: &SerialDeviceConfig) -> Result<(), ConfigValidationError> {
        if config.supported_baudrates.is_empty() {
            return Err(ConfigValidationError::InvalidValue {
                field: "devices.serial.supported_baudrates".to_string(),
                message: "cannot be empty".to_string(),
            });
        }

        if !config.supported_baudrates.contains(&config.auto_detect.default_baudrate) {
            return Err(ConfigValidationError::InvalidValue {
                field: "devices.serial.auto_detect.default_baudrate".to_string(),
                message: "must be in supported_baudrates".to_string(),
            });
        }

        if config.buffer_size == 0 {
            return Err(ConfigValidationError::InvalidValue {
                field: "devices.serial.buffer_size".to_string(),
                message: "must be greater than 0".to_string(),
            });
        }

        let valid_flow_controls: HashSet<&str> = ["none", "software", "hardware"].into_iter().collect();
        for fc in &config.flow_controls {
            if !valid_flow_controls.contains(fc.as_str()) {
                return Err(ConfigValidationError::InvalidValue {
                    field: format!("devices.serial.flow_controls: '{}'", fc),
                    message: "must be one of: none, software, hardware".to_string(),
                });
            }
        }

        Ok(())
    }

    pub fn validate_gpio_config(&self, config: &GpioDeviceConfig) -> Result<(), ConfigValidationError> {
        if let Some(ref pull) = config.default_pull {
            let valid_pulls: HashSet<&str> = ["none", "up", "down"].into_iter().collect();
            if !valid_pulls.contains(pull.as_str()) {
                return Err(ConfigValidationError::InvalidValue {
                    field: "devices.gpio.default_pull".to_string(),
                    message: "must be one of: none, up, down".to_string(),
                });
            }
        }

        for pin in &config.pin_definitions {
            if pin.gpio_number > 255 {
                return Err(ConfigValidationError::InvalidValue {
                    field: format!("devices.gpio.pin_definitions[{}].gpio_number", pin.physical_pin),
                    message: "must be between 0 and 255".to_string(),
                });
            }
        }

        Ok(())
    }

    pub fn validate_pwm_config(&self, config: &PwmDeviceConfig) -> Result<(), ConfigValidationError> {
        if config.default_frequency_hz == 0 {
            return Err(ConfigValidationError::InvalidValue {
                field: "devices.pwm.default_frequency_hz".to_string(),
                message: "must be greater than 0".to_string(),
            });
        }

        if !(0.0..=100.0).contains(&config.default_duty_cycle) {
            return Err(ConfigValidationError::InvalidValue {
                field: "devices.pwm.default_duty_cycle".to_string(),
                message: "must be between 0 and 100".to_string(),
            });
        }

        let mut channels: HashSet<u32> = HashSet::new();
        for channel in &config.channels {
            if !channels.insert(channel.channel) {
                return Err(ConfigValidationError::InvalidValue {
                    field: format!("devices.pwm.channels[{}]", channel.channel),
                    message: "duplicate channel number".to_string(),
                });
            }
        }

        Ok(())
    }

    pub fn validate_security_config(&self, config: &SecurityConfig) -> Result<(), ConfigValidationError> {
        if config.enable_audit_log && config.audit_log_path.is_empty() {
            return Err(ConfigValidationError::InvalidValue {
                field: "security.audit_log_path".to_string(),
                message: "must be specified when audit logging is enabled".to_string(),
            });
        }

        Ok(())
    }

    pub fn validate_observability_config(&self, config: &ObservabilityConfig) -> Result<(), ConfigValidationError> {
        if config.metrics_export_interval_seconds == 0 {
            return Err(ConfigValidationError::InvalidValue {
                field: "observability.metrics_export_interval_seconds".to_string(),
                message: "must be greater than 0".to_string(),
            });
        }

        if config.trace_max_spans == 0 {
            return Err(ConfigValidationError::InvalidValue {
                field: "observability.trace_max_spans".to_string(),
                message: "must be greater than 0".to_string(),
            });
        }

        Ok(())
    }
}

impl Default for ConfigValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum ConfigValidationError {
    InvalidValue { field: String, message: String },
    MissingField { field: String },
    ParseError { field: String, message: String },
}

impl std::fmt::Display for ConfigValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigValidationError::InvalidValue { field, message } => {
                write!(f, "Invalid value for '{}': {}", field, message)
            }
            ConfigValidationError::MissingField { field } => {
                write!(f, "Missing required field: '{}'", field)
            }
            ConfigValidationError::ParseError { field, message } => {
                write!(f, "Failed to parse '{}': {}", field, message)
            }
        }
    }
}

impl std::error::Error for ConfigValidationError {}
