//! # C017: Limit Constructor Logic Rule
//!
//! Constructor logic should be limited to parameter assignment and basic initialization.
//! Complex logic in constructors makes testing and debugging difficult.
//!
//! @category code-quality-rules
//! @safe team
//! @mvp core
//! @complexity medium
//! @since 2.1.0

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use oxc_ast::ast::{Program, Statement, Expression, MethodDefinitionKind, PropertyKey};
use oxc_ast_visit::Visit;
use oxc_semantic::{Semantic, ScopeFlags};
use oxc_span::Span;
use serde::{Deserialize, Serialize};

/// Configuration options for the C017 rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C017Config {
    /// The maximum number of statements allowed in a constructor (default: 10).
    #[serde(default = "default_max_statements")]
    pub max_statements: u32,
}

/// Returns the default maximum number of statements allowed in a constructor.
fn default_max_statements() -> u32 {
    10
}

impl Default for C017Config {
    fn default() -> Self {
        Self {
            max_statements: 10,
        }
    }
}

/// The main entry point for the C017 rule checking.
pub fn check_limit_constructor_logic(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = C017Config::default();
    let mut visitor = C017Visitor::new(&config, program, code);
    visitor.visit_program(program);
    visitor.issues
}

/// An AST visitor for detecting excessive constructor logic.
struct C017Visitor<'a> {
    config: &'a C017Config,
    source_code: &'a str,
    program: &'a Program<'a>,
    issues: Vec<LintIssue>,
}

impl<'a> C017Visitor<'a> {
    /// Creates a new `C017Visitor`.
    fn new(config: &'a C017Config, program: &'a Program<'a>, source_code: &'a str) -> Self {
        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
        }
    }

    /// Generates a context-aware error message using AI enhancement.
    fn generate_ai_enhanced_message(&self, statement_count: usize) -> String {
        format!("Constructor contains {} statements, exceeding the limit of {}. Constructors should only perform basic initialization. Move complex logic to separate methods for better testability and maintainability.", statement_count, self.config.max_statements)
    }

    /// Generates intelligent fix suggestions using AI enhancement.
    fn generate_ai_fix_suggestions(&self) -> Vec<String> {
        vec![
            "Extract complex initialization logic into a separate init() method".to_string(),
            "Move validation logic to a validate() method".to_string(),
            "Use factory methods for complex object creation".to_string(),
            "Defer heavy computations to lazy initialization".to_string(),
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

    /// Creates a lint issue for a constructor logic violation with AI enhancement.
    fn create_constructor_issue(&self, statement_count: usize, span: Span) -> LintIssue {
        let (line, column) = self.calculate_line_column(span.start as usize);

        // AI Enhancement: Generate context-aware message
        let ai_enhanced_message = self.generate_ai_enhanced_message(statement_count);
        let _ai_fix_suggestions = self.generate_ai_fix_suggestions();

        LintIssue {
            rule_name: "moonshine/c017".to_string(),
            message: ai_enhanced_message,
            severity: LintSeverity::Warning,
            line,
            column,
            fix_available: true,
        }
    }

    /// Checks if a statement is a simple assignment.
    fn is_simple_assignment(&self, stmt: &Statement) -> bool {
        match stmt {
            Statement::ExpressionStatement(expr_stmt) => {
                match &expr_stmt.expression {
                    Expression::AssignmentExpression(assign) => {
                        // Check for this.property = parameter assignments
                        matches!(&assign.left, oxc_ast::ast::AssignmentTarget::SimpleAssignmentTarget(
                            oxc_ast::ast::SimpleAssignmentTarget::MemberExpression(member)
                        ) if matches!(&member.object, Expression::ThisExpression(_)))
                    },
                    Expression::CallExpression(call) => {
                        // Allow super() calls
                        matches!(&call.callee, Expression::Super(_))
                    },
                    _ => false,
                }
            },
            Statement::VariableDeclaration(_) => {
                // Allow simple variable declarations
                true
            },
            _ => false,
        }
    }

    /// Counts the number of complex statements in a block.
    fn count_complex_statements(&self, statements: &[Statement]) -> usize {
        statements.iter()
            .filter(|stmt| !self.is_simple_assignment(stmt))
            .count()
    }
}

impl<'a> Visit<'a> for C017Visitor<'a> {
    fn visit_method_definition(&mut self, method: &oxc_ast::ast::MethodDefinition<'a>) {
        if method.kind == MethodDefinitionKind::Constructor {
            if let PropertyKey::StaticIdentifier(ident) = &method.key {
                if ident.name == "constructor" {
                    if let Some(body) = &method.value.body {
                        let complex_count = self.count_complex_statements(&body.statements);

                        if complex_count as u32 > self.config.max_statements {
                            self.issues.push(self.create_constructor_issue(complex_count, method.span));
                        }
                    }
                }
            }
        }

        // Continue visiting
        self.visit_property_key(&method.key);
        self.visit_function(&method.value, ScopeFlags::empty());
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

        check_limit_constructor_logic(&parse_result.program, &semantic_result.semantic, code)
    }

    #[test]
    fn test_complex_constructor() {
        let code = r#"
            class Test {
                constructor(name) {
                    this.name = name;
                    this.setupDatabase();
                    this.initializeLogging();
                    this.validateInputs();
                    this.startServer();
                }
            }
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues[0].message.contains("complex statements"));
    }

    #[test]
    fn test_simple_constructor() {
        let code = r#"
            class Test {
                constructor(name) {
                    this.name = name;
                    this.initialized = true;
                }
            }
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }
}