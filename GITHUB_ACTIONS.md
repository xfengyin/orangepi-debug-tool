# GitHub Actions 自动构建与发布指南

本文档详细说明如何使用 GitHub Actions 自动构建和发布 OrangePi Debug Tool 发行版软件。

## 📋 工作流概览

项目配置了三个 GitHub Actions 工作流：

| 工作流 | 触发条件 | 用途 |
|--------|----------|------|
| **CI** | 每次 push/PR 到 main/develop | 代码检查、测试、多平台构建 |
| **Nightly** | 每日 2:00 UTC / 手动触发 | 每日构建预览版本 |
| **Release** | 发布标签 (v*) | 正式版本发布 |

## 🚀 快速开始

### 1. 配置 GitHub Secrets

在 GitHub 仓库 Settings → Secrets and variables → Actions 中添加以下密钥：

#### Tauri 签名密钥（Release 需要）
```bash
# 生成签名密钥对
npm run tauri build -- --bundles none
tauri-keygen -w -k ~/.tauri/key.pem -p <your-password>

# 添加到 GitHub Secrets
TAURI_SIGNING_PRIVATE_KEY: <key.pem 的内容>
TAURI_SIGNING_PRIVATE_KEY_PASSWORD: <your-password>
```

### 2. 发布正式版本

```bash
# 确保所有代码已提交
git add .
git commit -m "Release v2.0.0"

# 创建版本标签
git tag v2.0.0

# 推送标签到 GitHub
git push origin v2.0.0
```

推送标签后，GitHub Actions 将自动：
1. 运行完整的 CI 测试
2. 构建所有目标平台（deb, appimage, msi, dmg）
3. 创建 GitHub Release 草稿
4. 上传所有构建产物

### 3. 查看构建状态

访问 Actions 页面查看构建进度：
```
https://github.com/<username>/orangepi-debug-tool-v2/actions
```

## 📦 构建产物

构建成功后，Release 页面将包含以下文件：

| 平台 | 文件格式 | 说明 |
|------|----------|------|
| Linux | .deb | Debian/Ubuntu 系统包 |
| Linux | .AppImage | 通用 Linux 应用包 |
| Windows | .msi | Windows 安装程序 |
| Windows | .exe | Windows 可执行文件 |
| macOS | .dmg | macOS 磁盘镜像 |
| macOS | .app | macOS 应用包 |

## 🔧 工作流详细说明

### CI 工作流

每次向 main 或 develop 分支推送代码时触发：

```yaml
触发条件:
  - push 到 main/develop
  - PR 到 main/develop

执行步骤:
  1. 单元测试 (test job)
     - TypeScript 类型检查
     - ESLint 代码检查
     - Rust 格式检查 (cargo fmt)
     - Rust Lint 检查 (cargo clippy)
  
  2. 多平台构建 (build job)
     - Ubuntu 构建
     - Windows 构建
     - macOS 构建
     - 上传构建产物（保留 7 天）
```

### Nightly 工作流

每日 UTC 02:00 自动构建：

```yaml
触发条件:
  - 每日 02:00 UTC
  - 手动触发 (workflow_dispatch)

执行步骤:
  1. 生成日期版本号: 2.0.0-nightly.YYYYMMDD
  2. 更新 package.json 版本
  3. 更新 Cargo.toml 版本
  4. 安装依赖并构建
  5. 创建预发布版本 (prerelease)
  6. 上传到 GitHub Release
```

### Release 工作流

推送语义化版本标签时触发：

```yaml
触发条件:
  - git tag v*

执行步骤:
  1. 检出代码
  2. 安装依赖
  3. 构建 Tauri 应用
  4. 使用 tauri-action 自动创建 Release
  5. 上传所有构建产物
```

## ⚙️ 自定义配置

### 修改构建目标平台

编辑 `src-tauri/tauri.conf.json` 中的 bundle 配置：

```json
{
  "bundle": {
    "targets": ["deb", "msi", "dmg", "appimage"]
  }
}
```

### 修改构建环境

编辑 `.github/workflows/*.yml` 中的环境变量：

```yaml
env:
  NODE_VERSION: '20'  # Node.js 版本
  # 可添加其他自定义环境变量
```

## 🐛 常见问题

### 构建失败：找不到 webkit2gtk

确保 CI 工作流中正确安装了依赖：

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev
```

### 签名验证失败

检查 `TAURI_SIGNING_PRIVATE_KEY` 是否正确配置：
- 确保密钥是完整的 PEM 格式
- 确保密码正确

### macOS 构建失败

macOS 构建需要签名和公证配置。在 tauri.conf.json 中配置：

```json
{
  "tauri": {
    "bundle": {
      "macOS": {
        "minimumSystemVersion": "10.13",
        "entitlements": null
      }
    }
  }
}
```

## 📊 监控构建

### 查看构建日志

```bash
# 使用 GitHub CLI
gh run list
gh run view <run-id> --log

# 查看特定 job 日志
gh run view <run-id> --job <job-id>
```

### 下载构建产物

```bash
# 使用 GitHub CLI
gh release view v2.0.0 --repo <owner>/<repo>

# 下载特定资产
gh release download v2.0.0 -p deb -D ./downloads
```

## 🔐 安全最佳实践

1. **密钥管理**：使用 GitHub Secrets 存储所有敏感信息
2. **权限最小化**：工作流只申请必要的 permissions
3. **依赖缓存**：使用 actions/cache 加速构建
4. **定期更新**：保持 GitHub Actions 版本最新

## 📚 参考资源

- [Tauri GitHub Actions 文档](https://tauri.app/distribute/ci/github/)
- [GitHub Actions 官方文档](https://docs.github.com/actions)
- [tauri-action 使用指南](https://github.com/tauri-apps/tauri-action)

## 🎯 下一步

1. 配置 GitHub Secrets
2. 推送第一个测试标签
3. 验证构建和发布流程
4. 配置团队成员权限
