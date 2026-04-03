# API 文档

## Tauri 命令接口

### 串口相关

#### `list_serial_ports`

列出可用的串口设备。

**参数:** 无

**返回:** `ApiResponse<SerialPortInfo[]>`

```typescript
interface SerialPortInfo {
  port_name: string;
  port_type: string;
  vid?: number;
  pid?: number;
  serial_number?: string;
  manufacturer?: string;
  product?: string;
}
```

**示例:**
```typescript
const response = await invoke<ApiResponse<SerialPortInfo[]>>('list_serial_ports');
```

---

#### `auto_detect_serial`

自动检测 OrangePi 串口设备。

**参数:** 无

**返回:** `ApiResponse<string | null>`

---

#### `connect_serial`

连接到指定的串口。

**参数:**
```typescript
{
  config: SerialConfig
}
```

**返回:** `ApiResponse<null>`

---

#### `disconnect_serial`

断开当前串口连接。

**参数:** 无

**返回:** `ApiResponse<null>`

---

#### `write_serial`

向串口写入二进制数据。

**参数:**
```typescript
{
  data: number[]  // 字节数组
}
```

**返回:** `ApiResponse<number>` (写入的字节数)

---

#### `write_serial_string`

向串口写入字符串数据。

**参数:**
```typescript
{
  data: string
}
```

**返回:** `ApiResponse<number>` (写入的字节数)

---

#### `get_serial_status`

获取当前串口连接状态。

**参数:** 无

**返回:** `ApiResponse<SerialStatus>`

```typescript
interface SerialStatus {
  connected: boolean;
  config?: SerialConfig;
}
```

---

### GPIO 相关

#### `list_gpio_pins`

列出可用的 GPIO 引脚。

**参数:** 无

**返回:** `ApiResponse<GpioPinInfo[]>`

```typescript
interface GpioPinInfo {
  pin: number;
  name: string;
  modes: string[];
  current_mode?: string;
  is_exported: boolean;
}
```

---

#### `configure_gpio`

配置 GPIO 引脚。

**参数:**
```typescript
{
  config: GpioConfig
}
```

```typescript
interface GpioConfig {
  pin: number;
  direction: 'input' | 'output';
  pull: 'none' | 'up' | 'down';
  initial_value: number;
  interrupt?: {
    trigger: 'rising' | 'falling' | 'both' | 'high' | 'low';
    debounce_ms: number;
  };
}
```

**返回:** `ApiResponse<null>`

---

#### `read_gpio`

读取 GPIO 引脚值。

**参数:**
```typescript
{
  pin: number
}
```

**返回:** `ApiResponse<number>` (0 或 1)

---

#### `write_gpio`

写入 GPIO 引脚值。

**参数:**
```typescript
{
  pin: number;
  value: number  // 0 或 1
}
```

**返回:** `ApiResponse<null>`

---

#### `toggle_gpio`

翻转 GPIO 引脚值。

**参数:**
```typescript
{
  pin: number
}
```

**返回:** `ApiResponse<number>` (新的值)

---

#### `unconfigure_gpio`

取消 GPIO 引脚配置。

**参数:**
```typescript
{
  pin: number
}
```

**返回:** `ApiResponse<null>`

---

### PWM 相关

#### `list_pwm_channels`

列出可用的 PWM 通道。

**参数:** 无

**返回:** `ApiResponse<PwmChannelInfo[]>`

```typescript
interface PwmChannelInfo {
  channel: number;
  chip: number;
  frequency: number;
  duty_cycle: number;
  polarity: 'normal' | 'inverted';
  enabled: boolean;
  period_ns: number;
  duty_cycle_ns: number;
}
```

---

#### `configure_pwm`

配置 PWM 通道。

**参数:**
```typescript
{
  config: PwmConfig
}
```

```typescript
interface PwmConfig {
  channel: number;
  chip: number;
  frequency: number;
  duty_cycle: number;
  polarity: 'normal' | 'inverted';
  enabled: boolean;
}
```

**返回:** `ApiResponse<null>`

---

#### `set_pwm_frequency`

设置 PWM 频率。

**参数:**
```typescript
{
  chip: number;
  channel: number;
  frequency: number  // Hz
}
```

**返回:** `ApiResponse<null>`

---

#### `set_pwm_duty_cycle`

设置 PWM 占空比。

**参数:**
```typescript
{
  chip: number;
  channel: number;
  duty_cycle: number  // 0-100
}
```

**返回:** `ApiResponse<null>`

---

#### `set_pwm_enabled`

启用/禁用 PWM 通道。

**参数:**
```typescript
{
  chip: number;
  channel: number;
  enabled: boolean
}
```

**返回:** `ApiResponse<null>`

---

#### `play_pwm_waveform`

播放 PWM 波形。

**参数:**
```typescript
{
  chip: number;
  channel: number;
  waveform: PwmWaveform
}
```

```typescript
interface PwmWaveform {
  name: string;
  frequency_steps: number[];
  duty_steps: number[];
  step_duration_ms: number;
  cycles: number;
}
```

**返回:** `ApiResponse<null>`

---

### 系统相关

#### `get_config`

获取应用配置。

**参数:** 无

**返回:** `ApiResponse<AppConfig>`

```typescript
interface AppConfig {
  theme: 'light' | 'dark' | 'system';
  language: string;
  auto_save_interval: number;
  serial_buffer_size: number;
  max_log_entries: number;
  hardware_acceleration: boolean;
}
```

---

#### `update_config`

更新应用配置。

**参数:**
```typescript
{
  config: AppConfig
}
```

**返回:** `ApiResponse<null>`

---

#### `get_system_info`

获取系统信息。

**参数:** 无

**返回:** `ApiResponse<SystemInfo>`

```typescript
interface SystemInfo {
  version: string;
  platform: string;
  arch: string;
}
```

---

#### `check_orangepi`

检查是否运行在 OrangePi 上。

**参数:** 无

**返回:** `ApiResponse<boolean>`

---

#### `open_link`

打开外部链接。

**参数:**
```typescript
{
  url: string
}
```

**返回:** `ApiResponse<null>`

---

## 错误码

| 错误码 | 说明 |
|--------|------|
| `SERIAL_ERROR` | 串口通信错误 |
| `GPIO_ERROR` | GPIO 操作错误 |
| `PWM_ERROR` | PWM 操作错误 |
| `DEVICE_ERROR` | 设备检测错误 |
| `DB_ERROR` | 数据库错误 |
| `CONFIG_ERROR` | 配置错误 |
| `IO_ERROR` | I/O 错误 |
| `INVALID_ARG` | 参数错误 |
| `NOT_FOUND` | 资源未找到 |
| `PERMISSION_DENIED` | 权限不足 |
| `TIMEOUT` | 操作超时 |
| `INTERNAL_ERROR` | 内部错误 |

---

## 事件系统

### 后端事件 (待实现)

```typescript
// 串口数据接收
listen('serial_data', (event) => {
  const data: SerialPacket = event.payload;
  // 处理数据
});

// GPIO 状态变化
listen('gpio_change', (event) => {
  const data: GpioEvent = event.payload;
  // 处理变化
});

// PWM 状态变化
listen('pwm_change', (event) => {
  const data: PwmEvent = event.payload;
  // 处理变化
});

// 日志消息
listen('log_message', (event) => {
  const data: LogEntry = event.payload;
  // 处理日志
});
```