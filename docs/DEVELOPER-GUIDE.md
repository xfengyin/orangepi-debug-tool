# 开发者指南

## 目录

- [项目概述](#项目概述)
- [技术栈](#技术栈)
- [环境配置](#环境配置)
- [架构设计](#架构设计)
- [开发流程](#开发流程)
- [性能优化](#性能优化)
- [调试技巧](#调试技巧)
- [发布流程](#发布流程)

---

## 项目概述

OrangePi Debug Tool v2.0 是基于 Tauri (Rust + React) 构建的跨平台调试工具，专为 OrangePi 开发者设计。

### 核心特性

- **跨平台**: 支持 Windows、Linux、macOS
- **高性能**: Rust后端 + 异步架构
- **可扩展**: 模块化设计，易于添加新功能
- **现代化UI**: Material Design 3 + React 18

---

## 技术栈

### 前端

| 技术 | 版本 | 用途 |
|------|------|------|
| React | 18.x | UI框架 |
| TypeScript | 5.x | 类型系统 |
| Material-UI | 5.x | 组件库 |
| Zustand | 4.x | 状态管理 |
| Vite | 5.x | 构建工具 |
| Recharts | 2.x | 数据可视化 |

### 后端

| 技术 | 版本 | 用途 |
|------|------|------|
| Rust | 1.75+ | 系统语言 |
| Tauri | 1.6 | 跨平台框架 |
| Tokio | 1.36 | 异步运行时 |
| Serde | 1.0 | 序列化 |
| SQLx | 0.7 | 数据库 |

---

## 环境配置

### 必需工具

```bash
# Node.js (使用 nvm 安装)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20
nvm use 20

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update

# Linux 依赖 (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev

# macOS 依赖
# 安装 Xcode Command Line Tools
xcode-select --install

# Windows 依赖
# 安装 Visual Studio Build Tools
```

### IDE 推荐配置

**VS Code 扩展:**
- rust-analyzer
- ESLint
- Prettier
- Tailwind CSS IntelliSense
- Tauri

**settings.json:**
```json
{
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "esbenp.prettier-vscode",
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy"
}
```

---

## 架构设计

### 整体架构

```
┌─────────────────────────────────────────────────────┐
│                    Frontend (React)                  │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐   │
│  │ Serial  │ │  GPIO   │ │  PWM    │ │  Log    │   │
│  │  Page   │ │  Page   │ │  Page   │ │  Page   │   │
│  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘   │
│       └─────────────┴─────────┴─────────────┘       │
│                   Zustand Stores                     │
└───────────────────────┬─────────────────────────────┘
                        │ Tauri IPC
                        │ (Binary Protocol)
┌───────────────────────┴─────────────────────────────┐
│                     Backend (Rust)                   │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐   │
│  │ Serial  │ │  GPIO   │ │  PWM    │ │  State  │   │
│  │ Manager │ │ Manager │ │ Manager │ │ Manager │   │
│  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘   │
│       └─────────────┴─────────┴─────────────┘       │
│                   Device Layer                       │
└─────────────────────────────────────────────────────┘
```

### 模块职责

**前端模块:**
- `stores/`: 全局状态管理
- `components/`: UI组件
- `services/`: API调用封装
- `hooks/`: 自定义React Hooks

**后端模块:**
- `commands/`: Tauri命令处理器
- `devices/`: 硬件设备管理
- `state/`: 应用状态管理
- `utils/`: 工具函数

---

## 开发流程

### 启动开发服务器

```bash
# 同时启动前端和后端
npm run tauri:dev

# 仅前端
npm run dev

# 仅后端
cd src-tauri && cargo run
```

### 代码规范检查

```bash
# 前端
npm run lint
npm run format:check
npm run typecheck

# 后端
cd src-tauri && cargo fmt -- --check
cd src-tauri && cargo clippy -- -D warnings
```

### 运行测试

```bash
# 单元测试
npm run test

# E2E测试
npm run test:e2e

# Rust测试
cd src-tauri && cargo test
```

### 提交代码

```bash
# 1. 检查代码规范
npm run lint
npm run format:check

# 2. 运行测试
npm run test

# 3. 提交 (遵循 Conventional Commits)
git commit -m "feat(serial): add auto-detect feature"
git commit -m "fix(gpio): fix pin toggle issue"
git commit -m "docs(readme): update installation guide"
```

---

## 性能优化

### 前端优化

**1. 组件优化:**
```tsx
// 使用 React.memo 避免不必要的重渲染
const MyComponent = React.memo(({ data }) => {
  return <div>{data}</div>;
});

// 使用 useMemo 缓存计算结果
const processedData = useMemo(() => {
  return data.map(item => expensiveOperation(item));
}, [data]);

// 使用 useCallback 缓存回调函数
const handleClick = useCallback(() => {
  doSomething();
}, []);
```

**2. 状态管理优化:**
```tsx
// 使用选择器避免不必要的订阅
const userName = useUserStore(state => state.user.name);

// 而不是订阅整个 state
const { user } = useUserStore();
```

**3. 列表虚拟化:**
```tsx
import { FixedSizeList } from 'react-window';

<FixedSizeList
  height={400}
  itemCount={items.length}
  itemSize={50}
>
  {Row}
</FixedSizeList>
```

### 后端优化

**1. 异步优化:**
```rust
// 使用 tokio::spawn 创建异步任务
let handle = tokio::spawn(async move {
    process_data().await
});

// 使用 tokio::join! 并行执行
let (a, b) = tokio::join!(
    task_a(),
    task_b()
);
```

**2. 内存优化:**
```rust
// 使用 #[inline] 内联小函数
#[inline]
pub fn get_config(&self) -> &Config {
    &self.config
}

// 使用 Bytes 避免内存拷贝
use bytes::Bytes;
let data: Bytes = Bytes::from_static(b"data");
```

**3. 二进制通信:**
```rust
// 使用二进制协议替代 JSON
let packet = BinaryCodec::encode_serial_data(&data, sequence);
window.emit("serial_data", packet)?;
```

---

## 调试技巧

### 前端调试

**1. React DevTools:**
- 安装浏览器扩展
- 检查组件树和 props
- 追踪状态变化

**2. Redux DevTools (Zustand):**
```ts
import { devtools } from 'zustand/middleware';

const useStore = create(devtools((set) => ({
  // ...
})));
```

**3. 日志记录:**
```ts
// 使用 useLogStore 记录日志
const { addLog } = useLogStore();
addLog('debug', 'Component', 'Debug message', { data });
```

### 后端调试

**1. 启用详细日志:**
```bash
RUST_LOG=debug cargo run
```

**2. 使用 tracing:**
```rust
use tracing::{info, debug, error};

#[tracing::instrument]
pub async fn process_data() {
    info!("Processing data");
    // ...
}
```

**3. 性能分析:**
```bash
# 使用 cargo flamegraph
cargo install flamegraph
cargo flamegraph
```

---

## 发布流程

### 版本号规则

遵循 [Semantic Versioning](https://semver.org/):
- `MAJOR.MINOR.PATCH`
- 例如: `2.0.0`, `2.1.0`, `2.1.1`

### 发布步骤

```bash
# 1. 更新版本号
npm version patch|minor|major

# 2. 更新 CHANGELOG.md
# 添加版本更新内容

# 3. 构建发布版本
npm run release

# 4. 创建 Git 标签
git tag -a v2.0.0 -m "Release version 2.0.0"
git push origin v2.0.0

# 5. 发布到 GitHub
# CI/CD 会自动构建并发布
```

### 多环境配置

| 环境 | 配置文件 | 用途 |
|------|----------|------|
| 开发 | `tauri.dev.conf.json` | 本地开发，启用开发者工具 |
| 测试 | `tauri.test.conf.json` | 自动化测试 |
| 生产 | `tauri.prod.conf.json` | 正式发布版本 |

---

## 常见问题

### Q: 编译失败

**A:** 
1. 检查 Rust 版本: `rustc --version` (>= 1.75)
2. 更新依赖: `cargo update`
3. 清理缓存: `cargo clean && npm run tauri:build`

### Q: 串口无法访问

**A:**
- Linux: 添加用户到 `dialout` 组: `sudo usermod -a -G dialout $USER`
- macOS: 安装驱动程序
- Windows: 检查设备管理器

### Q: 前端热更新失效

**A:**
1. 检查 Vite 配置
2. 重启开发服务器
3. 清除浏览器缓存

---

## 参考资源

- [Tauri 文档](https://tauri.app/v1/guides/)
- [React 文档](https://react.dev/)
- [Rust 文档](https://doc.rust-lang.org/)
- [Material-UI 文档](https://mui.com/material-ui/getting-started/)