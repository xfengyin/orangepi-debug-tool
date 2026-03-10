# OrangePi Debug Tool v2.0 - 开发者指南

---

## 📖 目录

1. [项目结构](#项目结构)
2. [开发环境](#开发环境)
3. [构建说明](#构建说明)
4. [架构说明](#架构说明)
5. [API 参考](#api-参考)
6. [贡献指南](#贡献指南)

---

## 📁 项目结构

```
orangepi-debug-tool/
├── src-tauri/              # Rust 后端
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── src/
│       ├── main.rs         # 应用入口
│       ├── serial/         # 串口模块
│       ├── gpio/           # GPIO 模块
│       ├── pwm/            # PWM 模块
│       ├── utils/          # 工具函数
│       └── ui/             # UI 模块
│
├── src/                    # React 前端
│   ├── components/         # UI 组件
│   ├── hooks/              # 自定义 Hooks
│   ├── styles/             # 样式文件
│   ├── App.tsx
│   └── main.tsx
│
├── package.json
├── tsconfig.json
└── README.md
```

---

## 🛠️ 开发环境

### 系统要求

- **操作系统**: Windows 10+, Linux (Debian/Ubuntu), macOS 10.15+
- **Node.js**: 18+
- **Rust**: 1.70+
- **内存**: 4GB+
- **磁盘**: 1GB+

### 安装依赖

#### Windows

```powershell
# 安装 Rust
winget install Rustlang.Rustup

# 安装 Node.js
winget install OpenJS.NodeJS.LTS

# 安装 Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools
```

#### Linux (Ubuntu/Debian)

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# 安装依赖
sudo apt-get install -y libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

#### macOS

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Node.js
brew install node
```

---

## 🔨 构建说明

### 开发模式

```bash
# 安装依赖
npm install

# 启动开发服务器
npm run tauri dev
```

### 生产构建

```bash
# 构建所有平台
npm run tauri build

# 构建特定平台
npm run tauri build -- --target x86_64-pc-windows-msvc
```

### 测试

```bash
# Rust 测试
cd src-tauri && cargo test

# 前端测试
npm test

# E2E 测试
npm run test:e2e
```

### 代码检查

```bash
# Rust 代码检查
cd src-tauri && cargo clippy

# 前端代码检查
npm run lint

# 格式化
npm run format
```

---

## 🏗️ 架构说明

### 技术栈

**后端 (Rust):**
- Tauri - 桌面应用框架
- tokio - 异步运行时
- tokio-serial - 串口通信
- serde - 序列化
- tracing - 日志

**前端 (TypeScript):**
- React 18 - UI 框架
- Vite - 构建工具
- Tailwind CSS - 样式
- Recharts - 图表库
- Zustand - 状态管理

### 通信机制

```
┌─────────────┐    Tauri IPC    ┌─────────────┐
│   React     │ ◄─────────────► │    Rust     │
│   Frontend  │                 │   Backend   │
└─────────────┘                 └─────────────┘
                                       │
                                       ▼
                                ┌─────────────┐
                                │   System    │
                                │  (Serial/   │
                                │   GPIO/     │
                                │   PWM)      │
                                └─────────────┘
```

---

## 📡 API 参考

### 串口命令

```typescript
// 列出串口
const ports = await invoke('list_ports')

// 连接串口
await invoke('connect_serial', {
  port: '/dev/ttyUSB0',
  baudrate: 115200,
  data_bits: 8,
  parity: 'None',
  stop_bits: 1,
  flow_control: false
})

// 发送数据
await invoke('send_data', {
  data: 'Hello',
  hex_mode: false,
  append_newline: false,
  append_cr: false
})

// 断开串口
await invoke('disconnect_serial')
```

### GPIO 命令

```typescript
// 列出引脚
const pins = await invoke('list_pins')

// 设置引脚
await invoke('set_gpio', {
  pin: 17,
  value: 'high'  // 'high' | 'low' | 'input'
})

// 获取引脚状态
const state = await invoke('get_gpio', { pin: 17 })
```

### PWM 命令

```typescript
// 设置 PWM
await invoke('set_pwm', {
  channel: 0,
  frequency: 1000,
  duty_cycle: 50
})

// 停止 PWM
await invoke('stop_pwm', { channel: 0 })
```

---

## 🤝 贡献指南

### 提交流程

1. Fork 仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

### 代码规范

**Rust:**
```rust
// 使用 rustfmt
cargo fmt

// 使用 clippy 检查
cargo clippy
```

**TypeScript:**
```typescript
// 使用 ESLint
npm run lint

// 使用 Prettier
npm run format
```

### 提交信息格式

```
<type>(<scope>): <subject>

<body>

<footer>
```

**type 选项:**
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式
- `refactor`: 重构
- `test`: 测试
- `chore`: 构建/工具

**示例:**
```
feat(serial): 添加 CRC 校验功能

- 实现 CRC8/CRC16/CRC32 计算
- 添加校验配置选项

Closes #123
```

---

## 📚 参考资源

- [Tauri 文档](https://tauri.app/)
- [React 文档](https://react.dev/)
- [TypeScript 文档](https://www.typescriptlang.org/)
- [Tailwind CSS](https://tailwindcss.com/)

---

_OrangePi Debug Tool v2.0 - 开发团队_
