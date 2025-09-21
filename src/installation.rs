//! # Installation: Setup and Initialization Logic for Moon Shine
//!
//! This module encapsulates the installation and initial setup procedures for the
//! `moon-shine` extension. It is responsible for preparing the necessary directory
//! structure, generating default configuration files (like `prompts.json` and `training.json`),
//! and defining Moon task templates for seamless integration.
//!
//! The installation process is designed to be WASM-compatible, delegating actual file
//! creation to the Moon host via JSON-based requests, as WASM modules cannot directly
//! interact with the file system.
//!
//! @category installation
//! @safe program
//! @mvp core
//! @complexity medium
//! @since 1.0.0

use crate::config::MoonShineConfig;
use crate::moon_pdk_interface::get_moon_config;
use crate::prompts;
use serde_json;

/// Initializes the `moon-shine` directory structure and base prompt templates.
///
/// This function is invoked when the extension is first installed or when its
/// required configuration files are missing. It generates a JSON structure
/// containing the installation request, which is then processed by Moon tasks
/// on the host machine to create the actual files and directories.
///
/// @returns An `anyhow::Result` containing a `serde_json::Value` representing the
///          installation request on success, or an `Error` on failure.
///
/// @category setup
/// @safe program
/// @mvp core
/// @complexity medium
/// @since 1.0.0
pub fn install_moonshine_extension() -> anyhow::Result<serde_json::Value> {
  let moonshine_dir = ".moon/moonshine";

  let created_at = chrono::Utc::now().to_rfc3339();

  // Return complete installation request for Moon tasks to handle
  let install_request = serde_json::json!({
      "action": "install_moonshine_extension",
      "moonshine_dir": moonshine_dir,
      "directory_structure": {
          "prompts.json": "All prompts: base + optimized + COPRO candidates",
          "training.json": "All learning: examples + history + patterns",
          "sessions/": "Temporary session data (auto-cleaned)",
          "cache/": "WASM runtime cache for fast access"
      },
      "initial_files": {
          "prompts.json": create_initial_prompts_file()?,
          "training.json": create_initial_training_file()?,
          "config.json": serde_json::to_value(MoonShineConfig::default())? // Add default config
      },
      "config_note": "Configuration managed by Moon PDK via get_moon_config() from .moon/workspace.yml",
      "version": env!("CARGO_PKG_VERSION"),
      "created_at": created_at,
      "task_templates": create_task_templates()?,
      "readme_content": include_str!("../README.md") // Include README content for initial setup
  });

  Ok(install_request)
}

/// Creates default Moon task templates for `moon-shine`.
///
/// These templates define common operations like `shine` (fix with AI) and `shine-lint`
/// (report issues without fixing), making it easy for users to integrate `moon-shine`
/// into their Moon-based projects.
///
/// @returns An `anyhow::Result` containing a `serde_json::Value` representing the
///          task templates on success, or an `Error` on failure.
///
/// @category setup
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
fn create_task_templates() -> anyhow::Result<serde_json::Value> {
  use serde_json::json;

  let templates = json!({
      "shine": {
          "command": "moon-shine",
          "inputs": ["src/**/*.{ts,tsx,js,jsx}"],
          "outputs": ["src/**/*.{ts,tsx,js,jsx}"],
          "deps": ["~:type-check"],
          "options": {
              "cache": true,
              "runInCI": true
          },
          "description": "Fix TypeScript/JavaScript issues with AI assistance (default mode)"
      },
      "shine-lint": {
          "command": "moon-shine --lint-only",
          "inputs": ["src/**/*.{ts,tsx,js,jsx}"],
          "outputs": ["reports/shine.json"],
          "deps": ["~:type-check"],
          "options": {
              "cache": true,
              "runInCI": true
          },
          "description": "Report TypeScript/JavaScript issues without fixing (CI mode)"
      }
  });

  Ok(templates)
}

/// Checks if `moon-shine` is already installed by verifying the presence of key configuration files.
///
/// This function queries Moon's configuration system to determine if `prompts.json`
/// and `training.json` exist, indicating a previous successful installation.
///
/// @returns `true` if `moon-shine` is considered installed, `false` otherwise.
///
/// @category utility
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
pub fn check_moonshine_installed() -> bool {
  // Check if our key configuration files exist via Moon's config system
  let prompts_exist = get_moon_config("moonshine_prompts").is_some();
  let training_exist = get_moon_config("moonshine_training").is_some();

  prompts_exist && training_exist
}

/// Creates the initial content for `prompts.json`.
///
/// This file stores base prompt templates, optimized versions, and COPRO candidates.
/// It is generated during the initial installation of the extension.
///
/// @returns An `anyhow::Result` containing a `serde_json::Value` representing the
///          initial `prompts.json` content on success, or an `Error` on failure.
///
/// @category setup
/// @safe team
/// @mvp core
/// @complexity medium
/// @since 1.0.0
/// <!-- TODO: Review the prompts for clarity, conciseness, and effectiveness. Ensure they align with the latest best practices for AI prompting. -->
fn create_initial_prompts_file() -> anyhow::Result<serde_json::Value> {
  use serde_json::json;

  // Rule-specific base prompts
  let rule_prompts = json!({
      "no_unused_vars": {
          "prompt": "IMPLEMENT unused parameters in method bodies instead of prefixing with underscore.\nCOMPLETE async method implementations with full business logic using parameters.\nReplace empty stubs with complete, functional code using ALL method parameters.\nFocus on: meaningful parameter usage, proper async implementations, business logic.\nExample: async createResource(id: UUID, config: ResourceConfig) should use both id and config.",
          "category": "completion",
          "confidence_threshold": 0.9
      },
      "missing_types": {
          "prompt": "Google TypeScript Style: Use nullish coalescing (??) over logical OR (||).\nGoogle TypeScript Style: Use optional chaining (?.) over manual null checks.\nReplace explicit 'any' types with proper TypeScript interfaces.\nAdd explicit return types to all public methods and functions.\nFocus on: strict type safety, modern TypeScript patterns, Google style compliance.",
          "category": "types",
          "confidence_threshold": 0.85
      },
      "no_console": {
          "prompt": "Remove console.log statements for production code.\nReplace with structured logging libraries or debug flags.\nRemove debugger statements before committing.\nFocus on: production readiness, proper logging patterns, clean deployments.",
          "category": "production",
          "confidence_threshold": 0.95
      },
      "missing_jsdoc": {
          "prompt": "Add comprehensive TSDoc comments for ALL missing methods targeting 90% coverage.\nInclude required custom tags: @category, @safe, @mvp, @complexity, @since.\nWrite clear descriptions with @param, @returns, @throws tags.\nFocus on: public APIs, exported modules, factual AI-optimized documentation.\nExample: @category coordination @safe large-solution @mvp core @complexity high @since 1.0.0",
          "category": "documentation",
          "confidence_threshold": 0.8
      },
      "async_best_practices": {
          "prompt": "Fix missing await on Promise-returning functions.\nHandle unhandled Promise rejections with proper error boundaries.\nImplement complete async method business logic, not empty placeholders.\nFocus on: proper async/await patterns, error handling, complete implementations.\nExample: async methods should use parameters meaningfully and return proper results.",
          "category": "async",
          "confidence_threshold": 0.9
      }
  });

  // Pass-specific prompts from proven ai-lint.js patterns
  let pass_prompts = json!({
      "pass_1_compilation_critical": {
          "prompt": "COMPILATION + CRITICAL ERRORS (Pass 1 Focus)\n- TypeScript compilation errors (TS2XXX codes) - HIGHEST PRIORITY\n- Syntax errors, type errors, compilation failures\n- Security issues (no-eval, no-implied-eval)\n- Runtime errors (no-undef, no-unreachable)\n- Promise handling (no-floating-promises)\nGOAL: TypeScript compilation success + 0 critical runtime errors",
          "phase": 1,
          "priority": "critical"
      },
      "pass_2_type_safety_implementation": {
          "prompt": "TYPE SAFETY + IMPLEMENTATION (Pass 2 Focus)\n- IMPLEMENT unused parameters in method bodies instead of prefixing with underscore\n- COMPLETE async method implementations with full business logic\n- Replace explicit 'any' types with proper TypeScript interfaces\n- Missing return types and interface definition errors\nGOAL: Full parameter usage + complete async implementations + strict types",
          "phase": 2,
          "priority": "high"
      },
      "pass_3_code_quality_google_style": {
          "prompt": "CODE QUALITY + GOOGLE STYLE (Pass 3 Focus)\n- Google TypeScript Style: Use nullish coalescing (??) over logical OR (||)\n- Google TypeScript Style: Use optional chaining (?.) over manual null checks\n- Modern patterns: prefer-optional-chain, prefer-nullish-coalescing\n- Complexity reduction (complexity, max-lines, cognitive-complexity)\n- Best practices (prefer-const, no-var, prefer-readonly)\nGOAL: Modern TypeScript patterns + Google style compliance",
          "phase": 3,
          "priority": "medium"
      }
  });

  // Create consolidated prompts.json structure
  Ok(json!({
      "base_prompts": rule_prompts,
      "pass_prompts": pass_prompts,
      "optimized_prompts": {},  // Will be populated by DSPy optimization
      "copro_candidates": {},   // Will store attempted instructions to avoid
      "meta": {
          "version": "1.0.0",
          "created_at": chrono::Utc::now().to_rfc3339(),
          "last_optimization": null,
          "optimization_count": 0
      }
  }))
}

/// Creates the initial content for `training.json`.
///
/// This file stores accumulated DSPy training examples, optimization history,
/// learned patterns, and AI effectiveness metrics. It is generated during the
/// initial installation of the extension.
///
/// @returns An `anyhow::Result` containing a `serde_json::Value` representing the
///          initial `training.json` content on success, or an `Error` on failure.
///
/// @category setup
/// @safe team
/// @mvp core
/// @complexity medium
/// @since 1.0.0
fn create_initial_training_file() -> anyhow::Result<serde_json::Value> {
  use serde_json::json;

  Ok(json!({
      "training_examples": [],      // Accumulated DSPy training examples
      "optimization_history": [],  // CodeFixingModule optimization attempts
      "learned_patterns": {},      // Pattern detection results
      "claude_effectiveness": {    // Success rates for different prompts
          "total_requests": 0,
          "successful_fixes": 0,
          "success_rate": 0.0,
          "rule_performance": {}
      },
      "meta": {
          "version": "1.0.0",
          "created_at": chrono::Utc::now().to_rfc3339(),
          "total_sessions": 0,
          "last_training": null,
          "data_retention_days": 30
      }
  }))
}

/// Loads a prompt template from storage based on a rule type.
///
/// This function implements a priority-based loading mechanism:
/// 1. Custom prompts from the `MoonShineConfig` (if provided).
/// 2. Embedded default prompts.
/// 3. Built-in fallback prompts.
///
/// @param rule_type The type of rule for which to load the prompt (e.g., "no_unused_vars").
/// @param config An optional reference to the `MoonShineConfig` for custom prompt overrides.
/// @returns The loaded prompt template as a `String`.
///
/// @category utility
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
pub fn load_prompt_from_storage(
  rule_type: &str,
  config: Option<&MoonShineConfig>,
) -> String {
  let custom_prompts = config.and_then(|c| c.custom_prompts.as_ref());
  prompts::get_prompt(rule_type, custom_prompts)
}

/// Logs an optimized prompt learned by DSPy COPRO to `prompts.json`.
///
/// This function records the details of a newly optimized prompt, including its
/// confidence score, the number of training examples used, and its success rate.
/// The update is persisted via a Moon host function.
///
/// @param rule_type The type of rule for which the prompt was optimized.
/// @param optimized_template The optimized prompt template string.
/// @param confidence_score The confidence score of the optimized prompt.
/// @param training_examples The number of training examples used for optimization.
/// @param success_rate The success rate achieved by the optimized prompt.
///
/// @category persistence
/// @safe program
/// @mvp core
/// @complexity medium
/// @since 1.0.0
pub fn log_optimized_prompt(
  rule_type: &str,
  optimized_template: &str,
  confidence_score: f32,
  training_examples: usize,
  success_rate: f32,
) {
  use extism_pdk::info;

  info!(
    "DSPy COPRO optimization: {} improved to {:.2}% success rate ({} examples)",
    rule_type,
    success_rate * 100.0,
    training_examples
  );

  // Create update object for prompts.json
  let file_path = ".moon/moonshine/prompts.json";
  let optimized_entry = serde_json::json!({
      "template": optimized_template,
      "confidence_score": confidence_score,
      "training_examples": training_examples,
      "success_rate": success_rate,
      "timestamp": chrono::Utc::now().to_rfc3339(),
  });

  let update_info = serde_json::json!({
      "action": "update_optimized_prompt",
      "rule_type": rule_type,
      "data": optimized_entry,
      "file_path": file_path
  })
  .to_string();

  if let Err(e) =
    crate::moon_pdk_interface::write_file_to_host(file_path, &update_info)
  {
    use extism_pdk::error;
    error!("Failed to persist optimized prompt update: {}", e);
  } else {
    info!(
      "Optimized prompt update for {} logged to {}",
      rule_type, file_path
    );
  }
}

/// Returns installation information for moon-shine extension.
///
/// This function provides comprehensive information about the installation
/// capabilities and configuration of the moon-shine extension.
///
/// @returns A `serde_json::Value` containing installation information.
///
/// @category utility
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
pub fn get_installation_info() -> serde_json::Value {
  serde_json::json!({
      "moonshine_version": env!("CARGO_PKG_VERSION"),
      "extension_type": "wasm",
      "moon_integration": true,
      "moonshine_dir": ".moon/moonshine",
      "supports_ai_optimization": true,
      "supports_dspy": true,
      "supports_copro": true,
      "supported_languages": ["typescript", "javascript", "tsx", "jsx"],
      "required_dependencies": ["moon-pdk"],
      "installation_status": if check_moonshine_installed() { "installed" } else { "not_installed" }
  })
}

/// Logs an optimized prompt update to the specified file.
///
/// This is a utility function for test purposes that handles
/// optimized prompt updates with proper error handling.
///
/// @param rule_type The type of rule being optimized.
/// @param optimized_data The optimized prompt data as JSON.
/// @param file_path The target file path for the update.
///
/// @category utility
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
pub fn log_optimized_prompt_update(
  rule_type: &str,
  optimized_data: &serde_json::Value,
  file_path: &str,
) {
  use extism_pdk::info;

  let update_info = serde_json::json!({
      "action": "update_optimized_prompt",
      "rule_type": rule_type,
      "data": optimized_data,
      "file_path": file_path,
      "timestamp": chrono::Utc::now().to_rfc3339()
  })
  .to_string();

  if let Err(e) =
    crate::moon_pdk_interface::write_file_to_host(file_path, &update_info)
  {
    use extism_pdk::error;
    error!("Failed to persist optimized prompt update: {}", e);
  } else {
    info!(
      "Optimized prompt update for {} logged to {}",
      rule_type, file_path
    );
  }
}

#[cfg(all(test, not(feature = "wasm")))]
mod tests {
  use super::*;

  #[test]
  fn test_install_moonshine_extension() {
    let result = install_moonshine_extension();
    assert!(result.is_ok());

    let install_data = result.unwrap();

    // Verify required fields exist
    assert!(install_data.get("action").is_some());
    assert_eq!(install_data["action"], "install_moonshine_extension");
    assert_eq!(install_data["moonshine_dir"], ".moon/moonshine");

    // Verify directory structure is defined
    let directory_structure = install_data.get("directory_structure");
    assert!(directory_structure.is_some());

    let dir_struct = directory_structure.unwrap();
    assert!(dir_struct.get("prompts.json").is_some());
    assert!(dir_struct.get("training.json").is_some());
    assert!(dir_struct.get("sessions/").is_some());
    assert!(dir_struct.get("cache/").is_some());

    // Verify initial files are created
    let initial_files = install_data.get("initial_files");
    assert!(initial_files.is_some());

    let files = initial_files.unwrap();
    assert!(files.get("prompts.json").is_some());
    assert!(files.get("training.json").is_some());
  }

  #[test]
  fn test_create_initial_prompts_file() {
    let result = create_initial_prompts_file();
    assert!(result.is_ok());

    let prompts_data = result.unwrap();

    // Should be a JSON object
    assert!(prompts_data.is_object());

    // Should contain prompt categories
    assert!(prompts_data.get("base_prompts").is_some());
    assert!(prompts_data.get("optimized_prompts").is_some());
    if let Some(meta) = prompts_data.get("meta") {
      assert!(meta.get("created_at").is_some());
    }

    // Verify base prompts structure
    let base_prompts = prompts_data.get("base_prompts").unwrap();
    assert!(base_prompts.is_object());
  }

  #[test]
  fn test_create_initial_training_file() {
    let result = create_initial_training_file();
    assert!(result.is_ok());

    let training_data = result.unwrap();

    // Should be a JSON object
    assert!(training_data.is_object());

    // Should contain training categories
    assert!(training_data.get("training_examples").is_some());
    assert!(training_data.get("optimization_history").is_some());
    assert!(training_data.get("learned_patterns").is_some());
    assert!(training_data.get("meta").is_some());

    // Verify training_examples is an array
    let examples = training_data.get("training_examples").unwrap();
    assert!(examples.is_array());

    // Verify meta contains created_at
    let meta = training_data.get("meta").unwrap();
    assert!(meta.get("created_at").is_some());
  }

  #[test]
  fn test_get_installation_info() {
    let info = get_installation_info();

    // Should contain key installation information
    assert!(info.get("moonshine_version").is_some());
    assert!(info.get("extension_type").is_some());
    assert_eq!(info["extension_type"], "wasm");

    // Should specify Moon integration
    assert!(info.get("moon_integration").is_some());
    assert_eq!(info["moon_integration"], true);

    // Should contain default directories
    assert!(info.get("moonshine_dir").is_some());
    assert_eq!(info["moonshine_dir"], ".moon/moonshine");
  }

  #[test]
  fn test_get_installation_info_structure() {
    let info = get_installation_info();

    // Verify all expected keys are present
    let expected_keys = vec![
      "moonshine_version",
      "extension_type",
      "moon_integration",
      "moonshine_dir",
      "supports_ai_optimization",
      "supports_dspy",
      "supports_copro",
    ];

    for key in expected_keys {
      assert!(info.get(key).is_some(), "Missing key: {}", key);
    }
  }

  #[test]
  fn test_log_optimized_prompt_update() {
    // Test that the function doesn't panic
    log_optimized_prompt_update(
      "typescript",
      &serde_json::json!({
        "prompt": "optimized prompt text",
        "score": 0.95
      }),
      ".moon/moonshine/prompts_optimized.json",
    );

    // Function should complete without error
    // (In WASM mode, file writing is handled by Moon host)
  }

  #[test]
  fn test_installation_serialization() {
    let install_result = install_moonshine_extension().unwrap();

    // Should be serializable to string
    let serialized = serde_json::to_string(&install_result);
    assert!(serialized.is_ok());

    let json_string = serialized.unwrap();
    assert!(json_string.contains("install_moonshine_extension"));
    assert!(json_string.contains(".moon/moonshine"));
  }

  #[test]
  fn test_prompts_file_structure() {
    let prompts = create_initial_prompts_file().unwrap();

    // Check base prompts structure
    let base_prompts = prompts.get("base_prompts").unwrap();
    assert!(base_prompts.get("no_unused_vars").is_some());
    assert!(base_prompts.get("missing_types").is_some());
    assert!(base_prompts.get("no_console").is_some());

    // Each prompt should have proper structure
    let no_unused_vars_prompt = base_prompts.get("no_unused_vars").unwrap();
    assert!(no_unused_vars_prompt.get("prompt").is_some());
    assert!(no_unused_vars_prompt.get("category").is_some());

    // Should have metadata
    assert!(prompts.get("meta").is_some());
    let metadata = prompts.get("meta").unwrap();
    assert!(metadata.get("version").is_some());
  }

  #[test]
  fn test_training_file_structure() {
    let training = create_initial_training_file().unwrap();

    // Check training_examples array
    let examples = training
      .get("training_examples")
      .unwrap()
      .as_array()
      .unwrap();
    assert!(examples.is_empty()); // Initially empty

    // Check optimization_history array
    let history = training
      .get("optimization_history")
      .unwrap()
      .as_array()
      .unwrap();
    assert!(history.is_empty()); // Initially empty

    // Check learned_patterns structure
    let patterns = training.get("learned_patterns").unwrap();
    assert!(patterns.is_object());

    // Check claude_effectiveness structure
    let effectiveness = training.get("claude_effectiveness").unwrap();
    assert!(effectiveness.get("total_requests").is_some());
    assert!(effectiveness.get("successful_fixes").is_some());
    assert!(effectiveness.get("success_rate").is_some());
  }

  #[test]
  fn test_installation_directory_paths() {
    let install_data = install_moonshine_extension().unwrap();
    let moonshine_dir = install_data["moonshine_dir"].as_str().unwrap();

    // Should use Moon standard location
    assert_eq!(moonshine_dir, ".moon/moonshine");

    // Directory structure should include all required paths
    let dir_structure = &install_data["directory_structure"];
    assert!(dir_structure.get("prompts.json").is_some());
    assert!(dir_structure.get("training.json").is_some());
    assert!(dir_structure.get("sessions/").is_some());
    assert!(dir_structure.get("cache/").is_some());
  }

  #[test]
  fn test_installation_timestamps() {
    let install_data = install_moonshine_extension().unwrap();

    // Should include creation timestamp
    assert!(install_data.get("created_at").is_some());

    let created_at = install_data["created_at"].as_str().unwrap();
    // Should be valid RFC3339 format
    assert!(chrono::DateTime::parse_from_rfc3339(created_at).is_ok());

    // Initial files should also have timestamps in their meta sections
    let prompts = &install_data["initial_files"]["prompts.json"];
    assert!(prompts.get("meta").is_some());
    let prompts_meta = prompts.get("meta").unwrap();
    assert!(prompts_meta.get("created_at").is_some());

    let training = &install_data["initial_files"]["training.json"];
    assert!(training.get("meta").is_some());
    let training_meta = training.get("meta").unwrap();
    assert!(training_meta.get("created_at").is_some());
  }
}
