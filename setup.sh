#!/bin/bash
#
# Moon Shine Development Environment Setup
# This script configures the proper development environment for the Moon Shine project
#

set -e

echo "🌙 Setting up Moon Shine development environment..."

# Activate proto environment
echo "📦 Activating proto toolchain..."
eval "$(proto activate bash --export)"

# Add Rust toolchain to PATH (proto rust plugin doesn't create shims)
RUST_TOOLCHAIN_PATH="$HOME/.rustup/toolchains/1.89.0-x86_64-unknown-linux-gnu/bin"
if [[ -d "$RUST_TOOLCHAIN_PATH" ]]; then
    export PATH="$RUST_TOOLCHAIN_PATH:$PATH"
fi

# Verify tools are available
echo "🔍 Verifying tools..."
if command -v cargo >/dev/null 2>&1; then
    echo "✅ Cargo $(cargo --version)"
else
    echo "❌ Cargo not found"
    exit 1
fi

if command -v rustc >/dev/null 2>&1; then
    echo "✅ Rustc $(rustc --version)"
else
    echo "❌ Rustc not found"
    exit 1
fi

if command -v moon >/dev/null 2>&1; then
    echo "✅ Moon $(moon --version)"
else
    echo "❌ Moon not found"
    exit 1
fi

echo "🎉 Environment ready! You can now run:"
echo "  moon run moon-shine:build"
echo "  moon run moon-shine:test"
echo "  moon run moon-shine:lint"