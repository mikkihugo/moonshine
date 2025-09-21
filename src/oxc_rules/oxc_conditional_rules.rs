//! Conditional logic and control flow rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoConstantCondition;

impl NoConstantCondition {
    pub const NAME: &'static str = "no-constant-condition";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoConstantCondition {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        match node.kind() {
            AstKind::IfStatement(if_stmt) => {
                if self.is_constant_expression(&if_stmt.test) {
                    ctx.diagnostic(constant_condition_diagnostic(if_stmt.span));
                }
            }
            AstKind::WhileStatement(while_stmt) => {
                if self.is_constant_expression(&while_stmt.test) {
                    ctx.diagnostic(constant_condition_diagnostic(while_stmt.span));
                }
            }
            AstKind::DoWhileStatement(do_while) => {
                if self.is_constant_expression(&do_while.test) {
                    ctx.diagnostic(constant_condition_diagnostic(do_while.span));
                }
            }
            _ => {}
        }
    }
}

impl NoConstantCondition {
    fn is_constant_expression(&self, expr: &oxc_ast::ast::Expression) -> bool {
        use oxc_ast::ast::Expression;
        match expr {
            Expression::BooleanLiteral(_) => true,
            Expression::NumericLiteral(num) => num.value != 0.0,
            Expression::StringLiteral(str_lit) => !str_lit.value.is_empty(),
            _ => false,
        }
    }
}

fn constant_condition_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Constant condition")
        .with_help("Condition always evaluates to the same value")
        .with_label(span)
}

impl EnhancedWasmRule for NoConstantCondition {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["Consider if this condition should be dynamic".to_string()]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoDuplicateCase;

impl NoDuplicateCase {
    pub const NAME: &'static str = "no-duplicate-case";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoDuplicateCase {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::SwitchStatement(switch) = node.kind() {
            let mut seen_cases = std::collections::HashSet::new();

            for case in &switch.cases {
                if let Some(test) = &case.test {
                    let case_value = self.get_case_value(test);
                    if seen_cases.contains(&case_value) {
                        ctx.diagnostic(duplicate_case_diagnostic(case.span));
                    } else {
                        seen_cases.insert(case_value);
                    }
                }
            }
        }
    }
}

impl NoDuplicateCase {
    fn get_case_value(&self, expr: &oxc_ast::ast::Expression) -> String {
        use oxc_ast::ast::Expression;
        match expr {
            Expression::StringLiteral(lit) => lit.value.to_string(),
            Expression::NumericLiteral(lit) => lit.value.to_string(),
            Expression::BooleanLiteral(lit) => lit.value.to_string(),
            _ => format!("{:?}", expr),
        }
    }
}

fn duplicate_case_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Duplicate case in switch statement")
        .with_help("Each case should be unique")
        .with_label(span)
}

impl EnhancedWasmRule for NoDuplicateCase {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["Combine duplicate cases or check logic".to_string()]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoFallthroughCases;

impl NoFallthroughCases {
    pub const NAME: &'static str = "no-fallthrough";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Suspicious;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoFallthroughCases {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::SwitchStatement(switch) = node.kind() {
            for (i, case) in switch.cases.iter().enumerate() {
                if i < switch.cases.len() - 1 && !case.consequent.is_empty() {
                    if !self.has_break_or_return(&case.consequent) {
                        ctx.diagnostic(fallthrough_diagnostic(case.span));
                    }
                }
            }
        }
    }
}

impl NoFallthroughCases {
    fn has_break_or_return(&self, statements: &[oxc_ast::ast::Statement]) -> bool {
        statements.iter().any(|stmt| {
            matches!(stmt,
                oxc_ast::ast::Statement::BreakStatement(_) |
                oxc_ast::ast::Statement::ReturnStatement(_) |
                oxc_ast::ast::Statement::ThrowStatement(_)
            )
        })
    }
}

fn fallthrough_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Case falls through")
        .with_help("Add break, return, or throw statement")
        .with_label(span)
}

impl EnhancedWasmRule for NoFallthroughCases {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["Add explicit break or use fall-through comment".to_string()]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoUselessElse;

impl NoUselessElse {
    pub const NAME: &'static str = "no-useless-else";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoUselessElse {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::IfStatement(if_stmt) = node.kind() {
            if let Some(alternate) = &if_stmt.alternate {
                if self.consequent_always_returns(&if_stmt.consequent) {
                    ctx.diagnostic(useless_else_diagnostic(alternate.span()));
                }
            }
        }
    }
}

impl NoUselessElse {
    fn consequent_always_returns(&self, stmt: &oxc_ast::ast::Statement) -> bool {
        use oxc_ast::ast::Statement;
        match stmt {
            Statement::ReturnStatement(_) => true,
            Statement::ThrowStatement(_) => true,
            Statement::BlockStatement(block) => {
                block.body.iter().any(|s| self.consequent_always_returns(s))
            }
            _ => false,
        }
    }
}

fn useless_else_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Useless else clause")
        .with_help("Remove else clause when if block always returns")
        .with_label(span)
}

impl EnhancedWasmRule for NoUselessElse {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec!["Early returns improve readability".to_string()]
    }
}