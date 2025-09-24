//! Adaptive workflow planning primitives.
//!
//! Provides basic assessment and strategy enums used to tailor the lint workflow
//! based on file complexity and expected issue count.

use serde::{Deserialize, Serialize};

/// Lightweight metrics gathered before constructing the full workflow plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickAssessment {
    pub complexity_score: f64,
    pub estimated_issues: u32,
    pub ai_recommended: bool,
}

/// High-level strategy determining how much AI/advanced analysis to run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIStrategy {
    SkipAI {
        reason: String,
    },
    LightAI {
        target_issues: u32,
        budget_estimate: f64,
    },
    StandardAI {
        passes: u32,
        budget_estimate: f64,
    },
    HeavyAI {
        passes: u32,
        specialized_models: Vec<String>,
        budget_estimate: f64,
    },
}

/// Minimal façade so callers can construct a planner even before it's feature-complete.
#[derive(Debug, Default)]
pub struct AdaptiveWorkflowPlanner;

impl AdaptiveWorkflowPlanner {
    pub fn new() -> Self {
        Self
    }
}
