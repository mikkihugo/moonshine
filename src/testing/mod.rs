//! # Testing Infrastructure
//!
//! Comprehensive testing utilities for moon-shine package supporting multiple
//! testing methodologies as requested by user.
//!
//! @category testing
//! @safe program
//! @complexity high
//! @since 2.0.0

pub mod assertions;
pub mod builders;
pub mod e2e;
pub mod edge_cases;
pub mod fixtures;
pub mod mocks;
pub mod stress_tests;
pub mod workflow_transitions;

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::path::Path;

use crate::config::MoonShineConfig;
use crate::error::Result;

/// Testing context for different test methodologies
#[derive(Debug, Clone)]
pub enum TestContext {
    /// London school (mockist) - isolated unit tests with mocks
    London(LondonContext),
    /// Chicago school (classicist) - integration tests with real collaborators
    Chicago(ChicagoContext),
    /// E2E tests - full workflow validation
    E2E(E2EContext),
}

impl TestContext {
    /// Create London school testing context with mocks
    pub fn london() -> Self {
        Self::London(LondonContext::new())
    }

    /// Create Chicago school testing context with real collaborators
    pub fn chicago() -> Self {
        Self::Chicago(ChicagoContext::new())
    }

    /// Create E2E testing context for full workflow validation
    pub fn e2e() -> Self {
        Self::E2E(E2EContext::new())
    }
}

/// London school testing context - isolated with mocks
#[derive(Debug, Clone)]
pub struct LondonContext {
    pub mock_registry: HashMap<String, String>,
    pub isolation_level: IsolationLevel,
    pub created_at: DateTime<Utc>,
}

impl LondonContext {
    pub fn new() -> Self {
        Self {
            mock_registry: HashMap::new(),
            isolation_level: IsolationLevel::Complete,
            created_at: Utc::now(),
        }
    }
}

/// Chicago school testing context - real collaborators
#[derive(Debug, Clone)]
pub struct ChicagoContext {
    pub real_dependencies: Vec<String>,
    pub integration_scope: IntegrationScope,
    pub created_at: DateTime<Utc>,
}

impl ChicagoContext {
    pub fn new() -> Self {
        Self {
            real_dependencies: vec!["config".to_string(), "workflow".to_string(), "analysis".to_string(), "linter".to_string()],
            integration_scope: IntegrationScope::ModuleLevel,
            created_at: Utc::now(),
        }
    }
}

/// E2E testing context - full workflow validation
#[derive(Debug, Clone)]
pub struct E2EContext {
    pub workflow_scenarios: Vec<String>,
    pub performance_requirements: PerformanceRequirements,
    pub created_at: DateTime<Utc>,
}

impl E2EContext {
    pub fn new() -> Self {
        Self {
            workflow_scenarios: vec![
                "full_analysis_pipeline".to_string(),
                "configuration_management".to_string(),
                "error_recovery".to_string(),
                "performance_optimization".to_string(),
            ],
            performance_requirements: PerformanceRequirements::default(),
            created_at: Utc::now(),
        }
    }
}

/// Test isolation levels for London school testing
#[derive(Debug, Clone)]
pub enum IsolationLevel {
    /// Complete isolation with all dependencies mocked
    Complete,
    /// Partial isolation with some real dependencies
    Partial,
    /// Minimal isolation for complex interactions
    Minimal,
}

/// Integration scope for Chicago school testing
#[derive(Debug, Clone)]
pub enum IntegrationScope {
    /// Test within single module
    ModuleLevel,
    /// Test across multiple modules
    CrossModule,
    /// Test full system integration
    SystemLevel,
}

/// Performance requirements for testing
#[derive(Debug, Clone)]
pub struct PerformanceRequirements {
    pub max_execution_time_ms: u64,
    pub max_memory_usage_mb: u64,
    pub min_throughput_ops_sec: u64,
}

impl Default for PerformanceRequirements {
    fn default() -> Self {
        Self {
            max_execution_time_ms: 5000, // 5 seconds max
            max_memory_usage_mb: 100,    // 100 MB max
            min_throughput_ops_sec: 10,  // 10 operations/sec min
        }
    }
}

/// Test environment setup and teardown
#[derive(Debug)]
pub struct TestEnvironment {
    pub temp_dir: std::path::PathBuf,
    pub config: MoonShineConfig,
    pub cleanup_tasks: Vec<CleanupTask>,
}

impl TestEnvironment {
    /// Create new test environment with temporary directory
    pub fn new() -> Result<Self> {
        let temp_dir = std::env::temp_dir().join(format!("moon-shine-test-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir)?;

        Ok(Self {
            temp_dir,
            config: MoonShineConfig::test_defaults(),
            cleanup_tasks: Vec::new(),
        })
    }

    /// Add cleanup task to run on environment teardown
    pub fn add_cleanup_task(&mut self, task: CleanupTask) {
        self.cleanup_tasks.push(task);
    }

    /// Setup test files in temporary directory
    pub fn setup_test_files(&self, files: HashMap<String, String>) -> Result<()> {
        for (relative_path, content) in files {
            let full_path = self.temp_dir.join(&relative_path);
            if let Some(parent) = full_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(full_path, content)?;
        }
        Ok(())
    }
}

impl Drop for TestEnvironment {
    fn drop(&mut self) {
        // Execute cleanup tasks
        for task in &self.cleanup_tasks {
            if let Err(e) = task.execute() {
                eprintln!("Cleanup task failed: {}", e);
            }
        }

        // Remove temporary directory
        if self.temp_dir.exists() {
            if let Err(e) = std::fs::remove_dir_all(&self.temp_dir) {
                eprintln!("Failed to cleanup test directory: {}", e);
            }
        }
    }
}

/// Cleanup task for test environment teardown
#[derive(Debug)]
pub struct CleanupTask {
    pub name: String,
    pub action: CleanupAction,
}

impl CleanupTask {
    pub fn new(name: String, action: CleanupAction) -> Self {
        Self { name, action }
    }

    pub fn execute(&self) -> Result<()> {
        match &self.action {
            CleanupAction::RemoveFile(path) => {
                if path.exists() {
                    std::fs::remove_file(path)?;
                }
            }
            CleanupAction::RemoveDir(path) => {
                if path.exists() {
                    std::fs::remove_dir_all(path)?;
                }
            }
            CleanupAction::Custom(func) => {
                func()?;
            }
        }
        Ok(())
    }
}

/// Cleanup actions for test environment
#[derive(Debug)]
pub enum CleanupAction {
    RemoveFile(std::path::PathBuf),
    RemoveDir(std::path::PathBuf),
    Custom(Box<dyn Fn() -> Result<()> + Send + Sync>),
}

// Test helpers for different methodologies
impl MoonShineConfig {
    /// Create test configuration for London school tests
    pub fn london_test() -> Self {
        Self {
            ai_model: Some("test-mock-model".to_string()),
            // max_files field removed
            include_patterns: Some(vec!["**/*.test.ts".to_string()]),
            exclude_patterns: Some(vec!["**/node_modules/**".to_string()]),
            // cache_enabled field removed
            ..Default::default()
        }
    }

    /// Create test configuration for Chicago school tests
    pub fn chicago_test() -> Self {
        Self {
            ai_model: Some("test-integration-model".to_string()),
            // max_files field removed
            include_patterns: Some(vec!["**/*.ts".to_string(), "**/*.tsx".to_string()]),
            exclude_patterns: Some(vec!["**/node_modules/**".to_string()]),
            // cache_enabled field removed
            ..Default::default()
        }
    }

    /// Create test configuration for E2E tests
    pub fn e2e_test() -> Self {
        Self {
            ai_model: Some("test-e2e-model".to_string()),
            // max_files field removed
            include_patterns: Some(vec!["**/*.ts".to_string(), "**/*.tsx".to_string(), "**/*.js".to_string()]),
            exclude_patterns: Some(vec!["**/node_modules/**".to_string(), "**/dist/**".to_string()]),
            // cache_enabled field removed
            ..Default::default()
        }
    }

    /// Create default test configuration
    pub fn test_defaults() -> Self {
        Self {
            ai_model: Some("test-default-model".to_string()),
            // max_files field removed
            include_patterns: Some(vec!["**/*.ts".to_string()]),
            exclude_patterns: Some(vec!["**/node_modules/**".to_string()]),
            // cache_enabled field removed
            ..Default::default()
        }
    }
}
