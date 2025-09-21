/*!
 * Moon JSON Communication Protocol
 *
 * Structured JSON communication between WASM extension and Moon tasks.
 * Replaces environment variables with typed JSON interfaces.
 */

use crate::analysis::MoonTaskRequest;
#[cfg(not(feature = "wasm"))]
use extism_pdk::info;
#[cfg(feature = "wasm")]
use extism_pdk::{error, info, Json};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use extism_pdk::host_fn;

#[cfg(feature = "wasm")]
#[host_fn]
extern "ExtismHost" {
  fn get_moon_config_value(key_ptr: u64) -> u64;
  fn write_file(path_ptr: u64, content_ptr: u64) -> u64;
  fn read_file(path_ptr: u64) -> u64;
  fn file_exists(path_ptr: u64) -> u64;
  fn list_directory(path_ptr: u64) -> u64;
  fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandOutput>;
}

/// Command execution input for Moon host
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecCommandInput {
  pub command: String,
  pub args: Vec<String>,
  pub env: std::collections::HashMap<String, String>,
  pub working_dir: Option<String>,
}

/// Command execution output from Moon host
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecCommandOutput {
  pub exit_code: i32,
  pub stdout: String,
  pub stderr: String,
}

/// Execute command via Moon host (wrapper for host function)
pub fn execute_command(
  input: ExecCommandInput,
) -> Result<ExecCommandOutput, Box<dyn std::error::Error>> {
  #[cfg(feature = "wasm")]
  {
    match unsafe { exec_command(Json(input)) } {
      Ok(Json(output)) => Ok(output),
      Err(e) => Err(format!("Command execution failed: {}", e).into()),
    }
  }
  #[cfg(not(feature = "wasm"))]
  {
    let _ = input;
    return Err("Command execution not available outside WASM environment - must run via Moon extension".into());
  }
}

/// Read file content via Moon host (wrapper for host function)
pub fn read_file_content(
  path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
  #[cfg(feature = "wasm")]
  {
    let path_data = extism_pdk::Memory::new(path)?;
    match unsafe { read_file(path_data.offset()) } {
      0 => Err(format!("Failed to read file: {}", path).into()),
      content_ptr => {
        let content_memory = extism_pdk::Memory::find(content_ptr)
          .ok_or("Invalid memory pointer from read_file")?;
        let content = String::from_utf8(content_memory.to_vec())
          .map_err(|e| format!("Invalid UTF-8 in file {}: {}", path, e))?;
        Ok(content)
      }
    }
  }
  #[cfg(not(feature = "wasm"))]
  {
    let _ = path;
    return Err("File reading not available outside WASM environment - must run via Moon extension".into());
  }
}

/// Check if file exists via Moon host (wrapper for host function)
pub fn check_file_exists(
  path: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
  #[cfg(feature = "wasm")]
  {
    let path_data = extism_pdk::Memory::new(path)?;
    let exists = unsafe { file_exists(path_data.offset()) };
    Ok(exists == 1)
  }
  #[cfg(not(feature = "wasm"))]
  {
    let _ = path;
    return Err("File existence check not available outside WASM environment - must run via Moon extension".into());
  }
}

/// List directory contents via Moon host (wrapper for host function)
pub fn list_directory_contents(
  path: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
  #[cfg(feature = "wasm")]
  {
    let path_data = extism_pdk::Memory::new(path)?;
    match unsafe { list_directory(path_data.offset()) } {
      0 => Err(format!("Failed to list directory: {}", path).into()),
      content_ptr => {
        let content_memory = extism_pdk::Memory::find(content_ptr)
          .ok_or("Invalid memory pointer from list_directory")?;
        let content = String::from_utf8(content_memory.to_vec())
          .map_err(|e| format!("Invalid UTF-8 in directory listing {}: {}", path, e))?;

        // Parse newline-separated file list
        let files = content
          .lines()
          .filter(|line| !line.trim().is_empty())
          .map(|line| line.to_string())
          .collect();
        Ok(files)
      }
    }
  }
  #[cfg(not(feature = "wasm"))]
  {
    let _ = path;
    return Err("Directory listing not available outside WASM environment - must run via Moon extension".into());
  }
}

/// AI Linter configuration for Moon tasks
/// <!-- TODO: The `AiLinterConfig` has many fields. Consider grouping related fields into smaller structs (e.g., `RateLimitingConfig`, `ClaudeConfig`) to improve organization and readability. -->
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiLinterConfig {
  pub enable_claude_ai: bool,
  pub enable_semantic_checks: bool,
  pub claude_model: String,
  pub max_processing_time: u32,
  pub quality_threshold: f32,

  // Concurrency and rate limiting controls
  pub max_concurrent_requests: u32,
  pub batch_size: u32,
  pub rate_limit_per_minute: u32,
  pub max_tokens_per_request: u32,
  pub retry_attempts: u32,
  pub retry_delay_ms: u32,
}

impl Default for AiLinterConfig {
  fn default() -> Self {
    Self {
      enable_claude_ai: true,
      enable_semantic_checks: true,
      claude_model: "sonnet".to_string(),
      max_processing_time: 600, // 10 minutes for big code files
      quality_threshold: 0.8,

      // Production-safe concurrency defaults
      max_concurrent_requests: 3, // Limit to 3 concurrent Claude calls
      batch_size: 5,              // Process 5 files per batch
      rate_limit_per_minute: 20,  // Max 20 requests per minute
      max_tokens_per_request: 16384, // Realistic token limit for big code files
      retry_attempts: 3,          // Retry failed requests 3 times
      retry_delay_ms: 1000,       // 1 second delay between retries
    }
  }
}

/// Semantic warning from AI analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticWarning {
  pub message: String,
  pub line: u32,
  pub column: u32,
  pub severity: String,
  pub category: String,
  pub ai_confidence: f32,
}

/// Configuration for Moon task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoonTaskConfig {
  /// Enable strictest TypeScript checking
  pub enable_strict_typescript: bool,
  /// Enable ESLint with existing config
  pub enable_eslint: bool,
  /// Enable Prettier formatting
  pub enable_prettier: bool,
  /// Enable TSDoc analysis and improvement
  pub enable_tsdoc: bool,
  /// Enable Claude AI fixing
  pub enable_claude_ai: bool,
  /// Enable deterministic semantic checks before AI fixes
  pub enable_semantic_checks: bool,
  /// Claude prompt customization
  pub claude_prompt_template: Option<String>,
  /// Maximum processing time (seconds)
  pub max_processing_time: u32,
}

impl Default for MoonTaskConfig {
  fn default() -> Self {
    Self {
      enable_strict_typescript: true,
      enable_eslint: true,
      enable_prettier: true,
      enable_tsdoc: true,
      enable_claude_ai: true,
      enable_semantic_checks: true,
      claude_prompt_template: None,
      max_processing_time: 600, // 10 minutes for big code files
    }
  }
}

impl MoonTaskConfig {
  pub fn from_ai_config(config: &AiLinterConfig) -> Self {
    Self {
      enable_tsdoc: true, // TSDoc enabled by default for AI optimization
      enable_claude_ai: config.enable_claude_ai,
      enable_semantic_checks: config.enable_semantic_checks,
      ..Default::default()
    }
  }
}

/// JSON response from Moon tasks back to WASM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoonTaskResponse {
  /// Session ID for correlation
  pub session_id: String,
  /// Task that generated this response
  pub task_name: String,
  /// Success status
  pub success: bool,
  /// Error message if failed
  pub error: Option<String>,
  /// Task-specific results
  pub results: MoonTaskResults,
  /// Processing time in milliseconds
  pub processing_time_ms: u64,
  /// Timestamp of completion
  pub completed_at: String,
}

/// Task-specific results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoonTaskResults {
  /// TypeScript compilation results
  pub typescript: Option<TypeScriptResults>,
  /// ESLint results
  pub eslint: Option<ESLintResults>,
  /// Prettier results
  pub prettier: Option<PrettierResults>,
  /// TSDoc analysis results
  pub tsdoc: Option<TSDocResults>,
  /// Claude AI results
  pub claude: Option<ClaudeResults>,
  /// Deterministic semantic validation summary
  pub semantic_validation: Option<SemanticValidationResults>,
}

/// TypeScript compilation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScriptResults {
  /// Compilation successful
  pub compilation_success: bool,
  /// Compilation errors
  pub errors: Vec<TypeScriptError>,
  /// Warnings
  pub warnings: Vec<TypeScriptWarning>,
  /// Configuration used
  pub config_flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScriptError {
  pub file: String,
  pub line: u32,
  pub column: u32,
  pub code: String,
  pub message: String,
  pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScriptWarning {
  pub file: String,
  pub line: u32,
  pub column: u32,
  pub code: String,
  pub message: String,
}

/// ESLint results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESLintResults {
  /// ESLint execution successful
  pub success: bool,
  /// Files processed
  pub files_processed: Vec<String>,
  /// Issues found
  pub issues: Vec<ESLintIssue>,
  /// Auto-fixes applied
  pub fixes_applied: u32,
  /// Config file used
  pub config_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESLintIssue {
  pub file: String,
  pub line: u32,
  pub column: u32,
  pub rule_id: String,
  pub message: String,
  pub severity: String,
  pub fixable: bool,
}

/// Prettier results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrettierResults {
  /// Formatting successful
  pub success: bool,
  /// Files formatted
  pub files_formatted: Vec<String>,
  /// Changes made
  pub changes_made: bool,
  /// Config file used
  pub config_file: Option<String>,
}

/// TSDoc analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TSDocResults {
  /// Analysis successful
  pub success: bool,
  /// Total functions found
  pub total_functions: u32,
  /// Functions with documentation
  pub documented_functions: u32,
  /// Coverage percentage
  pub coverage_percentage: f64,
  /// Missing documentation
  pub missing_docs: Vec<String>,
  /// Improvements suggested
  pub improvements: Vec<String>,
}

/// Claude AI results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeResults {
  /// Claude processing successful
  pub success: bool,
  /// Fixed file content
  pub fixed_content: Option<String>,
  /// Improvements made
  pub improvements: Vec<String>,
  /// Issues resolved
  pub issues_resolved: u32,
  /// Processing time
  pub claude_processing_time_ms: u64,
  /// Token usage
  pub token_usage: Option<ClaudeTokenUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticValidationResults {
  pub passed: bool,
  pub warnings_checked: u32,
  pub unresolved_warnings: Vec<SemanticValidationWarning>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticValidationWarning {
  pub code: String,
  pub message: String,
  pub severity: Option<String>,
  pub pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeTokenUsage {
  pub input_tokens: u32,
  pub output_tokens: u32,
  pub total_cost_usd: f64,
}

/// JSON communication helper functions
/// Helper to get a configuration value from the Moon host.
pub fn get_moon_config(key: &str) -> Option<String> {
  #[cfg(feature = "wasm")]
  {
    use extism_pdk::Memory;
    let key_mem = Memory::new(&key).expect("Failed to create memory for key");
    match unsafe { get_moon_config_value(key_mem.offset()) } {
      Ok(0) => None,
      Ok(value_mem_ptr) => {
        let value_mem =
          Memory::find(value_mem_ptr).expect("Failed to find memory");
        Some(
          value_mem
            .to_string()
            .expect("Failed to convert memory to string"),
        )
      }
      Err(_) => None,
    }
  }
  #[cfg(not(feature = "wasm"))]
  {
    let _ = key;
    return None; // Configuration only available via Moon PDK in WASM environment
  }
}

/// Safe configuration getter with error handling
pub fn get_moon_config_safe(key: &str) -> crate::error::Result<Option<String>> {
  #[cfg(feature = "wasm")]
  {
    use crate::error::Error;
    use extism_pdk::Memory;

    let key_mem = Memory::new(&key).map_err(|e| {
      error!("Memory allocation failed for config key '{}': {}", key, e);
      Error::moon_pdk(format!(
        "Failed to create memory for config key: {}",
        key
      ))
    })?;

    match unsafe { get_moon_config_value(key_mem.offset()) } {
      Ok(0) => Ok(None), // 0 indicates no value found
      Ok(value_mem_ptr) => {
        let value_mem = Memory::find(value_mem_ptr).ok_or_else(|| {
          Error::moon_pdk(format!(
            "Failed to find memory for config value: {}",
            key
          ))
        })?;

        let value_str = value_mem.to_string().map_err(|_| {
          Error::moon_pdk(format!(
            "Failed to convert config memory to string: {}",
            key
          ))
        })?;

        Ok(Some(value_str))
      }
      Err(_) => Ok(None), // Handle error by returning None
    }
  }
  #[cfg(not(feature = "wasm"))]
  {
    let _ = key;
    Ok(None) // Configuration only available via Moon PDK in WASM environment
  }
}

/// Request atomic write via Moon host - Moon host handles temp file + rename
pub fn write_file_atomic(
  path: &str,
  content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  // WASM requests atomic write - Moon host handles the actual temp file + rename operation
  // Moon host ensures atomicity by writing to .tmp file then renaming
  write_file_to_host(path, content)
}

/// Helper to write content to a file on the Moon host.
pub fn write_file_to_host(
  path: &str,
  content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  #[cfg(feature = "wasm")]
  {
    use extism_pdk::Memory;
    let path_mem =
      Memory::new(&path).expect("Failed to create memory for path");
    let content_mem =
      Memory::new(&content).expect("Failed to create memory for content");
    match unsafe { write_file(path_mem.offset(), content_mem.offset()) } {
      Ok(0) => Ok(()),
      Ok(error_code) => Err(
        format!(
          "Failed to write file to host: {} (error code: {})",
          path, error_code
        )
        .into(),
      ),
      Err(e) => Err(
        format!("Host function call failed for file {}: {}", path, e).into(),
      ),
    }
  }
  #[cfg(not(feature = "wasm"))]
  {
    let _ = (path, content);
    return Err("File writing not available outside WASM environment - must run via Moon extension".into());
  }
}

/// Send storage update request to Moon host for atomic JSON updates
pub fn request_storage_update(
  storage_type: &str,
  updates: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
  let storage_request = serde_json::json!({
      "operation": "update_storage",
      "storage_type": storage_type,
      "updates": updates,
      "atomic": true
  });

  let request_json = serde_json::to_string(&storage_request)?;

  // WASM sends request to Moon host - Moon host handles read-modify-write atomically
  let response_path = format!(".moon/moonshine/{}.json", storage_type);
  write_file_to_host(&response_path, &request_json)
}

/// Request prompts.json update via Moon host
pub fn update_prompts_json(
  updates: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
  request_storage_update("prompts", updates)
}

/// Request training.json update via Moon host
pub fn update_training_json(
  updates: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
  request_storage_update("training", updates)
}

/// Generate Moon task execution command with session-based JSON communication
pub fn generate_moon_task_command(
  request: &MoonTaskRequest,
) -> Result<String, Box<dyn std::error::Error>> {
  // WASM cannot write files - Moon host handles request data via JSON protocol
  let _request_json = request.to_json()?;

  const EXTENSION_VERSION: &str = env!("CARGO_PKG_VERSION");

  // WASM cannot execute shell commands - returning JSON protocol for Moon host
  Ok(format!(
    r#"{{
    "action": "execute_moon_tasks",
    "extension_version": "{}",
    "session_id": "{}"
  }}"#,
    EXTENSION_VERSION, &request.session_id
  ))
}
/// Clean up old session directories (WASM cannot access filesystem)
pub fn cleanup_old_sessions(max_age_hours: u32) -> Result<u32, std::io::Error> {
  // WASM cannot access filesystem - Moon host should handle cleanup
  info!("Session cleanup requested: {} hours max age", max_age_hours);
  Ok(0) // Return 0 cleaned as WASM cannot perform cleanup
}

/// List session directories (WASM cannot access filesystem)
pub fn list_session_directories() -> Result<Vec<String>, std::io::Error> {
  // WASM cannot access filesystem - Moon host should provide session info
  info!("Session directories requested");
  Ok(vec![]) // Return empty list as WASM cannot access filesystem
}

/// Generate embedded Moon task definitions for extension distribution
pub fn generate_extension_task_definitions() -> String {
  // Extension version injected at compile time from Cargo.toml
  const EXTENSION_VERSION: &str = env!("CARGO_PKG_VERSION");

  info!(
    "Generating Moon task definitions for extension v{}",
    EXTENSION_VERSION
  );

  format!(
    r#"
# Moon Shine Extension Tasks v{}
# Real CLI integration tasks for TypeScript, ESLint, Prettier, and Claude Code

$schema: 'https://moonrepo.dev/schemas/project.json'

type: 'library'
language: 'rust'
platform: 'unknown'

metadata:
  extension_version: '{}'
  extension_name: 'moon-shine'

tasks:
  # Main moon-shine WASM extension
  shine:
    command: 'moon'
    args: ['ext', 'run', 'moon-shine', '--']
    inputs: ['**/*.{{ts,tsx,js,jsx}}']
    options:
      cache: false
      persistent: false
      runFromWorkspaceRoot: true
      affectedFiles: true

  # CI-friendly WASM reporting mode
  shine-report:
    command: 'moon'
    args: ['ext', 'run', 'moon-shine', '--', '--reporting-only']
    inputs: ['**/*.{{ts,tsx,js,jsx}}']
    options:
      cache: true
      persistent: false
      runFromWorkspaceRoot: true
      affectedFiles: true
"#,
    EXTENSION_VERSION, EXTENSION_VERSION,
  )
}
