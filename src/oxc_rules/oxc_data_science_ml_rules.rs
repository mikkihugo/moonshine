//! # OXC Data Science and ML Workflow Rules
//!
//! This module implements WASM-safe OXC rules for data science and machine learning
//! workflows in JavaScript/TypeScript environments, including TensorFlow.js, model
//! deployment, data pipeline optimization, and ML-specific security patterns.
//!
//! ## Rule Categories:
//! - **TensorFlow.js Patterns**: Model loading, tensor management, and performance optimization
//! - **Model Deployment**: Production deployment, versioning, and monitoring
//! - **Data Pipeline Security**: Data validation, privacy protection, and secure processing
//! - **Performance Optimization**: Memory management, GPU utilization, and batch processing
//! - **ML Operations (MLOps)**: Model lifecycle, experiment tracking, and reproducibility
//! - **Data Preprocessing**: Feature engineering, normalization, and validation
//! - **Model Serving**: API design, scaling, and edge deployment
//! - **Ethical AI**: Bias detection, fairness metrics, and responsible AI practices
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
// TensorFlow.js and Model Management Rules
// ================================================================================================

/// Requires proper tensor disposal to prevent memory leaks
pub struct RequireTensorDisposal;

impl RequireTensorDisposal {
    pub const NAME: &'static str = "require-tensor-disposal";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireTensorDisposal {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("tf.tensor") || code.contains("tf.variable")) &&
           !code.contains("dispose()") && !code.contains("tf.tidy") {
            diagnostics.push(create_tensor_disposal_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireTensorDisposal {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use tf.tidy() for automatic cleanup or call .dispose() on tensors to prevent GPU memory leaks in TensorFlow.js applications".to_string(),
            confidence: 0.96,
            auto_fixable: true,
        }).collect()
    }
}

/// Enforces proper model loading with error handling
pub struct RequireModelErrorHandling;

impl RequireModelErrorHandling {
    pub const NAME: &'static str = "require-model-error-handling";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireModelErrorHandling {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("tf.loadLayersModel") || code.contains("tf.loadGraphModel")) &&
           code.contains("await") && !code.contains("catch") && !code.contains("try") {
            diagnostics.push(create_model_error_handling_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireModelErrorHandling {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Add try-catch blocks around model loading operations to handle network failures, corrupt models, and compatibility issues gracefully".to_string(),
            confidence: 0.94,
            auto_fixable: true,
        }).collect()
    }
}

/// Requires model versioning for reproducibility
pub struct RequireModelVersioning;

impl RequireModelVersioning {
    pub const NAME: &'static str = "require-model-versioning";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireModelVersioning {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("loadModel") && !code.contains("version") &&
           !code.contains("/v1/") && !code.contains("/v2/") {
            diagnostics.push(create_model_versioning_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireModelVersioning {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Include model version in the URL or metadata to ensure reproducible results and enable safe model updates in production".to_string(),
            confidence: 0.88,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// Data Pipeline Security and Validation Rules
// ================================================================================================

/// Requires input validation for ML data pipelines
pub struct RequireDataValidation;

impl RequireDataValidation {
    pub const NAME: &'static str = "require-data-validation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireDataValidation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("predict") || code.contains("inference")) &&
           code.contains("data") && !code.contains("validate") &&
           !code.contains("schema") && !code.contains("sanitize") {
            diagnostics.push(create_data_validation_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireDataValidation {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Validate input data shape, type, and range before feeding to ML models to prevent adversarial attacks and ensure model stability".to_string(),
            confidence: 0.95,
            auto_fixable: false,
        }).collect()
    }
}

/// Prevents sensitive data exposure in ML logging
pub struct NoSensitiveDataInLogs;

impl NoSensitiveDataInLogs {
    pub const NAME: &'static str = "no-sensitive-data-in-logs";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::FixDangerous;
}

impl WasmRule for NoSensitiveDataInLogs {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        let sensitive_keywords = ["personal", "private", "email", "phone", "address", "ssn", "pii"];
        let logging_functions = ["console.log", "logger.info", "console.error"];

        if sensitive_keywords.iter().any(|&keyword| code.contains(keyword)) &&
           logging_functions.iter().any(|&log_fn| code.contains(log_fn)) {
            diagnostics.push(create_sensitive_data_logs_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoSensitiveDataInLogs {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Remove sensitive data from logs or implement data masking/anonymization before logging to comply with privacy regulations".to_string(),
            confidence: 0.98,
            auto_fixable: false,
        }).collect()
    }
}

/// Requires data anonymization for privacy protection
pub struct RequireDataAnonymization;

impl RequireDataAnonymization {
    pub const NAME: &'static str = "require-data-anonymization";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireDataAnonymization {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("personalData") && code.contains("training") &&
           !code.contains("anonymize") && !code.contains("mask") && !code.contains("hash") {
            diagnostics.push(create_data_anonymization_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireDataAnonymization {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Implement data anonymization techniques (k-anonymity, differential privacy) when using personal data for ML training".to_string(),
            confidence: 0.91,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// Performance Optimization Rules
// ================================================================================================

/// Requires batch processing for efficient inference
pub struct RequireBatchProcessing;

impl RequireBatchProcessing {
    pub const NAME: &'static str = "require-batch-processing";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireBatchProcessing {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("for") && code.contains("predict") &&
           !code.contains("batch") && !code.contains("tf.stack") {
            diagnostics.push(create_batch_processing_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireBatchProcessing {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use batch processing with tf.stack() or model.predictOnBatch() instead of individual predictions for better GPU utilization and performance".to_string(),
            confidence: 0.89,
            auto_fixable: true,
        }).collect()
    }
}

/// Enforces WebGL backend for GPU acceleration
pub struct RequireWebGLBackend;

impl RequireWebGLBackend {
    pub const NAME: &'static str = "require-webgl-backend";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireWebGLBackend {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("tf.setBackend") && code.contains("'cpu'") &&
           !code.contains("fallback") {
            diagnostics.push(create_webgl_backend_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireWebGLBackend {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Use 'webgl' backend for GPU acceleration in browsers, with 'cpu' as fallback for compatibility when WebGL is unavailable".to_string(),
            confidence: 0.87,
            auto_fixable: true,
        }).collect()
    }
}

/// Prevents memory leaks in data preprocessing
pub struct NoMemoryLeaksInPreprocessing;

impl NoMemoryLeaksInPreprocessing {
    pub const NAME: &'static str = "no-memory-leaks-in-preprocessing";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for NoMemoryLeaksInPreprocessing {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("tf.image.") || code.contains("tf.data.")) &&
           code.contains("map(") && !code.contains("dispose") {
            diagnostics.push(create_preprocessing_memory_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoMemoryLeaksInPreprocessing {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Dispose intermediate tensors in data preprocessing pipelines to prevent memory accumulation during batch processing".to_string(),
            confidence: 0.92,
            auto_fixable: true,
        }).collect()
    }
}

// ================================================================================================
// MLOps and Model Lifecycle Rules
// ================================================================================================

/// Requires experiment tracking for reproducibility
pub struct RequireExperimentTracking;

impl RequireExperimentTracking {
    pub const NAME: &'static str = "require-experiment-tracking";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireExperimentTracking {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if (code.contains("model.fit") || code.contains("train")) &&
           !code.contains("experiment") && !code.contains("metadata") &&
           !code.contains("tracking") {
            diagnostics.push(create_experiment_tracking_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireExperimentTracking {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Add experiment tracking with metadata (hyperparameters, metrics, timestamps) for reproducibility and model comparison".to_string(),
            confidence: 0.85,
            auto_fixable: false,
        }).collect()
    }
}

/// Enforces model performance monitoring
pub struct RequireModelMonitoring;

impl RequireModelMonitoring {
    pub const NAME: &'static str = "require-model-monitoring";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireModelMonitoring {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        if code.contains("production") && code.contains("predict") &&
           !code.contains("metrics") && !code.contains("monitor") &&
           !code.contains("performance") {
            diagnostics.push(create_model_monitoring_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireModelMonitoring {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Implement model performance monitoring with accuracy metrics, drift detection, and alerting for production ML systems".to_string(),
            confidence: 0.90,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// Ethical AI and Bias Detection Rules
// ================================================================================================

/// Requires bias testing for fairness validation
pub struct RequireBiasTesting;

impl RequireBiasTesting {
    pub const NAME: &'static str = "require-bias-testing";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireBiasTesting {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        let protected_attributes = ["gender", "race", "age", "ethnicity", "religion"];

        if protected_attributes.iter().any(|&attr| code.contains(attr)) &&
           code.contains("prediction") && !code.contains("fairness") &&
           !code.contains("bias") && !code.contains("equity") {
            diagnostics.push(create_bias_testing_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireBiasTesting {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Implement bias testing and fairness metrics when using protected attributes to ensure equitable ML model outcomes".to_string(),
            confidence: 0.93,
            auto_fixable: false,
        }).collect()
    }
}

/// Prevents discriminatory feature usage
pub struct NoDiscriminatoryFeatures;

impl NoDiscriminatoryFeatures {
    pub const NAME: &'static str = "no-discriminatory-features";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Restriction;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::FixDangerous;
}

impl WasmRule for NoDiscriminatoryFeatures {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        let discriminatory_features = ["race", "gender", "religion", "sexual_orientation"];

        if discriminatory_features.iter().any(|&feature| code.contains(feature)) &&
           code.contains("feature") && code.contains("training") {
            diagnostics.push(create_discriminatory_features_diagnostic(1, 0));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for NoDiscriminatoryFeatures {
    fn ai_enhance(&self, _code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|d| AiSuggestion {
            rule_name: d.rule_name.clone(),
            suggestion: "Remove protected attributes as direct features and consider fair ML techniques like adversarial debiasing or fairness constraints".to_string(),
            confidence: 0.95,
            auto_fixable: false,
        }).collect()
    }
}

// ================================================================================================
// Diagnostic Creation Functions
// ================================================================================================

fn create_tensor_disposal_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireTensorDisposal::NAME.to_string(),
        message: "Tensors must be properly disposed to prevent GPU memory leaks".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Use tf.tidy() or call .dispose() on tensors".to_string()),
    }
}

fn create_model_error_handling_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireModelErrorHandling::NAME.to_string(),
        message: "Model loading operations must include error handling".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add try-catch blocks around model loading".to_string()),
    }
}

fn create_model_versioning_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireModelVersioning::NAME.to_string(),
        message: "Models should include version information for reproducibility".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Include version in model URL or metadata".to_string()),
    }
}

fn create_data_validation_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireDataValidation::NAME.to_string(),
        message: "Input data must be validated before ML inference".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add data validation and sanitization".to_string()),
    }
}

fn create_sensitive_data_logs_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoSensitiveDataInLogs::NAME.to_string(),
        message: "Sensitive data should not be logged in ML pipelines".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Remove sensitive data from logs or implement masking".to_string()),
    }
}

fn create_data_anonymization_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireDataAnonymization::NAME.to_string(),
        message: "Personal data used in ML training should be anonymized".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Implement data anonymization techniques".to_string()),
    }
}

fn create_batch_processing_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireBatchProcessing::NAME.to_string(),
        message: "Use batch processing for efficient ML inference".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Replace individual predictions with batch processing".to_string()),
    }
}

fn create_webgl_backend_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireWebGLBackend::NAME.to_string(),
        message: "Consider using WebGL backend for GPU acceleration".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Use 'webgl' backend with 'cpu' fallback".to_string()),
    }
}

fn create_preprocessing_memory_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoMemoryLeaksInPreprocessing::NAME.to_string(),
        message: "Dispose intermediate tensors in preprocessing to prevent memory leaks".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Add tensor disposal in preprocessing pipeline".to_string()),
    }
}

fn create_experiment_tracking_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireExperimentTracking::NAME.to_string(),
        message: "ML experiments should include tracking for reproducibility".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Add experiment metadata and tracking".to_string()),
    }
}

fn create_model_monitoring_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireModelMonitoring::NAME.to_string(),
        message: "Production ML models should include performance monitoring".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Implement model performance metrics and monitoring".to_string()),
    }
}

fn create_bias_testing_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireBiasTesting::NAME.to_string(),
        message: "Models using protected attributes should include bias testing".to_string(),
        line,
        column,
        severity: "warning".to_string(),
        fix_suggestion: Some("Implement fairness metrics and bias testing".to_string()),
    }
}

fn create_discriminatory_features_diagnostic(line: usize, column: usize) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: NoDiscriminatoryFeatures::NAME.to_string(),
        message: "Avoid using protected attributes as direct features in ML models".to_string(),
        line,
        column,
        severity: "error".to_string(),
        fix_suggestion: Some("Remove protected attributes and use fair ML techniques".to_string()),
    }
}

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_disposal_detection() {
        let code = r#"const tensor = tf.tensor([1, 2, 3, 4]);"#;
        let rule = RequireTensorDisposal;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireTensorDisposal::NAME);
    }

    #[test]
    fn test_model_error_handling_detection() {
        let code = r#"const model = await tf.loadLayersModel('/path/to/model');"#;
        let rule = RequireModelErrorHandling;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireModelErrorHandling::NAME);
    }

    #[test]
    fn test_data_validation_detection() {
        let code = r#"const prediction = model.predict(data);"#;
        let rule = RequireDataValidation;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireDataValidation::NAME);
    }

    #[test]
    fn test_sensitive_data_logs_detection() {
        let code = r#"console.log('User personal data:', userData);"#;
        let rule = NoSensitiveDataInLogs;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoSensitiveDataInLogs::NAME);
    }

    #[test]
    fn test_batch_processing_detection() {
        let code = r#"for (const item of items) { model.predict(item); }"#;
        let rule = RequireBatchProcessing;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, RequireBatchProcessing::NAME);
    }

    #[test]
    fn test_discriminatory_features_detection() {
        let code = r#"const features = { age: 25, race: 'asian', income: 50000 }; trainModel(features);"#;
        let rule = NoDiscriminatoryFeatures;
        let diagnostics = rule.run(code);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_name, NoDiscriminatoryFeatures::NAME);
    }

    #[test]
    fn test_ai_enhancement_suggestions() {
        let code = r#"const tensor = tf.tensor([1, 2, 3]);"#;
        let rule = RequireTensorDisposal;
        let diagnostics = rule.run(code);
        let suggestions = rule.ai_enhance(code, &diagnostics);

        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].confidence > 0.9);
        assert!(suggestions[0].auto_fixable);
    }
}