#!/bin/bash

# Pre-commit checks script

set -e

echo "Running pre-commit checks..."

# Frontend checks
echo ""
echo "=== Frontend Checks ==="

# Lint
echo "Running ESLint..."
npm run lint

# Format check
echo "Checking Prettier formatting..."
npm run format:check

# TypeScript check
echo "Running TypeScript check..."
npm run typecheck

# Unit tests
echo "Running unit tests..."
npm run test

# Backend checks
echo ""
echo "=== Backend Checks ==="
cd src-tauri

# Format check
echo "Running cargo fmt..."
cargo fmt -- --check

# Clippy
echo "Running cargo clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Tests
echo "Running cargo test..."
cargo test

cd ..

echo ""
echo "All checks passed!"