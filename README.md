# OrangePi Debug Tool v2.0

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/typescript-5.0+-blue.svg)](https://www.typescriptlang.org/)
[![Tauri](https://img.shields.io/badge/tauri-1.5-24C8DB.svg)](https://tauri.app/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-ready-brightgreen)]()

**现代化 OrangePi 调试工具** - 轻量级、高性能、跨平台桌面应用

> 📊 **功能完成度：100%** | 📦 **准备发布：v2.0.0**

---

## ✨ 特性

- 📡 **串口调试** - 支持多种波特率，文本/十六进制模式
- 🔌 **GPIO 控制** - 可视化引脚布局，一键切换电平
- 📈 **PWM 输出** - 频率/占空比可调，预设波形
- 📊 **数据日志** - 实时记录，导出 CSV/JSON
- 🎨 **现代 UI** - Material Design，深色/浅色主题
- 🚀 **高性能** - Rust 后端，轻量级 (<50MB 内存)
- 🌐 **跨平台** - Windows / Linux / macOS

---

## 📦 安装

### 方式 1: 下载预编译包

访问 [Releases](https://github.com/xfengyin/orangepi-debug-tool/releases) 下载：

- **Windows**: `.msi` 或 `.exe`
- **Linux**: `.deb` 或 `.AppImage`
- **macOS**: `.dmg` 或 `.app`

### 方式 2: 源码构建

```bash
# 克隆仓库
git clone https://github.com/xfengyin/orangepi-debug-tool.git
cd orangepi-debug-tool

# 安装依赖
npm install

# 开发模式运行
npm run tauri dev

# 构建发布版
npm run tauri build
```

---

## 🚀 使用指南

### 串口调试

1. **选择串口** - 从下拉菜单选择可用端口
2. **设置波特率** - 9600 - 921600
3. **点击连接** - 建立串口连接
4. **发送数据** - 输入数据，点击发送或按 Enter
5. **切换模式** - Hex 模式 / 文本模式

### GPIO 控制

1. **切换电平** - 点击引脚按钮切换 HIGH/LOW
2. **批量操作** - 使用全部 HIGH/LOW 按钮
3. **导出配置** - 保存当前 GPIO 状态

### PWM 输出

1. **选择通道** - PWM0 或 PWM1
2. **设置频率** - 1Hz - 50MHz
3. **调节占空比** - 0-100%
4. **启动 PWM** - 开始输出

### 数据日志

1. **开始记录** - 点击开始记录按钮
2. **查看波形** - 实时显示数据图表
3. **导出数据** - CSV 或 JSON 格式

---

## 🏗️ 架构

```
OrangePi Debug Tool v2.0
├── src-tauri/ (Rust 后端)
│   ├── serial/      # 串口通信
│   ├── gpio/        # GPIO 控制
│   └── pwm/         # PWM 输出
├── src/ (React 前端)
│   ├── components/  # UI 组件
│   └── hooks/       # 自定义 Hooks
└── 打包配置
```

---

## 📈 性能对比

| 指标 | v1.0 (Python) | v2.0 (Tauri) | 提升 |
|------|---------------|--------------|------|
| 启动时间 | ~2s | <0.5s | **4x** |
| 内存占用 | ~150MB | <50MB | **67%** |
| 串口响应 | ~50ms | <5ms | **10x** |
| 打包体积 | ~80MB | <30MB | **62%** |

---

## 🛠️ 开发

### 环境要求

- Node.js 18+
- Rust 1.70+
- 支持的平台：Windows 10+, Linux, macOS 10.15+

### 构建命令

```bash
# 开发模式
npm run tauri dev

# 生产构建
npm run tauri build

# 前端测试
npm test

# Rust 测试
cd src-tauri && cargo test
```

---

## 📄 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

---

## 🔗 链接

- GitHub: https://github.com/xfengyin/orangepi-debug-tool
- Tauri: https://tauri.app/
- 文档：https://github.com/xfengyin/orangepi-debug-tool/wiki

---

_OrangePi Debug Tool v2.0 - 让调试更优雅_ ✨
