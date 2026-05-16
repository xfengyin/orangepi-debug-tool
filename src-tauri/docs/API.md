# API 文档

## 概述

OrangePi Debug Tool 提供了一系列 Tauri 命令用于串口和网络调试。所有命令都通过 JSON 格式进行交互。

## 基础响应格式

```json
{
  "success": true,
  "data": {},
  "error": null
}
```

错误响应：

```json
{
  "success": false,
  "data": null,
  "error": {
    "code": 1001,
    "message": "错误描述"
  }
}
```

## 串口命令

### list_serial_ports

列出所有可用的串口。

**请求：**

```json
{
  "command": "list_serial_ports"
}
```

**响应：**

```json
{
  "success": true,
  "data": [
    {
      "name": "/dev/ttyUSB0",
      "port_type": "USB",
      "baudrate": 9600
    },
    {
      "name": "/dev/ttyACM0",
      "port_type": "ACM",
      "baudrate": 115200
    }
  ]
}
```

**字段说明：**

- `name`: 串口设备路径 (Linux: `/dev/tty*`, Windows: `COM*`)
- `port_type`: 端口类型 (USB, ACM, Bluetooth 等)
- `baudrate`: 当前波特率

### connect_serial

连接串口。

**请求：**

```json
{
  "command": "connect_serial",
  "params": {
    "port_name": "/dev/ttyUSB0",
    "baudrate": 115200,
    "data_bits": 8,
    "stop_bits": 1,
    "parity": "none"
  }
}
```

**参数说明：**

- `port_name` (必需): 串口设备路径
- `baudrate` (必需): 波特率 (可选值: 300, 1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200, 230400, 460800, 921600)
- `data_bits` (可选, 默认 8): 数据位 (5, 6, 7, 8)
- `stop_bits` (可选, 默认 1): 停止位 (1, 2)
- `parity` (可选, 默认 "none"): 校验位 ("none", "even", "odd")

**响应：**

```json
{
  "success": true,
  "data": "conn-uuid-string"
}
```

**错误码：**

- 1001: 串口未找到
- 1002: 连接失败
- 9003: 参数无效

### disconnect_serial

断开串口连接。

**请求：**

```json
{
  "command": "disconnect_serial",
  "params": {
    "id": "conn-uuid-string"
  }
}
```

**响应：**

```json
{
  "success": true,
  "data": null
}
```

### send_serial

发送数据到串口。

**请求：**

```json
{
  "command": "send_serial",
  "params": {
    "id": "conn-uuid-string",
    "data": "FF 00 12 34",
    "format": "hex"
  }
}
```

**参数说明：**

- `id` (必需): 连接 ID
- `data` (必需): 要发送的数据
- `format` (可选, 默认 "hex"): 数据格式 ("hex" 或 "ascii")

**响应：**

```json
{
  "success": true,
  "data": {
    "bytes_sent": 4
  }
}
```

**错误码：**

- 1004: 写入错误

### start_timing_send

启动定时发送。

**请求：**

```json
{
  "command": "start_timing_send",
  "params": {
    "id": "conn-uuid-string",
    "data": "FF 00 12 34",
    "format": "hex",
    "interval_ms": 1000
  }
}
```

**参数说明：**

- `id` (必需): 连接 ID
- `data` (必需): 要发送的数据
- `format` (可选, 默认 "hex"): 数据格式
- `interval_ms` (必需): 发送间隔 (毫秒, 最小值: 10)

**响应：**

```json
{
  "success": true,
  "data": {
    "task_id": "task-uuid-string"
  }
}
```

### stop_timing_send

停止定时发送。

**请求：**

```json
{
  "command": "stop_timing_send",
  "params": {
    "task_id": "task-uuid-string"
  }
}
```

**响应：**

```json
{
  "success": true,
  "data": null
}
```

### get_serial_status

获取串口连接状态。

**请求：**

```json
{
  "command": "get_serial_status",
  "params": {
    "id": "conn-uuid-string"
  }
}
```

**响应：**

```json
{
  "success": true,
  "data": {
    "connected": true,
    "port_name": "/dev/ttyUSB0",
    "baudrate": 115200,
    "bytes_sent": 1024,
    "bytes_received": 2048
  }
}
```

## 网络命令

### create_tcp_server

创建 TCP 服务器。

**请求：**

```json
{
  "command": "create_tcp_server",
  "params": {
    "port": 8080,
    "host": "0.0.0.0"
  }
}
```

**参数说明：**

- `port` (必需): 监听端口
- `host` (可选, 默认 "0.0.0.0"): 监听地址

**响应：**

```json
{
  "success": true,
  "data": {
    "server_id": "server-uuid-string",
    "port": 8080
  }
}
```

**错误码：**

- 2001: 连接被拒绝 (端口已被占用)

### connect_tcp

连接 TCP 服务器。

**请求：**

```json
{
  "command": "connect_tcp",
  "params": {
    "host": "192.168.1.100",
    "port": 8080
  }
}
```

**响应：**

```json
{
  "success": true,
  "data": {
    "connection_id": "conn-uuid-string"
  }
}
```

**错误码：**

- 2001: 连接被拒绝
- 2003: 连接超时

### connect_udp

创建 UDP 会话。

**请求：**

```json
{
  "command": "connect_udp",
  "params": {
    "local_port": 0,
    "remote_host": "192.168.1.100",
    "remote_port": 8080
  }
}
```

**参数说明：**

- `local_port` (可选, 默认 0): 本地端口 (0 表示自动分配)
- `remote_host` (必需): 远程主机地址
- `remote_port` (必需): 远程端口

**响应：**

```json
{
  "success": true,
  "data": {
    "session_id": "session-uuid-string"
  }
}
```

### send_network_data

发送网络数据。

**请求：**

```json
{
  "command": "send_network_data",
  "params": {
    "id": "conn-uuid-string",
    "data": "48656C6C6F20576F726C64",
    "format": "hex"
  }
}
```

**参数说明：**

- `id` (必需): 连接/会话 ID
- `data` (必需): 要发送的数据
- `format` (可选, 默认 "hex"): 数据格式 ("hex" 或 "ascii")

**响应：**

```json
{
  "success": true,
  "data": {
    "bytes_sent": 11
  }
}
```

**错误码：**

- 2002: 连接已关闭
- 2004: 发送错误

### close_connection

关闭网络连接。

**请求：**

```json
{
  "command": "close_connection",
  "params": {
    "id": "conn-uuid-string"
  }
}
```

**响应：**

```json
{
  "success": true,
  "data": null
}
```

### get_network_status

获取网络连接状态。

**请求：**

```json
{
  "command": "get_network_status",
  "params": {
    "id": "conn-uuid-string"
  }
}
```

**响应：**

```json
{
  "success": true,
  "data": {
    "connected": true,
    "local_addr": "192.168.1.50:54321",
    "remote_addr": "192.168.1.100:8080",
    "bytes_sent": 1024,
    "bytes_received": 2048
  }
}
```

## 日志命令

### save_log

保存日志到文件。

**请求：**

```json
{
  "command": "save_log",
  "params": {
    "filename": "debug_log_20240115.txt",
    "format": "text"
  }
}
```

**参数说明：**

- `filename` (必需): 文件名
- `format` (可选, 默认 "text"): 格式 ("text" 或 "json")

**响应：**

```json
{
  "success": true,
  "data": {
    "path": "/path/to/logs/debug_log_20240115.txt",
    "size": 10240
  }
}
```

### export_log

导出日志数据。

**请求：**

```json
{
  "command": "export_log",
  "params": {
    "start_time": "2024-01-01T00:00:00Z",
    "end_time": "2024-01-15T23:59:59Z",
    "filter": {
      "type": "serial"
    }
  }
}
```

**参数说明：**

- `start_time` (可选): 开始时间 (ISO 8601 格式)
- `end_time` (可选): 结束时间
- `filter` (可选): 过滤器

**响应：**

```json
{
  "success": true,
  "data": {
    "count": 100,
    "logs": [
      {
        "timestamp": "2024-01-15T10:30:00Z",
        "type": "serial",
        "direction": "send",
        "data": "FF 00 12 34"
      }
    ]
  }
}
```

### get_statistics

获取统计数据。

**请求：**

```json
{
  "command": "get_statistics",
  "params": {}
}
```

**响应：**

```json
{
  "success": true,
  "data": {
    "serial": {
      "total_connections": 5,
      "total_bytes_sent": 10240,
      "total_bytes_received": 20480
    },
    "network": {
      "total_connections": 3,
      "total_bytes_sent": 5120,
      "total_bytes_received": 10240
    },
    "logs": {
      "total_count": 1000,
      "file_size": 512000
    }
  }
}
```

### clear_logs

清除所有日志。

**请求：**

```json
{
  "command": "clear_logs",
  "params": {}
}
```

**响应：**

```json
{
  "success": true,
  "data": null
}
```

## 错误码

| 错误码 | 类别 | 说明 |
|--------|------|------|
| 1001 | serial | 串口未找到 |
| 1002 | serial | 连接失败 |
| 1003 | serial | 读取错误 |
| 1004 | serial | 写入错误 |
| 2001 | network | 连接被拒绝 |
| 2002 | network | 连接已关闭 |
| 2003 | network | 超时 |
| 2004 | network | 发送错误 |
| 3001 | config | 配置未找到 |
| 3002 | config | 配置解析错误 |
| 3003 | config | 配置验证失败 |
| 9001 | general | 内部错误 |
| 9002 | general | 未实现 |
| 9003 | general | 参数无效 |
| 9004 | general | 资源忙 |

## 事件

### serial_data_received

串口数据接收事件。

```json
{
  "event": "serial_data_received",
  "data": {
    "id": "conn-uuid-string",
    "data": "FF 00 12 34",
    "format": "hex",
    "timestamp": "2024-01-15T10:30:00Z"
  }
}
```

### network_data_received

网络数据接收事件。

```json
{
  "event": "network_data_received",
  "data": {
    "id": "conn-uuid-string",
    "data": "48656C6C6F",
    "format": "hex",
    "timestamp": "2024-01-15T10:30:00Z"
  }
}
```

### connection_closed

连接关闭事件。

```json
{
  "event": "connection_closed",
  "data": {
    "id": "conn-uuid-string",
    "reason": "peer_closed"
  }
}
```

## 数据格式

### HEX 格式

数据以十六进制字符串形式传输,字节之间用空格分隔。

示例: `"FF 00 12 34 AB CD"`

### ASCII 格式

数据以 ASCII 字符串形式传输。

示例: `"Hello World"`

### 转换规则

- 发送时: HEX 字符串 → 字节数组 → 写入设备
- 接收时: 字节数组 → HEX 字符串 → 返回给前端
