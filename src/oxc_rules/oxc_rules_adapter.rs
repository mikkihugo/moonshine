//! # OXC Rules Adapter for WASM
//!
//! Adapts OXC rule implementations for WASM-safe execution while maintaining
//! compatibility with OXC's AST parsing and diagnostic patterns.
//!
//! ## Architecture
//!
//! 1. **Use OXC AST parsing** (WASM-safe)
//! 2. **Adapt OXC rule logic** (extract the rule implementations)
//! 3. **Add AI enhancements** (our unique value)
//! 4. **WASM-compatible execution** (Moon extension runtime)

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use crate::ai_assistance::AiEnhancer;

use oxc_allocator::Allocator;
use oxc_ast::{AstKind, ast::Program};
use oxc_ast_visit::{Visit, walk};
use oxc_diagnostics::OxcDiagnostic;
use oxc_parser::Parser;
use oxc_semantic::{Semantic, SemanticBuilder};
use oxc_span::{SourceType, Span};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// WASM-safe rule trait (adapted from OXC's Rule trait)
///
/// This trait follows OXC's exact Rule trait pattern but adapted for WASM execution.
/// Each rule should follow the OXC template structure:
/// 1. Comprehensive documentation with examples
/// 2. Standardized categorization matching OXC categories
/// 3. Fix status classification
/// 4. Test cases with pass/fail examples
pub trait WasmSafeRule {
    /// Rule identifier (const NAME pattern from OXC)
    fn name(&self) -> &'static str;

    /// Rule category (matching OXC categories exactly)
    fn category(&self) -> RuleCategory;

    /// Fix status (matching OXC fix status)
    fn fix_status(&self) -> FixStatus;

    /// Run the rule on an AST node (mirrors OXC's run method)
    fn check_node(&self, node: &AstKind, context: &RuleContext) -> Vec<RuleDiagnostic>;

    /// Check if rule should run on this file
    fn should_run(&self, filename: &str) -> bool {
        // Default: run on all JS/TS files (same as OXC)
        filename.ends_with(".js")
            || filename.ends_with(".ts")
            || filename.ends_with(".jsx")
            || filename.ends_with(".tsx")
    }
}

/// Rule category (matching OXC's exact categories)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCategory {
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

/// Fix status (matching OXC's fix status exactly)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixStatus {
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

/// Rule execution context
pub struct RuleContext<'a> {
    pub semantic: &'a Semantic<'a>,
    pub source_text: &'a str,
    pub filename: &'a str,
}

/// Rule diagnostic result
#[derive(Debug, Clone)]
pub struct RuleDiagnostic {
    pub message: String,
    pub span: Span,
    pub severity: DiagnosticSeverity,
    pub help: Option<String>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
}

/// WASM-safe rule engine
pub struct WasmRuleEngine {
    rules: Vec<Box<dyn WasmSafeRule>>,
    ai_enhancer: Option<AiEnhancer>,
}

impl WasmRuleEngine {
    /// Create new rule engine
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            ai_enhancer: None,
        }
    }

    /// Add a rule to the engine
    pub fn add_rule(&mut self, rule: Box<dyn WasmSafeRule>) {
        self.rules.push(rule);
    }

    /// Set AI enhancer
    pub fn with_ai_enhancer(&mut self, enhancer: AiEnhancer) {
        self.ai_enhancer = Some(enhancer);
    }

    /// Lint source code
    pub fn lint(&self, source: &str, filename: &str) -> anyhow::Result<Vec<LintIssue>> {
        // Parse with OXC
        let allocator = Allocator::default();
        let source_type = SourceType::from_path(filename)
            .unwrap_or_else(|_| SourceType::default().with_typescript(true));

        let parser_result = Parser::new(&allocator, source, source_type).parse();

        if !parser_result.errors.is_empty() {
            // Return parse errors
            return Ok(parser_result.errors
                .into_iter()
                .map(|error| LintIssue {
                    rule_name: "parser".to_string(),
                    message: format!("Parse error: {}", error.message),
                    line: 1, // TODO: Extract from span
                    column: 1, // TODO: Extract from span
                    severity: LintSeverity::Error,
                    fix_available: false,
                })
                .collect());
        }

        let program = parser_result.program;
        let semantic = SemanticBuilder::new().build(&program);

        // Run rules
        let context = RuleContext {
            semantic: &semantic,
            source_text: source,
            filename,
        };

        let mut diagnostics = Vec::new();

        // Walk AST and run rules
        let mut rule_visitor = RuleVisitor {
            rules: &self.rules,
            context: &context,
            diagnostics: &mut diagnostics,
        };

        rule_visitor.visit_program(&program);

        // Convert to LintIssues
        let mut lint_issues: Vec<LintIssue> = diagnostics
            .into_iter()
            .map(|diag| self.rule_diagnostic_to_lint_issue(diag, source))
            .collect();

        // Apply AI enhancements
        if let Some(ref ai_enhancer) = self.ai_enhancer {
            lint_issues = ai_enhancer.enhance_lint_issues(lint_issues, source, &program, &semantic)?;
        }

        Ok(lint_issues)
    }

    fn rule_diagnostic_to_lint_issue(&self, diagnostic: RuleDiagnostic, source: &str) -> LintIssue {
        let (line, column) = self.span_to_line_col(diagnostic.span, source);

        LintIssue {
            message: diagnostic.message,
            severity: match diagnostic.severity {
                DiagnosticSeverity::Error => LintSeverity::Error,
                DiagnosticSeverity::Warning => LintSeverity::Warning,
                DiagnosticSeverity::Info => LintSeverity::Info,
            },
            line,
            column,
            rule_name: "oxc-adapted".to_string(), // TODO: Get from rule
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
}

/// AST visitor that runs rules on each node
struct RuleVisitor<'a> {
    rules: &'a [Box<dyn WasmSafeRule>],
    context: &'a RuleContext<'a>,
    diagnostics: &'a mut Vec<RuleDiagnostic>,
}

impl<'a> Visit<'a> for RuleVisitor<'a> {
    fn visit_program(&mut self, program: &Program<'a>) {
        self.check_node(&AstKind::Program(program));
        walk::walk_program(self, program);
    }

    fn visit_function(&mut self, func: &oxc_ast::ast::Function<'a>, _flags: oxc_semantic::ScopeFlags) {
        self.check_node(&AstKind::Function(func));
        walk::walk_function(self, func, _flags);
    }

    fn visit_expression(&mut self, expr: &oxc_ast::ast::Expression<'a>) {
        // Check for function expressions and other expressions
        self.check_node(&AstKind::Expression(expr));
        walk::walk_expression(self, expr);
    }

    fn visit_arrow_function_expression(&mut self, func: &oxc_ast::ast::ArrowFunctionExpression<'a>) {
        self.check_node(&AstKind::ArrowFunctionExpression(func));
        walk::walk_arrow_function_expression(self, func);
    }

    fn visit_variable_declarator(&mut self, declarator: &oxc_ast::ast::VariableDeclarator<'a>) {
        self.check_node(&AstKind::VariableDeclarator(declarator));
        walk::walk_variable_declarator(self, declarator);
    }

    fn visit_catch_clause(&mut self, catch: &oxc_ast::ast::CatchClause<'a>) {
        self.check_node(&AstKind::CatchClause(catch));
        walk::walk_catch_clause(self, catch);
    }

    fn visit_block_statement(&mut self, block: &oxc_ast::ast::BlockStatement<'a>) {
        self.check_node(&AstKind::BlockStatement(block));
        walk::walk_block_statement(self, block);
    }

    fn visit_method_definition(&mut self, method: &oxc_ast::ast::MethodDefinition<'a>) {
        self.check_node(&AstKind::MethodDefinition(method));
        walk::walk_method_definition(self, method);
    }
}

impl<'a> RuleVisitor<'a> {
    fn check_node(&mut self, node: &AstKind) {
        for rule in self.rules {
            if rule.should_run(self.context.filename) {
                let rule_diagnostics = rule.check_node(node, self.context);
                self.diagnostics.extend(rule_diagnostics);
            }
        }
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
pub struct NoEmptyRule {
    allow_empty_catch: bool,
}

// Following OXC's const pattern for rule metadata
impl NoEmptyRule {
    pub const NAME: &'static str = "no-empty";
    pub const CATEGORY: RuleCategory = RuleCategory::Suspicious;
    pub const FIX_STATUS: FixStatus = FixStatus::Suggestion;

    pub fn new(allow_empty_catch: bool) -> Self {
        Self { allow_empty_catch }
    }
}

impl WasmSafeRule for NoEmptyRule {
    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn category(&self) -> RuleCategory {
        Self::CATEGORY
    }

    fn fix_status(&self) -> FixStatus {
        Self::FIX_STATUS
    }

    fn check_node(&self, node: &AstKind, _context: &RuleContext) -> Vec<RuleDiagnostic> {
        let mut diagnostics = Vec::new();

        match node {
            AstKind::BlockStatement(block) => {
                if block.body.is_empty() {
                    diagnostics.push(RuleDiagnostic {
                        message: "Empty block statement".to_string(),
                        span: block.span,
                        severity: DiagnosticSeverity::Warning,
                        help: Some("Add code or a comment to explain why this block is empty".to_string()),
                        suggestions: vec!["Add a comment".to_string(), "Remove empty block".to_string()],
                    });
                }
            }
            AstKind::CatchClause(catch) => {
                if !self.allow_empty_catch && catch.body.body.is_empty() {
                    diagnostics.push(RuleDiagnostic {
                        message: "Empty catch clause".to_string(),
                        span: catch.span,
                        severity: DiagnosticSeverity::Warning,
                        help: Some("Handle the error or add a comment explaining why it's ignored".to_string()),
                        suggestions: vec!["Add error handling".to_string(), "Add explanatory comment".to_string()],
                    });
                }
            }
            _ => {}
        }

        diagnostics
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
pub struct BooleanNamingRule {
    allowed_prefixes: Vec<String>,
}

// Following OXC's const pattern for rule metadata
impl BooleanNamingRule {
    pub const NAME: &'static str = "boolean-naming";
    pub const CATEGORY: RuleCategory = RuleCategory::Style;
    pub const FIX_STATUS: FixStatus = FixStatus::Suggestion;

    pub fn new() -> Self {
        Self {
            allowed_prefixes: vec![
                "is".to_string(),
                "has".to_string(),
                "should".to_string(),
                "can".to_string(),
                "will".to_string(),
                "must".to_string(),
                "may".to_string(),
                "check".to_string(),
            ],
        }
    }
}

impl WasmSafeRule for BooleanNamingRule {
    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn category(&self) -> RuleCategory {
        Self::CATEGORY
    }

    fn fix_status(&self) -> FixStatus {
        Self::FIX_STATUS
    }

    fn check_node(&self, node: &AstKind, _context: &RuleContext) -> Vec<RuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if let AstKind::VariableDeclarator(declarator) = node {
            if let Some(init) = &declarator.init {
                if self.is_boolean_expression(init) {
                    if let Some(name) = self.get_variable_name(declarator) {
                        if !self.has_boolean_prefix(&name) {
                            diagnostics.push(RuleDiagnostic {
                                message: format!("Boolean variable '{}' should have a descriptive prefix", name),
                                span: declarator.span,
                                severity: DiagnosticSeverity::Warning,
                                help: Some(format!("Consider renaming to 'is{}', 'has{}', or 'should{}'",
                                    self.capitalize(&name), self.capitalize(&name), self.capitalize(&name))),
                                suggestions: vec![
                                    format!("is{}", self.capitalize(&name)),
                                    format!("has{}", self.capitalize(&name)),
                                    format!("should{}", self.capitalize(&name)),
                                ],
                            });
                        }
                    }
                }
            }
        }

        diagnostics
    }
}

impl BooleanNamingRule {
    fn is_boolean_expression(&self, expr: &oxc_ast::ast::Expression) -> bool {
        use oxc_ast::ast::Expression;

        match expr {
            Expression::BooleanLiteral(_) => true,
            Expression::UnaryExpression(unary) => unary.operator.is_not(),
            Expression::BinaryExpression(binary) => {
                matches!(binary.operator,
                    oxc_ast::ast::BinaryOperator::Equality |
                    oxc_ast::ast::BinaryOperator::Inequality |
                    oxc_ast::ast::BinaryOperator::StrictEquality |
                    oxc_ast::ast::BinaryOperator::StrictInequality |
                    oxc_ast::ast::BinaryOperator::LessThan |
                    oxc_ast::ast::BinaryOperator::LessEqualThan |
                    oxc_ast::ast::BinaryOperator::GreaterThan |
                    oxc_ast::ast::BinaryOperator::GreaterEqualThan
                )
            }
            Expression::LogicalExpression(_) => true,
            _ => false,
        }
    }

    fn get_variable_name(&self, declarator: &oxc_ast::ast::VariableDeclarator) -> Option<String> {
        use oxc_ast::ast::{BindingPattern, BindingPatternKind};

        if let BindingPattern { kind: BindingPatternKind::BindingIdentifier(ident), .. } = &declarator.id {
            Some(ident.name.to_string())
        } else {
            None
        }
    }

    fn has_boolean_prefix(&self, name: &str) -> bool {
        let lower_name = name.to_lowercase();
        self.allowed_prefixes.iter().any(|prefix| {
            lower_name.starts_with(&prefix.to_lowercase()) && name.len() > prefix.len()
        })
    }

    fn capitalize(&self, s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

impl Default for WasmRuleEngine {
    fn default() -> Self {
        let mut engine = Self::new();

        // Add adapted OXC rules
        engine.add_rule(Box::new(NoEmptyRule::new(false)));
        engine.add_rule(Box::new(BooleanNamingRule::new()));

        // TODO: Add more adapted OXC rules

        engine
    }
}