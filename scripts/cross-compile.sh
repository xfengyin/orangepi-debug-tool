#!/bin/bash
set -e

# 交叉编译脚本 for OrangePi ARM Linux

export PKG_CONFIG_SYSROOT_DIR="/path/to/arm-sysroot"
export CC="arm-linux-gnueabihf-gcc"
export CXX="arm-linux-gnueabihf-g++"
export AR="arm-linux-gnueabihf-ar"
export RANLIB="arm-linux-gnueabihf-ranlib"

echo "Building for ARM (armv7-unknown-linux-gnueabihf)..."
cargo build --release --target armv7-unknown-linux-gnueabihf

echo "Building for ARM64 (aarch64-unknown-linux-gnu)..."
cargo build --release --target aarch64-unknown-linux-gnu

echo "Build complete!"
echo "Output files:"
echo "  - target/armv7-unknown-linux-gnueabihf/release/orangepi-debug-tool"
echo "  - target/aarch64-unknown-linux-gnu/release/orangepi-debug-tool"
