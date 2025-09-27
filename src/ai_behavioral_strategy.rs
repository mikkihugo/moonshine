//! AI Behavioral Analysis Strategy
//!
//! This is the crown jewel - our own behavioral patterns analyzed by AI
//! that go beyond static rules to understand code intent and quality.

use crate::types::{DiagnosticSeverity, LintDiagnostic};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 3-Layer AI Architecture for Code Analysis
///
/// Layer 1: Static Rules (200 core rules) - Fast, deterministic
/// Layer 2: AI Enhancement (any violation) - Contextual, intelligent  
/// Layer 3: AI Behavioral (our own patterns) - Deep understanding

pub struct AiBehavioralEngine {
    // Our custom behavioral patterns (not from ESLint/any vendor)
    behavioral_patterns: Vec<BehavioralPattern>,

    // AI models for pattern detection
    ai_models: HashMap<String, Box<dyn AiModel>>,

    // Learning from codebase patterns
    pattern_learner: PatternLearner,
}

/// Our own behavioral patterns - unique to Moon Shine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralPattern {
    pub id: String,
    pub name: String,
    pub description: String,
    pub pattern_type: BehavioralPatternType,
    pub ai_prompt: String,
    pub confidence_threshold: f32,
    pub examples: Vec<PatternExample>,
}

/// Types of behavioral patterns we detect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BehavioralPatternType {
    // Cognitive & Mental Model Patterns
    CognitiveOverload,   // Too many concepts in one function
    MentalModelMismatch, // Code doesn't match expected mental model
    ContextSwitching,    // Frequent context changes

    // Architectural Patterns
    GodObject,          // Classes doing too much
    CircularDependency, // Circular imports/dependencies
    FeatureEnvy,        // Methods using other objects more than self
    ShotgunSurgery,     // Changes require many small edits

    // Performance Behavioral Patterns
    PrematureOptimization, // Optimizing before measuring
    ResourceLeak,          // Memory/file handles not released
    NPlusOneQuery,         // Database query patterns
    RenderThrashing,       // Excessive DOM updates

    // Security Behavioral Patterns
    TrustBoundaryViolation, // Trusting untrusted data
    PrivilegeEscalation,    // Security context issues
    DataExposure,           // Sensitive data in logs/errors

    // Maintainability Patterns
    CopyPasteProgramming,  // Duplicated code patterns
    DeadCode,              // Unreachable code
    SpeculativeGenerality, // Over-engineering
    RefusedBequest,        // Inheriting but not using

    // Team & Process Patterns
    KnowledgeSilo, // Only one person understands
    BusFactor,     // Single point of failure
    TechnicalDebt, // Quick fixes accumulating
}

/// Example of a pattern for AI training
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternExample {
    pub code_snippet: String,
    pub is_violation: bool,
    pub explanation: String,
    pub suggested_fix: Option<String>,
}

impl AiBehavioralEngine {
    /// Analyze code for behavioral patterns using AI
    pub async fn analyze_behavioral_patterns(
        &self,
        source_code: &str,
        file_path: &str,
        context: &AnalysisContext,
    ) -> Result<Vec<BehavioralViolation>, Box<dyn std::error::Error>> {
        let mut violations = Vec::new();

        // Analyze each behavioral pattern type
        for pattern in &self.behavioral_patterns {
            let analysis_result = self.analyze_pattern_type(pattern, source_code, file_path, context).await?;

            if analysis_result.confidence >= pattern.confidence_threshold {
                violations.push(BehavioralViolation {
                    pattern: pattern.clone(),
                    confidence: analysis_result.confidence,
                    explanation: analysis_result.explanation,
                    suggested_fix: analysis_result.suggested_fix,
                    line: analysis_result.line,
                    column: analysis_result.column,
                    code_snippet: analysis_result.code_snippet,
                });
            }
        }

        Ok(violations)
    }

    /// Analyze specific pattern type with AI
    async fn analyze_pattern_type(
        &self,
        pattern: &BehavioralPattern,
        source_code: &str,
        file_path: &str,
        context: &AnalysisContext,
    ) -> Result<PatternAnalysisResult, Box<dyn std::error::Error>> {
        // Create AI prompt for this specific behavioral pattern
        let prompt = self.create_behavioral_prompt(pattern, source_code, file_path, context);

        // Use appropriate AI model for this pattern type
        let model_name = self.get_model_for_pattern_type(&pattern.pattern_type);
        let ai_response = self.call_ai_model(model_name, &prompt).await?;

        // Parse AI response into structured result
        Ok(PatternAnalysisResult {
            confidence: ai_response.confidence,
            explanation: ai_response.explanation,
            suggested_fix: ai_response.suggested_fix,
            line: ai_response.line,
            column: ai_response.column,
            code_snippet: ai_response.code_snippet,
        })
    }

    /// Create AI prompt for behavioral pattern analysis
    fn create_behavioral_prompt(&self, pattern: &BehavioralPattern, source_code: &str, file_path: &str, context: &AnalysisContext) -> String {
        format!(
            "Analyze this code for the behavioral pattern: {}\n\n\
            Pattern Description: {}\n\
            Pattern Type: {:?}\n\n\
            Code to analyze:\n```typescript\n{}\n```\n\n\
            Context:\n\
            - File: {}\n\
            - Project Type: {}\n\
            - Team Size: {}\n\
            - Codebase Age: {}\n\n\
            Please analyze if this code exhibits the '{}' pattern and provide:\n\
            1. Confidence score (0-1)\n\
            2. Detailed explanation of why this pattern exists\n\
            3. Specific suggestions for improvement\n\
            4. Code example of better approach\n\
            5. Line/column where pattern is most evident\n\n\
            Focus on the behavioral and architectural aspects, not just syntax.",
            pattern.name,
            pattern.description,
            pattern.pattern_type,
            source_code,
            file_path,
            context.project_type,
            context.team_size,
            context.codebase_age,
            pattern.name
        )
    }

    /// Get appropriate AI model for pattern type
    fn get_model_for_pattern_type(&self, pattern_type: &BehavioralPatternType) -> &str {
        match pattern_type {
            BehavioralPatternType::CognitiveOverload | BehavioralPatternType::MentalModelMismatch | BehavioralPatternType::ContextSwitching => {
                "claude-cognitive"
            }

            BehavioralPatternType::GodObject | BehavioralPatternType::CircularDependency | BehavioralPatternType::FeatureEnvy => "claude-architectural",

            BehavioralPatternType::PrematureOptimization | BehavioralPatternType::ResourceLeak | BehavioralPatternType::NPlusOneQuery => "claude-performance",

            BehavioralPatternType::TrustBoundaryViolation | BehavioralPatternType::PrivilegeEscalation | BehavioralPatternType::DataExposure => {
                "claude-security"
            }

            _ => "claude-general",
        }
    }

    /// Call AI model (integrate with actual AI providers)
    async fn call_ai_model(&self, model_name: &str, prompt: &str) -> Result<AiBehavioralResponse, Box<dyn std::error::Error>> {
        // TODO: Integrate with Claude/Gemini/etc.
        // For now, return mock response
        Ok(AiBehavioralResponse {
            confidence: 0.85,
            explanation: "This code exhibits the behavioral pattern due to...".to_string(),
            suggested_fix: Some("Consider refactoring to...".to_string()),
            line: 42,
            column: 8,
            code_snippet: "// Problematic code snippet".to_string(),
        })
    }
}

/// Result of behavioral pattern analysis
#[derive(Debug)]
pub struct BehavioralViolation {
    pub pattern: BehavioralPattern,
    pub confidence: f32,
    pub explanation: String,
    pub suggested_fix: Option<String>,
    pub line: u32,
    pub column: u32,
    pub code_snippet: String,
}

/// Analysis context for behavioral patterns
#[derive(Debug)]
pub struct AnalysisContext {
    pub project_type: String,
    pub team_size: u32,
    pub codebase_age: String,
    pub tech_stack: Vec<String>,
    pub business_domain: String,
}

/// AI model interface - using BoxFuture for dyn compatibility
pub trait AiModel: Send + Sync {
    fn analyze(
        &self,
        prompt: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<AiBehavioralResponse, Box<dyn std::error::Error>>> + Send + '_>>;
}

/// AI response for behavioral analysis
#[derive(Debug)]
pub struct AiBehavioralResponse {
    pub confidence: f32,
    pub explanation: String,
    pub suggested_fix: Option<String>,
    pub line: u32,
    pub column: u32,
    pub code_snippet: String,
}

/// Pattern analysis result
#[derive(Debug)]
pub struct PatternAnalysisResult {
    pub confidence: f32,
    pub explanation: String,
    pub suggested_fix: Option<String>,
    pub line: u32,
    pub column: u32,
    pub code_snippet: String,
}

/// Pattern learner for adaptive behavioral analysis
pub struct PatternLearner {
    learned_patterns: HashMap<String, LearnedPattern>,
}

#[derive(Debug)]
pub struct LearnedPattern {
    pub pattern_signature: String,
    pub frequency: u32,
    pub success_rate: f32,
    pub examples: Vec<String>,
}

/// Our custom behavioral patterns (examples)
pub fn get_custom_behavioral_patterns() -> Vec<BehavioralPattern> {
    vec![
        BehavioralPattern {
            id: "cognitive-overload".to_string(),
            name: "Cognitive Overload".to_string(),
            description: "Function or class that requires too much mental effort to understand".to_string(),
            pattern_type: BehavioralPatternType::CognitiveOverload,
            ai_prompt: "Analyze cognitive complexity...".to_string(),
            confidence_threshold: 0.8,
            examples: vec![PatternExample {
                code_snippet: "function processUserData(user, settings, validation, transformation, output) { /* 200 lines */ }".to_string(),
                is_violation: true,
                explanation: "Too many parameters and responsibilities".to_string(),
                suggested_fix: Some("Break into smaller functions".to_string()),
            }],
        },
        BehavioralPattern {
            id: "god-object".to_string(),
            name: "God Object".to_string(),
            description: "Class that knows too much or does too much".to_string(),
            pattern_type: BehavioralPatternType::GodObject,
            ai_prompt: "Analyze class responsibilities...".to_string(),
            confidence_threshold: 0.75,
            examples: vec![PatternExample {
                code_snippet: "class UserManager { /* handles auth, validation, storage, email, logging, etc. */ }".to_string(),
                is_violation: true,
                explanation: "Single class handling multiple concerns".to_string(),
                suggested_fix: Some("Apply Single Responsibility Principle".to_string()),
            }],
        },
        BehavioralPattern {
            id: "premature-optimization".to_string(),
            name: "Premature Optimization".to_string(),
            description: "Optimizing code before measuring actual performance impact".to_string(),
            pattern_type: BehavioralPatternType::PrematureOptimization,
            ai_prompt: "Look for optimization without measurement...".to_string(),
            confidence_threshold: 0.7,
            examples: vec![PatternExample {
                code_snippet: "// Micro-optimizing this loop that runs once per day".to_string(),
                is_violation: true,
                explanation: "Optimizing code that doesn't impact performance".to_string(),
                suggested_fix: Some("Measure first, optimize second".to_string()),
            }],
        },
    ]
}

/// Key Benefits of AI Behavioral Analysis:
///
/// 1. **Unique Value**: Our own patterns, not vendor rules
/// 2. **Deep Understanding**: AI analyzes intent, not just syntax
/// 3. **Contextual**: Considers project type, team size, business domain
/// 4. **Adaptive**: Learns from codebase patterns over time
/// 5. **Comprehensive**: Covers cognitive, architectural, performance, security
/// 6. **Actionable**: Provides specific fixes, not just warnings
/// 7. **Confidence Scoring**: AI provides confidence levels for decisions
pub fn get_behavioral_benefits() -> Vec<&'static str> {
    vec![
        "Unique Value: Our own patterns, not vendor rules",
        "Deep Understanding: AI analyzes intent, not just syntax",
        "Contextual: Considers project type, team size, business domain",
        "Adaptive: Learns from codebase patterns over time",
        "Comprehensive: Covers cognitive, architectural, performance, security",
        "Actionable: Provides specific fixes, not just warnings",
        "Confidence Scoring: AI provides confidence levels for decisions",
    ]
}
