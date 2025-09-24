//! # Rule Generation Templates
//!
//! Template system for generating custom lint rules from detected patterns.
//! Provides reusable templates for common rule patterns and test cases.
//!
//! @category templates
//! @safe program
//! @complexity medium
//! @since 2.1.0

/// Template content for unused code rules
pub const UNUSED_CODE_TEMPLATE: &str = include_str!("unused_code_template.rs");

/// Template content for unused code tests
pub const UNUSED_CODE_TEST_TEMPLATE: &str = include_str!("unused_code_test_template.rs");

/// Template content for unused code documentation
pub const UNUSED_CODE_DOCS_TEMPLATE: &str = include_str!("unused_code_docs_template.md");
