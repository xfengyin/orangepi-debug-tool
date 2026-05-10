# OrangePi Debug Tool - 构建和发布指南

## 环境准备

### 系统要求
- Node.js >= 18.0.0
- Rust >= 1.75
- 操作系统: Windows 10/11, Ubuntu 20.04+, macOS 10.15+

### Ubuntu/Debian 依赖安装
```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

### macOS 依赖安装
```bash
# 使用 Homebrew
brew install rust node
```

### Windows 依赖安装
- 安装 Visual Studio C++ Build Tools
- 安装 Rust
- 安装 Node.js

## 构建步骤

### 1. 安装依赖
```bash
npm install
```

### 2. 开发模式运行
```bash
npm run tauri:dev
```

### 3. 运行测试
```bash
# Rust 后端测试
cd src-tauri
cargo test

# 前端测试
npm run test

# E2E 测试
npm run test:e2e
```

### 4. 构建发布版本
```bash
npm run tauri:build
```

### 5. 发布构建产物
构建成功后，发布包将位于 `src-tauri/target/release/bundle/` 目录下。

## Tauri 2.0 配置迁移说明

如果需要将配置文件完全升级到 Tauri 2.0 格式，参考以下步骤：

### 更新配置文件结构
Tauri 2.0 使用了新的配置结构。可以通过以下命令生成新的配置模板：
```bash
cd src-tauri
npx tauri init
```

### 主要配置变化
- `devPath` → `devUrl`
- `distDir` → `frontendDist`
- `package` 部分整合到顶层或 `app` 部分
- 插件配置变化

## 企业级特性说明

### SPI 插件架构
项目采用插件架构，支持通过 `DeviceAdapterRegistry` 动态加载和切换设备适配器。

### 可观测性
- 集成 Prometheus 指标导出
- 分布式追踪支持
- 健康检查端点

### 安全特性
- 权限控制
- 审计日志
- 数据脱敏
- Prompt 注入防护

### 配置驱动
支持 YAML/JSON/TOML 多种配置格式，支持热重载。

## 贡献指南

### 代码规范
- 前端: TypeScript + React + ESLint + Prettier
- 后端: Rust + clippy + rustfmt
- 提交前运行: `npm run lint` 和 `cargo clippy`

### 提交规范
使用 Conventional Commits 规范。

## 常见问题

### GTK/webkit2gtk 版本问题
在 Ubuntu 24.04+ 中，使用 `libwebkit2gtk-4.1-dev` 而不是 `libwebkit2gtk-4.0-dev`。

### 编译错误
确保所有系统依赖都已安装，并运行 `cargo update` 更新 Rust 依赖。
