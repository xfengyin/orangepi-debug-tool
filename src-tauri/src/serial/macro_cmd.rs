//! 宏命令/脚本模块

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 宏命令类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MacroType {
    Sequence,      // 顺序执行
    Condition,     // 条件执行
    Loop,          // 循环执行
    Script,        // 脚本执行
}

/// 宏命令
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MacroCommand {
    pub id: String,
    pub name: String,
    pub macro_type: MacroType,
    pub steps: Vec<MacroStep>,
    pub enabled: bool,
    pub trigger: Option<MacroTrigger>,
}

/// 宏步骤
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MacroStep {
    pub id: String,
    pub action: String,
    pub data: String,
    pub delay_ms: u64,
    pub hex_mode: bool,
    pub condition: Option<String>,
}

/// 触发器
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MacroTrigger {
    Manual,
    AutoSend { interval_ms: u64 },
    OnReceive { pattern: String },
    OnConnect,
    OnDisconnect,
}

/// 脚本引擎
#[derive(Debug, Clone)]
pub struct ScriptEngine {
    variables: HashMap<String, String>,
}

impl ScriptEngine {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
    
    /// 执行脚本
    pub fn execute_script(&mut self, script: &str) -> Result<String, String> {
        // 简单的脚本解析器
        // 支持基本命令：SET, PRINT, IF, SEND, WAIT
        
        let lines = script.lines();
        let mut output = String::new();
        
        for line in lines {
            let line = line.trim();
            
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            
            if let Some(result) = self.execute_line(line)? {
                output.push_str(&result);
                output.push('\n');
            }
        }
        
        Ok(output)
    }
    
    fn execute_line(&mut self, line: &str) -> Result<Option<String>, String> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.is_empty() {
            return Ok(None);
        }
        
        match parts[0] {
            "SET" => {
                if parts.len() >= 3 {
                    self.variables.insert(parts[1].to_string(), parts[2..].join(" "));
                }
                Ok(None)
            },
            "PRINT" => {
                let text = parts[1..].join(" ");
                Ok(Some(self.expand_variables(&text)))
            },
            "SEND" => {
                // TODO: 实际发送数据
                Ok(Some(format!("SEND: {}", parts[1..].join(" "))))
            },
            "WAIT" => {
                if parts.len() >= 2 {
                    let ms: u64 = parts[1].parse().unwrap_or(0);
                    // TODO: 实际等待
                    Ok(Some(format!("WAIT: {}ms", ms)))
                } else {
                    Err("WAIT 命令需要时间参数".to_string())
                }
            },
            "IF" => {
                // 简单条件判断
                Ok(None)
            },
            _ => Err(format!("未知命令：{}", parts[0])),
        }
    }
    
    fn expand_variables(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (key, value) in &self.variables {
            result = result.replace(&format!("${{{}}}", key), value);
        }
        result
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 预定义脚本模板
pub fn get_script_templates() -> HashMap<&'static str, &'static str> {
    let mut templates = HashMap::new();
    
    templates.insert(
        "hello",
        r#"// 问候脚本
SET device "OrangePi"
PRINT Hello, ${device}!
SEND 01 03 00 00 00 0A
WAIT 100
PRINT Command sent
"#,
    );
    
    templates.insert(
        "modbus_read",
        r#"// MODBUS 读取保持寄存器
// 从站地址：1
// 功能码：03
// 起始地址：0
// 寄存器数量：10

SET slave 1
SET function 03
SET address 0
SET count 10

PRINT Reading ${count} registers from slave ${slave}...
SEND ${slave} ${function} ${address} ${count}
WAIT 100
PRINT Done
"#,
    );
    
    templates.insert(
        "auto_response",
        r#"// 自动回复脚本
// 当收到特定数据时自动回复

SET trigger "HELLO"
SET response "WORLD"

PRINT Auto-response script loaded
PRINT Trigger: ${trigger}
PRINT Response: ${response}
"#,
    );
    
    templates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_execution() {
        let mut engine = ScriptEngine::new();
        let script = r#"
SET name Test
PRINT Hello ${name}
"#;
        let result = engine.execute_script(script).unwrap();
        assert!(result.contains("Hello Test"));
    }
    
    #[test]
    fn test_variable_expansion() {
        let mut engine = ScriptEngine::new();
        engine.variables.insert("var".to_string(), "value".to_string());
        let expanded = engine.expand_variables("Value is ${var}");
        assert_eq!(expanded, "Value is value");
    }
}
