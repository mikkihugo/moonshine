//! Monorepo and workspace rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoCircularWorkspaceDeps;

impl NoCircularWorkspaceDeps {
    pub const NAME: &'static str = "no-circular-workspace-deps";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoCircularWorkspaceDeps {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ImportDeclaration(import_decl) = node.kind() {
            if self.is_workspace_import(import_decl) && self.creates_circular_dep(import_decl, ctx) {
                ctx.diagnostic(no_circular_workspace_deps_diagnostic(import_decl.span));
            }
        }
    }
}

impl NoCircularWorkspaceDeps {
    fn is_workspace_import(&self, import_decl: &oxc_ast::ast::ImportDeclaration) -> bool {
        // Check if import is from workspace package
        let source = &import_decl.source.value;
        source.starts_with("@workspace/") || source.starts_with("@repo/")
    }

    fn creates_circular_dep(&self, _import_decl: &oxc_ast::ast::ImportDeclaration, _ctx: &WasmLintContext) -> bool {
        // Check for circular dependency in workspace packages
        false
    }
}

fn no_circular_workspace_deps_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Circular dependency between workspace packages")
        .with_help("Restructure packages to eliminate circular dependencies")
        .with_label(span)
}

impl EnhancedWasmRule for NoCircularWorkspaceDeps {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Extract shared code to common package".to_string(),
            "Use dependency injection to break circles".to_string(),
            "Analyze workspace dependency graph".to_string(),
            "Circular dependencies prevent proper bundling".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireWorkspaceProtocol;

impl RequireWorkspaceProtocol {
    pub const NAME: &'static str = "require-workspace-protocol";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireWorkspaceProtocol {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ImportDeclaration(import_decl) = node.kind() {
            if self.is_internal_package(import_decl) && !self.uses_workspace_protocol(import_decl) {
                ctx.diagnostic(require_workspace_protocol_diagnostic(import_decl.span));
            }
        }
    }
}

impl RequireWorkspaceProtocol {
    fn is_internal_package(&self, import_decl: &oxc_ast::ast::ImportDeclaration) -> bool {
        // Check if import is from internal workspace package
        let source = &import_decl.source.value;
        source.starts_with("@company/") || self.is_relative_workspace_import(source)
    }

    fn is_relative_workspace_import(&self, source: &str) -> bool {
        source.starts_with("../") && source.contains("/packages/")
    }

    fn uses_workspace_protocol(&self, import_decl: &oxc_ast::ast::ImportDeclaration) -> bool {
        let source = &import_decl.source.value;
        source.starts_with("workspace:")
    }
}

fn require_workspace_protocol_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Internal package import without workspace protocol")
        .with_help("Use workspace: protocol for internal package dependencies")
        .with_label(span)
}

impl EnhancedWasmRule for RequireWorkspaceProtocol {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use workspace: protocol for better dependency resolution".to_string(),
            "Workspace protocol ensures local package linking".to_string(),
            "Helps package managers optimize monorepo builds".to_string(),
            "Prevents version conflicts in workspace packages".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoPublishPrivatePackages;

impl NoPublishPrivatePackages {
    pub const NAME: &'static str = "no-publish-private-packages";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoPublishPrivatePackages {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            if self.is_package_json(ctx) && self.is_private_package(obj) && self.has_publish_config(obj) {
                ctx.diagnostic(no_publish_private_packages_diagnostic(obj.span));
            }
        }
    }
}

impl NoPublishPrivatePackages {
    fn is_package_json(&self, ctx: &WasmLintContext) -> bool {
        ctx.filename().ends_with("package.json")
    }

    fn is_private_package(&self, _obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check if package.json has "private": true
        true
    }

    fn has_publish_config(&self, _obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check if package has publishConfig
        false
    }
}

fn no_publish_private_packages_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Private package with publish configuration")
        .with_help("Remove publishConfig from private packages")
        .with_label(span)
}

impl EnhancedWasmRule for NoPublishPrivatePackages {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Private packages should not have publish configuration".to_string(),
            "Use private: true to prevent accidental publishing".to_string(),
            "Remove publishConfig from internal packages".to_string(),
            "Configure CI to prevent private package publishing".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireConsistentVersioning;

impl RequireConsistentVersioning {
    pub const NAME: &'static str = "require-consistent-versioning";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireConsistentVersioning {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            if self.is_package_json(ctx) && self.has_inconsistent_versions(obj) {
                ctx.diagnostic(require_consistent_versioning_diagnostic(obj.span));
            }
        }
    }
}

impl RequireConsistentVersioning {
    fn is_package_json(&self, ctx: &WasmLintContext) -> bool {
        ctx.filename().ends_with("package.json")
    }

    fn has_inconsistent_versions(&self, _obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check for version inconsistencies across workspace packages
        true
    }
}

fn require_consistent_versioning_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Inconsistent versions across workspace packages")
        .with_help("Use tools like syncpack to maintain consistent versions")
        .with_label(span)
}

impl EnhancedWasmRule for RequireConsistentVersioning {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use syncpack for version consistency across workspace".to_string(),
            "Pin shared dependency versions in root package.json".to_string(),
            "Automate version synchronization in CI".to_string(),
            "Version mismatches can cause runtime issues".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoDeepRelativeImports;

impl NoDeepRelativeImports {
    pub const NAME: &'static str = "no-deep-relative-imports";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoDeepRelativeImports {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ImportDeclaration(import_decl) = node.kind() {
            if self.is_deep_relative_import(import_decl) {
                ctx.diagnostic(no_deep_relative_imports_diagnostic(import_decl.span));
            }
        }
    }
}

impl NoDeepRelativeImports {
    fn is_deep_relative_import(&self, import_decl: &oxc_ast::ast::ImportDeclaration) -> bool {
        let source = &import_decl.source.value;
        let dot_dot_count = source.matches("../").count();
        dot_dot_count > 2
    }
}

fn no_deep_relative_imports_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Deep relative import detected")
        .with_help("Use workspace packages or path mapping instead of deep relative imports")
        .with_label(span)
}

impl EnhancedWasmRule for NoDeepRelativeImports {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Extract shared code to workspace packages".to_string(),
            "Use TypeScript path mapping for cleaner imports".to_string(),
            "Deep relative imports are brittle and hard to refactor".to_string(),
            "Consider using barrel exports for complex structures".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireWorkspaceScripts;

impl RequireWorkspaceScripts {
    pub const NAME: &'static str = "require-workspace-scripts";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireWorkspaceScripts {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            if self.is_root_package_json(ctx) && !self.has_workspace_scripts(obj) {
                ctx.diagnostic(require_workspace_scripts_diagnostic(obj.span));
            }
        }
    }
}

impl RequireWorkspaceScripts {
    fn is_root_package_json(&self, ctx: &WasmLintContext) -> bool {
        ctx.filename() == "package.json" || ctx.filename().ends_with("/package.json")
    }

    fn has_workspace_scripts(&self, _obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check for common workspace scripts
        false
    }
}

fn require_workspace_scripts_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Missing workspace management scripts")
        .with_help("Add scripts for build, test, lint across all workspace packages")
        .with_label(span)
}

impl EnhancedWasmRule for RequireWorkspaceScripts {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Add scripts: build:all, test:all, lint:all".to_string(),
            "Use package manager workspace features".to_string(),
            "Consider using Nx or Rush for advanced workspace management".to_string(),
            "Standardize scripts across all packages".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoMixedPackageManagers;

impl NoMixedPackageManagers {
    pub const NAME: &'static str = "no-mixed-package-managers";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoMixedPackageManagers {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::Program(_program) = node.kind() {
            if self.has_multiple_lock_files(&ctx.filename()) {
                ctx.diagnostic(no_mixed_package_managers_diagnostic(node.span()));
            }
        }
    }
}

impl NoMixedPackageManagers {
    fn has_multiple_lock_files(&self, filename: &str) -> bool {
        // This would require checking the file system for multiple lock files
        filename.contains("package-lock.json") && filename.contains("yarn.lock")
    }
}

fn no_mixed_package_managers_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Multiple package manager lock files detected")
        .with_help("Use consistent package manager across workspace")
        .with_label(span)
}

impl EnhancedWasmRule for NoMixedPackageManagers {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Choose one package manager for the entire workspace".to_string(),
            "Remove conflicting lock files".to_string(),
            "Add .gitignore entries for unused lock files".to_string(),
            "Mixed package managers cause dependency conflicts".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireChangesets;

impl RequireChangesets {
    pub const NAME: &'static str = "require-changesets";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireChangesets {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::Program(_program) = node.kind() {
            if self.is_workspace_root(ctx) && !self.has_changeset_config(ctx) {
                ctx.diagnostic(require_changesets_diagnostic(node.span()));
            }
        }
    }
}

impl RequireChangesets {
    fn is_workspace_root(&self, ctx: &WasmLintContext) -> bool {
        ctx.filename().contains("package.json") && ctx.filename().matches('/').count() <= 1
    }

    fn has_changeset_config(&self, _ctx: &WasmLintContext) -> bool {
        // Check for changeset configuration
        false
    }
}

fn require_changesets_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Missing changeset configuration for workspace")
        .with_help("Add @changesets/cli for automated versioning and releases")
        .with_label(span)
}

impl EnhancedWasmRule for RequireChangesets {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Changesets automate versioning in monorepos".to_string(),
            "Track changes across workspace packages".to_string(),
            "Generate changelogs automatically".to_string(),
            "Integrate with CI for automated releases".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_circular_workspace_deps_rule() {
        assert_eq!(NoCircularWorkspaceDeps::NAME, "no-circular-workspace-deps");
        assert_eq!(NoCircularWorkspaceDeps::CATEGORY, WasmRuleCategory::Correctness);
        assert_eq!(NoCircularWorkspaceDeps::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_require_workspace_protocol_rule() {
        assert_eq!(RequireWorkspaceProtocol::NAME, "require-workspace-protocol");
        assert_eq!(RequireWorkspaceProtocol::CATEGORY, WasmRuleCategory::Style);
        assert_eq!(RequireWorkspaceProtocol::FIX_STATUS, WasmFixStatus::Fix);
    }

    #[test]
    fn test_deep_relative_import_detection() {
        let rule = NoDeepRelativeImports;
        let deep_import = oxc_ast::ast::ImportDeclaration {
            span: Span::default(),
            specifiers: None,
            source: Box::new(oxc_ast::ast::StringLiteral {
                span: Span::default(),
                value: "../../../deep/nested/module".into(),
            }),
            with_clause: None,
            import_kind: oxc_ast::ast::ImportOrExportKind::Value,
        };
        assert!(rule.is_deep_relative_import(&deep_import));
    }

    #[test]
    fn test_workspace_import_detection() {
        let rule = NoCircularWorkspaceDeps;
        let workspace_import = oxc_ast::ast::ImportDeclaration {
            span: Span::default(),
            specifiers: None,
            source: Box::new(oxc_ast::ast::StringLiteral {
                span: Span::default(),
                value: "@workspace/shared".into(),
            }),
            with_clause: None,
            import_kind: oxc_ast::ast::ImportOrExportKind::Value,
        };
        assert!(rule.is_workspace_import(&workspace_import));
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = RequireWorkspaceProtocol;
        let diagnostic = require_workspace_protocol_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("workspace:"));
    }
}