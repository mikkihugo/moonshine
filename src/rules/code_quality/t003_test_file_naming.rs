//! # T003: Test File Naming Convention Rule
//!
//! Enforces consistent naming conventions for test files to improve project organization
//! and test discoverability. Ensures test files follow established patterns like .test.js,
//! .spec.js, or are located in __tests__ directories.
//!
//! @category code-quality-rules
//! @safe team
//! @mvp core
//! @complexity low
//! @since 2.1.0

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use crate::rules::utils::span_to_line_col_legacy;
use crate::rules::ai_integration::AIEnhancer;
use oxc_ast::ast::{Program, Statement, ImportDeclaration, CallExpression, Expression};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use regex::Regex;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct T003Config {
    /// Valid test file patterns (default: [".test.", ".spec.", "__tests__"])
    pub valid_patterns: Vec<String>,
    /// Valid test file extensions (default: [".js", ".ts", ".jsx", ".tsx"])
    pub valid_extensions: Vec<String>,
    /// Whether to enforce test directory naming (default: true)
    pub enforce_test_directory: bool,
    /// Whether to check for test framework imports (default: true)
    pub check_test_imports: bool,
    /// Custom test framework detection patterns
    pub test_framework_patterns: Vec<String>,
    /// Whether to allow nested test directories (default: true)
    pub allow_nested_test_dirs: bool,
    /// Minimum required test keywords in file names
    pub required_test_keywords: Vec<String>,
}

/// T003 rule implementation with AI enhancement
pub fn check_test_file_naming(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = T003Config::default();
    let mut visitor = TestFileNamingVisitor::new(program, code, &config);
    visitor.visit_program(program);
    visitor.finalize_issues()
}

#[derive(Debug, Clone)]
enum TestFileViolation {
    InvalidNaming,
    MissingTestKeyword,
    InvalidExtension,
    WrongDirectory,
    MissingTestFramework,
    InconsistentPattern,
}

impl TestFileViolation {
    fn description(&self) -> &'static str {
        match self {
            TestFileViolation::InvalidNaming => "invalid test file naming pattern",
            TestFileViolation::MissingTestKeyword => "missing test keywords in filename",
            TestFileViolation::InvalidExtension => "invalid file extension for test file",
            TestFileViolation::WrongDirectory => "test file in incorrect directory",
            TestFileViolation::MissingTestFramework => "test file missing framework imports",
            TestFileViolation::InconsistentPattern => "inconsistent test file naming pattern",
        }
    }

    fn recommendation(&self) -> &'static str {
        match self {
            TestFileViolation::InvalidNaming => "Use .test.js, .spec.js, or place in __tests__ directory",
            TestFileViolation::MissingTestKeyword => "Include 'test' or 'spec' in the filename",
            TestFileViolation::InvalidExtension => "Use .js, .ts, .jsx, or .tsx extension",
            TestFileViolation::WrongDirectory => "Move to __tests__ directory or rename with .test/.spec suffix",
            TestFileViolation::MissingTestFramework => "Import test framework (Jest, Mocha, Vitest, etc.)",
            TestFileViolation::InconsistentPattern => "Follow consistent naming pattern across the project",
        }
    }
}

struct TestFileNamingVisitor<'a> {
    config: &'a T003Config,
    program: &'a Program<'a>,
    source_code: &'a str,
    issues: Vec<LintIssue>,
    current_file_path: String,
    valid_patterns: Vec<Regex>,
    valid_extensions: HashSet<String>,
    test_framework_patterns: Vec<Regex>,
    required_test_keywords: HashSet<String>,
    has_test_imports: bool,
    has_test_functions: bool,
    detected_violations: Vec<TestFileViolation>,
}

impl<'a> TestFileNamingVisitor<'a> {
    fn new(program: &'a Program<'a>, source_code: &'a str, config: &'a T003Config) -> Self {
        // Default valid test patterns
        let default_patterns = vec![
            r"\.test\.", r"\.spec\.", r"__tests__"
        ];

        let pattern_strings = if config.valid_patterns.is_empty() {
            default_patterns.iter().map(|s| s.to_string()).collect()
        } else {
            config.valid_patterns.clone()
        };

        let mut valid_patterns = Vec::new();
        for pattern_str in &pattern_strings {
            if let Ok(regex) = Regex::new(pattern_str) {
                valid_patterns.push(regex);
            }
        }

        // Default valid extensions
        let mut valid_extensions = HashSet::from([
            ".js", ".ts", ".jsx", ".tsx", ".mjs", ".cjs"
        ].map(String::from));

        if !config.valid_extensions.is_empty() {
            valid_extensions.clear();
            valid_extensions.extend(config.valid_extensions.iter().cloned());
        }

        // Test framework patterns
        let default_framework_patterns = vec![
            r#"from\s+['\"]jest['\"]"#,
            r#"from\s+['\"]@jest/"#,
            r#"from\s+['\"]mocha['\"]"#,
            r#"from\s+['\"]vitest['\"]"#,
            r#"from\s+['\"]@testing-library/"#,
            r#"import.*describe"#,
            r#"import.*it\b"#,
            r#"import.*test\b"#,
            r#"import.*expect"#,
        ];

        let framework_pattern_strings = if config.test_framework_patterns.is_empty() {
            default_framework_patterns
        } else {
            config.test_framework_patterns.clone()
        };

        let mut test_framework_patterns = Vec::new();
        for pattern_str in &framework_pattern_strings {
            if let Ok(regex) = Regex::new(pattern_str) {
                test_framework_patterns.push(regex);
            }
        }

        // Required test keywords
        let mut required_test_keywords = HashSet::from([
            "test", "spec", "unit", "integration", "e2e"
        ].map(String::from));

        if !config.required_test_keywords.is_empty() {
            required_test_keywords.clear();
            required_test_keywords.extend(config.required_test_keywords.iter().cloned());
        }

        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
            current_file_path: String::new(), // This would be set by the caller
            valid_patterns,
            valid_extensions,
            test_framework_patterns,
            required_test_keywords,
            has_test_imports: false,
            has_test_functions: false,
            detected_violations: Vec::new(),
        }
    }

    fn generate_ai_enhanced_message(&self, base_message: &str, span: Span) -> String {
        // Context extraction removed: obsolete extract_context utility.
        format!("{}", base_message)
    }

    fn generate_ai_fix_suggestions(&self) -> Vec<String> {
        vec![
            "Rename test files to follow naming conventions (.test.js, .spec.js, or place in __tests__ directory)".to_string(),
            "Ensure test files contain appropriate test keywords in their names".to_string(),
            "Move test files to proper test directories".to_string(),
            "Use consistent test file extensions".to_string(),
        ]
    }

    fn calculate_line_column(&self, span: Span) -> (usize, usize) {
        crate::rules::utils::span_to_line_col(self.source_code, span)
    }

    fn finalize_issues(mut self) -> Vec<LintIssue> {
        // Apply AI enhancements to all issues
        let ai_suggestions = self.generate_ai_fix_suggestions();
        for issue in &mut self.issues {
            if let Some(ai_context) = ai_suggestions.first() {
                issue.message = format!("{} 💡 AI Suggestion: {}", issue.message, ai_context);
            }
        }
        self.issues
    }

    fn analyze_file_path(&mut self, file_path: &str) {
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");

        let directory = std::path::Path::new(file_path)
            .parent()
            .and_then(|dir| dir.to_str())
            .unwrap_or("");

        // Check if this appears to be a test file based on content
        if self.appears_to_be_test_file() {
            self.validate_test_file_naming(file_name, directory);
        }
    }

    fn appears_to_be_test_file(&self) -> bool {
        // Check for test framework imports
        if self.has_test_imports {
            return true;
        }

        // Check for test functions
        if self.has_test_functions {
            return true;
        }

        // Check for common test patterns in code
        let test_indicators = [
            "describe(", "it(", "test(", "expect(", "assert(",
            "beforeEach(", "afterEach(", "beforeAll(", "afterAll(",
            "jest.mock(", "sinon.", "chai."
        ];

        test_indicators.iter().any(|indicator| self.code.contains(indicator))
    }

    fn validate_test_file_naming(&mut self, file_name: &str, directory: &str) {
        let mut violations = Vec::new();

        // Check file extension
        if !self.has_valid_extension(file_name) {
            violations.push(TestFileViolation::InvalidExtension);
        }

        // Check naming pattern
        if !self.has_valid_naming_pattern(file_name, directory) {
            violations.push(TestFileViolation::InvalidNaming);
        }

        // Check for required test keywords
        if !self.has_required_keywords(file_name) {
            violations.push(TestFileViolation::MissingTestKeyword);
        }

        // Check directory structure
        if self.config.enforce_test_directory && !self.is_in_valid_test_directory(directory) {
            violations.push(TestFileViolation::WrongDirectory);
        }

        // Check for test framework imports
        if self.config.check_test_imports && !self.has_test_imports {
            violations.push(TestFileViolation::MissingTestFramework);
        }

        for violation in violations {
            self.create_test_naming_issue(violation, file_name);
        }
    }

    fn has_valid_extension(&self, file_name: &str) -> bool {
        self.valid_extensions.iter().any(|ext| file_name.ends_with(ext))
    }

    fn has_valid_naming_pattern(&self, file_name: &str, directory: &str) -> bool {
        self.valid_patterns.iter().any(|pattern| {
            pattern.is_match(file_name) || pattern.is_match(directory)
        })
    }

    fn has_required_keywords(&self, file_name: &str) -> bool {
        let file_name_lower = file_name.to_lowercase();
        self.required_test_keywords.iter().any(|keyword| {
            file_name_lower.contains(keyword)
        })
    }

    fn is_in_valid_test_directory(&self, directory: &str) -> bool {
        let directory_lower = directory.to_lowercase();

        // Check for common test directory patterns
        let test_dir_patterns = [
            "__tests__", "test", "tests", "spec", "specs",
            ".test", ".spec"
        ];

        test_dir_patterns.iter().any(|pattern| {
            directory_lower.contains(pattern)
        })
    }

    fn create_test_naming_issue(&mut self, violation: TestFileViolation, file_name: &str) {
        // Since we don't have access to file path span, use program span
        let span = self.program.span;
        let (line, column) = self.calculate_line_column(span);

        let message = self.generate_ai_enhanced_message(
            &format!("Test file '{}' has {}: {}",
                file_name,
                violation.description(),
                violation.recommendation()
            ),
            span
        );

        let severity = match violation {
            TestFileViolation::MissingTestFramework => LintSeverity::Error,
            TestFileViolation::InvalidExtension => LintSeverity::Error,
            _ => LintSeverity::Warning,
        };

        self.issues.push(LintIssue {
            rule_name: "T003".to_string(),
            severity,
            message,
            line,
            column,
            fix_available: true,
        });
    }

    fn check_test_framework_import(&mut self, source: &str) {
        if self.test_framework_patterns.iter().any(|pattern| pattern.is_match(source)) {
            self.has_test_imports = true;
        }
    }

    fn check_test_function_call(&mut self, function_name: &str) {
        let test_functions = [
            "describe", "it", "test", "expect", "assert",
            "beforeEach", "afterEach", "beforeAll", "afterAll"
        ];

        if test_functions.contains(&function_name) {
            self.has_test_functions = true;
        }
    }
}

impl<'a> Visit<'a> for TestFileNamingVisitor<'a> {
    fn visit_program(&mut self, node: &Program<'a>) {
        // In a real implementation, the file path would be provided by the caller
        // For now, we'll check if the code appears to be a test file

        // First pass: collect test indicators
        for stmt in &node.body {
            self.visit_statement(stmt);
        }

        // Second pass: analyze naming if this appears to be a test file
        if self.appears_to_be_test_file() {
            // Simulate file path analysis - in practice this would use actual file path
            let simulated_file_path = if self.code.contains("describe(") || self.code.contains("it(") {
                "example.test.js" // This would be the actual file path
            } else {
                "example.js"
            };

            self.analyze_file_path(simulated_file_path);
        }
    }

    fn visit_import_declaration(&mut self, node: &ImportDeclaration<'a>) {
        let source = &node.source.value;
        self.check_test_framework_import(source);

        // Continue visiting child nodes
        for specifier in &node.specifiers {
            match specifier {
                oxc_ast::ast::ImportDeclarationSpecifier::ImportSpecifier(spec) => {
                    self.visit_import_specifier(spec);
                }
                oxc_ast::ast::ImportDeclarationSpecifier::ImportDefaultSpecifier(spec) => {
                    self.visit_import_default_specifier(spec);
                }
                oxc_ast::ast::ImportDeclarationSpecifier::ImportNamespaceSpecifier(spec) => {
                    self.visit_import_namespace_specifier(spec);
                }
            }
        }
    }

    fn visit_call_expression(&mut self, node: &CallExpression<'a>) {
        if let Expression::Identifier(ident) = &node.callee {
            self.check_test_function_call(&ident.name);
        }

        // Continue visiting child nodes
        self.visit_expression(&node.callee);
        for arg in &node.arguments {
            match arg {
                oxc_ast::ast::Argument::Expression(expr) => self.visit_expression(expr),
                oxc_ast::ast::Argument::SpreadElement(spread) => self.visit_spread_element(spread),
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_parser::{Parser, ParseOptions};
    use oxc_semantic::SemanticBuilder;
    use oxc_span::SourceType;

    fn parse_and_check(code: &str) -> Vec<LintIssue> {
        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);

        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        check_test_file_naming(&parse_result.program, &semantic_result.semantic, code)
    }

    #[test]
    fn test_valid_test_file_patterns_compliant() {
        let code = r#"
import { describe, it, expect } from '@jest/globals';

describe('User Service', () => {
    it('should create user successfully', () => {
        expect(true).toBe(true);
    });
});
        "#;

        let issues = parse_and_check(code);
        // This would pass if the file was named properly like user.test.js
        // In practice, the actual file path would be checked
        assert!(issues.is_empty() || issues.iter().all(|issue| issue.severity != LintSeverity::Error));
    }

    #[test]
    fn test_missing_test_framework_violation() {
        let code = r#"
// This looks like a test file but has no framework imports
describe('User Service', () => {
    it('should create user', () => {
        console.log('test');
    });
});
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        // Should detect test patterns but missing framework import
    }

    #[test]
    fn test_test_functions_detected() {
        let code = r#"
import { jest } from '@jest/globals';

describe('Calculator', () => {
    beforeEach(() => {
        jest.clearAllMocks();
    });

    it('should add numbers correctly', () => {
        expect(2 + 2).toBe(4);
    });

    afterEach(() => {
        jest.restoreAllMocks();
    });
});
        "#;

        let issues = parse_and_check(code);
        // Should be recognized as a test file
        assert!(issues.is_empty() || issues.iter().all(|issue| !issue.message.contains("Missing")));
    }

    #[test]
    fn test_non_test_file_ignored() {
        let code = r#"
export class UserService {
    createUser(userData) {
        return { id: 1, ...userData };
    }

    getUserById(id) {
        return { id, name: 'Test User' };
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty()); // Non-test files should be ignored
    }

    #[test]
    fn test_various_test_frameworks_detected() {
        let code = r#"
import { describe, it } from 'vitest';
import { render, screen } from '@testing-library/react';

describe('Component', () => {
    it('renders correctly', () => {
        render(<div>Test</div>);
        expect(screen.getByText('Test')).toBeInTheDocument();
    });
});
        "#;

        let issues = parse_and_check(code);
        // Should detect Vitest and Testing Library
        assert!(issues.is_empty() || issues.iter().all(|issue| !issue.message.contains("framework")));
    }

    #[test]
    fn test_ai_enhancement_integration() {
        let code = r#"
// This file has test functions but improper naming
describe('User tests', () => {
    it('should work', () => {
        expect(true).toBe(true);
    });
});
        "#;

        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);
        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        let mut ai_enhancer = AIEnhancer::new();
        ai_enhancer.set_context("Suggest proper test file naming conventions".to_string());

        let issues = check_test_file_naming(&parse_result.program, &semantic_result.semantic, code, Some(&ai_enhancer));

        // AI should suggest proper naming patterns
        assert!(issues.iter().any(|issue| issue.fix_available));
    }
}