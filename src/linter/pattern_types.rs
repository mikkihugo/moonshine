//! Pattern matching types and configurations for code analysis
//!
//! Self-documenting pattern structures and matching algorithms.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Configurable pattern structure for external configuration
/// Enables loading patterns from JSON files for customization without recompilation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurablePatterns {
    pub function_patterns: Vec<String>,
    pub conditional_patterns: Vec<String>,
    pub loop_patterns: Vec<String>,
    pub comment_patterns: Vec<String>,
    pub test_patterns: Vec<String>,
    pub security_patterns: Vec<String>,
    pub performance_patterns: Vec<String>,
    pub typescript_patterns: Vec<String>,
    pub documentation_patterns: Vec<String>,
}

/// Task metrics for Moon integration and performance monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoonTaskMetrics {
    pub task_name: String,
    pub start_time: String,
    pub end_time: String,
    pub duration_ms: u64,
    pub files_processed: u32,
    pub suggestions_generated: u32,
    pub fixes_applied: u32,
    pub success_rate: f32,
    pub error_count: u32,
    pub warnings: Vec<String>,
}

/// Advanced AST-based metrics for comprehensive code analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstBasedMetrics {
    pub cyclomatic_complexity: u32,
    pub cognitive_complexity: u32,
    pub nesting_depth: u32,
    pub function_count: u32,
    pub class_count: u32,
    pub interface_count: u32,
    pub lines_of_code: u32,
    pub comment_ratio: f32,
    pub import_count: u32,
    pub export_count: u32,
    pub type_annotation_coverage: f32,
    pub test_coverage_estimate: f32,

    // Advanced metrics
    pub dependency_fan_in: u32,
    pub dependency_fan_out: u32,
    pub method_complexity_distribution: HashMap<String, u32>,
    pub type_usage_frequency: HashMap<String, u32>,
    pub async_usage_count: u32,
    pub promise_chain_depth: u32,
    pub callback_nesting_level: u32,
}

/// High-performance pattern matcher with multiple algorithms
#[derive(Debug)]
pub struct CodePatternMatcher {
    pub aho_corasick_matcher: Option<aho_corasick::AhoCorasick>,
    pub regex_patterns: Vec<regex::Regex>,
    pub function_signatures: HashSet<String>,
    pub security_keywords: HashSet<String>,
    pub performance_anti_patterns: Vec<String>,
    pub typescript_specific_patterns: Vec<String>,
    pub configurable_patterns: ConfigurablePatterns,
}

/// Comprehensive code metrics combining static and dynamic analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeMetrics {
    pub file_path: String,
    pub language: String,
    pub total_lines: u32,
    pub code_lines: u32,
    pub comment_lines: u32,
    pub blank_lines: u32,
    pub function_count: u32,
    pub class_count: u32,
    pub complexity_score: f32,
    pub maintainability_index: f32,
    pub technical_debt_hours: f32,
    pub test_coverage_percentage: f32,
    pub ast_metrics: AstBasedMetrics,
    pub moon_task_metrics: MoonTaskMetrics,
}

/// Detailed analysis result with actionable insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedCodeAnalysis {
    pub file_path: String,
    pub analysis_timestamp: String,
    pub pattern_matches: Vec<PatternMatch>,
    pub suggestions: Vec<AiSuggestion>,
    pub metrics: CodeMetrics,
    pub security_issues: Vec<SecurityIssue>,
    pub performance_recommendations: Vec<PerformanceRecommendation>,
    pub refactoring_opportunities: Vec<RefactoringOpportunity>,
}

/// Pattern match result with contextual information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMatch {
    pub pattern_name: String,
    pub line_number: u32,
    pub column_start: u32,
    pub column_end: u32,
    pub matched_text: String,
    pub severity: MatchSeverity,
    pub confidence: f32,
    pub description: String,
    pub suggested_fix: Option<String>,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// AI-generated suggestion with confidence scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSuggestion {
    pub id: String,
    pub suggestion_type: String,
    pub title: String,
    pub description: String,
    pub suggested_code: Option<String>,
    pub line_start: u32,
    pub line_end: u32,
    pub confidence: f32,
    pub severity: SuggestionSeverity,
    pub estimated_time_to_fix: u32, // minutes
    pub prerequisites: Vec<String>,
    pub impact_assessment: String,
    pub alternative_solutions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionSeverity {
    Critical,
    High,
    Medium,
    Low,
    Suggestion,
}

/// Security issue detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    pub issue_type: String,
    pub severity: String,
    pub description: String,
    pub line_number: u32,
    pub remediation_advice: String,
    pub cwe_reference: Option<String>,
}

/// Performance improvement recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRecommendation {
    pub recommendation_type: String,
    pub impact_level: String,
    pub description: String,
    pub line_number: u32,
    pub estimated_improvement: String,
    pub implementation_complexity: String,
}

/// Code refactoring opportunity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringOpportunity {
    pub refactoring_type: String,
    pub complexity_reduction: f32,
    pub maintainability_improvement: f32,
    pub description: String,
    pub affected_lines: Vec<u32>,
    pub implementation_steps: Vec<String>,
}
