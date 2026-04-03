# Changelog

All notable changes to OrangePi Debug Tool will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2026-04-03

### Added
- 全新架构重构，基于 Tauri (Rust + React)
- Material Design 3 界面设计
- 深色/浅色双主题支持
- 串口自动检测与波特率识别
- 实时数据可视化图表
- 自定义快捷指令集
- GPIO 引脚可视化控制
- PWM 波形编辑与播放
- SQLite 数据持久化
- 跨平台支持 (Windows/Linux/macOS)
- 系统托盘集成
- 全局错误捕获与处理
- 二进制通信协议优化

### Changed
- 从 Electron 迁移至 Tauri
- 重构串口通信模块，使用异步架构
- 优化 GPIO/PWM 设备管理
- 改进状态管理，使用 Zustand

### Removed
- 旧版 Electron 实现
- 过时的依赖包

---

## [1.0.0] - 2024-01-01

### Added
- 初始版本发布
- 基础串口通信功能
- GPIO 控制功能
- PWM 输出功能