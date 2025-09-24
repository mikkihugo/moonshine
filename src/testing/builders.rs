//! # Test Object Builders
//!
//! Fluent builder patterns for test object construction.
//! Enables readable and maintainable test setup.
//!
//! @category testing
//! @safe program
//! @complexity medium
//! @since 2.0.0

use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::analysis::{AnalysisResults, MoonShineResponse};
use crate::javascript_typescript_linter::{LintIssue, LintSeverity};
use crate::linter::SuggestionCategory;

// AI Suggestion Builder for testing
#[derive(Debug, Clone)]
pub struct AiSuggestionBuilder {
    message: String,
    file_path: String,
    line_number: u32,
    category: SuggestionCategory,
    confidence_score: f64,
}

impl AiSuggestionBuilder {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            file_path: "test.ts".to_string(),
            line_number: 1,
            category: SuggestionCategory::Performance,
            confidence_score: 0.9,
        }
    }

    pub fn error() -> Self {
        Self::new().category(SuggestionCategory::Error)
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn file_path(mut self, file_path: &str) -> Self {
        self.file_path = file_path.to_string();
        self
    }

    pub fn line_number(mut self, line_number: u32) -> Self {
        self.line_number = line_number;
        self
    }

    pub fn category(mut self, category: SuggestionCategory) -> Self {
        self.category = category;
        self
    }

    pub fn confidence_score(mut self, score: f64) -> Self {
        self.confidence_score = score;
        self
    }

    pub fn build(self) -> LintIssue {
        LintIssue {
            rule_name: "test-rule".to_string(),
            message: self.message,
            line: self.line_number,
            column: 1,
            severity: LintSeverity::Warning,
            fix_available: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LintIssueBuilder {
    rule_name: String,
    message: String,
    line: u32,
    column: u32,
    severity: LintSeverity,
    fix_available: bool,
}

impl LintIssueBuilder {
    pub fn new() -> Self {
        Self {
            rule_name: String::new(),
            message: String::new(),
            line: 1,
            column: 1,
            severity: LintSeverity::Warning,
            fix_available: false,
        }
    }

    pub fn rule_name(mut self, rule_name: &str) -> Self {
        self.rule_name = rule_name.to_string();
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn severity(mut self, severity: LintSeverity) -> Self {
        self.severity = severity;
        self
    }

    pub fn fix_available(mut self, fix_available: bool) -> Self {
        self.fix_available = fix_available;
        self
    }

    pub fn line(mut self, line: u32) -> Self {
        self.line = line;
        self
    }

    pub fn column(mut self, column: u32) -> Self {
        self.column = column;
        self
    }

    pub fn build(self) -> LintIssue {
        LintIssue {
            rule_name: self.rule_name,
            message: self.message,
            line: self.line,
            column: self.column,
            severity: self.severity,
            fix_available: self.fix_available,
        }
    }

    pub fn error() -> Self {
        Self::new()
            .rule_name("error_rule")
            .severity(LintSeverity::Error)
            .message("Error level issue found")
            .fix_available(false)
    }

    pub fn warning() -> Self {
        Self::new()
            .rule_name("warning_rule")
            .severity(LintSeverity::Warning)
            .message("Warning level issue found")
            .fix_available(false)
    }

    pub fn info() -> Self {
        Self::new()
            .rule_name("info_rule")
            .severity(LintSeverity::Info)
            .message("Info level suggestion")
            .fix_available(false)
    }
}

/// Builder for analysis results
#[derive(Debug, Clone)]
pub struct AnalysisResultsBuilder {
    suggestions: Vec<LintIssue>,
}

impl AnalysisResultsBuilder {
    /// Create new analysis results builder
    pub fn new() -> Self {
        Self { suggestions: Vec::new() }
    }

    /// Add suggestion
    pub fn suggestion(mut self, suggestion: LintIssue) -> Self {
        self.suggestions.push(suggestion);
        self
    }

    /// Add multiple suggestions
    pub fn suggestions(mut self, suggestions: Vec<LintIssue>) -> Self {
        self.suggestions.extend(suggestions);
        self
    }

    // Removed obsolete builder methods: files_processed, processing_time, metadata

    /// Build analysis results
    pub fn build(self) -> AnalysisResults {
        AnalysisResults {
            suggestions: self.suggestions,
            ..Default::default()
        }
    }

    /// Create empty results (clean code)
    pub fn clean() -> Self {
        Self::new()
    }

    /// Create results with TypeScript issues
    pub fn typescript_issues() -> Self {
        Self::new()
            .suggestion(
                LintIssueBuilder::new()
                    .rule_name("typescript_any_type")
                    .message("TypeScript any type used")
                    .severity(LintSeverity::Warning)
                    .build(),
            )
            .suggestion(
                LintIssueBuilder::new()
                    .rule_name("console_log_warning")
                    .message("console.log used")
                    .severity(LintSeverity::Warning)
                    .build(),
            )
    }

    /// Create results for TypeScript analysis
    pub fn typescript_analysis() -> Self {
        Self::new()
            .suggestion(
                LintIssueBuilder::new()
                    .rule_name("typescript_any_type")
                    .message("TypeScript any type used")
                    .severity(LintSeverity::Warning)
                    .line(5)
                    .build(),
            )
            .suggestion(
                LintIssueBuilder::new()
                    .rule_name("console_log_warning")
                    .message("console.log used")
                    .severity(LintSeverity::Warning)
                    .line(12)
                    .build(),
            )
    }

    /// Create results with performance issues
    pub fn performance_issues() -> Self {
        Self::new()
            .suggestion(
                LintIssueBuilder::new()
                    .rule_name("performance_issue")
                    .message("Performance issue detected")
                    .severity(LintSeverity::Warning)
                    .build(),
            )
            .suggestion(
                LintIssueBuilder::new()
                    .rule_name("inline_function_creation")
                    .message("Inline function creation in render")
                    .severity(LintSeverity::Warning)
                    .build(),
            )
    }

    /// Create results for large codebase
    pub fn large_codebase(file_count: usize) -> Self {
        let mut builder = Self::new();

        // Add suggestions proportional to file count
        for i in 0..file_count.min(10) {
            builder = builder.suggestion(LintIssueBuilder::warning().line(i as u32).message(&format!("Issue in file {}", i)).build());
        }

        builder
        // .processing_time((file_count as u64) * 50) // 50ms per file
        // .metadata("analysis_type", "large_codebase")
        // .metadata("file_count", &file_count.to_string())
    }
}

impl Default for AnalysisResultsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for moon-shine configuration
#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    ai_model: String,
    include_patterns: Vec<String>,
    exclude_patterns: Vec<String>,
}

impl ConfigBuilder {
    /// Create new config builder
    pub fn new() -> Self {
        Self {
            ai_model: "test-model".to_string(),
            include_patterns: vec!["**/*.ts".to_string()],
            exclude_patterns: vec!["**/node_modules/**".to_string()],
        }
    }

    /// Set AI model
    pub fn ai_model(mut self, model: &str) -> Self {
        self.ai_model = model.to_string();
        self
    }

    /// Add include pattern
    pub fn include_pattern(mut self, pattern: &str) -> Self {
        self.include_patterns.push(pattern.to_string());
        self
    }

    /// Build configuration
    pub fn build(self) -> MoonShineConfig {
        MoonShineConfig {
            ai_model: Some(self.ai_model),
            include_patterns: Some(self.include_patterns),
            exclude_patterns: Some(self.exclude_patterns),
            ..Default::default()
        }
    }

    /// Create development configuration
    pub fn development() -> Self {
        Self::new()
            .ai_model("development-model")
            .include_pattern("**/*.ts")
            .include_pattern("**/*.tsx")
            .include_pattern("**/*.js")
            .enable_cache()
    }

    /// Create production configuration
    pub fn production() -> Self {
        Self::new()
            .ai_model("production-model")
            .include_pattern("**/*.ts")
            .include_pattern("**/*.tsx")
            .exclude_pattern("**/node_modules/**")
            .exclude_pattern("**/dist/**")
            .exclude_pattern("**/*.test.*")
            .enable_cache()
    }

    /// Create testing configuration
    pub fn testing() -> Self {
        Self::new()
            .ai_model("test-model")
            .include_pattern("**/*.test.ts")
            .include_pattern("**/*.spec.ts")
            .disable_cache()
    }

    /// Create performance testing configuration
    pub fn performance() -> Self {
        Self::new()
            .ai_model("performance-test-model")
            .include_pattern("**/*.ts")
            .include_pattern("**/*.tsx")
            .include_pattern("**/*.js")
            .include_pattern("**/*.jsx")
            .enable_cache()
            .field("performance_mode", "true")
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for workflow steps
#[derive(Debug, Clone)]
pub struct WorkflowStepBuilder {
    name: String,
    description: String,
    duration_ms: u64,
    success: bool,
    output: Option<String>,
    metadata: HashMap<String, String>,
}

impl WorkflowStepBuilder {
    /// Create new workflow step builder
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: format!("Default description for {}", name),
            duration_ms: 100,
            success: true,
            output: None,
            metadata: HashMap::new(),
        }
    }

    /// Set description
    pub fn description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Set duration
    pub fn duration(mut self, ms: u64) -> Self {
        self.duration_ms = ms;
        self
    }

    /// Mark as successful
    pub fn success(mut self) -> Self {
        self.success = true;
        self
    }

    /// Mark as failed
    pub fn failure(mut self) -> Self {
        self.success = false;
        self
    }

    /// Set output
    pub fn output(mut self, output: &str) -> Self {
        self.output = Some(output.to_string());
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    /// Build workflow step
    pub fn build(self) -> WorkflowStep {
        WorkflowStep {
            name: self.name,
            description: self.description,
            duration_ms: self.duration_ms,
            success: self.success,
            output: self.output,
            metadata: self.metadata,
        }
    }

    /// Create pre-analysis step
    pub fn pre_analysis() -> Self {
        Self::new("pre_analysis").description("Prepare files for analysis").duration(50).success()
    }

    /// Create analysis step
    pub fn analysis() -> Self {
        Self::new("analysis")
            .description("Analyze code for issues")
            .duration(200)
            .success()
            .metadata("files_analyzed", "5")
    }

    /// Create post-analysis step
    pub fn post_analysis() -> Self {
        Self::new("post_analysis")
            .description("Generate suggestions and reports")
            .duration(100)
            .success()
    }
}

/// Collection of pre-built test objects
pub struct TestBuilders;

impl TestBuilders {
    /// Create suggestion for TypeScript any type usage
    pub fn typescript_any_suggestion() -> LintIssue {
        LintIssueBuilder::new()
            .rule_name("typescript_any_type")
            .message("TypeScript any type used")
            .severity(LintSeverity::Warning)
            .build()
    }

    /// Create suggestion for console.log usage
    pub fn console_log_suggestion() -> LintIssue {
        LintIssueBuilder::new()
            .rule_name("console_log_warning")
            .message("console.log used")
            .severity(LintSeverity::Warning)
            .build()
    }

    /// Create analysis results with common TypeScript issues
    pub fn common_typescript_issues() -> AnalysisResults {
        AnalysisResultsBuilder::typescript_issues().build()
    }

    /// Create clean analysis results
    pub fn clean_results() -> AnalysisResults {
        AnalysisResultsBuilder::clean().build()
    }

    /// Create development configuration
    pub fn dev_config() -> MoonShineConfig {
        ConfigBuilder::development().build()
    }

    /// Create production configuration
    pub fn prod_config() -> MoonShineConfig {
        ConfigBuilder::production().build()
    }

    /// Create test configuration
    pub fn test_config() -> MoonShineConfig {
        ConfigBuilder::testing().build()
    }

    /// Create workflow with typical steps
    pub fn typical_workflow() -> Vec<WorkflowStep> {
        vec![
            WorkflowStepBuilder::pre_analysis().build(),
            WorkflowStepBuilder::analysis().build(),
            WorkflowStepBuilder::post_analysis().build(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lint_issue_builder() {
        let issue = LintIssueBuilder::new()
            .rule_name("test_rule")
            .message("Test message")
            .severity(LintSeverity::Error)
            .line(10)
            .column(2)
            .fix_available(true)
            .build();

        assert_eq!(issue.rule_name, "test_rule");
        assert_eq!(issue.message, "Test message");
        assert!(matches!(issue.severity, LintSeverity::Error));
        assert_eq!(issue.line, 10);
        assert_eq!(issue.column, 2);
        assert!(issue.fix_available);
    }

    #[test]
    fn test_analysis_results_builder() {
        let issue = LintIssueBuilder::warning().build();
        let results = AnalysisResultsBuilder::new().suggestion(issue).build();

        assert_eq!(results.suggestions.len(), 1);
    }

    #[test]
    fn test_config_builder() {
        let config = ConfigBuilder::new().ai_model("custom-model").include_pattern("**/*.tsx").build();

        assert_eq!(config.ai_model.as_deref(), Some("custom-model"));
        assert!(config.include_patterns.as_ref().map_or(false, |v| v.contains(&"**/*.tsx".to_string())));
    }

    #[test]
    fn test_predefined_builders() {
        let typescript_issue = TestBuilders::typescript_any_suggestion();
        assert!(typescript_issue.message.contains("any"));
        assert!(matches!(typescript_issue.severity, LintSeverity::Warning));

        let console_issue = TestBuilders::console_log_suggestion();
        assert!(console_issue.message.contains("console.log"));
        assert!(matches!(console_issue.severity, LintSeverity::Warning));

        let clean_results = TestBuilders::clean_results();
        assert!(clean_results.suggestions.is_empty());

        let dev_config = TestBuilders::dev_config();
        assert_eq!(dev_config.ai_model.as_deref(), Some("development-model"));
        // cache_enabled field removed
    }

    #[test]
    fn test_workflow_step_builder() {
        let step = WorkflowStepBuilder::new("test_step")
            .description("Test step description")
            .duration(150)
            .success()
            .output("Test output")
            .metadata("key", "value")
            .build();

        assert_eq!(step.name, "test_step");
        assert_eq!(step.description, "Test step description");
        assert_eq!(step.duration_ms, 150);
        assert!(step.success);
        assert_eq!(step.output, Some("Test output".to_string()));
        assert_eq!(step.metadata.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_predefined_configurations() {
        let dev = ConfigBuilder::development().build();
        assert_eq!(dev.ai_model.as_deref(), Some("development-model"));
        // cache_enabled field removed

        let prod = ConfigBuilder::production().build();
        assert_eq!(prod.ai_model.as_deref(), Some("production-model"));
        // max_files field removed

        let test = ConfigBuilder::testing().build();
        assert_eq!(test.ai_model.as_deref(), Some("test-model"));
        // cache_enabled field removed
    }
}
