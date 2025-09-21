# Integration Guide for OXC Rules

## Overview

This guide provides step-by-step instructions for integrating new OXC rules and rule modules into the moon-shine WASM extension. Follow these procedures to ensure proper integration, testing, and deployment.

## 🎯 Integration Objectives

### Primary Goals
- **Seamless Integration**: New rules work with existing infrastructure
- **Registry Coordination**: Proper registration with unified rule registry
- **Performance Maintenance**: No degradation of overall system performance
- **Backward Compatibility**: Existing functionality remains unaffected

### Quality Assurance
- All integration steps documented and tested
- Zero breaking changes to existing APIs
- Comprehensive validation of new functionality
- Performance benchmarks maintained or improved

## 🔄 Integration Workflow

### Phase 1: Pre-Integration Preparation

#### 1.1 Development Environment Setup

```bash
# Ensure you have the latest moon-shine codebase
cd /path/to/moon-shine
git pull origin main

# Verify build system works
moon run moon-shine:build
moon run moon-shine:test
moon run moon-shine:lint
```

#### 1.2 Rule Module Validation

```bash
# Test your new rule module independently
cargo test --lib oxc_your_module_rules

# Verify WASM compatibility
cargo check --target wasm32-wasip1

# Run performance benchmarks
cargo bench --bench rule_performance
```

### Phase 2: File System Integration

#### 2.1 Module File Placement

```
src/
├── lib.rs                          # Main library exports
├── unified_rule_registry.rs        # Central rule registration
└── oxc_your_module_rules.rs       # Your new rule module
```

#### 2.2 Update lib.rs

```rust
// Add to the module declarations section
pub mod oxc_your_module_rules; // Your module description

// Verify the section looks like:
pub mod oxc_accessibility_i18n_rules; // Accessibility and internationalization rules
pub mod oxc_advanced_frameworks_rules; // Advanced framework integration rules
pub mod oxc_cloud_native_rules; // Cloud-native and container orchestration rules
pub mod oxc_api_integration_rules; // API design and integration patterns rules
pub mod oxc_your_module_rules; // Your module description
pub mod unified_rule_registry; // Unified registry for all OXC-compatible WASM rules
```

### Phase 3: Registry Integration

#### 3.1 Update unified_rule_registry.rs Imports

```rust
// Add to the import section
use crate::oxc_your_module_rules::*;

// Verify all imports are present:
use crate::oxc_advanced_frameworks_rules::*;
use crate::oxc_cloud_native_rules::*;
use crate::oxc_api_integration_rules::*;
use crate::oxc_your_module_rules::*;
```

#### 3.2 Register Rules in register_all_rules()

```rust
// In the register_all_rules() method, add your rules:
fn register_all_rules(&mut self) {
    // ... existing rules ...

    // Your module rules - add appropriate comment header
    self.register_rule(YourFirstRule {});
    self.register_rule(YourSecondRule {});
    self.register_rule(YourThirdRule {});
    // ... add all your rules
}
```

#### 3.3 Verify Registration Order

Rules should be registered in logical groups:

```rust
// Core and migrated rules first
// Framework-specific rules
// Security and restriction rules
// Performance and style rules
// Your new rules (in appropriate category)
```

### Phase 4: Compilation and Testing

#### 4.1 Build Verification

```bash
# Clean build to ensure no cached artifacts
cargo clean

# Full build with WASM target
moon run moon-shine:build

# Verify no compilation errors
echo $? # Should output 0
```

#### 4.2 Test Suite Execution

```bash
# Run all tests
moon run moon-shine:test

# Run specific module tests
cargo test oxc_your_module_rules

# Run integration tests
cargo test --test integration

# Run performance benchmarks
cargo bench
```

#### 4.3 Registry Validation

```rust
#[test]
fn test_registry_integration() {
    let registry = UnifiedRuleRegistry::new();

    // Verify your rules are registered
    assert!(registry.get_rule("your-first-rule").is_some());
    assert!(registry.get_rule("your-second-rule").is_some());

    // Verify rule count increased appropriately
    let stats = registry.get_statistics();
    assert!(stats.total_rules >= EXPECTED_MINIMUM_RULES);

    // Verify no duplicate rule names
    let rule_names = registry.get_all_rule_names();
    let unique_names: std::collections::HashSet<_> = rule_names.iter().collect();
    assert_eq!(rule_names.len(), unique_names.len());
}
```

### Phase 5: Documentation Updates

#### 5.1 Update Module Documentation

```markdown
# In src/rules/README.md, add your module to the table:

| `oxc_your_module_rules` | N | Your module description |
```

#### 5.2 Create Module-Specific Documentation

```rust
// Add comprehensive module documentation
//! # Your Module Title
//!
//! Brief description of the module's purpose and scope.
//!
//! ## Rule Categories:
//! - **Category 1**: Description
//! - **Category 2**: Description
//!
//! ## Examples:
//! ```javascript
//! // Example code patterns
//! ```
```

#### 5.3 Update Integration Counts

```rust
// Update the total rule count in documentation
// Current Status: XXX/582 rules (XX.X%)
```

### Phase 6: Performance Validation

#### 6.1 Benchmark New Rules

```rust
#[bench]
fn bench_your_rule_performance(b: &mut Bencher) {
    let code = include_str!("../test_data/sample_code.js");
    let rule = YourRule;

    b.iter(|| {
        black_box(rule.run(code))
    });
}
```

#### 6.2 System Performance Impact

```bash
# Before integration benchmark
cargo bench --bench system_performance > before.txt

# After integration benchmark
cargo bench --bench system_performance > after.txt

# Compare results
diff before.txt after.txt
```

#### 6.3 Memory Usage Analysis

```bash
# Use memory profiling tools for WASM
wasm-pack build --profiling

# Analyze memory usage patterns
# (specific tools depend on your WASM runtime)
```

### Phase 7: Integration Testing

#### 7.1 End-to-End Testing

```rust
#[test]
fn test_full_integration() {
    let registry = UnifiedRuleRegistry::new();
    let test_code = include_str!("../test_data/integration_test.js");

    // Run all rules including new ones
    let all_diagnostics = run_all_rules(&registry, test_code);

    // Verify expected behavior
    assert!(all_diagnostics.iter().any(|d| d.rule_name.starts_with("your-module")));
}
```

#### 7.2 Regression Testing

```bash
# Run comprehensive regression test suite
cargo test --all --release

# Verify no existing functionality broken
moon run moon-shine:test --verbose
```

#### 7.3 WASM Runtime Testing

```bash
# Test in actual WASM environment
wasm-pack test --node

# Verify memory constraints
# Verify execution time constraints
```

## 🔧 Common Integration Issues

### Issue 1: Compilation Failures

**Symptoms**: Build fails with missing imports or type errors

**Solution**:
```rust
// Ensure all required imports are present
use crate::unified_rule_registry::{RuleSettings, WasmRuleCategory, WasmFixStatus};
use serde::{Deserialize, Serialize};

// Verify trait implementations match exactly
impl WasmRule for YourRule {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        // Implementation
    }
}
```

### Issue 2: Registry Registration Failures

**Symptoms**: Rules not found at runtime, registration errors

**Solution**:
```rust
// Verify struct implements both required traits
impl WasmRule for YourRule { /* ... */ }
impl EnhancedWasmRule for YourRule { /* ... */ }

// Ensure Clone is implemented (required for registration)
#[derive(Clone)]
pub struct YourRule;

// Or implement Clone manually
impl Clone for YourRule {
    fn clone(&self) -> Self {
        YourRule
    }
}
```

### Issue 3: Performance Regressions

**Symptoms**: Overall system slower after integration

**Solution**:
```rust
// Optimize pattern matching
if code.contains("pattern") {
    // Use efficient string operations
    // Avoid regex if possible
    // Cache repeated calculations
}

// Minimize allocations
fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
    let mut diagnostics = Vec::with_capacity(expected_size);
    // Pre-allocate if known size
}
```

### Issue 4: WASM Compatibility Issues

**Symptoms**: Works in native Rust but fails in WASM

**Solution**:
```rust
// Avoid unsupported operations
// No file system access
// No network operations
// No threading
// Use approved dependencies only

// Example: Use slice operations instead of String::from
let substring = &code[start..end]; // Good
let substring = code[start..end].to_string(); // Avoid if possible
```

## 📋 Integration Checklist

### Pre-Integration
- [ ] Rule module follows template structure
- [ ] All tests pass independently
- [ ] Performance benchmarks meet requirements
- [ ] Documentation is complete
- [ ] WASM compatibility verified

### Integration Steps
- [ ] Module file added to correct location
- [ ] lib.rs updated with module export
- [ ] unified_rule_registry.rs updated with imports
- [ ] All rules registered in register_all_rules()
- [ ] Build passes with no warnings
- [ ] All tests pass including new ones

### Post-Integration
- [ ] Registry integration test passes
- [ ] No performance regressions
- [ ] Documentation updated
- [ ] Rule count updated in README
- [ ] Integration tests pass
- [ ] WASM build successful

### Quality Assurance
- [ ] No breaking changes to existing APIs
- [ ] Backward compatibility maintained
- [ ] Memory usage within limits
- [ ] Execution time within requirements
- [ ] Error handling robust

## 🚀 Deployment Considerations

### Version Management

```toml
# Update Cargo.toml version
[package]
version = "X.Y.Z" # Increment appropriately

# Document changes in CHANGELOG.md
## [X.Y.Z] - YYYY-MM-DD
### Added
- New rule module: oxc_your_module_rules with N rules
- Rules for [specific domain/technology]
```

### Release Notes

```markdown
## New Features
- Added XX new rules for [domain] patterns
- Enhanced AI suggestions for [specific area]
- Improved performance for [specific scenarios]

## Rule Additions
- `rule-name-1`: Description
- `rule-name-2`: Description
```

### Monitoring Integration

```rust
// Add telemetry for new rules if needed
#[cfg(feature = "telemetry")]
fn track_rule_usage(&self, rule_name: &str) {
    // Increment usage counter
    // Track performance metrics
    // Log any issues
}
```

## 📞 Support and Troubleshooting

### Debug Information

```rust
#[cfg(debug_assertions)]
fn debug_integration() {
    let registry = UnifiedRuleRegistry::new();
    println!("Total rules: {}", registry.get_all_rule_names().len());

    for rule_name in registry.get_all_rule_names() {
        println!("Rule: {}", rule_name);
    }
}
```

### Performance Profiling

```bash
# Profile during integration
cargo build --release
perf record --call-graph dwarf target/release/moon-shine-test
perf report
```

### Common Error Messages

1. **"Rule not found"**: Check registry registration
2. **"Trait not implemented"**: Verify WasmRule and EnhancedWasmRule implementations
3. **"WASM build failed"**: Check for unsupported dependencies or operations
4. **"Performance regression"**: Profile and optimize slow rules

---

**Version**: 1.0
**Last Updated**: Current Date
**Status**: Production Integration Guide
**Support**: Follow troubleshooting steps or create issue with integration logs