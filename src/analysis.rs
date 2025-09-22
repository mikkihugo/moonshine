//! Analysis types and functions for Moon Shine extension

use crate::config::MoonShineConfig;
use crate::rulebase::RuleResult as LintIssue;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Response from Moon Shine processing with persistence data
#[derive(Debug, Serialize, Deserialize)]
pub struct MoonShineResponse {
    pub success: bool,
    pub message: String,
    pub files_processed: u32,
    pub issues_found: u32,
    pub issues_fixed: u32,
    pub copro_optimizations: u32,
    pub patterns_learned: u32,
    pub processing_time_ms: u64,
    pub suggestions: Vec<LintIssue>,
    pub fixed_content: Option<String>,
    pub pattern_insights: Option<Vec<String>>,

    // Consolidated storage fields for simplified architecture
    pub prompts_updates: Option<serde_json::Value>,  // For Moon to update .moon/moonshine/prompts.json
    pub training_updates: Option<serde_json::Value>, // For Moon to update .moon/moonshine/training.json
    pub session_state: Option<serde_json::Value>,    // Session state for Moon to manage
}

/// Suggestion from analysis with optimized data types
/// Code analysis results from WASM processing
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnalysisResults {
    pub suggestions: Vec<crate::rulebase::RuleResult>,
    pub semantic_warnings: Vec<String>,
    pub tsdoc_coverage: f32,
    pub quality_score: f32,
    pub parse_errors: Vec<String>,
    pub ignored_files: Vec<String>,
    pub ai_model: String,
}

/// Moon task request structure
#[derive(Debug, Serialize, Deserialize)]
pub struct MoonTaskRequest {
    pub session_id: String,
    pub file_path: String,
    pub language: String,
    pub content: String,
    pub analysis_results: AnalysisResults,
}

impl MoonTaskRequest {
    pub fn new(file_path: String, language: String, content: String, analysis_results: AnalysisResults, session_id: String) -> Self {
        Self {
            session_id,
            file_path,
            language,
            content,
            analysis_results,
        }
    }

    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string(self)?)
    }
}

/// Moon task response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct MoonTaskResponse {
    pub session_id: String,
    pub success: bool,
    pub results: TaskResults,
    pub processing_time_ms: u64,
}

/// Task results from various tools
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResults {
    pub typescript: Option<TypeScriptResult>,
    pub eslint: Option<EslintResult>,
    pub claude: Option<ClaudeResult>,
    pub semantic_validation: Option<SemanticValidationResult>,
}

/// TypeScript compilation result
#[derive(Debug, Serialize, Deserialize)]
pub struct TypeScriptResult {
    pub compilation_success: bool,
    pub type_errors: Vec<String>,
}

/// ESLint analysis result
#[derive(Debug, Serialize, Deserialize)]
pub struct EslintResult {
    pub success: bool,
    pub issues: Vec<serde_json::Value>,
    pub fixes_applied: u32,
}

/// Claude AI analysis result
#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeResult {
    pub success: bool,
    pub fixed_content: Option<String>,
    pub issues_resolved: u32,
}

/// Semantic validation result
#[derive(Debug, Serialize, Deserialize)]
pub struct SemanticValidationResult {
    pub unresolved_warnings: Vec<String>,
}

/// Detect programming language from file extension and configuration defaults
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

fn detect_language_with_fallback(file_path: &str, config: &MoonShineConfig) -> String {
    if let Some(content_language) = detect_language_from_content(file_path) {
        return content_language;
    }

    if let Some(default_language) = config.default_language() {
        return default_language.to_string();
    }

    "typescript".to_string()
}

/// Content-based language detection using simple heuristics
fn detect_language_from_content(file_path: &str) -> Option<String> {
    // Read the file content if available; in WASM environments this may fail silently
    let content = std::fs::read_to_string(file_path).ok()?;
    let content_lower = content.to_lowercase();

    if content_lower.contains("#!/usr/bin/env python") || content_lower.contains("import ") && content_lower.contains("def ") {
        return Some("python".to_string());
    }

    if content_lower.contains("#!/bin/bash") || content_lower.contains("#!/bin/sh") {
        return Some("bash".to_string());
    }

    if content_lower.contains("use ") && content_lower.contains("fn ") && content_lower.contains("impl ") {
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

    if content_lower.contains("<?php") || content_lower.contains("$") && content_lower.contains("function") {
        return Some("php".to_string());
    }

    if content_lower.contains("interface ") || content_lower.contains("type ") || content_lower.contains(": ") {
        return Some("typescript".to_string());
    }

    if content_lower.contains("function ") || content_lower.contains("var ") || content_lower.contains("const ") {
        return Some("javascript".to_string());
    }

    None
}

/// Production: Get configurable Moon task name based on analysis type and language
fn resolve_moon_task_name(config: &MoonShineConfig, language: &str) -> String {
    config.resolve_task_name(language)
}

/// Analyze file using extension analysis with configuration
pub fn analyze_file_with_config(_content: &str, _language: &str, config: &MoonShineConfig) -> Result<AnalysisResults, Box<dyn std::error::Error>> {
    // Extension is purely for coordination - no analysis
    // All real analysis is done by the workflow phases via Moon tasks
    Ok(AnalysisResults {
        suggestions: Vec::new(),       // Populated by workflow phases
        semantic_warnings: Vec::new(), // Populated by workflow phases
        tsdoc_coverage: 0.0,           // Will be calculated by workflow
        quality_score: 0.0,            // Will be calculated by workflow
        parse_errors: Vec::new(),
        ignored_files: Vec::new(),
        ai_model: config.ai_model.clone().unwrap_or_else(|| "sonnet".to_string()),
    })
}

/// Main AI linting function with file content and config provided by Moon host
pub fn ai_lint_file_with_content(
    file_path: String,
    content: String,
    config: MoonShineConfig,
) -> std::result::Result<MoonShineResponse, Box<dyn std::error::Error>> {
    // Generate a unique Claude session ID for this file's processing.
    let claude_session_id = uuid::Uuid::new_v4().to_string();
    let start_time = std::time::Instant::now();

    // Step 1: Use file content provided by Moon host (already have it as parameter)

    // Step 2: Detect language from file extension
    let language = detect_language(&file_path, &config);

    // Step 3: Run initial analysis using extension config
    let analysis_results = analyze_file_with_config(&content, &language, &config)?;

    // Step 4: Create Moon task request with JSON protocol
    let task_request = MoonTaskRequest::new(
        file_path.clone(),
        language.clone(),
        content.clone(),
        analysis_results.clone(),
        claude_session_id.clone(),
    );

    // Step 5: Execute Moon tasks via JSON communication
    let task_response = execute_moon_tasks(&task_request, &config)?;

    // Step 6: Generate consolidated storage updates for Moon task persistence
    let training_updates = generate_training_updates_from_analysis(&analysis_results)?;
    let prompts_updates = generate_prompts_updates_for_persistence(&config)?;

    // Step 7: Aggregate results and apply COPRO optimizations if enabled
    let final_response = aggregate_analysis_results(
        analysis_results,
        task_response,
        start_time.elapsed().as_millis() as u64,
        training_updates,
        prompts_updates,
    );

    Ok(final_response)
}

/// Execute Moon tasks for comprehensive analysis (real implementation using Moon PDK)
fn execute_moon_tasks(request: &MoonTaskRequest, config: &MoonShineConfig) -> Result<MoonTaskResponse, Box<dyn std::error::Error>> {
    use crate::moon_pdk_interface::{execute_command, ExecCommandInput};

    // Construct the command to execute Moon tasks
    let command_input = ExecCommandInput {
        command: "moon".to_string(),
        args: vec![
            "run".to_string(),
            resolve_moon_task_name(config, &request.language),
            request.to_json()?, // Pass the request JSON as argument
        ],
        env: std::collections::HashMap::new(),
        working_dir: None,
    };

    // Execute the command via Moon host function
    let output = execute_command(command_input)?;

    if output.exit_code != 0 {
        return Err(format!("Moon task command failed with exit code {}: {}", output.exit_code, output.stderr).into());
    }

    // Parse the output from Moon task (expected to be JSON)
    let mut moon_response: MoonTaskResponse = serde_json::from_str(&output.stdout)?;
    moon_response.session_id = request.session_id.clone(); // Ensure session_id is propagated

    Ok(moon_response)
}

/// Generate training.json updates from analysis results for consolidated persistence
fn generate_training_updates_from_analysis(analysis: &AnalysisResults) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut patterns = serde_json::Map::new();

    // Extract patterns from suggestions
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

    // Return update object for training.json
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

/// Generate prompts.json updates for consolidated persistence
fn generate_prompts_updates_for_persistence(config: &MoonShineConfig) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut prompts = serde_json::Map::new();

    // Use any custom prompts from config as optimized versions
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

    // Return update object for prompts.json
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

/// Aggregate analysis results from extension and Moon tasks with consolidated persistence data
/// Production: This function should be refactored into smaller, focused functions
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
    let (patterns_learned, copro_optimizations) = calculate_learning_counts(&training_updates, &prompts_updates, &semantic_warnings);
    let total_issues = suggestions.len() + semantic_warnings.len();
    let overall_success = determine_overall_success(task_success, fix_outcome.issues_fixed, suggestions.len());
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

struct FixOutcome {
    issues_fixed: u32,
    fixed_content: Option<String>,
}

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

fn generate_pattern_insights(quality_score: f32, tsdoc_coverage: f32, results: &TaskResults) -> Option<Vec<String>> {
    let mut insights = vec![
        format!("Quality score: {:.2}", quality_score),
        format!("TSDoc coverage: {:.1}%", tsdoc_coverage * 100.0),
    ];

    if let Some(ts) = results.typescript.as_ref() {
        insights.push(format!("TypeScript compilation success: {}", ts.compilation_success));
    }

    if let Some(eslint) = results.eslint.as_ref() {
        insights.push(format!("ESLint pending issues: {}", eslint.issues.len()));
    }

    if let Some(validation) = results.semantic_validation.as_ref() {
        if !validation.unresolved_warnings.is_empty() {
            insights.push(format!("Semantic warnings outstanding: {}", validation.unresolved_warnings.len()));
        }
    }

    if insights.is_empty() {
        None
    } else {
        Some(insights)
    }
}

fn calculate_learning_counts(training_updates: &serde_json::Value, prompts_updates: &serde_json::Value, semantic_warnings: &[String]) -> (u32, u32) {
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

fn determine_overall_success(task_success: bool, issues_fixed: u32, suggestions_count: usize) -> bool {
    task_success && issues_fixed >= suggestions_count as u32
}

fn compose_analysis_message(total_issues: usize, issues_fixed: u32) -> String {
    format!("Analysis complete: {} issues found, {} issues flagged for fixing", total_issues, issues_fixed)
}

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
