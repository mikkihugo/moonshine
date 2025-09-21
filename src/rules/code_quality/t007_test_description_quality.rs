//! # T007: Test Description Quality Rule
//!
//! Enforces high-quality test descriptions that clearly explain what is being tested
//! and what the expected behavior is. Promotes maintainable and self-documenting
//! test suites by ensuring descriptive and meaningful test names.
//!
//! @category code-quality-rules
//! @safe team
//! @mvp core
//! @complexity medium
//! @since 2.1.0

use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use oxc_ast::ast::{Program, CallExpression, Expression};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::Span;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use regex::Regex;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct T007Config {
    /// Minimum description length (default: 10)
    pub min_description_length: u32,
    /// Maximum description length (default: 120)
    pub max_description_length: u32,
    /// Whether to enforce "should" pattern (default: true)
    pub enforce_should_pattern: bool,
    /// Whether to require behavior descriptions (default: true)
    pub require_behavior_description: bool,
    /// Banned words/phrases in test descriptions
    pub banned_words: Vec<String>,
    /// Required patterns for test descriptions
    pub required_patterns: Vec<String>,
    /// Whether to check describe block descriptions (default: true)
    pub check_describe_blocks: bool,
    /// Whether to enforce action-oriented descriptions (default: true)
    pub enforce_action_oriented: bool,
    /// Custom test function names to analyze
    pub test_function_names: Vec<String>,
}

/// T007 rule implementation with AI enhancement
pub fn check_test_description_quality(program: &Program, _semantic: &Semantic, code: &str) -> Vec<LintIssue> {
    let config = T007Config::default();
    let mut visitor = TestDescriptionVisitor::new(program, code, &config);
    visitor.visit_program(program);
    visitor.issues
}

#[derive(Debug, Clone)]
enum DescriptionViolation {
    TooShort,
    TooLong,
    VagueDescription,
    MissingShouldPattern,
    ContainsBannedWords,
    MissingBehaviorDescription,
    NotActionOriented,
    GenericDescription,
    PoorGrammar,
    MissingContext,
}

impl DescriptionViolation {
    fn description(&self) -> &'static str {
        match self {
            DescriptionViolation::TooShort => "test description too short",
            DescriptionViolation::TooLong => "test description too long",
            DescriptionViolation::VagueDescription => "vague or unclear test description",
            DescriptionViolation::MissingShouldPattern => "missing 'should' pattern in test description",
            DescriptionViolation::ContainsBannedWords => "contains banned words or phrases",
            DescriptionViolation::MissingBehaviorDescription => "missing clear behavior description",
            DescriptionViolation::NotActionOriented => "description not action-oriented",
            DescriptionViolation::GenericDescription => "generic or non-specific description",
            DescriptionViolation::PoorGrammar => "poor grammar or unclear wording",
            DescriptionViolation::MissingContext => "missing sufficient context",
        }
    }

    fn recommendation(&self) -> &'static str {
        match self {
            DescriptionViolation::TooShort => "Provide more detailed description of what is being tested",
            DescriptionViolation::TooLong => "Make description more concise while maintaining clarity",
            DescriptionViolation::VagueDescription => "Use specific, clear language describing the expected behavior",
            DescriptionViolation::MissingShouldPattern => "Use 'should [verb] [expected outcome]' pattern",
            DescriptionViolation::ContainsBannedWords => "Remove vague words like 'works', 'correct', 'proper'",
            DescriptionViolation::MissingBehaviorDescription => "Clearly describe what behavior is expected",
            DescriptionViolation::NotActionOriented => "Start with action verbs like 'should return', 'should throw'",
            DescriptionViolation::GenericDescription => "Be specific about inputs, conditions, and expected outputs",
            DescriptionViolation::PoorGrammar => "Use proper grammar and clear sentence structure",
            DescriptionViolation::MissingContext => "Include context about when/why this behavior occurs",
        }
    }
}

#[derive(Debug, Clone)]
enum TestBlockType {
    Describe,
    It,
    Test,
    Context,
    When,
}

struct TestDescriptionVisitor<'a> {
    config: &'a T007Config,
    program: &'a Program<'a>,
    source_code: &'a str,
    issues: Vec<LintIssue>,
    banned_words: HashSet<String>,
    required_patterns: Vec<Regex>,
    test_function_names: HashSet<String>,
    current_describe_context: Vec<String>,
}

impl<'a> TestDescriptionVisitor<'a> {
    fn new(program: &'a Program<'a>, source_code: &'a str, config: &'a T007Config) -> Self {
        // Default banned words
        let mut banned_words = HashSet::from([
            "works", "correct", "proper", "right", "wrong", "good", "bad",
            "test", "check", "verify", "ensure", "make sure", "ok", "fine"
        ].map(String::from));

        banned_words.extend(config.banned_words.iter().cloned());

        // Default test function names
        let mut test_function_names = HashSet::from([
            "describe", "it", "test", "context", "when", "given",
            "suite", "spec", "scenario"
        ].map(String::from));

        test_function_names.extend(config.test_function_names.iter().cloned());

        // Required patterns
        let mut required_patterns = Vec::new();
        for pattern_str in &config.required_patterns {
            if let Ok(regex) = Regex::new(pattern_str) {
                required_patterns.push(regex);
            }
        }

        Self {
            config,
            program,
            source_code,
            issues: Vec::new(),
            banned_words,
            required_patterns,
            test_function_names,
            current_describe_context: Vec::new(),
        }
    }

    fn generate_ai_enhanced_message(&self, base_message: &str, _span: Span) -> String {
        // Simple AI enhancement - in production this would use Claude API
        base_message.to_string()
    }

    fn generate_ai_fix_suggestions(&self) -> Vec<String> {
        vec![
            "Write clear, descriptive test names that explain what is being tested".to_string(),
            "Use 'should' in test descriptions to indicate expected behavior".to_string(),
            "Avoid vague words like 'works', 'correct', 'good' in test descriptions".to_string(),
            "Include specific actions and expected outcomes in test names".to_string(),
            "Keep test descriptions between 10-120 characters for readability".to_string(),
        ]
    }

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

    fn analyze_test_description(&mut self, description: &str, block_type: TestBlockType, span: Span) {
        let violations = self.detect_violations(description, &block_type);

        for violation in violations {
            self.create_description_issue(violation, description, span);
        }
    }

    fn detect_violations(&self, description: &str, block_type: &TestBlockType) -> Vec<DescriptionViolation> {
        let mut violations = Vec::new();

        // Length checks
        if (description.len() as u32) < self.config.min_description_length {
            violations.push(DescriptionViolation::TooShort);
        }

        if (description.len() as u32) > self.config.max_description_length {
            violations.push(DescriptionViolation::TooLong);
        }

        // Should pattern for 'it' blocks
        if matches!(block_type, TestBlockType::It | TestBlockType::Test) {
            if self.config.enforce_should_pattern && !self.has_should_pattern(description) {
                violations.push(DescriptionViolation::MissingShouldPattern);
            }

            if self.config.enforce_action_oriented && !self.is_action_oriented(description) {
                violations.push(DescriptionViolation::NotActionOriented);
            }
        }

        // Banned words check
        if self.contains_banned_words(description) {
            violations.push(DescriptionViolation::ContainsBannedWords);
        }

        // Vague description patterns
        if self.is_vague_description(description) {
            violations.push(DescriptionViolation::VagueDescription);
        }

        // Generic description check
        if self.is_generic_description(description) {
            violations.push(DescriptionViolation::GenericDescription);
        }

        // Behavior description check
        if self.config.require_behavior_description && !self.has_behavior_description(description) {
            violations.push(DescriptionViolation::MissingBehaviorDescription);
        }

        // Grammar and clarity
        if self.has_poor_grammar(description) {
            violations.push(DescriptionViolation::PoorGrammar);
        }

        // Context requirement for complex tests
        if self.needs_more_context(description) {
            violations.push(DescriptionViolation::MissingContext);
        }

        violations
    }

    fn has_should_pattern(&self, description: &str) -> bool {
        let should_patterns = [
            "should ", "must ", "will ", "can ", "cannot ", "does ", "doesn't "
        ];

        let description_lower = description.to_lowercase();
        should_patterns.iter().any(|pattern| description_lower.contains(pattern))
    }

    fn is_action_oriented(&self, description: &str) -> bool {
        let action_verbs = [
            "return", "throw", "create", "update", "delete", "validate", "calculate",
            "process", "handle", "reject", "accept", "transform", "convert", "parse",
            "format", "generate", "execute", "invoke", "call", "trigger", "emit"
        ];

        let description_lower = description.to_lowercase();
        action_verbs.iter().any(|verb| description_lower.contains(verb))
    }

    fn contains_banned_words(&self, description: &str) -> bool {
        let description_lower = description.to_lowercase();
        self.banned_words.iter().any(|banned| description_lower.contains(banned))
    }

    fn is_vague_description(&self, description: &str) -> bool {
        let vague_patterns = [
            "it works", "it should work", "works correctly", "behaves properly",
            "functions as expected", "does the right thing", "is correct"
        ];

        let description_lower = description.to_lowercase();
        vague_patterns.iter().any(|pattern| description_lower.contains(pattern))
    }

    fn is_generic_description(&self, description: &str) -> bool {
        let generic_patterns = [
            "test", "check", "verify", "ensure", "basic test", "simple test",
            "unit test", "integration test", "happy path", "edge case"
        ];

        let description_lower = description.to_lowercase();
        generic_patterns.iter().any(|pattern| {
            description_lower == *pattern || description_lower.starts_with(*pattern)
        })
    }

    fn has_behavior_description(&self, description: &str) -> bool {
        // Check for specific outcome descriptions
        let behavior_indicators = [
            "return", "throw", "emit", "call", "invoke", "create", "update",
            "when", "if", "given", "after", "before", "with", "without"
        ];

        let description_lower = description.to_lowercase();
        behavior_indicators.iter().any(|indicator| description_lower.contains(indicator))
    }

    fn has_poor_grammar(&self, description: &str) -> bool {
        // Simple grammar checks
        let poor_grammar_patterns = [
            "should returns", "should throws", "should creates", "should updates",
            "should deletes", "should validates", "can returns", "will throws"
        ];

        let description_lower = description.to_lowercase();
        poor_grammar_patterns.iter().any(|pattern| description_lower.contains(pattern))
    }

    fn needs_more_context(&self, description: &str) -> bool {
        // Check if description is too abstract without context
        if description.len() < 20 {
            return false; // Short descriptions are handled by length check
        }

        let context_indicators = [
            "when", "if", "given", "with", "without", "after", "before",
            "during", "while", "unless", "provided", "containing"
        ];

        let description_lower = description.to_lowercase();
        !context_indicators.iter().any(|indicator| description_lower.contains(indicator)) &&
        description.split_whitespace().count() > 8 // Long but no context
    }

    fn create_description_issue(&mut self, violation: DescriptionViolation, description: &str, span: Span) {
        let (line, column) = self.calculate_line_column(span.start as usize);

        let message = self.generate_ai_enhanced_message(
            &format!(
                "Test description '{}' has {}: {}",
                if description.len() > 50 {
                    format!("{}...", &description[..50])
                } else {
                    description.to_string()
                },
                violation.description(),
                violation.recommendation()
            ),
            span
        );

        let severity = match violation {
            DescriptionViolation::VagueDescription | DescriptionViolation::GenericDescription => LintSeverity::Warning,
            DescriptionViolation::TooShort | DescriptionViolation::ContainsBannedWords => LintSeverity::Warning,
            _ => LintSeverity::Info,
        };

        self.issues.push(LintIssue {
            rule_name: "T007".to_string(),
            severity,
            message,
            line,
            column,
            fix_available: true,
        });
    }

    fn get_test_block_type(&self, function_name: &str) -> TestBlockType {
        match function_name {
            "describe" | "suite" => TestBlockType::Describe,
            "it" | "spec" | "scenario" => TestBlockType::It,
            "test" => TestBlockType::Test,
            "context" => TestBlockType::Context,
            "when" | "given" => TestBlockType::When,
            _ => TestBlockType::It, // Default
        }
    }
}

impl<'a> Visit<'a> for TestDescriptionVisitor<'a> {
    fn visit_call_expression(&mut self, node: &CallExpression<'a>) {
        if let Expression::Identifier(ident) = &node.callee {
            let function_name = ident.name.as_ref();

            if self.test_function_names.contains(function_name) {
                if let Some(first_arg) = node.arguments.first() {
                    // Updated for OXC: match Argument::Expression and then Expression::StringLiteral
                    if let oxc_ast::ast::Argument::Expression(expr) = first_arg {
                        if let Expression::StringLiteral(literal) = &**expr {
                            let description = literal.value.to_string();
                            let block_type = self.get_test_block_type(function_name);
                            self.analyze_test_description(&description, block_type, literal.span);
                        }
                    }
                }
            }
        }

        // Continue visiting child nodes
        self.visit_expression(&node.callee);
        // TODO: Fix argument visiting - Argument enum may have changed in OXC
        // for arg in &node.arguments {
        //     self.visit_expression(arg);
        // }
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

        check_test_description_quality(&parse_result.program, &semantic_result.semantic, code, None)
    }

    #[test]
    fn test_poor_descriptions_violation() {
        let code = r#"
describe('Calculator', () => {
    it('works', () => { // Too short and vague
        expect(true).toBe(true);
    });

    it('should test the function properly and make sure it works correctly', () => { // Contains banned words
        expect(calculate(2, 2)).toBe(4);
    });

    it('returns correct value', () => { // Missing 'should' and vague
        expect(getValue()).toBe(42);
    });
});
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("too short") || issue.message.contains("vague")));
        assert!(issues.iter().any(|issue| issue.message.contains("banned words")));
    }

    #[test]
    fn test_good_descriptions_compliant() {
        let code = r#"
describe('User Service', () => {
    describe('createUser', () => {
        it('should return user object when valid data is provided', () => {
            const userData = { name: 'John', email: 'john@example.com' };
            const result = userService.createUser(userData);
            expect(result).toHaveProperty('id');
        });

        it('should throw ValidationError when email is missing', () => {
            const userData = { name: 'John' };
            expect(() => userService.createUser(userData)).toThrow('ValidationError');
        });

        it('should generate unique ID for each new user', () => {
            const userData1 = { name: 'John', email: 'john1@example.com' };
            const userData2 = { name: 'Jane', email: 'jane@example.com' };

            const user1 = userService.createUser(userData1);
            const user2 = userService.createUser(userData2);

            expect(user1.id).not.toBe(user2.id);
        });
    });
});
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_grammar_issues_violation() {
        let code = r#"
describe('Parser', () => {
    it('should returns parsed data', () => { // Grammar error
        expect(parse('test')).toBeDefined();
    });

    it('should throws error for invalid input', () => { // Grammar error
        expect(() => parse(null)).toThrow();
    });
});
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("grammar")));
    }

    #[test]
    fn test_missing_context_violation() {
        let code = r#"
describe('Complex Business Logic', () => {
    it('should handle the complex business logic processing appropriately', () => {
        // Long description but no specific context
        expect(processBusinessLogic()).toBeTruthy();
    });
});
        "#;

        let issues = parse_and_check(code);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.message.contains("context") || issue.message.contains("vague")));
    }

    #[test]
    fn test_action_oriented_descriptions_compliant() {
        let code = r#"
describe('Email Service', () => {
    it('should send email notification when user registers', () => {
        const user = { email: 'test@example.com' };
        emailService.sendWelcomeEmail(user);
        expect(mockSend).toHaveBeenCalledWith(user.email);
    });

    it('should reject invalid email addresses', () => {
        expect(() => emailService.validate('invalid-email')).toThrow();
    });

    it('should format email template with user data', () => {
        const template = emailService.formatTemplate('welcome', { name: 'John' });
        expect(template).toContain('Hello John');
    });
});
        "#;

        let issues = parse_and_check(code);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_ai_enhancement_integration() {
        let code = r#"
describe('Tests', () => {
    it('test', () => { // Poor description
        expect(true).toBe(true);
    });

    it('works correctly', () => { // Vague description
        expect(calculate()).toBeDefined();
    });
});
        "#;

        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true).with_jsx(true);
        let parse_result = Parser::new(&allocator, code, source_type).parse();
        let semantic_result = SemanticBuilder::new().build(&parse_result.program);

        let mut ai_enhancer = AIEnhancer::new();
        ai_enhancer.set_context("Suggest clear, descriptive test descriptions".to_string());

        let issues = check_test_description_quality(&parse_result.program, &semantic_result.semantic, code, Some(&ai_enhancer));

        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.fix_available));
        assert!(issues.iter().any(|issue| issue.message.contains("too short") || issue.message.contains("vague")));
    }
}