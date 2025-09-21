# SunLint to OXC Conversion Patterns

## Violation Field Mapping

### JavaScript SunLint Structure
```javascript
violations.push({
  ruleId: 'C073',                    // Rule identifier
  severity: 'error',                 // error|warning|info
  message: 'Detailed message',       // Human-readable description
  line: analysis.envAccess[0].line,  // Source line number
  column: 1,                        // Source column number
  filePath: filePath,               // Absolute file path
  suggestions: [                    // AI-powered fix suggestions
    'Move environment variable access to dedicated configuration modules',
    'Use dependency injection for configuration'
  ],
  category: category.name,          // Security rule categories
  categoryDescription: category.description,
  matchedPattern: original,         // Original regex pattern
  matchedText: matchedText          // Actual matched text
});
```

### Rust OXC LintIssue Structure
```rust
pub struct LintIssue {
    pub message: String,              // Maps to JS message
    pub severity: LintSeverity,       // Maps to JS severity
    pub line: u32,                    // Maps to JS line
    pub column: u32,                  // Maps to JS column
    pub rule_id: String,              // Maps to JS ruleId
    pub suggestion: Option<String>,   // Maps to JS suggestions[0]
    pub category: Option<String>,     // Maps to JS category
    pub matched_text: Option<String>, // Maps to JS matchedText
}
```

## Conversion Patterns

### 1. Regex to OXC AST Visitor

**Before (JavaScript Regex):**
```javascript
// C042: Boolean naming
const booleanAssignments = this.findBooleanAssignments(line);
const patterns = [
  /(?:let|const|var)\s+(\w+)\s*(?::\s*\w+\s*)?=\s*(.+?)(?:;|$)/g,
];
```

**After (OXC AST Visitor):**
```rust
// C042: Boolean naming
impl<'a> Visit<'a> for BooleanNamingVisitor<'a> {
    fn visit_variable_declarator(&mut self, declarator: &VariableDeclarator<'a>) {
        if let Some(init) = &declarator.init {
            // Check if assigned a boolean value
            if self.is_boolean_expression(init) {
                // Check naming convention
                self.check_boolean_naming(declarator);
            }
        }
    }
}
```

### 2. Multi-Stage Analysis Pipeline

**JavaScript Smart Pipeline:**
```javascript
// 3-stage: Regex → AST → Data Flow
if (this.smartPipeline) {
  return await this.smartPipeline.analyze(files, language, options);
} else {
  return await this.analyzeWithRegex(files, language, options);
}
```

**OXC Hybrid Approach:**
```rust
// RuleImplementation enum supports multiple strategies
pub enum RuleImplementation {
    OxcSemantic,      // OXC semantic analysis with AI context
    OxcAstVisitor,    // OXC AST visitor pattern with AI suggestions
    OxlintEnhanced,   // OXLint rules + AI enhancement layer
    AiAssisted,       // Pure AI analysis
    Hybrid,           // Combines OXC AST + OXLint + AI
}
```

### 3. AI Enhancement Integration

**JavaScript AI Suggestions:**
```javascript
generateSuggestions(varName) {
  const suggestions = [];
  const baseName = varName.replace(/^(is|has|should|can|will|must|may|check)/i, '');
  const capitalizedBase = baseName.charAt(0).toUpperCase() + baseName.slice(1);

  suggestions.push(`is${capitalizedBase}`);
  suggestions.push(`has${capitalizedBase}`);
  suggestions.push(`should${capitalizedBase}`);

  return suggestions.slice(0, 3);
}
```

**OXC AI Enhancement:**
```rust
// AI-powered suggestion generation using DSPy templates
fn generate_ai_suggestions(&self, violation_context: &ViolationContext) -> Vec<String> {
    let template = self.get_ai_template("boolean_naming_suggestions");
    let prompt = template.format(&violation_context);

    // Use AI provider to generate context-aware suggestions
    self.ai_provider.generate_suggestions(prompt)
}
```

## Template System Integration

### DSPy-Optimizable Templates
```rust
pub struct RuleTemplate {
    pub rule_id: String,
    pub template_type: TemplateType,
    pub protection_level: ProtectionLevel,
    pub dspy_optimized: bool,
    pub performance_metrics: PerformanceMetrics,
}

pub enum TemplateType {
    PromptTemplate,     // AI prompt engineering
    CodeGenTemplate,    // Code generation patterns
    SuggestionTemplate, // Fix suggestion generation
    AnalysisTemplate,   // Code analysis prompts
}
```

### AI Assistance Layers
```rust
pub trait AiEnhancedRule {
    fn analyze_with_ai(&self, context: &CodeContext) -> Vec<AiInsight>;
    fn generate_suggestions(&self, violation: &LintIssue) -> Vec<String>;
    fn explain_violation(&self, violation: &LintIssue) -> String;
    fn auto_fix_suggestion(&self, violation: &LintIssue) -> Option<CodeFix>;
}
```

## Workflow Integration

### 14-Phase CI/CD Pipeline
```rust
// Phase 3: OXC AST-based rule analysis (600 rules)
AnalysisPhase {
    name: "oxc-rules-analysis".to_string(),
    description: "🔍 OXC AST-based rule analysis (600 rules: OXLint + SunLint AI)".to_string(),
    command: "moon-shine-oxc".to_string(),
    priority: 3,
}
```

### Rule Execution Flow
```rust
pub fn execute_rule(&self, rule_id: &str, program: &Program, semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    match self.get_rule_implementation(rule_id) {
        RuleImplementation::OxcAstVisitor => {
            let mut visitor = self.create_ast_visitor(rule_id, code);
            visitor.visit_program(program);
            visitor.violations.into_iter().map(|v| v.into()).collect()
        }
        RuleImplementation::Hybrid => {
            let oxc_issues = self.run_oxc_analysis(rule_id, program, semantic, code);
            let ai_enhancements = self.run_ai_enhancement(rule_id, &oxc_issues, code);
            self.merge_analysis_results(oxc_issues, ai_enhancements)
        }
        _ => Vec::new()
    }
}
```

## Migration Statistics

- **Total Rules**: ~600 (582 OXLint + ~200 SunLint AI)
- **Code Quality**: 77 rules (C-series)
- **Security**: 49 rules (S-series)
- **TypeScript**: 10 rules (T-series)
- **Conversion Strategy**: Systematic regex → OXC AST with AI preservation

## Benefits of OXC Conversion

1. **Performance**: 10-100x faster than regex parsing
2. **Accuracy**: Semantic understanding vs pattern matching
3. **Maintainability**: Type-safe AST visitors vs brittle regex
4. **AI Integration**: Rich context for AI enhancement
5. **WASM Compatibility**: Native Rust performance in WASM runtime