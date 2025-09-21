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
pub mod ai_assistance; // AI enhancement and suggestion system
pub mod ai_code_fixer; // AI-powered code fixing
pub mod analysis;
pub mod cost_aware_ai_orchestrator; // Cost-aware AI orchestration for intelligent resource usage
pub mod code_analyzer; // OXC-based code analyzer with comprehensive linting and complexity analysis
pub mod complexity; // Comprehensive complexity analysis with enhanced Halstead metrics
pub mod config;
pub mod data; // Shared data handling components
pub mod dspy; // Embedded full DSPy framework <!-- TODO: Verify the completeness and fidelity of this DSPy implementation against the original Python framework. -->
pub mod error;
pub mod extension;
pub mod installation;
pub mod linter;
pub mod message_types; // Message and Chat structures
pub mod moon_pdk_interface; // Moon PDK communication interface
pub mod pattern_config; // Configurable pattern detection rules
pub mod prompt_optimizer; // DSPy-powered prompt optimization
pub mod prompts; // Embedded prompt management
pub mod provider_router; // AI provider routing and selection
pub mod security; // AST-based security vulnerability detection
pub mod storage; // Hybrid assemblage_kv + file persistence
pub mod token_usage; // LM token usage tracking
pub mod types; // Core data structures for moon-shine code analysis
pub mod workflow; // Multi-phase analysis workflow with feedback loops
pub mod orchestrator; // Unified hybrid orchestrator replacing sequential and parallel workflows
pub mod rust_workflow_engine; // Petgraph-based workflow engine for pure Rust orchestration
pub mod oxc_unified_workflow; // OXC-unified workflow replacing external tools
pub mod rule_storage; // WASM-safe ESLint rule storage with assemblage_kv
pub mod wasm_safe_linter; // WASM-compatible ESLint rule implementation
pub mod tool_replacements; // Complete toolchain replacements (TSC, ESLint, Prettier, TSDoc)
pub mod rules; // Modular MoonShine rule engine with AI enhancement

#[cfg(test)]
pub mod testing; // Comprehensive testing infrastructure for London, Chicago, and E2E methodologies
pub mod hybrid_linter; // Hybrid OXC + AI linter (WASM-safe)
pub mod oxc_rules_adapter; // WASM-safe adapter for OXC rule implementations
pub mod oxc_compatible_rules; // OXC-compatible rule implementations with AI enhancement
pub mod oxc_rules_migration; // Systematically migrated OXC rules with AI enhancement
pub mod oxc_performance_rules; // Performance-focused OXC rules with AI optimization guidance
pub mod oxc_string_rules; // String manipulation and text pattern rules
pub mod oxc_conditional_rules; // Conditional logic and control flow rules
pub mod oxc_object_rules; // Object and array pattern rules
pub mod oxc_function_rules; // Function definition and usage rules
pub mod oxc_variable_rules; // Variable declaration and usage rules
pub mod oxc_import_rules; // Import and export declaration rules
pub mod oxc_error_rules; // Error handling and exception rules
pub mod oxc_typescript_rules; // TypeScript-specific rules
pub mod oxc_security_rules; // Security and safety rules
pub mod oxc_advanced_security_rules; // Advanced security and cryptography rules
pub mod oxc_react_rules; // React-specific rules
pub mod oxc_accessibility_rules; // Accessibility (a11y) rules
pub mod oxc_es6_rules; // ES6+ Modern JavaScript rules
pub mod oxc_complexity_rules; // Code complexity and maintainability rules
pub mod oxc_nodejs_rules; // Node.js specific rules
pub mod oxc_async_rules; // Async/await and Promise rules
pub mod oxc_jsx_advanced_rules; // Advanced JSX and React rules
pub mod oxc_bestpractices_rules; // Best practices and style rules
pub mod oxc_css_rules; // CSS-in-JS and styling rules
pub mod oxc_testing_rules; // Testing framework rules
pub mod oxc_testing_framework_rules; // Testing framework specific rules (Jest, Vitest, Playwright, Cypress)
pub mod oxc_documentation_rules; // Documentation and comment rules
pub mod oxc_advanced_performance_rules; // Advanced performance optimization rules
pub mod oxc_vue_rules; // Vue.js specific rules
pub mod oxc_angular_rules; // Angular specific rules
pub mod oxc_build_tool_rules; // Build tool and bundling rules
pub mod oxc_build_tool_optimization_rules; // Advanced build tool optimization rules
pub mod oxc_database_orm_rules; // Database and ORM rules
pub mod oxc_database_optimization_rules; // Advanced database optimization rules
pub mod oxc_monorepo_workspace_rules; // Monorepo and workspace rules
pub mod oxc_state_management_rules; // State management rules (Redux, Zustand, Jotai)
pub mod oxc_graphql_rules; // GraphQL schema and resolver rules
pub mod oxc_testing_framework_integration_rules; // Testing framework integration rules (Jest, Vitest, Playwright)
pub mod oxc_devops_deployment_rules; // DevOps and deployment rules
pub mod oxc_pwa_modern_web_rules; // PWA and modern web platform rules
pub mod oxc_microfrontend_rules; // Micro-frontend architecture rules
pub mod oxc_edge_serverless_rules; // Edge computing and serverless rules
pub mod oxc_webrtc_realtime_rules; // WebRTC and real-time communication rules
pub mod oxc_web_payments_commerce_rules; // Web payments and commerce rules
pub mod oxc_enterprise_architecture_rules; // Enterprise architecture patterns rules
pub mod oxc_accessibility_i18n_rules; // Accessibility and internationalization rules
pub mod oxc_advanced_frameworks_rules; // Advanced framework integration rules (Svelte, SolidJS, Qwik, Astro)
pub mod oxc_cloud_native_rules; // Cloud-native and container orchestration rules
pub mod oxc_api_integration_rules; // API design and integration patterns rules
pub mod oxc_data_science_ml_rules; // Data science and ML workflow rules
pub mod oxc_blockchain_web3_rules; // Blockchain and Web3 development rules
pub mod oxc_performance_monitoring_rules; // Performance monitoring and observability rules
pub mod oxc_performance_profiling_rules; // Advanced performance profiling and benchmarking rules
pub mod oxc_gaming_interactive_rules; // Gaming and interactive media rules (WebGL, WebXR)
pub mod oxc_iot_embedded_rules; // IoT and embedded systems rules
pub mod oxc_ar_vr_development_rules; // AR/VR development rules (WebXR, immersive)
pub mod oxc_advanced_typescript_rules; // Advanced TypeScript rules (complex types, metaprogramming)
pub mod oxc_functional_programming_rules; // Functional programming rules (immutability, HOF)
pub mod oxc_design_systems_rules; // Design systems and component library rules
pub mod oxc_enterprise_patterns_rules; // Enterprise patterns rules (DDD, CQRS, Event Sourcing)
pub mod unified_rule_registry; // Unified registry for all OXC-compatible WASM rules
pub mod sunlinter_integration; // SunLinter JavaScript rules integration with behavioral analysis
pub mod sunlinter_rule_converter; // Systematic conversion framework for 192 SunLinter rules
pub mod sunlinter_plus_plus; // Superior SunLinter++ engine with enhanced capabilities

// Re-exports for convenience
pub use analysis::MoonShineResponse;
pub use config::{MoonShineArgs, MoonShineConfig};
pub use data::{Example, Prediction}; // Add data types re-export
pub use dspy::signature_macro::*; // Re-export signature macros at crate level

// Tool replacement re-exports
pub use rule_storage::{RuleStorage, RuleConfig, RuleSeverity, RuleCategory};
pub use tool_replacements::{ToolChainReplacements, TypeScriptCompilationResult, ESLintReplacementResult, PrettierReplacementResult};
pub use wasm_safe_linter::{WasmSafeLinter, WasmSafeLintResult};
pub use extension::{ExecuteExtensionInput, ExtensionManifest};
// MoonShine rule engine exports
pub use rules::{MoonShineRuleEngine, MoonShineRule, MoonShineRuleCategory, AIEnhancer};
// Unified rule registry exports
pub use unified_rule_registry::{UnifiedRuleRegistry, RuleRegistryStats, RuleSettings, WasmRuleDiagnostic, AiSuggestion, WasmRuleCategory, WasmFixStatus, WasmRule, EnhancedWasmRule};
// Workflow engine exports
pub use rust_workflow_engine::{RustWorkflowEngine, WorkflowStep, WorkflowResult, StepAction};

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

/// Main entry point for Moon extension - follows official PDK specification
/// Coordinates AI-powered code optimization with Moon task orchestration
#[plugin_fn]
pub fn execute_extension(
  Json(input): Json<ExecuteExtensionInput>,
) -> FnResult<()> {
  extension::execute_extension_logic(Json(input))
}
