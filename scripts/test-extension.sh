#!/usr/bin/env bash
set -euo pipefail

# Moon Shine Extension Testing Script
# Tests the moon-shine WASM extension with Moon integration

TEST_DIR="${TEST_DIR:-test-workspace}"
EXTENSION_DIR="${EXTENSION_DIR:-dist}"
CLEANUP="${CLEANUP:-true}"

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

cleanup_test_workspace() {
    if [[ "$CLEANUP" == "true" && -d "$TEST_DIR" ]]; then
        log_info "Cleaning up test workspace..."
        rm -rf "$TEST_DIR"
    fi
}

create_test_workspace() {
    log_info "Creating test workspace..."

    # Clean up existing
    cleanup_test_workspace

    # Create new workspace
    mkdir -p "$TEST_DIR"
    cd "$TEST_DIR"

    # Initialize Moon workspace
    cat > .moon/workspace.yml << 'EOF'
$schema: 'https://moonrepo.dev/schemas/workspace.json'

projects:
  - '.'

vcs:
  manager: 'git'
  defaultBranch: 'main'

runner:
  autoCleanCache: true
  cacheLifetime: '7 days'

telemetry: false
EOF

    # Create test TypeScript files
    mkdir -p src

    cat > src/test.ts << 'EOF'
// Test TypeScript file with intentional issues
function badFunction(x: any): any {
    if (x) {
        return x.toString()
    }
    return null
}

const unusedVariable = "this should trigger a warning";

// Missing documentation
export function complexFunction(a: number, b: string, c: boolean) {
    const result = badFunction(a);
    return result + b + c;
}

// Type issues
let implicitAny;
implicitAny = "hello";
implicitAny = 123;

export default badFunction;
EOF

    cat > src/good.ts << 'EOF'
/**
 * Well-documented TypeScript file
 */

/**
 * Calculates the sum of two numbers
 * @param a First number
 * @param b Second number
 * @returns The sum of a and b
 */
export function add(a: number, b: number): number {
    return a + b;
}

/**
 * Formats a greeting message
 * @param name The name to greet
 * @returns A formatted greeting
 */
export function greet(name: string): string {
    return `Hello, ${name}!`;
}
EOF

    # Create package.json
    cat > package.json << 'EOF'
{
  "name": "moon-shine-test",
  "version": "1.0.0",
  "type": "module",
  "devDependencies": {
    "typescript": "^5.0.0",
    "@types/node": "^20.0.0",
    "eslint": "^8.0.0",
    "@typescript-eslint/parser": "^6.0.0",
    "@typescript-eslint/eslint-plugin": "^6.0.0"
  }
}
EOF

    # Create TypeScript config
    cat > tsconfig.json << 'EOF'
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "node",
    "strict": true,
    "noImplicitAny": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "exactOptionalPropertyTypes": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist"]
}
EOF

    # Create ESLint config
    cat > eslint.config.js << 'EOF'
export default [
  {
    files: ["**/*.ts", "**/*.tsx"],
    languageOptions: {
      parser: "@typescript-eslint/parser",
      parserOptions: {
        ecmaVersion: "latest",
        sourceType: "module"
      }
    },
    plugins: {
      "@typescript-eslint": require("@typescript-eslint/eslint-plugin")
    },
    rules: {
      "@typescript-eslint/no-explicit-any": "error",
      "@typescript-eslint/no-unused-vars": "error",
      "@typescript-eslint/explicit-function-return-type": "warn"
    }
  }
];
EOF

    log_success "Test workspace created in $TEST_DIR"
}

install_test_extension() {
    log_info "Installing extension for testing..."

    local extension_wasm="../$EXTENSION_DIR/moon-shine.wasm"
    local extension_manifest="../$EXTENSION_DIR/manifest.json"

    if [[ ! -f "$extension_wasm" ]]; then
        log_error "Extension WASM not found: $extension_wasm"
        log_info "Build the extension first: ./scripts/build-extension.sh"
        exit 1
    fi

    if [[ ! -f "$extension_manifest" ]]; then
        log_error "Extension manifest not found: $extension_manifest"
        exit 1
    fi

    # Create extensions directory
    mkdir -p .moon/extensions/moon-shine

    # Copy extension files
    cp "$extension_wasm" .moon/extensions/moon-shine/
    cp "$extension_manifest" .moon/extensions/moon-shine/

    if [[ -f "../$EXTENSION_DIR/schema.json" ]]; then
        cp "../$EXTENSION_DIR/schema.json" .moon/extensions/moon-shine/
    fi

    log_success "Extension installed for testing"
}

create_moon_config() {
    log_info "Creating Moon project configuration..."

    cat > moon.yml << 'EOF'
$schema: 'https://moonrepo.dev/schemas/project.json'

type: 'library'
language: 'typescript'

tasks:
  # Test the moon-shine extension
  shine:
    command: 'moon'
    args: ['ext', 'run', 'moon-shine', '--']
    inputs: ['src/**/*.ts']
    options:
      cache: false
      runFromWorkspaceRoot: true

  # Test reporting mode
  shine-report:
    command: 'moon'
    args: ['ext', 'run', 'moon-shine', '--', '--reporting-only']
    inputs: ['src/**/*.ts']
    options:
      cache: true
      runFromWorkspaceRoot: true

  # Test lint-only mode
  shine-lint:
    command: 'moon'
    args: ['ext', 'run', 'moon-shine', '--', '--lint-only']
    inputs: ['src/**/*.ts']
    options:
      cache: true
      runFromWorkspaceRoot: true
EOF

    log_success "Moon configuration created"
}

test_extension_basic() {
    log_info "Testing basic extension functionality..."

    # Test extension registration
    if ! moon ext list | grep -q moon-shine; then
        log_error "Extension not registered with Moon"
        return 1
    fi

    log_success "Extension registered successfully"

    # Test health check (if supported)
    if moon ext run moon-shine -- --help &>/dev/null; then
        log_success "Extension responds to help command"
    else
        log_warn "Extension does not respond to help command"
    fi
}

test_extension_modes() {
    log_info "Testing extension execution modes..."

    # Test reporting mode (should be fastest)
    log_info "Testing reporting mode..."
    if timeout 60 moon run shine-report src/test.ts; then
        log_success "Reporting mode completed successfully"
    else
        log_error "Reporting mode failed or timed out"
        return 1
    fi

    # Test lint-only mode
    log_info "Testing lint-only mode..."
    if timeout 60 moon run shine-lint src/test.ts; then
        log_success "Lint-only mode completed successfully"
    else
        log_warn "Lint-only mode failed or timed out"
    fi

    # Test fix mode (may take longer)
    log_info "Testing fix mode..."
    if timeout 120 moon run shine src/test.ts; then
        log_success "Fix mode completed successfully"
    else
        log_warn "Fix mode failed or timed out"
    fi
}

test_file_processing() {
    log_info "Testing file processing..."

    # Process good file (should pass)
    if moon run shine-report src/good.ts; then
        log_success "Good file processed successfully"
    else
        log_warn "Good file processing failed"
    fi

    # Process bad file (should find issues)
    if moon run shine-report src/test.ts; then
        log_success "Bad file processed (issues expected)"
    else
        log_warn "Bad file processing failed"
    fi
}

test_performance() {
    log_info "Testing performance..."

    # Create multiple test files
    for i in {1..5}; do
        cp src/test.ts "src/test$i.ts"
    done

    # Time the execution
    log_info "Processing 6 files..."
    local start_time=$(date +%s)

    if moon run shine-report src/; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        log_success "Processed 6 files in ${duration}s"

        if [[ $duration -lt 30 ]]; then
            log_success "Performance: Excellent (< 30s)"
        elif [[ $duration -lt 60 ]]; then
            log_success "Performance: Good (< 60s)"
        else
            log_warn "Performance: Slow (> 60s)"
        fi
    else
        log_error "Batch processing failed"
        return 1
    fi
}

run_all_tests() {
    log_info "Running comprehensive extension tests..."

    local test_failures=0

    # Basic functionality tests
    if ! test_extension_basic; then
        ((test_failures++))
    fi

    # Mode tests
    if ! test_extension_modes; then
        ((test_failures++))
    fi

    # File processing tests
    if ! test_file_processing; then
        ((test_failures++))
    fi

    # Performance tests
    if ! test_performance; then
        ((test_failures++))
    fi

    # Report results
    if [[ $test_failures -eq 0 ]]; then
        log_success "All tests passed!"
        return 0
    else
        log_error "$test_failures test(s) failed"
        return 1
    fi
}

show_usage() {
    cat << EOF
Moon Shine Extension Testing Script

USAGE:
    $0 [OPTIONS] [TEST_TYPE]

OPTIONS:
    -d, --dir DIR           Test workspace directory (default: test-workspace)
    -e, --extension DIR     Extension directory (default: dist)
    --no-cleanup            Don't clean up test workspace
    -h, --help              Show this help message

TEST_TYPES:
    basic                   Test basic functionality only
    modes                   Test execution modes
    files                   Test file processing
    performance             Test performance
    all                     Run all tests (default)

EXAMPLES:
    # Run all tests
    $0

    # Test basic functionality only
    $0 basic

    # Use custom directories
    $0 --dir my-test --extension build all

EOF
}

main() {
    local test_type="all"

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -d|--dir)
                TEST_DIR="$2"
                shift 2
                ;;
            -e|--extension)
                EXTENSION_DIR="$2"
                shift 2
                ;;
            --no-cleanup)
                CLEANUP=false
                shift
                ;;
            -h|--help)
                show_usage
                exit 0
                ;;
            basic|modes|files|performance|all)
                test_type="$1"
                shift
                ;;
            *)
                log_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done

    # Trap cleanup
    trap cleanup_test_workspace EXIT

    log_info "Testing Moon Shine Extension"
    log_info "Test type: $test_type"
    log_info "Test directory: $TEST_DIR"
    log_info "Extension directory: $EXTENSION_DIR"

    # Set up test environment
    create_test_workspace
    install_test_extension
    create_moon_config

    # Run requested tests
    case $test_type in
        basic)
            test_extension_basic
            ;;
        modes)
            test_extension_modes
            ;;
        files)
            test_file_processing
            ;;
        performance)
            test_performance
            ;;
        all)
            run_all_tests
            ;;
    esac

    local exit_code=$?

    if [[ $exit_code -eq 0 ]]; then
        log_success "Testing completed successfully!"
    else
        log_error "Testing failed!"
    fi

    exit $exit_code
}

# Run main function
main "$@"