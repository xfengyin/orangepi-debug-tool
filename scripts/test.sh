#!/bin/bash
# OrangePi Debug Tool v2.0 - 测试脚本

set -e

echo "🧪 OrangePi Debug Tool v2.0 - 功能测试"
echo "========================================"
echo ""

# 颜色定义
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 测试计数器
TESTS_PASSED=0
TESTS_FAILED=0
TESTS_SKIPPED=0

# 测试函数
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    echo -n "📋 测试：$test_name ... "
    
    if eval "$test_command" > /dev/null 2>&1; then
        echo -e "${GREEN}通过${NC}"
        ((TESTS_PASSED++))
        return 0
    else
        echo -e "${RED}失败${NC}"
        ((TESTS_FAILED++))
        return 1
    fi
}

skip_test() {
    local test_name="$1"
    local reason="$2"
    
    echo -e "${YELLOW}⏭️  跳过：$test_name ($reason)${NC}"
    ((TESTS_SKIPPED++))
}

# 1. 检查项目结构
echo "📁 检查项目结构..."
echo "-------------------"

run_test "Rust 后端存在" "test -d src-tauri"
run_test "React 前端存在" "test -d src"
run_test "Cargo.toml 存在" "test -f src-tauri/Cargo.toml"
run_test "package.json 存在" "test -f package.json"
run_test "tauri.conf.json 存在" "test -f src-tauri/tauri.conf.json"

echo ""

# 2. Rust 代码测试
echo "🦀 Rust 代码测试..."
echo "-------------------"

cd src-tauri

run_test "Rust 编译检查" "cargo check"
run_test "Rust 格式检查" "cargo fmt -- --check"

# 如果有测试
if [ -f "src/lib.rs" ] || [ -f "src/main.rs" ]; then
    run_test "Rust 单元测试" "cargo test --lib" || skip_test "Rust 单元测试" "无库测试"
fi

cd ..

echo ""

# 3. 前端代码测试
echo "⚛️  前端代码测试..."
echo "-------------------"

run_test "Node.js 版本" "node --version"
run_test "npm 版本" "npm --version"

if [ -f "package-lock.json" ] || [ -f "yarn.lock" ]; then
    run_test "依赖安装" "npm ls" || skip_test "依赖安装" "依赖未安装"
fi

run_test "TypeScript 检查" "npx tsc --noEmit" || skip_test "TypeScript 检查" "TypeScript 未配置"

echo ""

# 4. 文件完整性检查
echo "📄 文件完整性检查..."
echo "-------------------"

run_test "README.md 存在" "test -f README.md"
run_test "USER-MANUAL.md 存在" "test -f USER-MANUAL.md"
run_test "DEVELOPER-GUIDE.md 存在" "test -f DEVELOPER-GUIDE.md"
run_test "图标文件存在" "test -f public/icon.svg"
run_test "测试计划存在" "test -f TEST-PLAN.md"

echo ""

# 5. 代码质量检查
echo "🔍 代码质量检查..."
echo "-------------------"

# 检查 Rust 代码
RUST_FILES=$(find src-tauri/src -name "*.rs" | wc -l)
echo "📊 Rust 文件数：$RUST_FILES"

# 检查 TypeScript 文件
TS_FILES=$(find src -name "*.tsx" -o -name "*.ts" | wc -l)
echo "📊 TypeScript 文件数：$TS_FILES"

# 检查文档
DOC_FILES=$(find . -name "*.md" | wc -l)
echo "📊 文档文件数：$DOC_FILES"

echo ""

# 6. 构建检查
echo "🔨 构建检查..."
echo "-------------------"

echo "⚠️  完整构建需要较长时间，跳过实际构建"
skip_test "Rust 构建" "构建检查跳过"
skip_test "前端构建" "构建检查跳过"

echo ""

# 输出测试结果
echo "========================================"
echo "📊 测试结果汇总"
echo "========================================"
echo -e "${GREEN}通过：$TESTS_PASSED${NC}"
echo -e "${RED}失败：$TESTS_FAILED${NC}"
echo -e "${YELLOW}跳过：$TESTS_SKIPPED${NC}"
echo ""

TOTAL=$((TESTS_PASSED + TESTS_FAILED + TESTS_SKIPPED))
echo "总计：$TOTAL 个测试"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ 所有测试通过！${NC}"
    exit 0
else
    echo -e "${RED}❌ 有 $TESTS_FAILED 个测试失败${NC}"
    exit 1
fi
