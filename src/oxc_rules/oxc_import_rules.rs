//! Import and export declaration rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoUnusedImports;

impl NoUnusedImports {
    pub const NAME: &'static str = "no-unused-imports";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoUnusedImports {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ImportDeclaration(import) = node.kind() {
            if let Some(specifiers) = &import.specifiers {
                for specifier in specifiers {
                    match specifier {
                        oxc_ast::ast::ImportDeclarationSpecifier::ImportSpecifier(spec) => {
                            if !self.is_imported_name_used(&spec.local.name, ctx) {
                                ctx.diagnostic(unused_import_diagnostic(&spec.local.name, spec.span));
                            }
                        },
                        oxc_ast::ast::ImportDeclarationSpecifier::ImportDefaultSpecifier(spec) => {
                            if !self.is_imported_name_used(&spec.local.name, ctx) {
                                ctx.diagnostic(unused_import_diagnostic(&spec.local.name, spec.span));
                            }
                        },
                        oxc_ast::ast::ImportDeclarationSpecifier::ImportNamespaceSpecifier(spec) => {
                            if !self.is_imported_name_used(&spec.local.name, ctx) {
                                ctx.diagnostic(unused_import_diagnostic(&spec.local.name, spec.span));
                            }
                        },
                    }
                }
            }
        }
    }
}

impl NoUnusedImports {
    fn is_imported_name_used(&self, name: &str, ctx: &WasmLintContext) -> bool {
        // Check if imported name is referenced anywhere in the code
        ctx.semantic.symbols().iter().any(|(_, symbol)| {
            ctx.semantic.symbol_name(symbol).as_str() == name &&
            symbol.flags().contains(oxc_semantic::SymbolFlags::Variable)
        })
    }
}

fn unused_import_diagnostic(name: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Unused import")
        .with_help(format!("'{}' is imported but never used", name))
        .with_label(span)
}

impl EnhancedWasmRule for NoUnusedImports {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Remove unused imports to reduce bundle size".to_string(),
            "Clean imports improve code readability".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoDuplicateImports;

impl NoDuplicateImports {
    pub const NAME: &'static str = "no-duplicate-imports";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoDuplicateImports {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ImportDeclaration(import) = node.kind() {
            let module_name = &import.source.value;
            if self.is_duplicate_import(module_name, ctx) {
                ctx.diagnostic(duplicate_import_diagnostic(module_name, import.span));
            }
        }
    }
}

impl NoDuplicateImports {
    fn is_duplicate_import(&self, _module_name: &str, _ctx: &WasmLintContext) -> bool {
        // Check if this module is imported multiple times
        // Simplified implementation - would need to track all imports
        false
    }
}

fn duplicate_import_diagnostic(module: &str, span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Duplicate import")
        .with_help(format!("Module '{}' is imported multiple times", module))
        .with_label(span)
}

impl EnhancedWasmRule for NoDuplicateImports {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Combine multiple imports from the same module".to_string(),
            "Use a single import statement per module".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferDefaultExport;

impl PreferDefaultExport {
    pub const NAME: &'static str = "prefer-default-export";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for PreferDefaultExport {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ExportNamedDeclaration(export) = node.kind() {
            if self.has_single_export(ctx) && export.declaration.is_some() {
                ctx.diagnostic(prefer_default_export_diagnostic(export.span));
            }
        }
    }
}

impl PreferDefaultExport {
    fn has_single_export(&self, _ctx: &WasmLintContext) -> bool {
        // Check if module has only one export
        // Simplified implementation - would need to count all exports
        true
    }
}

fn prefer_default_export_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Prefer default export")
        .with_help("Use default export when module exports a single binding")
        .with_label(span)
}

impl EnhancedWasmRule for PreferDefaultExport {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Default exports are simpler for single-purpose modules".to_string(),
            "Named exports are better for multi-purpose modules".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoDefaultExport;

impl NoDefaultExport {
    pub const NAME: &'static str = "no-default-export";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoDefaultExport {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ExportDefaultDeclaration(_export) = node.kind() {
            ctx.diagnostic(no_default_export_diagnostic(_export.span));
        }
    }
}

fn no_default_export_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Default export not allowed")
        .with_help("Use named exports instead of default exports")
        .with_label(span)
}

impl EnhancedWasmRule for NoDefaultExport {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Named exports provide better IDE support".to_string(),
            "Named exports are easier to refactor".to_string(),
            "Named exports prevent naming conflicts".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct OrderImports;

impl OrderImports {
    pub const NAME: &'static str = "sort-imports";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for OrderImports {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ImportDeclaration(import) = node.kind() {
            if !self.is_properly_ordered(import, ctx) {
                ctx.diagnostic(import_order_diagnostic(import.span));
            }
        }
    }
}

impl OrderImports {
    fn is_properly_ordered(&self, _import: &oxc_ast::ast::ImportDeclaration, _ctx: &WasmLintContext) -> bool {
        // Check if imports are in correct order (external -> internal -> relative)
        // Simplified implementation - would need to analyze all imports
        true
    }
}

fn import_order_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Import order violation")
        .with_help("Sort imports: external modules, then internal modules, then relative imports")
        .with_label(span)
}

impl EnhancedWasmRule for OrderImports {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Consistent import order improves readability".to_string(),
            "Group external libraries at the top".to_string(),
            "Separate internal and relative imports".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_unused_imports_rule() {
        assert_eq!(NoUnusedImports::NAME, "no-unused-imports");
        assert_eq!(NoUnusedImports::CATEGORY, WasmRuleCategory::Correctness);
        assert_eq!(NoUnusedImports::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_no_duplicate_imports_rule() {
        assert_eq!(NoDuplicateImports::NAME, "no-duplicate-imports");
        assert_eq!(NoDuplicateImports::CATEGORY, WasmRuleCategory::Style);
    }

    #[test]
    fn test_prefer_default_export_rule() {
        assert_eq!(PreferDefaultExport::NAME, "prefer-default-export");
        assert_eq!(PreferDefaultExport::CATEGORY, WasmRuleCategory::Style);
    }

    #[test]
    fn test_no_default_export_rule() {
        assert_eq!(NoDefaultExport::NAME, "no-default-export");
        assert_eq!(NoDefaultExport::CATEGORY, WasmRuleCategory::Restriction);
    }

    #[test]
    fn test_sort_imports_rule() {
        assert_eq!(OrderImports::NAME, "sort-imports");
        assert_eq!(OrderImports::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(OrderImports::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = NoUnusedImports;
        let diagnostic = unused_import_diagnostic("test", Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("bundle size"));
    }
}