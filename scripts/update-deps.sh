#!/bin/bash

# Update dependencies script

set -e

echo "Updating OrangePi Debug Tool dependencies..."

# Update Node.js dependencies
echo "Updating Node.js dependencies..."
npm update

# Update Rust dependencies
echo "Updating Rust dependencies..."
cd src-tauri
cargo update
cd ..

# Check for outdated packages
echo ""
echo "Checking for outdated npm packages..."
npm outdated || true

echo ""
echo "Checking for outdated cargo packages..."
cd src-tauri
cargo outdated || true
cd ..

echo ""
echo "Update complete!"