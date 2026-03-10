//! 数据导出模块

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
use chrono::Local;

/// 导出格式
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ExportFormat {
    CSV,
    JSON,
    TXT,
    Binary,
}

/// 导出数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportData {
    pub timestamp: String,
    pub direction: String, // "TX" or "RX"
    pub data: String,
    pub hex_mode: bool,
}

/// 导出配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportConfig {
    pub format: ExportFormat,
    pub include_timestamp: bool,
    pub include_direction: bool,
    pub filter_tx: bool,
    pub filter_rx: bool,
    pub date_range: Option<(String, String)>,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            format: ExportFormat::CSV,
            include_timestamp: true,
            include_direction: true,
            filter_tx: true,
            filter_rx: true,
            date_range: None,
        }
    }
}

/// 导出到 CSV
pub fn export_to_csv(data: &[ExportData], config: &ExportConfig, file_path: &str) -> Result<String, String> {
    let file = File::create(file_path)
        .map_err(|e| format!("创建文件失败：{}", e))?;
    let mut writer = BufWriter::new(file);
    
    // 写入表头
    let mut header = String::new();
    if config.include_timestamp {
        header.push_str("Timestamp,");
    }
    if config.include_direction {
        header.push_str("Direction,");
    }
    header.push_str("Data\n");
    
    writer.write_all(header.as_bytes())
        .map_err(|e| format!("写入文件失败：{}", e))?;
    
    // 写入数据
    for record in data {
        // 应用过滤器
        if !config.filter_tx && record.direction == "TX" {
            continue;
        }
        if !config.filter_rx && record.direction == "RX" {
            continue;
        }
        
        let mut line = String::new();
        
        if config.include_timestamp {
            line.push_str(&format!("{},", escape_csv(&record.timestamp)));
        }
        if config.include_direction {
            line.push_str(&format!("{},", record.direction));
        }
        
        // 处理数据中的特殊字符
        let escaped_data = escape_csv(&record.data);
        line.push_str(&escaped_data);
        line.push('\n');
        
        writer.write_all(line.as_bytes())
            .map_err(|e| format!("写入文件失败：{}", e))?;
    }
    
    writer.flush()
        .map_err(|e| format!("刷新文件失败：{}", e))?;
    
    Ok(format!("成功导出 {} 条记录到 {}", data.len(), file_path))
}

/// 导出到 JSON
pub fn export_to_json(data: &[ExportData], config: &ExportConfig, file_path: &str) -> Result<String, String> {
    let file = File::create(file_path)
        .map_err(|e| format!("创建文件失败：{}", e))?;
    let mut writer = BufWriter::new(file);
    
    // 应用过滤器
    let filtered_data: Vec<&ExportData> = data.iter()
        .filter(|record| {
            if !config.filter_tx && record.direction == "TX" {
                return false;
            }
            if !config.filter_rx && record.direction == "RX" {
                return false;
            }
            true
        })
        .collect();
    
    let json = serde_json::to_string_pretty(&filtered_data)
        .map_err(|e| format!("JSON 序列化失败：{}", e))?;
    
    writer.write_all(json.as_bytes())
        .map_err(|e| format!("写入文件失败：{}", e))?;
    
    writer.flush()
        .map_err(|e| format!("刷新文件失败：{}", e))?;
    
    Ok(format!("成功导出 {} 条记录到 {}", filtered_data.len(), file_path))
}

/// 导出到 TXT
pub fn export_to_txt(data: &[ExportData], config: &ExportConfig, file_path: &str) -> Result<String, String> {
    let file = File::create(file_path)
        .map_err(|e| format!("创建文件失败：{}", e))?;
    let mut writer = BufWriter::new(file);
    
    for record in data {
        // 应用过滤器
        if !config.filter_tx && record.direction == "TX" {
            continue;
        }
        if !config.filter_rx && record.direction == "RX" {
            continue;
        }
        
        let mut line = String::new();
        
        if config.include_timestamp {
            line.push_str(&format!("[{}] ", record.timestamp));
        }
        line.push_str(&format!("{}: ", record.direction));
        line.push_str(&record.data);
        line.push('\n');
        
        writer.write_all(line.as_bytes())
            .map_err(|e| format!("写入文件失败：{}", e))?;
    }
    
    writer.flush()
        .map_err(|e| format!("刷新文件失败：{}", e))?;
    
    Ok(format!("成功导出 {} 条记录到 {}", data.len(), file_path))
}

/// CSV 转义
fn escape_csv(text: &str) -> String {
    if text.contains(',') || text.contains('"') || text.contains('\n') {
        format!("\"{}\"", text.replace('"', "\"\""))
    } else {
        text.to_string()
    }
}

/// 生成导出文件名
pub fn generate_filename(format: &ExportFormat) -> String {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    match format {
        ExportFormat::CSV => format!("serial_data_{}.csv", timestamp),
        ExportFormat::JSON => format!("serial_data_{}.json", timestamp),
        ExportFormat::TXT => format!("serial_data_{}.txt", timestamp),
        ExportFormat::Binary => format!("serial_data_{}.bin", timestamp),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_escape() {
        assert_eq!(escape_csv("hello"), "hello");
        assert_eq!(escape_csv("hello,world"), "\"hello,world\"");
        assert_eq!(escape_csv("hello\"world"), "\"hello\"\"world\"");
    }
}
