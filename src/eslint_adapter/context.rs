//! # ESLint Context API Mapping
//!
//! This module provides an ESLint-compatible context object that maps ESLint's
//! rule context API to our OXC-based diagnostic system. This allows ESLint rules
//! to be executed using the same API they expect, while leveraging our fast
//! Rust-based implementation.

use crate::types::{DiagnosticSeverity, LintDiagnostic};
use oxc_ast::ast::{Expression, Node, Program};
use oxc_semantic::{Semantic, ScopeTree};
use oxc_span::{GetSpan, Span};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ESLint-compatible rule context
///
/// This struct provides the same interface that ESLint rules expect,
/// but maps calls to our internal diagnostic and AST systems.
pub struct ESLintRuleContext<'a> {
    /// Source code being analyzed
    source_code: &'a str,
    /// File path being analyzed
    file_path: &'a str,
    /// OXC AST program
    program: &'a Program<'a>,
    /// OXC semantic analysis
    semantic: Option<&'a Semantic<'a>>,
    /// Rule configuration options
    options: Vec<serde_json::Value>,
    /// Collected diagnostics
    diagnostics: Vec<LintDiagnostic>,
    /// Rule metadata
    rule_id: String,
    severity: DiagnosticSeverity,
}

impl<'a> ESLintRuleContext<'a> {
    pub fn new(
        source_code: &'a str,
        file_path: &'a str,
        program: &'a Program<'a>,
        semantic: Option<&'a Semantic<'a>>,
        rule_id: String,
        severity: DiagnosticSeverity,
        options: Vec<serde_json::Value>,
    ) -> Self {
        Self {
            source_code,
            file_path,
            program,
            semantic,
            options,
            diagnostics: Vec::new(),
            rule_id,
            severity,
        }
    }

    /// ESLint's context.report() method
    ///
    /// This is the primary method ESLint rules use to report problems.
    /// We map it to our internal diagnostic system.
    pub fn report(&mut self, report: ESLintReport) {
        let (line, column) = self.get_line_column_from_span(report.span);

        let diagnostic = LintDiagnostic {
            rule_name: self.rule_id.clone(),
            message: report.message,
            file_path: self.file_path.to_string(),
            line,
            column,
            severity: self.severity,
            fix_available: report.fix.is_some(),
        };

        self.diagnostics.push(diagnostic);
    }

    /// ESLint's context.getSourceCode() method
    ///
    /// Returns source code information that ESLint rules expect
    pub fn get_source_code(&self) -> ESLintSourceCode<'a> {
        ESLintSourceCode::new(self.source_code, self.program, self.semantic)
    }

    /// ESLint's context.getScope() method
    ///
    /// Returns scope information for the given node
    pub fn get_scope(&self, node: Option<&dyn Node<'a>>) -> Option<ESLintScope> {
        if let Some(semantic) = self.semantic {
            // Map OXC scope to ESLint scope format
            // For now, return a basic scope - this can be expanded
            let _ = (node, semantic);
            Some(ESLintScope::new())
        } else {
            None
        }
    }

    /// ESLint's context.options property
    ///
    /// Returns rule configuration options
    pub fn get_options(&self) -> &[serde_json::Value] {
        &self.options
    }

    /// Get collected diagnostics
    pub fn get_diagnostics(self) -> Vec<LintDiagnostic> {
        self.diagnostics
    }

    /// Convert span to line/column position
    fn get_line_column_from_span(&self, span: Span) -> (u32, u32) {
        let start = span.start.min(self.source_code.len() as u32) as usize;
        let prefix = &self.source_code[..start];
        let line = prefix.lines().count() as u32 + 1;
        let column = prefix
            .rsplit_once('\n')
            .map(|(_, tail)| tail.chars().count() as u32 + 1)
            .unwrap_or(prefix.chars().count() as u32 + 1);
        (line, column)
    }
}

/// ESLint report structure
///
/// Represents a problem report in ESLint's format
#[derive(Debug, Clone)]
pub struct ESLintReport {
    pub span: Span,
    pub message: String,
    pub fix: Option<ESLintFix>,
    pub suggest: Option<Vec<ESLintSuggestion>>,
}

impl ESLintReport {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
            fix: None,
            suggest: None,
        }
    }

    pub fn with_fix(mut self, fix: ESLintFix) -> Self {
        self.fix = Some(fix);
        self
    }

    pub fn with_suggestions(mut self, suggestions: Vec<ESLintSuggestion>) -> Self {
        self.suggest = Some(suggestions);
        self
    }
}

/// ESLint fix structure
///
/// Represents an automatic fix in ESLint's format
#[derive(Debug, Clone)]
pub struct ESLintFix {
    pub range: (u32, u32),
    pub text: String,
}

/// ESLint suggestion structure
///
/// Represents a fix suggestion in ESLint's format
#[derive(Debug, Clone)]
pub struct ESLintSuggestion {
    pub desc: String,
    pub fix: ESLintFix,
}

/// ESLint-compatible source code interface
///
/// Provides source code analysis methods that ESLint rules expect
pub struct ESLintSourceCode<'a> {
    source: &'a str,
    program: &'a Program<'a>,
    semantic: Option<&'a Semantic<'a>>,
}

impl<'a> ESLintSourceCode<'a> {
    pub fn new(
        source: &'a str,
        program: &'a Program<'a>,
        semantic: Option<&'a Semantic<'a>>,
    ) -> Self {
        Self {
            source,
            program,
            semantic,
        }
    }

    /// ESLint's sourceCode.getText() method
    ///
    /// Returns source text for a node or the entire file
    pub fn get_text(&self, node: Option<&dyn GetSpan>) -> &str {
        match node {
            Some(node) => {
                let span = node.span();
                let start = span.start as usize;
                let end = span.end as usize;
                &self.source[start..end.min(self.source.len())]
            }
            None => self.source,
        }
    }

    /// ESLint's sourceCode.getLines() method
    ///
    /// Returns source code split into lines
    pub fn get_lines(&self) -> Vec<&str> {
        self.source.lines().collect()
    }

    /// ESLint's sourceCode.getAllComments() method
    ///
    /// Returns all comments in the source (placeholder implementation)
    pub fn get_all_comments(&self) -> Vec<ESLintComment> {
        // TODO: Extract comments from OXC AST
        // For now, return empty vector
        Vec::new()
    }

    /// ESLint's sourceCode.getTokens() method
    ///
    /// Returns tokens for a node (placeholder implementation)
    pub fn get_tokens(&self, _node: Option<&dyn Node<'a>>) -> Vec<ESLintToken> {
        // TODO: Extract tokens from OXC AST
        // For now, return empty vector
        Vec::new()
    }

    /// ESLint's sourceCode.getScope() method
    ///
    /// Returns scope information for a node
    pub fn get_scope(&self, _node: &dyn Node<'a>) -> Option<ESLintScope> {
        if let Some(_semantic) = self.semantic {
            // TODO: Map OXC scope information to ESLint format
            Some(ESLintScope::new())
        } else {
            None
        }
    }
}

/// ESLint-compatible scope information
///
/// Represents scope information in ESLint's format
#[derive(Debug, Clone)]
pub struct ESLintScope {
    pub scope_type: String,
    pub variables: Vec<ESLintVariable>,
    pub references: Vec<ESLintReference>,
}

impl ESLintScope {
    pub fn new() -> Self {
        Self {
            scope_type: "global".to_string(),
            variables: Vec::new(),
            references: Vec::new(),
        }
    }
}

impl Default for ESLintScope {
    fn default() -> Self {
        Self::new()
    }
}

/// ESLint variable information
#[derive(Debug, Clone)]
pub struct ESLintVariable {
    pub name: String,
    pub scope: String,
}

/// ESLint reference information
#[derive(Debug, Clone)]
pub struct ESLintReference {
    pub identifier: String,
    pub resolved: bool,
}

/// ESLint comment information
#[derive(Debug, Clone)]
pub struct ESLintComment {
    pub comment_type: String,
    pub value: String,
    pub range: (u32, u32),
}

/// ESLint token information
#[derive(Debug, Clone)]
pub struct ESLintToken {
    pub token_type: String,
    pub value: String,
    pub range: (u32, u32),
}

/// Utility functions for ESLint rule implementations

/// Check if a call expression is a specific function call
pub fn is_function_call(call: &oxc_ast::ast::CallExpression, function_name: &str) -> bool {
    match &call.callee {
        Expression::Identifier(ident) => ident.name.as_str() == function_name,
        Expression::StaticMemberExpression(member) => {
            member.property.name.as_str() == function_name
        }
        _ => false,
    }
}

/// Check if a call expression is a console method call
pub fn is_console_call(call: &oxc_ast::ast::CallExpression) -> bool {
    match &call.callee {
        Expression::StaticMemberExpression(member) => {
            if let Expression::Identifier(obj) = &member.object {
                obj.name.as_str() == "console"
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Check if a call expression is eval or eval-like
pub fn is_eval_call(call: &oxc_ast::ast::CallExpression) -> bool {
    match &call.callee {
        Expression::Identifier(ident) => {
            matches!(ident.name.as_str(), "eval" | "Function" | "execScript")
        }
        _ => false,
    }
}

/// Extract string value from a literal expression
pub fn get_string_value(expr: &Expression) -> Option<&str> {
    match expr {
        Expression::StringLiteral(lit) => Some(lit.value.as_str()),
        _ => None,
    }
}

/// Check if an expression is a specific identifier
pub fn is_identifier(expr: &Expression, name: &str) -> bool {
    match expr {
        Expression::Identifier(ident) => ident.name.as_str() == name,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_parser::{ParseOptions, Parser};
    use oxc_span::SourceType;

    #[test]
    fn test_eslint_context_creation() {
        let source = "console.log('test');";
        let allocator = Allocator::default();
        let source_type = SourceType::default();
        let parser_result = Parser::new(&allocator, source, source_type)
            .with_options(ParseOptions::default())
            .parse();

        let context = ESLintRuleContext::new(
            source,
            "test.js",
            &parser_result.program,
            None,
            "no-console".to_string(),
            DiagnosticSeverity::Warning,
            vec![],
        );

        assert_eq!(context.rule_id, "no-console");
        assert_eq!(context.severity, DiagnosticSeverity::Warning);
    }

    #[test]
    fn test_eslint_report() {
        let source = "console.log('test');";
        let allocator = Allocator::default();
        let source_type = SourceType::default();
        let parser_result = Parser::new(&allocator, source, source_type)
            .with_options(ParseOptions::default())
            .parse();

        let mut context = ESLintRuleContext::new(
            source,
            "test.js",
            &parser_result.program,
            None,
            "no-console".to_string(),
            DiagnosticSeverity::Warning,
            vec![],
        );

        let report = ESLintReport::new(
            oxc_span::Span::new(0, 7),
            "Unexpected console statement",
        );

        context.report(report);

        let diagnostics = context.get_diagnostics();
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, "no-console");
        assert_eq!(diagnostics[0].message, "Unexpected console statement");
    }

    #[test]
    fn test_utility_functions() {
        let allocator = Allocator::default();
        let source_type = SourceType::default();

        // Test console.log detection
        let source = "console.log('test');";
        let parser_result = Parser::new(&allocator, source, source_type)
            .with_options(ParseOptions::default())
            .parse();

        // Find the call expression in the AST
        if let Some(stmt) = parser_result.program.body.first() {
            if let oxc_ast::ast::Statement::ExpressionStatement(expr_stmt) = stmt {
                if let Expression::CallExpression(call) = &expr_stmt.expression {
                    assert!(is_console_call(call));
                }
            }
        }
    }
}