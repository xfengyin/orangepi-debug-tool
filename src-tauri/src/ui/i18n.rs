//! 国际化模块

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 语言类型
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Chinese,
    English,
}

/// 翻译管理器
pub struct TranslationManager {
    current_language: Language,
    translations: HashMap<Language, HashMap<String, String>>,
}

impl TranslationManager {
    pub fn new() -> Self {
        let mut manager = Self {
            current_language: Language::Chinese,
            translations: HashMap::new(),
        };
        
        manager.load_translations();
        manager
    }
    
    /// 加载翻译
    fn load_translations(&mut self) {
        // 中文
        let mut zh = HashMap::new();
        zh.insert("app.title", "OrangePi 调试工具");
        zh.insert("app.version", "v2.0");
        zh.insert("serial.connect", "连接");
        zh.insert("serial.disconnect", "断开");
        zh.insert("serial.send", "发送");
        zh.insert("serial.clear", "清空");
        zh.insert("serial.hex_mode", "Hex 模式");
        zh.insert("serial.timestamp", "显示时间戳");
        zh.insert("serial.baudrate", "波特率");
        zh.insert("serial.databits", "数据位");
        zh.insert("serial.parity", "校验位");
        zh.insert("serial.stopbits", "停止位");
        zh.insert("serial.flowcontrol", "流控制");
        zh.insert("gpio.control", "GPIO 控制");
        zh.insert("pwm.output", "PWM 输出");
        zh.insert("logger.data", "数据日志");
        zh.insert("protocol.analyzer", "协议解析");
        zh.insert("stats.tx_bytes", "发送字节");
        zh.insert("stats.rx_bytes", "接收字节");
        zh.insert("stats.tx_frames", "发送帧数");
        zh.insert("stats.rx_frames", "接收帧数");
        zh.insert("stats.errors", "错误数");
        zh.insert("theme.light", "浅色");
        zh.insert("theme.dark", "深色");
        zh.insert("theme.system", "跟随系统");
        zh.insert("language.zh", "中文");
        zh.insert("language.en", "English");
        
        self.translations.insert(Language::Chinese, zh);
        
        // English
        let mut en = HashMap::new();
        en.insert("app.title", "OrangePi Debug Tool");
        en.insert("app.version", "v2.0");
        en.insert("serial.connect", "Connect");
        en.insert("serial.disconnect", "Disconnect");
        en.insert("serial.send", "Send");
        en.insert("serial.clear", "Clear");
        en.insert("serial.hex_mode", "Hex Mode");
        en.insert("serial.timestamp", "Show Timestamp");
        en.insert("serial.baudrate", "Baudrate");
        en.insert("serial.databits", "Data Bits");
        en.insert("serial.parity", "Parity");
        en.insert("serial.stopbits", "Stop Bits");
        en.insert("serial.flowcontrol", "Flow Control");
        en.insert("gpio.control", "GPIO Control");
        en.insert("pwm.output", "PWM Output");
        en.insert("logger.data", "Data Logger");
        en.insert("protocol.analyzer", "Protocol Analyzer");
        en.insert("stats.tx_bytes", "TX Bytes");
        en.insert("stats.rx_bytes", "RX Bytes");
        en.insert("stats.tx_frames", "TX Frames");
        en.insert("stats.rx_frames", "RX Frames");
        en.insert("stats.errors", "Errors");
        en.insert("theme.light", "Light");
        en.insert("theme.dark", "Dark");
        en.insert("theme.system", "System");
        en.insert("language.zh", "中文");
        en.insert("language.en", "English");
        
        self.translations.insert(Language::English, en);
    }
    
    /// 设置语言
    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }
    
    /// 获取翻译
    pub fn t(&self, key: &str) -> String {
        self.translations
            .get(&self.current_language)
            .and_then(|map| map.get(key))
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }
    
    /// 获取当前语言
    pub fn get_current_language(&self) -> &Language {
        &self.current_language
    }
    
    /// 获取所有可用语言
    pub fn get_available_languages() -> Vec<Language> {
        vec![Language::Chinese, Language::English]
    }
}

impl Default for TranslationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation() {
        let manager = TranslationManager::new();
        assert_eq!(manager.t("serial.connect"), "连接");
    }
    
    #[test]
    fn test_language_switch() {
        let mut manager = TranslationManager::new();
        manager.set_language(Language::English);
        assert_eq!(manager.t("serial.connect"), "Connect");
    }
}
