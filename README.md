# OrangePi Debug Tool v2.0

<p align="center">
  <img src="public/icons/icon.png" alt="OrangePi Debug Tool" width="120">
</p>

<p align="center">
  <strong>为OrangePi开发者打造的全功能调试工具</strong>
</p>

<p align="center">
  <a href="#功能特性">功能特性</a> •
  <a href="#安装指南">安装指南</a> •
  <a href="#使用说明">使用说明</a> •
  <a href="#开发文档">开发文档</a> •
  <a href="#贡献指南">贡献指南</a>
</p>

---

## 功能特性

### 🔌 串口调试
- 自动检测OrangePi设备串口
- 波特率自动识别
- 支持多种数据格式 (文本/十六进制)
- 实时数据可视化图表
- 自定义快捷指令集
- 数据过滤与搜索

### 🔧 GPIO控制
- 引脚可视化配置
- 实时状态监控
- 批量引脚操作
- 中断检测支持

### 📊 PWM输出
- 多通道PWM配置
- 频率/占空比精确调节
- 波形编辑与播放
- 实时波形预览

### 📝 数据日志
- SQLite持久化存储
- 多维度数据过滤
- 日志导出功能
- 可视化数据分析

### 🎨 界面特性
- Material Design 3 设计风格
- 深色/浅色双主题
- 响应式布局适配
- 跨平台支持 (Windows/Linux/macOS)

---

## 安装指南

### 系统要求

- **操作系统**: Windows 10/11, Ubuntu 20.04+, macOS 10.15+
- **Node.js**: >= 18.0.0
- **Rust**: >= 1.75
- **内存**: >= 4GB RAM
- **磁盘空间**: >= 500MB

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/orangepi-debug-tool/orangepi-debug-tool-v2.git
cd orangepi-debug-tool-v2

# 安装前端依赖
npm install

# 安装Rust依赖
cd src-tauri && cargo fetch && cd ..

# 开发模式运行
npm run tauri:dev

# 构建发布版本
npm run tauri:build
```

### 下载预编译版本

从 [Releases](https://github.com/orangepi-debug-tool/orangepi-debug-tool-v2/releases) 页面下载对应平台的安装包。

---

## 使用说明

### 快速开始

1. **连接设备**: 使用USB线将OrangePi连接到电脑
2. **选择串口**: 在串口页面点击"自动检测"或手动选择端口
3. **配置参数**: 设置波特率(默认115200)、数据位、校验位、停止位
4. **开始通信**: 点击"连接"按钮，即可开始收发数据

### 串口调试

```
发送指令: AT+VERSION
接收数据: VERSION: 2.0.0
```

### GPIO控制

在GPIO页面：
1. 点击引脚卡片查看详情
2. 使用开关切换引脚状态
3. 配置输入/输出模式

### PWM配置

在PWM页面：
1. 设置目标频率(Hz)
2. 调节占空比(%)
3. 点击"应用配置"

---

## 开发文档

详见 [DEVELOPER-GUIDE.md](docs/DEVELOPER-GUIDE.md)

### 项目结构

```
orangepi-debug-tool-v2/
├── src/                    # 前端React代码
│   ├── components/         # React组件
│   ├── stores/            # Zustand状态管理
│   ├── types/             # TypeScript类型定义
│   └── styles/            # CSS样式
├── src-tauri/             # Rust后端代码
│   ├── src/
│   │   ├── commands/      # Tauri命令
│   │   ├── devices/       # 设备管理模块
│   │   ├── utils/         # 工具模块
│   │   └── ...
│   └── configs/           # 多环境配置
├── tests/                 # 测试文件
├── docs/                  # 文档
└── .github/workflows/     # CI/CD配置
```

---

## 贡献指南

我们欢迎所有形式的贡献！

### 提交Issue

- 使用Issue模板
- 描述清晰的问题复现步骤
- 提供系统环境信息

### 提交Pull Request

1. Fork本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建Pull Request

### 代码规范

- 前端: ESLint + Prettier
- Rust: cargo fmt + clippy
- 提交信息遵循 [Conventional Commits](https://www.conventionalcommits.org/)

---

## 许可证

[MIT License](LICENSE) © 2026 OrangePi Team

---

## 致谢

- [Tauri](https://tauri.app/) - 跨平台应用框架
- [React](https://react.dev/) - UI框架
- [Material-UI](https://mui.com/) - 组件库
- [Tokio](https://tokio.rs/) - Rust异步运行时