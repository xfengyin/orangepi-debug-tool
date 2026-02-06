# OrangePi 上位机调试工具 - 使用指南

## 安装

1. 克隆仓库
   ```bash
   git clone https://github.com/xfengyin/orangepi-debug-tool.git
   cd orangepi-debug-tool
   ```

2. 安装依赖（如适用）
   ```bash
   pip install -r requirements.txt
   ```

## 快速开始

1. 启动上位机软件
2. 连接 OrangePi 设备
3. 选择相应的通信端口
4. 开始调试

## 主要功能

### 串口通信
- 支持多种波特率
- 实时数据显示
- 发送和接收数据

### GPIO 控制
- 引脚状态监控
- 输入输出控制
- PWM 控制

### 数据可视化
- 实时图表显示
- 数据记录
- 导出功能

## 配置

软件支持多种配置选项，可根据不同 OrangePi 型号调整参数。

## 故障排除

常见问题及解决方案：

1. 无法连接设备
   - 检查 USB 连接
   - 确认驱动程序已安装
   - 验证端口设置

2. 数据传输异常
   - 检查波特率设置
   - 确认协议匹配