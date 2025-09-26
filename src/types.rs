//! Core data structures for moon-shine code analysis
//!
//! This module contains all the fundamental types and structures used throughout
//! the moon-shine code analyzer. These are copied verbatim from code_analyzer.rs
//! to preserve all functionality during refactoring.

// Removed OXC diagnostic reporter dependency
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Removed NoopDiagnosticReporter - no longer needed with Biome

/// Comprehensive AST-based auto-fix result with detailed metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstAutoFixResult {
    pub file_path: String,
    pub success: bool,
    pub original_size: usize,
    pub fixed_size: usize,
    pub fixes_applied: Vec<AstFix>,
    pub diagnostics: Vec<AstDiagnostic>,
    pub semantic_errors: Vec<SemanticError>,
    pub performance_metrics: PerformanceMetrics,
    pub source_map: Option<String>,

    // Comprehensive complexity analysis
    pub file_complexity: ComplexityMetrics,
    pub function_complexities: Vec<FunctionComplexity>,
    pub complexity_hotspots: Vec<ComplexityHotspot>,
    pub refactoring_suggestions: Vec<RefactoringSuggestion>,

    // Change analysis
    pub complexity_improvement: f64, // Percentage improvement in overall complexity
    pub maintainability_improvement: f64,
}

/// Project-wide analysis result combining multiple files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectAnalysisResult {
    pub project_path: String,
    pub files_analyzed: usize,
    pub total_files: usize,
    pub file_results: Vec<AstAutoFixResult>,
    pub project_complexity: ComplexityMetrics,
    pub dependency_graph: DependencyGraph,
    pub refactoring_opportunities: Vec<RefactoringSuggestion>,
    pub security_issues: Vec<SecurityIssue>,
}

/// Dependency graph for project-level analysis
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DependencyGraph {
    pub nodes: Vec<String>,         // File paths
    pub edges: Vec<(usize, usize)>, // Dependencies between files
    pub circular_dependencies: Vec<Vec<String>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Security issue detected during analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    pub file_path: String,
    pub severity: SecuritySeverity,
    pub category: SecurityCategory,
    pub description: String,
    pub line: u32,
    pub column: u32,
    pub code_snippet: String,
    pub recommendation: String,
    pub cwe_id: Option<u32>, // Common Weakness Enumeration ID
}

/// Severity levels for security issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Categories of security issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityCategory {
    Injection,
    Authentication,
    Authorization,
    Cryptography,
    InputValidation,
    DataExposure,
    UnusafeEval,
    PathTraversal,
    CrossSiteScripting,
    Other(String),
}

/// Individual AST-based fix with precise location and transformation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstFix {
    pub fix_type: AstFixType,
    pub description: String,
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
    pub original_text: String,
    pub fixed_text: String,
    pub confidence: f32,
    pub impact_score: u8,
}

/// Types of AST-based fixes available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AstFixType {
    // Type system improvements
    AddTypeAnnotation,
    ReplaceAnyType,
    FixNullishCoalescing,
    AddOptionalChaining,

    // Import/Export fixes
    OrganizeImports,
    RemoveUnusedImports,
    AddMissingImports,

    // Code quality improvements
    SimplifyConditionals,
    ExtractComplexExpressions,
    InlineSimpleVariables,

    // Modern JavaScript/TypeScript patterns
    ConvertToArrowFunction,
    UseConstAssertion,
    ApplyDestructuring,

    // Performance optimizations
    CacheArrayLength,
    UseMapOverObject,
    OptimizeRegexPatterns,

    // Documentation
    AddTSDocComments,
    FixDocumentationTags,

    // Security fixes
    RemoveEvalUsage,
    FixHardcodedSecrets,
    ValidateInputs,
    SanitizeInput,
    RemoveUnsafeFunction,
}

/// Detailed diagnostic information from OXC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstDiagnostic {
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub line: u32,
    pub column: u32,
    pub rule_name: Option<String>,
    pub suggested_fix: Option<String>,
}

/// Semantic analysis errors detected by OXC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticError {
    pub error_type: String,
    pub message: String,
    pub span: (u32, u32),
    pub severity: DiagnosticSeverity,
}

/// Performance metrics for AST processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub parse_time_ms: u64,
    pub semantic_analysis_ms: u64,
    pub transformation_ms: u64,
    pub codegen_ms: u64,
    pub total_time_ms: u64,
    pub memory_usage_bytes: usize,
    pub complexity_analysis_ms: u64,
}

/// Comprehensive code complexity metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityMetrics {
    // Traditional complexity measures
    pub cyclomatic_complexity: u32,
    pub cognitive_complexity: u32,
    pub halstead_difficulty: f64,
    pub halstead_volume: f64,
    pub halstead_effort: f64,

    // Modern complexity measures
    pub nesting_depth: u32,
    pub parameter_count: u32,
    pub lines_of_code: u32,
    pub maintainability_index: f64,

    // Advanced analysis
    pub dependency_complexity: u32,
    pub fan_in: u32,      // Number of modules that depend on this
    pub fan_out: u32,     // Number of modules this depends on
    pub instability: f64, // (fan_out / (fan_in + fan_out))

    // TypeScript specific
    pub type_complexity: u32,
    pub interface_complexity: u32,
    pub generic_complexity: u32,

    // Performance indicators
    pub async_complexity: u32,
    pub promise_chain_depth: u32,
    pub callback_nesting: u32,
}

impl Default for ComplexityMetrics {
    fn default() -> Self {
        Self {
            cyclomatic_complexity: 1, // Minimum complexity
            cognitive_complexity: 0,
            halstead_difficulty: 0.0,
            halstead_volume: 0.0,
            halstead_effort: 0.0,
            nesting_depth: 0,
            parameter_count: 0,
            lines_of_code: 0,
            maintainability_index: 100.0, // Maximum maintainability
            dependency_complexity: 0,
            fan_in: 0,
            fan_out: 0,
            instability: 0.0,
            type_complexity: 0,
            interface_complexity: 0,
            generic_complexity: 0,
            async_complexity: 0,
            promise_chain_depth: 0,
            callback_nesting: 0,
        }
    }
}

/// Detailed function/method complexity analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionComplexity {
    pub name: String,
    pub start_line: u32,
    pub end_line: u32,
    pub metrics: ComplexityMetrics,
    pub complexity_hotspots: Vec<ComplexityHotspot>,
    pub refactoring_suggestions: Vec<RefactoringSuggestion>,
}

/// Specific complexity hotspot within a function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityHotspot {
    pub hotspot_type: ComplexityHotspotType,
    pub line: u32,
    pub column: u32,
    pub description: String,
    pub impact_score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityHotspotType {
    DeepNesting,
    LongParameterList,
    ComplexConditional,
    CallbackHell,
    LargeSwitch,
    RepeatedCode,
    TypeComplexity,
    AsyncComplexity,
}

/// Refactoring suggestion to reduce complexity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringSuggestion {
    pub suggestion_type: RefactoringSuggestionType,
    pub description: String,
    pub estimated_complexity_reduction: u32,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefactoringSuggestionType {
    ExtractMethod,
    ExtractClass,
    SimplifyConditional,
    ReduceNesting,
    SplitFunction,
    IntroduceParameter,
    ReplaceConditionalWithPolymorphism,
    ConvertToAsyncAwait,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

/// Lint diagnostic emitted by the rule execution pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintDiagnostic {
    pub rule_name: String,
    pub message: String,
    pub file_path: String,
    pub line: u32,
    pub column: u32,
    pub end_line: u32,
    pub end_column: u32,
    pub severity: DiagnosticSeverity,
    pub fix_available: bool,
    pub suggested_fix: Option<String>,
}

/// Structured description of an available autofix for a lint diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixableLintIssue {
    pub rule_name: String,
    pub description: String,
    pub original_text: String,
    pub fixed_text: String,
    pub line: u32,
    pub column: u32,
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

// ================================================================================================
// MISSING RESULT TYPES - Added to fix refactoring compilation errors
// ================================================================================================

/// Result of TypeScript compilation process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScriptCompilationResult {
    pub success: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub diagnostics: Vec<AstDiagnostic>,
    pub output_code: Option<String>,
    pub source_map: Option<String>,
}

/// Result of ESLint linting and fixing process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESLintReplacementResult {
    pub success: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub fixes_applied: Vec<AstFix>,
    pub fixed_code: String,
    pub rules_violated: Vec<String>,
}

/// Result of Prettier formatting process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrettierReplacementResult {
    pub success: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub formatted_code: String,
    pub changes_made: bool,
}

/// Result of TSDoc documentation analysis process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TSDocReplacementResult {
    pub success: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub documented_items: Vec<DocumentedItem>,
    pub coverage_percentage: f64,
    pub missing_docs: Vec<String>,
}

/// Item with documentation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentedItem {
    pub name: String,
    pub item_type: String,
    pub has_documentation: bool,
    pub documentation_quality: f64,
}

/// Documentation issue found during analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationIssue {
    pub function_name: String,
    pub issue_type: String,
    pub line: u32,
    pub column: u32,
}
