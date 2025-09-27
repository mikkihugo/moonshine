//! Complexity metrics and analysis for code quality assessment
//!
//! Self-documenting complexity calculation and hotspot detection.

use serde::{Deserialize, Serialize};

/// Comprehensive code complexity metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityMetrics {
    // Traditional complexity measures
    pub cyclomatic_complexity: u32,
    pub cognitive_complexity: u32,
    pub halstead_difficulty: f64,
    pub halstead_volume: f64,
    pub halstead_effort: f64,

    // Modern complexity measures
    pub nesting_depth: u32,
    pub parameter_count: u32,
    pub lines_of_code: u32,
    pub maintainability_index: f64,

    // Advanced analysis
    pub dependency_complexity: u32,
    pub fan_in: u32,      // Number of modules that depend on this
    pub fan_out: u32,     // Number of modules this depends on
    pub instability: f64, // (fan_out / (fan_in + fan_out))

    // TypeScript specific
    pub type_complexity: u32,
    pub interface_complexity: u32,
    pub generic_complexity: u32,

    // Performance indicators
    pub async_complexity: u32,
    pub promise_chain_depth: u32,
    pub callback_nesting: u32,
}

impl Default for ComplexityMetrics {
    fn default() -> Self {
        Self {
            cyclomatic_complexity: 1, // Minimum complexity
            cognitive_complexity: 0,
            halstead_difficulty: 0.0,
            halstead_volume: 0.0,
            halstead_effort: 0.0,
            nesting_depth: 0,
            parameter_count: 0,
            lines_of_code: 0,
            maintainability_index: 100.0, // Maximum maintainability
            dependency_complexity: 0,
            fan_in: 0,
            fan_out: 0,
            instability: 0.0,
            type_complexity: 0,
            interface_complexity: 0,
            generic_complexity: 0,
            async_complexity: 0,
            promise_chain_depth: 0,
            callback_nesting: 0,
        }
    }
}

/// Detailed function/method complexity analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionComplexity {
    pub name: String,
    pub start_line: u32,
    pub end_line: u32,
    pub metrics: ComplexityMetrics,
    pub complexity_hotspots: Vec<ComplexityHotspot>,
    pub refactoring_suggestions: Vec<RefactoringSuggestion>,
}

/// Specific complexity hotspot within a function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityHotspot {
    pub hotspot_type: ComplexityHotspotType,
    pub line: u32,
    pub column: u32,
    pub description: String,
    pub impact_score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityHotspotType {
    DeepNesting,
    LongParameterList,
    ComplexConditional,
    CallbackHell,
    LargeSwitch,
    RepeatedCode,
    TypeComplexity,
    AsyncComplexity,
}

/// Refactoring suggestion to reduce complexity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringSuggestion {
    pub suggestion_type: RefactoringSuggestionType,
    pub description: String,
    pub estimated_complexity_reduction: u32,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefactoringSuggestionType {
    ExtractFunction,
    ReduceNesting,
    SimplifyConditionals,
    SplitLargeFunction,
    ReduceParameters,
    ExtractClass,
    UsePolymorphism,
    CacheComputation,
    EliminateDuplication,
    UseBuiltinMethod,
    OptimizeDataStructure,
    ReduceAsyncComplexity,
}
