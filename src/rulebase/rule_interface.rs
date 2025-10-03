//! # Rule Interface for KV-Based Rules
//!
//! Simple, fast interface for all rules in our rulebase.
//! Optimized for KV storage and high-speed execution.

use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A simple rule interface for the rulebase.
#[async_trait]
pub trait Rule: Send + Sync {
    /// Returns the unique identifier for the rule.
    fn id(&self) -> &str;

    /// Returns the name of the rule.
    fn name(&self) -> &str;

    /// Returns a description of the rule's purpose.
    fn description(&self) -> &str;

    /// Executes the rule on the given code.
    async fn execute(&self, context: &RuleContext) -> Result<RuleResult>;
}

/// The context for a rule's execution.
#[derive(Debug, Clone)]
pub struct RuleContext {
    /// The source code to be analyzed.
    pub code: String,
    /// The path of the file being analyzed.
    pub file_path: String,
    /// The configuration for the rule.
    pub config: HashMap<String, String>,
    /// A shared context between rules.
    pub shared: HashMap<String, String>,
}

/// The result of a rule's execution, representing a single lint issue.
#[derive(Debug, Clone, Default)]
pub struct RuleResult {
    /// The ID of the rule that generated this issue.
    pub rule_id: String,
    /// A message describing the issue.
    pub message: String,
    /// The severity of the issue.
    pub severity: String,
    /// The line number where the issue was found.
    pub line: u32,
    /// The column number where the issue was found.
    pub column: u32,
    /// An optional suggestion for fixing the issue.
    pub suggestion: Option<String>,
}

/// The categories that a rule can belong to.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuleCategory {
    /// Rules related to security vulnerabilities.
    Security,
    /// Rules related to performance issues.
    Performance,
    /// Rules related to code quality and best practices.
    CodeQuality,
    /// Rules specific to TypeScript.
    TypeScript,
    /// Rules specific to JavaScript.
    JavaScript,
    /// Rules specific to React.
    React,
    /// Rules specific to NodeJS.
    NodeJS,
    /// Rules related to testing.
    Testing,
    /// Rules related to accessibility.
    Accessibility,
    /// Any other type of rule.
    Other,
}

/// The severity of a rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuleSeverity {
    /// An error that must be fixed.
    Error,
    /// A warning that should be addressed.
    Warning,
    /// An informational message.
    Info,
    /// A style suggestion.
    Style,
}

impl RuleContext {
    /// Creates a new `RuleContext`.
    pub fn new(code: String, file_path: String) -> Self {
        Self {
            code,
            file_path,
            config: HashMap::new(),
            shared: HashMap::new(),
        }
    }

    /// Adds a configuration key-value pair to the context.
    pub fn with_config(mut self, key: String, value: String) -> Self {
        self.config.insert(key, value);
        self
    }

    /// Adds a shared data key-value pair to the context.
    pub fn with_shared(mut self, key: String, value: String) -> Self {
        self.shared.insert(key, value);
        self
    }
}

impl RuleResult {
    /// Creates a new `RuleResult`.
    pub fn new(rule_id: String, message: String) -> Self {
        Self {
            rule_id,
            message,
            severity: "warning".to_string(),
            line: 1,
            column: 1,
            suggestion: None,
        }
    }

    /// Sets the severity of the result.
    pub fn with_severity(mut self, severity: String) -> Self {
        self.severity = severity;
        self
    }

    /// Sets the location of the result.
    pub fn with_location(mut self, line: u32, column: u32) -> Self {
        self.line = line;
        self.column = column;
        self
    }

    /// Sets the suggestion for the result.
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }
}
