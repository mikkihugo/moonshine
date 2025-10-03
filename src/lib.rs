#![recursion_limit = "256"]

//! # Moon Shine: AI-Powered Code Optimization for Moonrepo
//!
//! `moon-shine` is a sophisticated WebAssembly (WASM) extension designed for the `moonrepo` task orchestration system.
//! It integrates advanced AI capabilities, specifically leveraging Anthropic's Claude CLI and DSPy-powered prompt optimization,
//! to elevate TypeScript and JavaScript code to production excellence.
//!
//! This crate acts as a coordination layer, orchestrating multi-phase code analysis, linting, and automated fixing
//! through a hybrid architecture. WASM handles the core logic, configuration, and communication protocol,
//! while delegating heavy-lifting tasks (like running ESLint, TypeScript compiler, or Claude CLI) to native Moon tasks.
//!
//! ## Key Features:
//! - **AI-Powered Code Fixing**: Utilizes Claude AI for intelligent code suggestions and automated fixes.
//! - **COPRO Optimization**: Implements Collaborative Prompt Optimization (COPRO) for systematic prompt engineering
//!   to improve AI effectiveness.
//! - **Multi-Phase Analysis Workflow**: Orchestrates a series of analysis phases (e.g., TypeScript compilation, ESLint,
//!   TSDoc analysis, security checks) with feedback loops to achieve convergence on high-quality code.
//! - **Intelligent Provider Routing**: Dynamically selects the best AI model (Claude, Gemini, OpenAI Codex) based on
//!   task requirements and provider capabilities.
//! - **Session-Based Debugging**: Manages temporary session directories for detailed debugging and analysis of AI interactions.
//! - **Moon Integration**: Seamlessly integrates with Moon's task orchestration, caching, and configuration system.
//!
//! ## Architecture Overview:
//! The core logic resides within the WASM module, which communicates with the Moon host via Extism PDK.
//! This enables `moon-shine` to be highly portable and efficient, offloading computationally intensive operations
//! to the host environment while maintaining control over the overall workflow.
//!
//! ## Usage:
//! `moon-shine` is primarily invoked as a Moon extension. Refer to the `README.md` for detailed CLI usage and configuration.
//!
//! ## Custom Tags Used in Documentation:
//! - `@category`: Classifies the module/function's primary domain (e.g., `coordination`, `ai-integration`, `configuration`).
//! - `@safe`: Indicates the SAFe 6.0 level of the component (e.g., `team`, `program`, `large-solution`, `portfolio`).
//! - `@mvp`: Classifies the component's MVP status (e.g., `core`, `extension`, `future`).
//! - `@complexity`: Rates the complexity of the component (e.g., `low`, `medium`, `high`, `critical`).
//! - `@since`: Specifies the version when the component was introduced.

use crate::moon_host::{plugin_fn, FnResult, Json};

#[cfg(test)]
mod test_host_stubs;

// Moon PDK test utilities - imported but may appear unused
// since they're used in other modules' tests via pub use
#[cfg(test)]
#[allow(unused_imports)]
use moon_pdk_test_utils::*;
#[cfg(test)]
#[allow(unused_imports)]
pub use starbase_sandbox::create_empty_sandbox;

// Core modules
#[macro_use]
pub mod moon_log; // Moon PDK logging adapters
pub mod adaptive_workflow_planner; // Adaptive workflow planning and AI strategy selection
pub mod ai_assistance; // AI enhancement and suggestion system
pub mod ai_code_fixer; // AI-powered code fixing
pub mod analysis;
pub mod config;
pub mod data; // Shared data handling components
pub mod dspy; // Embedded full DSPy framework <!-- TODO: Verify the completeness and fidelity of this DSPy implementation against the original Python framework. -->
pub mod error;
pub mod extension;
pub mod installation;
pub mod oxc_adapter; // Modern OXC + AI behavioral linting system
                     // pub mod linter; // Disabled - replaced by Biome + AI analysis system
pub mod message_types; // Message and ConversationHistory structures
pub mod moon_host; // Centralized re-export of required Extism symbols
pub mod moon_pdk_interface; // Moon PDK communication interface
                            // pub mod pattern_config; // Legacy pattern config - replaced by Biome + AI system
pub mod prompts; // Embedded prompt management
pub mod provider_router; // AI provider routing and selection
pub mod rule_registry; // Rule registry and metadata management
pub mod rule_types; // Modern rule types for Biome + AI system
pub mod rulebase; // JSON-based rulebase and execution scaffolding
pub mod storage; // Hybrid assemblage_kv + file persistence
pub mod telemetry; // Telemetry logging for workflow runs
pub mod token_usage; // LM token usage tracking
pub mod types; // Core data structures for moon-shine code analysis
pub mod workflow; // Sequential workflow engine for orchestrated analysis pipelines
pub mod multi_language_analyzer; // Multi-language analysis system (TypeScript/JavaScript + Rust)
pub mod javascript_typescript_linter; // JavaScript/TypeScript linting using OXC
pub mod templates;
pub mod tsconfig; // TypeScript configuration resolution utilities
pub mod tsdoc; // Lightweight TSDoc analysis helpers // Rule generation templates


// Re-exports for convenience
pub use analysis::MoonShineResponse;
pub use config::{MoonShineArgs, MoonShineConfig};
pub use data::{Example, Prediction}; // Add data types re-export
// Legacy exports removed - using modern Biome + AI system
pub use extension::{ExecuteExtensionInput, ExtensionManifest};
pub use oxc_adapter::{AiBehavioralAnalyzer, MultiEngineAnalyzer, MultiEngineConfig, OxcAdapter};
pub use multi_language_analyzer::{LanguageConfig, MultiLanguageAnalyzer, SupportedLanguage};
pub use rule_types::{FixStatus, RuleCategory, RuleMetadata, RuleRegistryStats, RuleSeverity};
// Legacy workflow exports removed

/// Registers the `moon-shine` extension with the Moon task orchestration system.
///
/// This function is the entry point for Moon to discover and understand the capabilities
/// of the `moon-shine` WASM extension. It provides essential metadata such as the
/// extension's name, description, version, author, homepage, and its configuration schema.
///
/// The configuration schema is crucial for Moon to validate and provide autocompletion
/// for `moon-shine` specific settings within the `workspace.yml` or `project.yml` files.
///
/// @category coordination
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
#[cfg(not(test))]
#[plugin_fn]
pub fn register_extension() -> FnResult<Json<ExtensionManifest>> {
    Ok(Json(ExtensionManifest {
        name: "moon-shine".to_string(),
        description: "AI-powered TypeScript/JavaScript linter with COPRO optimization and pattern learning".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        author: Some("PrimeCode Moon Extensions".to_string()),
        homepage: Some("https://github.com/primecode/zenflow/tree/main/packages/tools/moon-shine".to_string()),
        config_schema: Some(serde_json::Value::String(config::create_config_schema())),
    }))
}

/// Main entry point for Moon extension - follows official PDK specification
/// Coordinates AI-powered code optimization with Moon task orchestration
#[plugin_fn]
pub fn execute_extension(Json(input): Json<ExecuteExtensionInput>) -> FnResult<()> {
    extension::execute_extension_logic(Json(input))
}
