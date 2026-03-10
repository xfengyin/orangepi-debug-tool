# 📦 OrangePi Debug Tool v2.0 - 多平台构建指南

---

## 🎯 自动构建 (推荐)

### GitHub Actions

项目已配置 GitHub Actions 工作流，在创建 Release 标签时自动构建所有平台的安装包。

#### 触发方式

```bash
# 1. 创建 Release 标签
git tag -a v2.0.0 -m "Release v2.0.0"
git push origin v2.0.0

# 或手动触发
# 访问 https://github.com/xfengyin/orangepi-debug-tool/actions
# 选择 "Build & Release" 工作流
# 点击 "Run workflow"
```

#### 构建流程

```
创建 Release 标签
    ↓
触发 GitHub Actions
    ↓
并行构建:
  - Windows (MSI + EXE)
  - Linux (DEB + AppImage)
  - macOS (DMG + APP)
    ↓
上传安装包到 Release
    ↓
自动生成 Release Notes
```

#### 构建产物

**Windows:**
- `OrangePi.Debug.Tool_2.0.0_x64_en-US.msi` - Windows Installer
- `OrangePi.Debug.Tool_2.0.0_x64-setup.exe` - NSIS 安装程序

**Linux:**
- `orangepi-debug-tool_2.0.0_amd64.deb` - Debian/Ubuntu
- `orangepi-debug-tool_2.0.0_x86_64.AppImage` - 通用 Linux

**macOS:**
- `OrangePi.Debug.Tool_2.0.0_x64.dmg` - macOS Disk Image
- `OrangePi.Debug.Tool.app` - macOS Application

---

## 🛠️ 手动构建

### 环境要求

**通用:**
- Node.js 18+
- Rust 1.70+
- npm 或 yarn

**Windows:**
- Visual Studio 2022 Build Tools
- WebView2

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

**macOS:**
- Xcode Command Line Tools

### 构建步骤

#### 1. 安装依赖

```bash
# 安装 Node.js 依赖
npm install

# 安装 Rust 依赖
cd src-tauri
cargo fetch
cd ..
```

#### 2. 构建应用

```bash
# 构建当前平台
npm run tauri build

# 或指定平台
npm run tauri build -- --target x86_64-pc-windows-msvc  # Windows
npm run tauri build -- --target x86_64-unknown-linux-gnu  # Linux
npm run tauri build -- --target x86_64-apple-darwin  # macOS
```

#### 3. 查找安装包

构建完成后，安装包位于：

```
src-tauri/target/release/bundle/
├── msi/           # Windows MSI
├── nsis/          # Windows EXE
├── deb/           # Linux DEB
├── appimage/      # Linux AppImage
├── dmg/           # macOS DMG
└── macos/         # macOS APP
```

---

## 📊 构建配置

### Tauri 配置 (tauri.conf.json)

```json
{
  "package": {
    "productName": "OrangePi Debug Tool",
    "version": "2.0.0"
  },
  "tauri": {
    "bundle": {
      "active": true,
      "targets": ["msi", "nsis", "deb", "appimage", "dmg"],
      "identifier": "com.xfengyin.orangepi-debug-tool",
      "category": "DeveloperTool",
      "shortDescription": "OrangePi 调试工具",
      "longDescription": "现代化 OrangePi 调试工具，支持串口调试、GPIO 控制、PWM 输出和数据可视化"
    }
  }
}
```

### GitHub Actions 配置

**工作流文件:** `.github/workflows/build-release.yml`

**构建矩阵:**
- Windows: `windows-latest`
- Linux: `ubuntu-22.04`
- macOS: `macos-latest`

**产物保留:** 90 天

---

## 🔧 自定义构建

### 修改版本号

**package.json:**
```json
{
  "version": "2.0.0"
}
```

**src-tauri/Cargo.toml:**
```toml
[package]
version = "2.0.0"
```

**src-tauri/tauri.conf.json:**
```json
{
  "package": {
    "version": "2.0.0"
  }
}
```

### 修改应用图标

1. 替换 `public/icon.svg`
2. 生成多尺寸 PNG:
   ```bash
   convert icon.svg -resize 32x32 icons/32x32.png
   convert icon.svg -resize 128x128 icons/128x128.png
   convert icon.svg -resize 256x256 icons/256x256.png
   convert icon.svg -resize 512x512 icons/512x512.png
   ```
3. 生成 ICO/ICNS:
   ```bash
   convert icons/*.png icons/icon.ico
   icnsutil -o icons/icon.icns icons/
   ```

### 修改应用名称

**tauri.conf.json:**
```json
{
  "package": {
    "productName": "Your App Name"
  },
  "tauri": {
    "bundle": {
      "identifier": "com.yourcompany.your-app"
    }
  }
}
```

---

## 📦 安装包说明

### Windows

**MSI 安装包:**
- 支持 Windows Installer
- 可集中部署
- 支持卸载

**EXE 安装包:**
- NSIS 制作
- 单文件
- 支持静默安装

### Linux

**DEB 包:**
- 适用于 Debian/Ubuntu
- 使用 `dpkg -i` 安装
- 自动处理依赖

**AppImage:**
- 通用 Linux 格式
- 无需安装
- 双击运行

### macOS

**DMG:**
- macOS Disk Image
- 拖拽安装
- 标准 macOS 格式

**APP:**
- 直接运行
- 适合测试

---

## 🚀 发布流程

### 1. 测试通过

```bash
# 运行测试
bash scripts/test.sh

# 查看测试报告
cat TEST-REPORT.md
```

### 2. 创建 Release 标签

```bash
git tag -a v2.0.0 -m "Release v2.0.0"
git push origin v2.0.0
```

### 3. 等待构建完成

访问 https://github.com/xfengyin/orangepi-debug-tool/actions

### 4. 下载安装包

访问 https://github.com/xfengyin/orangepi-debug-tool/releases

### 5. 发布公告

- GitHub Releases
- 社交媒体
- 社区论坛

---

## 📊 构建时间估算

| 平台 | 预计时间 |
|------|----------|
| Windows | ~15 分钟 |
| Linux | ~10 分钟 |
| macOS | ~20 分钟 |
| **总计** | ~20 分钟 (并行) |

---

## ⚠️ 常见问题

### Q: 构建失败？

**A:** 检查:
1. 依赖是否完整安装
2. 系统版本是否符合要求
3. 查看构建日志

### Q: 安装包太大？

**A:** 优化:
1. 启用 LTO (Link Time Optimization)
2. 移除未使用的依赖
3. 压缩资源文件

### Q: 签名问题？

**A:** Windows/macOS 需要代码签名:
1. 购买代码签名证书
2. 在 CI/CD 中配置签名密钥
3. 或使用自签名证书 (仅限测试)

---

_OrangePi Debug Tool v2.0 - 构建指南_
