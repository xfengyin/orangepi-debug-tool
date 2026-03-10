# OrangePi Debug Tool v2.0 - Tauri 重构架构

## 🎯 项目定位

**现代化 OrangePi 调试工具** - 轻量级、高性能、跨平台桌面应用

---

## 🏗️ 架构概览

```
orangepi-debug-tool v2.0
├── src-tauri/              # Rust 后端
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   └── src/
│       ├── main.rs         # 应用入口
│       ├── serial/         # 串口通信
│       │   ├── mod.rs
│       │   ├── port.rs     # 串口管理
│       │   └── protocol.rs # 协议解析
│       ├── gpio/           # GPIO 控制
│       │   ├── mod.rs
│       │   ├── pin.rs      # 引脚定义
│       │   └── controller.rs
│       ├── pwm/            # PWM 控制
│       │   ├── mod.rs
│       │   └── generator.rs
│       └── utils/          # 工具函数
│           ├── mod.rs
│           └── logger.rs
│
├── src/                    # React 前端
│   ├── components/
│   │   ├── SerialPanel.tsx     # 串口面板
│   │   ├── GPIOView.tsx        # GPIO 视图
│   │   ├── PWMControl.tsx      # PWM 控制
│   │   ├── DataLogger.tsx      # 数据日志
│   │   └── WaveformChart.tsx   # 波形图表
│   ├── hooks/
│   │   ├── useSerial.ts        # 串口 Hook
│   │   ├── useGPIO.ts          # GPIO Hook
│   │   └── useWebSocket.ts     # WebSocket Hook
│   ├── styles/
│   │   └── globals.css         # 全局样式
│   ├── App.tsx
│   └── main.tsx
│
├── public/
│   ├── icon.ico
│   └── icon.png
│
├── package.json
├── tailwind.config.js
├── vite.config.ts
├── tsconfig.json
└── README.md
```

---

## 🎨 技术栈

### 后端 (Rust + Tauri)

| 组件 | 库 | 用途 |
|------|-----|------|
| 串口通信 | `tokio-serial` | 异步串口读写 |
| GPIO 控制 | `rppal` / `sysfs_gpio` | GPIO 引脚控制 |
| PWM 生成 | `rppal::pwm` | PWM 信号输出 |
| 日志 | `tracing` + `tracing-subscriber` | 结构化日志 |
| 序列化 | `serde` + `serde_json` | 数据交换 |
| 异步运行时 | `tokio` | 异步 IO |

### 前端 (React + TypeScript)

| 组件 | 库 | 用途 |
|------|-----|------|
| UI 框架 | `React 18` | 组件系统 |
| 样式 | `Tailwind CSS` | 快速样式 |
| 图表 | `Recharts` | 数据可视化 |
| 状态 | `Zustand` | 状态管理 |
| 串口 | Tauri API | 原生串口访问 |
| 构建 | `Vite` | 快速构建 |

### 打包分发

| 平台 | 格式 | 工具 |
|------|------|------|
| Windows | `.exe`, `.msi` | Tauri + NSIS |
| Linux | `.deb`, `.AppImage` | Tauri + dpkg |
| macOS | `.dmg`, `.app` | Tauri + create-dmg |

---

## 📊 核心功能

### 1. 串口调试

**功能:**
- ✅ 自动检测可用串口
- ✅ 波特率选择 (9600 - 921600)
- ✅ 文本/十六进制模式
- ✅ 时间戳标记
- ✅ 数据发送/接收
- ✅ 自定义数据包格式

**界面:**
```
┌─────────────────────────────────────┐
│ 串口调试                           │
├─────────────────────────────────────┤
│ 端口：[COM3 ▼]  波特率：[115200 ▼] │
│                                     │
│ [连接] [断开]                       │
├─────────────────────────────────────┤
│ [接收区域 - 实时显示]               │
│ > 2026-03-10 07:00:00.123           │
│   TX: Hello OrangePi                │
│ > 2026-03-10 07:00:00.456           │
│   RX: OK                            │
├─────────────────────────────────────┤
│ [发送输入框...]      [发送] [清空]  │
│ ☐ Hex 模式  ☐ 显示时间戳            │
└─────────────────────────────────────┘
```

### 2. GPIO 控制

**功能:**
- ✅ 可视化引脚布局
- ✅ 一键切换电平 (HIGH/LOW)
- ✅ 实时状态显示
- ✅ 批量操作
- ✅ 快捷键支持

**界面:**
```
┌─────────────────────────────────────┐
│ GPIO 控制                           │
├─────────────────────────────────────┤
│  [3.3V] [5V] [GND] [GND]            │
│                                     │
│  [GPIO17] [GPIO27] [GPIO22] [GPIO23]│
│  [HIGH▼]  [LOW▼]   [HIGH▼] [LOW▼]  │
│                                     │
│  [GPIO24] [GPIO25] [GPIO5]  [GPIO6] │
│  [LOW▼]   [HIGH▼]  [LOW▼]  [HIGH▼] │
│                                     │
│  [批量设置] [导出配置] [导入配置]    │
└─────────────────────────────────────┘
```

### 3. PWM 控制

**功能:**
- ✅ 频率设置 (1Hz - 50MHz)
- ✅ 占空比调节 (0-100%)
- ✅ 预设波形 (正弦波、方波、三角波)
- ✅ 实时预览

### 4. 数据日志

**功能:**
- ✅ 实时数据记录
- ✅ 导出 CSV/JSON
- ✅ 波形图表显示
- ✅ 数据筛选/搜索

---

## 🎨 UI/UX 设计

### 配色方案

```css
/* 浅色主题 */
--primary: #6366f1;      /* Indigo */
--secondary: #8b5cf6;    /* Purple */
--success: #10b981;      /* Emerald */
--warning: #f59e0b;      /* Amber */
--danger: #ef4444;       /* Red */
--background: #f8fafc;   /* Slate 50 */
--surface: #ffffff;      /* White */
--text: #1e293b;         /* Slate 800 */

/* 深色主题 */
--primary-dark: #818cf8;
--background-dark: #0f172a;
--surface-dark: #1e293b;
--text-dark: #f1f5f9;
```

### 设计风格

- **Material Design 3** - 现代化设计语言
- **圆角卡片** - 8px border-radius
- **阴影层次** -  subtle shadow
- **流畅动画** - 200ms transition
- **响应式布局** - 适配不同窗口大小

---

## 🔧 Tauri 配置

### tauri.conf.json

```json
{
  "build": {
    "beforeBuildCommand": "npm run build",
    "beforeDevCommand": "npm run dev",
    "devPath": "http://localhost:5173",
    "distDir": "../dist"
  },
  "package": {
    "productName": "OrangePi Debug Tool",
    "version": "2.0.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "shell": {
        "all": false,
        "open": true
      },
      "fs": {
        "readFile": true,
        "writeFile": true
      }
    },
    "bundle": {
      "active": true,
      "targets": ["deb", "msi", "dmg", "appimage"],
      "identifier": "com.xfengyin.orangepi-debug-tool",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    }
  }
}
```

---

## 📈 性能目标

| 指标 | v1.0 | v2.0 目标 |
|------|------|----------|
| 启动时间 | ~2s | <0.5s |
| 内存占用 | ~150MB | <50MB |
| 串口响应延迟 | ~50ms | <5ms |
| 打包体积 | ~80MB | <30MB |

---

## 📋 重构阶段

### Phase 1: 项目骨架 (1 周)
- [ ] 初始化 Tauri 项目
- [ ] 配置 Rust 后端
- [ ] 配置 React 前端
- [ ] 设置 CI/CD

### Phase 2: 串口功能 (1 周)
- [ ] Rust 串口实现
- [ ] Tauri API 绑定
- [ ] React 前端界面
- [ ] 数据收发测试

### Phase 3: GPIO 功能 (1 周)
- [ ] Rust GPIO 实现
- [ ] 引脚状态同步
- [ ] 可视化界面
- [ ] 批量操作

### Phase 4: PWM 功能 (3 天)
- [ ] Rust PWM 实现
- [ ] 频率/占空比控制
- [ ] 波形预览

### Phase 5: 数据可视化 (1 周)
- [ ] 波形图表
- [ ] 数据日志
- [ ] 导出功能

### Phase 6: 打包发布 (3 天)
- [ ] Windows 打包
- [ ] Linux 打包
- [ ] macOS 打包
- [ ] GitHub Release

**总计：5 周**

---

## 🚀 快速开始

### 开发环境

```bash
# 克隆仓库
git clone https://github.com/xfengyin/orangepi-debug-tool.git
cd orangepi-debug-tool

# 安装依赖
npm install

# 安装 Rust (如果未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Tauri CLI
cargo install tauri-cli

# 开发模式运行
npm run tauri dev

# 构建发布版
npm run tauri build
```

### 系统要求

**开发:**
- Node.js 18+
- Rust 1.70+
- 支持的平台：Windows 10+, Linux (Debian/Ubuntu), macOS 10.15+

**运行:**
- Windows 10+ / Linux / macOS 10.15+
- 2GB RAM
- 100MB 磁盘空间

---

## 📦 交付物

### 代码
- ✅ Rust 后端 (串口/GPIO/PWM)
- ✅ React 前端 (完整 UI)
- ✅ Tauri 配置
- ✅ 测试套件

### 文档
- ✅ README.md
- ✅ 开发指南
- ✅ API 文档
- ✅ 用户手册

### 安装包
- ✅ Windows: `.msi`, `.exe`
- ✅ Linux: `.deb`, `.AppImage`
- ✅ macOS: `.dmg`, `.app`

---

## 🔗 参考资源

- [Tauri 文档](https://tauri.app/)
- [CustomTkinter](https://github.com/TomSchimansky/CustomTkinter)
- [pySerial](https://github.com/pyserial/pyserial)
- [rppal](https://github.com/golemparts/rppal)

---

_OrangePi Debug Tool v2.0 - 让调试更优雅_
