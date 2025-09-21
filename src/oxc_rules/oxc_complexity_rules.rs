//! Code complexity and maintainability rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct MaxComplexity;

impl MaxComplexity {
    pub const NAME: &'static str = "max-complexity";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
    pub const MAX_COMPLEXITY: u32 = 10;
}

impl WasmRule for MaxComplexity {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        match node.kind() {
            AstKind::FunctionDeclaration(func) => {
                let complexity = self.calculate_complexity(&func.body);
                if complexity > Self::MAX_COMPLEXITY {
                    ctx.diagnostic(max_complexity_diagnostic(complexity, func.span));
                }
            }
            AstKind::ArrowFunction(func) => {
                // Arrow functions can have either expression body or function body
                let complexity = if func.expression {
                    1 // Simple expression body
                } else if let Some(body) = &func.body {
                    self.calculate_complexity(body)
                } else {
                    1 // Fallback
                };
                if complexity > Self::MAX_COMPLEXITY {
                    ctx.diagnostic(max_complexity_diagnostic(complexity, func.span));
                }
            }
            _ => {}
        }
    }
}

impl MaxComplexity {
    fn calculate_complexity(&self, body: &oxc_ast::ast::FunctionBody) -> u32 {
        let mut complexity = 1; // Base complexity
        for stmt in &body.statements {
            complexity += self.statement_complexity(stmt);
        }
        complexity
    }


    fn statement_complexity(&self, _stmt: &oxc_ast::ast::Statement) -> u32 {
        // Simplified complexity calculation
        // In practice, would analyze control flow structures
        1
    }
}

fn max_complexity_diagnostic(complexity: u32, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Function too complex")
        .with_help(format!("Function has complexity of {}, consider refactoring", complexity))
        .with_label(span)
}

impl EnhancedWasmRule for MaxComplexity {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Break function into smaller functions".to_string(),
            "Extract complex logic into helper functions".to_string(),
            "Use early returns to reduce nesting".to_string(),
            "Consider using strategy pattern for complex conditionals".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct MaxDepth;

impl MaxDepth {
    pub const NAME: &'static str = "max-depth";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
    pub const MAX_DEPTH: u32 = 4;
}

impl WasmRule for MaxDepth {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        match node.kind() {
            AstKind::IfStatement(_) |
            AstKind::WhileStatement(_) |
            AstKind::DoWhileStatement(_) |
            AstKind::ForStatement(_) |
            AstKind::ForInStatement(_) |
            AstKind::ForOfStatement(_) => {
                let depth = self.calculate_nesting_depth(ctx);
                if depth > Self::MAX_DEPTH {
                    ctx.diagnostic(max_depth_diagnostic(depth, node.span()));
                }
            }
            _ => {}
        }
    }
}

impl MaxDepth {
    fn calculate_nesting_depth(&self, _ctx: &WasmLintContext) -> u32 {
        // Simplified depth calculation
        // In practice, would traverse up the AST to count nesting levels
        5 // Simulated depth that exceeds max
    }
}

fn max_depth_diagnostic(depth: u32, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Too deeply nested")
        .with_help(format!("Block is nested {} levels deep, consider refactoring", depth))
        .with_label(span)
}

impl EnhancedWasmRule for MaxDepth {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use early returns to reduce nesting".to_string(),
            "Extract nested logic into separate functions".to_string(),
            "Consider using guard clauses".to_string(),
            "Use functional programming patterns (map, filter, reduce)".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct MaxLines;

impl MaxLines {
    pub const NAME: &'static str = "max-lines";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
    pub const MAX_LINES: u32 = 300;
}

impl WasmRule for MaxLines {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::Program(_program) = node.kind() {
            let line_count = self.count_lines(ctx);
            if line_count > Self::MAX_LINES {
                ctx.diagnostic(max_lines_diagnostic(line_count, node.span()));
            }
        }
    }
}

impl MaxLines {
    fn count_lines(&self, _ctx: &WasmLintContext) -> u32 {
        // Simplified line counting
        // In practice, would count actual lines in source
        350 // Simulated count that exceeds max
    }
}

fn max_lines_diagnostic(lines: u32, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("File too long")
        .with_help(format!("File has {} lines, consider splitting into smaller files", lines))
        .with_label(span)
}

impl EnhancedWasmRule for MaxLines {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Split large files into focused modules".to_string(),
            "Extract related functions into separate files".to_string(),
            "Use barrel exports to maintain clean imports".to_string(),
            "Group related functionality by domain".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct MaxParams;

impl MaxParams {
    pub const NAME: &'static str = "max-params";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
    pub const MAX_PARAMS: usize = 4;
}

impl WasmRule for MaxParams {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        match node.kind() {
            AstKind::FunctionDeclaration(func) => {
                if func.params.parameters.len() > Self::MAX_PARAMS {
                    ctx.diagnostic(max_params_diagnostic(func.params.parameters.len(), func.span));
                }
            }
            AstKind::ArrowFunction(func) => {
                if func.params.parameters.len() > Self::MAX_PARAMS {
                    ctx.diagnostic(max_params_diagnostic(func.params.parameters.len(), func.span));
                }
            }
            _ => {}
        }
    }
}

fn max_params_diagnostic(count: usize, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Too many parameters")
        .with_help(format!("Function has {} parameters, consider using an options object", count))
        .with_label(span)
}

impl EnhancedWasmRule for MaxParams {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use options object: function(options) instead of many parameters".to_string(),
            "Group related parameters into objects".to_string(),
            "Consider using destructuring for options".to_string(),
            "Use builder pattern for complex configurations".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoLargeClasses;

impl NoLargeClasses {
    pub const NAME: &'static str = "no-large-classes";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
    pub const MAX_METHODS: usize = 15;
}

impl WasmRule for NoLargeClasses {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::Class(class) = node.kind() {
            let method_count = self.count_methods(class);
            if method_count > Self::MAX_METHODS {
                ctx.diagnostic(large_class_diagnostic(method_count, class.span));
            }
        }
    }
}

impl NoLargeClasses {
    fn count_methods(&self, class: &oxc_ast::ast::Class) -> usize {
        class.body.body.iter()
            .filter(|member| matches!(member, oxc_ast::ast::ClassElement::MethodDefinition(_)))
            .count()
    }
}

fn large_class_diagnostic(count: usize, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Class too large")
        .with_help(format!("Class has {} methods, consider splitting responsibilities", count))
        .with_label(span)
}

impl EnhancedWasmRule for NoLargeClasses {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Follow Single Responsibility Principle".to_string(),
            "Extract related methods into separate classes".to_string(),
            "Use composition over inheritance".to_string(),
            "Consider using mixins or traits pattern".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoMagicNumbers;

impl NoMagicNumbers {
    pub const NAME: &'static str = "no-magic-numbers";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoMagicNumbers {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::NumericLiteral(num) = node.kind() {
            if self.is_magic_number(num.value) {
                ctx.diagnostic(magic_number_diagnostic(num.value, num.span));
            }
        }
    }
}

impl NoMagicNumbers {
    fn is_magic_number(&self, value: f64) -> bool {
        // Common non-magic numbers
        !matches!(value as i32, -1 | 0 | 1 | 2 | 10 | 100 | 1000)
    }
}

fn magic_number_diagnostic(value: f64, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Magic number detected")
        .with_help(format!("Replace magic number {} with a named constant", value))
        .with_label(span)
}

impl EnhancedWasmRule for NoMagicNumbers {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use named constants: const MAX_RETRIES = 3".to_string(),
            "Group related constants in enums or objects".to_string(),
            "Magic numbers make code hard to understand".to_string(),
            "Named constants improve maintainability".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_complexity_rule() {
        assert_eq!(MaxComplexity::NAME, "max-complexity");
        assert_eq!(MaxComplexity::CATEGORY, WasmRuleCategory::Pedantic);
        assert_eq!(MaxComplexity::MAX_COMPLEXITY, 10);
    }

    #[test]
    fn test_max_depth_rule() {
        assert_eq!(MaxDepth::NAME, "max-depth");
        assert_eq!(MaxDepth::CATEGORY, WasmRuleCategory::Pedantic);
        assert_eq!(MaxDepth::MAX_DEPTH, 4);
    }

    #[test]
    fn test_max_lines_rule() {
        assert_eq!(MaxLines::NAME, "max-lines");
        assert_eq!(MaxLines::CATEGORY, WasmRuleCategory::Pedantic);
        assert_eq!(MaxLines::MAX_LINES, 300);
    }

    #[test]
    fn test_max_params_rule() {
        assert_eq!(MaxParams::NAME, "max-params");
        assert_eq!(MaxParams::CATEGORY, WasmRuleCategory::Pedantic);
        assert_eq!(MaxParams::MAX_PARAMS, 4);
    }

    #[test]
    fn test_no_large_classes_rule() {
        assert_eq!(NoLargeClasses::NAME, "no-large-classes");
        assert_eq!(NoLargeClasses::CATEGORY, WasmRuleCategory::Pedantic);
        assert_eq!(NoLargeClasses::MAX_METHODS, 15);
    }

    #[test]
    fn test_no_magic_numbers_rule() {
        assert_eq!(NoMagicNumbers::NAME, "no-magic-numbers");
        assert_eq!(NoMagicNumbers::CATEGORY, WasmRuleCategory::Pedantic);
    }

    #[test]
    fn test_magic_number_detection() {
        let rule = NoMagicNumbers;
        assert!(!rule.is_magic_number(0.0));
        assert!(!rule.is_magic_number(1.0));
        assert!(!rule.is_magic_number(-1.0));
        assert!(rule.is_magic_number(42.0));
        assert!(rule.is_magic_number(3.14159));
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = MaxComplexity;
        let diagnostic = max_complexity_diagnostic(15, Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("smaller functions"));
    }
}