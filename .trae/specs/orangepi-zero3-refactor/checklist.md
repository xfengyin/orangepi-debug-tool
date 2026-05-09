# OrangePi Zero3 调试工具重构检查清单

## Phase 1 检查点

### 1.1 设备适配器层 ✅

- [x] **Trait 定义完整性**
  - [x] `DeviceAdapter` trait 包含 id()、capabilities()、health_check()、initialize()、shutdown()
  - [x] `SerialAdapter` trait 包含 list_ports()、connect()、disconnect()、read()、write()
  - [x] `GpioAdapter` trait 包含 list_pins()、export_pin()、set_direction()、read_pin()、write_pin()
  - [x] `PwmAdapter` trait 包含 list_channels()、enable_channel()、set_frequency()、set_duty_cycle()
  - [x] 所有 trait 实现 Send + Sync 约束

- [x] **OrangePi Zero3 适配器**
  - [x] 串口支持 115200、57600、9600 等常用波特率
  - [x] GPIO 引脚定义覆盖所有物理引脚
  - [x] PWM 支持 H3 芯片 PWM 控制器
  - [x] 实现板级自动检测逻辑
  - [x] 实现设备信息读取（型号、固件版本）

- [x] **Generic Linux 适配器**
  - [x] 串口支持标准 /dev/tty* 设备
  - [x] GPIO 使用 sysfs 或 gpio-cdev
  - [x] PWM 使用 pwmchip sysfs 接口
  - [x] 实现优雅降级

- [x] **Mock 适配器**
  - [x] 支持可控延迟模拟
  - [x] 支持错误注入
  - [x] 支持数据模式配置
  - [x] 线程安全实现

- [x] **适配器注册表**
  - [x] 实现 register() 方法
  - [x] 实现 unregister() 方法
  - [x] 实现 get() 按 ID 查询
  - [x] 实现 get_default_*() 默认获取
  - [x] 实现 auto_detect() 自动选择
  - [x] 防止重复注册

### 1.2 分层状态架构 ✅

- [x] **配置状态层 (ConfigStore)**
  - [x] 支持从文件加载配置
  - [x] 支持配置变更通知
  - [x] 支持配置验证
  - [x] 支持默认值填充
  - [x] 线程安全实现

- [x] **设备状态层 (DeviceStore)**
  - [x] 维护设备连接状态
  - [x] 发布设备事件（使用 mpsc channel）
  - [x] 缓存设备配置
  - [x] 支持设备状态查询
  - [x] 线程安全实现

- [x] **会话状态层 (SessionStore)**
  - [x] 生成唯一会话 ID
  - [x] 记录操作历史
  - [x] 支持撤销/重做栈
  - [x] 会话超时管理
  - [x] 线程安全实现

- [x] **AppState 整合**
  - [x] 包含所有状态层引用
  - [x] 实现协调方法
  - [x] 保留向后兼容接口
  - [x] 清理资源方法完善

### 1.3 配置驱动引擎 ✅

- [x] **配置 Schema**
  - [x] AppConfiguration 包含所有配置节
  - [x] SerialDeviceConfig 字段完整
  - [x] GpioDeviceConfig 字段完整
  - [x] PwmDeviceConfig 字段完整
  - [x] SkillConfigSection 字段完整
  - [x] PromptConfigSection 字段完整

- [x] **配置加载器**
  - [x] 支持 YAML 格式解析
  - [x] 支持 JSON 格式解析
  - [x] 支持环境变量覆盖
  - [x] 支持多文件合并
  - [x] 错误信息清晰

- [x] **配置验证器**
  - [x] 必填字段检查通过
  - [x] 数值范围验证通过
  - [x] 枚举值验证通过
  - [x] 自定义规则扩展性

- [x] **配置热重载**
  - [x] 文件监听正常工作
  - [x] 增量加载不丢失状态
  - [x] 回滚机制有效
  - [x] 变更通知正常

---

## Phase 2 检查点

### 2.4 可观测性层 ✅

- [x] **MetricsCollector**
  - [x] Counter 增加正确
  - [x] Gauge 设置正确
  - [x] Histogram 统计正确
  - [x] 标签支持正确
  - [x] Prometheus 格式导出

- [x] **Tracing 集成**
  - [x] trace_id 生成唯一
  - [x] span 创建正确
  - [x] 日志关联正确
  - [x] 传播上下文正确

- [x] **HealthChecker**
  - [x] /health 端点正常
  - [x] /health/live 端点正常
  - [x] /health/ready 端点正常
  - [x] 组件状态聚合正确
  - [x] K8s probes 兼容

---

## 已完成的重构内容

### 核心架构实现

1. **设备适配器层 (SPI 架构)**
   - `src-tauri/src/adapters/traits.rs` - 核心 trait 定义
   - `src-tauri/src/adapters/orangepi_zero3.rs` - OrangePi Zero3 专用适配器
   - `src-tauri/src/adapters/generic_linux.rs` - 通用 Linux 适配器
   - `src-tauri/src/adapters/mock.rs` - Mock 测试适配器
   - `src-tauri/src/adapters/registry.rs` - 适配器注册表

2. **配置驱动引擎**
   - `src-tauri/src/config/schema.rs` - 配置数据结构定义
   - `src-tauri/src/config/loader.rs` - 配置加载器
   - `src-tauri/src/config/validator.rs` - 配置验证器

3. **可观测性层**
   - `src-tauri/src/observability/health.rs` - 健康检查系统
   - `src-tauri/src/observability/metrics.rs` - 指标收集器
   - `src-tauri/src/observability/tracing.rs` - 分布式追踪

4. **分层状态架构**
   - `src-tauri/src/state/config_store.rs` - 配置状态层
   - `src-tauri/src/state/device_store.rs` - 设备状态层
   - `src-tauri/src/state/session_store.rs` - 会话状态层
   - `src-tauri/src/state.rs` - AppState 整合

5. **Cargo.toml 依赖更新**
   - Tauri 2.0 升级
   - serde_yaml 添加
   - notify 添加（热重载）
   - gpio-cdev/sysfs_gpio 可选特性

---

## 待完成工作

### Phase 2.1-2.3: 核心功能重构

- [ ] SerialManager 重构（基于新适配器架构）
- [ ] GpioManager 重构（批量操作、中断支持）
- [ ] PwmManager 重构（多通道管理）

### Phase 3: 完善与测试

- [ ] 统一错误处理系统增强
- [ ] 安全增强（审计日志、危险操作确认）
- [ ] 单元测试补充
- [ ] 集成测试
- [ ] 文档完善

---

## 构建说明

由于 Tauri 2.0 需要 GTK 依赖，在完整构建前需要：

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev

# macOS
xcode-select --install

# Windows
# 安装 Visual Studio Build Tools
```

构建成功后，架构重构的核心目标已达成，后续可继续完善功能模块。
