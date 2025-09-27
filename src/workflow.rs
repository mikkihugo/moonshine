use crate::config::MoonShineConfig;
use crate::error::{Error, Result};
use crate::moon_pdk_interface::{execute_command, ExecCommandInput};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::io;

/// Definition of an entire workflow.
#[derive(Debug, Clone, Default)]
pub struct WorkflowDefinition {
    steps: Vec<WorkflowStep>,
}

impl WorkflowDefinition {
    /// Standard Moon Shine workflow: TypeScript → ESLint → Formatter → AI feedback.
    pub fn standard() -> Self {
        Self::chain(vec![
            WorkflowStep::typescript_check(),
            WorkflowStep::eslint_lint(),
            WorkflowStep::formatter(),
            WorkflowStep::ai_enhancement(),
        ])
    }

    /// Workflow limited to TypeScript and ESLint checks.
    pub fn lint_only() -> Self {
        Self::chain(vec![WorkflowStep::typescript_check(), WorkflowStep::eslint_lint()])
    }

    /// Workflow containing only the AI enhancement step.
    pub fn ai_only() -> Self {
        Self::chain(vec![WorkflowStep::ai_enhancement()])
    }

    /// Workflow containing just a single formatter step.
    pub fn formatter_only() -> Self {
        WorkflowDefinition {
            steps: vec![WorkflowStep::formatter()],
        }
    }

    /// Create a workflow based on a user-provided mode string.
    pub fn from_mode(mode: &str) -> Self {
        match mode {
            "static-analysis" | "lint-only" | "reporting-only" => Self::lint_only(),
            "typescript-only" => WorkflowDefinition {
                steps: vec![WorkflowStep::typescript_check()],
            },
            "eslint-only" => WorkflowDefinition {
                steps: vec![WorkflowStep::eslint_lint()],
            },
            "prettier-only" => Self::formatter_only(),
            "agent-based" | "tsdoc-only" => Self::ai_only(),
            _ => Self::standard(),
        }
    }

    fn chain(mut steps: Vec<WorkflowStep>) -> Self {
        for i in 1..steps.len() {
            let prev_id = steps[i - 1].id.clone();
            if !steps[i].depends_on.contains(&prev_id) {
                steps[i].depends_on.push(prev_id);
            }
        }
        WorkflowDefinition { steps }
    }

    fn ordered_steps(&self) -> Result<Vec<WorkflowStep>> {
        topological_sort(&self.steps)
    }
}

/// Workflow engine executing steps for a specific file.
pub struct WorkflowEngine {
    ordered_steps: Vec<WorkflowStep>,
    file_path: String,
    file_content: String,
    config: MoonShineConfig,
}

impl WorkflowEngine {
    pub fn new(definition: WorkflowDefinition, file_content: String, file_path: String, config: MoonShineConfig) -> Result<Self> {
        let ordered_steps = definition.ordered_steps()?;
        Ok(Self {
            ordered_steps,
            file_path,
            file_content,
            config,
        })
    }

    /// Execute the workflow synchronously.
    pub fn execute(&mut self) -> Result<WorkflowOutcome> {
        let mut step_results = Vec::new();
        let mut success = true;

        for step in &self.ordered_steps {
            info!("Running workflow step '{}'", step.name);
            match run_step(step, &self.file_path, &self.file_content, &self.config) {
                Ok(detail) => {
                    step_results.push(StepOutcome {
                        id: step.id.clone(),
                        name: step.name.clone(),
                        success: true,
                        detail,
                    });
                }
                Err(err) => {
                    warn!("Step '{}' failed: {}", step.name, err);
                    step_results.push(StepOutcome {
                        id: step.id.clone(),
                        name: step.name.clone(),
                        success: false,
                        detail: Some(err.to_string()),
                    });
                    if step.critical {
                        success = false;
                        break;
                    }
                }
            }
        }

        Ok(WorkflowOutcome {
            success,
            step_results,
            final_code: Some(self.file_content.clone()),
            quality_score: if success { 1.0 } else { 0.0 },
        })
    }
}

/// Outcome information for an executed workflow.
#[derive(Debug, Clone)]
pub struct WorkflowOutcome {
    pub success: bool,
    pub step_results: Vec<StepOutcome>,
    pub final_code: Option<String>,
    pub quality_score: f32,
}

/// Result of a single workflow step.
#[derive(Debug, Clone)]
pub struct StepOutcome {
    pub id: String,
    pub name: String,
    pub success: bool,
    pub detail: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WorkflowStep {
    pub id: String,
    pub name: String,
    pub description: String,
    pub depends_on: Vec<String>,
    pub action: WorkflowAction,
    pub critical: bool,
}

impl WorkflowStep {
    pub fn typescript_check() -> Self {
        WorkflowStep {
            id: "typescript-check".to_string(),
            name: "TypeScript Check".to_string(),
            description: "Runs project TypeScript diagnostics".to_string(),
            depends_on: Vec::new(),
            action: WorkflowAction::TypeScriptCheck,
            critical: true,
        }
    }

    pub fn eslint_lint() -> Self {
        WorkflowStep {
            id: "eslint".to_string(),
            name: "ESLint".to_string(),
            description: "Executes ESLint using project configuration".to_string(),
            depends_on: Vec::new(),
            action: WorkflowAction::Eslint,
            critical: true,
        }
    }

    pub fn formatter() -> Self {
        WorkflowStep {
            id: "formatter".to_string(),
            name: "Formatter".to_string(),
            description: "Runs project formatter (e.g., Prettier)".to_string(),
            depends_on: Vec::new(),
            action: WorkflowAction::Formatter,
            critical: false,
        }
    }

    pub fn ai_enhancement() -> Self {
        WorkflowStep {
            id: "ai-enhancement".to_string(),
            name: "AI Enhancement".to_string(),
            description: "Collects AI feedback and suggested fixes".to_string(),
            depends_on: Vec::new(),
            action: WorkflowAction::AiEnhancement,
            critical: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowAction {
    TypeScriptCheck,
    Eslint,
    Formatter,
    AiEnhancement,
    CustomCommand { program: String, args: Vec<String> },
}

fn run_step(step: &WorkflowStep, file_path: &str, file_content: &str, config: &MoonShineConfig) -> Result<Option<String>> {
    match &step.action {
        WorkflowAction::TypeScriptCheck => run_typescript_check(config, file_path),
        WorkflowAction::Eslint => run_eslint(config, file_path),
        WorkflowAction::Formatter => run_formatter(config, file_path),
        WorkflowAction::AiEnhancement => run_ai_feedback(config, file_path, file_content),
        WorkflowAction::CustomCommand { program, args } => run_custom_command(program, args),
    }
}

fn run_typescript_check(config: &MoonShineConfig, file_path: &str) -> Result<Option<String>> {
    let command = config.typescript_cli().unwrap_or_else(|| "pnpm".to_string());
    let mut args = if command == "pnpm" {
        vec!["exec".to_string(), "tsc".to_string(), "--noEmit".to_string()]
    } else {
        Vec::new()
    };
    if config.typescript_incremental() {
        args.push("--incremental".to_string());
    }
    if let Some(project) = config.typescript_project() {
        args.push("--project".to_string());
        args.push(project.to_string());
    }
    debug!("Executing TypeScript command: {} {:?}", command, args);
    let input = ExecCommandInput {
        command,
        args,
        env: HashMap::new(),
        working_dir: config.typescript_cwd().map(|c| c.to_string()),
    };
    match execute_command(input) {
        Ok(output) if output.exit_code == 0 => Ok(Some("TypeScript check succeeded".into())),
        Ok(output) => Err(Error::Analysis {
            operation: "TypeScript check".into(),
            file_path: Some(file_path.into()),
            source: Some(command_failure(format!("Command failed: {}", output.stderr))),
        }),
        Err(err) => Err(Error::Analysis {
            operation: "TypeScript check".into(),
            file_path: Some(file_path.into()),
            source: Some(command_failure(err.to_string())),
        }),
    }
}

fn run_eslint(config: &MoonShineConfig, file_path: &str) -> Result<Option<String>> {
    let command = config.eslint_cli().unwrap_or_else(|| "pnpm".to_string());
    let mut args = if command == "pnpm" {
        vec!["exec".to_string(), "eslint".to_string(), file_path.to_string()]
    } else {
        vec![file_path.to_string()]
    };
    if let Some(config_path) = config.eslint_config() {
        args.push("--config".to_string());
        args.push(config_path.to_string());
    }
    debug!("Executing ESLint command: {} {:?}", command, args);
    let input = ExecCommandInput {
        command,
        args,
        env: HashMap::new(),
        working_dir: config.eslint_cwd().map(|c| c.to_string()),
    };
    match execute_command(input) {
        Ok(output) if output.exit_code == 0 => Ok(Some("ESLint completed".into())),
        Ok(output) => Err(Error::Analysis {
            operation: "ESLint".into(),
            file_path: Some(file_path.into()),
            source: Some(command_failure(format!("Command failed: {}", output.stderr))),
        }),
        Err(err) => Err(Error::Analysis {
            operation: "ESLint".into(),
            file_path: Some(file_path.into()),
            source: Some(command_failure(err.to_string())),
        }),
    }
}

fn run_formatter(config: &MoonShineConfig, file_path: &str) -> Result<Option<String>> {
    let command = config.format_cli().unwrap_or_else(|| "pnpm".to_string());
    let args = if command == "pnpm" {
        vec!["exec".to_string(), "prettier".to_string(), "--write".to_string(), file_path.to_string()]
    } else {
        vec![file_path.to_string()]
    };
    debug!("Executing formatter command: {} {:?}", command, args);
    let input = ExecCommandInput {
        command,
        args,
        env: HashMap::new(),
        working_dir: config.format_cwd().map(|c| c.to_string()),
    };
    match execute_command(input) {
        Ok(output) if output.exit_code == 0 => Ok(Some("Formatter completed".into())),
        Ok(output) => Err(Error::Analysis {
            operation: "Formatter".into(),
            file_path: Some(file_path.into()),
            source: Some(command_failure(format!("Command failed: {}", output.stderr))),
        }),
        Err(err) => Err(Error::Analysis {
            operation: "Formatter".into(),
            file_path: Some(file_path.into()),
            source: Some(command_failure(err.to_string())),
        }),
    }
}

fn run_ai_feedback(_config: &MoonShineConfig, file_path: &str, _file_content: &str) -> Result<Option<String>> {
    info!("Collecting AI feedback for {}", file_path);
    Ok(Some("AI feedback collection deferred to provider router".into()))
}

fn run_custom_command(program: &str, args: &[String]) -> Result<Option<String>> {
    let input = ExecCommandInput {
        command: program.to_string(),
        args: args.to_vec(),
        env: HashMap::new(),
        working_dir: None,
    };
    match execute_command(input) {
        Ok(output) if output.exit_code == 0 => Ok(Some("Custom command completed".into())),
        Ok(output) => Err(Error::Analysis {
            operation: "Custom command".into(),
            file_path: None,
            source: Some(command_failure(format!("Command failed: {}", output.stderr))),
        }),
        Err(err) => Err(Error::Analysis {
            operation: "Custom command".into(),
            file_path: None,
            source: Some(command_failure(err.to_string())),
        }),
    }
}

fn command_failure(message: String) -> Box<dyn std::error::Error + Send + Sync> {
    Box::new(io::Error::new(io::ErrorKind::Other, message))
}

fn topological_sort(steps: &[WorkflowStep]) -> Result<Vec<WorkflowStep>> {
    let mut nodes: HashMap<String, WorkflowStep> = HashMap::new();
    let mut indegree: HashMap<String, usize> = HashMap::new();
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();

    for step in steps {
        if nodes.contains_key(&step.id) {
            return Err(Error::Config {
                message: format!("Duplicate workflow step id '{}'", step.id),
                field: Some("workflow.step.id".into()),
                value: None,
            });
        }
        indegree.insert(step.id.clone(), 0);
        adjacency.insert(step.id.clone(), Vec::new());
        nodes.insert(step.id.clone(), step.clone());
    }

    for step in steps {
        for dep in &step.depends_on {
            if !nodes.contains_key(dep) {
                return Err(Error::Config {
                    message: format!("Workflow dependency '{}' not found", dep),
                    field: Some("workflow.depends_on".into()),
                    value: Some(dep.clone()),
                });
            }
            adjacency.get_mut(dep).unwrap().push(step.id.clone());
            *indegree.get_mut(&step.id).unwrap() += 1;
        }
    }

    let mut queue: VecDeque<String> = indegree
        .iter()
        .filter_map(|(id, &deg)| if deg == 0 { Some(id.clone()) } else { None })
        .collect();
    let mut ordered = Vec::new();

    while let Some(id) = queue.pop_front() {
        if let Some(step) = nodes.get(&id) {
            ordered.push(step.clone());
        }
        if let Some(neighbors) = adjacency.get(&id) {
            for neighbor in neighbors {
                if let Some(entry) = indegree.get_mut(neighbor) {
                    *entry -= 1;
                    if *entry == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
    }

    if ordered.len() != steps.len() {
        return Err(Error::Config {
            message: "Workflow has cyclic dependencies".into(),
            field: Some("workflow".into()),
            value: None,
        });
    }

    Ok(ordered)
}

/// Helper trait to expose a few configuration hooks without exposing the full config surface.
trait WorkflowConfigExt {
    fn typescript_cli(&self) -> Option<String>;
    fn typescript_project(&self) -> Option<&str>;
    fn typescript_cwd(&self) -> Option<&str>;
    fn typescript_incremental(&self) -> bool;

    fn eslint_cli(&self) -> Option<String>;
    fn eslint_config(&self) -> Option<&str>;
    fn eslint_cwd(&self) -> Option<&str>;

    fn format_cli(&self) -> Option<String>;
    fn format_cwd(&self) -> Option<&str>;
}

impl WorkflowConfigExt for MoonShineConfig {
    fn typescript_cli(&self) -> Option<String> {
        None
    }

    fn typescript_project(&self) -> Option<&str> {
        None
    }

    fn typescript_cwd(&self) -> Option<&str> {
        None
    }

    fn typescript_incremental(&self) -> bool {
        false
    }

    fn eslint_cli(&self) -> Option<String> {
        None
    }

    fn eslint_config(&self) -> Option<&str> {
        None
    }

    fn eslint_cwd(&self) -> Option<&str> {
        None
    }

    fn format_cli(&self) -> Option<String> {
        None
    }

    fn format_cwd(&self) -> Option<&str> {
        None
    }
}
