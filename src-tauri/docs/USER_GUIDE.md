# OrangePi Zero3 调试工具用户指南

## 目录

1. [快速开始](#快速开始)
2. [界面介绍](#界面介绍)
3. [串口调试](#串口调试)
4. [GPIO 控制](#gpio-控制)
5. [PWM 输出](#pwm-输出)
6. [配置管理](#配置管理)
7. [故障排查](#故障排查)

---

## 快速开始

### 系统要求

- OrangePi Zero3 开发板 (H3 芯片)
- 或其他 Linux 系统（桌面测试模式）
- 8GB+ 存储空间
- USB 串口调试线（可选）

### 安装

#### 从源码构建

```bash
# 克隆项目
git clone https://github.com/your-repo/orangepi-debug-tool.git
cd orangepi-debug-tool

# 安装依赖 (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev

# 构建应用
cargo build --release

# 运行
./target/release/orangepi-debug-tool
```

#### 使用 Docker（桌面测试）

```bash
# 构建 Docker 镜像
docker build -t orangepi-debug-tool .

# 运行（需要 X11 转发）
docker run -e DISPLAY=$DISPLAY -v /tmp/.X11-unix:/tmp/.X11-unix orangepi-debug-tool
```

### 首次运行

1. 启动应用后，主界面将显示四个主要功能模块
2. 系统会自动检测连接的硬件设备
3. 左下角显示当前设备状态

---

## 界面介绍

### 主界面布局

```
┌─────────────────────────────────────────────────────────────┐
│  OrangePi Zero3 调试工具                    [版本 2.0.0]    │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐          │
│  │  串口   │ │  GPIO   │ │  PWM    │ │  日志   │          │
│  │  Serial │ │         │ │         │ │  Log   │          │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│                      功能区域                                │
│                   (根据选中模块变化)                          │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│  状态: 就绪    │  连接设备: /dev/ttyUSB0    │  内存: 45MB  │
└─────────────────────────────────────────────────────────────┘
```

### 状态栏说明

| 状态 | 颜色 | 含义 |
|------|------|------|
| 就绪 | 绿色 | 系统正常，可进行操作 |
| 忙碌 | 黄色 | 正在执行操作 |
| 错误 | 红色 | 发生错误，需要检查 |
| 离线 | 灰色 | 未检测到硬件 |

---

## 串口调试

### 连接串口

1. 点击 **串口** 模块
2. 从下拉列表中选择串口设备
3. 设置波特率（常用：9600, 115200）
4. 点击 **连接** 按钮

### 常用波特率

| 应用场景 | 推荐波特率 |
|----------|------------|
| 调试输出 | 115200 |
| GPS 模块 | 9600 |
| 蓝牙模块 | 9600 / 38400 |
| 工业设备 | 57600 / 115200 |

### 发送数据

1. 在发送输入框中输入文本
2. 选择编码格式（UTF-8 / ASCII / Hex）
3. 点击 **发送** 按钮
4. 发送记录显示在下方

### 接收数据

1. 自动接收模式默认开启
2. 收到的数据实时显示在接收区
3. 可勾选 **Hex 显示** 以十六进制查看
4. 点击 **清空** 按钮清除接收区

### 数据保存

- 点击 **保存日志** 将通信记录保存为文件
- 支持格式：`.txt`, `.log`, `.csv`
- 自动命名为 `serial_YYYYMMDD_HHMMSS.log`

---

## GPIO 控制

### 引脚布局

OrangePi Zero3 40Pin GPIO 引脚定义：

```
┌─────────────────────────────┐
│  3.3V │ 5V    │  1  │  2  │
│  PA12 │ 5V    │  3  │  4  │
│  PA11 │ GND   │  5  │  6  │
│  PA6  │ TXD   │  7  │  8  │
│  GND  │ RXD   │  9  │ 10  │
│  PA1  │ PA7   │ 11  │ 12  │
│  PA0  │ GND   │ 13  │ 14  │
│  PA3  │ PA15  │ 15  │ 16  │
│  3.3V │ PA16  │ 17  │ 18  │
│  PA14 │ GND   │ 19  │ 20  │
│  PA13 │ PA10  │ 21  │ 22  │
│  PA2  │ PA8   │ 23  │ 24  │
│  GND  │ PA9   │ 25  │ 26  │
│  PA19 │ PA18  │ 27  │ 28  │
│  PA21 │ GND   │ 29  │ 30  │
│  PA20 │ PA22   │ 31  │ 32  │
│  3.3V │ GND   │ 33  │ 34  │
│  PA10 │ PA9   │ 35  │ 36  │
│  PA19 │ GND   │ 37  │ 38  │
│  PA18 │ PA22   │ 39  │ 40  │
└─────────────────────────────┘
```

### 基本操作

#### 导出引脚

1. 在引脚列表中选择引脚（如 GPIO3）
2. 点击 **导出** 按钮
3. 引脚状态变为已导出

#### 设置方向

1. 选择已导出的引脚
2. 选择方向：**输入** 或 **输出**
3. 点击 **应用** 按钮

#### 读取值

1. 确保引脚方向为 **输入**
2. 点击 **读取** 按钮
3. 当前引脚状态显示在界面上

#### 写入值

1. 确保引脚方向为 **输出**
2. 点击 **高(1)** 或 **低(0)** 按钮
3. LED 或继电器响应输出

### 批量操作

1. 勾选多个引脚
2. 选择批量操作类型
3. 点击 **执行** 按钮
4. 查看执行结果

### 中断配置

1. 选择引脚
2. 选择触发方式：
   - **上升沿**：0→1
   - **下降沿**：1→0
   - **双边沿**：任意变化
3. 点击 **启用中断**
4. 引脚状态变化时自动记录

---

## PWM 输出

### PWM 通道

OrangePi Zero3 提供 2 个 PWM 通道：

| 通道 | 引脚 | 频率范围 |
|------|------|----------|
| PWM0 | GPIO 12 (PA7) | 0-24 MHz |
| PWM1 | GPIO 33 (PA10) | 0-24 MHz |

### 基本操作

#### 启用通道

1. 选择 PWM 通道（0 或 1）
2. 点击 **启用** 按钮
3. 通道状态显示为已启用

#### 设置频率

1. 在频率输入框输入数值（如 1000 表示 1kHz）
2. 点击 **设置频率**
3. 波形周期相应变化

#### 设置占空比

1. 拖动滑块或直接输入数值（0-100%）
2. 点击 **设置占空比**
3. LED 亮度或电机速度相应变化

### 预设模式

#### 伺服电机控制

1. 选择通道
2. 点击 **伺服模式**
3. 输入角度（0-180°）
4. 点击 **设置角度**

#### LED 调光

1. 选择通道
2. 输入亮度（0-100%）
3. LED 亮度平滑变化

### 效果

#### 呼吸灯

1. 设置基础频率（1000 Hz）
2. 选择效果：**呼吸灯**
3. 设置周期（如 2000ms）
4. 点击 **开始**

#### 渐变

1. 设置目标占空比
2. 设置持续时间
3. 点击 **淡入** 或 **淡出**

---

## 配置管理

### 配置文件

配置文件位于 `~/.config/orangepi-debug-tool/config.yaml`

```yaml
meta:
  version: "2.0.0"
  environment: production

system:
  log_level: info
  max_concurrent_tasks: 100
  task_timeout_seconds: 300

devices:
  serial:
    default_adapter: orangepi_zero3
    supported_baudrates:
      - 9600
      - 115200
      - 57600
    buffer_size: 65536

  gpio:
    default_adapter: orangepi_zero3
    interrupt_debounce_ms: 50

  pwm:
    default_adapter: orangepi_zero3
    default_frequency_hz: 1000
    default_duty_cycle: 50.0

security:
  enable_audit_log: true
  dangerous_operations_require_confirmation: true

observability:
  enable_metrics: true
  enable_tracing: true
  metrics_export_interval_seconds: 60
```

### 热重载

配置文件修改后自动生效，无需重启应用。

### 配置导出/导入

- **导出**：点击菜单 → 配置 → 导出配置
- **导入**：点击菜单 → 配置 → 导入配置

---

## 故障排查

### 常见问题

#### Q1: 找不到串口设备

**原因**：
- 串口设备未连接
- 当前用户没有串口访问权限

**解决方法**：

```bash
# 检查串口设备
ls -l /dev/ttyUSB*

# 添加用户到 dialout 组
sudo usermod -a -G dialout $USER
# 重新登录后生效
```

#### Q2: GPIO 操作失败

**原因**：
- 引脚已被其他程序占用
- 引脚编号错误

**解决方法**：

```bash
# 检查引脚占用
cat /sys/class/gpio/gpio{N}/direction

# 或者使用 gpioinfo (如果安装了 gpio-utils)
gpioinfo
```

#### Q3: PWM 不输出

**原因**：
- PWM 通道未启用
- 频率设置超出范围

**解决方法**：
1. 确保先点击 **启用** 按钮
2. 检查频率是否在 0-24MHz 范围内
3. 确认引脚复用配置正确

#### Q4: 应用启动失败

**原因**：
- 缺少 GTK 依赖
- 配置文件损坏

**解决方法**：

```bash
# 重新安装依赖
sudo apt-get install --reinstall libgtk-3-dev libwebkit2gtk-4.0-dev

# 删除损坏的配置文件
rm ~/.config/orangepi-debug-tool/config.yaml
# 应用将使用默认配置重新启动
```

### 日志分析

应用日志位于 `~/.local/share/orangepi-debug-tool/logs/`

```bash
# 查看最近的错误日志
tail -n 100 ~/.local/share/orangepi-debug-tool/logs/app.log

# 搜索特定错误
grep -i "error" ~/.local/share/orangepi-debug-tool/logs/app.log

# 启用调试模式
RUST_LOG=debug ./orangepi-debug-tool
```

### 性能问题

#### 内存占用过高

```bash
# 检查内存使用
top -p $(pidof orangepi-debug-tool)

# 减少日志级别
# 编辑配置文件：
# system.log_level: warn
```

#### 响应缓慢

1. 关闭不必要的监控功能
2. 减少日志输出
3. 检查串口缓冲区设置

---

## 技术支持

- GitHub Issues: https://github.com/your-repo/orangepi-debug-tool/issues
- 文档: https://docs.example.com/orangepi-debug-tool
- 社区论坛: https://forum.example.com
