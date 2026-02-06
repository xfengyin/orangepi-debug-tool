# OrangePi 上位机调试工具 - 详细使用指南

## 安装和配置

### 系统要求

- 操作系统：Windows 7+, macOS 10.12+, Linux (Ubuntu 16.04+, Debian 9+)
- Python 3.6 或更高版本
- 至少 100MB 可用磁盘空间
- USB 端口用于连接目标设备

### 安装依赖

```bash
pip install -r requirements.txt
```

如果出现权限问题，在 Linux/macOS 上使用：

```bash
pip install --user -r requirements.txt
```

## 硬件连接

### 连接 OrangePi

1. 使用 USB 转 TTL 模块连接 OrangePi 和电脑
2. 确保正确连接 TX/RX 线（交叉连接：TX->RX, RX->TX）
3. 连接 GND 线
4. 供电 OrangePi（可通过 USB 或外部电源）

### 波特率设置

不同 OrangePi 型号的默认串口波特率可能不同：

- OrangePi Zero: 115200
- OrangePi PC: 115200
- OrangePi Plus: 115200
- OrangePi 2: 115200
- OrangePi One: 115200

## 软件使用

### 启动软件

```bash
python orange_debug_tool.py
```

或

```bash
python src/main.py
```

### 连接设备

1. 在软件界面中点击"刷新端口"
2. 从下拉菜单中选择正确的串口号（通常是 COM3+ 在 Windows 或 /dev/ttyUSB0+/dev/ttyACM0+ 在 Linux）
3. 设置正确的波特率（默认为 115200）
4. 点击"连接"按钮

连接成功后，状态栏会显示连接信息。

### 数据收发

#### 文本模式

- 在发送区输入文本内容
- 确保"十六进制发送"未勾选
- 点击"发送"按钮发送数据
- 接收区会显示从设备返回的数据

#### 十六进制模式

- 在发送区输入十六进制数值，用空格分隔（例如：AA BB CC）
- 勾选"十六进制发送"
- 点击"发送"按钮发送数据
- 如需十六进制显示接收数据，勾选"十六进制显示"

### GPIO 控制

1. 在 GPIO 编号框中输入要控制的 GPIO 引脚号
2. 选择 HIGH 或 LOW 状态
3. 点击"设置GPIO"按钮

软件会发送类似 `GPIO,12,HIGH` 的命令到 OrangePi。

## 常见问题

### 无法识别串口

1. 检查 USB 连接是否牢固
2. 确认已安装 USB 转 TTL 模块的驱动程序
3. 尝试手动输入串口号
4. 在 Linux 上，可能需要添加用户到 dialout 组：
   ```bash
   sudo usermod -a -G dialout $USER
   ```
   然后重新登录

### 连接后无响应

1. 确认波特率设置正确
2. 检查 TX/RX 线是否正确连接
3. 确认目标设备正在运行并能够响应

### 权限错误

在 Linux 上，如果出现权限错误：

```bash
sudo chmod 666 /dev/ttyUSB0  # 替换为实际串口号
```

或永久添加用户到相应组：

```bash
sudo usermod -a -G dialout $USER
```

## 高级功能

### 自定义命令

可以直接在发送区输入自定义命令，例如：

- 重启命令：`reboot\n`
- 查看系统信息：`uname -a\n`
- 查看 CPU 信息：`cat /proc/cpuinfo\n`

### 批量发送

可以发送多行命令，每行作为一个独立命令发送。

## 故障排除

### 串口列表为空

- 检查硬件连接
- 确认驱动程序已安装
- 在终端中使用 `ls /dev/tty*` (Linux) 或 设 Gerätemanager (Windows) 检查设备

### 发送数据后无响应

- 检查命令格式
- 确认目标设备支持该命令
- 尝试降低波特率

### GUI 响应缓慢

- 关闭十六进制显示（如果不需要）
- 减少接收数据量
- 重启软件

## 开发者说明

### 代码结构

- `src/main.py`: 主应用程序代码
- `config.ini`: 配置文件
- `tests/test_basic.py`: 基本测试
- `orange_debug_tool.py`: 启动脚本

### 扩展功能

可以通过修改 `main.py` 中的 `set_gpio` 方法来添加更多 GPIO 控制功能。

## 更新日志

### 1.0.0
- 初始版本
- 基础串口通信功能
- GPIO 控制功能
- 十六进制收发模式
- 用户友好的图形界面