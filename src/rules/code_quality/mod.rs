//! # Code Quality Rules (C-Series)
//!
//! MoonShine code quality rules for improving code maintainability,
//! readability, and overall quality. Based on industry best practices
//! and enhanced with AI suggestions.
//!
//! @category code-quality-rules
//! @safe program
//! @mvp enhanced
//! @complexity medium
//! @since 2.1.0

use crate::wasm_safe_linter::LintIssue;
use crate::rules::engine::{MoonShineRule, MoonShineRuleCategory, RuleImplementation};
use crate::wasm_safe_linter::LintSeverity;
use oxc_ast::ast::Program;
use oxc_semantic::Semantic;
use std::collections::HashMap;

pub mod c002_no_duplicate_code;
pub mod c003_no_vague_abbreviations;
pub mod c010_limit_block_nesting;
pub mod c013_no_dead_code;
pub mod c014_abstract_dependency_preferred;
pub mod c018_no_generic_throw;
pub mod c006_function_naming;
pub mod c017_limit_constructor_logic;
pub mod c023_no_duplicate_variable_name;
pub mod c029_catch_block_logging;
pub mod c030_use_custom_error_classes;
pub mod c035_no_cryptographic_weaknesses;
pub mod c041_no_hardcoded_configuration;
pub mod c042_boolean_naming;
pub mod c043_no_magic_numbers;
pub mod c047_no_inconsistent_returns;
pub mod t002_interface_prefix_i;
pub mod t003_test_file_naming;
pub mod t004_test_coverage_threshold;
pub mod t007_test_description_quality;
pub mod t010_test_setup_teardown;

/// Register all code quality rules
pub fn register_rules(rules: &mut HashMap<String, MoonShineRule>) {
    // C002: No duplicate code blocks
    rules.insert("C002".to_string(), MoonShineRule {
        id: "C002".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Detect duplicate code blocks longer than 10 lines".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });

    // C003: No vague abbreviations
    rules.insert("C003".to_string(), MoonShineRule {
        id: "C003".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Clear variable names, avoid arbitrary abbreviations".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });

    // C010: Limit block nesting
    rules.insert("C010".to_string(), MoonShineRule {
        id: "C010".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Limit nested blocks to maximum 3 levels".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });

    // C013: No dead code
    rules.insert("C013".to_string(), MoonShineRule {
        id: "C013".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Do not leave dead code commented out or unreachable".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });

    // C014: Abstract dependency preferred
    rules.insert("C014".to_string(), MoonShineRule {
        id: "C014".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Use dependency injection instead of direct instantiation".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });

    // C018: No generic throw
    rules.insert("C018".to_string(), MoonShineRule {
        id: "C018".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Do not throw generic errors, always use specific messages".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });

    // C023: No duplicate variable name in scope
    rules.insert("C023".to_string(), MoonShineRule {
        id: "C023".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Error,
        description: "Prevent variable name shadowing and maintain clear variable scoping".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });

    // C035: No cryptographic weaknesses
    rules.insert("C035".to_string(), MoonShineRule {
        id: "C035".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Error,
        description: "Detect weak cryptographic algorithms and insecure random number generators".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });

    // C041: No hardcoded configuration values
    rules.insert("C041".to_string(), MoonShineRule {
        id: "C041".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Configuration values should be externalized to environment variables or config files".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });

    // C043: No magic numbers
    rules.insert("C043".to_string(), MoonShineRule {
        id: "C043".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Replace magic numbers with named constants for better code readability".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });

    // C047: No inconsistent returns
    rules.insert("C047".to_string(), MoonShineRule {
        id: "C047".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Ensure consistent return statements - all code paths return values or none do".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });

    // C006: Function naming conventions
    rules.insert("C006".to_string(), MoonShineRule {
        id: "C006".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Function names must be verbs or verb-noun phrases".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });

    // C017: Limit constructor logic
    rules.insert("C017".to_string(), MoonShineRule {
        id: "C017".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Constructor logic should be limited to parameter assignment and basic initialization".to_string(),
        ai_enhanced: false,
        implementation: RuleImplementation::Ast,
    });

    // C029: Catch block logging
    rules.insert("C029".to_string(), MoonShineRule {
        id: "C029".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Every catch block must log the error cause or rethrow".to_string(),
        ai_enhanced: false,
        implementation: RuleImplementation::Ast,
    });

    // C030: Use custom error classes
    rules.insert("C030".to_string(), MoonShineRule {
        id: "C030".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Use custom error classes instead of generic Error objects".to_string(),
        ai_enhanced: false,
        implementation: RuleImplementation::Ast,
    });

    // C042: Boolean naming convention
    rules.insert("C042".to_string(), MoonShineRule {
        id: "C042".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Boolean variable names should start with descriptive prefixes".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });

    // T002: Interface names should start with 'I'
    rules.insert("T002".to_string(), MoonShineRule {
        id: "T002".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Interface names should start with 'I' for consistent naming convention".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });

    // T003: Test file naming convention
    rules.insert("T003".to_string(), MoonShineRule {
        id: "T003".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Enforce consistent naming conventions for test files".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });

    // T004: Test coverage threshold
    rules.insert("T004".to_string(), MoonShineRule {
        id: "T004".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Enforce minimum test coverage thresholds".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });

    // T007: Test description quality
    rules.insert("T007".to_string(), MoonShineRule {
        id: "T007".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Enforce high-quality test descriptions".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });

    // T010: Test setup/teardown patterns
    rules.insert("T010".to_string(), MoonShineRule {
        id: "T010".to_string(),
        category: MoonShineRuleCategory::CodeQuality,
        severity: LintSeverity::Warning,
        description: "Enforce proper test setup and teardown patterns".to_string(),
        ai_enhanced: true,
        implementation: RuleImplementation::AiAssisted,
    });
}

/// Check semantic-based code quality rules
pub fn check_semantic_rule(_rule_id: &str, _program: &Program, _semantic: &Semantic, _code: &str) -> Vec<LintIssue> {
    Vec::new()
}

/// Check AST-based code quality rules
pub fn check_ast_rule(rule_id: &str, program: &Program, code: &str) -> Vec<LintIssue> {
    match rule_id {
        "C017" => c017_limit_constructor_logic::check_limit_constructor_logic(program, &oxc_semantic::SemanticBuilder::new().build(program).semantic, code),
        "C029" => c029_catch_block_logging::check_catch_block_logging(program, &oxc_semantic::SemanticBuilder::new().build(program).semantic, code),
        "C030" => c030_use_custom_error_classes::check_use_custom_error_classes(program, &oxc_semantic::SemanticBuilder::new().build(program).semantic, code),
        _ => Vec::new(),
    }
}

/// Check AI-assisted code quality rules
pub fn check_ai_rule(rule_id: &str, program: &Program, semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    match rule_id {
        "C002" => c002_no_duplicate_code::check_no_duplicate_code(program, semantic, code),
        "C003" => c003_no_vague_abbreviations::check_no_vague_abbreviations(program, semantic, code),
        "C010" => c010_limit_block_nesting::check_limit_block_nesting(program, semantic, code),
        "C013" => c013_no_dead_code::check_no_dead_code(program, semantic, code),
        "C014" => c014_abstract_dependency_preferred::check_abstract_dependency_preferred(program, semantic, code),
        "C018" => c018_no_generic_throw::check_no_generic_throw(program, semantic, code),
        "C023" => c023_no_duplicate_variable_name::check_no_duplicate_variable_name(program, semantic, code, None),
        "C035" => c035_no_cryptographic_weaknesses::check_no_cryptographic_weaknesses(program, semantic, code, None),
        "C041" => c041_no_hardcoded_configuration::check_no_hardcoded_configuration(program, semantic, code, None),
        "C043" => c043_no_magic_numbers::check_no_magic_numbers(program, semantic, code, None),
        "C047" => c047_no_inconsistent_returns::check_no_inconsistent_returns(program, semantic, code, None),
        "C006" => c006_function_naming::check_function_naming(program, semantic, code),
        "C042" => c042_boolean_naming::check_boolean_naming(program, semantic, code),
        "T002" => t002_interface_prefix_i::check_interface_prefix_i(program, semantic, code),
        "T003" => t003_test_file_naming::check_test_file_naming(program, semantic, code),
        "T004" => t004_test_coverage_threshold::check_test_coverage_threshold(program, semantic, code),
        "T007" => t007_test_description_quality::check_test_description_quality(program, semantic, code),
        "T010" => t010_test_setup_teardown::check_test_setup_teardown(program, semantic, code),
        _ => Vec::new(),
    }
}

/// NEW: Check OXC semantic-based code quality rules with AI enhancement
pub fn check_oxc_semantic_rule(_rule_id: &str, _program: &Program, _semantic: &Semantic, _code: &str) -> Vec<LintIssue> {
    // For now, delegate to existing semantic rule implementation
    check_semantic_rule(_rule_id, _program, _semantic, _code)
}

/// NEW: Check OXC AST visitor-based code quality rules with AI enhancement
pub fn check_oxc_ast_visitor_rule(rule_id: &str, program: &Program, semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    match rule_id {
        "C002" => c002_no_duplicate_code::check_no_duplicate_code(program, semantic, code),
        "C003" => c003_no_vague_abbreviations::check_no_vague_abbreviations(program, semantic, code),
        "C010" => c010_limit_block_nesting::check_limit_block_nesting(program, semantic, code),
        "C013" => c013_no_dead_code::check_no_dead_code(program, semantic, code),
        "C014" => c014_abstract_dependency_preferred::check_abstract_dependency_preferred(program, semantic, code),
        "C018" => c018_no_generic_throw::check_no_generic_throw(program, semantic, code),
        "C023" => c023_no_duplicate_variable_name::check_no_duplicate_variable_name(program, semantic, code, None),
        "C035" => c035_no_cryptographic_weaknesses::check_no_cryptographic_weaknesses(program, semantic, code, None),
        "C041" => c041_no_hardcoded_configuration::check_no_hardcoded_configuration(program, semantic, code, None),
        "C043" => c043_no_magic_numbers::check_no_magic_numbers(program, semantic, code, None),
        "C047" => c047_no_inconsistent_returns::check_no_inconsistent_returns(program, semantic, code, None),
        "C029" => c029_catch_block_logging::check_catch_block_logging(program, semantic, code),
        "C042" => c042_boolean_naming::check_boolean_naming(program, semantic, code),
        "C017" => c017_limit_constructor_logic::check_limit_constructor_logic(program, semantic, code),
        "C030" => c030_use_custom_error_classes::check_use_custom_error_classes(program, semantic, code),
        "T003" => t003_test_file_naming::check_test_file_naming(program, semantic, code),
        "T004" => t004_test_coverage_threshold::check_test_coverage_threshold(program, semantic, code),
        "T007" => t007_test_description_quality::check_test_description_quality(program, semantic, code),
        "T010" => t010_test_setup_teardown::check_test_setup_teardown(program, semantic, code),
        _ => Vec::new(),
    }
}