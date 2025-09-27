//! Configuration types for code analysis and formatting
//!
//! Self-documenting configuration structures for AST analysis behavior.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for AST auto-fix behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstAutoFixConfig {
    pub enable_semantic_analysis: bool,
    pub enable_type_checking: bool,
    pub enable_performance_fixes: bool,
    pub enable_security_fixes: bool,
    pub generate_source_maps: bool,
    pub preserve_comments: bool,
    pub target_typescript_version: String,
    pub min_confidence_threshold: f32,
    pub max_fixes_per_file: usize,
    // OXC-based code formatting configuration (Prettier replacement)
    pub enable_formatting: bool,
    pub format_config: FormattingConfig,
}

/// OXC-based code formatting configuration (replaces Prettier)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattingConfig {
    pub indent_width: u8,
    pub use_tabs: bool,
    pub line_width: u32,
    pub quote_style: QuoteStyle,
    pub trailing_comma: TrailingCommaStyle,
    pub semicolons: SemicolonStyle,
    pub arrow_parens: ArrowParensStyle,
    pub bracket_spacing: bool,
    pub jsx_single_quote: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuoteStyle {
    Single,
    Double,
    Preserve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrailingCommaStyle {
    None,
    ES5,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SemicolonStyle {
    Always,
    Never,
    Preserve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrowParensStyle {
    Always,
    Avoid,
    Preserve,
}

/// ESLint configuration parsed from project files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EslintConfig {
    pub extends: Vec<String>,
    pub rules: HashMap<String, EslintRuleConfig>,
    pub parser_options: EslintParserOptions,
    pub env: HashMap<String, bool>,
    pub globals: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EslintRuleConfig {
    pub level: EslintRuleLevel,
    pub options: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EslintRuleLevel {
    Off,
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EslintParserOptions {
    pub ecma_version: Option<i32>,
    pub source_type: Option<String>,
    pub ecma_features: HashMap<String, bool>,
}

impl Default for AstAutoFixConfig {
    fn default() -> Self {
        Self {
            enable_semantic_analysis: true,
            enable_type_checking: true,
            enable_performance_fixes: true,
            enable_security_fixes: true,
            generate_source_maps: true,
            preserve_comments: true,
            target_typescript_version: "5.0".to_string(),
            min_confidence_threshold: 0.8,
            max_fixes_per_file: 50,
            enable_formatting: true,
            format_config: FormattingConfig::default(),
        }
    }
}

impl Default for FormattingConfig {
    fn default() -> Self {
        Self {
            indent_width: 2,
            use_tabs: false,
            line_width: 80,
            quote_style: QuoteStyle::Double,
            trailing_comma: TrailingCommaStyle::ES5,
            semicolons: SemicolonStyle::Always,
            arrow_parens: ArrowParensStyle::Avoid,
            bracket_spacing: true,
            jsx_single_quote: false,
        }
    }
}
