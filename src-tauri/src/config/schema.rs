use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfiguration {
    pub meta: ConfigMetadata,
    pub system: SystemConfig,
    pub devices: DeviceConfigSection,
    pub plugins: PluginConfigSection,
    pub skills: SkillConfigSection,
    pub prompts: PromptConfigSection,
    pub security: SecurityConfig,
    pub observability: ObservabilityConfig,
}

impl Default for AppConfiguration {
    fn default() -> Self {
        Self {
            meta: ConfigMetadata::default(),
            system: SystemConfig::default(),
            devices: DeviceConfigSection::default(),
            plugins: PluginConfigSection::default(),
            skills: SkillConfigSection::default(),
            prompts: PromptConfigSection::default(),
            security: SecurityConfig::default(),
            observability: ObservabilityConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMetadata {
    pub version: String,
    pub schema_version: String,
    pub last_modified: DateTime<Utc>,
    pub environment: Environment,
}

impl Default for ConfigMetadata {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            schema_version: "1.0.0".to_string(),
            last_modified: Utc::now(),
            environment: Environment::Production,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Environment {
    Development,
    Testing,
    Staging,
    Production,
}

impl Default for Environment {
    fn default() -> Self {
        Self::Production
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub log_level: LogLevel,
    pub max_concurrent_tasks: usize,
    pub task_timeout_seconds: u64,
    pub retry_policy: RetryPolicy,
    pub circuit_breaker: CircuitBreakerConfig,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            log_level: LogLevel::Info,
            max_concurrent_tasks: 100,
            task_timeout_seconds: 300,
            retry_policy: RetryPolicy::default(),
            circuit_breaker: CircuitBreakerConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    pub enabled: bool,
    pub failure_threshold: u32,
    pub timeout_seconds: u64,
    pub half_open_max_calls: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            failure_threshold: 5,
            timeout_seconds: 60,
            half_open_max_calls: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfigSection {
    pub serial: SerialDeviceConfig,
    pub gpio: GpioDeviceConfig,
    pub pwm: PwmDeviceConfig,
}

impl Default for DeviceConfigSection {
    fn default() -> Self {
        Self {
            serial: SerialDeviceConfig::default(),
            gpio: GpioDeviceConfig::default(),
            pwm: PwmDeviceConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialDeviceConfig {
    pub default_adapter: String,
    pub auto_detect: AutoDetectConfig,
    pub supported_baudrates: Vec<u32>,
    pub buffer_size: usize,
    pub read_timeout_ms: u64,
    pub write_timeout_ms: u64,
    pub flow_controls: Vec<String>,
}

impl Default for SerialDeviceConfig {
    fn default() -> Self {
        Self {
            default_adapter: "generic_linux".to_string(),
            auto_detect: AutoDetectConfig::default(),
            supported_baudrates: vec![300, 1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200, 230400, 460800, 921600],
            buffer_size: 65536,
            read_timeout_ms: 1000,
            write_timeout_ms: 1000,
            flow_controls: vec!["none".to_string(), "software".to_string(), "hardware".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoDetectConfig {
    pub enabled: bool,
    pub default_baudrate: u32,
    pub scan_interval_ms: u64,
}

impl Default for AutoDetectConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_baudrate: 115200,
            scan_interval_ms: 5000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpioDeviceConfig {
    pub default_adapter: String,
    pub pin_definitions: Vec<PinDefinitionConfig>,
    pub default_pull: String,
    pub interrupt_debounce_ms: u64,
    pub batch_operation_timeout_ms: u64,
}

impl Default for GpioDeviceConfig {
    fn default() -> Self {
        Self {
            default_adapter: "orangepi_zero3".to_string(),
            pin_definitions: Vec::new(),
            default_pull: "none".to_string(),
            interrupt_debounce_ms: 50,
            batch_operation_timeout_ms: 5000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinDefinitionConfig {
    pub physical_pin: u32,
    pub gpio_number: u32,
    pub name: String,
    pub modes: Vec<String>,
    pub default_mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwmDeviceConfig {
    pub default_adapter: String,
    pub channels: Vec<PwmChannelConfig>,
    pub default_frequency_hz: u32,
    pub default_duty_cycle: f64,
}

impl Default for PwmDeviceConfig {
    fn default() -> Self {
        Self {
            default_adapter: "orangepi_zero3".to_string(),
            channels: vec![
                PwmChannelConfig { channel: 0, name: "PWM0".to_string(), enabled: false },
                PwmChannelConfig { channel: 1, name: "PWM1".to_string(), enabled: false },
            ],
            default_frequency_hz: 1000,
            default_duty_cycle: 50.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwmChannelConfig {
    pub channel: u32,
    pub name: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfigSection {
    pub enabled_plugins: Vec<String>,
    pub plugin_paths: Vec<String>,
    pub auto_load: bool,
}

impl Default for PluginConfigSection {
    fn default() -> Self {
        Self {
            enabled_plugins: Vec::new(),
            plugin_paths: Vec::new(),
            auto_load: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillConfigSection {
    pub enabled_skills: Vec<String>,
    pub skill_timeout_seconds: u64,
    pub skill_retry_count: u32,
    pub skill_chain: Vec<SkillChainDefinition>,
}

impl Default for SkillConfigSection {
    fn default() -> Self {
        Self {
            enabled_skills: Vec::new(),
            skill_timeout_seconds: 60,
            skill_retry_count: 3,
            skill_chain: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillChainDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<SkillStep>,
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillStep {
    pub skill: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub continue_on_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptConfigSection {
    pub templates: HashMap<String, PromptTemplate>,
    pub default_model: String,
    pub fallback_models: Vec<String>,
}

impl Default for PromptConfigSection {
    fn default() -> Self {
        Self {
            templates: HashMap::new(),
            default_model: "gpt-4".to_string(),
            fallback_models: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub id: String,
    pub name: String,
    pub system_prompt: String,
    pub user_prompt_template: String,
    pub parameters: Vec<PromptParameter>,
    pub max_tokens: usize,
    pub temperature: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptParameter {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub enable_audit_log: bool,
    pub audit_log_path: String,
    pub dangerous_operations_require_confirmation: bool,
    pub sensitive_data_masking: bool,
    pub permission_checks_enabled: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_audit_log: true,
            audit_log_path: "logs/audit.log".to_string(),
            dangerous_operations_require_confirmation: true,
            sensitive_data_masking: true,
            permission_checks_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    pub enable_metrics: bool,
    pub enable_tracing: bool,
    pub enable_health_check: bool,
    pub metrics_export_interval_seconds: u64,
    pub trace_max_spans: usize,
    pub health_check_interval_seconds: u64,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            enable_metrics: true,
            enable_tracing: true,
            enable_health_check: true,
            metrics_export_interval_seconds: 60,
            trace_max_spans: 10000,
            health_check_interval_seconds: 30,
        }
    }
}
