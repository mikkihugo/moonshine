#!/bin/bash
# Moon Shine AI Linter - Monitoring and Health Check Script
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
}

# Configuration
HEALTH_CHECK_INTERVAL=60  # seconds
MAX_RETRIES=3
TIMEOUT=30

# AI Provider Endpoints
declare -A AI_PROVIDERS=(
    ["openai"]="https://api.openai.com/v1/models"
    ["claude"]="https://api.anthropic.com/v1/messages"
    ["google"]="https://generativelanguage.googleapis.com/v1/models"
)

# Health Check Functions
check_ai_provider() {
    local provider=$1
    local endpoint=$2
    local api_key_var=""

    case $provider in
        "openai")
            api_key_var="OPENAI_API_KEY"
            ;;
        "claude")
            api_key_var="ANTHROPIC_API_KEY"
            ;;
        "google")
            api_key_var="GOOGLE_API_KEY"
            ;;
    esac

    if [ -z "${!api_key_var:-}" ]; then
        warn "$provider: API key not configured ($api_key_var)"
        return 1
    fi

    info "Checking $provider provider health..."

    local response_code
    case $provider in
        "openai")
            response_code=$(curl -s -o /dev/null -w "%{http_code}" \
                -H "Authorization: Bearer ${!api_key_var}" \
                -H "Content-Type: application/json" \
                --max-time $TIMEOUT \
                "$endpoint" || echo "000")
            ;;
        "claude")
            response_code=$(curl -s -o /dev/null -w "%{http_code}" \
                -X POST \
                -H "x-api-key: ${!api_key_var}" \
                -H "Content-Type: application/json" \
                -H "anthropic-version: 2023-06-01" \
                --max-time $TIMEOUT \
                -d '{"model":"claude-3-haiku-20240307","max_tokens":1,"messages":[{"role":"user","content":"test"}]}' \
                "$endpoint" || echo "000")
            ;;
        "google")
            response_code=$(curl -s -o /dev/null -w "%{http_code}" \
                -H "Content-Type: application/json" \
                --max-time $TIMEOUT \
                "$endpoint?key=${!api_key_var}" || echo "000")
            ;;
    esac

    if [[ "$response_code" =~ ^[2-3][0-9][0-9]$ ]]; then
        log "✓ $provider provider is healthy (HTTP $response_code)"
        return 0
    else
        error "✗ $provider provider health check failed (HTTP $response_code)"
        return 1
    fi
}

check_wasm_binary() {
    info "Checking WASM binary integrity..."

    local wasm_files=(
        "dist/moon_shine.wasm"
        "target/wasm32-unknown-unknown/release/moon_shine.wasm"
    )

    local found_wasm=false
    for wasm_file in "${wasm_files[@]}"; do
        if [ -f "$wasm_file" ]; then
            found_wasm=true
            local size=$(stat -f%z "$wasm_file" 2>/dev/null || stat -c%s "$wasm_file")

            # Check WASM magic number
            local magic=$(hexdump -C "$wasm_file" | head -1 | cut -d' ' -f2-5)
            if [[ "$magic" == "00 61 73 6d" ]]; then
                log "✓ WASM binary is valid ($wasm_file, $size bytes)"
            else
                error "✗ Invalid WASM magic number in $wasm_file"
                return 1
            fi
        fi
    done

    if [ "$found_wasm" = false ]; then
        error "✗ No WASM binary found"
        return 1
    fi

    return 0
}

check_moon_config() {
    info "Checking Moon configuration..."

    if [ ! -f "moon.yml" ]; then
        error "✗ moon.yml not found"
        return 1
    fi

    # Basic YAML syntax check
    if command -v yq >/dev/null 2>&1; then
        if yq eval '.' moon.yml >/dev/null 2>&1; then
            log "✓ moon.yml syntax is valid"
        else
            error "✗ moon.yml syntax error"
            return 1
        fi
    else
        warn "yq not found, skipping YAML syntax validation"
    fi

    # Check required tasks
    local required_tasks=("build-wasm" "test" "lint")
    for task in "${required_tasks[@]}"; do
        if grep -q "^  $task:" moon.yml; then
            log "✓ Required task '$task' found in moon.yml"
        else
            warn "○ Task '$task' not found in moon.yml"
        fi
    done

    return 0
}

check_environment() {
    info "Checking environment setup..."

    # Check Rust toolchain
    if command -v rustc >/dev/null 2>&1; then
        local rust_version=$(rustc --version)
        log "✓ Rust toolchain: $rust_version"
    else
        error "✗ Rust toolchain not found"
        return 1
    fi

    # Check WASM target
    if rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        log "✓ WASM target installed"
    else
        warn "○ WASM target not installed (run: rustup target add wasm32-unknown-unknown)"
    fi

    # Check Moon CLI
    if command -v moon >/dev/null 2>&1; then
        local moon_version=$(moon --version 2>&1 | head -1)
        log "✓ Moon CLI: $moon_version"
    else
        warn "○ Moon CLI not found"
    fi

    return 0
}

generate_health_report() {
    local timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    local report_file="deployment/health-report-$(date +%Y%m%d-%H%M%S).json"

    info "Generating health report: $report_file"

    cat > "$report_file" << EOF
{
  "timestamp": "$timestamp",
  "moon_shine_version": "2.0.0",
  "health_checks": {
    "wasm_binary": $(check_wasm_binary >/dev/null 2>&1 && echo "true" || echo "false"),
    "moon_config": $(check_moon_config >/dev/null 2>&1 && echo "true" || echo "false"),
    "environment": $(check_environment >/dev/null 2>&1 && echo "true" || echo "false"),
    "ai_providers": {
EOF

    local first=true
    for provider in "${!AI_PROVIDERS[@]}"; do
        [ "$first" = true ] && first=false || echo "," >> "$report_file"
        echo -n "      \"$provider\": $(check_ai_provider "$provider" "${AI_PROVIDERS[$provider]}" >/dev/null 2>&1 && echo "true" || echo "false")" >> "$report_file"
    done

    cat >> "$report_file" << EOF

    }
  },
  "system_info": {
    "os": "$(uname -s)",
    "arch": "$(uname -m)",
    "kernel": "$(uname -r)"
  }
}
EOF

    log "Health report saved to: $report_file"
}

# Main monitoring function
run_monitoring() {
    log "Starting Moon Shine AI Linter monitoring..."

    local health_status=0

    # Run all health checks
    check_environment || health_status=1
    check_moon_config || health_status=1
    check_wasm_binary || health_status=1

    # Check AI providers
    local provider_count=0
    local healthy_providers=0

    for provider in "${!AI_PROVIDERS[@]}"; do
        ((provider_count++))
        if check_ai_provider "$provider" "${AI_PROVIDERS[$provider]}"; then
            ((healthy_providers++))
        fi
    done

    info "AI Providers: $healthy_providers/$provider_count healthy"

    # Generate health report
    generate_health_report

    if [ $health_status -eq 0 ] && [ $healthy_providers -gt 0 ]; then
        log "✓ System health: GOOD"
        return 0
    elif [ $healthy_providers -gt 0 ]; then
        warn "⚠ System health: DEGRADED (some issues detected)"
        return 1
    else
        error "✗ System health: CRITICAL (no AI providers available)"
        return 2
    fi
}

# Continuous monitoring mode
continuous_monitoring() {
    log "Starting continuous monitoring (interval: ${HEALTH_CHECK_INTERVAL}s)"

    while true; do
        log "--- Health Check Cycle ---"
        run_monitoring
        log "--- Sleeping for ${HEALTH_CHECK_INTERVAL}s ---"
        sleep $HEALTH_CHECK_INTERVAL
    done
}

# CLI interface
case "${1:-check}" in
    "check")
        run_monitoring
        ;;
    "continuous")
        continuous_monitoring
        ;;
    "providers")
        for provider in "${!AI_PROVIDERS[@]}"; do
            check_ai_provider "$provider" "${AI_PROVIDERS[$provider]}"
        done
        ;;
    "wasm")
        check_wasm_binary
        ;;
    "config")
        check_moon_config
        ;;
    "env")
        check_environment
        ;;
    "report")
        generate_health_report
        ;;
    *)
        echo "Usage: $0 [check|continuous|providers|wasm|config|env|report]"
        echo ""
        echo "Commands:"
        echo "  check        Run all health checks once (default)"
        echo "  continuous   Run continuous monitoring"
        echo "  providers    Check AI provider connectivity only"
        echo "  wasm         Check WASM binary integrity only"
        echo "  config       Check Moon configuration only"
        echo "  env          Check environment setup only"
        echo "  report       Generate health report only"
        exit 1
        ;;
esac