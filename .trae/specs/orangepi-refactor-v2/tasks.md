# OrangePi Debug Tool 重构任务清单

---

## Phase 1: 项目初始化与架构准备 (已完成)

### 1.1 项目结构创建 ✅
- [x] 创建 src-tauri/src/adapters/ 目录
- [x] 创建 src-tauri/src/services/ 目录
- [x] 创建 src-tauri/src/commands/ 目录
- [x] 创建 src-tauri/src/infrastructure/ 目录

### 1.2 依赖配置 ✅
- [x] 配置 Cargo.toml 依赖 (tokio, serialport, parking_lot)
- [x] 配置前端依赖 (zustand, tailwindcss)

---

## Phase 2: 核心功能重构

### 2.1 设备适配器层实现

#### 2.1.1 串口适配器
- [ ] **Task 2.1.1.1**: 创建 `adapters/serial/mod.rs` - 定义 SerialAdapter trait
- [ ] **Task 2.1.1.2**: 创建 `adapters/serial/native.rs` - 实现 native serialport 适配器
- [ ] **Task 2.1.1.3**: 实现环形缓冲区 `CircularBuffer`
- [ ] **Task 2.1.1.4**: 实现批量读取优化 `read_batch()`
- [ ] **Task 2.1.1.5**: 实现连接超时与自动重连

#### 2.1.2 网络适配器
- [ ] **Task 2.1.2.1**: 创建 `adapters/network/mod.rs` - 定义 NetworkAdapter trait
- [ ] **Task 2.1.2.2**: 创建 `adapters/network/tcp.rs` - 实现 TCP 服务器/客户端
- [ ] **Task 2.1.2.3**: 创建 `adapters/network/udp.rs` - 实现 UDP 通信
- [ ] **Task 2.1.2.4**: 实现连接池管理 `ConnectionPool`

#### 2.1.3 Mock 适配器 (测试用)
- [ ] **Task 2.1.3.1**: 创建 `adapters/mock/mod.rs` - 实现 MockSerialAdapter
- [ ] **Task 2.1.3.2**: 实现可控延迟和模拟数据生成

### 2.2 服务层实现

#### 2.2.1 SerialService
- [ ] **Task 2.2.1.1**: 创建 `services/serial_service.rs`
- [ ] **Task 2.2.1.2**: 实现 `send_with_retry()` 带重试机制
- [ ] **Task 2.2.1.3**: 实现定时循环发送 `start_timing_mode()`
- [ ] **Task 2.2.1.4**: 实现 HEX/ASCII 格式转换
- [ ] **Task 2.2.1.5**: 实现数据统计收集

#### 2.2.2 NetworkService
- [ ] **Task 2.2.2.1**: 创建 `services/network_service.rs`
- [ ] **Task 2.2.2.2**: 实现 TCP 服务器模式
- [ ] **Task 2.2.2.3**: 实现 TCP 客户端模式
- [ ] **Task 2.2.2.4**: 实现 UDP 广播/单播
- [ ] **Task 2.2.2.5**: 实现连接状态管理

#### 2.2.3 LogService
- [ ] **Task 2.2.3.1**: 创建 `services/log_service.rs`
- [ ] **Task 2.2.3.2**: 实现异步日志写入
- [ ] **Task 2.2.3.3**: 实现日志轮转 `LogRotation`
- [ ] **Task 2.2.3.4**: 实现日志压缩

### 2.3 命令层实现

#### 2.3.1 串口命令
- [ ] **Task 2.3.1.1**: 实现 `list_serial_ports` 命令
- [ ] **Task 2.3.1.2**: 实现 `connect_serial` 命令
- [ ] **Task 2.3.1.3**: 实现 `disconnect_serial` 命令
- [ ] **Task 2.3.1.4**: 实现 `send_serial_data` 命令
- [ ] **Task 2.3.1.5**: 实现 `start_timing_send` 命令

#### 2.3.2 网络命令
- [ ] **Task 2.3.2.1**: 实现 `create_tcp_server` 命令
- [ ] **Task 2.3.2.2**: 实现 `connect_tcp` 命令
- [ ] **Task 2.3.2.3**: 实现 `connect_udp` 命令
- [ ] **Task 2.3.2.4**: 实现 `send_network_data` 命令
- [ ] **Task 2.3.2.5**: 实现 `close_connection` 命令

#### 2.3.3 日志命令
- [ ] **Task 2.3.3.1**: 实现 `save_log` 命令
- [ ] **Task 2.3.3.2**: 实现 `export_log` 命令
- [ ] **Task 2.3.3.3**: 实现 `get_statistics` 命令

### 2.4 基础设施层

#### 2.4.1 错误处理
- [ ] **Task 2.4.1.1**: 创建 `infrastructure/error.rs`
- [ ] **Task 2.4.1.2**: 定义统一错误枚举 `AppError`
- [ ] **Task 2.4.1.3**: 实现错误码映射

#### 2.4.2 指标采集
- [ ] **Task 2.4.2.1**: 创建 `infrastructure/metrics.rs`
- [ ] **Task 2.4.2.2**: 实现 Counter/Gauge/Histogram
- [ ] **Task 2.4.2.3**: 集成 Prometheus 导出

#### 2.4.3 健康检查
- [ ] **Task 2.4.3.1**: 创建 `infrastructure/health.rs`
- [ ] **Task 2.4.3.2**: 实现健康检查端点

---

## Phase 3: 测试与完善

### 3.1 单元测试
- [ ] **Task 3.1.1**: 编写 SerialAdapter 单元测试
- [ ] **Task 3.1.2**: 编写 NetworkAdapter 单元测试
- [ ] **Task 3.1.3**: 编写 CircularBuffer 单元测试
- [ ] **Task 3.1.4**: 编写 HEX/ASCII 转换测试

### 3.2 集成测试
- [ ] **Task 3.2.1**: 编写串口通信集成测试
- [ ] **Task 3.2.2**: 编写 TCP/UDP 集成测试
- [ ] **Task 3.2.3**: 编写日志写入集成测试

### 3.3 性能测试
- [ ] **Task 3.3.1**: 编写高波特率吞吐测试
- [ ] **Task 3.3.2**: 编写长时间稳定性测试

### 3.4 文档完善
- [ ] **Task 3.4.1**: 更新 API 文档
- [ ] **Task 3.4.2**: 添加架构图
- [ ] **Task 3.4.3**: 添加故障排查指南

---

## 任务依赖关系

```
Phase 2:
├── 2.1 适配器层 (前置依赖)
│   ├── 2.1.1 串口适配器
│   ├── 2.1.2 网络适配器
│   └── 2.1.3 Mock适配器
├── 2.2 服务层 (依赖 2.1)
│   ├── 2.2.1 SerialService
│   ├── 2.2.2 NetworkService
│   └── 2.2.3 LogService
├── 2.3 命令层 (依赖 2.2)
│   ├── 2.3.1 串口命令
│   ├── 2.3.2 网络命令
│   └── 2.3.3 日志命令
└── 2.4 基础设施 (并行)
    ├── 2.4.1 错误处理
    ├── 2.4.2 指标采集
    └── 2.4.3 健康检查

Phase 3: (依赖 Phase 2 全部完成)
├── 3.1 单元测试
├── 3.2 集成测试
├── 3.3 性能测试
└── 3.4 文档完善
```

---

## 验收标准

| 阶段 | 完成标准 |
|------|----------|
| Phase 2 | 所有命令可正常调用，串口/网络通信正常 |
| Phase 3 | 测试覆盖率 > 70%，文档完整 |
| 性能 | 115200 波特率无丢包，UI 帧率 > 30fps |
| 稳定性 | 24小时连续运行无崩溃 |
