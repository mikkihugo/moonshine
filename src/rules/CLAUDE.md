# 🎯 MoonShine Rules - AI-Enhanced Rule Engine

## 📖 Package Documentation

**For complete rule conversion methodology, see [RULE_CONVERSION_GUIDE.md](../../RULE_CONVERSION_GUIDE.md) in the package root.**

## 🎯 Rules Package Role

**AI-Enhanced Rule Engine with OXC Semantic Analysis** - Modular rule system for TypeScript/JavaScript code quality and security:

- **🧠 AI-Powered Enhancement**: Claude integration for intelligent rule suggestions and context-aware fixes
- **⚡ OXC Integration**: 10-100x faster than regex-based analysis via OXC semantic analysis and AST visitors
- **🔧 Exact Fidelity**: Precise behavioral equivalence from original JavaScript rules to Rust implementations
- **📁 1 Rule = 1 File**: Optimal modularity with isolated testing and AI enhancement per rule
- **🌐 WASM Compatible**: Full Moon PDK integration for complete project file access

## ✅ ALLOWED Operations

- **Rule Conversion**: JavaScript to Rust with Swiss watchmaker precision
- **AI Enhancement**: Claude integration for intelligent suggestions and fixes
- **OXC Analysis**: Semantic analysis and AST visitor patterns instead of regex
- **Moon PDK Integration**: Complete project introspection and file access
- **Modular Architecture**: Independent rule development and testing

## 🔧 MODIFICATION GUIDELINES

This is the **core rule engine** - modifications should focus on rule accuracy and AI enhancement:
- **Rule Conversion**: Follow the proven template exactly - no deviations
- **AI Integration**: Enhance rules with Claude suggestions and context-aware fixes
- **Performance**: Optimize via OXC semantic analysis over regex patterns
- **Testing**: Maintain comprehensive test coverage for all rules
- **Documentation**: Document AI enhancement hooks and rule behavior

### 🎯 **ESSENTIAL: Follow the Conversion Template**

**✅ REQUIRED Conversion Process:**
```bash
# 1. Read JavaScript rule completely
# 2. Apply proven conversion template methodology
# 3. Use OXC AST visitors instead of regex
# 4. Add AI enhancement hooks
# 5. Include comprehensive tests
# 6. Register in category dispatcher
```

**✅ Template Structure (Mandatory):**
```rust
//! # [RULE_ID]: [Rule Name] Rule
//! @category [category]-rules
//! @complexity [level]

pub fn check_[rule_name](program: &Program, semantic: &Semantic, code: &str) -> Vec<LintIssue>
struct [RuleName]Visitor<'a> // OXC AST visitor
impl<'a> Visit<'a> for [RuleName]Visitor<'a> // Semantic analysis
#[cfg(test)] mod tests // Violation + compliant cases
```

### ⚠️ Critical Requirements

- **Exact Fidelity**: Preserve exact JavaScript behavior and error messages
- **OXC Integration**: Use semantic analysis and AST visitors, not regex patterns
- **AI Enhancement**: Configure `ai_enhanced: true` and `RuleImplementation::AiAssisted`
- **Testing**: Include both violation and compliant test cases
- **Documentation**: Complete JSDoc-style comments with examples

### 🎨 AI Enhancement Architecture

**Rule Registration with AI:**
```rust
rules.insert("C029".to_string(), MoonShineRule {
    id: "C029".to_string(),
    category: MoonShineRuleCategory::CodeQuality,
    severity: LintSeverity::Warning,
    description: "Every catch block must log the error cause".to_string(),
    ai_enhanced: true, // Enable Claude integration
    implementation: RuleImplementation::AiAssisted,
});
```

**AI Enhancement Hooks:**
```rust
// Rules automatically enhanced with Claude suggestions
fn check_ai_assisted_rule(&self, rule_id: &str, program: &Program, semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let base_issues = match rule_id {
        "C029" => c029_catch_block_logging::check_catch_block_logging(program, semantic, code),
        // ... other AI-enhanced rules
    };

    // Claude provides context-aware suggestions
    super::ai_integration::enhance_with_ai(base_issues, &self.ai_context)
}
```

## 🏗️ Architecture Overview

### **File Structure (1 Rule = 1 File)**
```
src/rules/
├── engine.rs              # Core rule engine with AI integration
├── code_quality/          # C-series rules
│   ├── c029_catch_block_logging.rs
│   ├── c042_boolean_naming.rs
│   ├── c017_limit_constructor_logic.rs
│   └── mod.rs             # Category dispatcher
├── security/              # S-series rules
│   ├── s001_fail_securely.rs
│   └── mod.rs             # Category dispatcher
├── utils/                 # Shared utilities
│   └── mod.rs             # Span conversion, Unicode handling
├── ai_integration.rs      # Claude AI enhancement
└── integration_test.rs    # End-to-end testing
```

### **Rule Categories**
- **CodeQuality (C-series)**: Code maintainability, readability, complexity
- **Security (S-series)**: Vulnerability detection, security best practices
- **Performance (P-series)**: Performance optimization suggestions
- **Testing (T-series)**: Test quality and coverage rules
- **Naming (N-series)**: Naming convention enforcement

## 📊 Conversion Status

### ✅ **Completed Rules (5/193)**
- **C029**: Catch block logging - OXC AST visitor pattern
- **C042**: Boolean naming - Complex configuration preserved
- **C017**: Constructor logic - Method visitor with semantic analysis
- **C030**: Custom error classes - Expression visitor with type checking
- **C006**: Function naming - Advanced pattern matching

### 🔄 **Remaining Conversions (188/193)**
All remaining JavaScript rules in `moonshine-rules/` directory ready for systematic conversion using the proven template.

### 🎯 **AI Enhancement Ready**
All converted rules configured for Claude integration with:
- Context-aware error messages
- Intelligent fix suggestions
- Performance optimization hints
- Code quality insights

## 🚀 Performance Benefits

### **vs Traditional Linting**
- **10-100x faster** than regex-based analysis
- **Memory efficient** with OXC arena allocation
- **WASM compatible** for browser/extension deployment
- **Semantic accuracy** vs pattern matching

### **AI Enhancement Benefits**
- **Context-aware suggestions** based on surrounding code
- **Intelligent fix recommendations** beyond simple rule violations
- **Performance insights** and optimization opportunities
- **Educational explanations** for better developer understanding

## 🧪 Testing Architecture

### **Test Requirements (Per Rule)**
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_[rule_name]_violation() {
        // Code that SHOULD trigger the rule
        let issues = parse_and_check(violating_code);
        assert!(!issues.is_empty());
    }

    #[test]
    fn test_[rule_name]_compliant() {
        // Code that should NOT trigger the rule
        let issues = parse_and_check(compliant_code);
        assert!(issues.is_empty());
    }
}
```

### **Integration Testing**
- **Rule engine**: End-to-end testing with multiple rules
- **AI enhancement**: Claude suggestion quality verification
- **Performance**: Benchmarking against regex-based alternatives
- **WASM compatibility**: Moon PDK integration validation

## 🎯 Rule Development Workflow

### **Adding New Rules**
1. **Source Analysis**: Read JavaScript rule completely
2. **Template Application**: Follow conversion guide exactly
3. **OXC Integration**: Use semantic analysis over regex
4. **AI Configuration**: Enable enhancement hooks
5. **Testing**: Comprehensive violation/compliant cases
6. **Integration**: Register in category dispatcher
7. **Verification**: End-to-end rule engine testing

### **Quality Gates**
- [ ] Compiles without warnings
- [ ] Passes violation and compliant tests
- [ ] Follows exact template structure
- [ ] AI enhancement configured
- [ ] Registered in dispatcher
- [ ] Documentation complete

## 🤝 Integration Context

**Standalone Rule Engine within MoonShine**:
- Rules engine **operates as core component** of MoonShine WASM extension
- **Direct integration** with OXC semantic analysis for maximum performance
- **AI enhancement** ready for Claude integration via Moon PDK
- **Modular architecture** allows selective rule compilation and deployment

## 📚 Development Resources

- **[RULE_CONVERSION_GUIDE.md](../../RULE_CONVERSION_GUIDE.md)**: Complete conversion methodology
- **OXC Documentation**: AST types and visitor patterns
- **Moon PDK Guide**: File access and WASM integration
- **AI Integration**: Claude enhancement patterns and examples

---

**MoonShine Rules - AI-Enhanced Code Quality Engine - Swiss Precision Conversion Ready**