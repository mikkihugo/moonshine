//! Pattern matching implementation for code analysis
//!
//! Self-documenting pattern matcher with multiple algorithms for high-performance analysis.

use crate::linter::pattern_types::*;
use aho_corasick::AhoCorasick;
use regex::Regex;
use std::collections::HashSet;

impl CodePatternMatcher {
    /// Create new pattern matcher with default patterns
    pub fn new() -> Self {
        let configurable_patterns = ConfigurablePatterns::default();
        let mut matcher = Self {
            aho_corasick_matcher: None,
            regex_patterns: Vec::new(),
            function_signatures: HashSet::new(),
            security_keywords: HashSet::new(),
            performance_anti_patterns: Vec::new(),
            typescript_specific_patterns: Vec::new(),
            configurable_patterns,
        };

        matcher.initialize_patterns();
        matcher
    }

    /// Initialize all pattern matching algorithms
    fn initialize_patterns(&mut self) {
        self.setup_aho_corasick();
        self.setup_regex_patterns();
        self.setup_function_signatures();
        self.setup_security_keywords();
        self.setup_performance_patterns();
        self.setup_typescript_patterns();
    }

    /// Setup Aho-Corasick algorithm for fast multi-pattern matching
    fn setup_aho_corasick(&mut self) {
        let patterns = vec![
            "console.log",
            "debugger",
            "eval(",
            "setTimeout",
            "setInterval",
            "innerHTML",
            "document.write",
            "var ",
        ];

        self.aho_corasick_matcher = AhoCorasick::new(patterns).ok();
    }

    /// Setup regex patterns for complex matching
    fn setup_regex_patterns(&mut self) {
        let pattern_strings = vec![
            r"function\s+\w+\s*\([^)]*\)\s*\{",       // Function declarations
            r"const\s+\w+\s*=\s*\([^)]*\)\s*=>\s*\{", // Arrow functions
            r"if\s*\([^)]+\)\s*\{",                   // If statements
            r"for\s*\([^)]+\)\s*\{",                  // For loops
            r"while\s*\([^)]+\)\s*\{",                // While loops
            r"try\s*\{",                              // Try blocks
            r"catch\s*\([^)]*\)\s*\{",                // Catch blocks
        ];

        for pattern in pattern_strings {
            if let Ok(regex) = Regex::new(pattern) {
                self.regex_patterns.push(regex);
            }
        }
    }

    /// Setup function signature patterns
    fn setup_function_signatures(&mut self) {
        self.function_signatures.insert("function".to_string());
        self.function_signatures.insert("async function".to_string());
        self.function_signatures.insert("export function".to_string());
        self.function_signatures.insert("export async function".to_string());
    }

    /// Setup security-related keywords
    fn setup_security_keywords(&mut self) {
        self.security_keywords.insert("eval".to_string());
        self.security_keywords.insert("innerHTML".to_string());
        self.security_keywords.insert("document.write".to_string());
        self.security_keywords.insert("dangerouslySetInnerHTML".to_string());
        self.security_keywords.insert("execScript".to_string());
    }

    /// Setup performance anti-patterns
    fn setup_performance_patterns(&mut self) {
        self.performance_anti_patterns.push("for (let i = 0; i < array.length; i++)".to_string());
        self.performance_anti_patterns.push("array.push()".to_string());
        self.performance_anti_patterns.push("document.getElementById".to_string());
    }

    /// Setup TypeScript-specific patterns
    fn setup_typescript_patterns(&mut self) {
        self.typescript_specific_patterns.push("any".to_string());
        self.typescript_specific_patterns.push("as any".to_string());
        self.typescript_specific_patterns.push("@ts-ignore".to_string());
        self.typescript_specific_patterns.push("@ts-nocheck".to_string());
    }

    /// Find pattern matches in code
    pub fn find_matches(&self, code: &str, file_path: &str) -> Vec<PatternMatch> {
        let mut matches = Vec::new();

        // Aho-Corasick multi-pattern matching
        if let Some(ref ac) = self.aho_corasick_matcher {
            for match_result in ac.find_iter(code) {
                matches.push(PatternMatch {
                    pattern_name: "aho_corasick_match".to_string(),
                    line_number: self.get_line_number(code, match_result.start()),
                    column_start: match_result.start() as u32,
                    column_end: match_result.end() as u32,
                    matched_text: code[match_result.start()..match_result.end()].to_string(),
                    severity: MatchSeverity::Medium,
                    confidence: 0.8,
                    description: "Pattern detected by multi-pattern matcher".to_string(),
                    suggested_fix: None,
                    category: "general".to_string(),
                });
            }
        }

        // Regex pattern matching
        for (i, regex) in self.regex_patterns.iter().enumerate() {
            for regex_match in regex.find_iter(code) {
                matches.push(PatternMatch {
                    pattern_name: format!("regex_pattern_{}", i),
                    line_number: self.get_line_number(code, regex_match.start()),
                    column_start: regex_match.start() as u32,
                    column_end: regex_match.end() as u32,
                    matched_text: regex_match.as_str().to_string(),
                    severity: MatchSeverity::Low,
                    confidence: 0.9,
                    description: "Pattern detected by regex matcher".to_string(),
                    suggested_fix: None,
                    category: "structure".to_string(),
                });
            }
        }

        // Security keyword matching
        for keyword in &self.security_keywords {
            if code.contains(keyword) {
                let line_number = self.find_keyword_line(code, keyword);
                matches.push(PatternMatch {
                    pattern_name: format!("security_{}", keyword),
                    line_number,
                    column_start: 0,
                    column_end: keyword.len() as u32,
                    matched_text: keyword.clone(),
                    severity: MatchSeverity::High,
                    confidence: 0.95,
                    description: format!("Security-sensitive keyword '{}' detected", keyword),
                    suggested_fix: Some(format!("Consider safer alternatives to '{}'", keyword)),
                    category: "security".to_string(),
                });
            }
        }

        matches
    }

    /// Get line number for character position
    fn get_line_number(&self, code: &str, char_pos: usize) -> u32 {
        code[..char_pos].lines().count() as u32
    }

    /// Find line number containing keyword
    fn find_keyword_line(&self, code: &str, keyword: &str) -> u32 {
        for (line_num, line) in code.lines().enumerate() {
            if line.contains(keyword) {
                return (line_num + 1) as u32;
            }
        }
        1
    }
}

impl Default for ConfigurablePatterns {
    fn default() -> Self {
        Self {
            function_patterns: vec!["function ".to_string(), "async function".to_string(), "export function".to_string()],
            conditional_patterns: vec!["if (".to_string(), "else if (".to_string(), "switch (".to_string()],
            loop_patterns: vec!["for (".to_string(), "while (".to_string(), "do {".to_string()],
            comment_patterns: vec!["//".to_string(), "/*".to_string(), "/**".to_string()],
            test_patterns: vec!["describe(".to_string(), "it(".to_string(), "test(".to_string(), "expect(".to_string()],
            security_patterns: vec!["eval(".to_string(), "innerHTML".to_string(), "document.write".to_string()],
            performance_patterns: vec!["array.length".to_string(), "document.getElementById".to_string(), "querySelector".to_string()],
            typescript_patterns: vec![": any".to_string(), "as any".to_string(), "@ts-ignore".to_string()],
            documentation_patterns: vec!["TODO".to_string(), "FIXME".to_string(), "XXX".to_string()],
        }
    }
}
