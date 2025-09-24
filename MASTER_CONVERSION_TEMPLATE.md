# 🎯 Master Rule Conversion Template - Swiss Precision

**Corrected OXC AST Visitor Pattern for MoonShine Rule Engine**

## ✅ **Verified Template Structure**

```rust
//! # [RULE_ID]: [Rule Name] Rule
//!
//! [Description of what the rule checks and why it matters]
//! [Benefits for code quality/security/performance]
//!
//! @category [category]-rules
//! @safe team
//! @mvp [core|enhanced]
//! @complexity [low|medium|high]
//! @since 2.1.0

use crate::javascript_typescript_linter::{LintIssue, LintSeverity};
use crate::rules::utils::span_to_line_col_legacy;
use oxc_ast::ast::{Program, [specific AST types needed]};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct [RuleID]Options {
    /// [Configuration option description]
    pub [option_name]: Option<[type]>,
}

/// [RULE_ID] rule implementation with AI enhancement
pub fn check_[rule_function_name](program: &Program, semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let options = [RuleID]Options::default();
    let mut visitor = [RuleName]Visitor::new(program, code, options);
    visitor.visit_program(program);
    visitor.issues
}

struct [RuleName]Visitor<'a> {
    program: &'a Program<'a>,
    code: &'a str,
    issues: Vec<LintIssue>,
    // Rule-specific state from original JavaScript
}

impl<'a> [RuleName]Visitor<'a> {
    fn new(program: &'a Program<'a>, code: &'a str, options: [RuleID]Options) -> Self {
        Self {
            program,
            code,
            issues: Vec::new(),
            // Initialize rule state with Swiss precision
        }
    }

    // Helper methods ported from JavaScript with exact logic preservation
}

impl<'a> Visit<'a> for [RuleName]Visitor<'a> {
    // ✅ CORRECT PATTERN: Visit specific AST node types
    fn visit_[ast_node_type](&mut self, node: &[AstNodeType]<'a>) {
        // 1. Rule-specific analysis logic here
        if self.should_report_issue(node) {
            let (line, column) = span_to_line_col_legacy(self.program, node.span);
            self.issues.push(LintIssue {
                rule_name: "[RULE_ID]".to_string(),
                severity: LintSeverity::Warning, // Or Error based on original
                message: format!("[Exact message from JavaScript rule]"),
                line,
                column,
                fix_available: false, // AI can enhance this
            });
        }

        // 2. Continue visiting child nodes - USE THESE PATTERNS:

        // Option A: Call specific child visit methods (RECOMMENDED)
        if let Some(child) = &node.child_node {
            self.visit_[child_type](child);
        }

        // Option B: Use walk pattern for complex traversal
        walk_[ast_node_type](self, node);

        // ❌ NEVER DO THIS: self.visit_[same_method](node); // Infinite recursion!
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_parser::{Parser, ParseOptions};
    use oxc_semantic::SemanticBuilder;
    use oxc_span::SourceType;

    fn parse_and_check(code: &str) -> Vec<LintIssue> {
        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);

        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        check_[rule_function_name](&parse_result.program, &semantic_result.semantic, code)
    }

    #[test]
    fn test_[rule_name]_violation() {
        let code = r#"[Code that should trigger the rule]"#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues[0].message.contains("[key phrase from message]"));
    }

    #[test]
    fn test_[rule_name]_compliant() {
        let code = r#"[Code that should NOT trigger the rule]"#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }
}
```

## 🚨 **Critical Fix: Visitor Pattern**

### ❌ **WRONG Pattern (Infinite Recursion)**
```rust
impl<'a> Visit<'a> for MyVisitor<'a> {
    fn visit_function_declaration(&mut self, node: &FunctionDeclaration<'a>) {
        // Do analysis
        self.analyze_function(node);

        // ❌ THIS CAUSES INFINITE RECURSION:
        self.visit_function_declaration(node);
    }
}
```

### ✅ **CORRECT Pattern (Child Traversal)**
```rust
impl<'a> Visit<'a> for MyVisitor<'a> {
    fn visit_function_declaration(&mut self, node: &FunctionDeclaration<'a>) {
        // Do analysis
        self.analyze_function(node);

        // ✅ CORRECT: Visit specific child nodes
        if let Some(body) = &node.body {
            self.visit_block_statement(body);
        }

        // Visit parameters
        for param in &node.params.items {
            self.visit_formal_parameter(param);
        }

        // Or use walk pattern for automatic traversal:
        // walk_function_declaration(self, node);
    }
}
```

## 🎯 **Common AST Visitor Patterns**

### **Function Patterns**
```rust
// Function Declaration
fn visit_function_declaration(&mut self, node: &FunctionDeclaration<'a>) {
    // Analysis logic
    if let Some(body) = &node.body {
        self.visit_block_statement(body);
    }
    for param in &node.params.items {
        self.visit_formal_parameter(param);
    }
}

// Arrow Function
fn visit_arrow_function_expression(&mut self, node: &ArrowFunctionExpression<'a>) {
    // Analysis logic
    if !node.expression {
        if let Some(body) = node.body.as_block_statement() {
            self.visit_block_statement(body);
        }
    }
    for param in &node.params.items {
        self.visit_formal_parameter(param);
    }
}
```

### **Control Flow Patterns**
```rust
// If Statement
fn visit_if_statement(&mut self, node: &IfStatement<'a>) {
    // Analysis logic
    self.visit_expression(&node.test);
    self.visit_statement(&node.consequent);
    if let Some(alternate) = &node.alternate {
        self.visit_statement(alternate);
    }
}

// Loop Statements
fn visit_for_statement(&mut self, node: &ForStatement<'a>) {
    // Analysis logic
    if let Some(init) = &node.init {
        // Handle ForStatementInit enum
    }
    if let Some(test) = &node.test {
        self.visit_expression(test);
    }
    if let Some(update) = &node.update {
        self.visit_expression(update);
    }
    self.visit_statement(&node.body);
}
```

### **Variable/Binding Patterns**
```rust
// Variable Declarator
fn visit_variable_declarator(&mut self, node: &VariableDeclarator<'a>) {
    // Analysis logic
    self.visit_binding_pattern(&node.id);
    if let Some(init) = &node.init {
        self.visit_expression(init);
    }
}

// Binding Pattern (handles destructuring)
fn check_binding_pattern(&mut self, pattern: &BindingPattern) {
    match &pattern.kind {
        BindingPatternKind::BindingIdentifier(ident) => {
            // Handle identifier
        }
        BindingPatternKind::ObjectPattern(obj_pattern) => {
            for prop in &obj_pattern.properties {
                self.check_binding_pattern(&prop.value);
            }
        }
        BindingPatternKind::ArrayPattern(arr_pattern) => {
            for element in &arr_pattern.elements {
                if let Some(element) = element {
                    self.check_binding_pattern(element);
                }
            }
        }
        BindingPatternKind::AssignmentPattern(assign_pattern) => {
            self.check_binding_pattern(&assign_pattern.left);
        }
    }
}
```

## 📋 **Rule Registration Template**

```rust
// In code_quality/mod.rs (or appropriate category)
pub mod [rule_id]_[rule_name];

// In register_rules function
rules.insert("[RULE_ID]".to_string(), MoonShineRule {
    id: "[RULE_ID]".to_string(),
    category: MoonShineRuleCategory::[Category],
    severity: LintSeverity::Warning,
    description: "[Rule description]".to_string(),
    ai_enhanced: true,
    implementation: RuleImplementation::AiAssisted,
});

// In check_ai_rule function
"[RULE_ID]" => [module_name]::[function_name](program, semantic, code),

// In check_oxc_ast_visitor_rule function
"[RULE_ID]" => [module_name]::[function_name](program, semantic, code),
```

## ✅ **Quality Gates Checklist**

- [ ] Compiles without warnings
- [ ] Uses correct Visit pattern (no infinite recursion)
- [ ] Preserves exact JavaScript behavior and error messages
- [ ] Includes comprehensive tests (violation + compliant)
- [ ] AI enhancement configured (`ai_enhanced: true`)
- [ ] Registered in appropriate dispatcher
- [ ] Documentation complete with JSDoc-style comments

---

**This template ensures Swiss watchmaker precision with correct OXC AST visitor patterns for 10-100x performance over regex-based analysis.**