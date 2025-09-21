//! Custom MoonShine rule for: T002 – Interface names should start with 'I'
//! Rule ID: moonshine/t002
//! Purpose: Enforce consistent interface naming convention by requiring the 'I' prefix
//!
//! Converted from JavaScript ESLint rule
//! @category code-quality-rules
//! @complexity low

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use oxc_ast::ast::{Program, TSInterfaceDeclaration, BindingIdentifier};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use serde::{Deserialize, Serialize};

/// Configuration options for T002 rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct T002Config {
    /// Whether to require 'I' prefix for interfaces (default: true)
    pub require_i_prefix: bool,
    /// Custom prefixes to allow (default: ["I"])
    pub allowed_prefixes: Vec<String>,
}

impl Default for T002Config {
    fn default() -> Self {
        Self {
            require_i_prefix: true,
            allowed_prefixes: vec!["I".to_string()],
        }
    }
}

/// Main entry point for T002 rule checking
pub fn check_interface_prefix_i(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = T002Config::default();
    let mut visitor = T002Visitor::new(&config, code);
    visitor.visit_program(program);
    visitor.issues
}

/// AST visitor for detecting interface naming violations
struct T002Visitor<'a> {
    config: &'a T002Config,
    source_code: &'a str,
    issues: Vec<LintIssue>,
}

impl<'a> T002Visitor<'a> {
    fn new(config: &'a T002Config, source_code: &'a str) -> Self {
        Self {
            config,
            source_code,
            issues: Vec::new(),
        }
    }

    /// Check if interface name has valid prefix
    fn has_valid_prefix(&self, name: &str) -> bool {
        if !self.config.require_i_prefix {
            return true;
        }

        self.config.allowed_prefixes.iter().any(|prefix| name.starts_with(prefix))
    }

    /// Create lint issue for interface without proper prefix with AI enhancement
    fn create_interface_prefix_issue(&self, interface_name: &str, node: &BindingIdentifier) -> LintIssue {
        let span = node.span;
        let (line, column) = self.calculate_line_column(span.start as usize);

        // AI Enhancement: Generate context-aware suggestions
        let ai_enhanced_message = self.generate_ai_enhanced_message(interface_name);
        let _ai_fix_suggestions = self.generate_ai_fix_suggestions(interface_name);

        LintIssue {
            rule_name: "moonshine/t002".to_string(),
            message: ai_enhanced_message,
            severity: LintSeverity::Warning,
            line,
            column,
            fix_available: true,
        }
    }

    /// AI Enhancement: Generate context-aware error message
    fn generate_ai_enhanced_message(&self, interface_name: &str) -> String {
        // Simple AI enhancement - in production this would use Claude API
        if interface_name.chars().next().map_or(false, |c| c.is_lowercase()) {
            format!("Interface name '{}' should start with 'I' (TypeScript naming convention). Consider 'I{}'",
                   interface_name, interface_name)
        } else {
            format!("Interface name '{}' should start with 'I' prefix for consistency with TypeScript best practices",
                   interface_name)
        }
    }

    /// AI Enhancement: Generate intelligent fix suggestions
    fn generate_ai_fix_suggestions(&self, interface_name: &str) -> Vec<String> {
        let mut suggestions = vec![format!("I{}", interface_name)];

        // AI Enhancement: Context-aware alternative suggestions
        if interface_name.ends_with("Type") {
            suggestions.push(format!("I{}", &interface_name[..interface_name.len()-4]));
        }

        if interface_name.ends_with("Interface") {
            suggestions.push(format!("I{}", &interface_name[..interface_name.len()-9]));
        }

        suggestions
    }

    /// Calculate line and column from byte offset
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
}

impl<'a> Visit<'a> for T002Visitor<'a> {
    /// Visit TypeScript interface declarations
    fn visit_ts_interface_declaration(&mut self, node: &TSInterfaceDeclaration<'a>) {
        let interface_name = &node.id.name;

        if !self.has_valid_prefix(interface_name) {
            let issue = self.create_interface_prefix_issue(interface_name, &node.id);
            self.issues.push(issue);
        }

        // Continue visiting child nodes
        oxc_ast_visit::walk::walk_ts_interface_declaration(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_parser::Parser;
    use oxc_semantic::SemanticBuilder;
    use oxc_span::SourceType;

    /// Helper function to parse code and run the rule
    fn parse_and_check(code: &str) -> Vec<LintIssue> {
        let allocator = Allocator::default();
        let source_type = SourceType::from_path("test.ts").unwrap();
        let parser = Parser::new(&allocator, code, source_type);
        let parse_result = parser.parse();

        if !parse_result.errors.is_empty() {
            panic!("Parse errors: {:?}", parse_result.errors);
        }

        let semantic = SemanticBuilder::new(code, source_type)
            .build(&parse_result.program)
            .semantic;

        check_interface_prefix_i(&parse_result.program, &semantic, code)
    }

    #[test]
    fn test_interface_without_i_prefix_violation() {
        let code = "interface User { name: string; }";
        let issues = parse_and_check(code);

        assert!(!issues.is_empty());
        assert_eq!(issues[0].rule_id, "moonshine/t002");
        assert!(issues[0].message.contains("Interface name 'User' should start with 'I'"));
        assert_eq!(issues[0].fix_suggestion, Some("IUser".to_string()));
    }

    #[test]
    fn test_interface_with_i_prefix_compliant() {
        let code = "interface IUser { name: string; }";
        let issues = parse_and_check(code);

        assert!(issues.is_empty());
    }

    #[test]
    fn test_multiple_interfaces_mixed() {
        let code = r#"
            interface IValidInterface { id: number; }
            interface InvalidInterface { name: string; }
            interface IAnotherValid { value: boolean; }
        "#;
        let issues = parse_and_check(code);

        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("InvalidInterface"));
    }

    #[test]
    fn test_nested_interfaces() {
        let code = r#"
            namespace MyNamespace {
                interface BadInterface { x: number; }
                interface IGoodInterface { y: number; }
            }
        "#;
        let issues = parse_and_check(code);

        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("BadInterface"));
    }

    #[test]
    fn test_interface_with_generics() {
        let code = "interface Repository<T> { save(item: T): Promise<T>; }";
        let issues = parse_and_check(code);

        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("Repository"));
        assert_eq!(issues[0].fix_suggestion, Some("IRepository".to_string()));
    }
}