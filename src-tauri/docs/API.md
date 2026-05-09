# OrangePi Zero3 调试工具 API 文档

## 概述

OrangePi Zero3 调试工具提供了一套完整的 REST API，用于控制串口、GPIO、PWM 等硬件接口。

## 基础信息

- **基础 URL**: `http://localhost:1420/api/v1`
- **认证**: 无（本地应用）
- **响应格式**: JSON

## 通用响应格式

```json
{
  "success": true,
  "data": {},
  "error": null,
  "timestamp": "2026-05-09T12:00:00Z"
}
```

错误响应：

```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "SERIAL_ERROR",
    "message": "串口通信错误: 连接失败",
    "numeric_code": 1001,
    "recovery_suggestion": "请检查串口连接是否正常"
  },
  "timestamp": "2026-05-09T12:00:00Z"
}
```

---

## 串口 API

### 列出可用串口

获取所有可用的串口设备列表。

**请求**

```
GET /serial/ports
```

**响应**

```json
{
  "success": true,
  "data": [
    {
      "port_name": "/dev/ttyUSB0",
      "port_type": "USB-Serial",
      "vid": 6790,
      "pid": 29987,
      "serial_number": "MOCK001",
      "manufacturer": "Mock Manufacturer",
      "product": "Mock USB-UART"
    }
  ]
}
```

---

### 连接串口

建立与串口的连接。

**请求**

```
POST /serial/connect
Content-Type: application/json

{
  "port_name": "/dev/ttyUSB0",
  "baud_rate": 115200
}
```

**参数**

| 参数 | 类型 | 必填 | 描述 |
|------|------|------|------|
| port_name | string | 是 | 串口设备路径 |
| baud_rate | number | 是 | 波特率 (9600, 115200 等) |

**响应**

```json
{
  "success": true,
  "data": {
    "connection_id": "uuid-xxx",
    "port_name": "/dev/ttyUSB0",
    "baud_rate": 115200,
    "connected_at": "2026-05-09T12:00:00Z"
  }
}
```

---

### 断开串口连接

断开指定的串口连接。

**请求**

```
POST /serial/disconnect
Content-Type: application/json

{
  "connection_id": "uuid-xxx"
}
```

**响应**

```json
{
  "success": true,
  "data": null
}
```

---

### 发送数据

通过串口发送数据。

**请求**

```
POST /serial/send
Content-Type: application/json

{
  "connection_id": "uuid-xxx",
  "data": "SGVsbG8gT3JhbmdlUGkh"  // Base64 编码
}
```

**响应**

```json
{
  "success": true,
  "data": {
    "bytes_sent": 14,
    "total_bytes_sent": 100
  }
}
```

---

### 接收数据

从串口读取数据。

**请求**

```
POST /serial/receive
Content-Type: application/json

{
  "connection_id": "uuid-xxx",
  "timeout_ms": 1000,
  "max_bytes": 1024
}
```

**响应**

```json
{
  "success": true,
  "data": {
    "data": "SGVsbG8gT3JhbmdlUGkh",
    "bytes_received": 14,
    "total_bytes_received": 100
  }
}
```

---

## GPIO API

### 列出引脚

获取所有可用的 GPIO 引脚。

**请求**

```
GET /gpio/pins
```

**响应**

```json
{
  "success": true,
  "data": [
    {
      "pin": 3,
      "name": "PA12",
      "modes": ["gpio", "i2c"],
      "is_exported": false
    }
  ]
}
```

---

### 导出引脚

导出 GPIO 引脚供应用程序使用。

**请求**

```
POST /gpio/export
Content-Type: application/json

{
  "pin": 3
}
```

**响应**

```json
{
  "success": true,
  "data": {
    "pin": 3,
    "exported": true
  }
}
```

---

### 取消导出

取消导出 GPIO 引脚。

**请求**

```
POST /gpio/unexport
Content-Type: application/json

{
  "pin": 3
}
```

**响应**

```json
{
  "success": true,
  "data": null
}
```

---

### 设置引脚方向

设置 GPIO 引脚的输入/输出方向。

**请求**

```
POST /gpio/set_direction
Content-Type: application/json

{
  "pin": 3,
  "direction": "output"  // "input" 或 "output"
}
```

**响应**

```json
{
  "success": true,
  "data": {
    "pin": 3,
    "direction": "output"
  }
}
```

---

### 读取引脚值

读取 GPIO 引脚的当前值。

**请求**

```
GET /gpio/read?pin=3
```

**响应**

```json
{
  "success": true,
  "data": {
    "pin": 3,
    "value": 1
  }
}
```

---

### 写入引脚值

写入 GPIO 引脚的值。

**请求**

```
POST /gpio/write
Content-Type: application/json

{
  "pin": 3,
  "value": 1  // 0 或 1
}
```

**响应**

```json
{
  "success": true,
  "data": {
    "pin": 3,
    "value": 1
  }
}
```

---

### 批量读取

批量读取多个 GPIO 引脚的值。

**请求**

```
POST /gpio/batch_read
Content-Type: application/json

{
  "pins": [3, 5, 7]
}
```

**响应**

```json
{
  "success": true,
  "data": {
    "values": {
      "3": 1,
      "5": 0,
      "7": 1
    }
  }
}
```

---

### 批量写入

批量写入多个 GPIO 引脚的值。

**请求**

```
POST /gpio/batch_write
Content-Type: application/json

{
  "values": [
    {"pin": 3, "value": 1},
    {"pin": 5, "value": 0},
    {"pin": 7, "value": 1}
  ]
}
```

**响应**

```json
{
  "success": true,
  "data": {
    "written_count": 3
  }
}
```

---

### 启用中断

为 GPIO 引脚启用中断检测。

**请求**

```
POST /gpio/interrupt/enable
Content-Type: application/json

{
  "pin": 3,
  "trigger": "rising"  // "rising", "falling", "both"
}
```

**响应**

```json
{
  "success": true,
  "data": {
    "pin": 3,
    "interrupt_enabled": true,
    "trigger": "rising"
  }
}
```

---

## PWM API

### 列出通道

获取所有可用的 PWM 通道。

**请求**

```
GET /pwm/channels
```

**响应**

```json
{
  "success": true,
  "data": [
    {
      "channel": 0,
      "name": "PWM0",
      "enabled": false
    },
    {
      "channel": 1,
      "name": "PWM1",
      "enabled": false
    }
  ]
}
```

---

### 启用通道

启用 PWM 通道。

**请求**

```
POST /pwm/enable
Content-Type: application/json

{
  "channel": 0
}
```

**响应**

```json
{
  "success": true,
  "data": {
    "channel": 0,
    "enabled": true
  }
}
```

---

### 禁用通道

禁用 PWM 通道。

**请求**

```
POST /pwm/disable
Content-Type: application/json

{
  "channel": 0
}
```

**响应**

```json
{
  "success": true,
  "data": {
    "channel": 0,
    "enabled": false
  }
}
```

---

### 设置频率

设置 PWM 信号的频率。

**请求**

```
POST /pwm/frequency
Content-Type: application/json

{
  "channel": 0,
  "frequency_hz": 1000
}
```

**响应**

```json
{
  "success": true,
  "data": {
    "channel": 0,
    "frequency_hz": 1000
  }
}
```

---

### 设置占空比

设置 PWM 信号的占空比。

**请求**

```
POST /pwm/duty_cycle
Content-Type: application/json

{
  "channel": 0,
  "duty_percent": 50.0
}
```

**响应**

```json
{
  "success": true,
  "data": {
    "channel": 0,
    "duty_percent": 50.0
  }
}
```

---

### 伺服控制

控制伺服电机角度。

**请求**

```
POST /pwm/servo
Content-Type: application/json

{
  "channel": 0,
  "angle": 90.0  // 0-180 度
}
```

**响应**

```json
{
  "success": true,
  "data": {
    "channel": 0,
    "angle": 90.0,
    "duty_percent": 7.5
  }
}
```

---

### 电机控制

控制直流电机速度。

**请求**

```
POST /pwm/motor
Content-Type: application/json

{
  "channel": 0,
  "speed_percent": 75.0  // -100 到 100
}
```

**响应**

```json
{
  "success": true,
  "data": {
    "channel": 0,
    "speed_percent": 75.0,
    "duty_percent": 75.0
  }
}
```

---

### 淡入效果

渐变增加 PWM 输出。

**请求**

```
POST /pwm/fade_in
Content-Type: application/json

{
  "channel": 0,
  "target_duty": 100.0,
  "duration_ms": 2000
}
```

**响应**

```json
{
  "success": true,
  "data": null
}
```

---

### 淡出效果

渐变减少 PWM 输出到零。

**请求**

```
POST /pwm/fade_out
Content-Type: application/json

{
  "channel": 0,
  "duration_ms": 2000
}
```

**响应**

```json
{
  "success": true,
  "data": null
}
```

---

## 系统 API

### 健康检查

获取系统健康状态。

**请求**

```
GET /health
```

**响应**

```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "components": {
      "serial": {
        "status": "healthy",
        "latency_ms": 5
      },
      "gpio": {
        "status": "healthy",
        "latency_ms": 2
      },
      "pwm": {
        "status": "healthy",
        "latency_ms": 3
      }
    },
    "timestamp": "2026-05-09T12:00:00Z"
  }
}
```

---

### 获取指标

获取 Prometheus 格式的指标数据。

**请求**

```
GET /metrics
```

**响应**

```
# HELP serial_bytes_received_total Total bytes received
# TYPE serial_bytes_received_total counter
serial_bytes_received_total 1234

# HELP gpio_pin_reads_total Total GPIO pin reads
# TYPE gpio_pin_reads_total counter
gpio_pin_reads_total 567
```

---

### 获取版本

获取应用程序版本信息。

**请求**

```
GET /version
```

**响应**

```json
{
  "success": true,
  "data": {
    "version": "2.0.0",
    "build_date": "2026-05-09",
    "features": ["hardware-support", "mock-hardware"]
  }
}
```

---

## 错误码

| 错误码 | 代码 | 描述 |
|--------|------|------|
| 1001 | SERIAL_ERROR | 串口通信错误 |
| 2001 | GPIO_ERROR | GPIO 操作错误 |
| 3001 | PWM_ERROR | PWM 控制错误 |
| 4001 | DEVICE_ERROR | 设备错误 |
| 5001 | DB_ERROR | 数据库错误 |
| 6001 | CONFIG_ERROR | 配置错误 |
| 7001 | IO_ERROR | I/O 错误 |
| 8001 | INVALID_ARG | 参数错误 |
| 9001 | NOT_FOUND | 未找到 |
| 10001 | PERMISSION_DENIED | 权限不足 |
| 11001 | TIMEOUT | 操作超时 |
| 12001 | CIRCUIT_BREAKER_OPEN | 服务熔断 |
| 99999 | INTERNAL_ERROR | 内部错误 |

---

## 示例代码

### JavaScript/TypeScript

```typescript
const API_BASE = 'http://localhost:1420/api/v1';

async function connectSerial(port: string, baudRate: number) {
  const response = await fetch(`${API_BASE}/serial/connect`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ port_name: port, baud_rate: baudRate })
  });
  return response.json();
}

async function writeGpio(pin: number, value: number) {
  const response = await fetch(`${API_BASE}/gpio/write`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ pin, value })
  });
  return response.json();
}
```

### Python

```python
import requests
import base64

API_BASE = 'http://localhost:1420/api/v1'

def connect_serial(port: str, baud_rate: int):
    response = requests.post(
        f'{API_BASE}/serial/connect',
        json={'port_name': port, 'baud_rate': baud_rate}
    )
    return response.json()

def write_gpio(pin: int, value: int):
    response = requests.post(
        f'{API_BASE}/gpio/write',
        json={'pin': pin, 'value': value}
    )
    return response.json()
```
