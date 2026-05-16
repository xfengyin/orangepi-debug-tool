# OrangePi Debug Tool

A cross-platform debug tool for OrangePi development.

## 项目概述

这是一个基于 Tauri 2.0 + React + TypeScript + Rust 的跨平台调试工具，用于 OrangePi 开发。

### 技术栈

- **前端**: React + TypeScript
- **后端**: Rust
- **框架**: Tauri 2.0
- **目标平台**: Windows 10/11 + OrangePi ARM Linux

## 构建说明

### 环境要求

- Node.js 18+
- Rust 1.70+
- Tauri CLI 2.0+

### 安装依赖

```bash
# 安装 Node.js 依赖
npm install

# 安装 Tauri CLI (如果尚未安装)
npm install -g @tauri-apps/cli

# 安装 Rust 依赖
cd src-tauri
cargo fetch
cd ..
```

### 本地构建

```bash
# 开发模式
npm run tauri:dev

# 生产构建
npm run tauri:build

# 或者使用 Rust 原生命令
cd src-tauri
cargo build --release
```

### 交叉编译 (OrangePi ARM)

#### 安装交叉编译工具链

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y gcc-arm-linux-gnueabihf g++-arm-linux-gnueabihf gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
```

#### 运行构建脚本

```bash
# 给脚本添加执行权限
chmod +x scripts/cross-compile.sh

# 执行交叉编译
./scripts/cross-compile.sh
```

#### 手动交叉编译

```bash
# ARMv7 (OrangePi Zero 等)
cargo build --release --target armv7-unknown-linux-gnueabihf

# ARM64 (OrangePi 5 等)
cargo build --release --target aarch64-unknown-linux-gnu
```

## 部署

### Windows

```bash
# MSI 安装包
scp src-tauri/target/release/bundle/msi/*.msi user@windows-pc:/
```

### OrangePi

```bash
# ARMv7
scp target/armv7-unknown-linux-gnueabihf/release/orangepi-debug-tool pi@orangepi:/home/pi/

# ARM64
scp target/aarch64-unknown-linux-gnu/release/orangepi-debug-tool pi@orangepi:/home/pi/
```

## 运行

### 在 OrangePi 上运行

```bash
# 添加执行权限
chmod +x orangepi-debug-tool

# 运行程序
./orangepi-debug-tool
```

### 运行时选项

```bash
# 显示帮助信息
./orangepi-debug-tool --help

# 指定日志级别 (debug, info, warn, error)
./orangepi-debug-tool --log-level debug
```

## 项目结构

```
.
├── src/                    # React 前端源码
├── src-tauri/             # Rust 后端源码
│   ├── src/
│   │   ├── adapters/      # 适配器层 (串口、网络等)
│   │   ├── commands/      # Tauri 命令
│   │   ├── services/      # 业务逻辑服务
│   │   ├── lib.rs         # 库入口
│   │   └── main.rs        # 主程序入口
│   ├── Cargo.toml         # Rust 依赖配置
│   └── build.rs           # 构建脚本
├── scripts/               # 构建脚本
│   ├── build.sh          # 本地构建脚本
│   └── cross-compile.sh  # 交叉编译脚本
├── .cargo/               # Cargo 配置
│   └── config.toml       # 交叉编译工具链配置
└── README.md
```

## 功能特性

### 串口调试
- 列出可用串口
- 连接/断开串口
- 发送/接收数据
- 定时发送
- HEX/ASCII 格式支持

### 网络调试
- TCP 服务器/客户端
- UDP 会话
- 数据发送/接收

### 日志管理
- 实时日志记录
- 日志导出
- 统计数据

## 文档

- [API 文档](src-tauri/docs/API.md)
- [故障排查指南](src-tauri/docs/TROUBLESHOOTING.md)

## 许可证

MIT License

## 联系方式

- GitHub Issues: https://github.com/xfengyin/orangepi-debug-tool/issues
