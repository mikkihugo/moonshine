//! # Error Handling: Production-Grade Error Management for Moon Shine
//!
//! This module provides a comprehensive and structured approach to error handling within the
//! `moon-shine` extension. It defines a rich `Error` enum that covers various failure scenarios,
//! including issues with AI CLI execution, configuration, I/O operations, WASM runtime,
//! and Moon PDK interactions.
//!
//! The module emphasizes:
//! - **Structured Error Types**: Categorized errors with detailed context.
//! - **Source Chaining**: Preservation of underlying error causes for better debugging.
//! - **Recoverability**: Clear indication of whether an error allows for graceful degradation.
//! - **Severity Levels**: Classification of errors for logging, monitoring, and alerting.
//! - **User-Friendly Messages**: Conversion of technical errors into actionable messages for end-users.
//!
//! @category error-handling
//! @safe program
//! @mvp core
//! @complexity high
//! @since 1.0.0

use crate::moon_host::PluginError;
use thiserror::Error;

/// Represents all possible errors that can occur within the `moon-shine` extension.
///
/// This enum provides a detailed classification of errors, allowing for precise
/// handling, logging, and user feedback. Each variant encapsulates specific context
/// relevant to the error's origin.
///
/// @category error-enum
// @safe program
// @mvp core
// @complexity high
// @since 1.0.0
#[derive(Error, Debug)]
pub enum Error {
    /// Error specifically related to Claude CLI execution.
    #[error("Claude CLI execution failed: {message}")]
    ClaudeCli {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Error during AI provider execution (general AI interaction).
    #[error("AI provider '{provider}' execution failed: {message}")]
    AIExecution {
        provider: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Error during JSON serialization or deserialization.
    #[error("JSON serialization/deserialization failed")]
    Serialization {
        #[from]
        source: serde_json::Error,
    },

    /// Error during file system I/O operations.
    #[error("File system operation failed: {path}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },

    /// Error from standard library error types.
    #[error("Standard library error: {message}")]
    StdError {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Error originating from the Moon host runtime (via Extism PDK bindings).
    #[error("Moon host operation failed")]
    MoonHost {
        #[from]
        source: PluginError,
    },

    /// Configuration-related error.
    #[error("Configuration error: {message}")]
    Config {
        message: String,
        field: Option<String>,
        value: Option<String>,
    },

    /// Error during code analysis operations.
    #[error("Code analysis failed: {operation}")]
    Analysis {
        operation: String,
        file_path: Option<String>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// General WASM runtime error.
    #[error("WASM runtime error: {operation}")]
    Wasm { operation: String, context: Option<String> },

    /// Error during Moon PDK host function calls.
    #[error("Moon PDK host function call failed: {function}")]
    MoonPdk {
        function: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Error during pattern detection.
    #[error("Pattern detection failed: {pattern_type}")]
    PatternDetection {
        pattern_type: String,
        confidence: Option<f32>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Error during COPRO optimization.
    #[error("COPRO optimization failed: {stage}")]
    CoproOptimization {
        stage: String,
        iteration: Option<u32>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Task execution timed out.
    #[error("Task execution timeout: {task_name}")]
    Timeout { task_name: String, duration_ms: u64 },

    /// Validation error (e.g., invalid input data).
    #[error("Validation failed: {field}")]
    Validation { field: String, expected: String, actual: String },

    /// Container for multiple errors, typically from batch operations.
    #[error("Multiple errors occurred during batch processing")]
    Multiple { errors: Vec<Error>, successful_count: usize },

    /// Error during data access operations (e.g., reading/writing data stores).
    #[error("Data access error: {message}")]
    DataAccess { message: String, path: Option<String> },

    /// Error during data processing operations.
    #[error("Data processing error: {message}")]
    Processing { message: String },

    /// Error during workflow execution.
    #[error("Workflow execution error: {message}")]
    WorkflowError { message: String },

    /// Error during storage operations.
    #[error("Storage error: {message}")]
    Storage { message: String },

    /// Error during data parsing.
    #[error("Data parsing error: {message}")]
    DataParsing { message: String, line_number: Option<usize> },

    /// Error during data serialization.
    #[error("Data serialization error: {message}")]
    DataSerialization { message: String },

    /// Error during data validation.
    #[error("Data validation error: {message}")]
    DataValidation { message: String },
}

impl Error {
    /// Creates a `ClaudeCli` error with a descriptive message.
    ///
    /// @param message A string slice or type convertible to `String` describing the error.
    /// @returns A new `Error::ClaudeCli` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn claude_cli(message: impl Into<String>) -> Self {
        Self::ClaudeCli {
            message: message.into(),
            source: None,
        }
    }

    /// Creates an `AIExecution` error with a descriptive message and the AI provider.
    ///
    /// @param provider The `AIProviderConfig` associated with the failed execution.
    /// @param message A string slice or type convertible to `String` describing the error.
    /// @returns A new `Error::AIExecution` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn ai_execution(provider: &crate::provider_router::AIProviderConfig, message: impl Into<String>) -> Self {
        Self::AIExecution {
            provider: provider.name().to_string(),
            message: message.into(),
            source: None,
        }
    }

    /// Creates an `AIExecution` error with a descriptive message and a source error.
    ///
    /// @param provider The `AIProviderConfig` associated with the failed execution.
    /// @param message A string slice or type convertible to `String` describing the error.
    /// @param source The underlying error that caused this `AIExecution` error.
    /// @returns A new `Error::AIExecution` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn ai_execution_with_source(
        provider: &crate::provider_router::AIProviderConfig,
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::AIExecution {
            provider: provider.name().to_string(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a `ClaudeCli` error with a descriptive message and a source error.
    ///
    /// @param message A string slice or type convertible to `String` describing the error.
    /// @param source The underlying error that caused this `ClaudeCli` error.
    /// @returns A new `Error::ClaudeCli` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn claude_cli_with_source(message: impl Into<String>, source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::ClaudeCli {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a `Config` error with a descriptive message.
    ///
    /// @param message A string slice or type convertible to `String` describing the error.
    /// @returns A new `Error::Config` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
            field: None,
            value: None,
        }
    }

    /// Creates a `Config` error with a descriptive message and specific field context.
    ///
    /// @param message A string slice or type convertible to `String` describing the error.
    /// @param field The name of the configuration field that caused the error.
    /// @param value An optional string representation of the invalid value.
    /// @returns A new `Error::Config` instance with field context.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn config_field(message: impl Into<String>, field: impl Into<String>, value: Option<impl Into<String>>) -> Self {
        Self::Config {
            message: message.into(),
            field: Some(field.into()),
            value: value.map(Into::into),
        }
    }

    /// Creates an `Analysis` error with a descriptive operation.
    ///
    /// @param operation A string slice or type convertible to `String` describing the analysis operation that failed.
    /// @returns A new `Error::Analysis` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn analysis(operation: impl Into<String>) -> Self {
        Self::Analysis {
            operation: operation.into(),
            file_path: None,
            source: None,
        }
    }

    /// Creates an `Analysis` error with a descriptive operation and file path context.
    ///
    /// @param operation A string slice or type convertible to `String` describing the analysis operation that failed.
    /// @param file_path The path to the file that was being analyzed when the error occurred.
    /// @returns A new `Error::Analysis` instance with file path context.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn analysis_file(operation: impl Into<String>, file_path: impl Into<String>) -> Self {
        Self::Analysis {
            operation: operation.into(),
            file_path: Some(file_path.into()),
            source: None,
        }
    }

    /// Creates an `Analysis` error with a descriptive operation and a source error.
    ///
    /// @param operation A string slice or type convertible to `String` describing the analysis operation that failed.
    /// @param source The underlying error that caused this `Analysis` error.
    /// @returns A new `Error::Analysis` instance with source error.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn analysis_with_source(operation: impl Into<String>, source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Analysis {
            operation: operation.into(),
            file_path: None,
            source: Some(Box::new(source)),
        }
    }

    /// Creates a `Wasm` error related to an extension operation.
    ///
    /// @param operation A string slice or type convertible to `String` describing the WASM operation that failed.
    /// @returns A new `Error::Wasm` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn extension_operation(operation: impl Into<String>) -> Self {
        Self::Wasm {
            operation: operation.into(),
            context: None,
        }
    }

    /// Creates a `Wasm` error related to an extension operation with additional context.
    ///
    /// @param operation A string slice or type convertible to `String` describing the WASM operation that failed.
    /// @param context Additional context about the WASM error.
    /// @returns A new `Error::Wasm` instance with context.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn extension_operation_with_context(operation: impl Into<String>, context: impl Into<String>) -> Self {
        Self::Wasm {
            operation: operation.into(),
            context: Some(context.into()),
        }
    }

    /// Creates a `MoonPdk` error related to a Moon PDK host function call.
    ///
    /// @param function A string slice or type convertible to `String` describing the Moon PDK function that failed.
    /// @returns A new `Error::MoonPdk` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn moon_pdk(function: impl Into<String>) -> Self {
        Self::MoonPdk {
            function: function.into(),
            source: None,
        }
    }

    /// Creates a `MoonPdk` error with a descriptive function and a source error.
    ///
    /// @param function A string slice or type convertible to `String` describing the Moon PDK function that failed.
    /// @param source The underlying error that caused this `MoonPdk` error.
    /// @returns A new `Error::MoonPdk` instance with source error.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn moon_pdk_with_source(function: impl Into<String>, source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::MoonPdk {
            function: function.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a `PatternDetection` error with a descriptive pattern type and optional confidence.
    ///
    /// @param pattern_type A string slice or type convertible to `String` describing the type of pattern detection that failed.
    /// @param confidence An optional confidence score related to the pattern detection.
    /// @returns A new `Error::PatternDetection` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn pattern_detection(pattern_type: impl Into<String>, confidence: Option<f32>) -> Self {
        Self::PatternDetection {
            pattern_type: pattern_type.into(),
            confidence,
            source: None,
        }
    }

    /// Creates a `CoproOptimization` error with a descriptive stage and optional iteration.
    ///
    /// @param stage A string slice or type convertible to `String` describing the COPRO optimization stage that failed.
    /// @param iteration An optional iteration number during which the error occurred.
    /// @returns A new `Error::CoproOptimization` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn copro_optimization(stage: impl Into<String>, iteration: Option<u32>) -> Self {
        Self::CoproOptimization {
            stage: stage.into(),
            iteration,
            source: None,
        }
    }

    /// Creates a `Timeout` error for a task that exceeded its execution time limit.
    ///
    /// @param task_name A string slice or type convertible to `String` describing the task that timed out.
    /// @param duration_ms The duration in milliseconds after which the task timed out.
    /// @returns A new `Error::Timeout` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn timeout(task_name: impl Into<String>, duration_ms: u64) -> Self {
        Self::Timeout {
            task_name: task_name.into(),
            duration_ms,
        }
    }

    /// Creates a `Validation` error for invalid input or data.
    ///
    /// @param field The name of the field that failed validation.
    /// @param expected A string slice or type convertible to `String` describing the expected value or format.
    /// @param actual A string slice or type convertible to `String` describing the actual value received.
    /// @returns A new `Error::Validation` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn validation(field: impl Into<String>, expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    /// Creates a `Multiple` error to encapsulate a collection of errors.
    ///
    /// This is useful for operations that can partially succeed, where multiple individual
    /// errors might occur but the overall operation is not a complete failure.
    ///
    /// @param errors A `Vec<Error>` containing the individual errors.
    /// @param successful_count The number of successful operations in the batch.
    /// @returns A new `Error::Multiple` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn multiple(errors: Vec<Error>, successful_count: usize) -> Self {
        Self::Multiple { errors, successful_count }
    }

    /// Creates a `DataAccess` error for issues during data retrieval or storage.
    ///
    /// @param message A string slice or type convertible to `String` describing the data access error.
    /// @returns A new `Error::DataAccess` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn data_access(message: impl Into<String>) -> Self {
        Self::DataAccess {
            message: message.into(),
            path: None,
        }
    }

    /// Creates a `Processing` error for data processing operations.
    ///
    /// @param message A string slice or type convertible to `String` describing the processing error.
    /// @returns A new `Error::Processing` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn processing(message: impl Into<String>) -> Self {
        Self::Processing { message: message.into() }
    }

    /// Creates a `Storage` error for storage operations.
    ///
    /// @param message A string slice or type convertible to `String` describing the storage error.
    /// @returns A new `Error::Storage` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn storage(message: impl Into<String>) -> Self {
        Self::Storage { message: message.into() }
    }

    /// Creates a `Wasm` error for general WASM runtime issues.
    ///
    /// @param operation A string slice or type convertible to `String` describing the WASM operation that failed.
    /// @returns A new `Error::Wasm` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn wasm(operation: impl Into<String>) -> Self {
        Self::Wasm {
            operation: operation.into(),
            context: None,
        }
    }

    /// Creates a `DataParsing` error for issues during data parsing.
    ///
    /// @param message A string slice or type convertible to `String` describing the data parsing error.
    /// @returns A new `Error::DataParsing` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn data_parsing(message: impl Into<String>) -> Self {
        Self::DataParsing {
            message: message.into(),
            line_number: None,
        }
    }

    /// Creates a `DataSerialization` error for issues during data serialization.
    ///
    /// @param message A string slice or type convertible to `String` describing the data serialization error.
    /// @returns A new `Error::DataSerialization` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn data_serialization(message: impl Into<String>) -> Self {
        Self::DataSerialization { message: message.into() }
    }

    /// Creates a `DataValidation` error for issues during data validation.
    ///
    /// @param message A string slice or type convertible to `String` describing the data validation error.
    /// @returns A new `Error::DataValidation` instance.
    ///
    /// @category constructor
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    #[must_use]
    pub fn data_validation(message: impl Into<String>) -> Self {
        Self::DataValidation { message: message.into() }
    }

    /// Determines if the error is recoverable, allowing for graceful degradation or retry.
    ///
    /// Recoverable errors typically indicate transient issues or situations where the system
    /// can continue operation, possibly with reduced functionality or by attempting alternative strategies.
    ///
    /// @returns `true` if the error is recoverable, `false` otherwise.
    ///
    /// @category utility
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::ClaudeCli { .. } => true,          // Can fallback to other tools
            Self::AIExecution { .. } => true,        // Can fallback to other AI providers or tools
            Self::Serialization { .. } => false,     // Data corruption
            Self::Io { .. } => true,                 // Can retry or skip file
            Self::MoonHost { .. } => false,          // WASM runtime issue
            Self::Config { .. } => false,            // Invalid configuration
            Self::Analysis { .. } => true,           // Can skip problematic files
            Self::Wasm { .. } => false,              // Runtime issue
            Self::MoonPdk { .. } => true,            // Can fallback to local operations
            Self::PatternDetection { .. } => true,   // Can disable pattern detection
            Self::CoproOptimization { .. } => true,  // Can disable optimization
            Self::Timeout { .. } => true,            // Can retry with longer timeout
            Self::Validation { .. } => false,        // Invalid input
            Self::Multiple { .. } => true,           // Partial success is possible
            Self::DataAccess { .. } => true,         // Can retry or fallback
            Self::DataParsing { .. } => false,       // Invalid data format
            Self::DataSerialization { .. } => false, // Data corruption
            Self::DataValidation { .. } => false,    // Invalid data content
            Self::StdError { .. } => true,           // Can retry or fallback
            Self::Processing { .. } => true,         // Can retry or fallback
            Self::Storage { .. } => true,            // Can retry or fallback
            Self::WorkflowError { .. } => true,      // Can retry workflow steps
        }
    }

    /// Returns the severity level of the error.
    ///
    /// This is used for logging, monitoring, and determining the impact of the error.
    ///
    /// @returns The `ErrorSeverity` of the error.
    ///
    /// @category utility
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::ClaudeCli { .. } => ErrorSeverity::Warning,
            Self::AIExecution { .. } => ErrorSeverity::Warning,
            Self::Serialization { .. } => ErrorSeverity::Error,
            Self::Io { .. } => ErrorSeverity::Warning,
            Self::MoonHost { .. } => ErrorSeverity::Critical,
            Self::Config { .. } => ErrorSeverity::Error,
            Self::Analysis { .. } => ErrorSeverity::Info,
            Self::Wasm { .. } => ErrorSeverity::Critical,
            Self::MoonPdk { .. } => ErrorSeverity::Warning,
            Self::PatternDetection { .. } => ErrorSeverity::Info,
            Self::CoproOptimization { .. } => ErrorSeverity::Info,
            Self::Timeout { .. } => ErrorSeverity::Warning,
            Self::Validation { .. } => ErrorSeverity::Error,
            Self::Multiple { .. } => ErrorSeverity::Warning,
            Self::DataAccess { .. } => ErrorSeverity::Warning,
            Self::DataParsing { .. } => ErrorSeverity::Error,
            Self::DataSerialization { .. } => ErrorSeverity::Error,
            Self::DataValidation { .. } => ErrorSeverity::Error,
            Self::StdError { .. } => ErrorSeverity::Warning,
            Self::Processing { .. } => ErrorSeverity::Warning,
            Self::Storage { .. } => ErrorSeverity::Error,
            Self::WorkflowError { .. } => ErrorSeverity::Warning,
        }
    }

    /// Returns the category of the error.
    ///
    /// This is used for metrics, filtering, and understanding the domain of the error.
    ///
    /// @returns A static string slice representing the error category.
    ///
    /// @category utility
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn category(&self) -> &'static str {
        match self {
            Self::ClaudeCli { .. } => "claude_cli",
            Self::AIExecution { .. } => "ai_execution",
            Self::Serialization { .. } => "serialization",
            Self::Io { .. } => "io",
            Self::MoonHost { .. } => "moon_host",
            Self::Config { .. } => "config",
            Self::Analysis { .. } => "analysis",
            Self::Wasm { .. } => "wasm",
            Self::MoonPdk { .. } => "moon_pdk",
            Self::PatternDetection { .. } => "pattern_detection",
            Self::CoproOptimization { .. } => "copro_optimization",
            Self::Timeout { .. } => "timeout",
            Self::Validation { .. } => "validation",
            Self::Multiple { .. } => "multiple",
            Self::DataAccess { .. } => "data_access",
            Self::DataParsing { .. } => "data_parsing",
            Self::DataSerialization { .. } => "data_serialization",
            Self::DataValidation { .. } => "data_validation",
            Self::StdError { .. } => "std_error",
            Self::Processing { .. } => "processing",
            Self::Storage { .. } => "storage",
            Self::WorkflowError { .. } => "workflow",
        }
    }

    /// Converts the error into a user-friendly message for display.
    ///
    /// This message is intended to be presented to the end-user, providing actionable
    /// information without exposing internal technical details.
    ///
    /// @returns A `String` containing the user-friendly error message.
    ///
    /// @category utility
    /// @safe team
    /// @mvp core
    /// @complexity low
    /// @since 1.0.0
    pub fn user_message(&self) -> String {
        match self {
            Self::ClaudeCli { message, .. } => {
                format!("AI analysis failed: {}. Falling back to static analysis.", message)
            }
            Self::AIExecution { provider, message, .. } => {
                format!("AI provider '{}' failed: {}. Falling back to static analysis.", provider, message)
            }
            Self::Processing { message, .. } => {
                format!("Processing error: {}. Please retry or check logs for details.", message)
            }
            Self::Storage { message, .. } => {
                format!("Storage error: {}. Please check storage configuration or retry.", message)
            }
            Self::Config { message, field, .. } => {
                if let Some(field) = field {
                    format!("Configuration error in '{}': {}", field, message)
                } else {
                    format!("Configuration error: {}", message)
                }
            }
            Self::Analysis { operation, file_path, .. } => {
                if let Some(file_path) = file_path {
                    format!("Analysis error during '{}' on '{}': Please check file syntax", operation, file_path)
                } else {
                    format!("Analysis error during '{}': Please check input", operation)
                }
            }
            Self::Serialization { .. } => "Data processing error: Please check input format".to_string(),
            Self::Io { path, .. } => format!("File operation failed on '{}': Please check file permissions", path),
            Self::MoonHost { .. } => "Extension runtime error: Please restart the operation".to_string(),
            Self::Wasm { operation, .. } => {
                format!("Runtime error during '{}': Please try again", operation)
            }
            Self::MoonPdk { function, .. } => format!("Moon operation '{}' failed: Please check Moon configuration", function),
            Self::PatternDetection { pattern_type, .. } => format!("Pattern detection failed for '{}': Continuing with basic analysis", pattern_type),
            Self::CoproOptimization { stage, .. } => format!("Optimization failed at '{}': Using default configuration", stage),
            Self::Timeout { task_name, .. } => format!("Operation '{}' timed out: Please try with simpler input", task_name),
            Self::Validation { field, expected, .. } => format!("Invalid '{}': Expected {}", field, expected),
            Self::Multiple { errors, successful_count } => {
                format!("Partial completion: {} operations succeeded, {} failed", successful_count, errors.len())
            }
            Self::DataAccess { message, .. } => {
                format!("Data access error: {}", message)
            }
            Self::DataParsing { message, .. } => {
                format!("Data parsing error: {}", message)
            }
            Self::DataSerialization { message } => {
                format!("Data serialization error: {}", message)
            }
            Self::DataValidation { message } => {
                format!("Data validation error: {}", message)
            }
            Self::StdError { message, .. } => {
                format!("Standard library error: {}", message)
            }
            Self::WorkflowError { message } => {
                format!("Workflow execution error: {}", message)
            }
        }
    }
}
/// Implementation of From traits for common error types
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io {
            path: "unknown".to_string(),
            source: error,
        }
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::StdError {
            message: error.to_string(),
            source: error,
        }
    }
}

/// Result type alias for moon-shine operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error severity levels for logging and monitoring
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Informational level for debugging
    Info,
    /// Warning level for non-critical issues
    Warning,
    /// Error level for operation failures
    Error,
    /// Critical level for system-wide failures
    Critical,
}
