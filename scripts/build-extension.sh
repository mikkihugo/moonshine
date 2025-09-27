#!/usr/bin/env bash
set -euo pipefail

# Moon Shine Extension Build Script
# Builds and optimizes the moon-shine WASM extension

PROFILE="${PROFILE:-wasm-release}"
OPTIMIZE="${OPTIMIZE:-true}"
TARGET="${TARGET:-wasm32-unknown-unknown}"
OUTPUT_DIR="${OUTPUT_DIR:-dist}"

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

check_tools() {
    log_info "Checking build tools..."

    if ! command -v cargo &> /dev/null; then
        log_error "Cargo not found. Please install Rust: https://rustup.rs/"
        exit 1
    fi

    if ! rustup target list --installed | grep -q "$TARGET"; then
        log_info "Installing target $TARGET..."
        rustup target add "$TARGET"
    fi

    if [[ "$OPTIMIZE" == "true" ]] && ! command -v wasm-opt &> /dev/null; then
        log_warn "wasm-opt not found. Install binaryen for optimization:"
        log_warn "  macOS: brew install binaryen"
        log_warn "  Ubuntu/Debian: apt install binaryen"
        OPTIMIZE=false
    fi

    log_success "Build tools ready"
}

build_wasm() {
    log_info "Building WASM extension with profile $PROFILE..."

    # Set WASM-specific environment variables
    export RUSTFLAGS="-C target-feature=+bulk-memory,+mutable-globals"

    # Build the extension
    if ! cargo build --target "$TARGET" --profile "$PROFILE"; then
        log_error "Build failed"
        exit 1
    fi

    # Handle different profile directory names
    local profile_dir="$PROFILE"
    if [[ "$PROFILE" == "wasm-release" ]]; then
        profile_dir="release"
    fi

    local wasm_file="target/$TARGET/$profile_dir/moon_shine.wasm"

    if [[ ! -f "$wasm_file" ]]; then
        log_error "WASM file not found: $wasm_file"
        log_info "Available files in target/$TARGET/:"
        ls -la "target/$TARGET/" || true
        exit 1
    fi

    log_success "Build completed: $wasm_file"
    echo "$wasm_file"
}

optimize_wasm() {
    local input_file="$1"
    local output_file="$2"

    if [[ "$OPTIMIZE" != "true" ]]; then
        cp "$input_file" "$output_file"
        return
    fi

    log_info "Optimizing WASM binary..."

    # Apply aggressive optimization
    wasm-opt -Oz \
        --enable-bulk-memory \
        --enable-mutable-globals \
        --enable-nontrapping-float-to-int \
        --enable-sign-ext \
        --vacuum \
        "$input_file" \
        -o "$output_file"

    # Show size comparison
    local original_size=$(stat -f%z "$input_file" 2>/dev/null || stat -c%s "$input_file")
    local optimized_size=$(stat -f%z "$output_file" 2>/dev/null || stat -c%s "$output_file")
    local reduction=$((100 - (optimized_size * 100 / original_size)))

    log_success "Optimization complete: $reduction% size reduction"
    log_info "Original: $(numfmt --to=iec $original_size)B"
    log_info "Optimized: $(numfmt --to=iec $optimized_size)B"
}

create_manifest() {
    local version=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')

    cat > "$OUTPUT_DIR/manifest.json" << EOF
{
  "name": "moon-shine",
  "version": "$version",
  "description": "AI-powered TypeScript/JavaScript linter with COPRO optimization",
  "author": "PrimeCode Moon Extensions",
  "homepage": "https://github.com/primecode/zenflow/tree/main/packages/tools/moon-shine",
  "wasm": "moon-shine.wasm",
  "config_schema": "schema.json",
  "capabilities": [
    "typescript-linting",
    "javascript-linting",
    "ai-code-fixes",
    "pattern-learning",
    "copro-optimization"
  ],
  "moon": {
    "extension": {
      "enabled": true,
      "functions": [
        "register_extension",
        "execute_extension",
        "health_check",
        "get_capabilities",
        "get_schema"
      ]
    }
  }
}
EOF

    log_success "Created manifest.json"
}

generate_schema() {
    log_info "Generating configuration schema..."

    # Try to extract schema from the extension
    local wasm_file="$OUTPUT_DIR/moon-shine.wasm"

    # For now, create a basic schema. In the future, this could be
    # extracted from the WASM module itself
    cat > "$OUTPUT_DIR/schema.json" << 'EOF'
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Moon Shine Configuration",
  "type": "object",
  "properties": {
    "ai": {
      "type": "object",
      "properties": {
        "ai_model": {
          "type": "string",
          "description": "AI model to use for code analysis",
          "default": "sonnet",
          "enum": ["sonnet", "haiku", "opus"]
        },
        "ai_providers": {
          "type": "array",
          "items": {"type": "string"},
          "description": "Available AI providers",
          "default": ["claude"]
        },
        "max_files_per_task": {
          "type": "integer",
          "description": "Maximum files to process per task",
          "default": 50,
          "minimum": 1,
          "maximum": 1000
        }
      }
    },
    "operation_mode": {
      "type": "string",
      "description": "Default operation mode",
      "default": "fix",
      "enum": ["fix", "lint-only", "reporting-only"]
    },
    "enable_copro_optimization": {
      "type": "boolean",
      "description": "Enable COPRO prompt optimization",
      "default": true
    },
    "temperature": {
      "type": "number",
      "description": "AI temperature for creativity",
      "default": 0.7,
      "minimum": 0.0,
      "maximum": 2.0
    },
    "max_tokens": {
      "type": "integer",
      "description": "Maximum tokens per AI request",
      "default": 4096,
      "minimum": 100,
      "maximum": 32768
    }
  }
}
EOF

    log_success "Generated schema.json"
}

package_extension() {
    log_info "Creating distribution package..."

    mkdir -p "$OUTPUT_DIR"

    # Build and optimize WASM
    local wasm_file=$(build_wasm)
    optimize_wasm "$wasm_file" "$OUTPUT_DIR/moon-shine.wasm"

    # Generate metadata
    create_manifest
    generate_schema

    # Copy documentation
    if [[ -f "README.md" ]]; then
        cp "README.md" "$OUTPUT_DIR/"
    fi

    # Create checksums
    (cd "$OUTPUT_DIR" && sha256sum *.wasm *.json > checksums.txt)

    log_success "Extension packaged in $OUTPUT_DIR/"

    # Show package contents
    log_info "Package contents:"
    ls -la "$OUTPUT_DIR/"
}

show_usage() {
    cat << EOF
Moon Shine Extension Build Script

USAGE:
    $0 [OPTIONS]

OPTIONS:
    -p, --profile PROFILE   Build profile (default: wasm-release)
    -t, --target TARGET     Target triple (default: wasm32-unknown-unknown)
    -o, --output DIR        Output directory (default: dist)
    --no-optimize           Skip WASM optimization
    --optimize              Enable WASM optimization (default)
    -h, --help              Show this help message

ENVIRONMENT VARIABLES:
    PROFILE      Build profile (release, wasm-release, debug)
    OPTIMIZE     Enable optimization (true/false)
    TARGET       Target triple
    OUTPUT_DIR   Output directory

EXAMPLES:
    # Build optimized extension
    $0

    # Build debug version
    $0 --profile debug --no-optimize

    # Build to custom directory
    $0 --output build

EOF
}

main() {
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -p|--profile)
                PROFILE="$2"
                shift 2
                ;;
            -t|--target)
                TARGET="$2"
                shift 2
                ;;
            -o|--output)
                OUTPUT_DIR="$2"
                shift 2
                ;;
            --no-optimize)
                OPTIMIZE=false
                shift
                ;;
            --optimize)
                OPTIMIZE=true
                shift
                ;;
            -h|--help)
                show_usage
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done

    log_info "Building Moon Shine Extension"
    log_info "Profile: $PROFILE"
    log_info "Target: $TARGET"
    log_info "Output: $OUTPUT_DIR"
    log_info "Optimize: $OPTIMIZE"

    check_tools
    package_extension

    log_success "Build completed successfully!"
}

# Run main function
main "$@"