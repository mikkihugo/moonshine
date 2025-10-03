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

use extism_pdk::*;

#[cfg(all(test, not(feature = "wasm")))]
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
/// Provides AI-powered assistance and suggestion generation.
pub mod ai_assistance;
/// Handles automated code fixing based on AI suggestions.
pub mod ai_code_fixer;
/// Core analysis utilities and data structures.
pub mod analysis;
/// Implements a cost-aware AI orchestrator for intelligent resource management.
pub mod cost_aware_ai_orchestrator;
/// An OXC-based code analyzer for linting and complexity analysis.
pub mod code_analyzer;
/// Provides tools for comprehensive code complexity analysis, including Halstead metrics.
pub mod complexity;
/// Manages configuration for the `moon-shine` extension.
pub mod config;
/// Defines shared data structures for handling examples and predictions.
pub mod data;
/// An embedded implementation of the DSPy framework for prompt optimization.
/// TODO: Verify the completeness and fidelity of this DSPy implementation against the original Python framework.
pub mod dspy;
/// Defines custom error types for the crate.
pub mod error;
/// Contains the core logic for the Moon extension, including argument parsing and execution.
pub mod extension;
/// Manages installation and setup of required tools and dependencies.
pub mod installation;
/// The core linter engine.
pub mod linter;
/// Defines structures for messages and chat interactions.
pub mod message_types;
/// Provides an interface for communicating with the Moon PDK.
pub mod moon_pdk_interface;
/// Manages configurable rules for pattern detection.
pub mod pattern_config;
/// A DSPy-powered engine for prompt optimization.
pub mod prompt_optimizer;
/// Manages and provides embedded prompts for AI interactions.
pub mod prompts;
/// Implements routing and selection of different AI providers.
pub mod provider_router;
/// Provides tools for AST-based security vulnerability detection.
pub mod security;
/// A hybrid storage system using `assemblage_kv` and file persistence.
pub mod storage;
/// Tracks language model token usage for cost and performance monitoring.
pub mod token_usage;
/// Defines core data structures for code analysis.
pub mod types;
/// Implements a multi-phase analysis workflow with feedback loops.
pub mod workflow;
/// A unified hybrid orchestrator that replaces sequential and parallel workflows.
pub mod orchestrator;
/// A Petgraph-based workflow engine for pure Rust orchestration.
pub mod rust_workflow_engine;
/// A WASM-safe storage for ESLint rules using `assemblage_kv`.
pub mod rule_storage;
/// A WASM-compatible implementation of an ESLint rule engine.
pub mod wasm_safe_linter;
/// Provides complete replacements for the TypeScript/JavaScript toolchain (TSC, ESLint, Prettier, TSDoc).
pub mod tool_replacements;
/// A modular rule engine for MoonShine with AI enhancement capabilities.
pub mod rules;
/// A comprehensive collection of OXC-compatible rules organized by domain.
pub mod oxc_rules;

/// Comprehensive testing infrastructure for London, Chicago, and E2E methodologies.
#[cfg(test)]
pub mod testing;
/// A hybrid linter combining OXC static analysis with AI enhancement, designed to be WASM-safe.
pub mod hybrid_linter;
/// A unified registry for all OXC-compatible WASM rules.
pub mod unified_rule_registry;
/// Integration with SunLinter for JavaScript rules and behavioral analysis.
pub mod sunlinter_integration;
/// A systematic framework for converting the 192 SunLinter rules to a compatible format.
pub mod sunlinter_rule_converter;
/// An enhanced SunLinter++ engine with superior capabilities.
pub mod sunlinter_plus_plus;

// Re-exports for convenience
/// Re-exports `MoonShineResponse` for easy access.
pub use analysis::MoonShineResponse;
/// Re-exports `MoonShineArgs` and `MoonShineConfig` for easy access.
pub use config::{MoonShineArgs, MoonShineConfig};
/// Re-exports core data types `Example` and `Prediction`.
pub use data::{Example, Prediction};
/// Re-exports all signature macros from the `dspy` module for convenient use.
pub use dspy::signature_macro::*;

// Tool replacement re-exports
/// Re-exports core components from the `rule_storage` module.
pub use rule_storage::{RuleStorage, RuleConfig, RuleSeverity, RuleCategory};
/// Re-exports core components from the `tool_replacements` module.
pub use tool_replacements::{
    ESLintReplacementResult, PrettierReplacementResult, ToolChainReplacements,
    TypeScriptCompilationResult,
};
/// Re-exports core components from the `wasm_safe_linter` module.
pub use wasm_safe_linter::{WasmSafeLintResult, WasmSafeLinter};
/// Re-exports `ExecuteExtensionInput` and `ExtensionManifest` for easy access.
pub use extension::{ExecuteExtensionInput, ExtensionManifest};
// MoonShine rule engine exports
/// Re-exports core components from the `rules` module.
pub use rules::{AIEnhancer, MoonShineRule, MoonShineRuleCategory, MoonShineRuleEngine};
// Unified rule registry exports
/// Re-exports core components from the `unified_rule_registry` module.
pub use unified_rule_registry::{
    AiSuggestion, EnhancedWasmRule, RuleRegistryStats, RuleSettings, UnifiedRuleRegistry, WasmFixStatus,
    WasmRule, WasmRuleCategory, WasmRuleDiagnostic,
};
// Workflow engine exports
/// Re-exports core components from the `rust_workflow_engine` module.
pub use rust_workflow_engine::{RustWorkflowEngine, StepAction, WorkflowResult, WorkflowStep};

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
        config_schema: Some(config::create_config_schema()),
    }))
}

/// The main entry point for the `moon-shine` Moon extension.
///
/// This function is called by Moon to execute the extension's logic. It receives
/// input from the Moon environment, deserialized into an `ExecuteExtensionInput` struct.
/// The core logic is delegated to the `execute_extension_logic` function in the `extension` module.
///
/// This function adheres to the official Moon PDK specification for extensions.
///
/// # Arguments
///
/// * `input` - A JSON object containing the input data for the extension,
///   including arguments and configuration.
///
/// @category coordination
/// @safe team
/// @mvp core
/// @complexity low
/// @since 1.0.0
#[plugin_fn]
pub fn execute_extension(Json(input): Json<ExecuteExtensionInput>) -> FnResult<()> {
    extension::execute_extension_logic(Json(input))
}
