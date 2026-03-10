//! 数据包解析模块

use serde::{Deserialize, Serialize};

/// 数据帧定义
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataFrame {
    pub header: Vec<u8>,      // 帧头
    pub length: u16,           // 数据长度
    pub data: Vec<u8>,         // 数据
    pub checksum: u8,          // 校验和
    pub footer: Vec<u8>,       // 帧尾
}

/// 解析模式
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ParseMode {
    None,           // 无解析 (原始模式)
    HexDisplay,     // 十六进制显示
    TextDisplay,    // 文本显示
    CustomFrame,    // 自定义帧解析
}

/// 校验类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChecksumType {
    None,
    Sum,
    CRC8,
    CRC16,
    CRC32,
}

/// 解析配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParseConfig {
    pub mode: ParseMode,
    pub checksum_type: ChecksumType,
    pub frame_header: Vec<u8>,
    pub frame_footer: Vec<u8>,
    pub show_timestamp: bool,
    pub auto_scroll: bool,
    pub max_lines: usize,
}

impl Default for ParseConfig {
    fn default() -> Self {
        Self {
            mode: ParseMode::TextDisplay,
            checksum_type: ChecksumType::None,
            frame_header: vec![],
            frame_footer: vec![],
            show_timestamp: true,
            auto_scroll: true,
            max_lines: 1000,
        }
    }
}

/// 计算校验和
pub fn calculate_checksum(data: &[u8], checksum_type: &ChecksumType) -> u32 {
    match checksum_type {
        ChecksumType::None => 0,
        ChecksumType::Sum => data.iter().map(|&b| b as u32).sum(),
        ChecksumType::CRC8 => crc8(data),
        ChecksumType::CRC16 => crc16(data),
        ChecksumType::CRC32 => crc32(data),
    }
}

/// CRC8 计算
fn crc8(data: &[u8]) -> u32 {
    let mut crc = 0u8;
    for &byte in data {
        crc ^= byte;
        for _ in 0..8 {
            if crc & 0x80 != 0 {
                crc = (crc << 1) ^ 0x07;
            } else {
                crc <<= 1;
            }
        }
    }
    crc as u32
}

/// CRC16 计算 (MODBUS)
fn crc16(data: &[u8]) -> u32 {
    let mut crc = 0xFFFFu16;
    for &byte in data {
        crc ^= byte as u16;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    crc as u32
}

/// CRC32 计算
fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFFu32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

/// 解析数据帧
pub fn parse_frame(data: &[u8], config: &ParseConfig) -> Option<DataFrame> {
    if config.mode != ParseMode::CustomFrame {
        return None;
    }
    
    // 查找帧头
    if !config.frame_header.is_empty() {
        let header_pos = data.windows(config.frame_header.len())
            .position(|window| window == config.frame_header.as_slice())?;
        
        // 查找帧尾
        if !config.frame_footer.is_empty() {
            let footer_pos = data[header_pos..].windows(config.frame_footer.len())
                .position(|window| window == config.frame_footer.as_slice())?;
            
            // 提取数据
            let frame_data = &data[header_pos..header_pos + footer_pos + config.frame_footer.len()];
            
            // 解析长度和校验和 (根据具体协议)
            // TODO: 实现具体协议解析
            
            return Some(DataFrame {
                header: config.frame_header.clone(),
                length: frame_data.len() as u16,
                data: frame_data.to_vec(),
                checksum: 0,
                footer: config.frame_footer.clone(),
            });
        }
    }
    
    None
}

/// 字节数组转十六进制字符串
pub fn bytes_to_hex(data: &[u8], separator: Option<&str>) -> String {
    match separator {
        Some(sep) => data.iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(sep),
        None => data.iter()
            .map(|b| format!("{:02X}", b))
            .collect(),
    }
}

/// 十六进制字符串转字节数组
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    let hex = hex.replace(" ", "").replace("-", "").replace("0x", "").replace("0X", "");
    
    if hex.len() % 2 != 0 {
        return Err("十六进制字符串长度必须是偶数".to_string());
    }
    
    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i+2], 16)
                .map_err(|e| format!("解析失败：{}", e))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc16() {
        let data = b"123456789";
        let crc = crc16(data);
        assert_eq!(crc, 0xBB3D); // MODBUS CRC16 of "123456789"
    }

    #[test]
    fn test_hex_conversion() {
        let data = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F];
        let hex = bytes_to_hex(&data, Some(" "));
        assert_eq!(hex, "48 65 6C 6C 6F");
        
        let bytes = hex_to_bytes(&hex).unwrap();
        assert_eq!(bytes, data);
    }
}
