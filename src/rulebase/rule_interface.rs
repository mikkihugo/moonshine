//! # Rule Interface for KV-Based Rules
//!
//! Simple, fast interface for all rules in our rulebase.
//! Optimized for KV storage and high-speed execution.

use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Simple rule interface for our rulebase
#[async_trait]
pub trait Rule: Send + Sync {
    /// Rule identifier
    fn id(&self) -> &str;

    /// Rule name
    fn name(&self) -> &str;

    /// Rule description
    fn description(&self) -> &str;

    /// Execute rule on code
    async fn execute(&self, context: &RuleContext) -> Result<RuleResult>;
}

/// Context for rule execution
#[derive(Debug, Clone)]
pub struct RuleContext {
    /// Source code
    pub code: String,
    /// File path
    pub file_path: String,
    /// Rule configuration
    pub config: HashMap<String, String>,
    /// Shared context between rules
    pub shared: HashMap<String, String>,
}

/// Rule execution result - single lint issue
#[derive(Debug, Clone, Default)]
pub struct RuleResult {
    /// Rule ID that generated this issue
    pub rule_id: String,
    /// Issue message
    pub message: String,
    /// Issue severity
    pub severity: String,
    /// Line number
    pub line: u32,
    /// Column number
    pub column: u32,
    /// Optional fix suggestion
    pub suggestion: Option<String>,
}

/// Rule categories
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuleCategory {
    Security,
    Performance,
    CodeQuality,
    TypeScript,
    JavaScript,
    React,
    NodeJS,
    Testing,
    Accessibility,
    Other,
}

/// Rule severity
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuleSeverity {
    Error,
    Warning,
    Info,
    Style,
}

impl RuleContext {
    /// Create new rule context
    pub fn new(code: String, file_path: String) -> Self {
        Self {
            code,
            file_path,
            config: HashMap::new(),
            shared: HashMap::new(),
        }
    }

    /// Add configuration
    pub fn with_config(mut self, key: String, value: String) -> Self {
        self.config.insert(key, value);
        self
    }

    /// Add shared data
    pub fn with_shared(mut self, key: String, value: String) -> Self {
        self.shared.insert(key, value);
        self
    }
}

impl RuleResult {
    /// Create new result
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

    /// Set severity
    pub fn with_severity(mut self, severity: String) -> Self {
        self.severity = severity;
        self
    }

    /// Set location
    pub fn with_location(mut self, line: u32, column: u32) -> Self {
        self.line = line;
        self.column = column;
        self
    }

    /// Set suggestion
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }
}
