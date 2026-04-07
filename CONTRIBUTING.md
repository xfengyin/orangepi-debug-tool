# 贡献指南

感谢您对 OrangePi Debug Tool 项目的关注！我们非常欢迎各种形式的贡献，包括但不限于代码提交、问题反馈、文档改进等。

## 目录

- [行为准则](#行为准则)
- [快速开始](#快速开始)
- [开发环境](#开发环境)
- [开发流程](#开发流程)
- [代码规范](#代码规范)
- [提交规范](#提交规范)
- [测试要求](#测试要求)
- [文档贡献](#文档贡献)
- [问题反馈](#问题反馈)
- [版本发布](#版本发布)

## 行为准则

参与本项目的所有贡献者都必须遵守我们的 [行为准则](./CODE_OF_CONDUCT.md)。我们致力于为所有人提供一个友好、安全和热情的环境。

## 快速开始

### Fork 项目

1. 点击 GitHub 页面右上角的 **Fork** 按钮
2. 克隆您 fork 的仓库到本地：
   ```bash
   git clone https://github.com/<your-username>/orangepi-debug-tool-v2.git
   cd orangepi-debug-tool-v2
   ```

### 添加上游仓库

```bash
git remote add upstream https://github.com/orangepi-xunlong/orangepi-debug-tool-v2.git
```

### 创建开发分支

```bash
git checkout -b feature/your-feature-name
# 或
git checkout -b fix/your-bug-fix
```

## 开发环境

### 系统要求

| 组件 | 最低版本 | 推荐版本 |
|------|----------|----------|
| Rust | 1.70 | 1.75+ |
| Node.js | 16 | 18+ |
| npm | 8 | 9+ |

### 安装依赖

#### Ubuntu / Debian

```bash
# 安装系统依赖
sudo apt-get update
sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    pkg-config \
    git

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 安装 Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# 验证安装
rustc --version
node --version
npm --version
```

#### macOS

```bash
# 使用 Homebrew 安装依赖
brew install rust node

# 安装 Tauri CLI
cargo install tauri-cli
```

#### Windows

1. 安装 [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
2. 安装 [Rust](https://rustup.rs/)
3. 安装 [Node.js](https://nodejs.org/)
4. 安装 [Visual Studio Code](https://code.visualstudio.com/)（推荐）

### 项目设置

```bash
# 克隆仓库
git clone https://github.com/orangepi-xunlong/orangepi-debug-tool-v2.git
cd orangepi-debug-tool-v2

# 安装前端依赖
npm install

# 启动开发服务器
npm run tauri dev
```

## 开发流程

### 1. 选择任务

- 查看 [Issues](https://github.com/orangepi-xunlong/orangepi-debug-tool-v2/issues) 列表
- 选择您感兴趣或擅长处理的问题
- 在 issue 下留言告知您将处理此问题
- 等待维护者确认分配

### 2. 创建分支

```bash
# 确保主分支是最新的
git checkout main
git pull upstream main

# 创建功能分支
git checkout -b feature/serial-improvements
# 或
git checkout -b fix/uart-timeout-bug
```

### 3. 进行开发

#### 前端开发（Vue/React）

```bash
# 前端热重载开发
npm run dev

# 代码检查
npm run lint

# 格式化代码
npm run format
```

#### 后端开发（Rust/Tauri）

```bash
# Rust 格式化
cargo fmt

# Rust 代码检查
cargo clippy -- -D warnings

# 运行测试
cargo test
```

### 4. 编写测试

```bash
# 运行所有测试
npm test
# 或
cargo test

# 运行测试并显示输出
cargo test -- --nocapture

# 运行特定测试
cargo test test_serial_port
```

### 5. 提交代码

```bash
# 添加更改的文件
git add .

# 提交（使用规范提交信息）
git commit -m "feat(serial): 添加串口自动检测功能

- 实现串口热插拔检测
- 添加串口状态变更通知
- 更新 UI 以显示串口连接状态
- 添加单元测试"

# 推送分支到远程
git push origin feature/serial-improvements
```

### 6. 创建 Pull Request

1. 在 GitHub 上打开您的仓库
2. 点击 **Compare & pull request** 按钮
3. 填写 PR 模板中的所有必要信息
4. 关联相关 Issue（如有）
5. 等待代码审查

### 7. 代码审查

维护者将审查您的 PR，可能要求：

- 修改代码
- 添加更多测试
- 更新文档
- 解释某些设计决策

请及时响应审查意见！

### 8. 合并代码

一旦 PR 通过审查，维护者将合并代码。感谢您的贡献！

## 代码规范

### Rust 代码规范

- 遵循 [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 为公共 API 编写文档注释
- 编写单元测试和集成测试

```rust
/// 打开指定串口并返回串口连接
///
/// # 参数
///
/// * `port_name` - 串口名称，如 "/dev/ttyUSB0"
/// * `config` - 串口配置
///
/// # 示例
///
/// ```
/// use orangepi_debug_tool::serial;
///
/// let config = serial::Config::default();
/// let port = serial::open("/dev/ttyUSB0", &config);
/// ```
pub fn open(port_name: &str, config: &Config) -> Result<Port> {
    // ...
}
```

### 前端代码规范

- 遵循 ESLint 配置规则
- 使用 TypeScript 进行类型安全开发
- 组件使用 PascalCase 命名
- 函数和变量使用 camelCase 命名
- 常量使用 UPPER_SNAKE_CASE 命名

### 提交规范

本项目使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<type>(<scope>): <subject>

[optional body]

[optional footer]
```

#### 类型（Type）

| 类型 | 描述 |
|------|------|
| feat | 新功能 |
| fix | Bug 修复 |
| docs | 文档更改 |
| style | 代码格式（不影响功能） |
| refactor | 重构（不影响功能） |
| perf | 性能优化 |
| test | 测试相关 |
| build | 构建系统或依赖更改 |
| ci | CI/CD 配置更改 |
| chore | 其他更改 |

#### 范围（Scope）

| 范围 | 描述 |
|------|------|
| serial | 串口相关 |
| gpio | GPIO 相关 |
| pwm | PWM 相关 |
| ui | 用户界面 |
| plugin | 插件系统 |
| docs | 文档 |
| test | 测试 |
| ci | CI/CD |
| deps | 依赖更新 |

#### 示例

```
feat(serial): 添加波特率自动检测功能

fix(gpio): 修复 GPIO 引脚状态读取错误

docs(readme): 更新安装说明

refactor(ui): 重构串口配置组件

ci: 添加 GitHub Actions 构建流程
```

## 测试要求

### 测试策略

1. **单元测试**：测试独立函数和模块
2. **集成测试**：测试组件之间的交互
3. **端到端测试**：测试完整的用户流程

### 运行测试

```bash
# 运行所有测试
npm test

# 前端测试
npm run test:unit
npm run test:e2e

# Rust 测试
cargo test

# 带详细输出
cargo test -- --nocapture

# 只运行文档测试
cargo test --doc
```

### 测试覆盖率

- 核心功能测试覆盖率应达到 80%+
- 新功能必须包含测试
- Bug 修复必须包含回归测试

## 文档贡献

### 文档类型

1. **README.md**：项目主文档
2. **CONTRIBUTING.md**：贡献指南
3. **CODE_OF_CONDUCT.md**：行为准则
4. **CHANGELOG.md**：版本更新日志
5. **API 文档**：代码内文档注释
6. **Wiki**：详细的用户指南

### 文档更新

- 新功能必须包含使用文档
- API 更改必须更新 API 文档
- 破坏性更改必须在 PR 中详细说明

## 问题反馈

### 创建 Issue

请使用 Issue 模板创建问题报告或功能请求：

- [Bug Report](./.github/ISSUE_TEMPLATE/bug_report.md)
- [Feature Request](./.github/ISSUE_TEMPLATE/feature_request.md)

### Issue 规范

- 搜索现有 Issue 避免重复
- 提供详细的复现步骤
- 提供环境信息（OS、Rust 版本、Node 版本等）
- 附上相关日志和截图

## 版本发布

版本发布由维护者负责，遵循 [语义化版本](https://semver.org/) 规范：

- **主版本 (X.y.z)**：破坏性 API 更改
- **次版本 (x.Y.z)**：新功能（向后兼容）
- **补丁版本 (x.y.Z)**：Bug 修复（向后兼容）

### 发布流程

1. 更新 CHANGELOG.md
2. 创建 GitHub Release
3. 自动构建和发布二进制文件

## 常见问题

### Q: 如何配置串口权限？

**Linux:**
```bash
# 添加用户到 dialout 组
sudo usermod -a -G dialout $USER
# 重新登录使更改生效
```

### Q: 开发时遇到编译错误怎么办？

1. 确保 Rust 和 Node.js 版本正确
2. 清理缓存：`cargo clean && rm -rf node_modules`
3. 重新安装依赖：`npm install`
4. 重新构建

### Q: 如何调试 Tauri 应用？

```bash
# 启用 DevTools
npm run tauri dev -- --debug

# 查看 Rust 控制台日志
RUST_BACKTRACE=1 npm run tauri dev
```

## 联系方式

- GitHub Issues: [提交问题](https://github.com/orangepi-xunlong/orangepi-debug-tool-v2/issues)
- 论坛: [OrangePi 社区](https://forum.orangepi.org/)

## 致谢

感谢所有为 OrangePi Debug Tool 做出贡献的开发者！

<a href="https://github.com/orangepi-xunlong/orangepi-debug-tool-v2/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=orangepi-xunlong/orangepi-debug-tool-v2" />
</a>

---

再次感谢您的贡献！🎉
