# Rule Migration Guide: OXC → Moon-Shine WASM

## 🎯 Complete Migration Framework

This guide provides a systematic approach for migrating all 600 rules (582 OXLint + ~200 SunLint AI) to our WASM-compatible OXC template structure with AI enhancements.

## 📊 Migration Progress

### ✅ **Completed Rules (8 rules)**

| Rule Name | Category | Type | Status |
|-----------|----------|------|--------|
| `no-empty` | Suspicious | Base Template | ✅ Complete |
| `boolean-naming` | Style | Base Template | ✅ Complete |
| `no-unused-vars` | Correctness | Core Migration | ✅ Complete |
| `no-unreachable` | Correctness | Core Migration | ✅ Complete |
| `no-implicit-return` | Suspicious | Core Migration | ✅ Complete |
| `no-inefficient-regexp` | Performance | Performance Set | ✅ Complete |
| `no-inefficient-array-methods` | Performance | Performance Set | ✅ Complete |
| `no-expensive-computation-in-render` | Performance | Performance Set | ✅ Complete |

### 📋 **Remaining Migration (592 rules)**

- **OXC Correctness Rules**: 174 remaining (180 total)
- **OXC Suspicious Rules**: 95 remaining (100 total)
- **OXC Style Rules**: 89 remaining (92 total)
- **OXC Performance Rules**: 47 remaining (50 total)
- **OXC Pedantic Rules**: 38 remaining (40 total)
- **OXC Restriction Rules**: 29 remaining (30 total)
- **OXC Nursery Rules**: 28 remaining (30 total)
- **SunLint AI Rules**: 200 remaining

## 🏗️ Migration Template Structure

### 1. **OXC Rule Adaptation Pattern**

```rust
/// [Rule Name] - Following OXC Template Structure
///
/// ### What it does
/// [Brief description of what the rule checks]
///
/// ### Why is this bad?
/// [Explanation of why this pattern is problematic]
///
/// ### Examples
///
/// Examples of **incorrect** code:
/// ```js
/// // Invalid code examples with clear violations
/// ```
///
/// Examples of **correct** code:
/// ```js
/// // Valid code examples showing proper patterns
/// ```
#[derive(Debug, Default, Clone)]
pub struct RuleName {
    // Configuration options if needed
}

// OXC-compatible const pattern
impl RuleName {
    pub const NAME: &'static str = "rule-name";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::[Category];
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::[Status];
}

impl WasmRule for RuleName {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        match node.kind() {
            AstKind::[NodeType](node_data) => {
                // Adapt OXC rule logic here - maintain exact AST analysis
                if violation_detected {
                    ctx.diagnostic(rule_diagnostic("message", node_data.span));
                }
            }
            _ => {}
        }
    }
}

// Diagnostic function following OXC pattern
fn rule_diagnostic(message: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Rule violation")
        .with_help(message)
        .with_label(span)
}

// AI Enhancement (our unique value-add)
impl EnhancedWasmRule for RuleName {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "AI-generated suggestion 1".to_string(),
            "AI-generated suggestion 2".to_string(),
        ]
    }

    fn ai_explain(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Option<String> {
        Some("AI-generated contextual explanation".to_string())
    }
}

// Test cases following OXC pattern
#[cfg(test)]
mod tests {
    use super::*;
    use crate::oxc_compatible_rules::WasmTester;

    #[test]
    fn test_rule_name() {
        let pass = vec!["valid code", "another valid case"];
        let fail = vec!["invalid code", "another violation"];

        let tester = WasmTester::new(RuleName::NAME, pass, fail);
        tester.test_rule::<RuleName>().expect("RuleName tests should pass");
    }
}
```

### 2. **SunLint AI Rule Pattern**

```rust
/// AI-Powered Rule - Following OXC Template Structure
///
/// ### What it does
/// Uses AI analysis to detect patterns beyond static analysis capabilities.
///
/// ### Why is this bad?
/// [AI-contextual explanation of why this pattern should be avoided]
///
/// ### Examples
/// [Code examples with AI-enhanced insights]
#[derive(Debug, Default, Clone)]
pub struct AiPatternRule {
    ai_analyzer: Option<AiAnalyzer>,
}

impl AiPatternRule {
    pub const NAME: &'static str = "ai-pattern-rule";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for AiPatternRule {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        // Combine AST analysis with AI insights
        if let Some(analyzer) = &self.ai_analyzer {
            let context = extract_ast_context(node, ctx);
            let ai_analysis = analyzer.analyze_pattern(context);

            if ai_analysis.has_violation() {
                ctx.diagnostic(ai_pattern_diagnostic(ai_analysis));
            }
        }
    }
}
```

## 🎯 **Category-Specific Migration Strategies**

### **Correctness Rules (174 remaining)**
- **Priority**: Highest - these catch bugs
- **Pattern**: Direct AST logic adaptation
- **AI Enhancement**: Bug explanation, fix suggestions
- **Examples**: `no-unused-vars`, `no-unreachable`, `no-implicit-return`

### **Suspicious Rules (95 remaining)**
- **Priority**: High - potential issues
- **Pattern**: Pattern detection with AI context
- **AI Enhancement**: Intent analysis, alternative suggestions
- **Examples**: `no-implicit-return`, suspicious patterns

### **Performance Rules (47 remaining)**
- **Priority**: High - production critical
- **Pattern**: Performance analysis + AI optimization
- **AI Enhancement**: Optimization strategies, alternatives
- **Examples**: `no-inefficient-regexp`, `no-expensive-computation-in-render`

### **Style Rules (89 remaining)**
- **Priority**: Medium - code consistency
- **Pattern**: Style enforcement + AI formatting
- **AI Enhancement**: Style explanations, team conventions
- **Examples**: `boolean-naming`, formatting rules

## 🚀 **Migration Workflow**

### **Phase 1: Rule Discovery & Analysis**
```bash
# Find OXC rule to migrate
find oxc-project/crates/oxc_linter/src/rules -name "*.rs" | grep [rule-name]

# Analyze original implementation
cat oxc-project/crates/oxc_linter/src/rules/[category]/[rule-name].rs
```

### **Phase 2: Template Adaptation**
1. **Copy rule documentation** - maintain OXC format exactly
2. **Adapt struct definition** - add WASM-safe configuration
3. **Convert rule logic** - maintain AST analysis, ensure WASM safety
4. **Add AI enhancement** - provide intelligent suggestions
5. **Create test cases** - follow OXC test pattern

### **Phase 3: Integration & Testing**
```bash
# Add to rule engine
# In oxc_rules_migration.rs or category-specific file:
engine.add_enhanced_rule(NewRule::default());

# Test implementation
cargo test new_rule_name --lib -- --nocapture

# Validate with Moon
moon run moon-shine:test -- test_new_rule
```

## 📈 **Migration Priority Matrix**

| Priority | Category | Rules | Business Impact | Timeline |
|----------|----------|-------|-----------------|----------|
| **P0** | Correctness | 174 | Bugs in production | Week 1-2 |
| **P1** | Performance | 50 | User experience | Week 3-4 |
| **P1** | Suspicious | 100 | Code quality | Week 5-6 |
| **P2** | Style | 92 | Developer experience | Week 7-8 |
| **P2** | AI Rules | 200 | Intelligent analysis | Week 9-12 |
| **P3** | Other | 176 | Advanced features | Week 13-16 |

## 🔧 **Tools & Automation**

### **Migration Scripts**
```bash
# Generate rule template
./scripts/generate-oxc-rule.sh [rule-name] [category]

# Batch migrate category
./scripts/migrate-oxc-category.sh correctness

# Validate migration
./scripts/validate-migration.sh [rule-name]
```

### **Quality Assurance**
```bash
# Performance benchmark
moon run moon-shine:benchmark -- --rule [rule-name]

# Integration test
moon run moon-shine:integration-test

# WASM compatibility check
moon run moon-shine:wasm-test
```

## 🎯 **Success Metrics**

### **Migration Quality**
- ✅ **100% OXC compatibility** - exact logic preservation
- ✅ **WASM safety** - no threading, compatible dependencies
- ✅ **AI enhancement** - meaningful suggestions for each rule
- ✅ **Performance** - maintain 50-100x speed advantage

### **Test Coverage**
- ✅ **Unit tests** - each rule has pass/fail cases
- ✅ **Integration tests** - works with rule engine
- ✅ **Performance tests** - benchmarks vs regex approach
- ✅ **WASM tests** - validates Moon extension compatibility

### **Documentation Standards**
- ✅ **OXC format** - maintains exact documentation structure
- ✅ **AI insights** - explains intelligent enhancements
- ✅ **Migration notes** - documents adaptation decisions
- ✅ **Usage examples** - practical implementation guidance

## 🏆 **Expected Outcomes**

### **Production-Ready Linter**
- **600 rules** systematically migrated with AI enhancement
- **50-100x performance** improvement over regex-based approaches
- **WASM deployment** ready for Moon extension ecosystem
- **AI intelligence** providing unique value beyond static analysis

### **Systematic Foundation**
- **Template-driven** migration ensuring consistency
- **Quality assured** with comprehensive testing
- **Performance optimized** for production workloads
- **Future-ready** architecture for additional rule development

This migration framework provides a systematic approach to creating a production-ready linter that combines OXC's proven performance with AI-enhanced intelligence, deployed as a WASM-safe Moon extension.