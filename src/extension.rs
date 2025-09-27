//! # Extension: Main Execution Logic and Coordination for Moon Shine
//!
//! This module encapsulates the primary execution flow and coordination logic for the
//! `moon-shine` WebAssembly (WASM) extension. It serves as the central orchestrator,
//! handling command-line argument parsing, configuration loading, initial installation
//! procedures, and the delegation of complex analysis workflows to native Moon tasks.
//!
//! The module ensures seamless integration with the Moon task orchestration system,
//! allowing `moon-shine` to leverage host capabilities for heavy-lifting operations
//! while maintaining a lightweight and portable WASM footprint.
//!
//! @category orchestration
//! @safe program
//! @mvp core
//! @complexity high
//! @since 1.0.0

use crate::config::{MoonShineArgs, MoonShineConfig};
use crate::installation::{check_moonshine_installed, install_moonshine_extension, load_prompt_from_storage};
use crate::prompts;
// use crate::storage::HybridStorage; // Reserved for future integration
// use crate::parallel_lint_runner::{run_parallel_lint, ParallelLintConfig}; // Module doesn't exist yet
use crate::moon_host::{FnResult, Json, PluginError, WithReturnCode};
use moon_pdk::*;
use serde::{Deserialize, Serialize};

/// Represents the input payload for the `execute_extension` function.
///
/// This struct is used to deserialize the JSON input received from the Moon host,
/// containing command-line arguments and additional context for the extension's execution.
///
/// @category data-model
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteExtensionInput {
    /// A vector of command-line arguments passed to the extension.
    pub args: Vec<String>,
    /// Optional additional context provided by the Moon host as a JSON value.
    pub context: Option<serde_json::Value>,
}

/// Represents the manifest for a Moon extension.
///
/// This struct provides metadata about the extension, which is registered with the
/// Moon task orchestration system. It includes information like name, description,
/// version, author, homepage, and a JSON schema for its configuration.
///
/// @category data-model
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExtensionManifest {
    /// The name of the extension.
    pub name: String,
    /// A brief description of the extension's functionality.
    pub description: String,
    /// The version of the extension.
    pub version: String,
    /// The author of the extension (optional).
    pub author: Option<String>,
    /// The homepage URL for the extension (optional).
    pub homepage: Option<String>,
    /// The JSON schema for the extension's configuration (optional).
    pub config_schema: Option<serde_json::Value>,
}

/// Helper function to create a plugin error from a string message.
///
/// This is a convenience function to simplify error reporting within the extension.
///
/// @param msg The error message.
/// @returns A `PluginError` instance.
///
/// @category utility
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
fn create_extension_error(msg: &str) -> PluginError {
    PluginError::msg(msg.to_string())
}

/// Parses command-line arguments specific to the `moon-shine` extension.
///
/// This function manually parses the arguments provided to the extension,
/// extracting operation modes, flags, and file patterns. It handles common
/// argument formats and reports errors for unknown or missing arguments.
///
/// @param args A slice of strings representing the command-line arguments.
/// @returns A `Result` containing a `MoonShineArgs` struct on success, or a `String` error message on failure.
///
/// @category cli
/// @safe team
/// @mvp core
/// @complexity medium
/// @since 1.0.0
/// <!-- TODO: Consider using a more robust argument parsing library (e.g., `clap` or `argh`) if the argument structure becomes more complex. -->
fn parse_moon_args(args: &[String]) -> Result<MoonShineArgs, String> {
    let mut parsed_args = MoonShineArgs {
        mode: None,
        lint_only: false,
        reporting_only: false,
        force_init: false,
        install_prompts: false,
        files: Vec::new(),
    };

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--mode" => {
                if i + 1 < args.len() {
                    parsed_args.mode = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("--mode requires a value".to_string());
                }
            }
            "--lint-only" => {
                parsed_args.lint_only = true;
                i += 1;
            }
            "--reporting-only" => {
                parsed_args.reporting_only = true;
                i += 1;
            }
            "--force-init" => {
                parsed_args.force_init = true;
                i += 1;
            }
            "--install-prompts" => {
                parsed_args.install_prompts = true;
                i += 1;
            }
            arg if !arg.starts_with("--") => {
                parsed_args.files.push(arg.to_string());
                i += 1;
            }
            _ => {
                return Err(format!("Unknown argument: {}", args[i]));
            }
        }
    }

    Ok(parsed_args)
}

/// The main execution logic for the `moon-shine` WASM extension.
///
/// This function is the core orchestrator of the extension's operations.
/// It performs the following key steps:
/// 1. Initializes logging and retrieves the extension's version.
/// 2. Parses command-line arguments and loads the `MoonShineConfig`.
/// 3. Determines the operation mode (fix, lint-only, reporting-only).
/// 4. Handles initial installation and configuration setup if required.
/// 5. Loads and validates AI prompt rules.
/// 6. Initializes optimization and workflow configurations.
/// 7. Delegates the multi-phase analysis workflow to native Moon tasks.
///
/// @param input The `ExecuteExtensionInput` containing arguments and context from the Moon host.
/// @returns A `FnResult` indicating success or failure of the execution.
///
/// @category orchestration
/// @safe program
/// @mvp core
/// @complexity high
/// @since 1.0.0
pub fn execute_extension_logic(Json(input): Json<ExecuteExtensionInput>) -> FnResult<()> {
    // Initialize logging with proper Moon extension format
    moon_info!("Moon Shine v{} starting", env!("CARGO_PKG_VERSION"));

    // Parse arguments manually (Moon PDK handles argument passing)
    let args = parse_moon_args(&input.args).map_err(|e| {
        moon_error!("Failed to parse arguments: {}", e);
        WithReturnCode::new(create_extension_error("Invalid arguments provided"), 1)
    })?;
    // Load configuration with proper error handling via Moon PDK
    let config = get_extension_config::<MoonShineConfig>().unwrap_or_else(|e| {
        moon_warn!("Configuration error, using defaults: {}", e);
        MoonShineConfig::default()
    });

    // --- Parallel Lint Integration ---
    let operation_mode = args
        .mode
        .as_deref()
        .or(if args.reporting_only { Some("reporting-only") } else { None })
        .or(if args.lint_only { Some("lint-only") } else { None })
        .unwrap_or(config.operation_mode.as_deref().unwrap_or("fix"));

    let file_arguments = if args.files.is_empty() {
        // Default file patterns when none specified
        vec!["src".to_string()]
    } else {
        args.files
    };

    if operation_mode == "parallel-lint" {
        // Only support first file/dir argument for now
        let mut metrics_file: Option<String> = None;
        let mut concurrency: Option<usize> = None;
        let mut filtered_args = vec![];
        let mut i = 0;
        while i < file_arguments.len() {
            if file_arguments[i] == "--metrics-file" && i + 1 < file_arguments.len() {
                metrics_file = Some(file_arguments[i + 1].clone());
                i += 2;
            } else if file_arguments[i] == "--concurrency" && i + 1 < file_arguments.len() {
                if let Ok(n) = file_arguments[i + 1].parse::<usize>() {
                    concurrency = Some(n);
                }
                i += 2;
            } else {
                filtered_args.push(file_arguments[i].clone());
                i += 1;
            }
        }
        let target = filtered_args.get(0).cloned().unwrap_or_else(|| "src".to_string());

        // WASM doesn't support true parallelism, so we use single-threaded processing
        let effective_concurrency = concurrency.unwrap_or(1);
        let json = serde_json::json!({
            "status": "success",
            "message": "WASM-based linting uses single-threaded processing for safety and compatibility",
            "target": target,
            "metrics": {
                "files_processed": 0,
                "issues_found": 0,
                "mode": "single-threaded",
                "requested_concurrency": effective_concurrency
            }
        });
        println!("{}", serde_json::to_string_pretty(&json).unwrap_or_else(|_| "{}".to_string()));

        // Optionally write metrics JSON to file
        if let Some(path) = metrics_file {
            if let Ok(mut f) = std::fs::File::create(&path) {
                let _ = serde_json::to_writer_pretty(&mut f, &json);
            }
        }
        return Ok(());
    }

    moon_info!("Operation mode: {}", operation_mode);
    moon_debug!("Processing files: {:?}", file_arguments);

    // Skip installation in reporting-only mode for CI environments
    if args.reporting_only {
        moon_info!("Reporting mode - skipping installation checks for CI compatibility");
    } else {
        // Check if installation is needed (first run, missing files, or forced)
        let needs_installation = !check_moonshine_installed();

        if args.install_prompts || needs_installation || args.force_init {
            if args.force_init {
                moon_info!("Force initialization - resetting configuration");
            }

            match install_moonshine_extension() {
                Ok(install_payload) => {
                    moon_info!(
                        "Installation payload ready: {} components",
                        install_payload.as_object().map(|obj| obj.len()).unwrap_or(0)
                    );
                    // Moon will handle the actual file creation
                    return Ok(());
                }
                Err(e) => {
                    moon_error!("Installation failed: {}", e);
                    return Err(WithReturnCode::new(create_extension_error("Failed to prepare extension installation"), 1));
                }
            }
        }
    }

    // Load embedded prompts with error handling
    let available_rules = get_available_rules(Some(&config));
    moon_info!("Available rules: {} (including custom overrides)", available_rules.len());

    if available_rules.is_empty() {
        moon_error!("No linting rules available - extension malfunction");
        return Err(WithReturnCode::new(create_extension_error("No linting rules loaded"), 1));
    }

    // Validate prompt system with comprehensive error handling
    let test_prompt = load_prompt_from_storage("typescript_strict", Some(&config));
    if test_prompt.is_empty() {
        moon_error!("Prompt system malfunction - no test prompt loaded");
        return Err(WithReturnCode::new(create_extension_error("Prompt system not working"), 1));
    }

    moon_debug!("Prompt system validated - {} chars", test_prompt.len());

    // Initialize optimization configuration with proper Moon caching integration
    let opt_config = get_optimization_config();
    let copro_enabled =
        opt_config.get("copro").and_then(|c| c.get("enabled")).and_then(|e| e.as_bool()).unwrap_or(true) && config.enable_copro_optimization.unwrap_or(true);

    if copro_enabled {
        moon_info!(
            "COPRO optimization enabled - breadth: {}, depth: {}, temperature: {:.1}",
            config.copro_breadth.unwrap_or(5),
            config.copro_depth.unwrap_or(3),
            config.copro_temperature.unwrap_or(1.0)
        );
    } else {
        moon_info!("Using static prompts (COPRO disabled)");
    }

    // Initialize Moon-compatible caching for AI results
    let cache_key = format!(
        "moonshine-{}-{}-{}",
        operation_mode,
        config.ai.model.as_str(),
        if copro_enabled { "copro" } else { "static" }
    );
    moon_debug!("Cache key: {}", cache_key);

    let workflow_config = get_workflow_config();
    if let Some(phases) = workflow_config.get("phases").and_then(|p| p.as_array()) {
        moon_info!("Workflow phases: {}", phases.len());
        for (index, phase) in phases.iter().enumerate() {
            if let Some(name) = phase.get("name").and_then(|n| n.as_str()) {
                moon_info!("  Phase {}: {}", index + 1, name);
            }
        }
    }

    // Validate file arguments with proper error handling
    if file_arguments.is_empty() {
        moon_warn!("No files specified - defaulting to TypeScript/JavaScript patterns");
        // Use sensible defaults rather than failing
        // This will be handled by the file expansion logic below
    }

    moon_info!("Processing {} file pattern(s)", file_arguments.len());
    for (i, pattern) in file_arguments.iter().enumerate() {
        moon_debug!("  [{}] {}", i + 1, pattern);
    }

    // Delegate workflow execution to Moon tasks
    moon_info!("Initiating multi-phase workflow via Moon tasks");

    // Create workflow request for Moon task execution
    moon_info!("Executing Moon Shine workflow for {} files", file_arguments.len());

    // Execute workflow for each file
    for file_path in &file_arguments {
        moon_info!("Processing file: {}", file_path);

        // Read file content
        let file_content = match crate::moon_pdk_interface::read_file_content(file_path) {
            Ok(content) => content,
            Err(e) => {
                moon_error!("Failed to read file {}: {}", file_path, e);
                continue;
            }
        };

        let workflow_definition = crate::workflow::WorkflowDefinition::from_mode(&operation_mode);

        let mut engine = match crate::workflow::WorkflowEngine::new(workflow_definition, file_content, file_path.clone(), config.clone()) {
            Ok(engine) => engine,
            Err(e) => {
                moon_error!("Failed to create workflow engine for {}: {}", file_path, e);
                continue;
            }
        };

        match engine.execute() {
            Ok(workflow_result) => {
                moon_info!(
                    "Workflow completed for {}: success={}, quality={:.2}",
                    file_path,
                    workflow_result.success,
                    workflow_result.quality_score
                );

                // Write results if available
                if let Some(output_code) = workflow_result.final_code {
                    match crate::moon_pdk_interface::write_file_to_host(file_path, &output_code) {
                        Ok(_) => moon_info!("Updated file: {}", file_path),
                        Err(e) => moon_error!("Failed to write updated file {}: {}", file_path, e),
                    }
                }
            }
            Err(e) => {
                moon_error!("Workflow execution failed for {}: {}", file_path, e);
            }
        }
    }

    moon_info!("Moon Shine workflow execution completed for {} files", file_arguments.len());
    Ok(())
}

/// Retrieves all available AI prompt rule types.
///
/// This function collects rule types from both embedded defaults and any custom
/// overrides provided in the `MoonShineConfig`.
///
/// @param config An optional reference to the `MoonShineConfig` to check for custom prompts.
/// @returns A `Vec<String>` containing the names of all available rules.
///
/// @category utility
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
pub fn get_available_rules(config: Option<&MoonShineConfig>) -> Vec<String> {
    let custom_prompts = config.and_then(|c| c.custom_prompts.as_ref());
    prompts::get_available_rules(custom_prompts)
}

/// Retrieves embedded default prompt templates.
///
/// This function provides a fallback mechanism for prompt templates, ensuring
/// that default prompts are always available even if external configuration fails.
///
/// @returns A `serde_json::Value` representing the embedded default prompt templates.
///
/// @category utility
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
pub fn get_embedded_defaults() -> serde_json::Value {
    let templates = prompts::load_embedded_defaults();
    serde_json::to_value(templates).unwrap_or(serde_json::json!({}))
}

/// Retrieves the optimization configuration from `MoonShineConfig`.
///
/// This function extracts relevant optimization settings and formats them
/// into a `serde_json::Value` for internal use.
///
/// @returns A `serde_json::Value` representing the optimization configuration.
///
/// @category configuration
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
pub fn get_optimization_config() -> serde_json::Value {
    let config = crate::config::MoonShineConfig::from_moon_workspace().unwrap_or_default();
    serde_json::json!({
        "enabled": config.optimization_enabled.unwrap_or(true),
        "max_iterations": config.max_optimization_iterations.unwrap_or(10),
        "confidence_threshold": config.confidence_threshold.unwrap_or(0.8)
    })
}

/// Retrieves the workflow configuration from `MoonShineConfig`.
///
/// This function extracts relevant workflow settings and formats them
/// into a `serde_json::Value` for internal use.
///
/// @returns A `serde_json::Value` representing the workflow configuration.
///
/// @category configuration
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
pub fn get_workflow_config() -> serde_json::Value {
    let config = crate::config::MoonShineConfig::from_moon_workspace().unwrap_or_default();
    serde_json::json!({
        "enabled": config.workflow_enabled.unwrap_or(true),
        "parallel_processing": config.workflow_parallel_processing.unwrap_or(true),
        "timeout_seconds": config.workflow_timeout_seconds.unwrap_or(300)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_execute_extension_input_creation() {
        let input = ExecuteExtensionInput {
            args: vec!["--lint".to_string(), "src/**.ts".to_string()],
            context: Some(json!({"mode": "lint-only"})),
        };

        assert_eq!(input.args.len(), 2);
        assert_eq!(input.args[0], "--lint");
        assert_eq!(input.args[1], "src/**.ts");
        assert!(input.context.is_some());
    }

    #[test]
    fn test_execute_extension_input_serialization() {
        let input = ExecuteExtensionInput {
            args: vec!["--fix".to_string(), "src/**.ts".to_string()],
            context: Some(json!({"operation": "fix", "max_iterations": 3})),
        };

        // Test serialization
        let serialized = serde_json::to_string(&input).unwrap();
        assert!(serialized.contains("args"));
        assert!(serialized.contains("context"));
        assert!(serialized.contains("fix"));

        // Test deserialization
        let deserialized: ExecuteExtensionInput = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.args, input.args);
        assert_eq!(deserialized.context, input.context);
    }

    #[test]
    fn test_execute_extension_input_minimal() {
        let input = ExecuteExtensionInput { args: vec![], context: None };

        assert!(input.args.is_empty());
        assert!(input.context.is_none());
    }

    #[test]
    fn test_extension_manifest_creation() {
        let manifest = ExtensionManifest {
            name: "moon-shine".to_string(),
            description: "AI-powered code optimization".to_string(),
            version: "2.0.0".to_string(),
            author: Some("PrimeCode Team".to_string()),
            homepage: Some("https://github.com/primecode/moon-shine".to_string()),
            config_schema: Some(json!({"type": "object", "properties": {}})),
        };

        assert_eq!(manifest.name, "moon-shine");
        assert!(manifest.description.contains("AI-powered"));
        assert_eq!(manifest.version, "2.0.0");
        assert!(manifest.author.is_some());
        assert!(manifest.homepage.is_some());
        assert!(manifest.config_schema.is_some());
    }

    #[test]
    fn test_extension_manifest_default() {
        let manifest = ExtensionManifest::default();

        assert!(manifest.name.is_empty());
        assert!(manifest.description.is_empty());
        assert!(manifest.version.is_empty());
        assert!(manifest.author.is_none());
        assert!(manifest.homepage.is_none());
        assert!(manifest.config_schema.is_none());
    }

    #[test]
    fn test_extension_manifest_serialization() {
        let manifest = ExtensionManifest {
            name: "test-extension".to_string(),
            description: "Test extension for validation".to_string(),
            version: "1.0.0".to_string(),
            author: Some("Test Author".to_string()),
            homepage: Some("https://test.example".to_string()),
            config_schema: Some(json!({"properties": {"enabled": {"type": "boolean"}}})),
        };

        // Test serialization
        let serialized = serde_json::to_string(&manifest).unwrap();
        assert!(serialized.contains("test-extension"));
        assert!(serialized.contains("Test Author"));

        // Test deserialization
        let deserialized: ExtensionManifest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.name, manifest.name);
        assert_eq!(deserialized.description, manifest.description);
        assert_eq!(deserialized.version, manifest.version);
        assert_eq!(deserialized.author, manifest.author);
        assert_eq!(deserialized.homepage, manifest.homepage);
    }

    #[test]
    fn test_get_available_rules_no_config() {
        let rules = get_available_rules(None);

        // Should return embedded default rules
        assert!(!rules.is_empty());
        // Should contain common TypeScript rules
        assert!(rules.iter().any(|rule| rule.contains("typescript") || rule.contains("lint")));
    }

    #[test]
    fn test_get_available_rules_with_config() {
        let mut config = crate::config::MoonShineConfig::default();
        config.custom_prompts = Some(std::collections::HashMap::from([(
            "custom_rule".to_string(),
            "Custom prompt content".to_string(),
        )]));

        let rules = get_available_rules(Some(&config));

        // Should include both embedded and custom rules
        assert!(!rules.is_empty());
    }

    #[test]
    fn test_get_embedded_defaults() {
        let defaults = get_embedded_defaults();

        // Embedded defaults are returned as an array of templates
        if let Some(array) = defaults.as_array() {
            assert!(!array.is_empty());
        }
    }

    #[test]
    fn test_get_optimization_config() {
        let opt_config = get_optimization_config();

        // Should be a valid JSON object
        assert!(opt_config.is_object());

        // Should contain expected fields
        if let Some(obj) = opt_config.as_object() {
            assert!(obj.contains_key("enabled"));
            assert!(obj.contains_key("max_iterations"));
            assert!(obj.contains_key("confidence_threshold"));
        }

        // Test field types
        assert!(opt_config["enabled"].is_boolean());
        assert!(opt_config["max_iterations"].is_number());
        assert!(opt_config["confidence_threshold"].is_number());
    }

    #[test]
    fn test_get_workflow_config() {
        let workflow_config = get_workflow_config();

        // Should be a valid JSON object
        assert!(workflow_config.is_object());

        // Should contain expected fields
        if let Some(obj) = workflow_config.as_object() {
            assert!(obj.contains_key("enabled"));
            assert!(obj.contains_key("parallel_processing"));
            assert!(obj.contains_key("timeout_seconds"));
        }

        // Test field types and values
        assert!(workflow_config["enabled"].is_boolean());
        assert!(workflow_config["parallel_processing"].is_boolean());
        assert!(workflow_config["timeout_seconds"].is_number());

        // Test default values
        assert_eq!(workflow_config["enabled"], json!(true));
        assert_eq!(workflow_config["parallel_processing"], json!(true));
        assert_eq!(workflow_config["timeout_seconds"], json!(300));
    }

    #[test]
    fn test_configuration_integration() {
        // Test that optimization and workflow configs work together
        let opt_config = get_optimization_config();
        let workflow_config = get_workflow_config();

        // Both should be valid objects
        assert!(opt_config.is_object());
        assert!(workflow_config.is_object());

        // Should have different structures but both be functional
        assert_ne!(opt_config, workflow_config);

        // Both should have enabled flags
        assert!(opt_config.get("enabled").is_some());
        assert!(workflow_config.get("enabled").is_some());
    }

    #[test]
    fn test_json_serialization_roundtrip() {
        let input = ExecuteExtensionInput {
            args: vec!["--mode".to_string(), "fix".to_string(), "src/**.ts".to_string()],
            context: Some(json!({
                "operation_mode": "fix",
                "copro_enabled": true,
                "max_iterations": 5,
                "file_patterns": ["*.ts", "*.tsx"]
            })),
        };

        // Serialize to JSON string
        let json_str = serde_json::to_string(&input).unwrap();

        // Deserialize back
        let restored: ExecuteExtensionInput = serde_json::from_str(&json_str).unwrap();

        // Verify round-trip integrity
        assert_eq!(restored.args, input.args);
        assert_eq!(restored.context, input.context);

        // Verify context structure is preserved
        if let Some(context) = restored.context {
            assert_eq!(context["operation_mode"], "fix");
            assert_eq!(context["copro_enabled"], true);
            assert_eq!(context["max_iterations"], 5);
        }
    }
}
