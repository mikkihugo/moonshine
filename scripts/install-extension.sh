#!/usr/bin/env bash
set -euo pipefail

# Moon Shine Extension Installation Script
# Installs the moon-shine WASM extension into a Moon workspace

EXTENSION_NAME="moon-shine"
EXTENSION_VERSION="${MOON_SHINE_VERSION:-$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')}"
MOON_EXTENSIONS_DIR="${MOON_EXTENSIONS_DIR:-$HOME/.moon/extensions}"
FORCE_INSTALL="${FORCE_INSTALL:-false}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_dependencies() {
    log_info "Checking dependencies..."

    # Check if Moon is installed
    if ! command -v moon &> /dev/null; then
        log_error "Moon is not installed. Please install Moon first: https://moonrepo.dev/docs/install"
        exit 1
    fi

    # Check if Rust is installed
    if ! command -v cargo &> /dev/null; then
        log_error "Rust/Cargo is not installed. Please install Rust first: https://rustup.rs/"
        exit 1
    fi

    # Check if wasm32 target is available
    if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
        log_info "Installing wasm32-unknown-unknown target..."
        rustup target add wasm32-unknown-unknown
    fi

    # Check if wasm-opt is available (for optimization)
    if ! command -v wasm-opt &> /dev/null; then
        log_warn "wasm-opt not found. Install binaryen for WASM optimization:"
        log_warn "  macOS: brew install binaryen"
        log_warn "  Ubuntu/Debian: apt install binaryen"
        log_warn "  Windows: Download from https://github.com/WebAssembly/binaryen/releases"
    fi

    log_success "Dependencies check completed"
}

build_extension() {
    log_info "Building moon-shine extension..."

    # Build the WASM extension
    if ! cargo build --target wasm32-unknown-unknown --profile wasm-release; then
        log_error "Failed to build WASM extension"
        exit 1
    fi

    # Optimize WASM if wasm-opt is available
    local wasm_path="target/wasm32-unknown-unknown/wasm-release/moon_shine.wasm"
    local optimized_path="target/moon-shine-optimized.wasm"

    if command -v wasm-opt &> /dev/null; then
        log_info "Optimizing WASM binary..."
        wasm-opt -Oz --enable-bulk-memory --enable-mutable-globals \
            "$wasm_path" -o "$optimized_path"
        wasm_path="$optimized_path"
    fi

    # Verify WASM file exists and is valid
    if [[ ! -f "$wasm_path" ]]; then
        log_error "WASM file not found at $wasm_path"
        exit 1
    fi

    # Check WASM file size (warn if too large)
    local wasm_size=$(stat -f%z "$wasm_path" 2>/dev/null || stat -c%s "$wasm_path" 2>/dev/null)
    if [[ $wasm_size -gt 5242880 ]]; then  # 5MB
        log_warn "WASM file is large ($(($wasm_size / 1024 / 1024))MB). Consider optimizing dependencies."
    fi

    log_success "Extension built successfully"
    echo "$wasm_path"
}

install_extension() {
    local wasm_path="$1"
    local install_dir="$MOON_EXTENSIONS_DIR/$EXTENSION_NAME"

    log_info "Installing extension to $install_dir..."

    # Create extension directory
    mkdir -p "$install_dir"

    # Copy WASM file
    cp "$wasm_path" "$install_dir/moon-shine.wasm"

    # Create manifest file
    cat > "$install_dir/manifest.json" << EOF
{
  "name": "$EXTENSION_NAME",
  "version": "$EXTENSION_VERSION",
  "description": "AI-powered TypeScript/JavaScript linter with COPRO optimization",
  "author": "PrimeCode Moon Extensions",
  "homepage": "https://github.com/primecode/zenflow/tree/main/packages/tools/moon-shine",
  "wasm": "moon-shine.wasm",
  "capabilities": [
    "typescript-linting",
    "javascript-linting",
    "ai-code-fixes",
    "pattern-learning",
    "copro-optimization"
  ]
}
EOF

    # Copy configuration schema if available
    if [[ -f "schema.json" ]]; then
        cp "schema.json" "$install_dir/"
    fi

    # Copy documentation
    if [[ -f "README.md" ]]; then
        cp "README.md" "$install_dir/"
    fi

    log_success "Extension installed to $install_dir"
}

register_extension() {
    log_info "Registering extension with Moon..."

    # Check if we're in a Moon workspace
    if [[ ! -f ".moon/workspace.yml" ]]; then
        log_warn "Not in a Moon workspace. Extension installed but not registered."
        log_info "To use the extension, run this in a Moon workspace:"
        log_info "  moon ext install $EXTENSION_NAME"
        return
    fi

    # Register extension in workspace
    if moon ext install "$EXTENSION_NAME"; then
        log_success "Extension registered with Moon workspace"
    else
        log_warn "Failed to register extension. You may need to register manually:"
        log_info "  moon ext install $EXTENSION_NAME"
    fi
}

show_usage() {
    cat << EOF
Moon Shine Extension Installation Script

USAGE:
    $0 [OPTIONS]

OPTIONS:
    -f, --force         Force reinstall even if already installed
    -h, --help          Show this help message
    -v, --version       Show extension version

ENVIRONMENT VARIABLES:
    MOON_SHINE_VERSION     Override extension version
    MOON_EXTENSIONS_DIR    Override extensions directory (default: ~/.moon/extensions)
    FORCE_INSTALL          Force reinstall (default: false)

EXAMPLES:
    # Install extension
    $0

    # Force reinstall
    $0 --force

    # Install specific version
    MOON_SHINE_VERSION=2.0.0 $0

EOF
}

main() {
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -f|--force)
                FORCE_INSTALL=true
                shift
                ;;
            -h|--help)
                show_usage
                exit 0
                ;;
            -v|--version)
                echo "$EXTENSION_VERSION"
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done

    log_info "Installing Moon Shine Extension v$EXTENSION_VERSION"

    # Check if already installed
    local install_dir="$MOON_EXTENSIONS_DIR/$EXTENSION_NAME"
    if [[ -d "$install_dir" && "$FORCE_INSTALL" != "true" ]]; then
        log_warn "Extension already installed at $install_dir"
        log_info "Use --force to reinstall"
        exit 0
    fi

    # Run installation steps
    check_dependencies
    local wasm_path=$(build_extension)
    install_extension "$wasm_path"
    register_extension

    log_success "Moon Shine Extension installation completed!"
    log_info "Usage: moon run shine [files...]"
    log_info "       moon run shine-report [files...]  # CI mode"
}

# Run main function
main "$@"