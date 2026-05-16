# 故障排查指南

本文档提供 OrangePi Debug Tool 常见问题的解决方案。

## 常见问题

### 1. 编译错误

#### 缺少系统依赖

如果编译时提示缺少系统库,需要安装必要的开发依赖:

**Ubuntu/Debian:**

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev build-essential \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

**Ubuntu 24.04+:**

```bash
sudo apt install -y libwebkit2gtk-4.1-dev
```

注意: Ubuntu 24.04+ 使用的是 WebKit 4.1 版本。

**Fedora/RHEL:**

```bash
sudo dnf install webkit2gtk4.1-devel gtk3-devel \
  openssl-devel libudev-devel gcc-c++ make
```

**macOS:**

```bash
xcode-select --install
brew installwebkitgtk
```

#### 交叉编译工具链

交叉编译需要安装对应的工具链:

**ARMv7 (armv7-unknown-linux-gnueabihf):**

```bash
# Debian/Ubuntu
sudo apt install -y gcc-arm-linux-gnueabihf g++-arm-linux-gnueabihf

# 验证安装
arm-linux-gnueabihf-gcc --version
```

**ARM64 (aarch64-unknown-linux-gnu):**

```bash
# Debian/Ubuntu
sudo apt install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu

# 验证安装
aarch64-linux-gnu-gcc --version
```

**Windows (x86_64-pc-windows-gnu):**

```bash
# Debian/Ubuntu
sudo apt install -y mingw-w64

# 验证安装
x86_64-w64-mingw32-gcc --version
```

#### Rust 目标平台

确保已安装所需的 Rust 目标平台:

```bash
# ARMv7
rustup target add armv7-unknown-linux-gnueabihf

# ARM64
rustup target add aarch64-unknown-linux-gnu

# Windows
rustup target add x86_64-pc-windows-gnu

# 查看已安装的目标
rustup target list --installed
```

#### 编译错误排查

如果遇到编译错误,尝试以下步骤:

1. **清理并重新编译:**

```bash
cd src-tauri
cargo clean
cargo build --release
```

2. **更新依赖:**

```bash
cargo update
```

3. **检查 Rust 版本:**

```bash
rustc --version
cargo --version
# 确保 Rust 版本 >= 1.70
```

4. **查看详细错误信息:**

```bash
cargo build --release --verbose 2>&1 | tee build.log
```

### 2. 运行错误

#### 串口权限问题

串口设备需要适当的权限才能访问:

**临时解决方案 (需要 sudo):**

```bash
# 查看串口设备
ls -l /dev/tty*

# 以 sudo 运行程序
sudo ./orangepi-debug-tool
```

**永久解决方案 (推荐):**

将当前用户添加到 dialout 组:

```bash
# Debian/Ubuntu
sudo usermod -a -G dialout $USER

# 验证组成员
groups $USER

# 重新登录使更改生效
# 注销并重新登录后,无需 sudo 即可访问串口
```

#### 找不到串口

如果程序无法识别串口设备:

1. **检查 USB 连接:**

```bash
# 查看 USB 设备
lsusb

# 查看串口设备
ls -l /dev/tty*

# 实时监控设备插拔
udevadm monitor
```

2. **检查内核日志:**

```bash
# 查看最近的设备插入
dmesg | grep tty

# 实时监控
dmesg -w
```

3. **常见串口设备名称:**

- Linux: `/dev/ttyUSB0`, `/dev/ttyACM0`, `/dev/ttyS0`
- Windows: `COM1`, `COM2`, `COM3`

4. **检查设备驱动:**

```bash
# 查看已加载的串口驱动
lsmod | grep usbserial
lsmod | grep cdc_acm

# 手动加载驱动 (需要 sudo)
sudo modprobe usbserial
sudo modprobe cdc_acm
```

#### 串口连接失败

串口连接失败可能的原因:

1. **端口被占用:**

```bash
# 查看端口占用情况
lsof /dev/ttyUSB0

# 或
fuser /dev/ttyUSB0

# 关闭占用端口的程序
kill -9 <PID>
```

2. **权限不足:**

```bash
# 修改设备权限 (临时)
sudo chmod 666 /dev/ttyUSB0

# 永久解决方案: 创建 udev 规则
echo 'KERNEL=="ttyUSB*", MODE="0666"' | sudo tee /etc/udev/rules.d/99-serial.rules
sudo udevadm control --reload-rules
sudo udevadm trigger
```

3. **波特率不支持:**

确保使用的波特率是标准值: 300, 1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200, 230400, 460800, 921600

#### 网络连接失败

网络连接问题排查:

1. **检查端口是否被占用:**

```bash
# 查看端口占用
netstat -tlnp | grep PORT

# 或使用 ss
ss -tlnp | grep PORT
```

2. **检查防火墙设置:**

```bash
# 查看防火墙状态
sudo ufw status

# 开放端口
sudo ufw allow 8080/tcp

# 或使用 iptables
sudo iptables -A INPUT -p tcp --dport 8080 -j ACCEPT
```

3. **测试网络连通性:**

```bash
# ping 测试
ping -c 4 192.168.1.100

# telnet 测试端口
telnet 192.168.1.100 8080

# nc 测试
nc -zv 192.168.1.100 8080
```

4. **检查绑定地址:**

如果服务器无法绑定端口,可能是地址被占用或权限不足:

```bash
# 查看端口使用情况
sudo lsof -i :8080

# 尝试绑定到所有地址
host: "0.0.0.0"  # 而非 "127.0.0.1"
```

### 3. 性能问题

#### 串口高速传输丢包

使用高波特率时出现丢包:

1. **检查 USB 串口线质量:**

- 使用带 FTDI 芯片的高质量 USB 串口适配器
- 避免使用过长的 USB 延长线
- 尝试使用不同的 USB 端口

2. **降低波特率测试:**

```bash
# 测试不同波特率
baudrate: 9600     # 稳定
baudrate: 115200   # 通常可靠
baudrate: 921600   # 需要高质量硬件
```

3. **启用硬件流控制:**

如果设备支持,启用 RTS/CTS 流控制:

```json
{
  "flow_control": "hardware"
}
```

4. **调整接收缓冲区:**

在应用设置中增大接收缓冲区大小。

#### UI 界面卡顿

界面响应缓慢:

1. **检查数据接收频率:**

- 限制日志显示数量
- 启用自动滚动限制
- 使用数据节流

2. **减少日志输出:**

```bash
# 使用较低的日志级别
./orangepi-debug-tool --log-level warn
```

3. **内存使用过高:**

```bash
# 监控内存使用
top -p $(pgrep orangepi-debug-tool)

# 或
ps aux | grep orangepi-debug-tool
```

#### 网络延迟高

网络操作响应慢:

1. **检查网络质量:**

```bash
# 测试延迟
ping -c 10 192.168.1.100

# 测试带宽
iperf3 -c 192.168.1.100
```

2. **调整超时设置:**

在连接配置中增大超时时间。

3. **使用 TCP 选项优化:**

- 启用 TCP_NODELAY
- 调整缓冲区大小

### 4. 崩溃处理

#### 查看运行时日志

程序崩溃时,首先查看日志:

```bash
# 运行程序并保存日志
./orangepi-debug-tool --log-level debug 2>&1 | tee debug.log

# 查看日志文件 (如果程序创建了日志文件)
cat ~/.local/share/orangepi-debug-tool/logs/*.log
```

#### 调试信息收集

提交 Issue 时请包含以下信息:

1. **操作系统和版本:**

```bash
# Linux
uname -a
cat /etc/os-release

# Windows
winver
systeminfo | findstr /B /C:"OS Name" /C:"OS Version"
```

2. **程序版本:**

```bash
./orangepi-debug-tool --version
```

3. **错误信息:**

复制完整的错误信息和堆栈跟踪。

4. **复现步骤:**

详细描述触发问题的操作步骤。

#### 使用调试器

对于 Rust 代码崩溃:

```bash
# 安装调试器
sudo apt install -y rust-gdb

# 使用 GDB 调试
rust-gdb ./target/release/orangepi-debug-tool
(gdb) run
# 程序崩溃时会停在错误位置
```

对于内存泄漏检测 (Linux):

```bash
# 安装 valgrind
sudo apt install -y valgrind

# 运行内存检测
valgrind --leak-check=full ./target/release/orangepi-debug-tool
```

对于 AddressSanitizer (需要重新编译):

```bash
RUSTFLAGS="-Z sanitizer=address" cargo build --release
ASAN_OPTIONS="detect_leaks=1" ./target/release/orangepi-debug-tool
```

### 5. 特定平台问题

#### Windows 问题

1. **WebView2 加载失败:**

Windows 11 通常自带 WebView2,Windows 10 可能需要安装:

- 下载地址: https://developer.microsoft.com/en-us/microsoft-edge/webview2/
- 或运行: `winget install Microsoft.WebView2`

2. **杀毒软件拦截:**

某些杀毒软件可能阻止程序运行:

- 将程序添加到白名单
- 暂时禁用杀毒软件进行测试

3. **路径问题:**

Windows 路径使用反斜杠,确保正确处理路径分隔符。

#### Linux 问题

1. **Wayland 显示问题:**

如果使用 Wayland,尝试切换到 X11:

```bash
# 临时切换
WAYLAND_DISPLAY= ./orangepi-debug-tool

# 或永久设置
echo "WAYLAND_DISPLAY=" >> ~/.profile
```

2. **GTK 主题问题:**

如果界面显示异常:

```bash
# 设置默认主题
export GTK_THEME=Adwaita
./orangepi-debug-tool
```

3. **权限拒绝:**

某些系统操作需要额外权限:

```bash
# 网络操作可能需要
sudo setcap 'cap_net_bind_service=+ep' ./orangepi-debug-tool
```

#### OrangePi ARM 问题

1. **硬件不兼容:**

确保交叉编译的目标架构与 OrangePi 匹配:

- OrangePi Zero/One: ARMv7 (armv7-unknown-linux-gnueabihf)
- OrangePi 5/5B: ARM64 (aarch64-unknown-linux-gnu)

2. **动态链接库缺失:**

检查所需的动态库:

```bash
# 在 OrangePi 上查看依赖
ldd ./orangepi-debug-tool

# 如果有 "not found",安装缺失的库
sudo apt install libstdc++6 libgcc1
```

3. **性能限制:**

ARM 平台资源有限,避免同时打开多个连接。

## 获取帮助

如果以上指南无法解决您的问题:

1. **查看 GitHub Issues:**

- 搜索现有问题: https://github.com/xfengyin/orangepi-debug-tool/issues
- 创建新 Issue 报告问题

2. **提交 Issue 时请包含:**

- 操作系统和版本
- 程序版本 (`./orangepi-debug-tool --version`)
- 完整的错误日志
- 复现步骤
- 尝试过的解决方法

3. **社区支持:**

- GitHub Discussions: https://github.com/xfengyin/orangepi-debug-tool/discussions
- 提交 Pull Request 修复问题

## 高级调试

### 启用详细日志

在调试模式下运行程序:

```bash
# 调试级别
./orangepi-debug-tool --log-level debug

# 跟踪特定模块
RUST_LOG=orangepi_debug_tool=debug ./orangepi-debug-tool
```

### 网络抓包

分析网络问题:

```bash
# 使用 tcpdump 抓包
sudo tcpdump -i eth0 -w capture.pcap port 8080

# 使用 Wireshark 分析
wireshark capture.pcap
```

### 串口监控

监控串口通信:

```bash
# 使用 socat 镜像串口
sudo apt install -y socat
socat -d -d /dev/ttyUSB0,raw,echo=0 /dev/ttyUSB1

# 使用 stty 查看串口配置
stty -F /dev/ttyUSB0 -a
```
