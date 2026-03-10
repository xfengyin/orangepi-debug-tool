# 发布清单

---

## 📦 发布版本

**版本:** v2.0.0  
**发布日期:** 2026-03-10  
**类型:** Stable Release

---

## ✅ 发布前检查清单

### 代码质量

- [ ] 所有单元测试通过
- [ ] 代码审查完成
- [ ] 无已知严重 Bug
- [ ] 性能测试通过
- [ ] 内存泄漏检查

### 文档

- [x] README.md 更新
- [x] USER-MANUAL.md 完成
- [x] DEVELOPER-GUIDE.md 完成
- [x] CHANGELOG.md 更新
- [x] 图标设计完成

### 构建

- [ ] Windows 构建 (.msi, .exe)
- [ ] Linux 构建 (.deb, .AppImage)
- [ ] macOS 构建 (.dmg, .app)
- [ ] 构建日志检查
- [ ] 安装包大小检查 (<50MB)

### 测试

- [ ] 功能测试完成
- [ ] 兼容性测试完成
- [ ] 性能测试完成
- [ ] 用户验收测试

### GitHub

- [ ] Release Notes 编写
- [ ] Tag 创建 (v2.0.0)
- [ ] 上传安装包
- [ ] 更新仓库描述

---

## 📝 发布说明模板

### OrangePi Debug Tool v2.0.0

**发布日期:** 2026-03-10

#### 🎉 新功能

- **完整串口调试** - 支持所有标准串口配置
- **GPIO 控制** - 可视化引脚管理
- **PWM 输出** - 频率/占空比可调
- **数据日志** - 实时波形记录与导出
- **协议解析** - MODBUS RTU 支持
- **多串口管理** - 同时连接 8 个串口
- **宏命令/脚本** - 自动化任务
- **主题切换** - 浅色/深色模式
- **国际化** - 中文/English

#### 🐛 Bug 修复

- 修复了已知问题

#### 📈 性能提升

- 启动时间：<0.5s
- 内存占用：<50MB
- 串口响应：<5ms

#### 📦 安装包

**Windows:**
- `OrangePi.Debug.Tool_2.0.0_x64_en-US.msi`
- `OrangePi.Debug.Tool_2.0.0_x64-setup.exe`

**Linux:**
- `orangepi-debug-tool_2.0.0_amd64.deb`
- `orangepi-debug-tool_2.0.0_x86_64.AppImage`

**macOS:**
- `OrangePi.Debug.Tool_2.0.0_x64.dmg`

#### 🔗 链接

- [下载页面](https://github.com/xfengyin/orangepi-debug-tool/releases/tag/v2.0.0)
- [用户手册](USER-MANUAL.md)
- [开发者指南](DEVELOPER-GUIDE.md)

---

## 🚀 发布步骤

### 1. 版本号更新

```json
// package.json
{
  "version": "2.0.0"
}

// src-tauri/Cargo.toml
[package]
version = "2.0.0"

// src-tauri/tauri.conf.json
{
  "package": {
    "version": "2.0.0"
  }
}
```

### 2. 构建所有平台

```bash
# Windows
npm run tauri build -- --target x86_64-pc-windows-msvc

# Linux
npm run tauri build -- --target x86_64-unknown-linux-gnu

# macOS
npm run tauri build -- --target x86_64-apple-darwin
```

### 3. 创建 Git Tag

```bash
git tag -a v2.0.0 -m "Release v2.0.0 - Stable Release"
git push origin v2.0.0
```

### 4. 创建 GitHub Release

1. 访问 https://github.com/xfengyin/orangepi-debug-tool/releases/new
2. Tag version: `v2.0.0`
3. Release title: `OrangePi Debug Tool v2.0.0`
4. 粘贴发布说明
5. 上传安装包
6. 点击 "Publish release"

### 5. 更新文档

- [ ] GitHub Pages 更新
- [ ] Wiki 更新
- [ ] 社交媒体公告

---

## 📊 发布后监控

### 第一周

- [ ] 下载量统计
- [ ] 用户反馈收集
- [ ] Bug 报告跟踪
- [ ] 社交媒体提及

### 第一个月

- [ ] 活跃用户统计
- [ ] 问题关闭率
- [ ] 社区增长
- [ ] v2.0.1 规划

---

## 🎯 成功标准

- [ ] 无严重 Bug 报告
- [ ] 用户评分 > 4.5
- [ ] 下载量 > 1000 (首月)
- [ ] 社区活跃度提升

---

_OrangePi Debug Tool v2.0 - 发布清单_
