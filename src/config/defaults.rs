use std::path::PathBuf;

// AI defaults
pub fn ai_model() -> String {
    "sonnet".into()
}

pub fn ai_temperature() -> f32 {
    0.7
}

pub fn ai_providers() -> Vec<String> {
    vec!["claude".into(), "codex".into()]
}

pub fn ai_max_concurrent_requests() -> u32 {
    3
}

pub fn ai_batch_size() -> u32 {
    5
}

// Linting defaults
pub fn lint_include_patterns() -> Vec<String> {
    vec!["**/*.{ts,tsx,js,jsx}".into()]
}

pub fn lint_exclude_patterns() -> Vec<String> {
    vec!["node_modules/**".into(), "dist/**".into(), "build/**".into()]
}

pub fn lint_max_suggestions() -> u32 {
    50
}

// Adaptive defaults
pub fn adaptive_enabled() -> bool {
    true
}

pub fn pattern_min_frequency() -> u32 {
    10
}

pub fn pattern_max_age_days() -> u32 {
    30
}

pub fn pattern_similarity_threshold() -> f32 {
    0.7
}

pub fn rule_min_cluster_size() -> usize {
    5
}

pub fn rule_max_per_cluster() -> usize {
    3
}

pub fn rule_quality_threshold() -> f64 {
    0.85
}

pub fn rule_auto_activate() -> bool {
    false
}

pub fn rule_max_training_examples() -> usize {
    1000
}

pub fn starcoder_enabled() -> bool {
    true
}

pub fn starcoder_training_threshold() -> usize {
    10
}

pub fn starcoder_train_on_good() -> bool {
    true
}

pub fn starcoder_train_on_bad() -> bool {
    true
}

pub fn starcoder_model_path() -> Option<String> {
    Some(PathBuf::from("models/starcoder-1b").to_string_lossy().to_string())
}

pub fn default_operation_mode() -> Option<String> {
    Some("fix".to_string())
}

pub fn default_copro_temperature() -> Option<f32> {
    Some(1.0)
}

pub fn default_confidence_threshold() -> Option<f64> {
    Some(0.8)
}

pub fn default_workflow_timeout() -> Option<u64> {
    Some(300)
}

pub fn default_ai_model_option() -> Option<String> {
    Some(ai_model())
}

pub fn default_temperature_option() -> Option<f32> {
    Some(ai_temperature())
}

pub fn default_include_patterns_option() -> Option<Vec<String>> {
    Some(lint_include_patterns())
}

pub fn default_exclude_patterns_option() -> Option<Vec<String>> {
    Some(lint_exclude_patterns())
}
