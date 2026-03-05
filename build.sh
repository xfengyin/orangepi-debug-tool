#!/bin/bash
# OrangePi Debug Tool 构建脚本

set -e

echo "=========================================="
echo "  OrangePi Debug Tool - Build Script"
echo "=========================================="

# 检查 Python
if ! command -v python3 &> /dev/null; then
    echo "Error: Python3 not found"
    exit 1
fi

# 安装依赖
echo "[1/3] Installing dependencies..."
pip install pyinstaller
pip install -r requirements.txt || true

# 构建
echo "[2/3] Building executable..."
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    # Windows
    pyinstaller --onefile --windowed --name "OrangePiDebugTool" orange_debug_tool.py
else
    # Linux/macOS
    pyinstaller --onefile --windowed --name "OrangePiDebugTool" orange_debug_tool.py
fi

# 完成
echo "[3/3] Build complete!"
echo "Output: dist/OrangePiDebugTool*"
ls -la dist/
