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

/// Represents the input for executing a command on the Moon host.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecCommandInput {
    /// The command to execute.
    pub command: String,
    /// A list of arguments for the command.
    pub args: Vec<String>,
    /// A map of environment variables to set for the command.
    pub env: std::collections::HashMap<String, String>,
    /// The working directory in which to execute the command.
    pub working_dir: Option<String>,
}

/// Represents the output of an executed command from the Moon host.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecCommandOutput {
    /// The exit code of the command.
    pub exit_code: i32,
    /// The standard output of the command.
    pub stdout: String,
    /// The standard error of the command.
    pub stderr: String,
}

/// Executes a command via the Moon host.
///
/// This function is a wrapper around the `exec_command` host function.
///
/// # Arguments
///
/// * `input` - An `ExecCommandInput` struct containing the command and its arguments.
///
/// # Returns
///
/// A `Result` containing an `ExecCommandOutput` on success, or an error if the
/// command fails or if not in a WASM environment.
pub fn execute_command(input: ExecCommandInput) -> Result<ExecCommandOutput, Box<dyn std.error::Error>> {
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

/// Reads the content of a file via the Moon host.
///
/// This function is a wrapper around the `read_file` host function.
///
/// # Arguments
///
/// * `path` - The path to the file to read.
///
/// # Returns
///
/// A `Result` containing the file content as a `String` on success, or an error
/// if the file cannot be read or if not in a WASM environment.
pub fn read_file_content(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    #[cfg(feature = "wasm")]
    {
        let path_data = extism_pdk::Memory::new(path)?;
        match unsafe { read_file(path_data.offset()) } {
            0 => Err(format!("Failed to read file: {}", path).into()),
            content_ptr => {
                let content_memory = extism_pdk::Memory::find(content_ptr).ok_or("Invalid memory pointer from read_file")?;
                let content = String::from_utf8(content_memory.to_vec()).map_err(|e| format!("Invalid UTF-8 in file {}: {}", path, e))?;
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

/// Checks if a file exists via the Moon host.
///
/// This function is a wrapper around the `file_exists` host function.
///
/// # Arguments
///
/// * `path` - The path to the file to check.
///
/// # Returns
///
/// A `Result` containing `true` if the file exists, `false` otherwise, or an error
/// if not in a WASM environment.
pub fn check_file_exists(path: &str) -> Result<bool, Box<dyn std::error::Error>> {
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

/// Lists the contents of a directory via the Moon host.
///
/// This function is a wrapper around the `list_directory` host function.
///
/// # Arguments
///
/// * `path` - The path to the directory to list.
///
/// # Returns
///
/// A `Result` containing a `Vec<String>` of file and directory names on success,
/// or an error if the directory cannot be listed or if not in a WASM environment.
pub fn list_directory_contents(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    #[cfg(feature = "wasm")]
    {
        let path_data = extism_pdk::Memory::new(path)?;
        match unsafe { list_directory(path_data.offset()) } {
            0 => Err(format!("Failed to list directory: {}", path).into()),
            content_ptr => {
                let content_memory = extism_pdk::Memory::find(content_ptr).ok_or("Invalid memory pointer from list_directory")?;
                let content = String::from_utf8(content_memory.to_vec()).map_err(|e| format!("Invalid UTF-8 in directory listing {}: {}", path, e))?;

                // Parse newline-separated file list
                let files = content.lines().filter(|line| !line.trim().is_empty()).map(|line| line.to_string()).collect();
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

/// Defines the configuration for the AI-powered linter when run as a Moon task.
///
/// This struct includes settings for enabling AI features, selecting models,
/// and controlling concurrency and rate limiting.
/// <!-- TODO: The `AiLinterConfig` has many fields. Consider grouping related fields into smaller structs (e.g., `RateLimitingConfig`, `ClaudeConfig`) to improve organization and readability. -->
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiLinterConfig {
    /// Whether to enable Claude AI for fixing issues.
    pub enable_claude_ai: bool,
    /// Whether to enable deterministic semantic checks before AI fixes.
    pub enable_semantic_checks: bool,
    /// The Claude model to use for AI-powered fixes.
    pub claude_model: String,
    /// The maximum processing time in seconds for a single file.
    pub max_processing_time: u32,
    /// The quality threshold for AI suggestions.
    pub quality_threshold: f32,

    // Concurrency and rate limiting controls
    /// The maximum number of concurrent requests to the AI model.
    pub max_concurrent_requests: u32,
    /// The number of files to process in a single batch.
    pub batch_size: u32,
    /// The maximum number of requests per minute to the AI model.
    pub rate_limit_per_minute: u32,
    /// The maximum number of tokens per request to the AI model.
    pub max_tokens_per_request: u32,
    /// The number of times to retry a failed request to the AI model.
    pub retry_attempts: u32,
    /// The delay in milliseconds between retry attempts.
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
            max_concurrent_requests: 3,    // Limit to 3 concurrent Claude calls
            batch_size: 5,                 // Process 5 files per batch
            rate_limit_per_minute: 20,     // Max 20 requests per minute
            max_tokens_per_request: 16384, // Realistic token limit for big code files
            retry_attempts: 3,             // Retry failed requests 3 times
            retry_delay_ms: 1000,          // 1 second delay between retries
        }
    }
}

/// Represents a semantic warning generated by AI analysis.
///
/// These warnings highlight potential issues that may not be caught by
/// traditional linters but are identified by the AI model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticWarning {
    /// The warning message.
    pub message: String,
    /// The line number where the warning occurs.
    pub line: u32,
    /// The column number where the warning occurs.
    pub column: u32,
    /// The severity of the warning (e.g., "high", "medium", "low").
    pub severity: String,
    /// The category of the warning (e.g., "performance", "security", "best-practice").
    pub category: String,
    /// The AI's confidence score for this warning (0.0 to 1.0).
    pub ai_confidence: f32,
}

/// Defines the configuration for a Moon task execution.
///
/// This struct specifies which tools to run (TypeScript, ESLint, Prettier, etc.)
/// and how they should be configured.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoonTaskConfig {
    /// Whether to enable the strictest TypeScript checking.
    pub enable_strict_typescript: bool,
    /// Whether to enable ESLint with an existing configuration.
    pub enable_eslint: bool,
    /// Whether to enable Prettier for code formatting.
    pub enable_prettier: bool,
    /// Whether to enable TSDoc analysis and improvement.
    pub enable_tsdoc: bool,
    /// Whether to enable Claude AI for fixing issues.
    pub enable_claude_ai: bool,
    /// Whether to enable deterministic semantic checks before AI fixes.
    pub enable_semantic_checks: bool,
    /// A custom prompt template for Claude AI.
    pub claude_prompt_template: Option<String>,
    /// The maximum processing time in seconds for the task.
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

/// Represents the JSON response sent from a Moon task back to the WASM extension.
///
/// This struct encapsulates the results of a task's execution, including success
/// status, processing time, and task-specific data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoonTaskResponse {
    /// A unique identifier for the session, used for correlating requests and responses.
    pub session_id: String,
    /// The name of the task that generated this response.
    pub task_name: String,
    /// Whether the task executed successfully.
    pub success: bool,
    /// An optional error message if the task failed.
    pub error: Option<String>,
    /// A struct containing the specific results of the task.
    pub results: MoonTaskResults,
    /// The time it took to process the task, in milliseconds.
    pub processing_time_ms: u64,
    /// The timestamp when the task completed, in RFC 3339 format.
    pub completed_at: String,
}

/// Contains the specific results from the various tools run within a Moon task.
///
/// Each field is optional, allowing tasks to run a subset of the available tools.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoonTaskResults {
    /// The results of the TypeScript compilation.
    pub typescript: Option<TypeScriptResults>,
    /// The results of the ESLint analysis.
    pub eslint: Option<ESLintResults>,
    /// The results of the Prettier formatting.
    pub prettier: Option<PrettierResults>,
    /// The results of the TSDoc analysis.
    pub tsdoc: Option<TSDocResults>,
    /// The results of the Claude AI processing.
    pub claude: Option<ClaudeResults>,
    /// A summary of the deterministic semantic validation.
    pub semantic_validation: Option<SemanticValidationResults>,
}

/// Contains the results of a TypeScript compilation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScriptResults {
    /// Whether the compilation was successful.
    pub compilation_success: bool,
    /// A list of compilation errors.
    pub errors: Vec<TypeScriptError>,
    /// A list of compilation warnings.
    pub warnings: Vec<TypeScriptWarning>,
    /// A list of the configuration flags used for the compilation.
    pub config_flags: Vec<String>,
}

/// Represents a single error from the TypeScript compiler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScriptError {
    /// The file where the error occurred.
    pub file: String,
    /// The line number of the error.
    pub line: u32,
    /// The column number of the error.
    pub column: u32,
    /// The TypeScript error code (e.g., "TS2322").
    pub code: String,
    /// The error message.
    pub message: String,
    /// The severity of the error.
    pub severity: String,
}

/// Represents a single warning from the TypeScript compiler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScriptWarning {
    /// The file where the warning occurred.
    pub file: String,
    /// The line number of the warning.
    pub line: u32,
    /// The column number of the warning.
    pub column: u32,
    /// The TypeScript warning code.
    pub code: String,
    /// The warning message.
    pub message: String,
}

/// Contains the results of an ESLint analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESLintResults {
    /// Whether the ESLint execution was successful.
    pub success: bool,
    /// A list of the files that were processed by ESLint.
    pub files_processed: Vec<String>,
    /// A list of the issues found by ESLint.
    pub issues: Vec<ESLintIssue>,
    /// The number of auto-fixes that were applied.
    pub fixes_applied: u32,
    /// The path to the ESLint configuration file that was used.
    pub config_file: String,
}

/// Represents a single issue found by ESLint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESLintIssue {
    /// The file where the issue occurred.
    pub file: String,
    /// The line number of the issue.
    pub line: u32,
    /// The column number of the issue.
    pub column: u32,
    /// The ID of the ESLint rule that was violated.
    pub rule_id: String,
    /// The issue message.
    pub message: String,
    /// The severity of the issue (e.g., "error", "warning").
    pub severity: String,
    /// Whether the issue is fixable by ESLint's --fix option.
    pub fixable: bool,
}

/// Contains the results of a Prettier formatting run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrettierResults {
    /// Whether the formatting was successful.
    pub success: bool,
    /// A list of the files that were formatted.
    pub files_formatted: Vec<String>,
    /// Whether any changes were made to the files.
    pub changes_made: bool,
    /// The path to the Prettier configuration file that was used.
    pub config_file: Option<String>,
}

/// Contains the results of a TSDoc analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TSDocResults {
    /// Whether the analysis was successful.
    pub success: bool,
    /// The total number of functions found.
    pub total_functions: u32,
    /// The number of functions that have documentation.
    pub documented_functions: u32,
    /// The documentation coverage percentage.
    pub coverage_percentage: f64,
    /// A list of items that are missing documentation.
    pub missing_docs: Vec<String>,
    /// A list of suggested improvements for the documentation.
    pub improvements: Vec<String>,
}

/// Contains the results of processing by the Claude AI model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeResults {
    /// Whether the Claude AI processing was successful.
    pub success: bool,
    /// The content of the file after being fixed by the AI.
    pub fixed_content: Option<String>,
    /// A list of improvements made by the AI.
    pub improvements: Vec<String>,
    /// The number of issues that were resolved by the AI.
    pub issues_resolved: u32,
    /// The time it took for the AI to process the request, in milliseconds.
    pub claude_processing_time_ms: u64,
    /// The token usage for the AI request.
    pub token_usage: Option<ClaudeTokenUsage>,
}

/// Contains the results of a semantic validation check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticValidationResults {
    /// Whether the semantic validation passed.
    pub passed: bool,
    /// The number of warnings that were checked.
    pub warnings_checked: u32,
    /// A list of warnings that could not be resolved.
    pub unresolved_warnings: Vec<SemanticValidationWarning>,
}

/// Represents a single warning from the semantic validation process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticValidationWarning {
    /// The warning code.
    pub code: String,
    /// The warning message.
    pub message: String,
    /// The severity of the warning.
    pub severity: Option<String>,
    /// The pattern that triggered the warning.
    pub pattern: String,
}

/// Represents the token usage for a request to the Claude AI model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeTokenUsage {
    /// The number of input tokens used.
    pub input_tokens: u32,
    /// The number of output tokens generated.
    pub output_tokens: u32,
    /// The total cost of the request in USD.
    pub total_cost_usd: f64,
}

/// Gets a configuration value from the Moon host.
///
/// # Arguments
///
/// * `key` - The key of the configuration value to retrieve.
///
/// # Returns
///
/// An `Option<String>` containing the configuration value if it exists, or `None` otherwise.
pub fn get_moon_config(key: &str) -> Option<String> {
    #[cfg(feature = "wasm")]
    {
        use extism_pdk::Memory;
        let key_mem = Memory::new(&key).expect("Failed to create memory for key");
        match unsafe { get_moon_config_value(key_mem.offset()) } {
            Ok(0) => None,
            Ok(value_mem_ptr) => {
                let value_mem = Memory::find(value_mem_ptr).expect("Failed to find memory");
                Some(value_mem.to_string().expect("Failed to convert memory to string"))
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

/// A safe version of `get_moon_config` that returns a `Result` to handle errors.
///
/// # Arguments
///
/// * `key` - The key of the configuration value to retrieve.
///
/// # Returns
///
/// A `Result` containing an `Option<String>` with the configuration value, or an `Error` if something goes wrong.
pub fn get_moon_config_safe(key: &str) -> crate::error::Result<Option<String>> {
    #[cfg(feature = "wasm")]
    {
        use crate::error::Error;
        use extism_pdk::Memory;

        let key_mem = Memory::new(&key).map_err(|e| {
            error!("Memory allocation failed for config key '{}': {}", key, e);
            Error::moon_pdk(format!("Failed to create memory for config key: {}", key))
        })?;

        match unsafe { get_moon_config_value(key_mem.offset()) } {
            Ok(0) => Ok(None), // 0 indicates no value found
            Ok(value_mem_ptr) => {
                let value_mem = Memory::find(value_mem_ptr).ok_or_else(|| Error::moon_pdk(format!("Failed to find memory for config value: {}", key)))?;

                let value_str = value_mem
                    .to_string()
                    .map_err(|_| Error::moon_pdk(format!("Failed to convert config memory to string: {}", key)))?;

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

/// Requests an atomic write operation via the Moon host.
///
/// The host is responsible for handling the temporary file and rename operation to ensure atomicity.
///
/// # Arguments
///
/// * `path` - The path to the file to write.
/// * `content` - The content to write to the file.
///
/// # Returns
///
/// A `Result` that is empty on success, or an error if the write fails.
pub fn write_file_atomic(path: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    // WASM requests atomic write - Moon host handles the actual temp file + rename operation
    // Moon host ensures atomicity by writing to .tmp file then renaming
    write_file_to_host(path, content)
}

/// Writes content to a file on the Moon host.
///
/// This function is a wrapper around the `write_file` host function.
///
/// # Arguments
///
/// * `path` - The path to the file to write.
/// * `content` - The content to write to the file.
///
/// # Returns
///
/// A `Result` that is empty on success, or an error if the write fails.
pub fn write_file_to_host(path: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "wasm")]
    {
        use extism_pdk::Memory;
        let path_mem = Memory::new(&path).expect("Failed to create memory for path");
        let content_mem = Memory::new(&content).expect("Failed to create memory for content");
        match unsafe { write_file(path_mem.offset(), content_mem.offset()) } {
            Ok(0) => Ok(()),
            Ok(error_code) => Err(format!("Failed to write file to host: {} (error code: {})", path, error_code).into()),
            Err(e) => Err(format!("Host function call failed for file {}: {}", path, e).into()),
        }
    }
    #[cfg(not(feature = "wasm"))]
    {
        let _ = (path, content);
        return Err("File writing not available outside WASM environment - must run via Moon extension".into());
    }
}

/// Sends a storage update request to the Moon host for atomic JSON updates.
///
/// The host is responsible for handling the read-modify-write operation atomically.
///
/// # Arguments
///
/// * `storage_type` - The type of storage to update (e.g., "prompts", "training").
/// * `updates` - A `serde_json::Value` containing the updates to apply.
///
/// # Returns
///
/// A `Result` that is empty on success, or an error if the update fails.
pub fn request_storage_update(storage_type: &str, updates: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
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

/// Requests an update to the `prompts.json` file via the Moon host.
///
/// # Arguments
///
/// * `updates` - A `serde_json::Value` containing the updates to apply.
///
/// # Returns
///
/// A `Result` that is empty on success, or an error if the update fails.
pub fn update_prompts_json(updates: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    request_storage_update("prompts", updates)
}

/// Requests an update to the `training.json` file via the Moon host.
///
/// # Arguments
///
/// * `updates` - A `serde_json::Value` containing the updates to apply.
///
/// # Returns
///
/// A `Result` that is empty on success, or an error if the update fails.
pub fn update_training_json(updates: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    request_storage_update("training", updates)
}

/// Generates a Moon task execution command with session-based JSON communication.
///
/// Since the WASM extension cannot execute shell commands directly, this function
/// returns a JSON protocol string for the Moon host to interpret.
///
/// # Arguments
///
/// * `request` - A `MoonTaskRequest` to be serialized into the command.
///
/// # Returns
///
/// A `Result` containing the JSON protocol string for the Moon host on success, or an error on failure.
pub fn generate_moon_task_command(request: &MoonTaskRequest) -> Result<String, Box<dyn std::error::Error>> {
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
/// Requests the Moon host to clean up old session directories.
///
/// Since the WASM extension cannot access the filesystem, this function logs a
/// request that the host can act upon.
///
/// # Arguments
///
/// * `max_age_hours` - The maximum age in hours for a session to be kept.
///
/// # Returns
///
/// A `Result` containing the number of cleaned sessions (always 0 in WASM), or an error.
pub fn cleanup_old_sessions(max_age_hours: u32) -> Result<u32, std::io::Error> {
    // WASM cannot access filesystem - Moon host should handle cleanup
    info!("Session cleanup requested: {} hours max age", max_age_hours);
    Ok(0) // Return 0 cleaned as WASM cannot perform cleanup
}

/// Requests a list of session directories from the Moon host.
///
/// Since the WASM extension cannot access the filesystem, this function returns
/// an empty list and logs a request for the host.
///
/// # Returns
///
/// A `Result` containing a `Vec<String>` of session directories, or an error.
pub fn list_session_directories() -> Result<Vec<String>, std::io::Error> {
    // WASM cannot access filesystem - Moon host should provide session info
    info!("Session directories requested");
    Ok(vec![]) // Return empty list as WASM cannot access filesystem
}

/// Generates embedded Moon task definitions for distribution with the extension.
///
/// # Returns
///
/// A `String` containing the Moon task definitions in YAML format.
pub fn generate_extension_task_definitions() -> String {
    // Extension version injected at compile time from Cargo.toml
    const EXTENSION_VERSION: &str = env!("CARGO_PKG_VERSION");

    info!("Generating Moon task definitions for extension v{}", EXTENSION_VERSION);

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
