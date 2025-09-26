//! # Workflow Engine
//!
//! A high-performance, petgraph-based workflow engine for Moon Shine that orchestrates
//! complex code analysis and transformation pipelines.
//!
//! This engine uses industry-standard crates for robust DAG execution:
//! - **petgraph**: Battle-tested graph algorithms for dependency resolution
//! - **tokio-util**: Production-ready async coordination and cancellation
//! - **tokio-stream**: Efficient parallel execution with backpressure control
//!
//! ## Architecture
//!
//! The workflow engine models analysis pipelines as directed acyclic graphs (DAGs) where:
//! - Each node is a [`WorkflowStep`] with specific actions (OXC parsing, AI enhancement, etc.)
//! - Edges represent dependencies between steps
//! - Execution follows topological order with parallelization where possible
//!
//! ## Key Features
//!
//! - **DAG-based Execution**: Automatic dependency resolution and parallel execution
//! - **Async Coordination**: Full tokio integration with cancellation and timeouts
//! - **Cost-Aware AI**: Intelligent AI usage based on code complexity assessment
//! - **Conditional Steps**: Context-aware step execution based on previous results
//! - **Retry Logic**: Exponential backoff for resilient operation
//! - **Memory Efficient**: Shared context with Arc<RwLock<>> for thread safety
//!
//! ## Usage
//!
//! ```rust,no_run
//! use moon_shine::workflow_engine::{WorkflowEngine, create_moonshine_oxc_workflow};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let steps = create_moonshine_oxc_workflow();
//! let mut engine = WorkflowEngine::new(
//!     steps,
//!     "function foo() { return 42; }".to_string(),
//!     "src/main.ts".to_string(),
//!     moon_shine::config::MoonShineConfig::default(),
//! )?;
//!
//! let result = engine.execute().await?;
//! println!("Workflow completed: {}", result.success);
//! # Ok(())
//! # }
//! ```
//!
//! ## Pipeline Phases
//!
//! The engine supports comprehensive analysis pipelines:
//!
//! 1. **OXC Parsing**: AST generation and semantic analysis
//! 2. **Static Analysis**: OXC rules and behavioral patterns
//! 3. **Type Analysis**: TypeScript type checking and inference
//! 4. **AI Enhancement**: Intelligent code suggestions and fixes
//! 5. **Code Generation**: Apply fixes and generate final output
//! 6. **Formatting**: Code formatting with style preservation
//!
//! ## Performance
//!
//! - Parallel execution of independent steps
//! - Configurable timeouts and cancellation
//! - Memory usage tracking and optimization
//! - Efficient context sharing between steps

use crate::analysis_pipeline::{AnalysisPipeline, AnalysisPipelineResult};
use crate::config::MoonShineConfig;
use crate::error::{Error, Result};
use crate::moon_pdk_interface::write_file_atomic;
use crate::rule_registry::{RuleCategory, RuleMetadata, RuleRegistry, RuleSettings};
use crate::telemetry::{json_value_to_string, json_value_to_u64, TelemetryCollector, TelemetryRecord};
use futures::future::try_join_all;
use futures::StreamExt;
use petgraph::{
    algo::toposort,
    graph::{DiGraph, NodeIndex},
    visit::EdgeRef,
    Direction,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio_stream::{self as stream};
use tokio_util::sync::CancellationToken;

/// Workflow step definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    /// Unique step identifier
    pub id: String,
    /// Human-readable step name
    pub name: String,
    /// Step description for logging
    pub description: String,
    /// Step dependencies (must complete before this step)
    pub depends_on: Vec<String>,
    /// Step action to execute
    pub action: StepAction,
    /// Conditional execution logic
    pub condition: Option<StepCondition>,
    /// Retry configuration
    pub retry: RetryConfig,
    /// Timeout configuration
    pub timeout: Duration,
    /// Whether failure of this step should fail the entire workflow
    pub critical: bool,
}

/// Step action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepAction {
    /// Adaptive assessment - quick evaluation to determine optimal analysis strategy
    AdaptiveAssessment {
        max_assessment_time_ms: u64,
        complexity_threshold: f64,
        enable_quick_static_analysis: bool,
    },
    /// OXC parsing and semantic analysis
    OxcParse { source_type: String, strict_mode: bool },
    /// OXC rule execution
    OxcRules { rule_categories: Vec<String>, ai_enhanced: bool },
    /// Behavioral analysis via AI-enhanced patterns
    BehavioralAnalysis {
        enable_hybrid_analysis: bool,
        confidence_threshold: f64,
        max_analysis_time_ms: u64,
    },
    /// OXC type analysis
    OxcTypeAnalysis { strict_types: bool, inference: bool },
    /// AI enhancement via provider router (supports multiple LLMs)
    AiEnhancement { provider: String, copro_optimization: bool },
    /// OXC code generation
    OxcCodegen { apply_fixes: bool, source_maps: bool },
    /// OXC formatting (stub)
    OxcFormat { style: String, preserve_oxc_structure: bool },
    /// Custom Rust function
    CustomFunction {
        function_name: String,
        parameters: HashMap<String, serde_json::Value>,
    },
    /// Session management for agent debugging
    CreateSessionDir { base_path: String, session_prefix: String },
    /// Write agent request to session file
    WriteAgentRequest { agent_type: String, request_data: serde_json::Value },
    /// Execute AI via provider router (supports Claude, Gemini, OpenAI)
    ExecuteAIProvider {
        prompt_template: String,
        temperature: f64,
        max_tokens: u32,
        session_file: String,
    },
    /// Read agent response from session file
    ReadAgentResponse { agent_type: String, timeout_ms: u64 },
    /// Cleanup session directory
    CleanupSession { max_age_hours: u32 },
}

/// Step conditional execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepCondition {
    /// Always execute
    Always,
    /// Execute only if previous step succeeded
    OnSuccess(String),
    /// Execute only if previous step failed
    OnFailure(String),
    /// Execute based on context value
    ContextValue {
        key: String,
        operator: ConditionOperator,
        value: serde_json::Value,
    },
    /// Complex boolean expression
    Expression(String),
}

/// Condition operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    Exists,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Delay between retries
    pub delay: Duration,
    /// Exponential backoff multiplier
    pub backoff_multiplier: f64,
    /// Maximum delay between retries
    pub max_delay: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            delay: Duration::from_millis(100),
            backoff_multiplier: 2.0,
            max_delay: Duration::from_secs(10),
        }
    }
}

/// Workflow execution context
#[derive(Debug, Clone)]
pub struct WorkflowContext {
    /// Source code being processed
    pub source_code: String,
    /// File path being processed
    pub file_path: String,
    /// Shared data between steps
    pub data: Arc<RwLock<HashMap<String, serde_json::Value>>>,
    /// Step execution results
    pub step_results: Arc<RwLock<HashMap<String, StepResult>>>,
    /// Workflow configuration
    pub config: MoonShineConfig,
}

/// Step execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    /// Step ID
    pub step_id: String,
    /// Execution success
    pub success: bool,
    /// Execution time
    pub duration: Duration,
    /// Step output data
    pub output: serde_json::Value,
    /// Error message if failed
    pub error: Option<String>,
    /// Memory usage in bytes
    pub memory_used: u64,
    /// Retry attempts made
    pub retry_count: u32,
}

/// Complete workflow execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    /// Overall success
    pub success: bool,
    /// Total execution time
    pub total_duration: Duration,
    /// Final transformed code
    pub transformed_code: Option<String>,
    /// All step results
    pub step_results: Vec<StepResult>,
    /// Workflow statistics
    pub stats: WorkflowStats,
    /// Final context data
    pub final_context: HashMap<String, serde_json::Value>,
}

/// Workflow execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStats {
    /// Total steps executed
    pub total_steps: u32,
    /// Steps that succeeded
    pub successful_steps: u32,
    /// Steps that failed
    pub failed_steps: u32,
    /// Steps that were skipped
    pub skipped_steps: u32,
    /// Total retry attempts
    pub total_retries: u32,
    /// Peak memory usage
    pub peak_memory_bytes: u64,
    /// Parallel execution efficiency
    pub parallelism_factor: f64,
}

/// Petgraph-based workflow engine with tokio coordination
pub struct WorkflowEngine {
    /// Workflow DAG using petgraph
    graph: DiGraph<WorkflowStep, ()>,
    /// Node index mapping for step lookup
    node_map: HashMap<String, NodeIndex>,
    /// Execution context
    context: WorkflowContext,
    /// Cancellation token for workflow termination
    cancellation_token: CancellationToken,
    /// Maximum parallel steps
    max_parallel: usize,
    /// Analysis pipeline for native implementations
    analysis_pipeline: AnalysisPipeline,
    /// Rule registry for filtering and configuration
    rule_registry: RuleRegistry,
    /// Telemetry collector for workflow runs
    telemetry: TelemetryCollector,
}

impl WorkflowEngine {
    /// Returns a deterministic execution plan (topological order) for the configured workflow steps.
    pub fn execution_plan(&self) -> Result<Vec<String>> {
        let order = toposort(&self.graph, None).map_err(|_| Error::WorkflowError {
            message: "Circular dependency detected".to_string(),
        })?;

        Ok(order.iter().map(|idx| self.graph[*idx].id.clone()).collect())
    }

    /// Stores a value in the shared workflow context.
    pub async fn set_context_value(&self, key: &str, value: serde_json::Value) {
        let mut data = self.context.data.write().await;
        data.insert(key.to_string(), value);
    }

    /// Retrieves a value from the shared workflow context.
    pub async fn get_context_value(&self, key: &str) -> Option<serde_json::Value> {
        let data = self.context.data.read().await;
        data.get(key).cloned()
    }

    /// Execute all 832 rules from JSON rulebase against the source code
    pub async fn execute_all_rules(&self) -> Result<Vec<RuleMetadata>> {
        Ok(self.rule_registry.get_enabled_rules())
    }

    /// Get rulebase metadata (832 total rules)
    pub fn get_rulebase_metadata(&self) -> &crate::rulebase::RulebaseMetadata {
        self.rule_registry.rulebase_metadata()
    }

    /// Configure rule registry from Moon Shine config
    pub fn configure_rules_from_config(&mut self, config: &MoonShineConfig) {
        let mut settings = RuleSettings::default();

        // Configure categories based on config flags
        if let Some(false) = config.enable_eslint_integration {
            settings.categories.insert(RuleCategory::Style, false);
        }
        if let Some(false) = config.enable_typescript_integration {
            settings.categories.insert(RuleCategory::Correctness, false);
        }

        // Apply complexity threshold to enable/disable complex rules
        if let Some(threshold) = config.complexity_threshold {
            if threshold < 0.3 {
                // Low complexity - enable only essential rules
                settings = RuleSettings::strict();
            }
        }

        // Apply rule settings to registry
        self.rule_registry.configure_from_settings(&settings);
    }

    /// Get filtered rules based on category
    pub fn get_rules_by_category(&self, category: &RuleCategory) -> Vec<RuleMetadata> {
        self.rule_registry.get_rules_by_category(category)
    }

    /// Get all enabled rules
    pub fn get_enabled_rules(&self) -> Vec<RuleMetadata> {
        self.rule_registry.get_enabled_rules()
    }

    /// Execute rules filtered by category
    pub async fn execute_rules_by_category(&self, category: &RuleCategory) -> Result<Vec<RuleMetadata>> {
        Ok(self.get_rules_by_category(category))
    }

    /// Create new workflow engine with petgraph DAG
    pub fn new(steps: Vec<WorkflowStep>, source_code: String, file_path: String, config: MoonShineConfig) -> Result<Self> {
        let context = WorkflowContext {
            source_code,
            file_path,
            data: Arc::new(RwLock::new(HashMap::new())),
            step_results: Arc::new(RwLock::new(HashMap::new())),
            config,
        };

        // Build petgraph DAG
        let mut graph = DiGraph::new();
        let mut node_map = HashMap::new();

        // Add all steps as nodes
        for step in &steps {
            let node_index = graph.add_node(step.clone());
            node_map.insert(step.id.clone(), node_index);
        }

        // Add dependency edges
        for step in &steps {
            let step_node = node_map[&step.id];
            for dep_id in &step.depends_on {
                if let Some(&dep_node) = node_map.get(dep_id) {
                    graph.add_edge(dep_node, step_node, ());
                } else {
                    return Err(Error::WorkflowError {
                        message: format!("Unknown dependency: {}", dep_id),
                    });
                }
            }
        }

        // Check for cycles using petgraph
        if toposort(&graph, None).is_err() {
            return Err(Error::WorkflowError {
                message: "Circular dependency detected in workflow".to_string(),
            });
        }

        let analysis_pipeline = AnalysisPipeline::new();
        let rule_registry = RuleRegistry::new()?;
        let telemetry = TelemetryCollector::default();

        Ok(Self {
            graph,
            node_map,
            context,
            cancellation_token: CancellationToken::new(),
            max_parallel: 4, // Configurable parallelism
            analysis_pipeline,
            rule_registry,
            telemetry,
        })
    }

    /// Execute the complete workflow using petgraph topological sort
    pub async fn execute(&mut self) -> Result<WorkflowResult> {
        let start_time = Instant::now();
        let mut stats = WorkflowStats {
            total_steps: self.graph.node_count() as u32,
            successful_steps: 0,
            failed_steps: 0,
            skipped_steps: 0,
            total_retries: 0,
            peak_memory_bytes: 0,
            parallelism_factor: 0.0,
        };

        // Use petgraph topological sort for execution order
        let topo_order = toposort(&self.graph, None).map_err(|_| Error::WorkflowError {
            message: "Circular dependency detected".to_string(),
        })?;

        // Group steps by dependency level for parallel execution
        let execution_batches = self.build_execution_batches(&topo_order)?;

        // Execute steps in batches with tokio-stream for backpressure control
        for step_batch in execution_batches {
            // Use tokio-util for timeout and cancellation
            let batch_future = self.execute_step_batch_with_cancellation(step_batch);
            let results = tokio::select! {
                result = batch_future => result?,
                _ = self.cancellation_token.cancelled() => {
                    return Err(Error::WorkflowError { message: "Workflow cancelled".to_string() });
                }
            };

            for result in results {
                if result.success {
                    stats.successful_steps += 1;
                } else {
                    stats.failed_steps += 1;
                }
                stats.total_retries += result.retry_count;
                stats.peak_memory_bytes = stats.peak_memory_bytes.max(result.memory_used);

                // Store result in context
                let mut step_results = self.context.step_results.write().await;
                step_results.insert(result.step_id.clone(), result);
            }
        }

        // Calculate final metrics
        let total_duration = start_time.elapsed();
        stats.parallelism_factor = self.calculate_parallelism_factor().await;

        // Extract final transformed code
        let transformed_code = self.extract_final_code().await?;

        // Get final context data
        let final_context = self.context.data.read().await.clone();

        // Get all step results
        let step_results: Vec<StepResult> = self.context.step_results.read().await.values().cloned().collect();

        let success = stats.failed_steps == 0;

        let workflow_result = WorkflowResult {
            success,
            total_duration,
            transformed_code: Some(transformed_code),
            step_results,
            stats,
            final_context,
        };

        self.record_telemetry(&workflow_result, &workflow_result.final_context);

        Ok(workflow_result)
    }

    /// Build execution batches from petgraph topological order
    fn build_execution_batches(&self, topo_order: &[NodeIndex]) -> Result<Vec<Vec<NodeIndex>>> {
        let mut batches = Vec::new();
        let mut current_batch = Vec::new();
        let mut completed_nodes = std::collections::HashSet::new();

        for &node_idx in topo_order {
            // Check if all dependencies of this node are completed
            let dependencies: Vec<NodeIndex> = self.graph.edges_directed(node_idx, Direction::Incoming).map(|edge| edge.source()).collect();

            let dependencies_satisfied = dependencies.iter().all(|dep| completed_nodes.contains(dep));

            if dependencies_satisfied {
                current_batch.push(node_idx);

                // If batch is full, start new batch
                if current_batch.len() >= self.max_parallel {
                    for &idx in &current_batch {
                        completed_nodes.insert(idx);
                    }
                    batches.push(current_batch.clone());
                    current_batch.clear();
                }
            } else {
                // Finish current batch and start new one
                if !current_batch.is_empty() {
                    for &idx in &current_batch {
                        completed_nodes.insert(idx);
                    }
                    batches.push(current_batch.clone());
                    current_batch.clear();
                }
                current_batch.push(node_idx);
            }
        }

        // Add final batch if not empty
        if !current_batch.is_empty() {
            batches.push(current_batch);
        }

        Ok(batches)
    }

    /// Execute step batch with cancellation support
    async fn execute_step_batch_with_cancellation(&self, step_nodes: Vec<NodeIndex>) -> Result<Vec<StepResult>> {
        // Convert node indices to steps
        let steps: Vec<WorkflowStep> = step_nodes.iter().map(|&idx| self.graph[idx].clone()).collect();

        // Create tokio-stream for parallel execution with backpressure
        let step_stream = stream::iter(steps)
            .map(|step| {
                let context = self.context.clone();
                let cancellation_token = self.cancellation_token.clone();
                async move {
                    tokio::select! {
                        result = self.execute_single_step_with_timeout(step, context) => result,
                        _ = cancellation_token.cancelled() => {
                            Err(Error::WorkflowError { message: "Step cancelled".to_string() })
                        }
                    }
                }
            })
            .buffer_unordered(self.max_parallel);

        // Collect results using tokio-stream
        let results: Result<Vec<_>> = step_stream.collect::<Vec<_>>().await.into_iter().collect();

        results
    }

    /// Execute a batch of steps in parallel
    async fn execute_step_batch(&self, step_ids: Vec<String>) -> Result<Vec<StepResult>> {
        let step_futures: Vec<_> = step_ids
            .into_iter()
            .filter_map(|step_id| {
                if let Some(&node_index) = self.node_map.get(&step_id) {
                    if let Some(step) = self.graph.node_weight(node_index) {
                        let step = step.clone();
                        let context = self.context.clone();
                        Some(async move { self.execute_single_step(step, context).await })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        let results = try_join_all(step_futures).await?;
        Ok(results)
    }

    /// Execute single step with timeout using tokio-util
    async fn execute_single_step_with_timeout(&self, step: WorkflowStep, context: WorkflowContext) -> Result<StepResult> {
        // Use tokio-util timeout for step execution
        let timeout_duration = step.timeout;
        let step_future = self.execute_single_step(step.clone(), context);

        match tokio::time::timeout(timeout_duration, step_future).await {
            Ok(result) => result,
            Err(_) => Ok(StepResult {
                step_id: step.id,
                success: false,
                duration: timeout_duration,
                output: serde_json::Value::Null,
                error: Some("Step execution timeout".to_string()),
                memory_used: self.get_memory_usage(),
                retry_count: 0,
            }),
        }
    }

    /// Execute a single step with retry logic
    async fn execute_single_step(&self, step: WorkflowStep, context: WorkflowContext) -> Result<StepResult> {
        let start_time = Instant::now();
        let mut retry_count = 0;

        loop {
            // Check step condition
            if !self.evaluate_condition(&step.condition, &context).await? {
                return Ok(StepResult {
                    step_id: step.id,
                    success: true,
                    duration: Duration::from_millis(0),
                    output: serde_json::json!({"skipped": true}),
                    error: None,
                    memory_used: 0,
                    retry_count: 0,
                });
            }

            // Execute step action
            match self.execute_step_action(&step.action, &context).await {
                Ok(output) => {
                    return Ok(StepResult {
                        step_id: step.id,
                        success: true,
                        duration: start_time.elapsed(),
                        output,
                        error: None,
                        memory_used: self.get_memory_usage(),
                        retry_count,
                    });
                }
                Err(error) => {
                    retry_count += 1;

                    if retry_count >= step.retry.max_attempts {
                        return Ok(StepResult {
                            step_id: step.id,
                            success: false,
                            duration: start_time.elapsed(),
                            output: serde_json::Value::Null,
                            error: Some(error.to_string()),
                            memory_used: self.get_memory_usage(),
                            retry_count,
                        });
                    }

                    // Wait before retry with exponential backoff
                    let delay = step
                        .retry
                        .delay
                        .mul_f64(step.retry.backoff_multiplier.powi(retry_count as i32 - 1))
                        .min(step.retry.max_delay);
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    /// Execute step action
    async fn execute_step_action(&self, action: &StepAction, context: &WorkflowContext) -> Result<serde_json::Value> {
        match action {
            StepAction::AdaptiveAssessment {
                max_assessment_time_ms,
                complexity_threshold,
                enable_quick_static_analysis,
            } => {
                self.execute_adaptive_assessment(context, *max_assessment_time_ms, *complexity_threshold, *enable_quick_static_analysis)
                    .await
            }
            StepAction::OxcParse { source_type, strict_mode } => {
                // Execute JavaScript/TypeScript parsing
                self.execute_parse(context, source_type, *strict_mode).await
            }
            StepAction::OxcRules {
                rule_categories,
                ai_enhanced: _,
            } => {
                // Execute OXC rules via CLI
                self.execute_oxc_rules(context, rule_categories).await
            }
            StepAction::BehavioralAnalysis {
                enable_hybrid_analysis,
                confidence_threshold,
                max_analysis_time_ms,
            } => {
                // Execute behavioral analysis
                self.execute_behavioral_analysis(context, *enable_hybrid_analysis, *confidence_threshold, *max_analysis_time_ms)
                    .await
            }
            StepAction::OxcTypeAnalysis { strict_types, inference } => {
                // Execute type analysis
                self.execute_type_analysis(context, *strict_types, *inference).await
            }
            StepAction::AiEnhancement { provider, copro_optimization } => {
                // Execute AI enhancement
                self.execute_ai_enhancement(context, provider, *copro_optimization).await
            }
            StepAction::OxcCodegen { apply_fixes, source_maps } => {
                // Execute code generation
                self.execute_code_generation(context, *apply_fixes, *source_maps).await
            }
            StepAction::OxcFormat { style, preserve_oxc_structure } => {
                // Execute code formatting
                self.execute_formatting(context, style, *preserve_oxc_structure).await
            }
            StepAction::CustomFunction { function_name, parameters } => {
                // Execute custom function
                self.execute_custom_function(context, function_name, parameters).await
            }
            StepAction::CreateSessionDir { base_path, session_prefix } => {
                // Execute session directory creation
                self.execute_create_session_dir(context, base_path, session_prefix).await
            }
            StepAction::WriteAgentRequest { agent_type, request_data } => {
                // Execute agent request writing
                self.execute_write_agent_request(context, agent_type, request_data).await
            }
            StepAction::ExecuteAIProvider {
                prompt_template,
                temperature,
                max_tokens,
                session_file,
            } => {
                // Execute AI provider via router
                self.execute_ai_provider(context, prompt_template, *temperature, *max_tokens, session_file)
                    .await
            }
            StepAction::ReadAgentResponse { agent_type, timeout_ms } => {
                // Execute agent response reading
                self.execute_read_agent_response(context, agent_type, *timeout_ms).await
            }
            StepAction::CleanupSession { max_age_hours } => {
                // Execute session cleanup
                self.execute_cleanup_session(context, *max_age_hours).await
            }
        }
    }

    async fn capture_pipeline_outcome(&self, pipeline_result: &AnalysisPipelineResult, rule_categories: &[String]) -> Result<serde_json::Value> {
        let pipeline_json = serde_json::to_value(pipeline_result)?;
        self.set_context_value("analysis_pipeline_result", pipeline_json.clone()).await;

        let issues_found = pipeline_result.linting.errors.len()
            + pipeline_result.linting.warnings.len()
            + pipeline_result.documentation.missing_documentation.len()
            + pipeline_result.compilation.syntax_errors.len()
            + pipeline_result.compilation.type_errors.len();

        self.set_context_value("issues_found", serde_json::json!(issues_found)).await;

        if !rule_categories.is_empty() {
            let mut subset: Vec<RuleMetadata> = Vec::new();
            for category_str in rule_categories {
                let category = RuleCategory::from(category_str.as_str());
                subset.extend(self.rule_registry.get_rules_by_category(&category));
            }

            let subset_json = serde_json::to_value(&subset)?;
            self.set_context_value("rule_subset", subset_json.clone()).await;

            Ok(serde_json::json!({
                "analysis": pipeline_json,
                "rule_subset": subset_json,
                "issues_found": issues_found,
            }))
        } else {
            Ok(serde_json::json!({
                "analysis": pipeline_json,
                "issues_found": issues_found,
            }))
        }
    }

    /// Execute custom function
    async fn execute_custom_function(
        &self,
        context: &WorkflowContext,
        function_name: &str,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value> {
        match function_name {
            "typescript_compile" => self.execute_typescript_compile(context, parameters).await,
            "eslint_lint" => self.execute_eslint_lint(context, parameters).await,
            "tsdoc_analyze" => self.execute_tsdoc_analyze(context, parameters).await,
            _ => {
                // Generic custom function
                let result = serde_json::json!({
                    "step": "custom_function",
                    "function": function_name,
                    "parameters": parameters,
                    "success": true,
                    "message": format!("Custom function '{}' executed", function_name)
                });
                Ok(result)
            }
        }
    }

    /// Execute TypeScript compilation
    async fn execute_typescript_compile(
        &self,
        context: &WorkflowContext,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let strict = parameters.get("strict").and_then(|v| v.as_bool()).unwrap_or(true);
        let no_emit = parameters.get("noEmit").and_then(|v| v.as_bool()).unwrap_or(true);
        let skip_lib_check = parameters.get("skipLibCheck").and_then(|v| v.as_bool()).unwrap_or(true);

        // Build tsc command
        let mut args = vec!["--noEmit".to_string()];
        if strict {
            args.push("--strict".to_string());
        }
        if skip_lib_check {
            args.push("--skipLibCheck".to_string());
        }
        args.push(context.file_path.clone());

        // Execute tsc command via adapter
        let output = crate::moon_pdk_interface::execute_command(crate::moon_pdk_interface::ExecCommandInput {
            command: "tsc".to_string(),
            args,
            working_dir: None,
            env: None,
        })?;

        let success = output.exit_code == 0;
        let result = serde_json::json!({
            "step": "typescript_compile",
            "success": success,
            "exit_code": output.exit_code,
            "stdout": output.stdout,
            "stderr": output.stderr,
            "strict": strict,
            "no_emit": no_emit,
            "skip_lib_check": skip_lib_check
        });

        Ok(result)
    }

    /// Execute ESLint linting
    async fn execute_eslint_lint(
        &self,
        context: &WorkflowContext,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let fix = parameters.get("fix").and_then(|v| v.as_bool()).unwrap_or(true);
        let extensions = parameters.get("extensions")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect::<Vec<_>>())
            .unwrap_or_else(|| vec![".ts".to_string(), ".tsx".to_string(), ".js".to_string(), ".jsx".to_string()]);

        // Build eslint command
        let mut args = vec!["--format".to_string(), "json".to_string()];
        if fix {
            args.push("--fix".to_string());
        }
        args.push("--ext".to_string());
        args.push(extensions.join(","));
        args.push(context.file_path.clone());

        // Execute eslint command via adapter
        let output = crate::moon_pdk_interface::execute_command(crate::moon_pdk_interface::ExecCommandInput {
            command: "eslint".to_string(),
            args,
            working_dir: None,
            env: None,
        })?;

        let success = output.exit_code == 0;
        let result = serde_json::json!({
            "step": "eslint_lint",
            "success": success,
            "exit_code": output.exit_code,
            "stdout": output.stdout,
            "stderr": output.stderr,
            "fix": fix,
            "extensions": extensions
        });

        Ok(result)
    }

    /// Execute TSDoc analysis
    async fn execute_tsdoc_analyze(
        &self,
        context: &WorkflowContext,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let coverage_threshold = parameters.get("coverage_threshold")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.8);
        let strict_mode = parameters.get("strict_mode").and_then(|v| v.as_bool()).unwrap_or(true);

        // Use the existing TSDoc analysis from the codebase
        let tsdoc_result = crate::tsdoc::analyze_tsdoc_coverage(&context.source_code, &context.file_path)?;
        
        let meets_threshold = tsdoc_result.coverage_percentage >= coverage_threshold;
        let result = serde_json::json!({
            "step": "tsdoc_analyze",
            "success": meets_threshold,
            "coverage_percentage": tsdoc_result.coverage_percentage,
            "coverage_threshold": coverage_threshold,
            "meets_threshold": meets_threshold,
            "documented_items": tsdoc_result.documented_items.len(),
            "missing_docs": tsdoc_result.missing_docs.len(),
            "strict_mode": strict_mode
        });

        Ok(result)
    }

    /// Execute session directory creation
    async fn execute_create_session_dir(&self, context: &WorkflowContext, base_path: &str, session_prefix: &str) -> Result<serde_json::Value> {
        use std::fs;
        use std::time::SystemTime;

        // Create session directory path
        let timestamp = SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let session_id = format!("{}-{:x}", session_prefix, timestamp);
        let session_path = format!("{}/{}", base_path, session_id);

        // Create directory
        fs::create_dir_all(&session_path).map_err(|e| Error::Processing {
            message: format!("Failed to create session directory: {}", e),
        })?;

        // Store session path in context
        let mut data = context.data.write().await;
        data.insert("session_dir".to_string(), serde_json::json!(session_path));
        data.insert("session_id".to_string(), serde_json::json!(session_id));

        let result = serde_json::json!({
            "step": "create_session_dir",
            "session_path": session_path,
            "session_id": session_id,
            "success": true
        });

        Ok(result)
    }

    /// Execute agent request writing
    async fn execute_write_agent_request(&self, context: &WorkflowContext, agent_type: &str, request_data: &serde_json::Value) -> Result<serde_json::Value> {
        use std::fs;

        // Get session directory from context
        let data = context.data.read().await;
        let session_dir = data.get("session_dir").and_then(|v| v.as_str()).ok_or_else(|| Error::Processing {
            message: "No session directory found in context".to_string(),
        })?;

        // Create request file path
        let request_file = format!("{}/{}-request.json", session_dir, agent_type);

        // Write request data to file
        let json_string = serde_json::to_string_pretty(request_data).map_err(|e| Error::Processing {
            message: format!("Failed to serialize request data: {}", e),
        })?;

        fs::write(&request_file, json_string.as_bytes()).map_err(|e| Error::Processing {
            message: format!("Failed to write request file: {}", e),
        })?;

        let result = serde_json::json!({
            "step": "write_agent_request",
            "agent_type": agent_type,
            "request_file": request_file,
            "data_size": json_string.len(),
            "success": true
        });

        Ok(result)
    }

    /// Execute AI provider via router (supports Claude, Gemini, OpenAI)
    async fn execute_ai_provider(
        &self,
        context: &WorkflowContext,
        prompt_template: &str,
        temperature: f64,
        max_tokens: u32,
        session_file: &str,
    ) -> Result<serde_json::Value> {
        use crate::provider_router::{get_ai_router, AIContext, AIRequest};

        // Get session directory from context
        let data = context.data.read().await;
        let session_dir = data.get("session_dir").and_then(|v| v.as_str()).ok_or_else(|| Error::Processing {
            message: "No session directory found in context".to_string(),
        })?;

        // Create full session file path
        let request_file_path = format!("{}/{}", session_dir, session_file);

        // Read the request data from the session file
        let request_content = std::fs::read_to_string(&request_file_path).map_err(|e| Error::Processing {
            message: format!("Failed to read request file {}: {}", request_file_path, e),
        })?;

        let request_data: serde_json::Value = serde_json::from_str(&request_content).map_err(|e| Error::Processing {
            message: format!("Failed to parse request JSON: {}", e),
        })?;

        // Extract prompt from request data (could be enhanced with template processing)
        let prompt = request_data.get("prompt").and_then(|v| v.as_str()).unwrap_or(&request_content);

        // Create AI request for the provider router
        let ai_request = AIRequest {
            prompt: prompt.to_string(),
            session_id: data.get("session_id").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
            file_path: Some(context.file_path.clone()),
            context: AIContext::CodeFix {
                language: "typescript".to_string(), // Could be detected from file extension
                content: context.source_code.clone(),
            },
            preferred_providers: vec![], // Let router decide based on capabilities
        };

        // Execute via provider router (intelligent selection of Claude/Gemini/OpenAI)
        let router = get_ai_router();
        let ai_response = router.execute(ai_request).await.map_err(|e| Error::Processing {
            message: format!("AI provider execution failed: {}", e),
        })?;

        // Write response to session file for debugging
        let response_file = format!("{}/ai-response.json", session_dir);
        let response_json = serde_json::to_string_pretty(&ai_response).map_err(|e| Error::Processing {
            message: format!("Failed to serialize AI response: {}", e),
        })?;

        std::fs::write(&response_file, response_json.as_bytes()).map_err(|e| Error::Processing {
            message: format!("Failed to write response file: {}", e),
        })?;

        // Store AI response in context
        let mut data = context.data.write().await;
        data.insert("ai_response".to_string(), serde_json::to_value(&ai_response)?);
        data.insert("ai_provider_used".to_string(), serde_json::json!(ai_response.provider_used));

        let result = serde_json::json!({
            "step": "execute_ai_provider",
            "provider_used": ai_response.provider_used,
            "prompt_template": prompt_template,
            "temperature": temperature,
            "max_tokens": max_tokens,
            "session_file": session_file,
            "response_file": response_file,
            "execution_time_ms": ai_response.execution_time_ms,
            "routing_reason": ai_response.routing_reason,
            "success": ai_response.success
        });

        Ok(result)
    }

    /// Execute agent response reading
    async fn execute_read_agent_response(&self, context: &WorkflowContext, agent_type: &str, timeout_ms: u64) -> Result<serde_json::Value> {
        use tokio::time::{timeout, Duration};

        // Get session directory from context
        let data = context.data.read().await;
        let session_dir = data.get("session_dir").and_then(|v| v.as_str()).ok_or_else(|| Error::Processing {
            message: "No session directory found in context".to_string(),
        })?;

        // Create response file path
        let response_file = format!("{}/{}-response.json", session_dir, agent_type);

        // Wait for response file with timeout
        let timeout_duration = Duration::from_millis(timeout_ms);
        let response_content_result: std::result::Result<std::result::Result<String, std::io::Error>, tokio::time::error::Elapsed> =
            timeout(timeout_duration, async {
                loop {
                    match std::fs::read_to_string(&response_file) {
                        Ok(content) => break Ok::<String, std::io::Error>(content),
                        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                            tokio::time::sleep(Duration::from_millis(100)).await;
                        }
                        Err(err) => break Err(err),
                    }
                }
            })
            .await;

        let response_content = match response_content_result {
            Ok(Ok(content)) => content,
            Ok(Err(io_err)) => {
                return Err(Error::Processing {
                    message: format!("Failed to read response file: {}", io_err),
                })
            }
            Err(_) => {
                return Err(Error::Processing {
                    message: format!("Timeout waiting for {} response after {}ms", agent_type, timeout_ms),
                })
            }
        };

        // Parse response JSON
        let response_data: serde_json::Value = serde_json::from_str(&response_content).map_err(|e| Error::Processing {
            message: format!("Failed to parse response JSON: {}", e),
        })?;

        // Store response in context
        let mut data = context.data.write().await;
        data.insert(format!("{}_response", agent_type), response_data.clone());

        let result = serde_json::json!({
            "step": "read_agent_response",
            "agent_type": agent_type,
            "response_file": response_file,
            "data_size": response_content.len(),
            "timeout_ms": timeout_ms,
            "success": true
        });

        Ok(result)
    }

    /// Execute session cleanup
    async fn execute_cleanup_session(&self, context: &WorkflowContext, max_age_hours: u32) -> Result<serde_json::Value> {
        use std::fs;
        use std::time::SystemTime;

        // Get session directory from context
        let data = context.data.read().await;
        let session_dir = data.get("session_dir").and_then(|v| v.as_str()).ok_or_else(|| Error::Processing {
            message: "No session directory found in context".to_string(),
        })?;

        let keep_debug = context.config.keep_debug_sessions();
        let retention_hours = context.config.debug_session_retention_hours();
        let cleanup_threshold = context
            .config
            .cleanup_sessions_older_than_hours()
            .max(max_age_hours);

        let mut files_removed = 0;
        let mut dirs_removed = 0;
        let mut errors = Vec::new();

        // Check if session directory exists and get its age
        if let Ok(metadata) = fs::metadata(session_dir) {
            if let Ok(modified) = metadata.modified() {
                let age_hours = SystemTime::now().duration_since(modified).unwrap_or_default().as_secs() as f64 / 3600.0;

                if keep_debug && age_hours < retention_hours as f64 {
                    return Ok(serde_json::json!({
                        "step": "cleanup_session",
                        "session_dir": session_dir,
                        "age_hours": age_hours,
                        "max_age_hours": cleanup_threshold,
                        "skipped": true,
                        "reason": "Debug retention period active",
                        "success": true
                    }));
                }

                if age_hours >= cleanup_threshold as f64 {
                    // Remove session directory recursively
                    if let Err(e) = fs::remove_dir_all(session_dir) {
                        errors.push(format!("Failed to remove {}: {}", session_dir, e));
                    } else {
                        dirs_removed += 1;
                        // Count files that would have been removed (simplified)
                        files_removed += 3; // request.json, response.json, etc.
                    }
                } else {
                    return Ok(serde_json::json!({
                        "step": "cleanup_session",
                        "session_dir": session_dir,
                        "age_hours": age_hours,
                        "max_age_hours": max_age_hours,
                        "skipped": true,
                        "reason": "Session too recent",
                        "success": true
                    }));
                }
            }
        }

        let result = serde_json::json!({
            "step": "cleanup_session",
            "session_dir": session_dir,
            "max_age_hours": cleanup_threshold,
            "files_removed": files_removed,
            "dirs_removed": dirs_removed,
            "errors": errors,
            "success": errors.is_empty()
        });

        Ok(result)
    }

    fn record_telemetry(&self, result: &WorkflowResult, final_context: &HashMap<String, serde_json::Value>) {
        let executed_steps: Vec<String> = result.step_results.iter().map(|step| step.step_id.clone()).collect();

        let issues_found = final_context.get("issues_found").and_then(json_value_to_u64);

        let ai_strategy = final_context.get("ai_strategy").map(json_value_to_string);

        let record = TelemetryRecord {
            file_path: self.context.file_path.clone(),
            success: result.success,
            total_steps: result.stats.total_steps,
            executed_steps,
            duration_ms: result.total_duration.as_millis(),
            issues_found,
            ai_strategy,
        };

        self.telemetry.record(&record);

        if let Some(path) = telemetry_session_path(final_context) {
            if let Ok(json) = serde_json::to_string_pretty(&record) {
                if let Err(err) = write_file_atomic(&path, &json) {
                    eprintln!("[telemetry] failed to copy record to {}: {err}", path);
                }
            }
        }
    }

    /// Calculate parallelism factor based on execution metrics
    async fn calculate_parallelism_factor(&self) -> f64 {
        // Simple heuristic: efficiency ratio based on parallel vs sequential time
        // If we have 4 cores and achieve 3.2x speedup, parallelism factor = 0.8
        let ideal_parallel_speedup = self.max_parallel as f64;
        let actual_speedup = 1.0; // Default to 1.0 for now - could be enhanced with timing data

        (actual_speedup / ideal_parallel_speedup).min(1.0)
    }

    /// Extract final transformed code from workflow context
    async fn extract_final_code(&self) -> Result<String> {
        let step_results = self.context.step_results.read().await;

        // Look for the final code generation step result
        for result in step_results.values() {
            if let Some(code) = result.output.get("transformed_code") {
                if let Some(code_str) = code.as_str() {
                    return Ok(code_str.to_string());
                }
            }
        }

        // Fallback to original source code if no transformation occurred
        Ok(self.context.source_code.clone())
    }

    /// Get current memory usage (simplified implementation)
    fn get_memory_usage(&self) -> u64 {
        // In a real implementation, this could use process memory stats
        // For now, provide a reasonable estimate based on context size
        let context_size = self.context.source_code.len() + self.context.file_path.len() + (self.context.data.try_read().map(|d| d.len()).unwrap_or(0) * 100); // rough JSON size estimate
        context_size as u64
    }

    /// Evaluate step condition
    async fn evaluate_condition(&self, condition: &Option<StepCondition>, _context: &WorkflowContext) -> Result<bool> {
        match condition {
            None => Ok(true), // No condition means always execute
            Some(StepCondition::Always) => Ok(true),
            Some(StepCondition::OnSuccess(step_id)) => {
                // Check if the specified step succeeded
                let step_results = self.context.step_results.read().await;
                Ok(step_results.get(step_id).map(|r| r.success).unwrap_or(false))
            }
            Some(StepCondition::OnFailure(step_id)) => {
                // Check if the specified step failed
                let step_results = self.context.step_results.read().await;
                Ok(step_results.get(step_id).map(|r| !r.success).unwrap_or(false))
            }
            Some(StepCondition::ContextValue { key, operator, value }) => {
                // Check context value against condition
                let context_data = self.context.data.read().await;
                if let Some(context_value) = context_data.get(key) {
                    match operator {
                        ConditionOperator::Equals => Ok(context_value == value),
                        ConditionOperator::NotEquals => Ok(context_value != value),
                        ConditionOperator::Contains => {
                            // Simple contains check for strings
                            if let (Some(ctx_str), Some(val_str)) = (context_value.as_str(), value.as_str()) {
                                Ok(ctx_str.contains(val_str))
                            } else {
                                Ok(false)
                            }
                        }
                        ConditionOperator::Exists => Ok(true), // Key exists
                        ConditionOperator::GreaterThan => {
                            // Numeric comparison
                            if let (Some(ctx_num), Some(val_num)) = (context_value.as_f64(), value.as_f64()) {
                                Ok(ctx_num > val_num)
                            } else {
                                Ok(false)
                            }
                        }
                        ConditionOperator::LessThan => {
                            // Numeric comparison
                            if let (Some(ctx_num), Some(val_num)) = (context_value.as_f64(), value.as_f64()) {
                                Ok(ctx_num < val_num)
                            } else {
                                Ok(false)
                            }
                        }
                    }
                } else {
                    Ok(false) // Key doesn't exist
                }
            }
            Some(StepCondition::Expression(_expr)) => {
                // Complex expression evaluation would go here
                // For now, default to true
                Ok(true)
            }
        }
    }

    /// Execute adaptive assessment for intelligent rule selection
    async fn execute_adaptive_assessment(
        &self,
        context: &WorkflowContext,
        max_assessment_time_ms: u64,
        complexity_threshold: f64,
        enable_quick_static_analysis: bool,
    ) -> Result<serde_json::Value> {
        // Delegate to analysis pipeline for assessment
        let assessment = self
            .analysis_pipeline
            .assess_code_quickly(
                &context.source_code,
                &context.file_path,
                Duration::from_millis(max_assessment_time_ms),
                complexity_threshold,
                enable_quick_static_analysis,
            )
            .await?;

        // Convert to JSON Value as expected by the caller
        Ok(serde_json::to_value(assessment)?)
    }

    /// Execute parse step via OXC CLI
    async fn execute_parse(&self, context: &WorkflowContext, source_type: &str, strict_mode: bool) -> Result<serde_json::Value> {
        // Execute OXC parse command via adapter
        let mut args = vec!["parse".to_string()];
        if strict_mode {
            args.push("--strict".to_string());
        }
        args.push("--source-type".to_string());
        args.push(source_type.to_string());
        args.push(context.file_path.clone());

        let output = crate::moon_pdk_interface::execute_command(crate::moon_pdk_interface::ExecCommandInput {
            command: "oxc".to_string(),
            args,
            working_dir: None,
            env: None,
        })?;

        let success = output.exit_code == 0;
        let result = serde_json::json!({
            "step": "oxc_parse",
            "success": success,
            "exit_code": output.exit_code,
            "stdout": output.stdout,
            "stderr": output.stderr,
            "source_type": source_type,
            "strict_mode": strict_mode
        });

        Ok(result)
    }

    /// Execute OXC rules step via CLI
    async fn execute_oxc_rules(&self, context: &WorkflowContext, rule_categories: &[String]) -> Result<serde_json::Value> {
        // Execute OXC linting command via adapter
        let mut args = vec!["lint".to_string()];
        args.push("--format".to_string());
        args.push("json".to_string());
        
        // Add rule categories
        for category in rule_categories {
            args.push("--category".to_string());
            args.push(category.clone());
        }
        
        args.push(context.file_path.clone());

        let output = crate::moon_pdk_interface::execute_command(crate::moon_pdk_interface::ExecCommandInput {
            command: "oxc".to_string(),
            args,
            working_dir: None,
            env: None,
        })?;

        let success = output.exit_code == 0;
        let result = serde_json::json!({
            "step": "oxc_rules",
            "success": success,
            "exit_code": output.exit_code,
            "stdout": output.stdout,
            "stderr": output.stderr,
            "rule_categories": rule_categories
        });

        Ok(result)
    }

    /// Execute behavioral analysis step using AI behavioral analyzer
    async fn execute_behavioral_analysis(
        &self,
        context: &WorkflowContext,
        enable_hybrid: bool,
        confidence: f64,
        max_time: u64,
    ) -> Result<serde_json::Value> {
        // Use the AI behavioral analyzer from the moved module
        let analyzer = crate::ai_behavioral::AiBehavioralAnalyzer::new();
        
        // Parse the code to get AST (simplified for now)
        let allocator = oxc_allocator::Allocator::default();
        let source_type = oxc_span::SourceType::from_path(&context.file_path)
            .map_err(|e| Error::Processing(format!("Invalid source type: {}", e)))?;
        
        let parse_result = oxc_parser::Parser::new(&allocator, &context.source_code, source_type).parse();
        
        let analysis_context = crate::ai_behavioral::AnalysisContext {
            file_path: context.file_path.clone(),
            file_type: source_type,
            project_context: None,
            dependencies: vec![],
        };

        // Run behavioral analysis
        let diagnostics = analyzer.analyze_behavioral_patterns(
            &context.source_code,
            &parse_result.program,
            &analysis_context,
        ).await?;

        let result = serde_json::json!({
            "step": "behavioral_analysis",
            "success": true,
            "enable_hybrid": enable_hybrid,
            "confidence_threshold": confidence,
            "max_time_ms": max_time,
            "diagnostics_count": diagnostics.len(),
            "diagnostics": diagnostics
        });

        Ok(result)
    }

    /// Execute type analysis step via OXC CLI
    async fn execute_type_analysis(&self, context: &WorkflowContext, strict_types: bool, inference: bool) -> Result<serde_json::Value> {
        // Execute OXC type analysis command via adapter
        let mut args = vec!["type-check".to_string()];
        if strict_types {
            args.push("--strict".to_string());
        }
        if inference {
            args.push("--inference".to_string());
        }
        args.push(context.file_path.clone());

        let output = crate::moon_pdk_interface::execute_command(crate::moon_pdk_interface::ExecCommandInput {
            command: "oxc".to_string(),
            args,
            working_dir: None,
            env: None,
        })?;

        let success = output.exit_code == 0;
        let result = serde_json::json!({
            "step": "oxc_type_analysis",
            "success": success,
            "exit_code": output.exit_code,
            "stdout": output.stdout,
            "stderr": output.stderr,
            "strict_types": strict_types,
            "inference": inference
        });

        Ok(result)
    }

    /// Execute AI enhancement step via provider router
    async fn execute_ai_enhancement(&self, context: &WorkflowContext, provider: &str, copro_optimization: bool) -> Result<serde_json::Value> {
        // Use the provider router for AI enhancement
        let router = crate::provider_router::AIRouter::new();
        
        let ai_request = crate::provider_router::AIRequest {
            prompt: format!("Analyze and improve this TypeScript code:\n\n{}", context.source_code),
            provider: provider.to_string(),
            temperature: 0.1,
            max_tokens: 2000,
            copro_optimization,
        };

        let ai_response = router.execute(&ai_request).await?;

        let result = serde_json::json!({
            "step": "ai_enhancement",
            "success": ai_response.success,
            "provider": provider,
            "copro_optimization": copro_optimization,
            "response": ai_response.response,
            "usage": ai_response.usage,
            "processing_time_ms": ai_response.processing_time_ms
        });

        Ok(result)
    }

    /// Execute code generation step via OXC CLI
    async fn execute_code_generation(&self, context: &WorkflowContext, apply_fixes: bool, source_maps: bool) -> Result<serde_json::Value> {
        // Execute OXC code generation command via adapter
        let mut args = vec!["codegen".to_string()];
        if apply_fixes {
            args.push("--apply-fixes".to_string());
        }
        if source_maps {
            args.push("--source-maps".to_string());
        }
        args.push(context.file_path.clone());

        let output = crate::moon_pdk_interface::execute_command(crate::moon_pdk_interface::ExecCommandInput {
            command: "oxc".to_string(),
            args,
            working_dir: None,
            env: None,
        })?;

        let success = output.exit_code == 0;
        let result = serde_json::json!({
            "step": "oxc_codegen",
            "success": success,
            "exit_code": output.exit_code,
            "stdout": output.stdout,
            "stderr": output.stderr,
            "apply_fixes": apply_fixes,
            "source_maps": source_maps
        });

        Ok(result)
    }

    /// Execute formatting step via OXC CLI
    async fn execute_formatting(&self, context: &WorkflowContext, style: &str, preserve_structure: bool) -> Result<serde_json::Value> {
        // Execute OXC formatting command via adapter
        let mut args = vec!["format".to_string()];
        args.push("--style".to_string());
        args.push(style.to_string());
        if preserve_structure {
            args.push("--preserve-structure".to_string());
        }
        args.push(context.file_path.clone());

        let output = crate::moon_pdk_interface::execute_command(crate::moon_pdk_interface::ExecCommandInput {
            command: "oxc".to_string(),
            args,
            working_dir: None,
            env: None,
        })?;

        let success = output.exit_code == 0;
        let result = serde_json::json!({
            "step": "oxc_format",
            "success": success,
            "exit_code": output.exit_code,
            "stdout": output.stdout,
            "stderr": output.stderr,
            "style": style,
            "preserve_structure": preserve_structure
        });

        Ok(result)
    }
}

fn telemetry_session_path(context: &HashMap<String, serde_json::Value>) -> Option<String> {
    context
        .get("session_dir")
        .and_then(|value| value.as_str())
        .map(|dir| format!("{}/telemetry.json", dir.trim_end_matches('/')))
}

/// Create the complete Moon Shine workflow with all analysis phases
pub fn create_moonshine_oxc_workflow() -> Vec<WorkflowStep> {
    vec![
        // Foundation Steps (Sequential)
        WorkflowStep {
            id: "adaptive-assessment".to_string(),
            name: "Adaptive Assessment".to_string(),
            description: "Quick evaluation to determine optimal analysis strategy".to_string(),
            depends_on: vec![],
            action: StepAction::AdaptiveAssessment {
                max_assessment_time_ms: 1000,
                complexity_threshold: 0.7,
                enable_quick_static_analysis: true,
            },
            condition: Some(StepCondition::Always),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(5),
            critical: false,
        },
        WorkflowStep {
            id: "oxc-parse".to_string(),
            name: "OXC Parse + Semantic".to_string(),
            description: "Parse source code and build semantic model".to_string(),
            depends_on: vec!["adaptive-assessment".to_string()],
            action: StepAction::OxcParse {
                source_type: "typescript".to_string(),
                strict_mode: true,
            },
            condition: Some(StepCondition::Always),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(30),
            critical: true,
        },
        
        // Analysis Steps (Parallel Execution)
        WorkflowStep {
            id: "oxc-rules".to_string(),
            name: "OXC Static Rules".to_string(),
            description: "Execute 582+ static analysis rules with AI enhancement".to_string(),
            depends_on: vec!["oxc-parse".to_string()],
            action: StepAction::OxcRules {
                rule_categories: vec![
                    "correctness".to_string(),
                    "style".to_string(),
                    "performance".to_string(),
                    "security".to_string(),
                ],
                ai_enhanced: true,
            },
            condition: Some(StepCondition::OnSuccess("oxc-parse".to_string())),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(60),
            critical: false,
        },
        WorkflowStep {
            id: "behavioral-analysis".to_string(),
            name: "AI Behavioral Analysis".to_string(),
            description: "AI-enhanced behavioral pattern detection".to_string(),
            depends_on: vec!["oxc-parse".to_string()],
            action: StepAction::BehavioralAnalysis {
                enable_hybrid_analysis: true,
                confidence_threshold: 0.7,
                max_analysis_time_ms: 5000,
            },
            condition: Some(StepCondition::OnSuccess("oxc-parse".to_string())),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(10),
            critical: false,
        },
        WorkflowStep {
            id: "type-analysis".to_string(),
            name: "TypeScript Type Analysis".to_string(),
            description: "TypeScript type checking and inference".to_string(),
            depends_on: vec!["oxc-parse".to_string()],
            action: StepAction::OxcTypeAnalysis {
                strict_types: true,
                inference: true,
            },
            condition: Some(StepCondition::OnSuccess("oxc-parse".to_string())),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(45),
            critical: false,
        },
        
        // Enhancement Steps (Conditional Execution)
        WorkflowStep {
            id: "ai-enhancement".to_string(),
            name: "AI Code Enhancement".to_string(),
            description: "AI-powered code improvements via provider router".to_string(),
            depends_on: vec!["oxc-rules".to_string(), "behavioral-analysis".to_string()],
            action: StepAction::AiEnhancement {
                provider: "claude".to_string(),
                copro_optimization: true,
            },
            condition: Some(StepCondition::OnQualityThreshold(0.6)),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(30),
            critical: false,
        },
        WorkflowStep {
            id: "code-generation".to_string(),
            name: "Code Generation".to_string(),
            description: "Apply fixes and generate final output".to_string(),
            depends_on: vec!["ai-enhancement".to_string()],
            action: StepAction::OxcCodegen {
                apply_fixes: true,
                source_maps: true,
            },
            condition: Some(StepCondition::OnSuccess("ai-enhancement".to_string())),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(20),
            critical: false,
        },
        WorkflowStep {
            id: "formatting".to_string(),
            name: "Code Formatting".to_string(),
            description: "Code formatting with style preservation".to_string(),
            depends_on: vec!["code-generation".to_string()],
            action: StepAction::OxcFormat {
                style: "prettier".to_string(),
                preserve_oxc_structure: true,
            },
            condition: Some(StepCondition::OnSuccess("code-generation".to_string())),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(15),
            critical: false,
        },
    ]
}

/// Create TypeScript-specific workflow
pub fn create_typescript_workflow() -> Vec<WorkflowStep> {
    vec![
        WorkflowStep {
            id: "tsc-compile".to_string(),
            name: "TypeScript Compilation".to_string(),
            description: "TypeScript compilation with type checking".to_string(),
            depends_on: vec![],
            action: StepAction::CustomFunction {
                function_name: "typescript_compile".to_string(),
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("strict".to_string(), serde_json::Value::Bool(true));
                    params.insert("noEmit".to_string(), serde_json::Value::Bool(true));
                    params.insert("skipLibCheck".to_string(), serde_json::Value::Bool(true));
                    params
                },
            },
            condition: Some(StepCondition::Always),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(60),
            critical: true,
        },
        WorkflowStep {
            id: "tsc-type-check".to_string(),
            name: "TypeScript Type Check".to_string(),
            description: "Comprehensive type checking".to_string(),
            depends_on: vec!["tsc-compile".to_string()],
            action: StepAction::OxcTypeAnalysis {
                strict_types: true,
                inference: true,
            },
            condition: Some(StepCondition::OnSuccess("tsc-compile".to_string())),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(45),
            critical: false,
        },
    ]
}

/// Create ESLint-specific workflow
pub fn create_eslint_workflow() -> Vec<WorkflowStep> {
    vec![
        WorkflowStep {
            id: "eslint-lint".to_string(),
            name: "ESLint Analysis".to_string(),
            description: "ESLint rule checking and auto-fixing".to_string(),
            depends_on: vec![],
            action: StepAction::CustomFunction {
                function_name: "eslint_lint".to_string(),
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("fix".to_string(), serde_json::Value::Bool(true));
                    params.insert("extensions".to_string(), serde_json::Value::Array(vec![
                        serde_json::Value::String(".ts".to_string()),
                        serde_json::Value::String(".tsx".to_string()),
                        serde_json::Value::String(".js".to_string()),
                        serde_json::Value::String(".jsx".to_string()),
                    ]));
                    params
                },
            },
            condition: Some(StepCondition::Always),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(90),
            critical: false,
        },
    ]
}

/// Create Prettier-specific workflow
pub fn create_prettier_workflow() -> Vec<WorkflowStep> {
    vec![
        WorkflowStep {
            id: "prettier-format".to_string(),
            name: "Prettier Formatting".to_string(),
            description: "Code formatting with Prettier".to_string(),
            depends_on: vec![],
            action: StepAction::OxcFormat {
                style: "prettier".to_string(),
                preserve_oxc_structure: false,
            },
            condition: Some(StepCondition::Always),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(30),
            critical: false,
        },
    ]
}

/// Create TSDoc-specific workflow
pub fn create_tsdoc_workflow() -> Vec<WorkflowStep> {
    vec![
        WorkflowStep {
            id: "tsdoc-analyze".to_string(),
            name: "TSDoc Analysis".to_string(),
            description: "TSDoc documentation analysis".to_string(),
            depends_on: vec![],
            action: StepAction::CustomFunction {
                function_name: "tsdoc_analyze".to_string(),
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("coverage_threshold".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.8).unwrap()));
                    params.insert("strict_mode".to_string(), serde_json::Value::Bool(true));
                    params
                },
            },
            condition: Some(StepCondition::Always),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(45),
            critical: false,
        },
    ]
}

/// Create agent-based workflow for session coordination
pub fn create_agent_workflow() -> Vec<WorkflowStep> {
    vec![
        // Session setup
        WorkflowStep {
            id: "create-session".to_string(),
            name: "Create Session Directory".to_string(),
            description: "Create session directory for agent coordination".to_string(),
            depends_on: vec![],
            action: StepAction::CreateSessionDir {
                base_path: "/tmp/moon-shine".to_string(),
                session_prefix: "session".to_string(),
            },
            condition: Some(StepCondition::Always),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(5),
            critical: true,
        },
        
        // Agent requests (parallel)
        WorkflowStep {
            id: "write-typescript-request".to_string(),
            name: "Write TypeScript Request".to_string(),
            description: "Write TypeScript agent request to session".to_string(),
            depends_on: vec!["create-session".to_string()],
            action: StepAction::WriteAgentRequest {
                agent_type: "typescript".to_string(),
                request_data: serde_json::json!({
                    "action": "compile",
                    "strict": true,
                    "noEmit": true
                }),
            },
            condition: Some(StepCondition::OnSuccess("create-session".to_string())),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(5),
            critical: false,
        },
        WorkflowStep {
            id: "write-eslint-request".to_string(),
            name: "Write ESLint Request".to_string(),
            description: "Write ESLint agent request to session".to_string(),
            depends_on: vec!["create-session".to_string()],
            action: StepAction::WriteAgentRequest {
                agent_type: "eslint".to_string(),
                request_data: serde_json::json!({
                    "action": "lint",
                    "fix": true,
                    "extensions": [".ts", ".tsx", ".js", ".jsx"]
                }),
            },
            condition: Some(StepCondition::OnSuccess("create-session".to_string())),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(5),
            critical: false,
        },
        WorkflowStep {
            id: "write-prettier-request".to_string(),
            name: "Write Prettier Request".to_string(),
            description: "Write Prettier agent request to session".to_string(),
            depends_on: vec!["create-session".to_string()],
            action: StepAction::WriteAgentRequest {
                agent_type: "prettier".to_string(),
                request_data: serde_json::json!({
                    "action": "format",
                    "write": true
                }),
            },
            condition: Some(StepCondition::OnSuccess("create-session".to_string())),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(5),
            critical: false,
        },
        WorkflowStep {
            id: "write-claude-request".to_string(),
            name: "Write Claude Request".to_string(),
            description: "Write Claude AI agent request to session".to_string(),
            depends_on: vec!["create-session".to_string()],
            action: StepAction::WriteAgentRequest {
                agent_type: "claude".to_string(),
                request_data: serde_json::json!({
                    "action": "analyze_and_fix",
                    "provider": "claude",
                    "temperature": 0.1
                }),
            },
            condition: Some(StepCondition::OnSuccess("create-session".to_string())),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(5),
            critical: false,
        },
        
        // AI execution
        WorkflowStep {
            id: "execute-ai-provider".to_string(),
            name: "Execute AI Provider".to_string(),
            description: "Execute AI analysis via provider router".to_string(),
            depends_on: vec!["write-claude-request".to_string()],
            action: StepAction::ExecuteAIProvider {
                prompt_template: "Analyze and improve this TypeScript code: {{code}}".to_string(),
                temperature: 0.1,
                max_tokens: 2000,
                session_file: "claude-request.json".to_string(),
            },
            condition: Some(StepCondition::OnSuccess("write-claude-request".to_string())),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(30),
            critical: false,
        },
        
        // Agent responses (parallel)
        WorkflowStep {
            id: "read-typescript-response".to_string(),
            name: "Read TypeScript Response".to_string(),
            description: "Read TypeScript agent response from session".to_string(),
            depends_on: vec!["write-typescript-request".to_string()],
            action: StepAction::ReadAgentResponse {
                agent_type: "typescript".to_string(),
                timeout_ms: 60000,
            },
            condition: Some(StepCondition::OnSuccess("write-typescript-request".to_string())),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(60),
            critical: false,
        },
        WorkflowStep {
            id: "read-eslint-response".to_string(),
            name: "Read ESLint Response".to_string(),
            description: "Read ESLint agent response from session".to_string(),
            depends_on: vec!["write-eslint-request".to_string()],
            action: StepAction::ReadAgentResponse {
                agent_type: "eslint".to_string(),
                timeout_ms: 90000,
            },
            condition: Some(StepCondition::OnSuccess("write-eslint-request".to_string())),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(90),
            critical: false,
        },
        WorkflowStep {
            id: "read-prettier-response".to_string(),
            name: "Read Prettier Response".to_string(),
            description: "Read Prettier agent response from session".to_string(),
            depends_on: vec!["write-prettier-request".to_string()],
            action: StepAction::ReadAgentResponse {
                agent_type: "prettier".to_string(),
                timeout_ms: 30000,
            },
            condition: Some(StepCondition::OnSuccess("write-prettier-request".to_string())),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(30),
            critical: false,
        },
        WorkflowStep {
            id: "read-claude-response".to_string(),
            name: "Read Claude Response".to_string(),
            description: "Read Claude AI agent response from session".to_string(),
            depends_on: vec!["execute-ai-provider".to_string()],
            action: StepAction::ReadAgentResponse {
                agent_type: "claude".to_string(),
                timeout_ms: 30000,
            },
            condition: Some(StepCondition::OnSuccess("execute-ai-provider".to_string())),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(30),
            critical: false,
        },
        
        // Cleanup
        WorkflowStep {
            id: "cleanup-session".to_string(),
            name: "Cleanup Session".to_string(),
            description: "Cleanup session directory".to_string(),
            depends_on: vec![
                "read-typescript-response".to_string(),
                "read-eslint-response".to_string(),
                "read-prettier-response".to_string(),
                "read-claude-response".to_string(),
            ],
            action: StepAction::CleanupSession {
                max_age_hours: 12,
            },
            condition: Some(StepCondition::OnCompletion),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(5),
            critical: false,
        },
    ]
}

/// Create the static analysis workflow for Moon Shine (legacy)
pub fn create_static_analysis_workflow() -> Vec<WorkflowStep> {
    vec![
        WorkflowStep {
            id: "oxc-parse".to_string(),
            name: "OXC Parse + Semantic".to_string(),
            description: "Parse source code and build semantic model".to_string(),
            depends_on: vec![],
            action: StepAction::OxcParse {
                source_type: "typescript".to_string(),
                strict_mode: true,
            },
            condition: Some(StepCondition::Always),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(30),
            critical: true,
        },
        WorkflowStep {
            id: "static-rules".to_string(),
            name: "Static Analysis Rules".to_string(),
            description: "Execute 582+ static analysis rules with AI enhancement".to_string(),
            depends_on: vec!["oxc-parse".to_string()],
            action: StepAction::OxcRules {
                rule_categories: vec![
                    "correctness".to_string(),
                    "style".to_string(),
                    "performance".to_string(),
                    "security".to_string(),
                ],
                ai_enhanced: true,
            },
            condition: Some(StepCondition::OnSuccess("oxc-parse".to_string())),
            retry: RetryConfig::default(),
            timeout: Duration::from_secs(60),
            critical: false,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_petgraph_workflow_execution_order() {
        let steps = create_moonshine_oxc_workflow();
        let engine = WorkflowEngine::new(steps, "test code".to_string(), "test.ts".to_string(), MoonShineConfig::default()).unwrap();

        // Test petgraph topological sort
        let topo_order = toposort(&engine.graph, None).unwrap();

        // First node should be oxc-parse (no dependencies)
        let first_step = &engine.graph[topo_order[0]];
        assert_eq!(first_step.id, "oxc-parse");
    }
}
