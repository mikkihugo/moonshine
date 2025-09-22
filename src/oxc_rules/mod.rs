//! # OXC Rules Module
//!
//! Comprehensive collection of OXC-compatible rules for code quality, security,
//! and best practices. These rules use the OXC (JavaScript Oxidation Compiler)
//! toolchain for fast, semantic analysis of TypeScript and JavaScript code.
//!
//! ## Rule Categories
//! - **Accessibility**: `oxc_accessibility_*` - Web accessibility guidelines
//! - **Security**: `oxc_*_security_*` - Security vulnerability detection
//! - **Performance**: `oxc_*_performance_*` - Performance optimization rules
//! - **Frameworks**: `oxc_*_framework_*` - Framework-specific best practices
//! - **TypeScript**: `oxc_typescript_*` - TypeScript-specific rules
//! - **Testing**: `oxc_testing_*` - Testing framework integration
//!
//! @category oxc-rules
//! @safe program  
//! @mvp core
//! @complexity high
//! @since 2.0.0

// Core rule modules
pub mod oxc_accessibility_rules;
pub mod oxc_accessibility_i18n_rules;
pub mod oxc_security_rules;
pub mod oxc_advanced_security_rules;
pub mod oxc_performance_rules;
pub mod oxc_advanced_performance_rules;
pub mod oxc_performance_monitoring_rules;
pub mod oxc_performance_profiling_rules;

// Framework-specific rules
pub mod oxc_angular_rules;
pub mod oxc_react_rules;
pub mod oxc_vue_rules;
pub mod oxc_advanced_frameworks_rules;
pub mod oxc_jsx_advanced_rules;

// Language and syntax rules
pub mod oxc_typescript_rules;
pub mod oxc_advanced_typescript_rules;
pub mod oxc_es6_rules;
pub mod oxc_async_rules;
pub mod oxc_function_rules;
pub mod oxc_variable_rules;
pub mod oxc_object_rules;
pub mod oxc_string_rules;
pub mod oxc_error_rules;
pub mod oxc_conditional_rules;
pub mod oxc_import_rules;

// Testing and quality assurance
pub mod oxc_testing_rules;
pub mod oxc_testing_framework_rules;
pub mod oxc_testing_framework_integration_rules;

// Build and development tools
pub mod oxc_build_tool_rules;
pub mod oxc_build_tool_optimization_rules;
pub mod oxc_css_rules;
pub mod oxc_documentation_rules;

// Enterprise and architecture patterns
pub mod oxc_enterprise_patterns_rules;
pub mod oxc_enterprise_architecture_rules;
pub mod oxc_monorepo_workspace_rules;
pub mod oxc_microfrontend_rules;
pub mod oxc_design_systems_rules;

// Modern web technologies
pub mod oxc_pwa_modern_web_rules;
pub mod oxc_edge_serverless_rules;
pub mod oxc_web_payments_commerce_rules;
pub mod oxc_webrtc_realtime_rules;
pub mod oxc_graphql_rules;

// Specialized domains
pub mod oxc_data_science_ml_rules;
pub mod oxc_blockchain_web3_rules;
pub mod oxc_gaming_interactive_rules;
pub mod oxc_ar_vr_development_rules;
pub mod oxc_iot_embedded_rules;

// Infrastructure and deployment
pub mod oxc_cloud_native_rules;
pub mod oxc_devops_deployment_rules;
pub mod oxc_database_orm_rules;
pub mod oxc_database_optimization_rules;
pub mod oxc_api_integration_rules;
pub mod oxc_nodejs_rules;

// Core functionality and utilities
pub mod oxc_bestpractices_rules;
pub mod oxc_complexity_rules;
pub mod oxc_functional_programming_rules;
pub mod oxc_state_management_rules;
pub mod oxc_compatible_rules;
pub mod oxc_rules_adapter;
pub mod oxc_rules_migration;
pub mod oxc_unified_workflow;

// Testing utilities (only available in test mode)
#[cfg(test)]
pub mod oxc_ast_test;

// Re-export commonly used types and functions
pub use oxc_rules_adapter::*;
pub use oxc_compatible_rules::*;