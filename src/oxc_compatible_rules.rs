//! # OXC-Compatible Rule Implementation
//!
//! This module implements rules using the exact OXC pattern while maintaining
//! WASM compatibility and adding AI enhancements.
//!
//! ## Rule Template Structure
//!
//! Following OXC's exact `declare_oxc_lint!` macro pattern:
//! - Comprehensive documentation with examples
//! - Standardized categorization (correctness, suspicious, pedantic, style, etc.)
//! - Fix status classification (pending, fix, fix_dangerous, suggestion)
//! - Test cases with passing and failing examples

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use crate::ai_assistance::AiEnhancer;

use oxc_allocator::Allocator;
use oxc_ast::{AstKind, ast::Program};
use oxc_diagnostics::OxcDiagnostic;
use oxc_parser::Parser;
use oxc_semantic::{Semantic, SemanticBuilder};
use oxc_span::{SourceType, Span};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// WASM-compatible version of OXC's LintContext
pub struct WasmLintContext<'a> {
    pub semantic: &'a Semantic<'a>,
    pub source_text: &'a str,
    pub filename: &'a str,
    pub diagnostics: Vec<OxcDiagnostic>,
}

impl<'a> WasmLintContext<'a> {
    pub fn new(semantic: &'a Semantic<'a>, source_text: &'a str, filename: &'a str) -> Self {
        Self {
            semantic,
            source_text,
            filename,
            diagnostics: Vec::new(),
        }
    }

    /// Report a diagnostic (mirrors OXC's ctx.diagnostic())
    pub fn diagnostic(&mut self, diagnostic: OxcDiagnostic) {
        self.diagnostics.push(diagnostic);
    }
}

/// WASM-compatible version of OXC's AstNode
pub struct WasmAstNode<'a> {
    kind: &'a AstKind<'a>,
    id: usize,
}

impl<'a> WasmAstNode<'a> {
    pub fn new(kind: &'a AstKind<'a>, id: usize) -> Self {
        Self { kind, id }
    }

    pub fn kind(&self) -> &AstKind<'a> {
        self.kind
    }
}

/// WASM-compatible version of OXC's Rule trait
///
/// Follows OXC's exact Rule trait pattern but adapted for WASM safety.
/// Each rule implementation should follow the OXC template structure:
/// 1. Rule struct with Debug, Default, Clone derives
/// 2. Documentation following OXC's format
/// 3. Category and fix status classification
/// 4. Test cases with pass/fail examples
pub trait WasmRule {
    /// Rule name (used for configuration and reporting)
    fn name(&self) -> &'static str;

    /// Rule category (matching OXC categories)
    fn category(&self) -> WasmRuleCategory;

    /// Fix status (matching OXC fix status)
    fn fix_status(&self) -> WasmFixStatus;

    /// Run the rule on an AST node
    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>);

    /// Check if rule should run on this file
    fn should_run(&self, filename: &str) -> bool {
        filename.ends_with(".js")
            || filename.ends_with(".ts")
            || filename.ends_with(".jsx")
            || filename.ends_with(".tsx")
    }
}

/// Rule categories (matching OXC's exact categories)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WasmRuleCategory {
    /// Nursery rules (experimental)
    Nursery,
    /// Correctness rules (likely bugs)
    Correctness,
    /// Suspicious rules (code that looks wrong)
    Suspicious,
    /// Pedantic rules (nitpicky but useful)
    Pedantic,
    /// Performance rules (optimization opportunities)
    Perf,
    /// Restriction rules (enforce coding standards)
    Restriction,
    /// Style rules (formatting and conventions)
    Style,
}

/// Fix status (matching OXC's fix status)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WasmFixStatus {
    /// No fix available yet
    Pending,
    /// Safe automatic fix available
    Fix,
    /// Potentially unsafe automatic fix
    FixDangerous,
    /// Suggestion for manual fix
    Suggestion,
    /// Conditional fix with suggestions
    ConditionalFixSuggestion,
}

/// Enhanced rule that combines OXC pattern with AI capabilities
pub trait EnhancedWasmRule: WasmRule {
    /// Generate AI-enhanced suggestions for a diagnostic
    fn ai_enhance(&self, diagnostic: &OxcDiagnostic, source: &str) -> Vec<String> {
        // Default: no AI enhancement
        Vec::new()
    }

    /// Provide contextual explanation via AI
    fn ai_explain(&self, diagnostic: &OxcDiagnostic, source: &str) -> Option<String> {
        // Default: no AI explanation
        None
    }
}

/// No Empty Rule - Following OXC Template Structure
///
/// ### What it does
/// Disallows empty block statements and catch clauses.
///
/// ### Why is this bad?
/// Empty blocks can indicate incomplete implementation or unnecessary code structure.
/// They can confuse other developers about the intent of the code.
///
/// ### Examples
///
/// Examples of **incorrect** code:
/// ```js
/// if (foo) {
///     // empty block
/// }
///
/// try {
///     doSomething();
/// } catch (e) {
///     // empty catch
/// }
/// ```
///
/// Examples of **correct** code:
/// ```js
/// if (foo) {
///     // TODO: implement this
/// }
///
/// if (foo) {
///     doSomething();
/// }
///
/// try {
///     doSomething();
/// } catch (e) {
///     console.error(e);
/// }
/// ```
#[derive(Debug, Default, Clone)]
pub struct NoEmpty {
    allow_empty_catch: bool,
}

// WASM-compatible version of declare_oxc_lint! pattern
impl NoEmpty {
    pub const NAME: &'static str = "no-empty";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Suspicious;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoEmpty {
    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn category(&self) -> WasmRuleCategory {
        Self::CATEGORY
    }

    fn fix_status(&self) -> WasmFixStatus {
        Self::FIX_STATUS
    }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        match node.kind() {
            AstKind::BlockStatement(block) => {
                if block.body.is_empty() {
                    ctx.diagnostic(no_empty_diagnostic("block", block.span));
                }
            }
            AstKind::CatchClause(catch) => {
                if !self.allow_empty_catch && catch.body.body.is_empty() {
                    ctx.diagnostic(no_empty_diagnostic("catch clause", catch.span));
                }
            }
            _ => {}
        }
    }
}

fn no_empty_diagnostic(stmt_kind: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Empty block statement")
        .with_help(format!("Remove this {} or add a comment inside it", stmt_kind))
        .with_label(span)
}

impl EnhancedWasmRule for NoEmpty {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Consider adding error handling logic".to_string(),
            "Add a TODO comment explaining the intention".to_string(),
            "Remove the empty block if it's not needed".to_string(),
        ]
    }

    fn ai_explain(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Option<String> {
        Some("Empty blocks can indicate incomplete implementation or unnecessary code structure".to_string())
    }
}

/// Boolean Naming Rule - Following OXC Template Structure
///
/// ### What it does
/// Enforces descriptive prefixes for boolean variables to improve code readability.
///
/// ### Why is this bad?
/// Boolean variables without descriptive prefixes can be ambiguous and reduce code clarity.
/// Using conventional prefixes like 'is', 'has', 'should', 'can' makes the intent immediately clear.
///
/// ### Examples
///
/// Examples of **incorrect** code:
/// ```js
/// const enabled = true;
/// const visible = isElementVisible();
/// const valid = checkValidation();
/// ```
///
/// Examples of **correct** code:
/// ```js
/// const isEnabled = true;
/// const isVisible = isElementVisible();
/// const isValid = checkValidation();
/// const hasPermission = checkPermission();
/// const shouldUpdate = needsUpdate();
/// const canEdit = hasEditRights();
/// ```
#[derive(Debug, Default, Clone)]
pub struct BooleanNaming {
    allowed_prefixes: Vec<String>,
}

// WASM-compatible version of declare_oxc_lint! pattern
impl BooleanNaming {
    pub const NAME: &'static str = "boolean-naming";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl BooleanNaming {
    pub fn new() -> Self {
        Self {
            allowed_prefixes: vec![
                "is".to_string(),
                "has".to_string(),
                "should".to_string(),
                "can".to_string(),
                "will".to_string(),
                "must".to_string(),
            ],
        }
    }

    fn is_boolean_expression(&self, expr: &oxc_ast::ast::Expression) -> bool {
        use oxc_ast::ast::Expression;

        match expr {
            Expression::BooleanLiteral(_) => true,
            Expression::UnaryExpression(unary) => unary.operator.is_not(),
            Expression::BinaryExpression(binary) => {
                binary.operator.is_equality() || binary.operator.is_compare()
            }
            _ => false,
        }
    }

    fn has_boolean_prefix(&self, name: &str) -> bool {
        let lower_name = name.to_lowercase();
        self.allowed_prefixes.iter().any(|prefix| {
            lower_name.starts_with(&prefix.to_lowercase()) && name.len() > prefix.len()
        })
    }

    fn get_variable_name(&self, declarator: &oxc_ast::ast::VariableDeclarator) -> Option<String> {
        use oxc_ast::ast::{BindingPattern, BindingPatternKind};

        if let BindingPattern { kind: BindingPatternKind::BindingIdentifier(ident), .. } = &declarator.id {
            Some(ident.name.to_string())
        } else {
            None
        }
    }
}

impl WasmRule for BooleanNaming {
    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn category(&self) -> WasmRuleCategory {
        Self::CATEGORY
    }

    fn fix_status(&self) -> WasmFixStatus {
        Self::FIX_STATUS
    }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::VariableDeclarator(declarator) = node.kind() {
            if let Some(init) = &declarator.init {
                if self.is_boolean_expression(init) {
                    if let Some(name) = self.get_variable_name(declarator) {
                        if !self.has_boolean_prefix(&name) {
                            ctx.diagnostic(boolean_naming_diagnostic(&name, declarator.span));
                        }
                    }
                }
            }
        }
    }
}

fn boolean_naming_diagnostic(var_name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Boolean variable should have descriptive prefix")
        .with_help(format!("Consider renaming '{}' to use prefixes like 'is', 'has', 'should', 'can'", var_name))
        .with_label(span)
}

impl EnhancedWasmRule for BooleanNaming {
    fn ai_enhance(&self, diagnostic: &OxcDiagnostic, source: &str) -> Vec<String> {
        // Extract variable name from diagnostic context
        // This would use AI to generate contextual suggestions
        vec![
            "Use 'is' prefix for state checks".to_string(),
            "Use 'has' prefix for ownership/possession".to_string(),
            "Use 'should' prefix for conditional actions".to_string(),
        ]
    }

    fn ai_explain(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Option<String> {
        Some("Boolean variables with descriptive prefixes improve code readability and make intent clearer to other developers".to_string())
    }
}

/// WASM-compatible rule engine that follows OXC patterns
pub struct WasmRuleEngine {
    rules: Vec<Box<dyn WasmRule>>,
    enhanced_rules: Vec<Box<dyn EnhancedWasmRule>>,
    ai_enhancer: Option<AiEnhancer>,
}

impl WasmRuleEngine {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            enhanced_rules: Vec::new(),
            ai_enhancer: None,
        }
    }

    pub fn add_rule<R: WasmRule + 'static>(&mut self, rule: R) {
        self.rules.push(Box::new(rule));
    }

    pub fn add_enhanced_rule<R: EnhancedWasmRule + 'static>(&mut self, rule: R) {
        self.enhanced_rules.push(Box::new(rule));
    }

    pub fn with_ai_enhancer(&mut self, enhancer: AiEnhancer) {
        self.ai_enhancer = Some(enhancer);
    }

    /// Lint source code using OXC-compatible patterns
    pub fn lint(&self, source: &str, filename: &str) -> anyhow::Result<Vec<LintIssue>> {
        // Parse with OXC
        let allocator = Allocator::default();
        let source_type = SourceType::from_path(filename)
            .unwrap_or_else(|_| SourceType::default().with_typescript(true));

        let parser_result = Parser::new(&allocator, source, source_type).parse();

        if !parser_result.errors.is_empty() {
            return Ok(parser_result.errors
                .into_iter()
                .map(|error| LintIssue {
                    rule_name: "parser".to_string(),
                    message: format!("Parse error: {}", error.message),
                    line: 1,
                    column: 1,
                    severity: LintSeverity::Error,
                    fix_available: false,
                })
                .collect());
        }

        let program = parser_result.program;
        let semantic = SemanticBuilder::new().build(&program);

        // Create WASM lint context
        let mut context = WasmLintContext::new(&semantic, source, filename);

        // Walk AST and run rules (simplified for demonstration)
        self.walk_program(&program, &mut context);

        // Convert OXC diagnostics to LintIssues
        let mut lint_issues: Vec<LintIssue> = context.diagnostics
            .into_iter()
            .map(|diag| self.oxc_diagnostic_to_lint_issue(diag, source))
            .collect();

        // Apply AI enhancements if available
        if let Some(ref ai_enhancer) = self.ai_enhancer {
            lint_issues = self.apply_ai_enhancements(lint_issues, source)?;
        }

        Ok(lint_issues)
    }

    fn walk_program(&self, program: &Program, context: &mut WasmLintContext) {
        // Simplified AST walking - in practice, would use proper visitor pattern
        // This demonstrates the OXC-compatible rule execution pattern

        let ast_kind = AstKind::Program(program);
        // Ensure ast_kind lives long enough for all node uses
        {
            let node = WasmAstNode::new(&ast_kind, 0);

            // Run regular rules
            for rule in &self.rules {
                if rule.should_run(context.filename) {
                    rule.run(&node, context);
                }
            }

            // Run enhanced rules
            for rule in &self.enhanced_rules {
                if rule.should_run(context.filename) {
                    rule.run(&node, context);
                }
            }
        }
        // ast_kind dropped here, after all uses
    }

    fn oxc_diagnostic_to_lint_issue(&self, diagnostic: OxcDiagnostic, source: &str) -> LintIssue {
        let (line, column) = self.span_to_line_col(diagnostic.labels.first().map(|l| l.span).unwrap_or_default(), source);

        LintIssue {
            message: diagnostic.message.clone(),
            severity: LintSeverity::Warning, // Map from OXC severity
            line,
            column,
            rule_name: "oxc-rule".to_string(),
            fix_available: diagnostic.help.is_some(),
        }
    }

    fn span_to_line_col(&self, span: Span, source: &str) -> (u32, u32) {
        let mut line = 1;
        let mut column = 1;
        let mut current_pos = 0;

        for ch in source.chars() {
            if current_pos >= span.start as usize {
                break;
            }

            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }

            current_pos += ch.len_utf8();
        }

        (line, column)
    }

    fn apply_ai_enhancements(&self, mut issues: Vec<LintIssue>, source: &str) -> anyhow::Result<Vec<LintIssue>> {
        // Apply AI enhancements to each issue
        for issue in &mut issues {
            // Find corresponding enhanced rule and apply AI
            for enhanced_rule in &self.enhanced_rules {
                if enhanced_rule.NAME == issue.rule_id {
                    // Create dummy diagnostic for AI enhancement
                    let dummy_diagnostic = OxcDiagnostic::warn(&issue.message);

                    let ai_suggestions = enhanced_rule.ai_enhance(&dummy_diagnostic, source);
                    let ai_explanation = enhanced_rule.ai_explain(&dummy_diagnostic, source);

                    if !ai_suggestions.is_empty() {
                        issue.message = format!("{} (AI: {})", issue.message, ai_suggestions.join(", "));
                    }

                    if let Some(explanation) = ai_explanation {
                        issue.suggestion = Some(explanation);
                    }
                    break;
                }
            }
        }

        Ok(issues)
    }
}

impl Default for WasmRuleEngine {
    fn default() -> Self {
        let mut engine = Self::new();

        // Add OXC-compatible rules
        engine.add_enhanced_rule(NoEmpty::default());
        engine.add_enhanced_rule(BooleanNaming::new());

        engine
    }
}

/// Test infrastructure following OXC's snapshot testing pattern
///
/// Note: In a full OXC implementation, this would use cargo insta for snapshot testing.
/// For our WASM-safe implementation, we provide the test structure that could be
/// integrated with Moon's testing framework.
#[cfg(test)]
mod tests {
    use super::*;

    /// Test helper for validating rule implementations
    /// (WASM-compatible alternative to OXC's Tester)
    pub struct WasmTester {
        rule_name: &'static str,
        pass_cases: Vec<&'static str>,
        fail_cases: Vec<&'static str>,
    }

    impl WasmTester {
        pub fn new(
            rule_name: &'static str,
            pass_cases: Vec<&'static str>,
            fail_cases: Vec<&'static str>,
        ) -> Self {
            Self {
                rule_name,
                pass_cases,
                fail_cases,
            }
        }

        pub fn test_rule<R: WasmRule + Default>(&self) -> anyhow::Result<()> {
            let rule = R::default();
            let engine = WasmRuleEngine::new();

            // Test pass cases - should not generate diagnostics
            for (i, code) in self.pass_cases.iter().enumerate() {
                let result = engine.lint(code, "test.js")?;
                assert!(
                    result.is_empty(),
                    "Expected no lint issues for pass case {}: {}",
                    i,
                    code
                );
            }

            // Test fail cases - should generate diagnostics
            for (i, code) in self.fail_cases.iter().enumerate() {
                let result = engine.lint(code, "test.js")?;
                assert!(
                    !result.is_empty(),
                    "Expected lint issues for fail case {}: {}",
                    i,
                    code
                );
            }

            Ok(())
        }
    }

    #[test]
    fn test_no_empty() {
        let pass = vec![
            "if (foo) { bar(); }",
            "if (foo) { /* comment */ }",
            "try { doSomething(); } catch (e) { handle(e); }",
        ];

        let fail = vec![
            "if (foo) {}",
            "while (foo) {}",
            "try { doSomething(); } catch (e) {}",
        ];

        let tester = WasmTester::new(NoEmpty::NAME, pass, fail);
        tester.test_rule::<NoEmpty>().expect("NoEmpty tests should pass");
    }

    #[test]
    fn test_boolean_naming() {
        let pass = vec![
            "const isValid = true;",
            "const hasPermission = checkAuth();",
            "const shouldUpdate = needsUpdate();",
            "const canEdit = hasRights();",
        ];

        let fail = vec![
            "const valid = true;",
            "const enabled = checkEnabled();",
            "const visible = isVisible();",
        ];

        let tester = WasmTester::new(BooleanNaming::NAME, pass, fail);
        tester.test_rule::<BooleanNaming>().expect("BooleanNaming tests should pass");
    }

    #[test]
    fn test_rule_categories() {
        // Verify our rule categories match OXC's pattern
        assert_eq!(NoEmpty::CATEGORY, WasmRuleCategory::Suspicious);
        assert_eq!(BooleanNaming::CATEGORY, WasmRuleCategory::Style);
    }

    #[test]
    fn test_fix_status() {
        // Verify fix status classifications
        assert_eq!(NoEmpty::FIX_STATUS, WasmFixStatus::Suggestion);
        assert_eq!(BooleanNaming::FIX_STATUS, WasmFixStatus::Suggestion);
    }
}