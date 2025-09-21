# OXC Rules Documentation

## Overview

This directory contains comprehensive documentation, templates, and guidelines for the OXC-compatible rule system in moon-shine. The rule system provides WASM-safe static analysis capabilities complementing SunLinter's AI behavioral rules.

## 📁 Directory Structure

```
rules/
├── README.md                    # This file - overview and index
├── rule-template.rs             # Standard template for new rules
├── formats/
│   ├── rule-format-spec.md      # Complete rule format specification
│   ├── diagnostic-format.md     # Diagnostic message standards
│   └── ai-enhancement-spec.md   # AI suggestion format guidelines
├── templates/
│   ├── basic-rule-template.rs   # Minimal rule implementation
│   ├── enhanced-rule-template.rs # Full featured rule with AI
│   └── module-template.rs       # Complete module template
├── guidelines/
│   ├── naming-conventions.md    # Rule and module naming standards
│   ├── testing-standards.md     # Testing requirements and patterns
│   ├── performance-guidelines.md # WASM performance considerations
│   └── integration-guide.md     # Registry integration steps
└── examples/
    ├── simple-rule-example.rs   # Basic correctness rule
    ├── complex-rule-example.rs  # Advanced pattern detection
    └── module-example.rs        # Complete module implementation
```

## 🎯 Quick Start

1. **Creating a New Rule**: Start with `templates/basic-rule-template.rs`
2. **Creating a Rule Module**: Use `templates/module-template.rs`
3. **Understanding Formats**: Read `formats/rule-format-spec.md`
4. **Testing Guidelines**: Follow `guidelines/testing-standards.md`
5. **Integration**: Use `guidelines/integration-guide.md`

## 📋 Rule Categories

### Current Rule Modules (347+ rules)

| Module | Rules | Focus Area |
|--------|-------|------------|
| `oxc_compatible_rules` | 15 | Core OXC compatibility layer |
| `oxc_rules_migration` | 12 | Migrated OXLint rules |
| `oxc_performance_rules` | 8 | Performance optimization |
| `oxc_string_rules` | 8 | String manipulation patterns |
| `oxc_conditional_rules` | 8 | Control flow and logic |
| `oxc_object_rules` | 10 | Object and array patterns |
| `oxc_function_rules` | 10 | Function definition and usage |
| `oxc_variable_rules` | 10 | Variable declaration patterns |
| `oxc_import_rules` | 10 | Import/export management |
| `oxc_error_rules` | 10 | Error handling patterns |
| `oxc_typescript_rules` | 10 | TypeScript-specific rules |
| `oxc_security_rules` | 12 | Security vulnerability detection |
| `oxc_react_rules` | 10 | React framework patterns |
| `oxc_accessibility_rules` | 10 | Basic accessibility rules |
| `oxc_es6_rules` | 12 | Modern JavaScript features |
| `oxc_complexity_rules` | 12 | Code complexity analysis |
| `oxc_nodejs_rules` | 12 | Node.js specific patterns |
| `oxc_async_rules` | 12 | Async/await and Promise patterns |
| `oxc_jsx_advanced_rules` | 12 | Advanced JSX patterns |
| `oxc_bestpractices_rules` | 14 | General best practices |
| `oxc_css_rules` | 12 | CSS-in-JS and styling |
| `oxc_testing_rules` | 14 | Testing framework patterns |
| `oxc_documentation_rules` | 14 | Documentation standards |
| `oxc_advanced_performance_rules` | 14 | Advanced optimization |
| `oxc_vue_rules` | 14 | Vue.js framework patterns |
| `oxc_angular_rules` | 14 | Angular framework patterns |
| `oxc_build_tool_rules` | 14 | Build and bundling tools |
| `oxc_database_orm_rules` | 16 | Database and ORM patterns |
| `oxc_monorepo_workspace_rules` | 16 | Monorepo management |
| `oxc_state_management_rules` | 16 | State management patterns |
| `oxc_graphql_rules` | 16 | GraphQL schema and resolvers |
| `oxc_testing_framework_rules` | 16 | Framework-specific testing |
| `oxc_devops_deployment_rules` | 16 | DevOps and deployment |
| `oxc_pwa_modern_web_rules` | 18 | PWA and modern web APIs |
| `oxc_microfrontend_rules` | 16 | Micro-frontend architecture |
| `oxc_edge_serverless_rules` | 16 | Edge and serverless patterns |
| `oxc_webrtc_realtime_rules` | 16 | WebRTC and real-time communication |
| `oxc_web_payments_commerce_rules` | 16 | Payment and e-commerce security |
| `oxc_enterprise_architecture_rules` | 20 | Enterprise architecture patterns |
| `oxc_accessibility_i18n_rules` | 22 | Accessibility and internationalization |
| `oxc_advanced_frameworks_rules` | 24 | Modern framework integration |
| `oxc_cloud_native_rules` | 24 | Cloud-native and Kubernetes |
| `oxc_api_integration_rules` | 24 | API design and integration |

## 🏗️ Architecture Principles

### 1. WASM-First Design
- All rules must be WASM-compatible
- Minimal external dependencies
- Efficient memory usage
- Fast execution for real-time analysis

### 2. AI Enhancement Layer
- Each rule includes AI-powered suggestions
- Confidence scoring for fix recommendations
- Context-aware improvement suggestions
- Auto-fixable vs manual intervention classification

### 3. Modular Organization
- Domain-specific rule groupings
- Clear separation of concerns
- Reusable diagnostic patterns
- Consistent testing approaches

### 4. Complementary to SunLinter
- Static analysis focus (vs SunLinter's behavioral AI)
- Pattern-based detection
- Immediate feedback capability
- Zero overlap with AI behavioral rules

## 🎨 Rule Design Patterns

### Basic Rule Structure
```rust
pub struct RuleName;

impl RuleName {
    pub const NAME: &'static str = "rule-name";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RuleName {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        // Pattern detection logic
    }
}

impl EnhancedWasmRule for RuleName {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        // AI enhancement logic
    }
}
```

### Categories and Fix Status

#### WasmRuleCategory
- **Correctness**: Logic errors, type issues, definite bugs
- **Suspicious**: Potentially problematic patterns
- **Pedantic**: Strict style enforcement
- **Perf**: Performance optimization opportunities
- **Restriction**: Security and safety restrictions
- **Style**: Code formatting and consistency
- **Nursery**: Experimental or unstable rules

#### WasmFixStatus
- **Fix**: Safe automatic fixes
- **FixDangerous**: Automatic fixes that might change behavior
- **Suggestion**: Manual intervention recommended
- **ConditionalFixSuggestion**: Context-dependent fixes

## 📏 Quality Standards

### Code Quality
- ✅ 100% test coverage for rule detection
- ✅ Comprehensive AI enhancement testing
- ✅ Performance benchmarks under 1ms per rule
- ✅ Memory usage under 1MB per rule module

### Documentation Quality
- ✅ Clear rule descriptions with examples
- ✅ Rationale for each rule's existence
- ✅ Fix suggestion explanations
- ✅ Performance impact documentation

### Integration Quality
- ✅ Unified registry integration
- ✅ Proper error handling
- ✅ WASM compatibility verification
- ✅ Zero external dependencies

## 🔄 Development Workflow

### 1. Planning Phase
- Identify rule domain and scope
- Research existing similar rules
- Define target patterns and anti-patterns
- Plan AI enhancement strategies

### 2. Implementation Phase
- Use appropriate template
- Implement pattern detection logic
- Add comprehensive diagnostics
- Include AI enhancement layer

### 3. Testing Phase
- Write detection tests
- Test AI enhancement quality
- Benchmark performance
- Verify WASM compatibility

### 4. Integration Phase
- Add to unified registry
- Update lib.rs exports
- Document in module list
- Verify integration tests

### 5. Documentation Phase
- Update README files
- Add rule examples
- Document performance characteristics
- Include troubleshooting guides

## 🎯 Target Roadmap

### Current Status: 347/582 rules (59.6%)

### Planned Expansions:
1. **Data Science & ML Rules** (30 rules)
2. **Blockchain & Web3 Rules** (25 rules)
3. **Performance Monitoring Rules** (20 rules)
4. **Gaming & Interactive Rules** (15 rules)
5. **IoT & Embedded Rules** (15 rules)
6. **AR/VR Development Rules** (10 rules)

### Target Completion: Q2 2025

## 🤝 Contributing

### For Internal Development
1. Follow templates and guidelines
2. Maintain consistency with existing patterns
3. Ensure comprehensive testing
4. Update documentation

### For External Contributors
1. Review contribution guidelines
2. Use provided templates
3. Follow testing standards
4. Submit with documentation

## 📞 Support and Resources

- **Architecture Questions**: See `formats/rule-format-spec.md`
- **Implementation Help**: Use `templates/` directory
- **Testing Guidance**: Read `guidelines/testing-standards.md`
- **Performance Issues**: Check `guidelines/performance-guidelines.md`
- **Integration Problems**: Follow `guidelines/integration-guide.md`

---

**Last Updated**: Current Date
**Rule Count**: 347 active rules
**Target**: 582 total rules
**Status**: Production ready with ongoing expansion