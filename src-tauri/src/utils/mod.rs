//! Utility modules

pub mod logging;
pub mod binary_protocol;

use crate::AppResult;

/// Convert bytes to hex string
#[inline]
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ")
}

/// Convert hex string to bytes
#[inline]
pub fn hex_to_bytes(hex: &str) -> AppResult<Vec<u8>> {
    let hex = hex.replace(" ", "").replace("0x", "");
    if hex.len() % 2 != 0 {
        return Err(crate::AppError::InvalidArgument(
            "Hex string length must be even".to_string(),
        ));
    }
    
    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| crate::AppError::InvalidArgument(format!("Invalid hex: {}", e)))
        })
        .collect()
}

/// Format bytes as human-readable string
#[inline]
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.2} {}", size, UNITS[unit_index])
}

/// Truncate string with ellipsis
#[inline]
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}