# OrangePi Debug Tool 重构检查清单

---

## Phase 2: 核心功能重构检查点

### 2.1 设备适配器层检查点

#### 串口适配器检查点
- [ ] `adapters/serial/mod.rs` 实现了 SerialAdapter trait
- [ ] `adapters/serial/native.rs` 实现了跨平台串口通信
- [ ] `CircularBuffer` 环形缓冲区实现正确
- [ ] `read_batch()` 批量读取无数据丢失
- [ ] 连接超时机制正常工作
- [ ] 自动重连逻辑正确实现

#### 网络适配器检查点
- [ ] `adapters/network/mod.rs` 实现了 NetworkAdapter trait
- [ ] TCP 服务器可正常监听和接受连接
- [ ] TCP 客户端可正常连接和断开
- [ ] UDP 单播正常工作
- [ ] UDP 广播正常工作
- [ ] 连接池管理正常

#### Mock 适配器检查点
- [ ] `adapters/mock/mod.rs` 实现了 MockSerialAdapter
- [ ] 可配置模拟延迟
- [ ] 可配置模拟数据

### 2.2 服务层检查点

#### SerialService 检查点
- [ ] `services/serial_service.rs` 实现了 SerialService
- [ ] `send_with_retry()` 重试机制正常
- [ ] 定时循环发送正常工作
- [ ] HEX 格式转换正确 (0xFF -> "FF")
- [ ] ASCII 格式转换正确
- [ ] 数据统计收集正常

#### NetworkService 检查点
- [ ] `services/network_service.rs` 实现了 NetworkService
- [ ] TCP 服务器可创建和关闭
- [ ] TCP 客户端可连接和断开
- [ ] UDP 会话管理正常
- [ ] 连接状态正确同步

#### LogService 检查点
- [ ] `services/log_service.rs` 实现了 LogService
- [ ] 日志异步写入不阻塞
- [ ] 日志轮转正常
- [ ] 日志可正常导出

### 2.3 命令层检查点

#### 串口命令检查点
- [ ] `list_serial_ports` 返回可用串口列表
- [ ] `connect_serial` 可成功建立连接
- [ ] `disconnect_serial` 可成功断开连接
- [ ] `send_serial_data` 可发送数据
- [ ] `start_timing_send` 定时发送正常

#### 网络命令检查点
- [ ] `create_tcp_server` 可创建 TCP 服务器
- [ ] `connect_tcp` 可建立 TCP 连接
- [ ] `connect_udp` 可建立 UDP 会话
- [ ] `send_network_data` 可发送数据
- [ ] `close_connection` 可关闭连接

#### 日志命令检查点
- [ ] `save_log` 可保存日志
- [ ] `export_log` 可导出日志
- [ ] `get_statistics` 返回统计数据

### 2.4 基础设施层检查点

#### 错误处理检查点
- [ ] `infrastructure/error.rs` 定义了统一错误类型
- [ ] 错误码完整覆盖所有场景
- [ ] 错误信息可正确返回给前端

#### 指标采集检查点
- [ ] `infrastructure/metrics.rs` 实现了指标收集
- [ ] Counter 计数正常
- [ ] Gauge 数值正常
- [ ] Histogram 分布正常

#### 健康检查检查点
- [ ] `infrastructure/health.rs` 实现了健康检查
- [ ] 可检测服务状态

---

## Phase 3: 测试与完善检查点

### 3.1 单元测试检查点
- [ ] SerialAdapter 单元测试通过
- [ ] NetworkAdapter 单元测试通过
- [ ] CircularBuffer 单元测试通过
- [ ] HEX/ASCII 转换测试通过

### 3.2 集成测试检查点
- [ ] 串口通信集成测试通过
- [ ] TCP 通信集成测试通过
- [ ] UDP 通信集成测试通过
- [ ] 日志写入集成测试通过

### 3.3 性能测试检查点
- [ ] 115200 波特率无丢包
- [ ] 921600 波特率无丢包
- [ ] 1000 并发连接无崩溃
- [ ] 24小时稳定性测试通过

### 3.4 文档检查点
- [ ] API 文档完整
- [ ] 架构图清晰
- [ ] 故障排查指南完整

---

## 编译检查点

- [ ] `cargo build` 成功 (debug)
- [ ] `cargo build --release` 成功
- [ ] `cargo clippy` 无警告
- [ ] `cargo fmt` 格式化正确
- [ ] 前端 `npm run build` 成功

---

## 跨平台检查点

### Windows
- [ ] Windows 10/11 可编译
- [ ] 串口通信正常
- [ ] 可执行文件运行正常

### Linux ARM (OrangePi)
- [ ] ARM 交叉编译成功
- [ ] 串口通信正常 (ttyS*)
- [ ] TCP/UDP 通信正常

---

## 回归检查点

- [ ] 原有串口参数设置不变
- [ ] TCP/UDP 收发格式不变
- [ ] HEX/ASCII 模式不变
- [ ] 定时循环发送功能不变
- [ ] 日志保存功能不变
- [ ] 数据统计功能不变
- [ ] UI 布局不变
- [ ] 快捷键不变

---

## 最终验收

- [ ] 所有 Phase 2 检查点通过
- [ ] 所有 Phase 3 检查点通过
- [ ] 所有编译检查点通过
- [ ] 所有跨平台检查点通过
- [ ] 所有回归检查点通过
- [ ] 代码审查通过
- [ ] 性能指标达标
- [ ] 稳定性测试通过
