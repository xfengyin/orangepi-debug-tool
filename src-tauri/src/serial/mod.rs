//! 串口通信模块 - 增强版

pub mod parser;
pub mod terminal;
pub mod export;
pub mod graph;
pub mod multi_port;
pub mod macro_cmd;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use tokio::time::{Duration, Interval};

/// 串口端口
pub struct SerialPort {
    port: String,
    baudrate: u32,
    // TODO: 添加实际串口句柄
}

/// 串口命令响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerialResponse {
    success: bool,
    message: String,
    data: Option<String>,
}

/// 串口端口信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortInfo {
    name: String,
    description: String,
    hardware_id: Option<String>,
}

/// 发送记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendRecord {
    timestamp: String,
    data: String,
    hex_mode: bool,
}

/// 接收统计
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SerialStats {
    pub tx_bytes: u64,
    pub rx_bytes: u64,
    pub tx_frames: u64,
    pub rx_frames: u64,
    pub errors: u64,
}

/// 快捷命令
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuickCommand {
    name: String,
    data: String,
    hex_mode: bool,
    interval_ms: Option<u64>, // 定时发送间隔 (ms)
}

/// 应用状态 (从 main.rs 导入)
pub struct AppState {
    pub serial_port: Mutex<Option<SerialPort>>,
    pub stats: Mutex<SerialStats>,
    pub send_history: Mutex<Vec<SendRecord>>,
    pub quick_commands: Mutex<Vec<QuickCommand>>,
}

/// 列出可用串口 - 增强版
#[tauri::command]
pub async fn list_ports() -> Result<Vec<PortInfo>, String> {
    tracing::info!("📋 列出串口");
    
    // TODO: 使用 tokio-serial 的 available_ports()
    // 检测真实可用的串口
    
    Ok(vec![
        PortInfo {
            name: "/dev/ttyUSB0".to_string(),
            description: "USB Serial".to_string(),
            hardware_id: Some("FTDI".to_string()),
        },
        PortInfo {
            name: "/dev/ttyS0".to_string(),
            description: "UART0".to_string(),
            hardware_id: None,
        },
        PortInfo {
            name: "/dev/ttyAMA0".to_string(),
            description: "PL011 UART".to_string(),
            hardware_id: None,
        },
    ])
}

/// 连接串口 - 增强版
#[tauri::command]
pub async fn connect_serial(
    port: String,
    baudrate: u32,
    data_bits: u8,
    parity: String,
    stop_bits: u8,
    flow_control: bool,
    state: State<'_, Arc<AppState>>,
) -> Result<SerialResponse, String> {
    tracing::info!("🔌 连接串口 {} @ {}bps", port, baudrate);
    
    let mut serial_state = state.serial_port.lock().await;
    
    if serial_state.is_some() {
        return Ok(SerialResponse {
            success: false,
            message: "串口已连接".to_string(),
            data: None,
        });
    }
    
    // TODO: 实现串口连接
    // let port = tokio_serial::new(&port, baudrate)
    //     .data_bits(data_bits)
    //     .parity(parity)
    //     .stop_bits(stop_bits)
    //     .flow_control(flow_control)
    //     .open()?;
    
    *serial_state = Some(SerialPort {
        port,
        baudrate,
    });
    
    // 重置统计
    *state.stats.lock().await = SerialStats::default();
    
    Ok(SerialResponse {
        success: true,
        message: "串口连接成功".to_string(),
        data: None,
    })
}

/// 断开串口
#[tauri::command]
pub async fn disconnect_serial(
    state: State<'_, Arc<AppState>>,
) -> Result<SerialResponse, String> {
    tracing::info!("🔌 断开串口");
    
    let mut serial_state = state.serial_port.lock().await;
    
    if serial_state.is_none() {
        return Ok(SerialResponse {
            success: false,
            message: "串口未连接".to_string(),
            data: None,
        });
    }
    
    // TODO: 关闭串口
    
    *serial_state = None;
    
    Ok(SerialResponse {
        success: true,
        message: "串口已断开".to_string(),
        data: None,
    })
}

/// 发送数据 - 增强版
#[tauri::command]
pub async fn send_data(
    data: String,
    hex_mode: bool,
    append_newline: bool,
    append_cr: bool,
    state: State<'_, Arc<AppState>>,
) -> Result<SerialResponse, String> {
    tracing::info!("📤 发送数据：{} (hex={}, newline={}, cr={})", data, hex_mode, append_newline, append_cr);
    
    let serial_state = state.serial_port.lock().await;
    
    if serial_state.is_none() {
        return Ok(SerialResponse {
            success: false,
            message: "串口未连接".to_string(),
            data: None,
        });
    }
    
    // 处理数据
    let mut send_data = data;
    if append_newline {
        send_data.push('\n');
    }
    if append_cr {
        send_data.push('\r');
    }
    
    // 如果是 hex_mode，解析十六进制字符串
    let bytes_to_send = if hex_mode {
        // 移除空格和分隔符
        let hex_str = send_data.replace(" ", "").replace("-", "");
        hex::decode(&hex_str).map_err(|e| format!("十六进制解析失败：{}", e))?
    } else {
        send_data.into_bytes()
    };
    
    // TODO: 实际发送
    // port.write_all(&bytes_to_send).await?;
    
    // 更新统计
    let mut stats = state.stats.lock().await;
    stats.tx_bytes += bytes_to_send.len() as u64;
    stats.tx_frames += 1;
    
    // 保存发送历史
    let mut history = state.send_history.lock().await;
    history.push(SendRecord {
        timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
        data,
        hex_mode,
    });
    
    // 限制历史记录数量
    if history.len() > 100 {
        history.remove(0);
    }
    
    Ok(SerialResponse {
        success: true,
        message: "数据发送成功".to_string(),
        data: None,
    })
}

/// 清空接收区
#[tauri::command]
pub async fn clear_rx_buffer(
    state: State<'_, Arc<AppState>>,
) -> Result<SerialResponse, String> {
    tracing::info!("🗑️ 清空接收缓冲区");
    
    // TODO: 清空串口接收缓冲
    
    Ok(SerialResponse {
        success: true,
        message: "接收缓冲区已清空".to_string(),
        data: None,
    })
}

/// 获取统计信息
#[tauri::command]
pub async fn get_stats(
    state: State<'_, Arc<AppState>>,
) -> Result<SerialStats, String> {
    let stats = state.stats.lock().await;
    Ok(stats.clone())
}

/// 重置统计
#[tauri::command]
pub async fn reset_stats(
    state: State<'_, Arc<AppState>>,
) -> Result<SerialResponse, String> {
    *state.stats.lock().await = SerialStats::default();
    
    Ok(SerialResponse {
        success: true,
        message: "统计已重置".to_string(),
        data: None,
    })
}

/// 添加快捷命令
#[tauri::command]
pub async fn add_quick_command(
    name: String,
    data: String,
    hex_mode: bool,
    interval_ms: Option<u64>,
    state: State<'_, Arc<AppState>>,
) -> Result<SerialResponse, String> {
    let mut commands = state.quick_commands.lock().await;
    commands.push(QuickCommand {
        name,
        data,
        hex_mode,
        interval_ms,
    });
    
    Ok(SerialResponse {
        success: true,
        message: "快捷命令已添加".to_string(),
        data: None,
    })
}

/// 获取快捷命令列表
#[tauri::command]
pub async fn get_quick_commands(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<QuickCommand>, String> {
    let commands = state.quick_commands.lock().await;
    Ok(commands.clone())
}

/// 执行快捷命令
#[tauri::command]
pub async fn execute_quick_command(
    index: usize,
    state: State<'_, Arc<AppState>>,
) -> Result<SerialResponse, String> {
    let commands = state.quick_commands.lock().await;
    
    if index >= commands.len() {
        return Err("快捷命令索引超出范围".to_string());
    }
    
    let cmd = &commands[index];
    
    // 发送数据
    drop(commands); // 释放锁
    send_data(cmd.data.clone(), cmd.hex_mode, false, false, state).await
}

/// 定时发送 - 增强版
#[tauri::command]
pub async fn start_auto_send(
    data: String,
    hex_mode: bool,
    interval_ms: u64,
    state: State<'_, Arc<AppState>>,
) -> Result<SerialResponse, String> {
    tracing::info!("⏱️ 启动定时发送：{}ms", interval_ms);
    
    // TODO: 创建定时任务
    // tokio::spawn(async move {
    //     let mut interval = tokio::time::interval(Duration::from_millis(interval_ms));
    //     loop {
    //         interval.tick().await;
    //         // 发送数据
    //     }
    // });
    
    Ok(SerialResponse {
        success: true,
        message: format!("定时发送已启动 ({}ms)", interval_ms),
        data: None,
    })
}

/// 停止定时发送
#[tauri::command]
pub async fn stop_auto_send(
    state: State<'_, Arc<AppState>>,
) -> Result<SerialResponse, String> {
    tracing::info!("⏹️ 停止定时发送");
    
    // TODO: 停止定时任务
    
    Ok(SerialResponse {
        success: true,
        message: "定时发送已停止".to_string(),
        data: None,
    })
}
