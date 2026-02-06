# OrangePi 上位机调试工具

一款专为 OrangePi 开发者设计的上位机调试工具，用于简化硬件调试和测试流程。

## 功能特性

- **实时串口通信**：支持多种波特率，实时发送和接收数据
- **十六进制模式**：支持十六进制数据的发送和接收显示
- **GPIO 控制**：通过串口命令控制 OrangePi 的 GPIO 引脚
- **数据可视化**：带时间戳的数据接收显示
- **日志记录**：自动记录通信数据
- **用户友好界面**：直观的图形界面，易于操作

## 安装

### 环境要求

- Python 3.6+
- OrangePi 开发板
- USB 转 TTL 模块（用于串口通信）

### 安装步骤

1. 克隆仓库：
   ```bash
   git clone https://github.com/xfengyin/orangepi-debug-tool.git
   cd orangepi-debug-tool
   ```

2. 安装依赖：
   ```bash
   pip install -r requirements.txt
   ```

## 使用方法

1. **连接设备**：将 OrangePi 通过 USB 转 TTL 模块连接到电脑
2. **启动软件**：
   ```bash
   python src/main.py
   ```
3. **配置串口**：在软件中选择对应的串口号和波特率（默认 115200）
4. **连接**：点击"连接"按钮
5. **调试**：通过发送区发送命令，查看接收区返回的数据

## GPIO 控制

软件支持通过特定命令控制 OrangePi 的 GPIO 引脚：

```
GPIO,<pin_number>,<HIGH|LOW>
```

例如，设置 GPIO 12 为高电平：
```
GPIO,12,HIGH
```

## 界面说明

- **连接设置区**：配置串口参数
- **发送区**：输入要发送的数据，支持文本和十六进制
- **接收区**：显示接收到的数据，支持十六进制显示
- **GPIO 控制区**：快速设置 GPIO 引脚状态

## 配置

可以通过 `config.ini` 文件修改默认配置，如默认波特率、窗口大小等。

## 依赖项

- `pyserial`: 串口通信
- `tkinter`: 图形界面
- `threading`: 多线程接收数据
- `datetime`: 时间戳

## 贡献

欢迎提交 Issue 和 Pull Request 来改进此工具。

## 许可证

此项目采用 MIT 许可证 - 详见 [LICENSE](./LICENSE) 文件。

## 版本历史

### 1.0.0
- 初始版本
- 基础串口通信功能
- GPIO 控制功能
- 图形用户界面