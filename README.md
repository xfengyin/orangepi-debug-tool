<div align="center">

<h1>🔧 OrangePi Debug Tool V2</h1>
<h3>为OrangePi开发者打造的全功能企业级调试工具</h3>

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
  <img src="https://img.shields.io/badge/Enterprise%20Grade-✓-green" alt="Enterprise Grade">
</p>

</div>

---

## 📋 简介

**OrangePi Debug Tool V2** 是一款基于 Tauri 2.0 框架开发的企业级跨平台调试工具，专为 OrangePi Zero3 及其他 OrangePi 系列开发板设计。采用 12 项企业级工程化规范重构，提供高可用、可扩展、安全可靠的调试能力。

## ✨ 企业级特性

### 🏗️ 架构优势

| 特性 | 说明 |
|------|------|
| **SPI 插件架构** | 支持热插拔设备适配器，扩展开发板支持 |
| **配置驱动** | YAML/JSON/TOML 多格式配置，支持热重载 |
| **依赖倒置** | 面向抽象接口，支持模型/工具无缝替换 |
| **单一职责** | 模块、工具、技能解耦清晰 |

### ⚡ 高可用保障

- **超时、重试、熔断**：自动故障恢复，指数退避重试策略
- **多模型兜底**：设备自动检测与降级适配
- **限流降级**：保护系统资源，防止过载

### 🔍 可观测性

- **全链路日志**：traceId 分布式追踪
- **Prometheus 指标**：性能监控与告警
- **健康检查**：Kubernetes 兼容的健康探针

### 🔒 安全合规

- **防 Prompt 注入**：安全拦截与数据脱敏
- **权限管控**：细粒度权限验证
- **审计日志**：完整操作记录
- **敏感数据掩码**：自动过滤机密信息

### 💾 状态管理

- **三层状态架构**：配置存储、设备状态、会话历史
- **撤销/重做**：支持操作回滚
- **事务一致性**：防重复执行，最终一致性保证

## 🎯 核心功能

### 🔌 串口调试
- 自动检测串口、波特率识别、数据可视化
- 实时终端、命令面板、历史记录

### 🔧 GPIO控制
- 引脚可视化配置、实时状态监控
- 输入/输出模式切换、电平控制

### 📊 PWM输出
- 多通道配置、频率/占空比调节
- 波形预览、实时参数调整

### 📝 数据日志
- SQLite持久化、多维度过滤
- 导出功能、统计分析

## 🚀 快速开始

### 系统要求

- Windows 10/11, Ubuntu 20.04+, macOS 10.15+
- Node.js >= 18.0.0
- Rust >= 1.75

### Ubuntu/Debian 依赖

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/xfengyin/orangepi-debug-tool-v2.git
cd orangepi-debug-tool-v2

# 安装前端依赖
npm install

# 开发模式运行
npm run tauri:dev

# 运行测试
cd src-tauri
cargo test
cd ..

# 构建发布版本
npm run tauri:build
```

### 下载预编译版本

从 [Releases](https://github.com/xfengyin/orangepi-debug-tool-v2/releases) 页面下载最新版本。

## 🛠️ 技术栈

| 层级 | 技术 |
|------|------|
| 前端 | React + TypeScript + Vite + Tailwind CSS |
| 后端 | Rust + Tauri 2.0 |
| 配置 | serde_yaml + serde_json + toml |
| 可观测性 | Prometheus + tracing + metrics |
| 测试 | cargo test + vitest + playwright |
| 构建 | Tauri CLI + GitHub Actions |

## 📖 文档

- [用户指南](src-tauri/docs/USER_GUIDE.md)
- [API文档](src-tauri/docs/API.md)
- [开发指南](docs/DEVELOPER-GUIDE.md)
- [更新日志](CHANGELOG.md)

## 🤝 贡献

请参考 [CONTRIBUTING.md](CONTRIBUTING.md)。

## 📄 许可证

MIT License
