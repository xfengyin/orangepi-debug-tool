<div align="center">

<h1>🔧 OrangePi Debug Tool V2</h1>
<h3>为OrangePi开发者打造的全功能调试工具</h3>

<p>
  <a href="https://github.com/xfengyin/orangepi-debug-tool-v2/actions/workflows/ci.yml">
    <img src="https://github.com/xfengyin/orangepi-debug-tool-v2/actions/workflows/ci.yml/badge.svg" alt="CI Status">
  </a>
  <a href="https://github.com/xfengyin/orangepi-debug-tool-v2/releases">
    <img src="https://img.shields.io/github/v/release/xfengyin/orangepi-debug-tool-v2?label=Release" alt="Release">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/github/license/xfengyin/orangepi-debug-tool-v2" alt="License">
  </a>
</p>

<p>
  <img src="https://img.shields.io/badge/Tauri-2.0-blue?logo=tauri" alt="Tauri">
  <img src="https://img.shields.io/badge/Rust-1.70+-orange?logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/React-18+-61dafb?logo=react" alt="React">
</p>

</div>

---

## 📋 简介

**OrangePi Debug Tool V2** 是一款基于 Tauri 2.0 框架开发的跨平台调试工具，专为 OrangePi 系列开发板设计。提供直观的图形界面，让开发者轻松进行串口通信、GPIO 控制和 PWM 信号调试。

## ✨ 功能特性

### 🔌 串口调试
- 自动检测串口、波特率识别、数据可视化

### 🔧 GPIO控制
- 引脚可视化配置、实时状态监控

### 📊 PWM输出
- 多通道配置、频率/占空比调节

### 📝 数据日志
- SQLite持久化、多维度过滤

## 🚀 快速开始

### 系统要求

- Windows 10/11, Ubuntu 20.04+, macOS 10.15+
- Node.js >= 18.0.0
- Rust >= 1.75

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/xfengyin/orangepi-debug-tool-v2.git
cd orangepi-debug-tool-v2

# 安装前端依赖
npm install

# 开发模式运行
npm run tauri:dev

# 构建发布版本
npm run tauri:build
```

### 下载预编译版本

从 [Releases](https://github.com/xfengyin/orangepi-debug-tool-v2/releases) 页面下载。

## 🛠️ 技术栈

| 层级 | 技术 |
|------|------|
| 前端 | React + TypeScript + Vite |
| 后端 | Rust + Tauri |
| 构建 | Tauri CLI |

## 📖 文档

- [开发指南](docs/DEVELOPER-GUIDE.md)
- [API文档](docs/API.md)
- [更新日志](docs/CHANGELOG.md)

## 🤝 贡献

请参考 [CONTRIBUTING.md](CONTRIBUTING.md)。

## 📄 许可证

MIT License
