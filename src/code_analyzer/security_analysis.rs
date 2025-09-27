//! Security analysis types and vulnerability detection
//!
//! Self-documenting security issue detection and categorization.

use serde::{Deserialize, Serialize};

/// Security issue detected during analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    pub file_path: String,
    pub severity: SecuritySeverity,
    pub category: SecurityCategory,
    pub description: String,
    pub line: u32,
    pub column: u32,
    pub code_snippet: String,
    pub recommendation: String,
    pub cwe_id: Option<u32>, // Common Weakness Enumeration ID
}

/// Severity levels for security issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Categories of security issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityCategory {
    Injection,
    Authentication,
    Authorization,
    Cryptography,
    InputValidation,
    DataExposure,
    UnusafeEval,
    PathTraversal,
    CrossSiteScripting,
    Other(String),
}

impl SecurityIssue {
    pub fn new_injection_vulnerability(file_path: String, line: u32, column: u32, code_snippet: String) -> Self {
        Self {
            file_path,
            severity: SecuritySeverity::High,
            category: SecurityCategory::Injection,
            description: "Potential injection vulnerability detected".to_string(),
            line,
            column,
            code_snippet,
            recommendation: "Sanitize user input and use parameterized queries".to_string(),
            cwe_id: Some(89), // CWE-89: SQL Injection
        }
    }

    pub fn new_unsafe_eval(file_path: String, line: u32, column: u32, code_snippet: String) -> Self {
        Self {
            file_path,
            severity: SecuritySeverity::Critical,
            category: SecurityCategory::UnusafeEval,
            description: "Use of eval() or similar dangerous function".to_string(),
            line,
            column,
            code_snippet,
            recommendation: "Replace eval() with safer alternatives like JSON.parse()".to_string(),
            cwe_id: Some(94), // CWE-94: Code Injection
        }
    }

    pub fn new_hardcoded_secret(file_path: String, line: u32, column: u32, code_snippet: String) -> Self {
        Self {
            file_path,
            severity: SecuritySeverity::High,
            category: SecurityCategory::DataExposure,
            description: "Hardcoded secret or API key detected".to_string(),
            line,
            column,
            code_snippet,
            recommendation: "Move secrets to environment variables or secure storage".to_string(),
            cwe_id: Some(798), // CWE-798: Use of Hard-coded Credentials
        }
    }
}
