//! # OXC Accessibility and Internationalization Rules
//!
//! This module implements WASM-safe OXC rules for web accessibility (a11y),
//! internationalization (i18n), localization (l10n), and inclusive design patterns.
//!
//! ## Rule Categories:
//! - **WCAG Compliance**: Web Content Accessibility Guidelines conformance
//! - **Screen Reader Support**: Semantic HTML and ARIA attributes
//! - **Keyboard Navigation**: Focus management and keyboard accessibility
//! - **Color and Contrast**: Visual accessibility and color blindness support
//! - **Internationalization**: Multi-language and locale support
//! - **Text Direction**: RTL/LTR layout and bidirectional text support
//! - **Cultural Adaptation**: Date, number, and currency formatting
//! - **Content Localization**: Translation keys and pluralization rules
//!
//! Each rule follows the OXC template with WasmRule and EnhancedWasmRule traits.

use crate::unified_rule_registry::{RuleSettings, WasmRuleCategory, WasmFixStatus};
use serde::{Deserialize, Serialize};

/// Trait for basic WASM-compatible rule implementation
pub trait WasmRule {
    const NAME: &'static str;
    const CATEGORY: WasmRuleCategory;
    const FIX_STATUS: WasmFixStatus;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic>;
}

/// Enhanced trait for AI-powered rule suggestions
pub trait EnhancedWasmRule: WasmRule {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmRuleDiagnostic {
    pub rule_name: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub severity: String,
    pub fix_suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSuggestion {
    pub rule_name: String,
    pub suggestion: String,
    pub confidence: f32,
    pub auto_fixable: bool,
}

// ================================================================================================
// WCAG Compliance and Screen Reader Support Rules
// ================================================================================================

/// Requires alt text for images to support screen readers
pub struct RequireImageAltText;

impl RequireImageAltText {
    pub const NAME: &'static str = "require-image-alt-text";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireImageAltText {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("<img") && !code.contains("alt=") {
            diagnostics.push(create_image_alt_text_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireImageAltText {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Add descriptive alt text for images to support screen readers and meet WCAG guidelines. Use empty alt='' for decorative images".to_string(),
            confidence: 0.96,
            auto_fixable: true,
        }).collect()
    }
}

/// Enforces semantic HTML structure for accessibility
pub struct RequireSemanticHtml;

impl RequireSemanticHtml {
    pub const NAME: &'static str = "require-semantic-html";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireSemanticHtml {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("<div") &&
           (code.contains("button") || code.contains("link")) &&
           !code.contains("<button") && !code.contains("<a ") {
            diagnostics.push(create_semantic_html_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireSemanticHtml {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use semantic HTML elements like <button>, <nav>, <main>, <article> instead of generic divs for better accessibility".to_string(),
            confidence: 0.89,
            auto_fixable: true,
        }).collect()
    }
}

/// Requires ARIA labels for interactive elements without text content
pub struct RequireAriaLabels;

impl RequireAriaLabels {
    pub const NAME: &'static str = "require-aria-labels";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireAriaLabels {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("<button") && code.contains("/>") &&
           !code.contains("aria-label") && !code.contains("aria-labelledby") {
            diagnostics.push(create_aria_labels_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireAriaLabels {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Add aria-label or aria-labelledby to interactive elements without visible text for screen reader support".to_string(),
            confidence: 0.94,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// Keyboard Navigation and Focus Management Rules
// ================================================================================================

/// Requires proper focus management for interactive elements
pub struct RequireFocusManagement;

impl RequireFocusManagement {
    pub const NAME: &'static str = "require-focus-management";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireFocusManagement {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("modal") || code.contains("dialog")) &&
           !code.contains("focus()") && !code.contains("tabIndex") {
            diagnostics.push(create_focus_management_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireFocusManagement {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Implement proper focus management for modals and dialogs - focus first element on open, trap focus within, restore focus on close".to_string(),
            confidence: 0.91,
            auto_fixable: false,
        }).collect()
    }
}

/// Prevents keyboard traps in navigation
pub struct NoKeyboardTraps;

impl NoKeyboardTraps {
    pub const NAME: &'static str = "no-keyboard-traps";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoKeyboardTraps {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("tabIndex=\"-1\"") &&
           !code.contains("escape") && !code.contains("Escape") {
            diagnostics.push(create_keyboard_traps_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoKeyboardTraps {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Ensure keyboard users can always escape from focus traps using Esc key or other accessible navigation methods".to_string(),
            confidence: 0.93,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// Color and Visual Accessibility Rules
// ================================================================================================

/// Prevents relying solely on color to convey information
pub struct NoColorOnlyInformation;

impl NoColorOnlyInformation {
    pub const NAME: &'static str = "no-color-only-information";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for NoColorOnlyInformation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("error") || code.contains("success") || code.contains("warning")) &&
           (code.contains("color:") || code.contains("backgroundColor")) &&
           !code.contains("icon") && !code.contains("text") {
            diagnostics.push(create_color_only_information_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoColorOnlyInformation {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Don't rely solely on color to convey information - add icons, text, or patterns for colorblind accessibility".to_string(),
            confidence: 0.88,
            auto_fixable: false,
        }).collect()
    }
}

/// Requires sufficient color contrast for text elements
pub struct RequireColorContrast;

impl RequireColorContrast {
    pub const NAME: &'static str = "require-color-contrast";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireColorContrast {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("color:") && code.contains("#") &&
           (code.contains("#fff") || code.contains("#ccc")) &&
           !code.contains("contrast") {
            diagnostics.push(create_color_contrast_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireColorContrast {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Ensure color combinations meet WCAG contrast requirements - minimum 4.5:1 for normal text, 3:1 for large text".to_string(),
            confidence: 0.85,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// Internationalization and Localization Rules
// ================================================================================================

/// Requires proper language declaration for HTML documents
pub struct RequireLanguageDeclaration;

impl RequireLanguageDeclaration {
    pub const NAME: &'static str = "require-language-declaration";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireLanguageDeclaration {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("<html") && !code.contains("lang=") {
            diagnostics.push(create_language_declaration_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireLanguageDeclaration {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Add lang attribute to html element (e.g., lang='en') to help screen readers pronounce content correctly".to_string(),
            confidence: 0.97,
            auto_fixable: true,
        }).collect()
    }
}

/// Enforces use of translation keys instead of hardcoded strings
pub struct RequireTranslationKeys;

impl RequireTranslationKeys {
    pub const NAME: &'static str = "require-translation-keys";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireTranslationKeys {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("\"Hello") || code.contains("'Hello") &&
           !code.contains("t(") && !code.contains("translate") && !code.contains("i18n") {
            diagnostics.push(create_translation_keys_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireTranslationKeys {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Replace hardcoded strings with translation keys using i18n libraries (e.g., t('greeting.hello')) for internationalization support".to_string(),
            confidence: 0.86,
            auto_fixable: true,
        }).collect()
    }
}

/// Requires proper locale-specific formatting for dates and numbers
pub struct RequireLocaleFormatting;

impl RequireLocaleFormatting {
    pub const NAME: &'static str = "require-locale-formatting";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireLocaleFormatting {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("toDateString") || code.contains("toFixed")) &&
           !code.contains("toLocaleString") && !code.contains("Intl.") {
            diagnostics.push(create_locale_formatting_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireLocaleFormatting {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use Intl.DateTimeFormat, Intl.NumberFormat, or toLocaleString() for locale-appropriate date and number formatting".to_string(),
            confidence: 0.90,
            auto_fixable: true,
        }).collect()
    }
}

/// Requires RTL (Right-to-Left) layout support for international applications
pub struct RequireRtlSupport;

impl RequireRtlSupport {
    pub const NAME: &'static str = "require-rtl-support";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireRtlSupport {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("margin-left") || code.contains("padding-left") &&
           !code.contains("logical") && !code.contains("inline-start") {
            diagnostics.push(create_rtl_support_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireRtlSupport {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use logical CSS properties (margin-inline-start, padding-inline-end) instead of directional properties for RTL language support".to_string(),
            confidence: 0.87,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// Diagnostic Creation Functions
// ================================================================================================

fn create_image_alt_text_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireImageAltText::NAME.to_string(),
        message: "Images must have alt attributes for screen reader accessibility".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add descriptive alt text or alt='' for decorative images".to_string()),
    }
}

fn create_semantic_html_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireSemanticHtml::NAME.to_string(),
        message: "Use semantic HTML elements instead of generic divs for better accessibility".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Replace with appropriate semantic element (button, nav, main, etc.)".to_string()),
    }
}

fn create_aria_labels_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireAriaLabels::NAME.to_string(),
        message: "Interactive elements without text content need ARIA labels".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add aria-label or aria-labelledby attribute".to_string()),
    }
}

fn create_focus_management_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireFocusManagement::NAME.to_string(),
        message: "Modal dialogs require proper focus management for keyboard accessibility".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Implement focus trap and restore focus on close".to_string()),
    }
}

fn create_keyboard_traps_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoKeyboardTraps::NAME.to_string(),
        message: "Ensure keyboard users can escape from focus traps".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add Escape key handler to exit focus trap".to_string()),
    }
}

fn create_color_only_information_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoColorOnlyInformation::NAME.to_string(),
        message: "Don't rely solely on color to convey information".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add icons, text, or patterns alongside color".to_string()),
    }
}

fn create_color_contrast_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireColorContrast::NAME.to_string(),
        message: "Ensure sufficient color contrast for text readability".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Check contrast ratio meets WCAG requirements".to_string()),
    }
}

fn create_language_declaration_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireLanguageDeclaration::NAME.to_string(),
        message: "HTML documents must declare their language".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add lang attribute to html element".to_string()),
    }
}

fn create_translation_keys_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireTranslationKeys::NAME.to_string(),
        message: "Use translation keys instead of hardcoded strings for internationalization".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Replace with translation key using i18n library".to_string()),
    }
}

fn create_locale_formatting_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireLocaleFormatting::NAME.to_string(),
        message: "Use locale-aware formatting for dates and numbers".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Use Intl API or toLocaleString() for proper formatting".to_string()),
    }
}

fn create_rtl_support_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireRtlSupport::NAME.to_string(),
        message: "Use logical CSS properties for RTL language support".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Replace with logical properties (margin-inline-start, etc.)".to_string()),
    }
}

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_alt_text_detection() {
        let code = r#"<img src="photo.jpg" />"#;
        let rule = RequireImageAltText;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireImageAltText::NAME);
    }

    #[test]
    fn test_semantic_html_detection() {
        let code = r#"<div role="button" onclick="handleClick()">Click me</div>"#;
        let rule = RequireSemanticHtml;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireSemanticHtml::NAME);
    }

    #[test]
    fn test_aria_labels_detection() {
        let code = r#"<button><i class="icon-close"></i></button>"#;
        let rule = RequireAriaLabels;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireAriaLabels::NAME);
    }

    #[test]
    fn test_focus_management_detection() {
        let code = r#"const modal = { show: () => setVisible(true) };"#;
        let rule = RequireFocusManagement;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireFocusManagement::NAME);
    }

    #[test]
    fn test_keyboard_traps_detection() {
        let code = r#"<div tabIndex="-1">Focusable content</div>"#;
        let rule = NoKeyboardTraps;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoKeyboardTraps::NAME);
    }

    #[test]
    fn test_color_only_information_detection() {
        let code = r#"<div style="color: red;">Error message</div>"#;
        let rule = NoColorOnlyInformation;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoColorOnlyInformation::NAME);
    }

    #[test]
    fn test_language_declaration_detection() {
        let code = r#"<html><head><title>Page</title></head></html>"#;
        let rule = RequireLanguageDeclaration;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireLanguageDeclaration::NAME);
    }

    #[test]
    fn test_translation_keys_detection() {
        let code = r#"const message = "Hello, World!";"#;
        let rule = RequireTranslationKeys;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireTranslationKeys::NAME);
    }

    #[test]
    fn test_locale_formatting_detection() {
        let code = r#"const formatted = date.toDateString();"#;
        let rule = RequireLocaleFormatting;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireLocaleFormatting::NAME);
    }

    #[test]
    fn test_rtl_support_detection() {
        let code = r#".container { margin-left: 20px; }"#;
        let rule = RequireRtlSupport;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireRtlSupport::NAME);
    }

    #[test]
    fn test_ai_enhancement_suggestions() {
        let code = r#"<img src="logo.png" />"#;
        let rule = RequireImageAltText;
        let diagnostics = rule.run(code);
        let suggestions = rule.ai_enhance(code, &diagnostics);

        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].confidence > 0.95);
        assert!(suggestions[0].auto_fixable);
    }
}