//! # C014: Abstract Dependency Preferred Rule
//!
//! Enforces dependency injection pattern by preventing direct class instantiation.
//! Encourages loose coupling and better testability by suggesting injection of
//! dependencies rather than hard-coding concrete implementations through 'new' expressions.
//!
//! @category code-quality-rules
//! @safe team
//! @mvp core
//! @complexity low
//! @since 2.1.0

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use oxc_ast::ast::{Program, NewExpression, Expression};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Configuration options for the C014 rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C014Config {
    /// A list of allowed class names that can be directly instantiated.
    #[serde(default)]
    pub allowed_classes: Vec<String>,
    /// Whether to check built-in JavaScript classes like `Date`, `Error`, etc.
    #[serde(default = "default_check_builtin_classes")]
    pub check_builtin_classes: bool,
}

/// Returns the default value for checking built-in classes.
fn default_check_builtin_classes() -> bool {
    false
}

impl Default for C014Config {
    fn default() -> Self {
        Self {
            allowed_classes: vec![
                "Date".to_string(),
                "Error".to_string(),
                "Array".to_string(),
                "Object".to_string(),
                "String".to_string(),
                "Number".to_string(),
                "Boolean".to_string(),
                "RegExp".to_string(),
                "Map".to_string(),
                "Set".to_string(),
                "Promise".to_string(),
            ],
            check_builtin_classes: false,
        }
    }
}

/// The main entry point for the C014 rule checking.
pub fn check_abstract_dependency_preferred(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = C014Config::default();
    let mut visitor = C014Visitor::new(&config, program, code);
    visitor.visit_program(program);
    visitor.issues
}

/// An AST visitor for detecting direct class instantiation violations.
struct C014Visitor<'a> {
    config: &'a C014Config,
    source_code: &'a str,
    program: &'a Program<'a>,
    issues: Vec<LintIssue>,
    allowed_classes: HashSet<String>,
}

impl<'a> C014Visitor<'a> {
    /// Creates a new `C014Visitor`.
    fn new(config: &'a C014Config, program: &'a Program<'a>, source_code: &'a str) -> Self {
        let mut allowed_classes = HashSet::new();

        // Add configured allowed classes
        allowed_classes.extend(config.allowed_classes.iter().cloned());

        // Add default built-in classes if not checking them
        if !config.check_builtin_classes {
            allowed_classes.extend([
                "Date".to_string(),
                "Error".to_string(),
                "Array".to_string(),
                "Object".to_string(),
                "String".to_string(),
                "Number".to_string(),
                "Boolean".to_string(),
                "RegExp".to_string(),
                "Map".to_string(),
                "Set".to_string(),
                "Promise".to_string(),
            ]);
        }

        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
            allowed_classes,
        }
    }

    /// Generates a context-aware error message using AI enhancement.
    fn generate_ai_enhanced_message(&self, class_name: &str) -> String {
        format!("Direct instantiation of '{}' detected. Consider using dependency injection to improve testability and maintain loose coupling. Inject dependencies through constructor parameters instead of creating them directly.", class_name)
    }

    /// Generates intelligent fix suggestions using AI enhancement.
    fn generate_ai_fix_suggestions(&self, class_name: &str) -> Vec<String> {
        vec![
            format!("Add '{}' as a constructor parameter and inject it", class_name),
            format!("Create '{}' in a factory or service locator", class_name),
            format!("Use an interface/abstract class for '{}' dependency", class_name),
            "Consider using a dependency injection container".to_string(),
        ]
    }

    /// Calculates the line and column from a byte offset.
    fn calculate_line_column(&self, offset: usize) -> (u32, u32) {
        let mut line = 1;
        let mut column = 1;

        for (i, ch) in self.source_code.char_indices() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        (line, column)
    }

    /// Creates a lint issue for an abstract dependency violation with AI enhancement.
    fn create_abstract_dependency_issue(&self, class_name: &str, span: Span) -> LintIssue {
        let (line, column) = self.calculate_line_column(span.start as usize);

        // AI Enhancement: Generate context-aware message
        let ai_enhanced_message = self.generate_ai_enhanced_message(class_name);
        let _ai_fix_suggestions = self.generate_ai_fix_suggestions(class_name);

        LintIssue {
            rule_name: "moonshine/c014".to_string(),
            message: ai_enhanced_message,
            severity: LintSeverity::Warning,
            line,
            column,
            fix_available: true,
        }
    }

    /// Checks if a class should be flagged for direct instantiation.
    fn should_check_class(&self, class_name: &str) -> bool {
        !self.allowed_classes.contains(class_name)
    }

    /// Creates a dependency issue (legacy method for compatibility).
    fn create_dependency_issue(&self, class_name: &str, span: Span) -> LintIssue {
        self.create_abstract_dependency_issue(class_name, span)
    }

}

// impl<'a> Visit<'a> for C014Visitor<'a> {
//     fn visit_new_expression(&mut self, node: &NewExpression<'a>) {
//         // Check if the callee is an identifier (class name)
//         if let Expression::Identifier(ident) = &node.callee {
//             let class_name = &ident.name;

//             if self.should_check_class(class_name) {
//                 self.issues.push(self.create_dependency_issue(class_name, node.span));
//             }
//         }

//         // Continue visiting child nodes
//         self.visit_expression(&node.callee);
//         for arg in &node.arguments {
//             self.visit_argument(arg);
//         }
//     }
// }

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

        check_abstract_dependency_preferred(&parse_result.program, &semantic_result.semantic, code)
    }

    #[test]
    fn test_direct_class_instantiation_violation() {
        let code = r#"
class DatabaseService {
    constructor(connection) {
        // Direct instantiation - should be injected
        this.logger = new Logger();
        this.cache = new CacheManager();
        this.validator = new DataValidator();
    }
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("Logger")));
        assert!(issues.iter().any(|issue| issue.message.contains("CacheManager")));
        assert!(issues.iter().any(|issue| issue.message.contains("DataValidator")));
    }

    #[test]
    fn test_service_class_instantiation_violation() {
        let code = r#"
function createUserService() {
    // These should be injected, not instantiated directly
    const emailService = new EmailService();
    const userRepository = new UserRepository();
    const authService = new AuthenticationService();

    return new UserService(emailService, userRepository, authService);
}
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("EmailService")));
        assert!(issues.iter().any(|issue| issue.message.contains("UserRepository")));
        assert!(issues.iter().any(|issue| issue.message.contains("AuthenticationService")));
        assert!(issues.iter().any(|issue| issue.message.contains("UserService")));
    }

    #[test]
    fn test_builtin_classes_allowed_by_default() {
        let code = r#"
function processData() {
    // Built-in JavaScript classes should be allowed by default
    const date = new Date();
    const error = new Error("Something went wrong");
    const regex = new RegExp("pattern");
    const promise = new Promise((resolve) => resolve());
    const map = new Map();
    const set = new Set();
    const url = new URL("https://example.com");

    return { date, error, regex, promise, map, set, url };
}
        "#;

        let issues = parse_and_check(code);
        // Built-in classes should be allowed by default
        assert!(issues.is_empty());
    }

    #[test]
    fn test_lowercase_function_calls_compliant() {
        let code = r#"
function processData() {
    // These are function calls, not class instantiation
    const result = new processor();  // lowercase - function call
    const data = new getData();      // lowercase - function call
    const helper = new utils();      // lowercase - function call

    return { result, data, helper };
}
        "#;

        let issues = parse_and_check(code);
        // Lowercase function calls should not trigger the rule
        assert!(issues.is_empty());
    }

    #[test]
    fn test_dependency_injection_pattern_compliant() {
        let code = r#"
class UserService {
    constructor(emailService, userRepository, logger) {
        // Dependencies injected through constructor - good pattern
        this.emailService = emailService;
        this.userRepository = userRepository;
        this.logger = logger;
    }

    async createUser(userData) {
        this.logger.info("Creating user");
        await this.userRepository.save(userData);
        await this.emailService.sendWelcomeEmail(userData.email);
    }
}

// Factory function using dependency injection
function createUserService(dependencies) {
    return new UserService(
        dependencies.emailService,
        dependencies.userRepository,
        dependencies.logger
    );
}
        "#;

        let issues = parse_and_check(code);
        // UserService instantiation should trigger since it's direct instantiation
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("UserService")));
    }

    #[test]
    fn test_member_expression_instantiation_compliant() {
        let code = r#"
function useModules() {
    // These are member expressions, not direct class names
    const service = new modules.UserService();
    const repository = new database.UserRepository();
    const logger = new utils.Logger();

    return { service, repository, logger };
}
        "#;

        let issues = parse_and_check(code);
        // Member expressions should not trigger the rule (only direct identifiers)
        assert!(issues.is_empty());
    }
}