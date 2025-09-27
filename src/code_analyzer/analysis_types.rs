//! Analysis result types and data structures for code analysis
//!
//! Self-documenting types for AST analysis, complexity metrics, and security analysis.

use serde::{Deserialize, Serialize};

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

/// Diagnostic severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
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
    pub analysis_time_ms: u64,
    pub semantic_time_ms: u64,
    pub memory_used_kb: u64,
}
