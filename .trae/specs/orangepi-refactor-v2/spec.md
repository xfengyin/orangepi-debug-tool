# OrangePi Debug Tool V2 - 企业级重构优化规范

## 一、项目背景

### 1.1 项目概述
**项目名称**: orangepi-debug-tool
**技术栈**: Tauri 2.0 + React + TypeScript + Rust
**目标平台**: Windows 10/11 + OrangePi ARM Linux

### 1.2 重构目标
1. 构建**企业级、高可用、可观测**的调试工具架构
2. 实现**开闭原则**的插件化设计，支持设备无缝替换
3. 确保**生产级**的稳定性、性能、安全性
4. 解决现有卡顿、丢包、崩溃问题

### 1.3 重构约束
- 通信完全兼容：串口参数、TCP/UDP 收发格式、HEX/ASCII 模式不变
- 功能 1:1 不变：所有原有功能必须保留
- 跨平台不变：支持 Windows + OrangePi ARM Linux
- 实时性不劣化：高波特率不丢包、UI 不卡顿
- UI 不变：布局、操作流程、快捷键保持一致

---

## 二、架构设计

### 2.1 整体架构分层

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Frontend Layer (React)                          │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │  Serial  │  │   TCP    │  │   UDP    │  │   Log    │  │   Stats  │  │
│  │   Page   │  │   Page   │  │   Page   │  │   Page   │  │   Page   │  │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘  │
│       │              │              │              │              │         │
│  ┌────┴──────────────┴──────────────┴──────────────┴──────────────┴─────┐  │
│  │                      Zustand State Layer                              │  │
│  │  ConfigStore  │  DeviceStore  │  SessionStore  │  UIStore         │  │
│  └────────────────────────────┬───────────────────────────────────────────┘  │
└──────────────────────────────┼───────────────────────────────────────────────┘
                               │ Tauri IPC (Binary Protocol)
┌──────────────────────────────┼───────────────────────────────────────────────┐
│                              Backend Layer (Rust)                            │
│  ┌───────────────────────────┴───────────────────────────────────────────┐  │
│  │                      Command Gateway Layer                              │  │
│  │  Serial Commands  │  TCP/UDP Commands  │  Log Commands  │  Stats Cmd │  │
│  └───────────────────────────┬───────────────────────────────────────────┘  │
│                              │                                              │
│  ┌───────────────────────────┴───────────────────────────────────────────┐  │
│  │                        Service Layer                                   │  │
│  │  SerialService  │  NetworkService  │  LogService  │  ConfigService  │  │
│  └───────────────────────────┬───────────────────────────────────────────┘  │
│                              │                                              │
│  ┌───────────────────────────┴───────────────────────────────────────────┐  │
│  │                    Device Adapter Layer (SPI)                          │  │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐    │  │
│  │  │ SerialPort │  │  TCP/UDP   │  │   File     │  │    Mock    │    │  │
│  │  │  Adapter   │  │   Adapter  │  │  Adapter   │  │  Adapter   │    │  │
│  │  └────────────┘  └────────────┘  └────────────┘  └────────────┘    │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                    Infrastructure Layer                               │  │
│  │  Logging  │  Metrics  │  Health  │  Config  │  Error Handling     │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 多线程模型

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Main Thread (UI)                                │
│  - Tauri Event Loop                                                        │
│  - UI Rendering (React)                                                    │
│  - User Input Processing                                                   │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                    ┌───────────────┼───────────────┐
                    ▼               ▼               ▼
┌──────────────────┐ ┌──────────────┐ ┌──────────────┐
│  Serial Worker   │ │ Network Work │ │   Log Worker │
│    Thread Pool   │ │    Thread    │ │    Thread    │
├──────────────────┤ ├──────────────┤ ├──────────────┤
│ - read_loop()    │ │ - tcp_accept │ │ - async_write│
│ - write_queue()   │ │ - udp_recv   │ │ - rotate     │
│ - port_monitor()  │ │ - keep_alive │ │ - compress   │
└──────────────────┘ └──────────────┘ └──────────────┘
         │                  │                │
         └──────────────────┼────────────────┘
                            ▼
              ┌─────────────────────────┐
              │   Channel-based IPC     │
              │   (mpsc::unbounded)    │
              └─────────────────────────┘
                            │
                            ▼
              ┌─────────────────────────┐
              │     Event Emission      │
              │   (emit to Frontend)    │
              └─────────────────────────┘
```

### 2.3 防丢包、防卡顿方案

#### 串口防丢包策略
```rust
// 环形缓冲区设计
pub struct CircularBuffer {
    buffer: Vec<u8>,
    write_pos: AtomicUsize,
    read_pos: AtomicUsize,
    capacity: usize,
}

// 批量读取优化
impl SerialPort {
    pub async fn read_batch(&self, buffer: &mut [u8]) -> Result<usize> {
        let mut total_read = 0;
        let deadline = Instant::now() + Duration::from_millis(self.config.read_timeout_ms);

        while total_read < buffer.len() {
            let remaining = buffer.len() - total_read;
            let chunk = self.port.read(&mut buffer[total_read..total_read + remaining.min(4096)])?;
            if chunk == 0 {
                if Instant::now() >= deadline { break; }
                tokio::time::sleep(Duration::from_micros(100)).await;
                continue;
            }
            total_read += chunk;
        }
        Ok(total_read)
    }
}
```

#### 网络防卡顿策略
```rust
// 连接池 + 超时控制
pub struct ConnectionPool<T> {
    pool: Vec<PooledConnection<T>>,
    max_idle: usize,
    timeout: Duration,
}

// 非阻塞写入队列
pub struct AsyncWriteQueue {
    sender: mpsc::Sender<WriteRequest>,
    high_priority: bool,
}
```

#### UI 响应优化
```rust
// 节流发送，避免高频更新
pub struct ThrottledEmitter {
    last_emit: AtomicInstant,
    throttle_ms: u64,
    pending: Mutex<Vec<DataPacket>>,
}

// 批量事件合并
pub struct BatchEmitter {
    buffer: Vec<DataEvent>,
    flush_interval: Duration,
}
```

---

## 三、核心模块设计

### 3.1 设备适配器层 (Device Adapter Layer)

#### Trait 定义
```rust
/// 基础设备适配器 Trait
#[async_trait]
pub trait DeviceAdapter: Send + Sync {
    fn id(&self) -> &'static str;
    async fn health_check(&self) -> Result<HealthStatus, DeviceError>;
    async fn initialize(&self) -> Result<(), DeviceError>;
    async fn shutdown(&self) -> Result<(), DeviceError>;
}

/// 串口适配器 Trait
#[async_trait]
pub trait SerialAdapter: DeviceAdapter {
    async fn list_ports(&self) -> Result<Vec<SerialPortInfo>, DeviceError>;
    async fn connect(&self, config: &SerialConfig) -> Result<SerialHandle, DeviceError>;
    async fn disconnect(&self, handle: &SerialHandle) -> Result<(), DeviceError>;
    async fn read(&self, handle: &SerialHandle, buf: &mut [u8]) -> Result<usize, DeviceError>;
    async fn write(&self, handle: &SerialHandle, data: &[u8]) -> Result<usize, DeviceError>;
}

/// 网络适配器 Trait
#[async_trait]
pub trait NetworkAdapter: DeviceAdapter {
    async fn connect_tcp(&self, addr: &str, port: u16) -> Result<TcpHandle, DeviceError>;
    async fn connect_udp(&self, addr: &str, port: u16) -> Result<UdpHandle, DeviceError>;
    async fn send(&self, handle: &NetworkHandle, data: &[u8]) -> Result<(), DeviceError>;
    async fn receive(&self, handle: &NetworkHandle, buf: &mut [u8]) -> Result<usize, DeviceError>;
}
```

### 3.2 服务层 (Service Layer)

```rust
/// 串口服务
pub struct SerialService {
    adapter: Arc<dyn SerialAdapter>,
    connections: RwLock<HashMap<String, Arc<SerialConnection>>>,
    metrics: Arc<MetricsCollector>,
}

impl SerialService {
    pub async fn send_with_retry(&self, port: &str, data: &[u8], retries: u32) -> Result<()>;
    pub fn enable_timing_mode(&self, port: &str, interval_ms: u64) -> Result<()>;
}

/// 网络服务
pub struct NetworkService {
    adapter: Arc<dyn NetworkAdapter>,
    tcp_connections: RwLock<HashMap<String, TcpStream>>,
    udp_sessions: RwLock<HashMap<String, UdpSocket>>,
}

impl NetworkService {
    pub async fn create_tcp_server(&self, port: u16) -> Result<TcpListener>;
    pub async fn send_udp_broadcast(&self, data: &[u8], port: u16) -> Result<()>;
}
```

---

## 四、内存泄漏/崩溃修复点

### 4.1 常见问题修复清单

| 问题 | 根因 | 修复方案 |
|------|------|----------|
| 串口掉线崩溃 | 未处理 disconnect 事件 | 添加 port_disconnected 监听 |
| 内存持续增长 | 环形缓冲区未释放 | 使用固定容量池 |
| 发送卡顿 | 同步写入阻塞 | 改用 async 写入队列 |
| UI 假死 | 主线程阻塞 | Worker 线程处理通信 |
| 连接泄漏 | 未超时关闭 | 添加连接超时控制 |
| 数据丢失 | 缓冲区溢出 | 实现背压控制 |

### 4.2 生命周期管理
```rust
impl Drop for SerialConnection {
    fn drop(&mut self) {
        if let Some(port) = self.port.take() {
            let _ = port.clear_break();
            let _ = port.clear_break_condition();
        }
        info!("SerialConnection dropped for port {}", self.config.port_name);
    }
}
```

---

## 五、代码规范

### 5.1 模块结构
```
src-tauri/src/
├── main.rs                 # 入口点
├── lib.rs                  # 库导出
├── adapters/               # 设备适配器层
│   ├── mod.rs
│   ├── serial/
│   │   ├── mod.rs
│   │   └── native.rs       # native serialport 实现
│   ├── network/
│   │   ├── mod.rs
│   │   └── tokio_net.rs    # tokio 网络实现
│   └── mock/
│       └── mod.rs          # 测试用 mock
├── services/               # 服务层
│   ├── mod.rs
│   ├── serial_service.rs
│   ├── network_service.rs
│   └── log_service.rs
├── commands/               # Tauri 命令
│   ├── mod.rs
│   ├── serial_commands.rs
│   └── network_commands.rs
├── state/                   # 状态管理
│   ├── mod.rs
│   └── app_state.rs
├── infrastructure/          # 基础设施
│   ├── error.rs
│   ├── metrics.rs
│   └── health.rs
└── utils/                   # 工具函数
    ├── mod.rs
    └── hex_utils.rs
```

### 5.2 命名规范
- 模块: snake_case
- Trait: PascalCase
- 函数: snake_case
- 常量: SCREAMING_SNAKE_CASE
- 类型: PascalCase

---

## 六、编译与部署

### 6.1 开发环境
```bash
# Ubuntu/Debian
sudo apt install -y libwebkit2gtk-4.1-dev build-essential \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable

# 安装 Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs
```

### 6.2 构建命令
```bash
# 开发模式
npm run tauri:dev

# 生产构建
npm run tauri:build

# 跨平台交叉编译 (ARM)
cargo build --target armv7-unknown-linux-gnueabihf --release
```

### 6.3 部署
```bash
# Windows
./src-tauri/target/release/bundle/msi/*.msi

# Linux ARM
scp src-tauri/target/armv7-unknown-linux-gnueabihf/release/orangepi-debug-tool pi@orangepi:~/
```

---

## 七、测试验证

### 7.1 单元测试
```bash
cd src-tauri
cargo test
```

### 7.2 集成测试
```bash
# 串口测试 (需要硬件)
cargo test serial_integration

# 网络测试
cargo test network_integration
```

### 7.3 性能测试
```bash
# 高频数据吞吐
cargo test --release throughput_100kbps

# 长时间稳定性
cargo test --release stability_24h
```
