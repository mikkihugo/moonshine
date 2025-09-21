# OXC-Compatible Rule Template for 600 Rules Migration

## 🎯 Complete Template Structure

Our WASM-safe implementation follows OXC's exact rule template pattern while adding AI enhancement capabilities. This template provides the foundation for systematically migrating all 600 rules (582 OXLint + ~200 SunLint AI).

## 📝 Rule Implementation Template

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
/// // Invalid code examples
/// ```
///
/// Examples of **correct** code:
/// ```js
/// // Valid code examples
/// ```
#[derive(Debug, Default, Clone)]
pub struct RuleName {
    // Configuration options (if any)
}

// Following OXC's const pattern for rule metadata
impl RuleName {
    pub const NAME: &'static str = "rule-name";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::[Category];
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::[Status];

    pub fn new() -> Self {
        Self::default()
    }
}

impl WasmRule for RuleName {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        match node.kind() {
            AstKind::[NodeType](node_data) => {
                // Rule logic here
                if violation_detected {
                    ctx.diagnostic(rule_diagnostic("message", node_data.span));
                }
            }
            _ => {}
        }
    }
}

// Diagnostic creation function (following OXC pattern)
fn rule_diagnostic(message: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn(message)
        .with_help("Helpful suggestion")
        .with_label(span)
}

// AI Enhancement (our unique value-add)
impl EnhancedWasmRule for RuleName {
    fn ai_enhance(&self, diagnostic: &OxcDiagnostic, source: &str) -> Vec<String> {
        vec![
            "AI-generated suggestion 1".to_string(),
            "AI-generated suggestion 2".to_string(),
        ]
    }

    fn ai_explain(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Option<String> {
        Some("AI-generated contextual explanation".to_string())
    }
}

// Test cases (following OXC pattern)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_name() {
        let pass = vec![
            "valid code example 1",
            "valid code example 2",
        ];

        let fail = vec![
            "invalid code example 1",
            "invalid code example 2",
        ];

        let tester = WasmTester::new(RuleName::NAME, pass, fail);
        tester.test_rule::<RuleName>().expect("RuleName tests should pass");
    }
}
```

## 📊 Rule Categories (Matching OXC Exactly)

| Category | Description | Use Cases |
|----------|-------------|-----------|
| `Nursery` | Experimental rules | New rules under development |
| `Correctness` | Likely bugs | Logic errors, undefined behavior |
| `Suspicious` | Code that looks wrong | Unusual patterns, potential issues |
| `Pedantic` | Nitpicky but useful | Style preferences, best practices |
| `Perf` | Performance optimization | Inefficient patterns, bottlenecks |
| `Restriction` | Coding standards | Enforce team conventions |
| `Style` | Formatting/conventions | Code formatting, naming |

## 🔧 Fix Status Classification

| Status | Description | When to Use |
|--------|-------------|-------------|
| `Pending` | No fix available | Complex rules requiring manual intervention |
| `Fix` | Safe automatic fix | Simple, deterministic fixes |
| `FixDangerous` | Potentially unsafe fix | Fixes that might change behavior |
| `Suggestion` | Manual fix suggestion | AI-generated recommendations |
| `ConditionalFixSuggestion` | Context-dependent | Fixes requiring developer judgment |

## 🎯 600 Rules Migration Strategy

### Phase 1: OXC Rule Adaptation (582 rules)

**Pattern**: Convert existing OXC rule logic to WasmRule trait

```rust
// Original OXC pattern (oxc_linter)
declare_oxc_lint!(
    /// Documentation
    NoEmpty,
    correctness
);

impl Rule for NoEmpty {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        // OXC rule logic
    }
}

// Our WASM-safe adaptation
impl WasmRule for NoEmpty {
    const NAME: &'static str = "no-empty";
    const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        // Adapted rule logic (same AST analysis)
    }
}
```

### Phase 2: SunLint AI Rules (~200 rules)

**Pattern**: AI-driven pattern analysis with AST context

```rust
impl WasmRule for AiPatternRule {
    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        // Use AST for context, AI for analysis
        let context = extract_ast_context(node, ctx);
        let ai_analysis = self.ai_analyzer.analyze_pattern(context)?;

        if ai_analysis.has_violation() {
            ctx.diagnostic(ai_pattern_diagnostic(ai_analysis));
        }
    }
}
```

## 🚀 Implementation Workflow

### 1. Rule Discovery
```bash
# Find OXC rules to adapt
find oxc-project/crates/oxc_linter/src/rules -name "*.rs" | head -10

# Analyze SunLint rules
ls moonshine-rules/common/*/analyzer.js | head -10
```

### 2. Rule Adaptation
```rust
// 1. Copy OXC documentation format
// 2. Adapt rule logic for WASM safety
// 3. Add AI enhancement layer
// 4. Create test cases
// 5. Register with rule engine
```

### 3. Testing & Validation
```bash
# Test individual rules
moon run moon-shine:test -- test_no_empty

# Test rule categories
moon run moon-shine:test -- test_rule_categories

# Performance benchmarking
moon run moon-shine:benchmark
```

## 📈 Performance Characteristics

| Metric | Traditional Regex | Our OXC + AI Solution |
|--------|------------------|----------------------|
| **Parsing** | 1x baseline | 50-100x faster (OXC AST) |
| **Accuracy** | Pattern matching | Semantic understanding |
| **AI Enhancement** | None | Intelligent suggestions |
| **WASM Compatibility** | Limited | Full support |
| **Type Awareness** | None | Via Moon task delegation |

## 🎯 Unique Value Proposition

Our hybrid architecture provides:

1. **OXC Performance**: 50-100x faster than regex-based linting
2. **AI Intelligence**: Contextual suggestions beyond static analysis
3. **WASM Safety**: Full compatibility with Moon extension runtime
4. **Systematic Migration**: Clear path for all 600 rules
5. **Type Awareness**: Advanced analysis via Moon task delegation

## 📝 Next Steps

1. **Rule Migration**: Use this template to systematically adapt 582 OXC rules
2. **AI Rules**: Implement 200 SunLint AI patterns using the AI enhancement layer
3. **Performance Testing**: Benchmark against existing solutions
4. **Integration**: Deploy as Moon extension with workflow integration

This template provides the foundation for creating a production-ready linter that combines the speed of Rust/OXC, the intelligence of AI, and the convenience of WASM deployment in the Moon ecosystem.