//! Analysis types and functions for Moon Shine extension

use crate::config::MoonShineConfig;
use crate::rulebase::RuleResult as LintIssue;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// The response from a Moon Shine processing operation, including persistence data.
#[derive(Debug, Serialize, Deserialize)]
pub struct MoonShineResponse {
    /// If `true`, the operation was successful.
    pub success: bool,
    /// A message describing the result of the operation.
    pub message: String,
    /// The number of files that were processed.
    pub files_processed: u32,
    /// The number of issues that were found.
    pub issues_found: u32,
    /// The number of issues that were fixed.
    pub issues_fixed: u32,
    /// The number of COPRO optimizations that were performed.
    pub copro_optimizations: u32,
    /// The number of patterns that were learned.
    pub patterns_learned: u32,
    /// The total processing time in milliseconds.
    pub processing_time_ms: u64,
    /// A list of suggestions for improving the code.
    pub suggestions: Vec<LintIssue>,
    /// The fixed code content, if any.
    pub fixed_content: Option<String>,
    /// A list of insights about the code patterns that were found.
    pub pattern_insights: Option<Vec<String>>,
    /// For Moon to update `.moon/moonshine/prompts.json`.
    pub prompts_updates: Option<serde_json::Value>,
    /// For Moon to update `.moon/moonshine/training.json`.
    pub training_updates: Option<serde_json::Value>,
    /// The session state for Moon to manage.
    pub session_state: Option<serde_json::Value>,
}

/// The results of a code analysis operation from WASM processing.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnalysisResults {
    /// A list of suggestions for improving the code.
    pub suggestions: Vec<crate::rulebase::RuleResult>,
    /// A list of semantic warnings that were found.
    pub semantic_warnings: Vec<String>,
    /// The TSDoc coverage of the analyzed code.
    pub tsdoc_coverage: f32,
    /// The quality score of the analyzed code.
    pub quality_score: f32,
    /// A list of parse errors that were found.
    pub parse_errors: Vec<String>,
    /// A list of files that were ignored during the analysis.
    pub ignored_files: Vec<String>,
    /// The AI model that was used for the analysis.
    pub ai_model: String,
}

/// A request to a Moon task.
#[derive(Debug, Serialize, Deserialize)]
pub struct MoonTaskRequest {
    /// The ID of the session.
    pub session_id: String,
    /// The path of the file to be processed.
    pub file_path: String,
    /// The language of the file.
    pub language: String,
    /// The content of the file.
    pub content: String,
    /// The results of the initial analysis.
    pub analysis_results: AnalysisResults,
}

impl MoonTaskRequest {
    /// Creates a new `MoonTaskRequest`.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path of the file to be processed.
    /// * `language` - The language of the file.
    /// * `content` - The content of the file.
    /// * `analysis_results` - The results of the initial analysis.
    /// * `session_id` - The ID of the session.
    pub fn new(
        file_path: String,
        language: String,
        content: String,
        analysis_results: AnalysisResults,
        session_id: String,
    ) -> Self {
        Self {
            session_id,
            file_path,
            language,
            content,
            analysis_results,
        }
    }

    /// Converts the request to a JSON string.
    ///
    /// # Returns
    ///
    /// A `Result` containing the JSON string or an error.
    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string(self)?)
    }
}

/// The response from a Moon task.
#[derive(Debug, Serialize, Deserialize)]
pub struct MoonTaskResponse {
    /// The ID of the session.
    pub session_id: String,
    /// If `true`, the task was successful.
    pub success: bool,
    /// The results from the various tools that were run.
    pub results: TaskResults,
    /// The processing time in milliseconds.
    pub processing_time_ms: u64,
}

/// The results from the various tools that were run in a Moon task.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResults {
    /// The result of the TypeScript compilation.
    pub typescript: Option<TypeScriptResult>,
    /// The result of the ESLint analysis.
    pub eslint: Option<EslintResult>,
    /// The result of the Claude AI analysis.
    pub claude: Option<ClaudeResult>,
    /// The result of the semantic validation.
    pub semantic_validation: Option<SemanticValidationResult>,
}

/// The result of a TypeScript compilation.
#[derive(Debug, Serialize, Deserialize)]
pub struct TypeScriptResult {
    /// If `true`, the compilation was successful.
    pub compilation_success: bool,
    /// A list of type errors that were found.
    pub type_errors: Vec<String>,
}

/// The result of an ESLint analysis.
#[derive(Debug, Serialize, Deserialize)]
pub struct EslintResult {
    /// If `true`, the analysis was successful.
    pub success: bool,
    /// A list of issues that were found.
    pub issues: Vec<serde_json::Value>,
    /// The number of fixes that were applied.
    pub fixes_applied: u32,
}

/// The result of a Claude AI analysis.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeResult {
    /// If `true`, the analysis was successful.
    pub success: bool,
    /// The fixed code content, if any.
    pub fixed_content: Option<String>,
    /// The number of issues that were resolved.
    pub issues_resolved: u32,
}

/// The result of a semantic validation.
#[derive(Debug, Serialize, Deserialize)]
pub struct SemanticValidationResult {
    /// A list of unresolved warnings.
    pub unresolved_warnings: Vec<String>,
}

/// Detects the programming language from a file extension and configuration defaults.
///
/// # Arguments
///
/// * `file_path` - The path of the file to detect the language for.
/// * `config` - The Moon Shine configuration.
///
/// # Returns
///
/// The detected language as a string.
pub fn detect_language(file_path: &str, config: &MoonShineConfig) -> String {
    let extension = Path::new(file_path).extension().and_then(|ext| ext.to_str()).unwrap_or("");

    match extension {
        "ts" | "tsx" => "typescript".to_string(),
        "js" | "jsx" => "javascript".to_string(),
        "rs" => "rust".to_string(),
        "py" => "python".to_string(),
        "go" => "go".to_string(),
        "java" => "java".to_string(),
        "cpp" | "cc" | "cxx" | "c++" => "cpp".to_string(),
        "c" => "c".to_string(),
        _ => detect_language_with_fallback(file_path, config),
    }
}

/// Detects the programming language with a fallback to content-based detection and configuration defaults.
///
/// # Arguments
///
/// * `file_path` - The path of the file to detect the language for.
/// * `config` - The Moon Shine configuration.
///
/// # Returns
///
/// The detected language as a string.
fn detect_language_with_fallback(file_path: &str, config: &MoonShineConfig) -> String {
    if let Some(content_language) = detect_language_from_content(file_path) {
        return content_language;
    }

    if let Some(default_language) = config.default_language() {
        return default_language.to_string();
    }

    "typescript".to_string()
}

/// Detects the programming language from the file content using simple heuristics.
///
/// # Arguments
///
/// * `file_path` - The path of the file to detect the language for.
///
/// # Returns
///
/// An `Option` containing the detected language as a string.
fn detect_language_from_content(file_path: &str) -> Option<String> {
    let content = std::fs::read_to_string(file_path).ok()?;
    let content_lower = content.to_lowercase();

    if content_lower.contains("#!/usr/bin/env python")
        || content_lower.contains("import ") && content_lower.contains("def ")
    {
        return Some("python".to_string());
    }

    if content_lower.contains("#!/bin/bash") || content_lower.contains("#!/bin/sh") {
        return Some("bash".to_string());
    }

    if content_lower.contains("use ")
        && content_lower.contains("fn ")
        && content_lower.contains("impl ")
    {
        return Some("rust".to_string());
    }

    if content_lower.contains("package ") && content_lower.contains("func ") {
        return Some("go".to_string());
    }

    if content_lower.contains("class ") && content_lower.contains("public ") {
        return Some("java".to_string());
    }

    if content_lower.contains("#include") && content_lower.contains("int main") {
        return Some("cpp".to_string());
    }

    if content_lower.contains("<?php")
        || content_lower.contains('$') && content_lower.contains("function")
    {
        return Some("php".to_string());
    }

    if content_lower.contains("interface ")
        || content_lower.contains("type ")
        || content_lower.contains(": ")
    {
        return Some("typescript".to_string());
    }

    if content_lower.contains("function ")
        || content_lower.contains("var ")
        || content_lower.contains("const ")
    {
        return Some("javascript".to_string());
    }

    None
}

/// Gets the configurable Moon task name based on the analysis type and language.
///
/// # Arguments
///
/// * `config` - The Moon Shine configuration.
/// * `language` - The programming language of the file.
///
/// # Returns
///
/// The name of the Moon task to run.
fn resolve_moon_task_name(config: &MoonShineConfig, language: &str) -> String {
    config.resolve_task_name(language)
}

/// Analyzes a file using the extension analysis with the given configuration.
///
/// # Arguments
///
/// * `_content` - The content of the file.
/// * `_language` - The programming language of the file.
/// * `config` - The Moon Shine configuration.
///
/// # Returns
///
/// A `Result` containing the `AnalysisResults` or an error.
pub fn analyze_file_with_config(
    _content: &str,
    _language: &str,
    config: &MoonShineConfig,
) -> Result<AnalysisResults, Box<dyn std::error::Error>> {
    Ok(AnalysisResults {
        suggestions: Vec::new(),
        semantic_warnings: Vec::new(),
        tsdoc_coverage: 0.0,
        quality_score: 0.0,
        parse_errors: Vec::new(),
        ignored_files: Vec::new(),
        ai_model: config.ai_model.clone().unwrap_or_else(|| "sonnet".to_string()),
    })
}

/// The main AI linting function, with file content and config provided by the Moon host.
///
/// # Arguments
///
/// * `file_path` - The path of the file to lint.
/// * `content` - The content of the file.
/// * `config` - The Moon Shine configuration.
///
/// # Returns
///
/// A `Result` containing the `MoonShineResponse` or an error.
pub fn ai_lint_file_with_content(
    file_path: String,
    content: String,
    config: MoonShineConfig,
) -> std::result::Result<MoonShineResponse, Box<dyn std::error::Error>> {
    let claude_session_id = uuid::Uuid::new_v4().to_string();
    let start_time = std::time::Instant::now();

    let language = detect_language(&file_path, &config);
    let analysis_results = analyze_file_with_config(&content, &language, &config)?;

    let task_request = MoonTaskRequest::new(
        file_path.clone(),
        language.clone(),
        content.clone(),
        analysis_results.clone(),
        claude_session_id.clone(),
    );

    let task_response = execute_moon_tasks(&task_request, &config)?;
    let training_updates = generate_training_updates_from_analysis(&analysis_results)?;
    let prompts_updates = generate_prompts_updates_for_persistence(&config)?;

    let final_response = aggregate_analysis_results(
        analysis_results,
        task_response,
        start_time.elapsed().as_millis() as u64,
        training_updates,
        prompts_updates,
    );

    Ok(final_response)
}

/// Executes Moon tasks for comprehensive analysis.
///
/// # Arguments
///
/// * `request` - The `MoonTaskRequest` to execute.
/// * `config` - The Moon Shine configuration.
///
/// # Returns
///
/// A `Result` containing the `MoonTaskResponse` or an error.
fn execute_moon_tasks(
    request: &MoonTaskRequest,
    config: &MoonShineConfig,
) -> Result<MoonTaskResponse, Box<dyn std::error::Error>> {
    use crate::moon_pdk_interface::{execute_command, ExecCommandInput};

    let command_input = ExecCommandInput {
        command: "moon".to_string(),
        args: vec![
            "run".to_string(),
            resolve_moon_task_name(config, &request.language),
            request.to_json()?,
        ],
        env: std::collections::HashMap::new(),
        working_dir: None,
    };

    let output = execute_command(command_input)?;

    if output.exit_code != 0 {
        return Err(format!(
            "Moon task command failed with exit code {}: {}",
            output.exit_code, output.stderr
        )
        .into());
    }

    let mut moon_response: MoonTaskResponse = serde_json::from_str(&output.stdout)?;
    moon_response.session_id = request.session_id.clone();

    Ok(moon_response)
}

/// Generates `training.json` updates from analysis results for consolidated persistence.
///
/// # Arguments
///
/// * `analysis` - The analysis results.
///
/// # Returns
///
/// A `Result` containing the JSON value for the training updates or an error.
fn generate_training_updates_from_analysis(
    analysis: &AnalysisResults,
) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut patterns = serde_json::Map::new();

    for suggestion in &analysis.suggestions {
        if let Some(frequency) = suggestion.pattern_frequency {
            if frequency > 0.0 {
                let pattern_data = serde_json::json!({
                    "frequency": frequency,
                    "confidence": suggestion.ai_confidence,
                    "last_seen": chrono::Utc::now().to_rfc3339(),
                    "rule_type": suggestion.rule,
                    "severity": suggestion.severity
                });
                patterns.insert(suggestion.rule.to_string(), pattern_data);
            }
        }
    }

    Ok(serde_json::json!({
        "action": "update_training_data",
        "updates": {
            "learned_patterns": patterns,
            "meta": {
                "last_pattern_update": chrono::Utc::now().to_rfc3339(),
                "total_patterns": patterns.len(),
                "ai_model": analysis.ai_model
            }
        }
    }))
}

/// Generates `prompts.json` updates for consolidated persistence.
///
/// # Arguments
///
/// * `config` - The Moon Shine configuration.
///
/// # Returns
///
/// A `Result` containing the JSON value for the prompts updates or an error.
fn generate_prompts_updates_for_persistence(
    config: &MoonShineConfig,
) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut prompts = serde_json::Map::new();

    if let Some(custom_prompts) = &config.custom_prompts {
        for (rule_type, prompt) in custom_prompts {
            let prompt_data = serde_json::json!({
                "prompt": prompt,
                "rule_type": rule_type,
                "optimized": true
            });
            prompts.insert(rule_type.clone(), prompt_data);
        }
    }

    Ok(serde_json::json!({
        "action": "update_prompts",
        "updates": {
            "custom_prompts": prompts,
            "meta": {
                "last_prompt_update": chrono::Utc::now().to_rfc3339(),
                "custom_count": prompts.len()
            }
        }
    }))
}

/// Aggregates analysis results from the extension and Moon tasks with consolidated persistence data.
///
/// # TODO
///
/// This function should be refactored into smaller, focused functions.
fn aggregate_analysis_results(
    analysis_results: AnalysisResults,
    task_response: MoonTaskResponse,
    processing_time_ms: u64,
    training_updates: serde_json::Value,
    prompts_updates: serde_json::Value,
) -> MoonShineResponse {
    let AnalysisResults {
        suggestions,
        semantic_warnings,
        tsdoc_coverage,
        quality_score,
        ..
    } = analysis_results;

    let MoonTaskResponse {
        success: task_success,
        results,
        processing_time_ms: task_duration,
        ..
    } = task_response;

    let fix_outcome = determine_fix_outcome(&results);
    let pattern_insights = generate_pattern_insights(quality_score, tsdoc_coverage, &results);
    let (patterns_learned, copro_optimizations) =
        calculate_learning_counts(&training_updates, &prompts_updates, &semantic_warnings);
    let total_issues = suggestions.len() + semantic_warnings.len();
    let overall_success =
        determine_overall_success(task_success, fix_outcome.issues_fixed, suggestions.len());
    let message = compose_analysis_message(total_issues, fix_outcome.issues_fixed);
    let session_state = build_session_state();

    MoonShineResponse {
        success: overall_success,
        message,
        files_processed: 1,
        issues_found: total_issues as u32,
        issues_fixed: fix_outcome.issues_fixed,
        copro_optimizations,
        patterns_learned,
        processing_time_ms: processing_time_ms + task_duration,
        suggestions,
        fixed_content: fix_outcome.fixed_content,
        pattern_insights,
        prompts_updates: Some(prompts_updates),
        training_updates: Some(training_updates),
        session_state: Some(session_state),
    }
}

/// The outcome of a fixing operation.
struct FixOutcome {
    /// The number of issues that were fixed.
    issues_fixed: u32,
    /// The fixed code content, if any.
    fixed_content: Option<String>,
}

/// Determines the outcome of a fixing operation.
fn determine_fix_outcome(results: &TaskResults) -> FixOutcome {
    if let Some(claude) = results.claude.as_ref() {
        if claude.success {
            return FixOutcome {
                issues_fixed: claude.issues_resolved,
                fixed_content: claude.fixed_content.clone(),
            };
        }
    }

    if let Some(eslint) = results.eslint.as_ref() {
        if eslint.success {
            return FixOutcome {
                issues_fixed: eslint.fixes_applied,
                fixed_content: None,
            };
        }
    }

    FixOutcome {
        issues_fixed: 0,
        fixed_content: None,
    }
}

/// Generates insights about the code patterns that were found.
fn generate_pattern_insights(
    quality_score: f32,
    tsdoc_coverage: f32,
    results: &TaskResults,
) -> Option<Vec<String>> {
    let mut insights = vec![
        format!("Quality score: {:.2}", quality_score),
        format!("TSDoc coverage: {:.1}%", tsdoc_coverage * 100.0),
    ];

    if let Some(ts) = results.typescript.as_ref() {
        insights.push(format!(
            "TypeScript compilation success: {}",
            ts.compilation_success
        ));
    }

    if let Some(eslint) = results.eslint.as_ref() {
        insights.push(format!("ESLint pending issues: {}", eslint.issues.len()));
    }

    if let Some(validation) = results.semantic_validation.as_ref() {
        if !validation.unresolved_warnings.is_empty() {
            insights.push(format!(
                "Semantic warnings outstanding: {}",
                validation.unresolved_warnings.len()
            ));
        }
    }

    if insights.is_empty() {
        None
    } else {
        Some(insights)
    }
}

/// Calculates the number of patterns learned and COPRO optimizations performed.
fn calculate_learning_counts(
    training_updates: &serde_json::Value,
    prompts_updates: &serde_json::Value,
    semantic_warnings: &[String],
) -> (u32, u32) {
    let learned_count = training_updates
        .get("updates")
        .and_then(|updates| updates.get("learned_patterns"))
        .and_then(|value| value.as_object())
        .map(|collection| collection.len() as u32)
        .unwrap_or_else(|| semantic_warnings.len() as u32);

    let optimized_count = prompts_updates
        .get("updates")
        .and_then(|updates| updates.get("custom_prompts"))
        .and_then(|value| value.as_object())
        .map(|collection| collection.len() as u32)
        .unwrap_or(0);

    (learned_count, optimized_count)
}

/// Determines the overall success of the operation.
fn determine_overall_success(task_success: bool, issues_fixed: u32, suggestions_count: usize) -> bool {
    task_success && issues_fixed >= suggestions_count as u32
}

/// Composes a message summarizing the analysis results.
fn compose_analysis_message(total_issues: usize, issues_fixed: u32) -> String {
    format!(
        "Analysis complete: {} issues found, {} issues flagged for fixing",
        total_issues, issues_fixed
    )
}

/// Builds the session state to be persisted.
fn build_session_state() -> serde_json::Value {
    serde_json::json!({
        "last_analysis": chrono::Utc::now().to_rfc3339(),
        "repository_path": "workspace",
        "plugin_version": env!("CARGO_PKG_VERSION"),
    })
}

#[cfg(all(test, not(feature = "wasm")))]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_detect_language() {
        let config = MoonShineConfig::default();
        // Test TypeScript files
        assert_eq!(detect_language("src/component.ts", &config), "typescript");
        assert_eq!(detect_language("src/component.tsx", &config), "typescript");

        // Test JavaScript files
        assert_eq!(detect_language("src/script.js", &config), "javascript");
        assert_eq!(detect_language("src/component.jsx", &config), "javascript");

        // Test other languages
        assert_eq!(detect_language("src/main.rs", &config), "rust");
        assert_eq!(detect_language("src/main.py", &config), "python");
        assert_eq!(detect_language("src/main.go", &config), "go");
        assert_eq!(detect_language("src/Main.java", &config), "java");
        assert_eq!(detect_language("src/main.cpp", &config), "cpp");
        assert_eq!(detect_language("src/main.c", &config), "c");

        // Test unknown extension defaults to TypeScript
        assert_eq!(detect_language("src/unknown.xyz", &config), "typescript");
        assert_eq!(detect_language("no_extension", &config), "typescript");
    }

    #[test]
    fn test_detect_language_with_config_default() {
        let mut config = MoonShineConfig::default();
        config.default_language = Some("python".to_string());
        assert_eq!(detect_language("README", &config), "python");
    }

    #[test]
    fn test_resolve_task_name_with_mapping() {
        let mut config = MoonShineConfig::default();
        config.moon_task_mapping = Some(HashMap::from([("rust".to_string(), "moon-shine:custom-rust".to_string())]));
        assert_eq!(config.resolve_task_name("rust"), "moon-shine:custom-rust");

        config.moon_task_name = Some("moon-shine:override".to_string());
        assert_eq!(config.resolve_task_name("python"), "moon-shine:override");
        assert_eq!(config.resolve_task_name("unknown"), "moon-shine:override");
    }

    #[test]
    fn test_lint_issue_creation() {
        let suggestion = crate::rulebase::RuleResult::new("no-unused-vars".to_string(), "Variable 'x' is never used".to_string())
            .with_severity("warning".to_string())
            .with_location(10, 5)
            .with_suggestion("Fix available".to_string());

        assert_eq!(suggestion.rule_id, "no-unused-vars");
        assert_eq!(suggestion.line, 10);
        assert_eq!(suggestion.column, 5);
        assert!(suggestion.suggestion.is_some());
    }

    #[test]
    fn test_analysis_results_creation() {
        let suggestion = crate::rulebase::RuleResult::new("no-unused-vars".to_string(), "Variable 'x' is never used".to_string())
            .with_severity("warning".to_string())
            .with_location(10, 5)
            .with_suggestion("Fix available".to_string());

        let results = AnalysisResults {
            suggestions: vec![suggestion],
            semantic_warnings: vec!["Unused import".to_string()],
            tsdoc_coverage: 0.75,
            quality_score: 0.80,
            parse_errors: vec![],
            ignored_files: vec![],
            ai_model: "sonnet".to_string(),
        };

        assert_eq!(results.suggestions.len(), 1);
        assert_eq!(results.semantic_warnings.len(), 1);
        assert!((results.tsdoc_coverage - 0.75).abs() < f32::EPSILON);
        assert!((results.quality_score - 0.80).abs() < f32::EPSILON);
        assert_eq!(results.ai_model, "sonnet");
    }

    #[test]
    fn test_moon_task_request_creation() {
        let analysis_results = AnalysisResults {
            suggestions: vec![],
            semantic_warnings: vec![],
            tsdoc_coverage: 0.0,
            quality_score: 0.0,
            parse_errors: vec![],
            ignored_files: vec![],
            ai_model: "sonnet".to_string(),
        };

        let request = MoonTaskRequest::new(
            "src/test.ts".to_string(),
            "typescript".to_string(),
            "const x = 1;".to_string(),
            analysis_results,
            "session-123".to_string(),
        );

        assert_eq!(request.session_id, "session-123");
        assert_eq!(request.file_path, "src/test.ts");
        assert_eq!(request.language, "typescript");
        assert_eq!(request.content, "const x = 1;");
        assert_eq!(request.analysis_results.ai_model, "sonnet");
    }

    #[test]
    fn test_moon_task_request_json_serialization() {
        let analysis_results = AnalysisResults {
            suggestions: vec![],
            semantic_warnings: vec![],
            tsdoc_coverage: 0.5,
            quality_score: 0.8,
            parse_errors: vec![],
            ignored_files: vec![],
            ai_model: "sonnet".to_string(),
        };

        let request = MoonTaskRequest::new(
            "src/test.ts".to_string(),
            "typescript".to_string(),
            "const x = 1;".to_string(),
            analysis_results,
            "session-123".to_string(),
        );

        let json_result = request.to_json();
        assert!(json_result.is_ok());

        let json_str = json_result.unwrap();
        assert!(json_str.contains("session-123"));
        assert!(json_str.contains("src/test.ts"));
        assert!(json_str.contains("typescript"));
        assert!(json_str.contains("sonnet"));
    }

    #[test]
    fn test_analyze_file_with_config() {
        let config = MoonShineConfig {
            ai_model: Some("sonnet".to_string()),
            max_tokens: Some(4000),
            enable_copro_optimization: Some(true),
            custom_prompts: None,
            include_patterns: None,
            temperature: Some(0.7),
            ..Default::default()
        };

        let result = analyze_file_with_config("const x = 1;", "typescript", &config);
        assert!(result.is_ok());

        let analysis_results = result.unwrap();
        assert_eq!(analysis_results.ai_model, "sonnet");
        assert_eq!(analysis_results.suggestions.len(), 0); // Extension doesn't do analysis
        assert_eq!(analysis_results.tsdoc_coverage, 0.0);
        assert_eq!(analysis_results.quality_score, 0.0);
    }

    #[test]
    fn test_analyze_file_with_config_default_model() {
        let config = MoonShineConfig {
            ai_model: None, // No model specified
            max_tokens: Some(4000),
            enable_copro_optimization: Some(true),
            custom_prompts: None,
            include_patterns: None,
            temperature: Some(0.7),
            ..Default::default()
        };

        let result = analyze_file_with_config("const x = 1;", "typescript", &config);
        assert!(result.is_ok());

        let analysis_results = result.unwrap();
        assert_eq!(analysis_results.ai_model, "sonnet"); // Should default to sonnet
    }

    #[test]
    fn test_typescript_result() {
        let ts_result = TypeScriptResult {
            compilation_success: true,
            type_errors: vec!["Type 'string' is not assignable to type 'number'".to_string()],
        };

        assert!(ts_result.compilation_success);
        assert_eq!(ts_result.type_errors.len(), 1);
        assert!(ts_result.type_errors[0].contains("not assignable"));
    }

    #[test]
    fn test_eslint_result() {
        let eslint_result = EslintResult {
            success: true,
            issues: vec![serde_json::json!({"ruleId": "no-unused-vars", "severity": "warning"})],
            fixes_applied: 5,
        };

        assert!(eslint_result.success);
        assert_eq!(eslint_result.issues.len(), 1);
        assert_eq!(eslint_result.fixes_applied, 5);
    }

    #[test]
    fn test_claude_result() {
        let claude_result = ClaudeResult {
            success: true,
            fixed_content: Some("const x: number = 1;".to_string()),
            issues_resolved: 3,
        };

        assert!(claude_result.success);
        assert!(claude_result.fixed_content.is_some());
        assert_eq!(claude_result.issues_resolved, 3);
        assert!(claude_result.fixed_content.unwrap().contains("number"));
    }

    #[test]
    fn test_semantic_validation_result() {
        let validation_result = SemanticValidationResult {
            unresolved_warnings: vec!["Unused import 'React'".to_string(), "Missing return type".to_string()],
        };

        assert_eq!(validation_result.unresolved_warnings.len(), 2);
        assert!(validation_result.unresolved_warnings[0].contains("Unused import"));
        assert!(validation_result.unresolved_warnings[1].contains("Missing return"));
    }

    #[test]
    fn test_task_results() {
        let task_results = TaskResults {
            typescript: Some(TypeScriptResult {
                compilation_success: false,
                type_errors: vec!["Error on line 5".to_string()],
            }),
            eslint: Some(EslintResult {
                success: true,
                issues: vec![],
                fixes_applied: 2,
            }),
            claude: Some(ClaudeResult {
                success: true,
                fixed_content: Some("Fixed code".to_string()),
                issues_resolved: 1,
            }),
            semantic_validation: None,
        };

        assert!(task_results.typescript.is_some());
        assert!(task_results.eslint.is_some());
        assert!(task_results.claude.is_some());
        assert!(task_results.semantic_validation.is_none());

        let ts = task_results.typescript.unwrap();
        assert!(!ts.compilation_success);
        assert_eq!(ts.type_errors.len(), 1);

        let eslint = task_results.eslint.unwrap();
        assert!(eslint.success);
        assert_eq!(eslint.fixes_applied, 2);

        let claude = task_results.claude.unwrap();
        assert!(claude.success);
        assert_eq!(claude.issues_resolved, 1);
    }

    #[test]
    fn test_moon_task_response() {
        let response = MoonTaskResponse {
            session_id: "session-456".to_string(),
            success: true,
            results: TaskResults {
                typescript: None,
                eslint: None,
                claude: None,
                semantic_validation: None,
            },
            processing_time_ms: 1500,
        };

        assert_eq!(response.session_id, "session-456");
        assert!(response.success);
        assert_eq!(response.processing_time_ms, 1500);
    }

    #[test]
    fn test_moon_shine_response_creation() {
        let suggestion = crate::rulebase::RuleResult::new("no-unused-vars".to_string(), "Variable 'x' is never used".to_string())
            .with_severity("warning".to_string())
            .with_location(10, 5)
            .with_suggestion("Fix available".to_string());

        let response = MoonShineResponse {
            success: true,
            message: "Analysis complete".to_string(),
            files_processed: 1,
            issues_found: 3,
            issues_fixed: 2,
            copro_optimizations: 1,
            patterns_learned: 2,
            processing_time_ms: 2500,
            suggestions: vec![suggestion],
            fixed_content: Some("Fixed code content".to_string()),
            pattern_insights: Some(vec!["Quality improved".to_string()]),
            prompts_updates: None,
            training_updates: None,
            session_state: None,
        };

        assert!(response.success);
        assert_eq!(response.files_processed, 1);
        assert_eq!(response.issues_found, 3);
        assert_eq!(response.issues_fixed, 2);
        assert_eq!(response.suggestions.len(), 1);
        assert!(response.fixed_content.is_some());
        assert!(response.pattern_insights.is_some());
    }
}
