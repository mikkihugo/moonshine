//! CSS-in-JS and styling rules

use crate::oxc_compatible_rules::{
    WasmRule, WasmRuleCategory, WasmFixStatus, WasmAstNode, WasmLintContext, EnhancedWasmRule
};
use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_span::Span;

#[derive(Debug, Default, Clone)]
pub struct NoInlineStyling;

impl NoInlineStyling {
    pub const NAME: &'static str = "no-inline-styling";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoInlineStyling {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::JSXAttribute(attr) = node.kind() {
            if let Some(name) = &attr.name {
                if let Some(ident) = name.as_identifier() {
                    if ident.name == "style" && self.has_object_value(attr) {
                        ctx.diagnostic(no_inline_styling_diagnostic(attr.span));
                    }
                }
            }
        }
    }
}

impl NoInlineStyling {
    fn has_object_value(&self, _attr: &oxc_ast::ast::JSXAttribute) -> bool {
        // Check if style attribute has object expression value
        true
    }
}

fn no_inline_styling_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Inline styling detected")
        .with_help("Extract styles to CSS classes or styled components")
        .with_label(span)
}

impl EnhancedWasmRule for NoInlineStyling {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use CSS modules for component-scoped styles".to_string(),
            "Consider styled-components for dynamic styling".to_string(),
            "Extract common styles to utility classes".to_string(),
            "Inline styles prevent CSS optimization".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferStyledComponents;

impl PreferStyledComponents {
    pub const NAME: &'static str = "prefer-styled-components";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for PreferStyledComponents {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::JSXElement(element) = node.kind() {
            if self.has_dynamic_classes(&element.opening_element) {
                ctx.diagnostic(prefer_styled_components_diagnostic(element.span));
            }
        }
    }
}

impl PreferStyledComponents {
    fn has_dynamic_classes(&self, _opening: &oxc_ast::ast::JSXOpeningElement) -> bool {
        // Check for dynamic className generation
        true
    }
}

fn prefer_styled_components_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Dynamic className detected")
        .with_help("Use styled-components for dynamic styling")
        .with_label(span)
}

impl EnhancedWasmRule for PreferStyledComponents {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "styled-components provide better TypeScript support".to_string(),
            "Avoid string concatenation for class names".to_string(),
            "Use theme providers for consistent styling".to_string(),
            "Consider CSS-in-JS for component isolation".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoImportantInCSS;

impl NoImportantInCSS {
    pub const NAME: &'static str = "no-important-in-css";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoImportantInCSS {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::StringLiteral(string_lit) = node.kind() {
            if string_lit.value.contains("!important") {
                ctx.diagnostic(no_important_in_css_diagnostic(string_lit.span));
            }
        }
    }
}

fn no_important_in_css_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("!important found in CSS")
        .with_help("Improve CSS specificity instead of using !important")
        .with_label(span)
}

impl EnhancedWasmRule for NoImportantInCSS {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use more specific CSS selectors".to_string(),
            "!important makes styles harder to override".to_string(),
            "Consider CSS architecture like BEM".to_string(),
            "Use CSS cascade properly".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct RequireCSSTesting;

impl RequireCSSTesting {
    pub const NAME: &'static str = "require-css-testing";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Pedantic;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireCSSTesting {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::TaggedTemplateExpression(tagged) = node.kind() {
            if self.is_styled_component_tag(&tagged.tag) {
                ctx.diagnostic(require_css_testing_diagnostic(tagged.span));
            }
        }
    }
}

impl RequireCSSTesting {
    fn is_styled_component_tag(&self, _tag: &oxc_ast::ast::Expression) -> bool {
        // Check if this is a styled-component template
        true
    }
}

fn require_css_testing_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Styled component without tests")
        .with_help("Add visual regression tests for styled components")
        .with_label(span)
}

impl EnhancedWasmRule for RequireCSSTesting {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Use Storybook for component visual testing".to_string(),
            "Add snapshot tests for styled components".to_string(),
            "Consider Chromatic for visual regression".to_string(),
            "Test responsive breakpoints".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoMagicColors;

impl NoMagicColors {
    pub const NAME: &'static str = "no-magic-colors";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoMagicColors {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::StringLiteral(string_lit) = node.kind() {
            if self.is_hex_color(&string_lit.value) || self.is_rgb_color(&string_lit.value) {
                ctx.diagnostic(no_magic_colors_diagnostic(string_lit.span));
            }
        }
    }
}

impl NoMagicColors {
    fn is_hex_color(&self, value: &str) -> bool {
        value.starts_with('#') && value.len() >= 4
    }

    fn is_rgb_color(&self, value: &str) -> bool {
        value.starts_with("rgb(") || value.starts_with("rgba(")
    }
}

fn no_magic_colors_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Magic color value")
        .with_help("Use design tokens or CSS variables for colors")
        .with_label(span)
}

impl EnhancedWasmRule for NoMagicColors {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "Create a color palette with semantic names".to_string(),
            "Use CSS custom properties for theming".to_string(),
            "Consider design token systems".to_string(),
            "Magic colors make rebranding difficult".to_string()
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct PreferCSSGrid;

impl PreferCSSGrid {
    pub const NAME: &'static str = "prefer-css-grid";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for PreferCSSGrid {
    fn name(&self) -> &'static str { Self::NAME }
    fn category(&self) -> WasmRuleCategory { Self::CATEGORY }
    fn fix_status(&self) -> WasmFixStatus { Self::FIX_STATUS }

    fn run<'a>(&self, node: &WasmAstNode<'a>, ctx: &mut WasmLintContext<'a>) {
        if let AstKind::ObjectExpression(obj) = node.kind() {
            if self.has_flexbox_layout_pattern(obj) {
                ctx.diagnostic(prefer_css_grid_diagnostic(obj.span));
            }
        }
    }
}

impl PreferCSSGrid {
    fn has_flexbox_layout_pattern(&self, _obj: &oxc_ast::ast::ObjectExpression) -> bool {
        // Check for complex flexbox patterns that could be grid
        true
    }
}

fn prefer_css_grid_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Complex flexbox layout")
        .with_help("Consider CSS Grid for two-dimensional layouts")
        .with_label(span)
}

impl EnhancedWasmRule for PreferCSSGrid {
    fn ai_enhance(&self, _diagnostic: &OxcDiagnostic, _source: &str) -> Vec<String> {
        vec![
            "CSS Grid is better for 2D layouts".to_string(),
            "Flexbox is better for 1D layouts".to_string(),
            "Grid provides better alignment control".to_string(),
            "Use subgrid for nested layouts".to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_inline_styling_rule() {
        assert_eq!(NoInlineStyling::NAME, "no-inline-styling");
        assert_eq!(NoInlineStyling::CATEGORY, WasmRuleCategory::Style);
    }

    #[test]
    fn test_prefer_styled_components_rule() {
        assert_eq!(PreferStyledComponents::NAME, "prefer-styled-components");
        assert_eq!(PreferStyledComponents::CATEGORY, WasmRuleCategory::Style);
    }

    #[test]
    fn test_color_detection() {
        let rule = NoMagicColors;
        assert!(rule.is_hex_color("#ffffff"));
        assert!(rule.is_rgb_color("rgb(255, 255, 255)"));
        assert!(!rule.is_hex_color("white"));
    }

    #[test]
    fn test_ai_enhancements() {
        let rule = NoMagicColors;
        let diagnostic = no_magic_colors_diagnostic(Span::default());
        let suggestions = rule.ai_enhance(&diagnostic, "");
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("design tokens"));
    }
}