//! 多串口管理模块

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// 串口连接信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerialConnection {
    pub id: String,
    pub port: String,
    pub baudrate: u32,
    pub connected: bool,
    pub label: Option<String>,
    pub color: String,
}

/// 多串口管理器
pub struct MultiSerialManager {
    connections: HashMap<String, SerialConnection>,
    max_connections: usize,
}

impl MultiSerialManager {
    pub fn new(max_connections: usize) -> Self {
        Self {
            connections: HashMap::new(),
            max_connections,
        }
    }
    
    /// 添加串口连接
    pub fn add_connection(&mut self, port: String, baudrate: u32, label: Option<String>) -> Result<String, String> {
        if self.connections.len() >= self.max_connections {
            return Err(format!("已达到最大连接数限制 ({})", self.max_connections));
        }
        
        // 检查是否已存在
        if self.connections.values().any(|c| c.port == port && c.connected) {
            return Err(format!("串口 {} 已连接", port));
        }
        
        let id = format!("serial_{}", self.connections.len() + 1);
        let color = Self::generate_color(self.connections.len());
        
        self.connections.insert(id.clone(), SerialConnection {
            id: id.clone(),
            port,
            baudrate,
            connected: false,
            label,
            color,
        });
        
        Ok(id)
    }
    
    /// 移除串口连接
    pub fn remove_connection(&mut self, id: &str) -> Result<(), String> {
        if self.connections.remove(id).is_some() {
            Ok(())
        } else {
            Err(format!("串口连接 {} 不存在", id))
        }
    }
    
    /// 获取所有连接
    pub fn get_connections(&self) -> Vec<&SerialConnection> {
        self.connections.values().collect()
    }
    
    /// 获取连接数
    pub fn get_connection_count(&self) -> usize {
        self.connections.len()
    }
    
    /// 生成颜色 (用于区分不同串口)
    fn generate_color(index: usize) -> String {
        let colors = [
            "#6366f1", // Indigo
            "#10b981", // Emerald
            "#f59e0b", // Amber
            "#ef4444", // Red
            "#8b5cf6", // Purple
            "#06b6d4", // Cyan
        ];
        colors[index % colors.len()].to_string()
    }
}

/// Tauri 命令

/// 添加串口连接
#[tauri::command]
pub async fn add_serial_connection(
    port: String,
    baudrate: u32,
    label: Option<String>,
    state: tauri::State<'_, Arc<Mutex<MultiSerialManager>>>,
) -> Result<String, String> {
    let mut manager = state.lock().await;
    manager.add_connection(port, baudrate, label)
}

/// 移除串口连接
#[tauri::command]
pub async fn remove_serial_connection(
    id: String,
    state: tauri::State<'_, Arc<Mutex<MultiSerialManager>>>,
) -> Result<(), String> {
    let mut manager = state.lock().await;
    manager.remove_connection(&id)
}

/// 获取所有串口连接
#[tauri::command]
pub async fn get_all_connections(
    state: tauri::State<'_, Arc<Mutex<MultiSerialManager>>>,
) -> Result<Vec<SerialConnection>, String> {
    let manager = state.lock().await;
    Ok(manager.get_connections().into_iter().cloned().collect())
}
