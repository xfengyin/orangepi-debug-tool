use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::debug;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum SensitiveData {
    SerialNumber,
    MacAddress,
    IpAddress,
    DeviceId,
    UserCredential,
    ApiKey,
    Password,
    Token,
    Email,
    PhoneNumber,
    CreditCard,
    Custom(String),
}

impl std::fmt::Display for SensitiveData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SensitiveData::SerialNumber => write!(f, "serial_number"),
            SensitiveData::MacAddress => write!(f, "mac_address"),
            SensitiveData::IpAddress => write!(f, "ip_address"),
            SensitiveData::DeviceId => write!(f, "device_id"),
            SensitiveData::UserCredential => write!(f, "user_credential"),
            SensitiveData::ApiKey => write!(f, "api_key"),
            SensitiveData::Password => write!(f, "password"),
            SensitiveData::Token => write!(f, "token"),
            SensitiveData::Email => write!(f, "email"),
            SensitiveData::PhoneNumber => write!(f, "phone_number"),
            SensitiveData::CreditCard => write!(f, "credit_card"),
            SensitiveData::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}

pub struct DataMasker {
    patterns: Arc<RwLock<HashMap<SensitiveData, Regex>>>,
    custom_masking_rules: Arc<RwLock<HashMap<String, MaskingRule>>>,
}

#[derive(Debug, Clone)]
pub struct MaskingRule {
    pub pattern: String,
    pub replacement: String,
    pub preserve_length: bool,
}

impl DataMasker {
    pub fn new() -> Self {
        let patterns = Self::default_patterns();
        Self {
            patterns: Arc::new(RwLock::new(patterns)),
            custom_masking_rules: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn default_patterns() -> HashMap<SensitiveData, Regex> {
        let mut patterns = HashMap::new();
        
        patterns.insert(
            SensitiveData::SerialNumber,
            Regex::new(r"^[A-Z0-9]{8,16}$").unwrap(),
        );
        
        patterns.insert(
            SensitiveData::MacAddress,
            Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap(),
        );
        
        patterns.insert(
            SensitiveData::IpAddress,
            Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap(),
        );
        
        patterns.insert(
            SensitiveData::Email,
            Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap(),
        );
        
        patterns.insert(
            SensitiveData::PhoneNumber,
            Regex::new(r"^1[3-9]\d{9}$").unwrap(),
        );
        
        patterns.insert(
            SensitiveData::CreditCard,
            Regex::new(r"^\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}$").unwrap(),
        );
        
        patterns.insert(
            SensitiveData::ApiKey,
            Regex::new(r"^[A-Za-z0-9_\-]{32,64}$").unwrap(),
        );
        
        patterns
    }

    pub fn mask(&self, data: &str, data_type: &str) -> String {
        let sensitive_type = self.parse_data_type(data_type);
        
        match sensitive_type {
            Some(stype) => self.mask_with_type(data, &stype),
            None => self.mask_custom(data),
        }
    }

    fn parse_data_type(&self, data_type: &str) -> Option<SensitiveData> {
        match data_type.to_lowercase().as_str() {
            "serial_number" | "serial" | "serialnumber" => Some(SensitiveData::SerialNumber),
            "mac_address" | "mac" | "macaddress" => Some(SensitiveData::MacAddress),
            "ip_address" | "ip" | "ipaddress" => Some(SensitiveData::IpAddress),
            "device_id" | "deviceid" => Some(SensitiveData::DeviceId),
            "api_key" | "apikey" => Some(SensitiveData::ApiKey),
            "password" | "pwd" => Some(SensitiveData::Password),
            "token" => Some(SensitiveData::Token),
            "email" => Some(SensitiveData::Email),
            "phone_number" | "phone" | "phonenumber" => Some(SensitiveData::PhoneNumber),
            "credit_card" | "card" | "creditcard" => Some(SensitiveData::CreditCard),
            _ => None,
        }
    }

    fn mask_with_type(&self, data: &str, data_type: &SensitiveData) -> String {
        match data_type {
            SensitiveData::Password | SensitiveData::Token => {
                "*".repeat(data.len().min(8))
            }
            SensitiveData::SerialNumber => {
                Self::mask_partial(data, 4, 4, '*')
            }
            SensitiveData::MacAddress => {
                Self::mask_partial(data, 0, 12, '*')
            }
            SensitiveData::IpAddress => {
                Self::mask_ip(data)
            }
            SensitiveData::Email => {
                Self::mask_email(data)
            }
            SensitiveData::PhoneNumber => {
                Self::mask_partial(data, 0, 7, '*')
            }
            SensitiveData::CreditCard => {
                Self::mask_partial(data, 12, 16, '*')
            }
            SensitiveData::ApiKey => {
                Self::mask_partial(data, 0, 8, '*')
            }
            SensitiveData::Custom(name) => {
                self.mask_custom_named(data, name)
            }
            _ => data.to_string(),
        }
    }

    pub fn mask_partial(data: &str, preserve_start: usize, preserve_end: usize, mask_char: char) -> String {
        if data.len() <= preserve_start + preserve_end {
            return mask_char.to_string().repeat(data.len());
        }
        
        let start = &data[..preserve_start.min(data.len())];
        let end_start = data.len().saturating_sub(preserve_end);
        let end = if end_start > preserve_start {
            &data[end_start..]
        } else {
            ""
        };
        let middle_len = data.len() - start.len() - end.len();
        let middle = mask_char.to_string().repeat(middle_len);
        
        format!("{}{}{}", start, middle, end)
    }

    pub fn mask_ip(ip: &str) -> String {
        let parts: Vec<&str> = ip.split('.').collect();
        if parts.len() == 4 {
            format!("{}.*.*.*", parts[0])
        } else {
            ip.to_string()
        }
    }

    pub fn mask_email(email: &str) -> String {
        if let Some(at_idx) = email.find('@') {
            let local = &email[..at_idx];
            let domain = &email[at_idx..];
            
            let masked_local = if local.len() <= 2 {
                "*".repeat(local.len())
            } else {
                format!("{}***{}", &local[0..1], &local[local.len()-1..])
            };
            
            format!("{}{}", masked_local, domain)
        } else {
            email.to_string()
        }
    }

    pub fn mask_custom(&self, data: &str) -> String {
        if data.len() <= 4 {
            return "*".repeat(data.len());
        }
        Self::mask_partial(data, 2, 2, '*')
    }

    pub fn mask_custom_named(&self, data: &str, name: &str) -> String {
        let rules = self.custom_masking_rules.read();
        if let Some(rule) = rules.get(name) {
            let re = Regex::new(&rule.pattern);
            if let Ok(re) = re {
                if rule.preserve_length {
                    let len = data.len();
                    let mask = rule.replacement.repeat(len);
                    return re.replace(data, mask.as_str()).to_string();
                } else {
                    return re.replace(data, rule.replacement.as_str()).to_string();
                }
            }
        }
        self.mask_custom(data)
    }

    pub fn register_rule(&self, name: &str, rule: MaskingRule) {
        self.custom_masking_rules.write().insert(name.to_string(), rule);
        debug!("Registered custom masking rule: {}", name);
    }

    pub fn remove_rule(&self, name: &str) -> bool {
        self.custom_masking_rules.write().remove(name).is_some()
    }

    pub fn detect_sensitive_data(&self, data: &str) -> Vec<SensitiveData> {
        let patterns = self.patterns.read();
        let mut detected = Vec::new();
        
        for (data_type, pattern) in patterns.iter() {
            if pattern.is_match(data) {
                detected.push(data_type.clone());
            }
        }
        
        detected
    }
}

impl Default for DataMasker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_password() {
        let masker = DataMasker::new();
        let result = masker.mask("secret123", "password");
        assert_eq!(result, "********");
    }

    #[test]
    fn test_mask_partial() {
        let result = DataMasker::mask_partial("ABCDEFGH", 2, 2, '*');
        assert_eq!(result, "AB****GH");
    }

    #[test]
    fn test_mask_ip() {
        let result = DataMasker::mask_ip("192.168.1.100");
        assert_eq!(result, "192.*.*.*");
    }

    #[test]
    fn test_mask_email() {
        let result = DataMasker::mask_email("test@example.com");
        assert_eq!(result, "t***t@example.com");
    }
}
