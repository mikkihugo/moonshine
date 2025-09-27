#!/bin/bash
# Moon Shine AI Linter - Final Deployment Package Generator
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}"
}

info() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}"
}

warn() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] WARNING: $1${NC}"
}

error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ERROR: $1${NC}"
    exit 1
}

# Configuration
VERSION="2.0.0"
PACKAGE_NAME="moon-shine-ai-linter"
BUILD_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
PACKAGE_DIR="deployment/package"
DIST_DIR="deployment/dist"

log "Creating Moon Shine AI Linter deployment package v${VERSION}..."

# Clean and create package directory
rm -rf "$PACKAGE_DIR"
mkdir -p "$PACKAGE_DIR"
mkdir -p "$DIST_DIR"

# Core files to include
log "Collecting core files..."

# WASM binary (required)
if [ -f "dist/moon_shine.wasm" ]; then
    cp "dist/moon_shine.wasm" "$PACKAGE_DIR/"
    log "✓ WASM binary included"
elif [ -f "target/wasm32-unknown-unknown/release/moon_shine.wasm" ]; then
    cp "target/wasm32-unknown-unknown/release/moon_shine.wasm" "$PACKAGE_DIR/"
    log "✓ WASM binary included (from target)"
else
    error "WASM binary not found. Run build-script.sh first."
fi

# Optimized WASM binary (optional)
if [ -f "dist/moon_shine_optimized.wasm" ]; then
    cp "dist/moon_shine_optimized.wasm" "$PACKAGE_DIR/"
    log "✓ Optimized WASM binary included"
elif [ -f "target/wasm32-unknown-unknown/release/moon_shine_optimized.wasm" ]; then
    cp "target/wasm32-unknown-unknown/release/moon_shine_optimized.wasm" "$PACKAGE_DIR/"
    log "✓ Optimized WASM binary included (from target)"
fi

# Configuration files
log "Including configuration files..."
cp "moon.yml" "$PACKAGE_DIR/"
cp "Cargo.toml" "$PACKAGE_DIR/"

# Default configurations
mkdir -p "$PACKAGE_DIR/defaults"
cp -r defaults/* "$PACKAGE_DIR/defaults/"
log "✓ Default configurations included"

# Rulebase
mkdir -p "$PACKAGE_DIR/rulebase"
cp -r rulebase/presets "$PACKAGE_DIR/rulebase/"
cp -r rulebase/schemas "$PACKAGE_DIR/rulebase/"
if [ -f "rulebase/output/moonshine-rulebase-complete.json" ]; then
    cp "rulebase/output/moonshine-rulebase-complete.json" "$PACKAGE_DIR/rulebase/"
fi
log "✓ Rulebase included"

# Documentation
log "Including documentation..."
mkdir -p "$PACKAGE_DIR/docs"
cp README.md "$PACKAGE_DIR/"
cp ARCHITECTURE_CURRENT.md "$PACKAGE_DIR/docs/" 2>/dev/null || true
cp INTEGRATION_STATUS.md "$PACKAGE_DIR/docs/" 2>/dev/null || true
cp PRODUCTION_READINESS_FINAL.md "$PACKAGE_DIR/docs/" 2>/dev/null || true

# Scripts
log "Including deployment scripts..."
mkdir -p "$PACKAGE_DIR/scripts"
cp deployment/build-script.sh "$PACKAGE_DIR/scripts/"
cp deployment/test-extension.sh "$PACKAGE_DIR/scripts/"
cp deployment/monitoring.sh "$PACKAGE_DIR/scripts/"
chmod +x "$PACKAGE_DIR/scripts/"*.sh

# Create installation script
log "Creating installation script..."
cat > "$PACKAGE_DIR/install.sh" << 'EOF'
#!/bin/bash
# Moon Shine AI Linter Installation Script
set -euo pipefail

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log() { echo -e "${GREEN}[INSTALL] $1${NC}"; }
warn() { echo -e "${YELLOW}[INSTALL] WARNING: $1${NC}"; }
error() { echo -e "${RED}[INSTALL] ERROR: $1${NC}"; exit 1; }

log "Installing Moon Shine AI Linter..."

# Check prerequisites
command -v moon >/dev/null 2>&1 || error "Moon CLI is required. Install from: https://moonrepo.dev"

# Get Moon workspace root
if [ -f "moon.yml" ]; then
    MOON_ROOT="."
elif [ -f "../moon.yml" ]; then
    MOON_ROOT=".."
else
    error "No Moon workspace found. Run this from a Moon workspace root."
fi

# Create .moon/extensions directory
EXTENSIONS_DIR="$MOON_ROOT/.moon/extensions"
mkdir -p "$EXTENSIONS_DIR"

# Copy WASM extension
if [ -f "moon_shine_optimized.wasm" ]; then
    cp "moon_shine_optimized.wasm" "$EXTENSIONS_DIR/moon_shine.wasm"
    log "✓ Optimized WASM extension installed"
elif [ -f "moon_shine.wasm" ]; then
    cp "moon_shine.wasm" "$EXTENSIONS_DIR/moon_shine.wasm"
    log "✓ WASM extension installed"
else
    error "WASM binary not found"
fi

# Copy configuration files
cp -r defaults "$EXTENSIONS_DIR/moon_shine_defaults"
cp -r rulebase "$EXTENSIONS_DIR/moon_shine_rulebase"
log "✓ Configuration files installed"

# Update .moon/workspace.yml if needed
WORKSPACE_YML="$MOON_ROOT/.moon/workspace.yml"
if [ -f "$WORKSPACE_YML" ]; then
    if ! grep -q "moon_shine" "$WORKSPACE_YML"; then
        log "Adding Moon Shine extension to workspace.yml..."
        cat >> "$WORKSPACE_YML" << 'WORKSPACE_EOF'

# Moon Shine AI Linter Extension
extensions:
  wasm:
    - id: moon_shine
      file: ./extensions/moon_shine.wasm
      config:
        enabled: true
        ai_threshold: 0.5
        max_budget_per_file: 0.10
WORKSPACE_EOF
        log "✓ Extension configuration added to workspace.yml"
    else
        warn "Moon Shine extension already configured in workspace.yml"
    fi
else
    warn "No workspace.yml found, you may need to configure the extension manually"
fi

log "Installation complete!"
log "Next steps:"
log "1. Configure AI provider API keys in your environment"
log "2. Run: moon run :lint to test the extension"
log "3. See docs/ for configuration options"
EOF

chmod +x "$PACKAGE_DIR/install.sh"
log "✓ Installation script created"

# Create package manifest
log "Creating package manifest..."
cat > "$PACKAGE_DIR/MANIFEST.json" << EOF
{
  "name": "$PACKAGE_NAME",
  "version": "$VERSION",
  "build_date": "$BUILD_DATE",
  "description": "AI-powered code quality extension for moonrepo - TypeScript/JavaScript linting, fixing, and optimization",
  "author": "ZenFlow Contributors",
  "homepage": "https://github.com/zenflow/zenflow",
  "files": {
    "wasm_binary": "moon_shine.wasm",
    "wasm_optimized": "moon_shine_optimized.wasm",
    "config": "moon.yml",
    "defaults": "defaults/",
    "rulebase": "rulebase/",
    "scripts": "scripts/",
    "documentation": "docs/"
  },
  "requirements": {
    "moon_cli": ">=0.25.0",
    "rust_toolchain": ">=1.80",
    "wasm_target": "wasm32-unknown-unknown"
  },
  "ai_providers": {
    "supported": ["openai", "claude", "google"],
    "environment_variables": {
      "openai": "OPENAI_API_KEY",
      "claude": "ANTHROPIC_API_KEY",
      "google": "GOOGLE_API_KEY"
    }
  },
  "features": [
    "TypeScript/JavaScript linting",
    "AI-powered code analysis",
    "Automatic code fixing",
    "Performance optimization suggestions",
    "Security vulnerability detection",
    "Custom rule generation",
    "DSPy-based pattern learning"
  ]
}
EOF

# Calculate checksums
log "Calculating checksums..."
cd "$PACKAGE_DIR"
find . -type f -name "*.wasm" -o -name "*.yml" -o -name "*.json" | sort | xargs sha256sum > CHECKSUMS.txt
cd - >/dev/null

# Create compressed archive
log "Creating deployment archive..."
ARCHIVE_NAME="${PACKAGE_NAME}-${VERSION}-$(date +%Y%m%d).tar.gz"
tar -czf "$DIST_DIR/$ARCHIVE_NAME" -C "$PACKAGE_DIR" .

# Generate deployment summary
ARCHIVE_SIZE=$(stat -f%z "$DIST_DIR/$ARCHIVE_NAME" 2>/dev/null || stat -c%s "$DIST_DIR/$ARCHIVE_NAME")
WASM_SIZE=$(stat -f%z "$PACKAGE_DIR/moon_shine.wasm" 2>/dev/null || stat -c%s "$PACKAGE_DIR/moon_shine.wasm")

log "Creating deployment summary..."
cat > "$DIST_DIR/DEPLOYMENT_SUMMARY.md" << EOF
# Moon Shine AI Linter - Deployment Summary

**Version:** $VERSION
**Build Date:** $BUILD_DATE
**Package:** $ARCHIVE_NAME
**Package Size:** $ARCHIVE_SIZE bytes
**WASM Binary Size:** $WASM_SIZE bytes

## Package Contents

- **moon_shine.wasm** - Main WASM extension binary
- **moon_shine_optimized.wasm** - Size-optimized WASM binary (if available)
- **moon.yml** - Moon task configuration
- **defaults/** - Default AI provider configs and prompts
- **rulebase/** - TypeScript/JavaScript rule presets and schemas
- **scripts/** - Deployment and monitoring scripts
- **docs/** - Documentation and architecture guides
- **install.sh** - Automated installation script
- **MANIFEST.json** - Package metadata and requirements
- **CHECKSUMS.txt** - File integrity verification

## Installation

1. Extract the archive in your Moon workspace
2. Run: \`./install.sh\`
3. Configure AI provider API keys
4. Test with: \`moon run :lint\`

## AI Providers

Configure one or more AI providers by setting environment variables:

- **OpenAI:** \`OPENAI_API_KEY\`
- **Claude:** \`ANTHROPIC_API_KEY\`
- **Google:** \`GOOGLE_API_KEY\`

## Monitoring

Use the included monitoring script to check system health:

\`\`\`bash
./scripts/monitoring.sh check
./scripts/monitoring.sh continuous  # For continuous monitoring
\`\`\`

## Support

- Documentation: See docs/ directory
- Issues: https://github.com/zenflow/zenflow/issues
- Repository: https://github.com/zenflow/zenflow
EOF

# Final summary
log "Deployment package created successfully!"
info "Package: $DIST_DIR/$ARCHIVE_NAME"
info "Size: $ARCHIVE_SIZE bytes"
info "Files: $(find "$PACKAGE_DIR" -type f | wc -l)"
info "Summary: $DIST_DIR/DEPLOYMENT_SUMMARY.md"

# Verification
log "Verifying package integrity..."
if tar -tzf "$DIST_DIR/$ARCHIVE_NAME" >/dev/null 2>&1; then
    log "✓ Archive integrity verified"
else
    error "Archive integrity check failed"
fi

log "Deployment package ready for distribution!"