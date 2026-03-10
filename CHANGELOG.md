# Changelog

All notable changes to OrangePi Debug Tool will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [2.0.0] - 2026-03-10

### 🎉 Initial Release

#### Added

**串口调试:**
- 完整串口配置 (波特率/数据位/校验/停止位/流控制)
- Hex/文本模式切换
- 实时统计信息
- 快捷命令
- 定时发送
- 发送历史
- CRC 校验 (CRC8/CRC16/CRC32)
- 终端仿真 (VT100/VT220)

**GPIO 控制:**
- 可视化引脚布局
- 一键切换 HIGH/LOW
- 批量操作
- 配置导出/导入

**PWM 输出:**
- 频率设置 (1Hz - 50MHz)
- 占空比调节 (0-100%)
- 预设波形 (正弦波/方波/三角波)
- 实时预览

**数据日志:**
- 实时波形显示
- 可配置采样率 (10ms - 1000ms)
- 数据导出 (CSV/JSON/TXT)
- 数据表格

**协议解析:**
- MODBUS RTU 协议支持
- 自定义协议配置
- CRC 校验
- 字段解析

**高级功能:**
- 多串口管理 (最多 8 个)
- 宏命令/脚本引擎
- 主题切换 (浅色/深色/系统)
- 国际化 (中文/English)

**文档:**
- 用户手册
- 开发者指南
- 项目状态
- 测试计划
- 发布清单
- 图标设计

#### Technical

**后端 (Rust):**
- Tauri 桌面应用框架
- tokio 异步运行时
- tokio-serial 串口通信
- rppal GPIO/PWM 控制
- serde 序列化
- tracing 日志

**前端 (TypeScript):**
- React 18 UI 框架
- Vite 构建工具
- Tailwind CSS 样式
- Recharts 图表库
- Zustand 状态管理

#### Performance

- 启动时间：<0.5s
- 内存占用：<50MB
- 串口响应：<5ms
- 打包体积：<30MB

---

## [Unreleased]

### Planned

- [ ] E2E 自动化测试
- [ ] 更多协议支持 (CAN, I2C, SPI)
- [ ] 插件系统
- [ ] 云同步
- [ ] 移动端支持

---

## Version Support

| Version | Supported | End of Life |
|---------|-----------|-------------|
| 2.0.x   | ✅ Yes    | -           |

---

## Release Notes

### v2.0.0 Highlights

**OrangePi Debug Tool v2.0.0** is the first stable release of the modernized debugging tool for OrangePi developers.

**Key Features:**
- ✅ Professional serial port debugging
- ✅ GPIO control with visual interface
- ✅ PWM output with waveform preview
- ✅ Real-time data logging
- ✅ Protocol analyzer (MODBUS RTU)
- ✅ Multi-port management
- ✅ Macro/scripting support
- ✅ Theme switching
- ✅ Internationalization

**Target Users:**
- OrangePi developers
- Embedded system engineers
- IoT developers
- Electronics hobbyists

**Getting Started:**
```bash
# Download and install
# Windows: Download .msi installer
# Linux: Download .deb package
# macOS: Download .dmg file

# Or build from source
git clone https://github.com/xfengyin/orangepi-debug-tool.git
cd orangepi-debug-tool
npm install
npm run tauri build
```

**Documentation:**
- [User Manual](USER-MANUAL.md)
- [Developer Guide](DEVELOPER-GUIDE.md)
- [API Reference](docs/API.md)

---

*For more details, see [README.md](./README.md) and [USER-MANUAL.md](./USER-MANUAL.md).*
