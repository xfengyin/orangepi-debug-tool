# OrangePi Zero3 调试工具重构任务清单

## Phase 1: 架构重构（第 1-2 周）✅

### 1.1 设备适配器层实现 (P0) ✅

- [x] **1.1.1 定义设备适配器 Trait**
  - [x] 创建 `src-tauri/src/adapters/mod.rs` 模块
  - [x] 定义 `DeviceAdapter` 基础 trait
  - [x] 定义 `SerialAdapter` trait
  - [x] 定义 `GpioAdapter` trait
  - [x] 定义 `PwmAdapter` trait
  - [x] 实现默认方法

- [x] **1.1.2 实现 OrangePi Zero3 专用适配器**
  - [x] 创建 `src-tauri/src/adapters/orangepi_zero3.rs`
  - [x] 实现 SerialAdapter for OrangePiZero3
  - [x] 实现 GpioAdapter for OrangePiZero3（基于 gpio-cdev）
  - [x] 实现 PwmAdapter for OrangePiZero3
  - [x] 添加板级 pin 定义配置

- [x] **1.1.3 实现 Generic Linux 适配器**
  - [x] 创建 `src-tauri/src/adapters/generic_linux.rs`
  - [x] 实现 SerialAdapter for GenericLinux
  - [x] 实现 GpioAdapter for GenericLinux（sysfs fallback）
  - [x] 实现 PwmAdapter for GenericLinux

- [x] **1.1.4 实现 Mock 适配器（测试用）**
  - [x] 创建 `src-tauri/src/adapters/mock.rs`
  - [x] 实现所有 trait 的 mock 版本
  - [x] 支持可控的模拟数据生成
  - [x] 支持延迟模拟

- [x] **1.1.5 实现适配器注册表**
  - [x] 创建 `DeviceAdapterRegistry` 结构体
  - [x] 实现适配器注册/注销
  - [x] 实现适配器查询接口
  - [x] 实现自动检测逻辑
  - [x] 实现默认适配器选择策略

### 1.2 分层状态架构重构 (P0) ✅

- [x] **1.2.1 创建配置状态层**
  - [x] 创建 `src-tauri/src/state/config_store.rs`
  - [x] 定义 ConfigStore 结构体
  - [x] 实现配置加载/保存
  - [x] 实现配置验证
  - [x] 实现配置变更订阅

- [x] **1.2.2 创建设备状态层**
  - [x] 创建 `src-tauri/src/state/device_store.rs`
  - [x] 定义 DeviceStore 结构体
  - [x] 实现设备状态管理
  - [x] 实现设备事件发布
  - [x] 实现设备状态持久化

- [x] **1.2.3 创建会话状态层**
  - [x] 创建 `src-tauri/src/state/session_store.rs`
  - [x] 定义 SessionStore 结构体
  - [x] 实现会话追踪
  - [x] 实现操作历史记录
  - [x] 实现撤销/重做支持

- [x] **1.2.4 重构 AppState**
  - [x] 更新 `src-tauri/src/state.rs`
  - [x] 整合三个状态层
  - [x] 实现状态协调器
  - [x] 保留向后兼容性

### 1.3 配置驱动引擎实现 (P1) ✅

- [x] **1.3.1 定义配置 Schema**
  - [x] 创建 `src-tauri/src/config/schema.rs`
  - [x] 定义 AppConfiguration 结构体
  - [x] 定义各子模块配置结构体
  - [x] 添加 serde Deserialize 实现

- [x] **1.3.2 实现配置加载器**
  - [x] 创建 `src-tauri/src/config/loader.rs`
  - [x] 支持 YAML/JSON/TOML 格式
  - [x] 实现配置合并策略
  - [x] 实现默认值填充

- [x] **1.3.3 实现配置验证器**
  - [x] 创建 `src-tauri/src/config/validator.rs`
  - [x] 实现必填字段检查
  - [x] 实现数值范围验证
  - [x] 实现枚举值验证

- [x] **1.3.4 实现配置热重载**
  - [x] 创建 `src-tauri/src/config/hot_reload.rs`
  - [x] 实现文件监听
  - [x] 实现增量加载
  - [x] 实现回滚机制

### 1.4 可观测性层实现 (P1) ✅

- [x] **1.4.1 实现 MetricsCollector**
  - [x] 创建 `src-tauri/src/observability/metrics.rs`
  - [x] 实现 Counter/Gauge/Histogram
  - [x] 实现标签支持
  - [x] 实现 Prometheus 导出

- [x] **1.4.2 实现 Tracing 集成**
  - [x] 创建 `src-tauri/src/observability/tracing.rs`
  - [x] 实现 trace_id 生成
  - [x] 实现 span 管理
  - [x] 实现日志关联

- [x] **1.4.3 实现 HealthChecker**
  - [x] 创建 `src-tauri/src/observability/health.rs`
  - [x] 实现 HealthCheck trait
  - [x] 实现健康检查端点
  - [x] 实现 K8s probes 兼容

---

## Phase 2: 核心功能重构（第 3-4 周）⏳

### 2.1 SerialManager 重构 (P0) ⏳

- [ ] **2.1.1 解耦 SerialManager**
  - [ ] 保留 SerialAdapter trait 定义
  - [ ] 创建 SerialService 业务层
  - [ ] 移除直接设备操作逻辑
  - [ ] 实现命令路由

- [ ] **2.1.2 实现异步批处理**
  - [ ] 实现批量读取优化
  - [ ] 实现批量写入优化
  - [ ] 实现并行端口检测

- [ ] **2.1.3 添加连接池管理**
  - [ ] 实现 SerialConnectionPool
  - [ ] 实现连接复用
  - [ ] 实现连接健康检查

- [ ] **2.1.4 实现自动重连机制**
  - [ ] 实现断连检测
  - [ ] 实现指数退避重连
  - [ ] 实现重连成功回调

- [ ] **2.1.5 添加性能指标采集**
  - [ ] 添加吞吐量计数
  - [ ] 添加延迟直方图
  - [ ] 添加错误率统计

### 2.2 GpioManager 重构 (P0) ⏳

- [ ] **2.2.1 解耦 GpioManager**
  - [ ] 保留 GpioAdapter trait 定义
  - [ ] 创建 GpioService 业务层
  - [ ] 实现引脚状态缓存

- [ ] **2.2.2 实现批量引脚操作**
  - [ ] 实现 batch_export
  - [ ] 实现 batch_configure
  - [ ] 实现 batch_read
  - [ ] 实现 batch_write

- [ ] **2.2.3 添加中断边沿检测**
  - [ ] 实现 epoll 事件监听
  - [ ] 实现边沿检测回调
  - [ ] 实现中断去抖

- [ ] **2.2.4 添加性能指标采集**
  - [ ] 添加读写计数
  - [ ] 添加中断计数
  - [ ] 添加延迟统计

### 2.3 PwmManager 重构 (P1) ⏳

- [ ] **2.3.1 解耦 PwmManager**
  - [ ] 保留 PwmAdapter trait 定义
  - [ ] 创建 PwmService 业务层
  - [ ] 实现通道状态管理

- [ ] **2.3.2 实现多通道管理**
  - [ ] 实现通道资源分配
  - [ ] 实现通道冲突检测
  - [ ] 实现通道优先级

- [ ] **2.3.3 实现精确频率控制**
  - [ ] 实现频率计算算法
  - [ ] 实现分数分频器配置
  - [ ] 实现频率抖动抑制

- [ ] **2.3.4 实现占空比平滑调整**
  - [ ] 实现渐变占空比
  - [ ] 实现步进配置
  - [ ] 实现平滑曲线

---

## Phase 3: 完善与测试（第 5-6 周）⏳

### 3.1 统一错误处理系统 (P1) ⏳

- [ ] **3.1.1 扩展 AppError 枚举**
  - [ ] 添加设备相关错误
  - [ ] 添加配置相关错误
  - [ ] 添加安全相关错误

- [ ] **3.1.2 实现错误码体系**
  - [ ] 定义错误码规范
  - [ ] 实现错误码映射
  - [ ] 实现错误码文档生成

- [ ] **3.1.3 实现错误恢复策略**
  - [ ] 实现自动重试
  - [ ] 实现降级处理
  - [ ] 实现熔断机制

### 3.2 安全增强 (P2) ⏳

- [ ] **3.2.1 实现操作审计日志**
  - [ ] 记录所有命令调用
  - [ ] 记录操作人/时间
  - [ ] 记录操作结果

- [ ] **3.2.2 添加危险操作确认**
  - [ ] 定义危险操作列表
  - [ ] 实现二次确认机制
  - [ ] 实现操作超时

- [ ] **3.2.3 实现敏感数据脱敏**
  - [ ] 识别敏感字段
  - [ ] 实现脱敏规则
  - [ ] 应用于日志输出

### 3.3 测试完善 (P2) ⏳

- [ ] **3.3.1 补充单元测试**
  - [ ] 补充 SerialManager 测试
  - [ ] 补充 GpioManager 测试
  - [ ] 补充 PwmManager 测试
  - [ ] 补充 ConfigManager 测试

- [ ] **3.3.2 添加集成测试**
  - [ ] 创建 `tests/integration/` 目录
  - [ ] 添加设备通信测试
  - [ ] 添加配置加载测试
  - [ ] 添加热重载测试

- [ ] **3.3.3 实现 Mock 模式**
  - [ ] 实现 --mock 启动参数
  - [ ] 实现环境变量配置
  - [ ] 实现测试 fixture

### 3.4 文档完善 (P2) ⏳

- [ ] **3.4.1 更新 API 文档**
  - [ ] 生成 OpenAPI 规范
  - [ ] 添加请求/响应示例
  - [ ] 添加错误码说明

- [ ] **3.4.2 补充架构文档**
  - [ ] 补充模块依赖图
  - [ ] 补充序列图
  - [ ] 补充决策记录

- [ ] **3.4.3 添加故障排查指南**
  - [ ] 常见问题 FAQ
  - [ ] 错误码速查表
  - [ ] 日志分析指南

---

## 里程碑

| 里程碑 | 完成标准 | 状态 |
|--------|----------|------|
| M1: 适配器架构完成 | 三个适配器可正常工作 | ✅ 完成 |
| M2: 状态架构完成 | 配置/设备/会话分离 | ✅ 完成 |
| M3: 核心功能重构 | 串口/GPIO/PWM 正常工作 | ⏳ 进行中 |
| M4: 可观测性完成 | metrics/traces/health 可用 | ✅ 完成 |
| M5: 测试完善 | 测试覆盖率 > 70% | ⏳ 待开始 |
| M6: 发布准备 | 文档完整，可发布 | ⏳ 待开始 |
