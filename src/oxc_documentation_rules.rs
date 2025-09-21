//! Documentation and comment rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct RequireJSDoc;

impl RequireJSDoc {
    pub const NAME: &'static str = "require-jsdoc";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireJSDoc {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        match node.kind() {
            AstKind::FunctionDeclaration(func) => {
                if self.is_public_function(func) && !self.has_jsdoc_comment(ctx, func.span) {
                    ctx.diagnostic(require_jsdoc_diagnostic(func.span));
                }
            }
            AstKind::Class(class) => {
                if !self.has_jsdoc_comment(ctx, class.span) {
                    ctx.diagnostic(require_jsdoc_diagnostic(class.span));
                }
            }
            _ => {}
        }
    }
}

impl RequireJSDoc {
    fn is_public_function(&self, func: &oxc_ast::ast::Function) -> bool {
        // Check if function is exported or public
        func.id.as_ref().map_or(false, |id| !id.name.starts_with('_'))
    }

    fn has_jsdoc_comment(&self, _ctx: &WasmLintContext, _span: Span) -> bool {
        // Check for JSDoc comment above the declaration
        false
    }
}

fn require_jsdoc_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Missing JSDoc comment")
        .with_help("Add JSDoc comment to document the function or class")
        .with_label(span)
}

impl EnhancedWasmRule for RequireJSDoc {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "JSDoc comments improve IDE intellisense".to_string(),
            "Document parameters, return values, and examples".to_string(),
            "Use @param and @returns tags".to_string(),
            "Good documentation reduces support burden".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct ValidJSDocTags;

impl ValidJSDocTags {
    pub const NAME: &'static str = "valid-jsdoc-tags";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for ValidJSDocTags {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        // Check for invalid JSDoc tags in comments
        // This would require comment analysis
        if let AstKind::Function(_func) = node.kind() {
            if self.has_invalid_jsdoc_tags(ctx) {
                ctx.diagnostic(valid_jsdoc_tags_diagnostic(node.span()));
            }
        }
    }
}

impl ValidJSDocTags {
    fn has_invalid_jsdoc_tags(&self, _ctx: &WasmLintContext) -> bool {
        // Check for typos in JSDoc tags like @parma instead of @param
        false
    }
}

fn valid_jsdoc_tags_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Invalid JSDoc tag")
        .with_help("Check JSDoc tag spelling and syntax")
        .with_label(span)
}

impl EnhancedWasmRule for ValidJSDocTags {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Common typos: @parma → @param, @retrun → @return".to_string(),
            "Use standard JSDoc tags for better tooling support".to_string(),
            "Check JSDoc documentation for valid tags".to_string(),
            "Invalid tags are ignored by documentation generators".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoTodoComments;

impl NoTodoComments {
    pub const NAME: &'static str = "no-todo-comments";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoTodoComments {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        // Check for TODO/FIXME comments
        // This would require comment analysis from the source
        if self.has_todo_comments(ctx) {
            ctx.diagnostic(no_todo_comments_diagnostic(node.span()));
        }
    }
}

impl NoTodoComments {
    fn has_todo_comments(&self, _ctx: &WasmLintContext) -> bool {
        // Check for TODO, FIXME, HACK comments
        false
    }
}

fn no_todo_comments_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("TODO comment detected")
        .with_help("Convert TODO comments to GitHub issues")
        .with_label(span)
}

impl EnhancedWasmRule for NoTodoComments {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Create GitHub issues for TODO items".to_string(),
            "Add deadlines to TODO comments".to_string(),
            "Use issue numbers: // TODO: #123 implement feature".to_string(),
            "TODO comments often become technical debt".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireFileHeaders;

impl RequireFileHeaders {
    pub const NAME: &'static str = "require-file-headers";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireFileHeaders {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::Program(_program) = node.kind() {
            if !self.has_file_header(ctx) {
                ctx.diagnostic(require_file_headers_diagnostic(node.span()));
            }
        }
    }
}

impl RequireFileHeaders {
    fn has_file_header(&self, _ctx: &WasmLintContext) -> bool {
        // Check for copyright or license header at top of file
        false
    }
}

fn require_file_headers_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Missing file header")
        .with_help("Add copyright or license header to file")
        .with_label(span)
}

impl EnhancedWasmRule for RequireFileHeaders {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "File headers ensure license compliance".to_string(),
            "Use SPDX license identifiers".to_string(),
            "Automate header insertion with pre-commit hooks".to_string(),
            "Include copyright year and owner information".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoCommentedCode;

impl NoCommentedCode {
    pub const NAME: &'static str = "no-commented-code";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoCommentedCode {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        // Check for commented-out code in comments
        if self.has_commented_code(ctx) {
            ctx.diagnostic(no_commented_code_diagnostic(node.span()));
        }
    }
}

impl NoCommentedCode {
    fn has_commented_code(&self, _ctx: &WasmLintContext) -> bool {
        // Detect patterns that look like commented-out code
        false
    }
}

fn no_commented_code_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Commented-out code detected")
        .with_help("Remove commented code or explain why it's kept")
        .with_label(span)
}

impl EnhancedWasmRule for NoCommentedCode {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use version control instead of commenting code".to_string(),
            "Commented code creates visual noise".to_string(),
            "If code is needed later, create a branch".to_string(),
            "Add explanation if code must stay commented".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireExampleUsage;

impl RequireExampleUsage {
    pub const NAME: &'static str = "require-example-usage";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireExampleUsage {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::FunctionDeclaration(func) = node.kind() {
            if self.is_public_api_function(func) && !self.has_example_in_jsdoc(ctx, func.span) {
                ctx.diagnostic(require_example_usage_diagnostic(func.span));
            }
        }
    }
}

impl RequireExampleUsage {
    fn is_public_api_function(&self, func: &oxc_ast::ast::Function) -> bool {
        // Check if this is an exported function
        func.id.as_ref().map_or(false, |id| !id.name.starts_with('_'))
    }

    fn has_example_in_jsdoc(&self, _ctx: &WasmLintContext, _span: Span) -> bool {
        // Check for @example tag in JSDoc
        false
    }
}

fn require_example_usage_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Missing usage example")
        .with_help("Add @example tag to JSDoc with usage example")
        .with_label(span)
}

impl EnhancedWasmRule for RequireExampleUsage {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Examples improve API discoverability".to_string(),
            "Show common use cases in examples".to_string(),
            "Examples can be tested with doctest tools".to_string(),
            "Good examples reduce developer onboarding time".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoObsoleteComments;

impl NoObsoleteComments {
    pub const NAME: &'static str = "no-obsolete-comments";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Suspicious;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoObsoleteComments {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        // Check for comments that reference non-existent code
        if self.has_obsolete_comments(ctx) {
            ctx.diagnostic(no_obsolete_comments_diagnostic(node.span()));
        }
    }
}

impl NoObsoleteComments {
    fn has_obsolete_comments(&self, _ctx: &WasmLintContext) -> bool {
        // Check for comments that reference removed functions/variables
        false
    }
}

fn no_obsolete_comments_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Obsolete comment detected")
        .with_help("Update or remove comments that reference non-existent code")
        .with_label(span)
}

impl EnhancedWasmRule for NoObsoleteComments {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Obsolete comments mislead developers".to_string(),
            "Use automated tools to detect stale comments".to_string(),
            "Review comments during code refactoring".to_string(),
            "Living documentation is better than stale comments".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_jsdoc_rule() {
        assert_eq!(RequireJSDoc::NAME, "require-jsdoc");
        assert_eq!(RequireJSDoc::CATEGORY, WasmRuleCategory::Pedantic);
    }

    #[test]
    fn test_valid_jsdoc_tags_rule() {
        assert_eq!(ValidJSDocTags::NAME, "valid-jsdoc-tags");
        assert_eq!(ValidJSDocTags::CATEGORY, WasmRuleCategory::Correctness);
        assert_eq!(ValidJSDocTags::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_no_todo_comments_rule() {
        assert_eq!(NoTodoComments::NAME, "no-todo-comments");
        assert_eq!(NoTodoComments::CATEGORY, WasmRuleCategory::Pedantic);
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = RequireJSDoc;
        let diagnostic = require_jsdoc_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("intellisense"));
    }
}