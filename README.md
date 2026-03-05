# OrangePi 上位机调试工具

[![CI](https://github.com/xfengyin/orangepi-debug-tool/actions/workflows/ci.yml/badge.svg)](https://github.com/xfengyin/orangepi-debug-tool/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Python](https://img.shields.io/badge/python-3.8%2B-blue)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey)

一款专为 OrangePi 开发者设计的**现代化上位机调试工具**，采用 Material Design 设计风格，提供直观美观的用户界面。

![Screenshot](docs/screenshot.png)

## ✨ 核心特性

### 🔌 串口通信
- 支持多种波特率（9600 - 921600）
- 实时数据发送和接收
- 十六进制/文本模式切换
- 自定义数据包格式

### 🎛️ GPIO 控制
- 可视化 GPIO 引脚状态
- 一键切换引脚电平
- 支持批量操作
- PWM 输出控制

### 📊 数据可视化
- 实时数据波形显示
- 数据日志记录
- 支持导出 CSV/JSON
- 时间戳标记

### 🎨 现代化界面
- Material Design 设计风格
- 深色/浅色主题切换
- 响应式布局
- 自定义配色方案

## 🚀 快速开始

### 安装

```bash
# 克隆仓库
git clone https://github.com/xfengyin/orangepi-debug-tool.git
cd orangepi-debug-tool

# 安装依赖
pip install -e ".[dev]"
```

### 运行

```bash
# 命令行启动
orangepi-debug

# 或者直接运行
python -m orangi_debug_tool
```

## 📖 使用指南

### 串口连接

1. 选择串口设备（自动检测）
2. 设置波特率
3. 点击"连接"按钮

### GPIO 控制

1. 切换到"GPIO"标签页
2. 选择要控制的引脚
3. 点击切换按钮或使用快捷键

### 数据发送

1. 在输入框输入数据
2. 选择发送模式（文本/十六进制）
3. 点击发送或按 Enter

## 🛠️ 开发

### 运行测试

```bash
pytest tests/ -v --cov=src
```

### 代码格式化

```bash
black src/ tests/
```

### 类型检查

```bash
mypy src/
```

## 📁 项目结构

```
orangepi-debug-tool/
├── src/orangepi_debug_tool/
│   ├── __init__.py
│   ├── main.py          # 应用入口
│   ├── app.py           # 主应用类
│   ├── models/          # 数据模型
│   │   ├── serial_port.py
│   │   └── gpio.py
│   ├── views/           # 视图层
│   │   ├── main_window.py
│   │   ├── serial_view.py
│   │   └── gpio_view.py
│   ├── controllers/     # 控制器
│   │   ├── serial_controller.py
│   │   └── gpio_controller.py
│   ├── widgets/         # 自定义组件
│   │   ├── styled_button.py
│   │   └── status_panel.py
│   └── utils/           # 工具函数
│       ├── config.py
│       └── logger.py
├── tests/
├── docs/
├── pyproject.toml
└── README.md
```

## 🤝 贡献

欢迎贡献代码！请查看 [CONTRIBUTING.md](CONTRIBUTING.md)

## 📄 许可证

[MIT License](LICENSE)

## 🙏 致谢

- [CustomTkinter](https://github.com/TomSchimansky/CustomTkinter) - 现代化 UI 框架
- [pySerial](https://github.com/pyserial/pyserial) - 串口通信库
