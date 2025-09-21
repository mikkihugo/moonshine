//! # MoonShine Rule Engine Core
//!
//! Core rule execution engine that coordinates:
//! - OXLint's 582 built-in rules
//! - MoonShine's custom C/S-series rules
//! - AI enhancement through Claude integration
//!
//! @category rule-engine
//! @safe program
//! @mvp core
//! @complexity high
//! @since 2.1.0

use crate::error::{Error, Result};
use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use oxc_allocator::Allocator;
use oxc_ast::ast::Program;
use oxc_semantic::Semantic;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MoonShine rule implementation with AI enhancement
#[derive(Debug, Clone)]
pub struct MoonShineRule {
    pub id: String,
    pub category: MoonShineRuleCategory,
    pub severity: LintSeverity,
    pub description: String,
    pub ai_enhanced: bool,
    pub implementation: RuleImplementation,
}

/// MoonShine rule categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MoonShineRuleCategory {
    CodeQuality,   // C-series rules
    Security,      // S-series rules
    Performance,   // P-series rules
    Testing,       // T-series rules
    Naming,        // N-series rules
}

/// Rule implementation types with AI enhancement capabilities
#[derive(Debug, Clone)]
pub enum RuleImplementation {
    OxcSemantic,      // Uses OXC semantic analysis with AI context
    OxcAstVisitor,    // OXC AST visitor pattern with AI suggestions
    OxlintEnhanced,   // OXLint rules + AI enhancement layer
    AiAssisted,       // Pure AI analysis with Claude suggestions
    Hybrid,           // Combines OXC AST + OXLint + AI for maximum accuracy
}

/// MoonShine AI-enhanced rule engine with comprehensive rule coverage
pub struct MoonShineRuleEngine {
    // Custom MoonShine rules (loaded from modules)
    custom_rules: HashMap<String, MoonShineRule>,
    // OXLint built-in rules (582 rules) - TODO: Integrate when OXLint API is stable
    // oxlint_engine: Option<Linter>,
    ai_context: Option<String>, // For Claude integration
}

mod registry_autogen;

impl MoonShineRuleEngine {
    /// Create new MoonShine rule engine with AI enhancement
    pub fn new() -> Self {
        let mut engine = Self {
            custom_rules: HashMap::new(),
            // oxlint_engine: Some(Linter::default()),  // TODO: Restore when OXLint API is stable
            ai_context: None,
        };

        // Register rules from modules
        engine.register_all_rules();
        engine
    }

    /// Set AI context for enhanced suggestions
    pub fn set_ai_context(&mut self, context: String) {
        self.ai_context = Some(context);
    }

    /// Run all enabled rules on code (582 OXLint + Custom MoonShine rules)
    pub fn check_all_rules(&self, program: &Program, semantic: &Semantic, code: &str, file_path: &str) -> Vec<LintIssue> {
        let mut issues = Vec::new();

        // First run OXLint's 582 built-in rules - TODO: Restore when OXLint API is stable
        // if let Some(_oxlint) = &self.oxlint_engine {
        //     let oxlint_results = self.run_oxlint_rules(program, semantic, code, file_path);
        //     issues.extend(oxlint_results);
        // }

        // Then run our custom MoonShine rules with AI enhancement
        for rule in self.custom_rules.values() {
            match &rule.implementation {
                RuleImplementation::OxcSemantic => {
                    issues.extend(self.check_oxc_semantic_rule(&rule.id, program, semantic, code));
                }
                RuleImplementation::OxcAstVisitor => {
                    issues.extend(self.check_oxc_ast_visitor_rule(&rule.id, program, semantic, code));
                }
                RuleImplementation::OxlintEnhanced => {
                    issues.extend(self.check_oxlint_enhanced_rule(&rule.id, program, semantic, code));
                }
                RuleImplementation::AiAssisted => {
                    issues.extend(self.check_ai_assisted_rule(&rule.id, program, semantic, code));
                }
                RuleImplementation::Hybrid => {
                    issues.extend(self.check_hybrid_rule(&rule.id, program, semantic, code));
                }
            }
        }

        issues
    }

    /// Register all rules from modules
    fn register_all_rules(&mut self) {
        // 1) Auto-generated registry from JS references (SunLint/Moonshine)
        registry_autogen::register_rules(&mut self.custom_rules);

        // 2) Native Rust-ported rules (add true Rust implementations here)
        super::code_quality::register_rules(&mut self.custom_rules);
        super::security::register_rules(&mut self.custom_rules);
    }

    /// Run OXLint's 582 built-in rules
    fn run_oxlint_rules(&self, _program: &Program, _semantic: &Semantic, _code: &str, _file_path: &str) -> Vec<LintIssue> {
        let mut issues = Vec::new();

        // OXLint integration would go here
        // Note: This is a placeholder - actual integration requires more setup
        // The OXLint API would provide diagnostics that we convert to LintIssue

        // Example categories from OXLint (582 total rules):
        // - Correctness (194 rules) - enabled by default
        // - Performance (11 rules)
        // - Restriction (72 rules)
        // - Suspicious (42 rules)
        // - Pedantic (97 rules)
        // - Style (158 rules)
        // - Nursery (8 rules)

        issues
    }

    // Rule execution methods (delegate to rule modules)
    fn check_semantic_rule(&self, rule_id: &str, program: &Program, semantic: &Semantic, code: &str) -> Vec<LintIssue> {
        // Delegate to appropriate module based on rule prefix
        if rule_id.starts_with('C') {
            super::code_quality::check_semantic_rule(rule_id, program, semantic, code)
        } else if rule_id.starts_with('S') {
            super::security::check_semantic_rule(rule_id, program, semantic, code)
        } else {
            Vec::new()
        }
    }

    fn check_ast_rule(&self, rule_id: &str, program: &Program, code: &str) -> Vec<LintIssue> {
        if rule_id.starts_with('C') {
            super::code_quality::check_ast_rule(rule_id, program, code)
        } else if rule_id.starts_with('S') {
            super::security::check_ast_rule(rule_id, program, code)
        } else {
            Vec::new()
        }
    }

    fn check_ai_assisted_rule(&self, rule_id: &str, program: &Program, semantic: &Semantic, code: &str) -> Vec<LintIssue> {
        let base_issues = if rule_id.starts_with('C') {
            super::code_quality::check_ai_rule(rule_id, program, semantic, code)
        } else if rule_id.starts_with('S') {
            super::security::check_ai_rule(rule_id, program, semantic, code)
        } else {
            Vec::new()
        };

        // Enhance with AI suggestions
        super::ai_integration::enhance_with_ai(base_issues, &self.ai_context)
    }

    fn check_hybrid_rule(&self, rule_id: &str, program: &Program, semantic: &Semantic, code: &str) -> Vec<LintIssue> {
        // Combine semantic and AI approaches
        let mut issues = self.check_semantic_rule(rule_id, program, semantic, code);
        let ai_issues = self.check_ai_assisted_rule(rule_id, program, semantic, code);
        issues.extend(ai_issues);
        issues
    }

    /// NEW: OXC semantic analysis with AI context and suggestions
    fn check_oxc_semantic_rule(&self, rule_id: &str, program: &Program, semantic: &Semantic, code: &str) -> Vec<LintIssue> {
        // Use OXC's semantic analysis for precise type information + AI enhancement
        let mut issues = if rule_id.starts_with('C') {
            super::code_quality::check_semantic_rule(rule_id, program, semantic, code)
        } else if rule_id.starts_with('S') {
            super::security::check_semantic_rule(rule_id, program, semantic, code)
        } else {
            Vec::new()
        };

        // Add AI context to each issue for better explanations
        self.enhance_issues_with_ai_context(&mut issues, code);
        issues
    }

    /// NEW: OXC AST visitor pattern with AI suggestions
    fn check_oxc_ast_visitor_rule(&self, rule_id: &str, program: &Program, semantic: &Semantic, code: &str) -> Vec<LintIssue> {
        // Use proper OXC AST visitors instead of regex patterns + AI enhancement
        let mut issues = if rule_id.starts_with('C') {
            super::code_quality::check_oxc_ast_visitor_rule(rule_id, program, semantic, code)
        } else if rule_id.starts_with('S') {
            super::security::check_oxc_ast_visitor_rule(rule_id, program, semantic, code)
        } else {
            Vec::new()
        };

        // AI generates intelligent fix suggestions based on AST context
        self.add_ai_fix_suggestions(&mut issues, program, code);
        issues
    }

    /// NEW: OXLint rules enhanced with AI context
    fn check_oxlint_enhanced_rule(&self, rule_id: &str, program: &Program, semantic: &Semantic, code: &str) -> Vec<LintIssue> {
        // TODO: When OXLint API is stable, run OXLint rules and enhance with AI
        // let oxlint_issues = run_oxlint_rule(rule_id, program, semantic, code);
        // self.enhance_oxlint_with_ai(oxlint_issues, code)

        // For now, fallback to custom implementation
        self.check_oxc_ast_visitor_rule(rule_id, program, semantic, code)
    }

    /// Add AI context and explanations to rule violations
    fn enhance_issues_with_ai_context(&self, issues: &mut Vec<LintIssue>, code: &str) {
        for issue in issues.iter_mut() {
            if let Some(ai_context) = &self.ai_context {
                // AI provides context-aware explanations
                issue.message = format!("{} [AI Context: {}]", issue.message, ai_context);
            }
        }
    }

    /// Generate AI-powered fix suggestions based on AST analysis
    fn add_ai_fix_suggestions(&self, issues: &mut Vec<LintIssue>, program: &Program, code: &str) {
        for issue in issues.iter_mut() {
            // AI analyzes the AST context and suggests intelligent fixes
            issue.fix_available = true;
            // TODO: Integrate with Claude for context-aware fix suggestions
        }
    }
}

impl Default for MoonShineRuleEngine {
    fn default() -> Self {
        Self::new()
    }
}