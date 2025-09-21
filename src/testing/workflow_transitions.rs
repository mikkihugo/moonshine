//! # Workflow Phase Transition Testing
//!
//! Tests for the complex 14-phase analysis workflow with proper phase transitions,
//! feedback loops, and validation gates.
//!
//! @category testing
//! @safe program
//! @complexity high
//! @since 2.0.0

use std::time::{Duration, Instant};
use std::collections::HashMap;

use crate::testing::builders::{AnalysisResultsBuilder, LintIssueBuilder};
use crate::testing::assertions::PerformanceAssertions;
use crate::config::MoonShineConfig;
use crate::analysis::AnalysisResults;
use crate::wasm_safe_linter::{LintIssue, LintSeverity};
use crate::error::Result;

/// Workflow phase definition for testing
#[derive(Debug, Clone)]
pub struct WorkflowPhase {
    pub name: String,
    pub description: String,
    pub command: String,
    pub args: Vec<String>,
    pub priority: u8,
    pub blocking: bool,
    pub validation: bool,
    pub expected_duration_ms: u64,
    pub dependencies: Vec<String>,
}

/// Workflow transition test framework
pub struct WorkflowTransitionTester {
    phases: Vec<WorkflowPhase>,
    execution_results: HashMap<String, PhaseExecutionResult>,
    feedback_loops: Vec<FeedbackLoop>,
}

impl WorkflowTransitionTester {
    pub fn new() -> Self {
        Self {
            phases: Self::create_14_phase_workflow(),
            execution_results: HashMap::new(),
            feedback_loops: Vec::new(),
        }
    }

    /// Create the complete 14-phase workflow for testing
    fn create_14_phase_workflow() -> Vec<WorkflowPhase> {
        vec![
            WorkflowPhase {
                name: "tsc".to_string(),
                description: "🔥 TypeScript compilation critical errors".to_string(),
                command: "tsc".to_string(),
                args: vec!["--noEmit".to_string(), "--strict".to_string()],
                priority: 1,
                blocking: true,
                validation: false,
                expected_duration_ms: 2000,
                dependencies: vec![],
            },
            WorkflowPhase {
                name: "eslint-fix".to_string(),
                description: "🛠️  ESLint auto-fix (layout, suggestions, problems)".to_string(),
                command: "eslint".to_string(),
                args: vec!["--fix".to_string(), "--fix-type".to_string(), "problem,suggestion,layout".to_string()],
                priority: 2,
                blocking: false,
                validation: false,
                expected_duration_ms: 1500,
                dependencies: vec!["tsc".to_string()],
            },
            WorkflowPhase {
                name: "oxc-rules-analysis".to_string(),
                description: "🔍 OXC AST-based rule analysis (600 rules)".to_string(),
                command: "moon-shine-oxc".to_string(),
                args: vec!["--all-rules".to_string(), "--ai-enhanced".to_string()],
                priority: 3,
                blocking: false,
                validation: false,
                expected_duration_ms: 3000,
                dependencies: vec!["eslint-fix".to_string()],
            },
            WorkflowPhase {
                name: "typescript-compilation-fixer".to_string(),
                description: "🔥 TypeScript compilation + critical runtime errors".to_string(),
                command: "ai-provider".to_string(),
                args: vec!["typescript_compilation_fixer".to_string()],
                priority: 4,
                blocking: true,
                validation: false,
                expected_duration_ms: 5000,
                dependencies: vec!["oxc-rules-analysis".to_string()],
            },
            WorkflowPhase {
                name: "method-implementation-completer".to_string(),
                description: "⚡ Complete method implementations + type safety".to_string(),
                command: "ai-provider".to_string(),
                args: vec!["method_implementation_completer".to_string()],
                priority: 5,
                blocking: true,
                validation: false,
                expected_duration_ms: 4000,
                dependencies: vec!["typescript-compilation-fixer".to_string()],
            },
            WorkflowPhase {
                name: "google-style-modernizer".to_string(),
                description: "🎨 Google TypeScript style + modern patterns".to_string(),
                command: "ai-provider".to_string(),
                args: vec!["google_style_modernizer".to_string()],
                priority: 6,
                blocking: false,
                validation: false,
                expected_duration_ms: 3000,
                dependencies: vec!["method-implementation-completer".to_string()],
            },
            WorkflowPhase {
                name: "complexity-analysis".to_string(),
                description: "📊 Complexity analysis and optimization".to_string(),
                command: "complexity-analyzer".to_string(),
                args: vec!["--threshold".to_string(), "10".to_string()],
                priority: 7,
                blocking: false,
                validation: false,
                expected_duration_ms: 2000,
                dependencies: vec!["google-style-modernizer".to_string()],
            },
            WorkflowPhase {
                name: "import-style-organizer".to_string(),
                description: "✨ Import organization + style consistency".to_string(),
                command: "ai-provider".to_string(),
                args: vec!["import_style_organizer".to_string()],
                priority: 8,
                blocking: false,
                validation: false,
                expected_duration_ms: 2500,
                dependencies: vec!["complexity-analysis".to_string()],
            },
            WorkflowPhase {
                name: "security-analysis".to_string(),
                description: "🔒 Security vulnerability analysis".to_string(),
                command: "codeql".to_string(),
                args: vec!["database".to_string(), "analyze".to_string()],
                priority: 9,
                blocking: false,
                validation: false,
                expected_duration_ms: 6000,
                dependencies: vec!["import-style-organizer".to_string()],
            },
            WorkflowPhase {
                name: "tsdoc-analysis".to_string(),
                description: "📚 TSDoc documentation coverage analysis".to_string(),
                command: "tsdoc-analyzer".to_string(),
                args: vec!["--target-coverage".to_string(), "90".to_string()],
                priority: 10,
                blocking: false,
                validation: false,
                expected_duration_ms: 1500,
                dependencies: vec!["security-analysis".to_string()],
            },
            WorkflowPhase {
                name: "edge-case-handler".to_string(),
                description: "🎯 Edge cases + final polish".to_string(),
                command: "ai-provider".to_string(),
                args: vec!["edge_case_handler".to_string()],
                priority: 11,
                blocking: false,
                validation: false,
                expected_duration_ms: 3500,
                dependencies: vec!["tsdoc-analysis".to_string()],
            },
            WorkflowPhase {
                name: "ai-tsdoc-enhancement".to_string(),
                description: "📚 AI TSDoc enhancement targeting 90% coverage".to_string(),
                command: "ai-provider".to_string(),
                args: vec!["tsdoc_enhancement".to_string()],
                priority: 12,
                blocking: false,
                validation: false,
                expected_duration_ms: 4000,
                dependencies: vec!["edge-case-handler".to_string()],
            },
            WorkflowPhase {
                name: "production-perfectionist".to_string(),
                description: "🏆 Zero tolerance perfection".to_string(),
                command: "ai-provider".to_string(),
                args: vec!["production_perfectionist".to_string()],
                priority: 13,
                blocking: false,
                validation: false,
                expected_duration_ms: 5000,
                dependencies: vec!["ai-tsdoc-enhancement".to_string()],
            },
            WorkflowPhase {
                name: "final-validation".to_string(),
                description: "✅ Final validation: TypeScript + ESLint zero tolerance".to_string(),
                command: "validator".to_string(),
                args: vec!["--zero-tolerance".to_string()],
                priority: 14,
                blocking: false,
                validation: true,
                expected_duration_ms: 2000,
                dependencies: vec!["production-perfectionist".to_string()],
            },
        ]
    }

    /// Execute the complete workflow with transition validation
    pub async fn execute_workflow_with_transitions(&mut self, config: &MoonShineConfig) -> Result<WorkflowExecutionSummary> {
        let workflow_start = Instant::now();
        let mut executed_phases = Vec::new();
        let mut total_suggestions = Vec::new();
        let mut restart_count = 0;
        let max_restarts = 3;

        loop {
            let mut cycle_successful = true;

            // Execute phases in priority order
            for phase in &self.phases {
                // Check dependencies
                if !self.are_dependencies_satisfied(phase, &executed_phases) {
                    continue;
                }

                // Execute phase
                let execution_result = self.execute_phase(phase).await?;
                self.execution_results.insert(phase.name.clone(), execution_result.clone());
                executed_phases.push(phase.name.clone());

                // Add suggestions from this phase
                total_suggestions.extend(execution_result.suggestions);

                // Check if validation phase triggered restart
                if phase.validation && execution_result.restart_triggered {
                    cycle_successful = false;
                    restart_count += 1;

                    if restart_count >= max_restarts {
                        return Ok(WorkflowExecutionSummary {
                            total_phases: self.phases.len(),
                            executed_phases: executed_phases.len(),
                            total_execution_time: workflow_start.elapsed(),
                            total_suggestions: total_suggestions.len(),
                            restart_count,
                            completed_successfully: false,
                            feedback_loops: self.feedback_loops.len(),
                            phase_results: self.execution_results.clone(),
                        });
                    }

                    // Clear results and restart
                    executed_phases.clear();
                    self.execution_results.clear();
                    total_suggestions.clear();
                    break;
                }
            }

            if cycle_successful {
                break; // Workflow completed successfully
            }
        }

        Ok(WorkflowExecutionSummary {
            total_phases: self.phases.len(),
            executed_phases: executed_phases.len(),
            total_execution_time: workflow_start.elapsed(),
            total_suggestions: total_suggestions.len(),
            restart_count,
            completed_successfully: true,
            feedback_loops: self.feedback_loops.len(),
            phase_results: self.execution_results.clone(),
        })
    }

    /// Check if phase dependencies are satisfied
    fn are_dependencies_satisfied(&self, phase: &WorkflowPhase, executed_phases: &[String]) -> bool {
        phase.dependencies.iter().all(|dep| executed_phases.contains(dep))
    }

    /// Execute a single phase
    async fn execute_phase(&mut self, phase: &WorkflowPhase) -> Result<PhaseExecutionResult> {
        let start_time = Instant::now();

        // Simulate phase execution based on phase type
        let suggestions = self.simulate_phase_execution(phase).await;

        // Check for restart condition (validation phase only)
        let restart_triggered = phase.validation && suggestions.iter().any(|s|
            matches!(s.severity, SuggestionSeverity::Error) && s.message.contains("requires restart")
        );

        // Record feedback loop if this phase creates one
        if restart_triggered {
            self.feedback_loops.push(FeedbackLoop {
                triggering_phase: phase.name.clone(),
                restart_reason: "Validation failed - critical errors found".to_string(),
                iteration: self.feedback_loops.len() + 1,
            });
        }

        let execution_time = start_time.elapsed();

        // Validate execution time is within expected bounds
        let performance_acceptable = execution_time.as_millis() as u64 <= phase.expected_duration_ms * 2;

        Ok(PhaseExecutionResult {
            phase_name: phase.name.clone(),
            execution_time,
            suggestions,
            restart_triggered,
            performance_acceptable,
            dependencies_satisfied: true,
        })
    }

    /// Simulate phase execution and generate appropriate suggestions
    async fn simulate_phase_execution(&self, phase: &WorkflowPhase) -> Vec<LintIssue> {
        let mut suggestions = Vec::new();

        // Simulate execution delay
        let delay_ms = (phase.expected_duration_ms / 10).min(100); // Scale down for tests
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;

        match phase.name.as_str() {
            "tsc" => {
                suggestions.push(LintIssueBuilder::error()
                    .message("Type 'string' is not assignable to type 'number'")
                    .category(SuggestionCategory::TypeSafety)
                    .line(42)
                    .build());
            },
            "eslint-fix" => {
                suggestions.push(LintIssueBuilder::warning()
                    .message("Prefer const assertion")
                    .category(SuggestionCategory::CodeStyle)
                    .line(15)
                    .build());
            },
            "oxc-rules-analysis" => {
                suggestions.push(LintIssueBuilder::info()
                    .message("Consider using nullish coalescing operator")
                    .category(SuggestionCategory::BestPractices)
                    .line(23)
                    .build());
            },
            "security-analysis" => {
                suggestions.push(LintIssueBuilder::error()
                    .message("Potential XSS vulnerability detected")
                    .category(SuggestionCategory::Security)
                    .line(67)
                    .build());
            },
            "final-validation" => {
                // Validation phase - sometimes triggers restart
                if self.execution_results.len() % 3 == 0 { // Every 3rd execution
                    suggestions.push(LintIssueBuilder::error()
                        .message("Critical validation failure - requires restart")
                        .category(SuggestionCategory::Validation)
                        .line(1)
                        .build());
                }
            },
            _ => {
                // Generic AI provider phases
                if phase.command == "ai-provider" {
                    suggestions.push(LintIssueBuilder::info()
                        .message(format!("AI enhancement from {}", phase.name))
                        .category(SuggestionCategory::AiEnhanced)
                        .line(10)
                        .build());
                }
            }
        }

        suggestions
    }

    /// Test phase dependency resolution
    pub fn test_dependency_resolution(&self) -> Result<DependencyTestResult> {
        let mut dependency_graph = HashMap::new();
        let mut circular_dependencies = Vec::new();
        let mut missing_dependencies = Vec::new();

        // Build dependency graph
        for phase in &self.phases {
            dependency_graph.insert(phase.name.clone(), phase.dependencies.clone());
        }

        // Check for circular dependencies
        for phase in &self.phases {
            if self.has_circular_dependency(&phase.name, &dependency_graph, &mut Vec::new()) {
                circular_dependencies.push(phase.name.clone());
            }
        }

        // Check for missing dependencies
        for phase in &self.phases {
            for dep in &phase.dependencies {
                if !self.phases.iter().any(|p| p.name == *dep) {
                    missing_dependencies.push(format!("{} -> {}", phase.name, dep));
                }
            }
        }

        Ok(DependencyTestResult {
            total_phases: self.phases.len(),
            circular_dependencies,
            missing_dependencies,
            dependency_graph,
            is_valid: circular_dependencies.is_empty() && missing_dependencies.is_empty(),
        })
    }

    /// Check for circular dependencies
    fn has_circular_dependency(&self, phase_name: &str, graph: &HashMap<String, Vec<String>>, path: &mut Vec<String>) -> bool {
        if path.contains(&phase_name.to_string()) {
            return true;
        }

        path.push(phase_name.to_string());

        if let Some(dependencies) = graph.get(phase_name) {
            for dep in dependencies {
                if self.has_circular_dependency(dep, graph, path) {
                    return true;
                }
            }
        }

        path.pop();
        false
    }
}

/// Result of executing a single phase
#[derive(Debug, Clone)]
pub struct PhaseExecutionResult {
    pub phase_name: String,
    pub execution_time: Duration,
    pub suggestions: Vec<LintIssue>,
    pub restart_triggered: bool,
    pub performance_acceptable: bool,
    pub dependencies_satisfied: bool,
}

/// Feedback loop information
#[derive(Debug, Clone)]
pub struct FeedbackLoop {
    pub triggering_phase: String,
    pub restart_reason: String,
    pub iteration: usize,
}

/// Summary of complete workflow execution
#[derive(Debug, Clone)]
pub struct WorkflowExecutionSummary {
    pub total_phases: usize,
    pub executed_phases: usize,
    pub total_execution_time: Duration,
    pub total_suggestions: usize,
    pub restart_count: usize,
    pub completed_successfully: bool,
    pub feedback_loops: usize,
    pub phase_results: HashMap<String, PhaseExecutionResult>,
}

/// Dependency resolution test result
#[derive(Debug, Clone)]
pub struct DependencyTestResult {
    pub total_phases: usize,
    pub circular_dependencies: Vec<String>,
    pub missing_dependencies: Vec<String>,
    pub dependency_graph: HashMap<String, Vec<String>>,
    pub is_valid: bool,
}

impl WorkflowExecutionSummary {
    pub fn success_rate(&self) -> f64 {
        if self.total_phases == 0 {
            0.0
        } else {
            (self.executed_phases as f64 / self.total_phases as f64) * 100.0
        }
    }

    pub fn print_summary(&self) {
        println!("🔄 Workflow Execution Summary:");
        println!("  Total phases: {}", self.total_phases);
        println!("  Executed phases: {}", self.executed_phases);
        println!("  Success rate: {:.1}%", self.success_rate());
        println!("  Total execution time: {:?}", self.total_execution_time);
        println!("  Total suggestions: {}", self.total_suggestions);
        println!("  Restart count: {}", self.restart_count);
        println!("  Feedback loops: {}", self.feedback_loops);
        println!("  Completed successfully: {}", self.completed_successfully);
    }
}

#[cfg(test)]
mod workflow_transition_tests {
    use super::*;
    use crate::testing::builders::ConfigBuilder;

    #[tokio::test]
    async fn test_workflow_phase_creation() {
        let tester = WorkflowTransitionTester::new();
        assert_eq!(tester.phases.len(), 14);

        // Verify phase order
        assert_eq!(tester.phases[0].name, "tsc");
        assert_eq!(tester.phases[13].name, "final-validation");

        // Verify priorities are sequential
        for (i, phase) in tester.phases.iter().enumerate() {
            assert_eq!(phase.priority, (i + 1) as u8);
        }
    }

    #[tokio::test]
    async fn test_dependency_resolution() {
        let tester = WorkflowTransitionTester::new();
        let dep_result = tester.test_dependency_resolution().unwrap();

        assert!(dep_result.is_valid, "Workflow should have valid dependencies");
        assert!(dep_result.circular_dependencies.is_empty(), "No circular dependencies");
        assert!(dep_result.missing_dependencies.is_empty(), "No missing dependencies");
    }

    #[tokio::test]
    async fn test_single_phase_execution() {
        let mut tester = WorkflowTransitionTester::new();
        let tsc_phase = &tester.phases[0].clone();

        let result = tester.execute_phase(tsc_phase).await.unwrap();

        assert_eq!(result.phase_name, "tsc");
        assert!(!result.suggestions.is_empty());
        assert!(result.performance_acceptable);
        assert!(result.dependencies_satisfied);
    }

    #[tokio::test]
    async fn test_workflow_execution_simple() {
        let mut tester = WorkflowTransitionTester::new();
        let config = ConfigBuilder::testing().build();

        let summary = tester.execute_workflow_with_transitions(&config).await.unwrap();

        assert!(summary.executed_phases > 0);
        assert!(summary.total_suggestions > 0);
        assert!(summary.total_execution_time < Duration::from_secs(30));
    }

    #[tokio::test]
    async fn test_feedback_loops() {
        let mut tester = WorkflowTransitionTester::new();
        let config = ConfigBuilder::testing().build();

        let summary = tester.execute_workflow_with_transitions(&config).await.unwrap();

        // Feedback loops may or may not occur depending on validation
        assert!(summary.restart_count <= 3); // Max restarts

        if summary.restart_count > 0 {
            assert!(!tester.feedback_loops.is_empty());
        }
    }

    #[tokio::test]
    async fn test_blocking_phase_behavior() {
        let tester = WorkflowTransitionTester::new();

        // Find blocking phases
        let blocking_phases: Vec<_> = tester.phases.iter()
            .filter(|p| p.blocking)
            .collect();

        assert!(!blocking_phases.is_empty());

        // Verify critical phases are blocking
        assert!(blocking_phases.iter().any(|p| p.name == "tsc"));
        assert!(blocking_phases.iter().any(|p| p.name == "typescript-compilation-fixer"));
    }

    #[tokio::test]
    async fn test_validation_phase_behavior() {
        let tester = WorkflowTransitionTester::new();

        // Find validation phases
        let validation_phases: Vec<_> = tester.phases.iter()
            .filter(|p| p.validation)
            .collect();

        assert_eq!(validation_phases.len(), 1);
        assert_eq!(validation_phases[0].name, "final-validation");
    }

    #[tokio::test]
    async fn test_performance_requirements() {
        let mut tester = WorkflowTransitionTester::new();

        // Test that each phase completes within expected time
        for phase in &tester.phases.clone() {
            let result = tester.execute_phase(phase).await.unwrap();

            // Should complete within 2x expected duration (allows for variance)
            let max_allowed = phase.expected_duration_ms * 2;
            assert!(result.execution_time.as_millis() as u64 <= max_allowed,
                   "Phase {} took too long: {}ms > {}ms",
                   phase.name, result.execution_time.as_millis(), max_allowed);
        }
    }

    #[tokio::test]
    async fn test_complete_workflow_with_restarts() {
        let mut tester = WorkflowTransitionTester::new();
        let config = ConfigBuilder::performance().build();

        let summary = tester.execute_workflow_with_transitions(&config).await.unwrap();

        // Should handle restarts gracefully
        assert!(summary.restart_count <= 3);

        if summary.completed_successfully {
            assert_eq!(summary.executed_phases, 14); // All phases completed
        }

        summary.print_summary();
    }
}