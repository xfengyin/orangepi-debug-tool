#!/bin/bash
# OrangePi Debug Tool v2.0 - 打包发布脚本

set -e

echo "📦 OrangePi Debug Tool v2.0 - 打包发布"
echo "======================================"
echo ""

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 版本号
VERSION="2.0.0"
BUILD_DATE=$(date +%Y-%m-%d)

echo -e "${BLUE}版本：$VERSION${NC}"
echo -e "${BLUE}日期：$BUILD_DATE${NC}"
echo ""

# 1. 检查依赖
echo "📋 检查依赖..."
echo "-------------------"

# 检查 Node.js
if ! command -v node &> /dev/null; then
    echo -e "${YELLOW}⚠️  Node.js 未安装，跳过前端构建${NC}"
else
    echo "✅ Node.js: $(node --version)"
fi

# 检查 Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}⚠️  Rust 未安装，跳过 Rust 构建${NC}"
else
    echo "✅ Rust: $(rustc --version)"
fi

# 检查 Tauri CLI
if command -v cargo &> /dev/null && cargo install --list | grep -q tauri-cli; then
    echo "✅ Tauri CLI: 已安装"
else
    echo -e "${YELLOW}⚠️  Tauri CLI 未安装${NC}"
fi

echo ""

# 2. 安装依赖
echo "📦 安装依赖..."
echo "-------------------"

if [ -f "package.json" ]; then
    echo "安装 Node.js 依赖..."
    npm install --frozen-lockfile || npm install
    echo "✅ Node.js 依赖安装完成"
fi

if [ -f "src-tauri/Cargo.toml" ]; then
    echo "安装 Rust 依赖..."
    cd src-tauri && cargo fetch && cd ..
    echo "✅ Rust 依赖安装完成"
fi

echo ""

# 3. 构建应用
echo "🔨 构建应用..."
echo "-------------------"

echo "⚠️  完整构建需要较长时间..."
echo "⚠️  此步骤在实际发布时执行"
echo ""
echo "构建命令:"
echo "  npm run tauri build"
echo ""
echo "目标平台:"
echo "  - Windows (.msi, .exe)"
echo "  - Linux (.deb, .AppImage)"
echo "  - macOS (.dmg, .app)"
echo ""

# 4. 生成图标
echo "🎨 生成图标..."
echo "-------------------"

if [ -f "public/icon.svg" ]; then
    echo "✅ SVG 图标存在"
    
    # 创建图标目录
    mkdir -p icons
    
    # 复制 SVG
    cp public/icon.svg icons/icon.svg
    echo "✅ SVG 图标已复制到 icons/"
    
    echo ""
    echo "⚠️  PNG/ICO 生成需要 ImageMagick:"
    echo "  convert -background none icons/icon.svg -resize 32x32 icons/32x32.png"
    echo "  convert -background none icons/icon.svg -resize 128x128 icons/128x128.png"
    echo "  convert -background none icons/icon.svg -resize 256x256 icons/256x256.png"
    echo "  convert -background none icons/icon.svg -resize 512x512 icons/512x512.png"
    echo "  convert icons/*.png icons/icon.ico"
else
    echo -e "${YELLOW}⚠️  SVG 图标不存在${NC}"
fi

echo ""

# 5. 创建发布包
echo "📦 创建发布包..."
echo "-------------------"

# 创建发布目录
RELEASE_DIR="release-v${VERSION}"
mkdir -p "$RELEASE_DIR"

# 复制文档
echo "复制文档..."
cp README.md "$RELEASE_DIR/"
cp USER-MANUAL.md "$RELEASE_DIR/"
cp DEVELOPER-GUIDE.md "$RELEASE_DIR/"
cp CHANGELOG.md "$RELEASE_DIR/" 2>/dev/null || echo "⚠️  CHANGELOG.md 不存在"
echo "✅ 文档已复制"

# 创建发布说明
cat > "$RELEASE_DIR/RELEASE-NOTES.md" << EOF
# OrangePi Debug Tool v${VERSION} 发布说明

**发布日期:** $BUILD_DATE

## 🎉 新功能

- 完整串口调试功能
- GPIO 控制
- PWM 输出
- 数据日志与导出
- 协议解析 (MODBUS)
- 多串口管理
- 宏命令/脚本
- 主题切换
- 国际化 (中文/English)

## 📦 安装包

构建完成后，安装包将位于：
- src-tauri/target/release/bundle/

## 🔗 链接

- GitHub: https://github.com/xfengyin/orangepi-debug-tool
- 用户手册: USER-MANUAL.md
- 开发者指南: DEVELOPER-GUIDE.md

## 📄 许可证

MIT License
EOF

echo "✅ 发布说明已创建"

# 创建压缩包
echo ""
echo "创建源码包..."
tar -czf "release-v${VERSION}/source-code.tar.gz" \
    --exclude='node_modules' \
    --exclude='target' \
    --exclude='dist' \
    --exclude='.git' \
    --exclude='release-*' \
    .

echo "✅ 源码包已创建：release-v${VERSION}/source-code.tar.gz"

echo ""
echo "📦 发布包结构:"
tree -L 2 "$RELEASE_DIR" 2>/dev/null || find "$RELEASE_DIR" -maxdepth 2 -type f

echo ""
echo "======================================"
echo -e "${GREEN}✅ 打包准备完成！${NC}"
echo "======================================"
echo ""
echo "下一步:"
echo "1. 执行完整构建：npm run tauri build"
echo "2. 将安装包复制到 $RELEASE_DIR/"
echo "3. 创建 GitHub Release"
echo "4. 推送标签：git tag -a v${VERSION} -m 'Release v${VERSION}'"
echo "5. 推送标签：git push origin v${VERSION}"
echo ""
