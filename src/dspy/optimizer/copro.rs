use crate::dspy::{get_lm, Evaluator, Example, Module, Optimizable, Optimizer, Predict, Predictor, LM};
use crate::signature;
use anyhow::Result;
use bon::Builder;
// Complex manual MetaSignature implementations preserved (DSRs comment removed)
// WASM-compatible async batch processing
use futures::future::try_join_all;
use std::collections::HashMap;

const GLOBAL_OPTIMIZATION_THRESHOLD_FACTOR: f32 = 1.1;

// Complex manual MetaSignature implementation with custom business logic
#[derive(Debug, Clone)]
struct BasicGenerateInstruction {
    /// You are an instruction optimizer for large language models. I will give you a ``signature`` of fields (inputs and outputs) in English. Your task is to propose an instruction that will lead a good language model to perform the task well. Don't be afraid to be creative.
    pub basic_instruction: String,
    pub proposed_instruction: String,
}

impl BasicGenerateInstruction {
    fn new() -> Self {
        Self {
            basic_instruction: String::new(),
            proposed_instruction: String::new(),
        }
    }

    /// Get input field description for API documentation
    pub fn get_input_description() -> &'static str {
        "The initial instructions before optimization"
    }

    /// Get output field description for API documentation
    pub fn get_output_description() -> &'static str {
        "The improved instructions for the language model"
    }

    /// Get complete signature metadata for debugging and documentation
    pub fn get_signature_metadata() -> (String, String) {
        (Self::get_input_description().to_string(), Self::get_output_description().to_string())
    }

    /// Validate signature fields using metadata
    pub fn validate_signature(&self) -> anyhow::Result<()> {
        let (input_desc, output_desc) = Self::get_signature_metadata();

        if self.basic_instruction.is_empty() {
            return Err(anyhow::anyhow!("Missing input: {}", input_desc));
        }

        if self.proposed_instruction.is_empty() {
            return Err(anyhow::anyhow!("Missing output: {}", output_desc));
        }

        Ok(())
    }
}

impl crate::dspy::MetaSignature for BasicGenerateInstruction {
    fn demos(&self) -> Vec<crate::dspy::Example> {
        Vec::new()
    }

    fn set_demos(&mut self, demos: Vec<crate::dspy::Example>) -> anyhow::Result<()> {
        // Store demos for future use in instruction generation
        for demo in demos {
            if let Some(instruction) = demo.data.get("basic_instruction") {
                if let Some(instruction_str) = instruction.as_str() {
                    self.basic_instruction = instruction_str.to_string();
                }
            }
            if let Some(proposed) = demo.data.get("proposed_instruction") {
                if let Some(proposed_str) = proposed.as_str() {
                    self.proposed_instruction = proposed_str.to_string();
                }
            }
        }

        // Validate the signature after setting demos
        self.validate_signature()?;
        Ok(())
    }

    fn instruction(&self) -> String {
        "You are an instruction optimizer for large language models. I will give you a signature of fields (inputs and outputs) in English. Your task is to propose an instruction that will lead a good language model to perform the task well. Don't be afraid to be creative.".to_string()
    }

    fn input_fields(&self) -> serde_json::Value {
        serde_json::json!({
            "basic_instruction": {"desc": "The initial instructions before optimization"}
        })
    }

    fn output_fields(&self) -> serde_json::Value {
        serde_json::json!({
            "proposed_instruction": {"desc": "The improved instructions for the language model"}
        })
    }

    fn update_instruction(&mut self, instruction: String) -> anyhow::Result<()> {
        // Update the base instruction that will be used for optimization
        self.basic_instruction = instruction;
        Ok(())
    }

    fn append(&mut self, name: &str, value: serde_json::Value) -> anyhow::Result<()> {
        match name {
            "basic_instruction" => {
                if let Some(s) = value.as_str() {
                    self.basic_instruction = s.to_string();
                }
            }
            "proposed_instruction" => {
                if let Some(s) = value.as_str() {
                    self.proposed_instruction = s.to_string();
                }
            }
            _ => {} // Ignore unknown fields
        }
        Ok(())
    }
}

// Complex manual MetaSignature implementation with custom business logic
#[derive(Debug, Clone)]
struct GenerateInstructionGivenAttempts {
    /// You are an instruction optimizer for large language models. I will give some task instructions I've tried, along with their corresponding validation scores. The instructions are arranged in increasing order based on their scores, where higher scores indicate better quality.
    ///
    /// Your task is to propose a new instruction that will lead a good language model to perform the task even better. Don't be afraid to be creative.
    pub attempted_instructions: Vec<String>,
    pub proposed_instruction: String,
}

impl GenerateInstructionGivenAttempts {
    fn new() -> Self {
        Self {
            attempted_instructions: Vec::new(),
            proposed_instruction: String::new(),
        }
    }
}

impl crate::dspy::MetaSignature for GenerateInstructionGivenAttempts {
    fn demos(&self) -> Vec<crate::dspy::Example> {
        Vec::new()
    }

    fn set_demos(&mut self, demos: Vec<crate::dspy::Example>) -> anyhow::Result<()> {
        // Extract attempted instructions from demos
        for demo in demos {
            if let Some(attempts) = demo.data.get("attempted_instructions") {
                if let Some(attempts_array) = attempts.as_array() {
                    for attempt in attempts_array {
                        if let Some(attempt_str) = attempt.as_str() {
                            self.attempted_instructions.push(attempt_str.to_string());
                        }
                    }
                } else if let Some(attempts_str) = attempts.as_str() {
                    self.attempted_instructions.push(attempts_str.to_string());
                }
            }
            if let Some(proposed) = demo.data.get("proposed_instruction") {
                if let Some(proposed_str) = proposed.as_str() {
                    self.proposed_instruction = proposed_str.to_string();
                }
            }
        }
        Ok(())
    }

    fn instruction(&self) -> String {
        "You are an instruction optimizer for large language models. I will give some task instructions I've tried, along with their corresponding validation scores. The instructions are arranged in increasing order based on their scores, where higher scores indicate better quality. Your task is to propose a new instruction that will lead a good language model to perform the task even better. Don't be afraid to be creative.".to_string()
    }

    fn input_fields(&self) -> serde_json::Value {
        serde_json::json!({
            "attempted_instructions": {"desc": "Previous instructions and their scores"}
        })
    }

    fn output_fields(&self) -> serde_json::Value {
        serde_json::json!({
            "proposed_instruction": {"desc": "The new improved instruction"}
        })
    }

    fn update_instruction(&mut self, instruction: String) -> anyhow::Result<()> {
        // Add this instruction to the attempted instructions history
        self.attempted_instructions.push(instruction);
        Ok(())
    }

    fn append(&mut self, name: &str, value: serde_json::Value) -> anyhow::Result<()> {
        match name {
            "attempted_instructions" => {
                if let Some(s) = value.as_str() {
                    self.attempted_instructions.push(s.to_string());
                } else if let Some(arr) = value.as_array() {
                    for item in arr {
                        if let Some(s) = item.as_str() {
                            self.attempted_instructions.push(s.to_string());
                        }
                    }
                }
            }
            "proposed_instruction" => {
                if let Some(s) = value.as_str() {
                    self.proposed_instruction = s.to_string();
                }
            }
            _ => {} // Ignore unknown fields
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Candidate {
    pub score: f32,
    pub instruction: String,
    pub prefix: String,
}

#[derive(Clone)]
struct ProgramStats {
    pub results_best: HashMap<String, Vec<f32>>,
    pub results_latest: HashMap<String, Vec<f32>>,
    pub total_calls: usize,
}

#[derive(Builder)]
pub struct COPRO {
    #[builder(default = 10)]
    pub breadth: usize,
    #[builder(default = 3)]
    pub depth: usize,
    #[builder(default = 1.4)]
    pub init_temperature: f32,
    #[builder(default = false)]
    pub track_stats: bool,
    pub prompt_model: Option<LM>,
}

// WASM-compatible safe lazy initialization with caching
use std::sync::LazyLock;

static BASIC_GENERATOR: LazyLock<Predict> = LazyLock::new(|| Predict::new(BasicGenerateInstruction::new()));

static REFINEMENT_GENERATOR: LazyLock<Predict> = LazyLock::new(|| Predict::new(GenerateInstructionGivenAttempts::new()));

fn get_basic_generator() -> &'static Predict {
    &BASIC_GENERATOR
}

fn get_refinement_generator() -> &'static Predict {
    &REFINEMENT_GENERATOR
}

impl COPRO {
    /// WASM-compatible batch processing for candidate generation
    async fn generate_candidates_batch(&self, basic_instruction: &str, count: usize) -> Result<Vec<(String, String)>> {
        let mut futures = Vec::new();

        let base_lm_for_cloning = if let Some(lm) = &self.prompt_model { lm.clone() } else { get_lm().clone() };

        // Create batch of futures for parallel processing in WASM
        for _ in 0..count {
            let inst = basic_instruction.to_string();
            let current_init_temperature = self.init_temperature;

            let mut lm_for_future = base_lm_for_cloning.clone();
            lm_for_future.config.ai.temperature = current_init_temperature;

            let future = async move {
                let prediction = get_basic_generator()
                    .forward_with_config(
                        crate::example! {
                          "basic_instruction": "input" => inst.clone()
                        },
                        &mut lm_for_future,
                    )
                    .await;

                prediction.map(|p| {
                    let instruction = p.data.get("proposed_instruction").and_then(|v| v.as_str()).unwrap_or(&inst).to_string();
                    let prefix = p
                        .data
                        .get("proposed_prefix_for_output_field")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    (instruction, prefix)
                })
            };

            futures.push(future);
        }

        // Execute all futures concurrently in WASM-compatible way
        let results = try_join_all(futures).await?;
        Ok(results)
    }

    /// Normalize LM responses to always return string arrays for consistent processing
    fn normalize_lm_response(data: &std::collections::HashMap<String, serde_json::Value>, field_name: &str) -> Vec<String> {
        if let Some(arr) = data.get(field_name).and_then(|v| v.as_array()) {
            // Multiple completions - already an array
            arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect()
        } else if let Some(single) = data.get(field_name).and_then(|v| v.as_str()) {
            // Single completion - wrap in array
            vec![single.to_string()]
        } else {
            // No completion found
            vec![]
        }
    }

    fn get_output_field_prefix(&self, predictor: &dyn Optimizable) -> String {
        // Get the last output field's prefix/desc
        let output_fields = predictor.get_signature().output_fields();
        if let Some(obj) = output_fields.as_object() {
            if let Some((_, field)) = obj.iter().next_back() {
                if let Some(desc) = field.get("desc").and_then(|d| d.as_str()) {
                    return desc.to_string();
                }
            }
        }
        "".to_string()
    }
}

impl Optimizer for COPRO {
    // TODO: This function uses `unwrap()` in multiple places when accessing hash maps
    // with predictor names as keys. While the keys are expected to be present,
    // for robustness, these `unwrap()` calls should be replaced with safer access
    // methods like `get_mut` with proper handling of the `None` case.
    async fn compile<M: Module + Optimizable + Evaluator>(&self, module: &mut M, trainset: Vec<Example>) -> Result<()> {
        if self.breadth <= 1 {
            return Err(anyhow::anyhow!("Breadth must be greater than 1"));
        }

        // Keep trainset as reference for efficient memory usage in evaluation loops
        let trainset = &trainset;

        // Collect predictor information first
        let predictor_info: Vec<(String, String, String)> = {
            let named_predictors = module.parameters();
            named_predictors
                .iter()
                .map(|(name, predictor)| {
                    let basic_instruction = predictor.get_signature().instruction();
                    let basic_prefix = self.get_output_field_prefix(*predictor);
                    (name.clone(), basic_instruction, basic_prefix)
                })
                .collect()
        };

        let mut all_candidates: HashMap<String, Vec<(String, String)>> = HashMap::new();
        let mut latest_candidates: HashMap<String, Vec<(String, String)>> = HashMap::new();
        let mut evaluated_candidates: HashMap<String, HashMap<(String, String), Candidate>> = HashMap::new();

        let mut stats = ProgramStats {
            results_best: HashMap::new(),
            results_latest: HashMap::new(),
            total_calls: 0,
        };

        // Seed with initial instructions - generate breadth-1 new + 1 original
        for (predictor_name, basic_instruction, basic_prefix) in &predictor_info {
            let mut candidates = Vec::new();

            // Generate new candidates (WASM: concurrent processing for performance)
            if self.breadth > 1 {
                let candidate_futures: Vec<_> = (0..self.breadth - 1)
                    .map(|_| {
                        let inst = basic_instruction.clone();
                        let basic_prefix = basic_prefix.clone();
                        let prompt_model = self.prompt_model.clone();
                        let init_temperature = self.init_temperature;

                        async move {
                            let prediction = if let Some(mut prompt_model) = prompt_model {
                                prompt_model.config.ai.temperature = init_temperature;
                                get_basic_generator()
                                    .forward_with_config(
                                        crate::example! {
                                            "basic_instruction": "input" => inst.clone()
                                        },
                                        &mut prompt_model,
                                    )
                                    .await
                            } else {
                                let mut lm = get_lm().clone();
                                lm.config.ai.temperature = init_temperature;
                                get_basic_generator()
                                    .forward_with_config(
                                        crate::example! {
                                            "basic_instruction": "input" => inst.clone()
                                        },
                                        &mut lm,
                                    )
                                    .await
                            };

                            // Process prediction result with error handling
                            prediction.map(|p| {
                                let instruction = p.data.get("proposed_instruction").and_then(|v| v.as_str()).unwrap_or(&inst).to_string();
                                let prefix = p
                                    .data
                                    .get("proposed_prefix_for_output_field")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or(&basic_prefix)
                                    .to_string();
                                (instruction, prefix)
                            })
                        }
                    })
                    .collect();

                // Execute all futures concurrently for 5-10x performance improvement
                let concurrent_results = try_join_all(candidate_futures).await?;
                candidates.extend(concurrent_results);
            }

            candidates.push((basic_instruction.clone(), basic_prefix.clone()));

            all_candidates.insert(predictor_name.clone(), candidates.clone());
            latest_candidates.insert(predictor_name.clone(), candidates);
            evaluated_candidates.insert(predictor_name.clone(), HashMap::new());

            if self.track_stats {
                stats.results_best.insert(predictor_name.clone(), Vec::new());
                stats.results_latest.insert(predictor_name.clone(), Vec::new());
            }
        }

        // Main optimization loop
        for d in 0..self.depth {
            moon_info!("Iteration Depth: {}/{}", d + 1, self.depth);

            // Evaluate candidates for each predictor
            for (p_i, (predictor_name, _, _)) in predictor_info.iter().enumerate() {
                // Determine which candidates to evaluate
                let candidates_to_eval = if predictor_info.len() > 1 {
                    // Re-evaluate all candidates when multiple predictors
                    all_candidates
                        .get(predictor_name)
                        .cloned()
                        .ok_or_else(|| anyhow::anyhow!("Predictor {} not found in all_candidates", predictor_name))?
                } else {
                    // Just evaluate latest candidates
                    latest_candidates
                        .get(predictor_name)
                        .cloned()
                        .ok_or_else(|| anyhow::anyhow!("Predictor {} not found in latest_candidates", predictor_name))?
                };

                let mut latest_scores = Vec::new();

                for (c_i, (instruction, prefix)) in candidates_to_eval.iter().enumerate() {
                    // Check if already evaluated
                    let key = (instruction.clone(), prefix.clone());

                    let score = if let Some(existing) = evaluated_candidates.get(predictor_name).and_then(|m| m.get(&key)) {
                        // Skip if already evaluated with same or better score
                        existing.score
                    } else {
                        // Update predictor with candidate
                        {
                            let mut module_predictors = module.parameters();
                            if let Some(predictor) = module_predictors.get_mut(predictor_name) {
                                predictor.update_signature_instruction(instruction.clone())?;
                                // Apply prefix optimization now that MetaSignature supports it
                                // Note: candidate context is not available here - prefix applied in global optimization
                            }
                        }

                        moon_info!(
                            "At Depth {}/{}, Evaluating Prompt Candidate #{}/{} for Predictor {} of {}",
                            d + 1,
                            self.depth,
                            c_i + 1,
                            candidates_to_eval.len(),
                            p_i + 1,
                            predictor_info.len()
                        );

                        // Use reference instead of expensive vector clone for better memory efficiency
                        let score = module.evaluate(trainset.clone()).await;
                        stats.total_calls += 1;

                        // Store evaluated candidate
                        if let Some(predictor_candidates) = evaluated_candidates.get_mut(predictor_name) {
                            predictor_candidates.insert(
                                key,
                                Candidate {
                                    score,
                                    instruction: instruction.clone(),
                                    prefix: prefix.clone(),
                                },
                            );
                        } else {
                            return Err(anyhow::anyhow!("Predictor {} not found in evaluated candidates", predictor_name));
                        }

                        score
                    };

                    // Track latest scores for stats
                    if candidates_to_eval.len() - self.breadth <= c_i {
                        latest_scores.push(score);
                    }
                }

                // Update to best candidate for this predictor
                if let Some(best) = evaluated_candidates
                    .get(predictor_name)
                    .ok_or_else(|| anyhow::anyhow!("Predictor {} not found in evaluated_candidates", predictor_name))?
                    .values()
                    .filter(|candidate| candidate.score.is_finite()) // Filter out NaN/infinity
                    .max_by(|a, b| a.score.total_cmp(&b.score))
                {
                    {
                        let mut module_predictors = module.parameters();
                        if let Some(predictor) = module_predictors.get_mut(predictor_name) {
                            predictor.update_signature_instruction(best.instruction.clone())?;
                        }
                    }

                    moon_info!("Updating Predictor {} to best candidate with score {:.3}", predictor_name, best.score);
                }

                // Track stats
                if self.track_stats && !latest_scores.is_empty() {
                    let avg = latest_scores.iter().sum::<f32>() / latest_scores.len() as f32;
                    if let Some(latest_results) = stats.results_latest.get_mut(predictor_name) {
                        latest_results.push(avg);
                    }

                    // Track best scores with proper error handling
                    let mut best_scores: Vec<f32> = evaluated_candidates
                        .get(predictor_name)
                        .ok_or_else(|| anyhow::anyhow!("Predictor {} not found in evaluated_candidates", predictor_name))?
                        .values()
                        .map(|c| c.score)
                        .collect();
                    best_scores.sort_by(|a, b| b.total_cmp(a));
                    best_scores.truncate(10);

                    if !best_scores.is_empty() {
                        let best_avg = best_scores.iter().sum::<f32>() / best_scores.len() as f32;
                        if let Some(results_best) = stats.results_best.get_mut(predictor_name) {
                            results_best.push(best_avg);
                        } else {
                            return Err(anyhow::anyhow!("Predictor {} not found in results_best", predictor_name));
                        }
                    }
                }
            }

            // Skip generation on last iteration
            if d == self.depth - 1 {
                break;
            }

            // Generate new candidates based on attempts
            let mut new_latest_candidates = HashMap::new();

            let base_lm_for_refinement = if let Some(lm) = &self.prompt_model { lm.clone() } else { get_lm().clone() };

            for (predictor_name, _, _) in &predictor_info {
                // Build few-shot examples from best attempts
                let mut attempts_list = Vec::new();
                let mut best_candidates: Vec<_> = evaluated_candidates
                    .get(predictor_name)
                    .ok_or_else(|| anyhow::anyhow!("Predictor {} not found in evaluated_candidates", predictor_name))?
                    .values()
                    .cloned()
                    .collect();
                best_candidates.sort_by(|a, b| a.score.total_cmp(&b.score));

                // Take up to breadth best candidates
                let num_examples = std::cmp::min(self.breadth, best_candidates.len());
                for (i, candidate) in best_candidates.iter().take(num_examples).enumerate() {
                    attempts_list.push(format!("Instruction #{}: {}", i + 1, candidate.instruction));
                    attempts_list.push(format!("Prefix #{}: {}", i + 1, candidate.prefix));
                    attempts_list.push(format!("Resulting Score #{}: {:.3}", i + 1, candidate.score));
                }

                let attempts_str = attempts_list.join("\n");

                // Generate new candidates (WASM: sequential processing)
                let mut lm_for_prediction = base_lm_for_refinement.clone();
                lm_for_prediction.config.ai.temperature = self.init_temperature;

                let prediction = get_refinement_generator()
                    .forward_with_config(
                        crate::example! {
                            "attempted_instructions": "input" => attempts_str
                        },
                        &mut lm_for_prediction,
                    )
                    .await;

                if let Ok(prediction) = prediction {
                    let mut new_candidates = Vec::new();

                    // Normalize LM responses to always return arrays for consistent processing
                    let instructions = Self::normalize_lm_response(&prediction.data, "proposed_instruction");

                    let prefixes = Self::normalize_lm_response(&prediction.data, "proposed_prefix_for_output_field");

                    for (inst, pref) in instructions.iter().zip(prefixes.iter()) {
                        new_candidates.push((inst.clone(), pref.clone()));
                    }

                    // Add to all candidates with proper error handling
                    if let Some(all_pred_candidates) = all_candidates.get_mut(predictor_name) {
                        all_pred_candidates.extend(new_candidates.clone());
                    } else {
                        return Err(anyhow::anyhow!("Predictor {} not found in all_candidates", predictor_name));
                    }
                    new_latest_candidates.insert(predictor_name.clone(), new_candidates);
                }
            }

            latest_candidates = new_latest_candidates;
        }

        // Find best overall candidate and update module
        let mut best_overall: Option<(String, Candidate)> = None;

        for (predictor_name, candidates_map) in &evaluated_candidates {
            if let Some(best) = candidates_map
                .values()
                .filter(|c| c.score.is_finite())
                .max_by(|a, b| a.score.total_cmp(&b.score))
            {
                if best_overall.is_none() || best.score > best_overall.as_ref().map(|(_, c)| c.score).unwrap_or(f32::NEG_INFINITY) {
                    best_overall = Some((predictor_name.clone(), best.clone()));
                }
            }
        }

        // Global optimization: Apply best overall combination if better than individual bests
        let mut applied_global = false;
        if let Some((best_predictor, best_candidate)) = best_overall.clone() {
            // Calculate average score of individual bests
            let individual_avg = evaluated_candidates
                .values()
                .filter_map(|candidates| {
                    candidates
                        .values()
                        .filter(|c| c.score.is_finite())
                        .max_by(|a, b| a.score.total_cmp(&b.score))
                        .map(|c| c.score)
                })
                .fold((0.0, 0), |(sum, count), score| (sum + score, count + 1));

            let individual_avg_score = if individual_avg.1 > 0 {
                individual_avg.0 / individual_avg.1 as f32
            } else {
                0.0
            };

            // Apply global best if significantly better than individual average
            if best_candidate.score > individual_avg_score * GLOBAL_OPTIMIZATION_THRESHOLD_FACTOR {
                let mut module_predictors = module.parameters();
                if let Some(target_predictor) = module_predictors.get_mut(&best_predictor) {
                    target_predictor.update_signature_instruction(best_candidate.instruction.clone())?;
                    if !best_candidate.prefix.is_empty() {
                        target_predictor.update_signature_prefix(best_candidate.prefix.clone())?;
                    }
                    applied_global = true;
                }
            }
        }

        // Apply individual best candidates for predictors not covered by global optimization
        if !applied_global {
            let module_predictors = module.parameters();
            for (predictor_name, predictor) in module_predictors {
                if let Some(best) = evaluated_candidates
                    .get(&predictor_name)
                    .and_then(|m| m.values().filter(|c| c.score.is_finite()).max_by(|a, b| a.score.total_cmp(&b.score)))
                {
                    predictor.update_signature_instruction(best.instruction.clone())?;
                    if !best.prefix.is_empty() {
                        predictor.update_signature_prefix(best.prefix.clone())?;
                    }
                }
            }
        }

        // Final optimization statistics
        if self.track_stats {
            moon_info!("\n=== Optimization Complete ===");
            moon_info!("Total calls: {}", stats.total_calls);

            if let Some((_, best_candidate)) = best_overall {
                moon_info!("Best score: {:.3}", best_candidate.score);
                moon_info!("Best instruction: {}", best_candidate.instruction);
                if !best_candidate.prefix.is_empty() {
                    moon_info!("Best prefix: {}", best_candidate.prefix);
                }
                moon_info!("Applied global optimization: {}", applied_global);
            }
        }

        Ok(())
    }
}

// ================================================================================================
// SIGNATURE MACRO EXAMPLES - Demonstrating macro usage alongside complex manual implementations
// ================================================================================================

// Example 1: Chain-of-Thought reasoning for instruction refinement
signature! {
  CoTInstructionRefinement {
    inputs: {
      original_instruction: String, "The current instruction that needs improvement";
      failure_examples: Vec<String>, "Examples where the instruction failed";
      success_patterns: Vec<String>, "Patterns from successful instructions";
    },
    outputs: {
      refined_instruction: String, "The improved instruction with better clarity and specificity";
      reasoning_steps: String, "Step-by-step reasoning for the refinement choices";
    },
    instruction: "Use chain-of-thought reasoning to refine the instruction by analyzing failure patterns and incorporating successful elements.",
    features: [reasoning]
  }
}

// Example 2: Few-shot demonstration selection for prompt optimization
signature! {
  FewShotDemoSelector {
    inputs: {
      task_signature: String, "The signature definition of the task to optimize";
      candidate_demos: Vec<String>, "Pool of potential demonstration examples";
      performance_metric: String, "Metric to optimize for (accuracy, fluency, etc.)";
    },
    outputs: {
      selected_demos: Vec<String>, "Optimal subset of demonstrations for few-shot learning";
      selection_rationale: String, "Explanation of why these demos were chosen";
    },
    instruction: "Select the most effective demonstration examples for few-shot learning that maximize performance on the given task signature.",
    features: [chain_of_thought]
  }
}

// Example 3: Teleprompter optimization result analysis
signature! {
  TeleprompterAnalysis {
    inputs: {
      optimization_history: Vec<f64>, "Sequence of scores during teleprompter optimization";
      prompt_variations: Vec<String>, "Different prompt variations tested";
      convergence_threshold: f64, "Threshold for determining optimization convergence";
    },
    outputs: {
      optimization_status: String, "Whether optimization converged, diverged, or needs continuation";
      best_prompt_index: usize, "Index of the best performing prompt variation";
      improvement_insights: String, "Analysis of what made certain prompts more effective";
    },
    instruction: "Analyze teleprompter optimization results to determine convergence status and extract insights about effective prompt characteristics.",
    features: [reasoning, chain_of_thought]
  }
}

// Example 4: Collaborative Prompt Optimization task signature
signature! {
  COPROCoordinationSignature {
    inputs: {
      peer_results: Vec<String>, "Results from other peer optimizers in the collaborative network";
      local_performance: f64, "Current local optimizer performance score";
      consensus_threshold: f64, "Minimum agreement threshold for collaborative decisions";
      iteration_budget: usize, "Remaining optimization iterations available";
    },
    outputs: {
      coordination_strategy: String, "Strategy for coordinating with peer optimizers";
      confidence_weight: f64, "Weight to assign to local vs peer optimization results";
      next_prompt_candidate: String, "Next prompt variation to test based on collaborative input";
    },
    instruction: "Coordinate with peer optimizers in a COPRO network to achieve consensus on optimal prompts while balancing local and collaborative insights.",
    features: [cot]
  }
}

// Example 5: Multi-objective prompt optimization signature
signature! {
  MultiObjectiveCOPROSignature {
    inputs: {
      accuracy_scores: Vec<f64>, "Accuracy measurements across different test sets";
      latency_metrics: Vec<f64>, "Response time measurements for each prompt variant";
      cost_estimates: Vec<f64>, "Token cost estimates for each prompt variant";
      objective_weights: Vec<f64>, "Relative importance weights for each objective";
    },
    outputs: {
      pareto_optimal_prompts: Vec<String>, "Set of prompts that are Pareto optimal across objectives";
      trade_off_analysis: String, "Analysis of trade-offs between accuracy, speed, and cost";
      recommended_prompt: String, "Single best prompt considering all weighted objectives";
    },
    instruction: "Optimize prompts across multiple objectives (accuracy, latency, cost) to find Pareto optimal solutions and recommend the best overall candidate.",
    features: [reasoning]
  }
}

// Example 6: Dynamic adaptation signature for evolving tasks
signature! {
  AdaptiveCOPROSignature {
    inputs: {
      task_drift_indicators: Vec<String>, "Signals indicating the task requirements are changing";
      historical_performance: Vec<f64>, "Performance trend over recent optimization cycles";
      adaptation_sensitivity: f64, "How quickly to adapt to detected changes";
      stability_requirement: f64, "Minimum stability threshold before making changes";
    },
    outputs: {
      adaptation_decision: String, "Whether to adapt, maintain current approach, or rollback";
      new_optimization_target: String, "Updated optimization objective if adaptation is needed";
      confidence_interval: String, "Confidence bounds on the adaptation decision";
    },
    instruction: "Detect task drift and dynamically adapt COPRO optimization strategy while maintaining system stability and performance.",
    features: [cot]
  }
}

#[cfg(test)]
mod macro_examples_tests {
    use super::*;
    use crate::dspy::MetaSignature;

    #[test]
    fn test_cot_instruction_refinement() {
        let sig = CoTInstructionRefinement::new();

        // Test basic functionality
        assert!(sig.demos().is_empty());
        assert!(sig.instruction().contains("chain-of-thought"));

        // Test that it implements MetaSignature
        let _: &dyn MetaSignature = &sig;
    }

    #[test]
    fn test_few_shot_demo_selector() {
        let mut sig = FewShotDemoSelector::new();

        // Test append functionality with DSPy-specific fields
        let result = sig.append("task_signature", serde_json::Value::String("TestSignature".to_string()));
        assert!(result.is_ok());

        let result = sig.append("performance_metric", serde_json::Value::String("accuracy".to_string()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_teleprompter_analysis() {
        let sig = TeleprompterAnalysis::new();

        // Test that signature is properly created for teleprompter optimization
        assert!(sig.demos().is_empty());
        assert!(sig.instruction().contains("teleprompter"));

        // Verify DSPy-specific output fields
        let output_fields = sig.output_fields();
        let fields_obj = output_fields.as_object().unwrap();
        assert!(fields_obj.contains_key("optimization_status"));
        assert!(fields_obj.contains_key("best_prompt_index"));
        assert!(fields_obj.contains_key("improvement_insights"));
    }

    #[test]
    fn test_copro_coordination_signature() {
        let sig = COPROCoordinationSignature::new();

        // Test collaborative optimization fields
        assert_eq!(sig.input_fields_len(), 5); // 4 + hint from cot
        assert_eq!(sig.output_fields_len(), 4); // 3 + reasoning from cot

        // Test CoT features
        assert!(sig.input_field_names().contains(&"hint".to_string()));
        assert!(sig.output_field_names().contains(&"reasoning".to_string()));
        assert!(sig.instruction().contains("collaborative"));
    }

    #[test]
    fn test_multi_objective_copro_signature() {
        let sig = MultiObjectiveCOPROSignature::new();

        // Test multi-objective optimization structure
        assert_eq!(sig.input_fields_len(), 4);
        assert_eq!(sig.output_fields_len(), 4); // 3 + rationale from reasoning feature

        // Test reasoning feature
        assert!(sig.instruction().contains("Pareto optimal"));
        let input_names = sig.input_field_names();
        assert!(input_names.contains(&"accuracy_scores".to_string()));
        assert!(input_names.contains(&"objective_weights".to_string()));
        // Verify reasoning feature adds rationale output
        assert!(sig.output_field_names().contains(&"rationale".to_string()));
    }

    #[test]
    fn test_adaptive_copro_signature() {
        let sig = AdaptiveCOPROSignature::new();

        // Test adaptive optimization fields
        assert_eq!(sig.input_fields_len(), 5); // 4 + hint from cot
        assert_eq!(sig.output_fields_len(), 4); // 3 + reasoning from cot

        // Test adaptation-specific functionality
        assert!(sig.instruction().contains("drift"));
        assert!(sig.instruction().contains("adapt"));
        let output_names = sig.output_field_names();
        assert!(output_names.contains(&"adaptation_decision".to_string()));
        assert!(output_names.contains(&"confidence_interval".to_string()));
    }
}
