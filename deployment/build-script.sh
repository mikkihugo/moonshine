#!/bin/bash
# Moon Shine AI Linter - Production Build Script
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}"
}

warn() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] WARNING: $1${NC}"
}

error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ERROR: $1${NC}"
    exit 1
}

# Check prerequisites
log "Checking prerequisites..."
command -v rustup >/dev/null 2>&1 || error "rustup is required"
command -v cargo >/dev/null 2>&1 || error "cargo is required"

# Ensure WASM target is installed
log "Installing WASM target..."
rustup target add wasm32-unknown-unknown

# Install wasm-opt for optimization
log "Installing wasm-opt..."
cargo install wasm-opt || warn "wasm-opt installation failed, continuing without optimization"

# Clean previous builds
log "Cleaning previous builds..."
cargo clean

# Build WASM extension
log "Building WASM extension for production..."
export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER=lld
cargo build --target wasm32-unknown-unknown --release --features wasm

# Check if build succeeded
if [ -f "target/wasm32-unknown-unknown/release/moon_shine.wasm" ]; then
    log "WASM build successful!"

    # Get file size
    SIZE=$(stat -f%z "target/wasm32-unknown-unknown/release/moon_shine.wasm" 2>/dev/null || stat -c%s "target/wasm32-unknown-unknown/release/moon_shine.wasm")
    log "WASM file size: $SIZE bytes"

    # Optimize WASM binary
    if command -v wasm-opt >/dev/null 2>&1; then
        log "Optimizing WASM binary..."
        wasm-opt -Oz --enable-bulk-memory target/wasm32-unknown-unknown/release/moon_shine.wasm -o target/wasm32-unknown-unknown/release/moon_shine_optimized.wasm

        # Check optimized size
        if [ -f "target/wasm32-unknown-unknown/release/moon_shine_optimized.wasm" ]; then
            OPT_SIZE=$(stat -f%z "target/wasm32-unknown-unknown/release/moon_shine_optimized.wasm" 2>/dev/null || stat -c%s "target/wasm32-unknown-unknown/release/moon_shine_optimized.wasm")
            log "Optimized WASM file size: $OPT_SIZE bytes"
            SAVINGS=$((SIZE - OPT_SIZE))
            log "Size reduction: $SAVINGS bytes ($(( SAVINGS * 100 / SIZE ))%)"
        fi
    fi

    # Copy to dist directory
    log "Copying WASM files to dist directory..."
    mkdir -p dist
    cp target/wasm32-unknown-unknown/release/moon_shine.wasm dist/
    [ -f "target/wasm32-unknown-unknown/release/moon_shine_optimized.wasm" ] && cp target/wasm32-unknown-unknown/release/moon_shine_optimized.wasm dist/

    log "Build complete! WASM files available in dist/ directory"
else
    error "WASM build failed!"
fi