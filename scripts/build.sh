#!/bin/bash
set -e

# 本地构建脚本

echo "Building for local platform..."
cargo build --release

echo "Build complete!"
echo "Output: target/release/orangepi-debug-tool"
