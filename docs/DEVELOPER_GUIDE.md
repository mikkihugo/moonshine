# Moon Shine AI Linter - Developer Guide

## 🏗️ Architecture Overview

Moon Shine is built as a WASM extension for Moon using the PDK (Plugin Development Kit). The architecture combines ultra-fast static analysis with intelligent AI enhancements.

### Core Components

```
┌─────────────────────────────────────────────────────────────┐
│                     Moon PDK Interface                      │
├─────────────────────────────────────────────────────────────┤
│  Workflow Engine  │  Provider Router  │  Configuration     │
├─────────────────────────────────────────────────────────────┤
│        OXC Adapter        │        AI Behavioral         │
├─────────────────────────────────────────────────────────────┤
│     Rule Registry     │     Quality Metrics    │ Cache   │
├─────────────────────────────────────────────────────────────┤
│                        Storage Layer                        │
└─────────────────────────────────────────────────────────────┘
```

## 🔧 Development Setup

### Prerequisites

```bash
# Rust toolchain with WASM target
rustup target add wasm32-unknown-unknown

# Moon CLI for testing
curl -fsSL https://moonrepo.dev/install/moon.sh | bash

# Development dependencies
cargo install wasm-pack
cargo install extism-cli
```

### Build from Source

```bash
# Clone repository
git clone https://github.com/moonrepo/moon-shine.git
cd moon-shine

# Install dependencies
cargo fetch

# Build WASM extension
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test
cargo test --target wasm32-unknown-unknown
```

### Development Workflow

```bash
# Watch mode for development
cargo watch -x "build --target wasm32-unknown-unknown"

# Test specific module
cargo test oxc_adapter::tests

# Benchmark performance
cargo bench

# Check WASM binary size
wasm-pack build --target nodejs --out-dir pkg
ls -la pkg/
```

## 📁 Project Structure

```
src/
├── lib.rs                    # WASM entry point
├── config/                   # Configuration management
│   ├── mod.rs
│   ├── ai_config.rs
│   └── validation.rs
├── workflow/                 # Workflow orchestration
│   ├── mod.rs
│   ├── engine.rs
│   └── actions.rs
├── provider_router/          # AI provider management
│   ├── mod.rs
│   ├── claude.rs
│   ├── openai.rs
│   └── selection.rs
├── oxc_adapter/             # OXC integration + AI behavioral
│   ├── mod.rs
│   ├── ai_behavioral.rs
│   └── integration.rs
├── rulebase/                # Rule management system
│   ├── mod.rs
│   ├── ai_enhanced_executor.rs
│   ├── adaptive_learning.rs
│   └── quality_metrics.rs
├── ai_linter/               # Core AI linter components
│   ├── mod.rs
│   ├── coordinator.rs
│   ├── cache.rs
│   └── optimization.rs
├── storage/                 # Persistence layer
│   ├── mod.rs
│   ├── kv_store.rs
│   └── cache_manager.rs
└── moon_pdk_interface.rs    # Moon PDK bindings

tests/                       # Test suite
├── integration/
├── benchmarks/
└── fixtures/

docs/                        # Documentation
├── architecture/
├── api/
└── examples/
```

## 🧩 Core APIs

### Moon PDK Interface

```rust
// Main WASM entry point
#[host_fn]
extern "ExtismHost" {
    fn execute_command(command: &str) -> String;
    fn read_file(path: &str) -> String;
    fn write_file(path: &str, content: &str);
}

// Extension lifecycle
#[plugin_fn]
pub fn configure(config: String) -> FnResult<String> {
    let config: MoonShineConfig = serde_json::from_str(&config)?;
    let manager = ConfigManager::new(config);
    Ok(serde_json::to_string(&manager.validate()?)?)
}

#[plugin_fn]
pub fn analyze_code(input: String) -> FnResult<String> {
    let request: AnalysisRequest = serde_json::from_str(&input)?;
    let analyzer = create_analyzer()?;
    let result = analyzer.analyze(request).await?;
    Ok(serde_json::to_string(&result)?)
}
```

### AI Provider Interface

```rust
#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn analyze_code(
        &self,
        context: AIContext,
        code: &str,
        language: &str,
    ) -> Result<AIAnalysisResult>;

    fn get_capabilities(&self) -> ProviderCapabilities;
    fn estimate_cost(&self, input_size: usize) -> f64;
    fn get_rate_limits(&self) -> RateLimits;
}

// Claude implementation
pub struct ClaudeProvider {
    client: AnthropicClient,
    model: String,
    config: ClaudeConfig,
}

impl AIProvider for ClaudeProvider {
    async fn analyze_code(
        &self,
        context: AIContext,
        code: &str,
        language: &str,
    ) -> Result<AIAnalysisResult> {
        let prompt = self.build_analysis_prompt(context, code, language);
        let response = self.client.complete(&prompt).await?;
        Ok(self.parse_response(response)?)
    }
}
```

### Workflow Engine

```rust
pub struct WorkflowEngine {
    graph: StableGraph<WorkflowNode, WorkflowEdge>,
    executor: WorkflowExecutor,
    state: WorkflowState,
}

impl WorkflowEngine {
    pub async fn execute_workflow(
        &mut self,
        definition: WorkflowDefinition,
        context: ExecutionContext,
    ) -> Result<WorkflowResult> {
        let execution_plan = self.plan_execution(&definition)?;

        for action in execution_plan.actions {
            match action {
                WorkflowAction::StaticAnalysis(config) => {
                    self.execute_static_analysis(config, &context).await?;
                }
                WorkflowAction::AiLinting(config) => {
                    self.execute_ai_linting(config, &context).await?;
                }
                WorkflowAction::QualityGating(config) => {
                    self.execute_quality_gates(config, &context).await?;
                }
            }
        }

        Ok(self.collect_results())
    }
}
```

### Rule System

```rust
pub trait Rule: Send + Sync {
    fn get_metadata(&self) -> RuleMetadata;
    fn execute(&self, context: &RuleContext) -> RuleResult;
    fn can_auto_fix(&self) -> bool;
    fn get_confidence(&self, context: &RuleContext) -> f64;
}

// AI-enhanced rule implementation
pub struct AIEnhancedRule {
    base_rule: Box<dyn Rule>,
    ai_provider: Arc<dyn AIProvider>,
    enhancement_config: AIEnhancementConfig,
}

impl Rule for AIEnhancedRule {
    fn execute(&self, context: &RuleContext) -> RuleResult {
        let static_result = self.base_rule.execute(context);

        if self.should_enhance(&static_result, context) {
            let ai_result = self.ai_provider
                .analyze_code(AIContext::RuleEnhancement, context.code, context.language)
                .await?;

            self.merge_results(static_result, ai_result)
        } else {
            static_result
        }
    }
}
```

## 🎯 Creating Custom Rules

### Static Rule

```rust
use moon_shine::rulebase::{Rule, RuleMetadata, RuleContext, RuleResult};

pub struct NoConsoleLogRule;

impl Rule for NoConsoleLogRule {
    fn get_metadata(&self) -> RuleMetadata {
        RuleMetadata {
            id: "no-console-log".to_string(),
            name: "No Console Log".to_string(),
            description: "Disallow console.log statements".to_string(),
            category: RuleCategory::BestPractices,
            severity: RuleSeverity::Warning,
            languages: vec!["javascript", "typescript"],
            auto_fixable: true,
        }
    }

    fn execute(&self, context: &RuleContext) -> RuleResult {
        let issues = find_console_logs(context.code);
        RuleResult {
            issues,
            confidence: 1.0, // Static rules have perfect confidence
            metadata: self.get_metadata(),
        }
    }

    fn can_auto_fix(&self) -> bool {
        true
    }
}

fn find_console_logs(code: &str) -> Vec<Issue> {
    // Implementation using OXC parser
    let mut issues = Vec::new();
    // ... parsing logic
    issues
}
```

### AI-Enhanced Rule

```rust
use moon_shine::rulebase::{AIEnhancedRule, AIEnhancementConfig};

pub fn create_performance_rule() -> AIEnhancedRule {
    let base_rule = Box::new(StaticPerformanceRule::new());
    let ai_provider = Arc::new(ClaudeProvider::new(ClaudeConfig::default()));

    let enhancement_config = AIEnhancementConfig {
        confidence_threshold: 0.7,
        enhancement_triggers: vec![
            EnhancementTrigger::ComplexCode,
            EnhancementTrigger::PerformanceCritical,
        ],
        ai_context: AIContext::PerformanceAnalysis,
    };

    AIEnhancedRule::new(base_rule, ai_provider, enhancement_config)
}
```

### Behavioral Pattern Rule

```rust
use moon_shine::oxc_adapter::ai_behavioral::{BehavioralPattern, PatternConfig};

pub struct SecurityPatternRule {
    pattern: BehavioralPattern,
    ai_provider: Arc<dyn AIProvider>,
}

impl SecurityPatternRule {
    pub fn new() -> Self {
        let pattern = BehavioralPattern {
            pattern_type: PatternType::Security,
            triggers: vec![
                PatternTrigger::ExternalInput,
                PatternTrigger::DataFlow,
                PatternTrigger::PrivilegeEscalation,
            ],
            confidence_threshold: 0.8,
            description: "Detects potential security vulnerabilities".to_string(),
        };

        Self {
            pattern,
            ai_provider: Arc::new(ClaudeProvider::new(ClaudeConfig::security_focused())),
        }
    }
}

impl Rule for SecurityPatternRule {
    fn execute(&self, context: &RuleContext) -> RuleResult {
        // First check if pattern might apply
        if !self.pattern.quick_check(context.code) {
            return RuleResult::empty();
        }

        // Use AI for deep analysis
        let ai_result = self.ai_provider
            .analyze_code(AIContext::SecurityAnalysis, context.code, context.language)
            .await?;

        self.convert_ai_result_to_rule_result(ai_result)
    }
}
```

## 🔌 Adding New AI Providers

### Provider Implementation

```rust
use moon_shine::provider_router::{AIProvider, ProviderCapabilities, RateLimits};

pub struct CustomAIProvider {
    api_client: CustomAPIClient,
    config: CustomProviderConfig,
}

impl AIProvider for CustomAIProvider {
    async fn analyze_code(
        &self,
        context: AIContext,
        code: &str,
        language: &str,
    ) -> Result<AIAnalysisResult> {
        let prompt = match context {
            AIContext::SecurityAnalysis => self.build_security_prompt(code, language),
            AIContext::PerformanceAnalysis => self.build_performance_prompt(code, language),
            AIContext::CodeQuality => self.build_quality_prompt(code, language),
            _ => self.build_general_prompt(code, language),
        };

        let response = self.api_client
            .complete(&prompt)
            .timeout(Duration::from_secs(30))
            .await?;

        Ok(self.parse_response(response)?)
    }

    fn get_capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            max_context_length: 32768,
            supports_streaming: true,
            supports_function_calling: false,
            cost_per_1k_tokens: 0.002,
            languages: vec!["javascript", "typescript", "python", "rust"],
        }
    }

    fn estimate_cost(&self, input_size: usize) -> f64 {
        let tokens = input_size / 4; // Rough token estimation
        (tokens as f64 / 1000.0) * self.get_capabilities().cost_per_1k_tokens
    }

    fn get_rate_limits(&self) -> RateLimits {
        RateLimits {
            requests_per_minute: 60,
            tokens_per_minute: 100_000,
            requests_per_day: 1000,
        }
    }
}
```

### Provider Registration

```rust
// In src/provider_router/mod.rs
pub fn register_custom_provider() {
    let mut registry = PROVIDER_REGISTRY.write().unwrap();
    registry.register(
        "custom-ai".to_string(),
        Box::new(|| Box::new(CustomAIProvider::new()))
    );
}

// Usage in configuration
let config = ProviderConfig {
    name: "custom-ai".to_string(),
    settings: CustomProviderConfig {
        api_endpoint: "https://api.custom-ai.com/v1".to_string(),
        api_key: env::var("CUSTOM_AI_API_KEY")?,
        model: "custom-model-v1".to_string(),
    },
};
```

## 📊 Performance Optimization

### Caching Strategy

```rust
use moon_shine::storage::{CacheManager, CacheKey, CacheEntry};

pub struct PerformantAnalyzer {
    cache: Arc<CacheManager>,
    static_analyzer: StaticAnalyzer,
    ai_provider: Arc<dyn AIProvider>,
}

impl PerformantAnalyzer {
    pub async fn analyze_with_caching(
        &self,
        code: &str,
        language: &str,
    ) -> Result<AnalysisResult> {
        let cache_key = CacheKey::from_code(code, language);

        // Check memory cache first
        if let Some(cached) = self.cache.get_memory(&cache_key) {
            return Ok(cached);
        }

        // Check persistent cache
        if let Some(cached) = self.cache.get_persistent(&cache_key).await? {
            self.cache.set_memory(cache_key.clone(), cached.clone());
            return Ok(cached);
        }

        // Perform actual analysis
        let result = self.analyze_fresh(code, language).await?;

        // Cache the result
        self.cache.set_memory(cache_key.clone(), result.clone());
        self.cache.set_persistent(cache_key, result.clone()).await?;

        Ok(result)
    }
}
```

### Incremental Analysis

```rust
pub struct IncrementalAnalyzer {
    file_hashes: HashMap<PathBuf, String>,
    analysis_cache: HashMap<String, AnalysisResult>,
}

impl IncrementalAnalyzer {
    pub async fn analyze_project(&mut self, project_path: &Path) -> Result<ProjectAnalysisResult> {
        let files = discover_source_files(project_path)?;
        let mut changed_files = Vec::new();
        let mut results = HashMap::new();

        for file_path in files {
            let content = fs::read_to_string(&file_path)?;
            let current_hash = calculate_hash(&content);

            if let Some(cached_hash) = self.file_hashes.get(&file_path) {
                if cached_hash == &current_hash {
                    // File unchanged, use cached result
                    if let Some(cached_result) = self.analysis_cache.get(cached_hash) {
                        results.insert(file_path.clone(), cached_result.clone());
                        continue;
                    }
                }
            }

            // File changed or not cached, analyze
            changed_files.push((file_path.clone(), content));
            self.file_hashes.insert(file_path, current_hash);
        }

        // Analyze changed files in parallel
        let analysis_tasks: Vec<_> = changed_files
            .into_iter()
            .map(|(path, content)| {
                let analyzer = self.analyzer.clone();
                tokio::spawn(async move {
                    let result = analyzer.analyze(&content, &detect_language(&path)).await?;
                    Ok((path, result))
                })
            })
            .collect();

        for task in analysis_tasks {
            let (path, result) = task.await??;
            let hash = self.file_hashes.get(&path).unwrap();
            self.analysis_cache.insert(hash.clone(), result.clone());
            results.insert(path, result);
        }

        Ok(ProjectAnalysisResult::new(results))
    }
}
```

## 🧪 Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use moon_shine::test_utils::{MockAIProvider, TestConfig};

    #[tokio::test]
    async fn test_ai_enhanced_rule_execution() {
        let mock_provider = MockAIProvider::new()
            .with_response(AIAnalysisResult {
                issues: vec![
                    Issue::new("Performance issue detected", Severity::Warning, 10, 15)
                ],
                confidence: 0.85,
                reasoning: "Function contains blocking I/O in async context".to_string(),
            });

        let rule = AIEnhancedRule::new(
            Box::new(StaticAsyncRule::new()),
            Arc::new(mock_provider),
            AIEnhancementConfig::default(),
        );

        let context = RuleContext {
            code: "async function test() { fs.readFileSync('file.txt'); }",
            language: "javascript",
            file_path: "test.js".into(),
        };

        let result = rule.execute(&context);
        assert_eq!(result.issues.len(), 1);
        assert!(result.confidence > 0.8);
    }

    #[test]
    fn test_provider_selection_logic() {
        let router = ProviderRouter::new(ProviderConfig::test_config());

        // Test cost-based selection
        let provider = router.select_provider(AIContext::SimpleAnalysis, 1000);
        assert_eq!(provider.name(), "cost-effective-provider");

        // Test quality-based selection
        let provider = router.select_provider(AIContext::SecurityAnalysis, 500);
        assert_eq!(provider.name(), "high-quality-provider");
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_end_to_end_analysis() {
    let temp_dir = tempfile::tempdir()?;
    let test_file = temp_dir.path().join("test.ts");

    fs::write(&test_file, r#"
        function processData(data: any) {
            // Performance issue: blocking operation
            const result = expensive_sync_operation(data);
            return result;
        }
    "#)?;

    let config = MoonShineConfig {
        ai: AIConfig {
            providers: vec![
                ProviderConfig::claude_test_config(),
            ],
            enable_ai_behavioral: true,
            confidence_threshold: 0.7,
        },
        ..Default::default()
    };

    let analyzer = IntegratedAnalyzer::new(config)?;
    let result = analyzer.analyze_file(&test_file).await?;

    assert!(!result.issues.is_empty());
    assert!(result.issues.iter().any(|issue| {
        issue.rule_id.contains("performance") && issue.confidence > 0.7
    }));
}
```

### Property-Based Tests

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_analysis_consistency(code in ".*{0,1000}") {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let analyzer = create_test_analyzer();

            // Same input should produce same output
            let result1 = analyzer.analyze(&code, "javascript").await.unwrap();
            let result2 = analyzer.analyze(&code, "javascript").await.unwrap();

            prop_assert_eq!(result1.issues.len(), result2.issues.len());
            prop_assert_eq!(result1.confidence, result2.confidence);
        });
    }

    #[test]
    fn test_confidence_bounds(
        code in ".*{0,500}",
        language in prop::sample::select(vec!["javascript", "typescript", "rust"])
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let analyzer = create_test_analyzer();
            let result = analyzer.analyze(&code, &language).await.unwrap();

            // Confidence should always be between 0 and 1
            prop_assert!(result.confidence >= 0.0 && result.confidence <= 1.0);

            // All issue confidences should be valid
            for issue in result.issues {
                prop_assert!(issue.confidence >= 0.0 && issue.confidence <= 1.0);
            }
        });
    }
}
```

## 📈 Monitoring & Metrics

### Built-in Metrics

```rust
use moon_shine::metrics::{MetricsCollector, AnalysisMetrics};

pub struct AnalysisMetrics {
    pub analysis_duration: Duration,
    pub ai_api_calls: u32,
    pub total_cost_usd: f64,
    pub cache_hit_rate: f64,
    pub issues_found: u32,
    pub false_positive_rate: f64,
}

impl MetricsCollector {
    pub fn record_analysis(&mut self, metrics: AnalysisMetrics) {
        self.histogram("analysis_duration_ms")
            .record(metrics.analysis_duration.as_millis() as f64);

        self.counter("ai_api_calls_total")
            .increment(metrics.ai_api_calls as u64);

        self.gauge("cost_per_analysis_usd")
            .set(metrics.total_cost_usd);

        self.gauge("cache_hit_rate")
            .set(metrics.cache_hit_rate);
    }
}
```

### Custom Metrics

```rust
use moon_shine::metrics::CustomMetrics;

// Track custom business metrics
pub struct CodeQualityMetrics {
    technical_debt_ratio: f64,
    maintainability_index: f64,
    security_score: f64,
}

impl CustomMetrics for CodeQualityMetrics {
    fn collect(&self) -> Vec<Metric> {
        vec![
            Metric::gauge("technical_debt_ratio", self.technical_debt_ratio),
            Metric::gauge("maintainability_index", self.maintainability_index),
            Metric::gauge("security_score", self.security_score),
        ]
    }
}
```

## 🚀 Deployment

### WASM Optimization

```toml
# Cargo.toml optimization settings
[profile.release]
opt-level = "s"        # Optimize for size
lto = true            # Link-time optimization
codegen-units = 1     # Better optimization
panic = "abort"       # Smaller binary size

[profile.release.package."*"]
opt-level = "s"

# WASM-specific optimizations
[target.wasm32-unknown-unknown]
rustflags = [
    "-C", "target-feature=+simd128",    # Enable SIMD
    "-C", "link-arg=--import-memory",   # Import memory
]
```

### Binary Size Optimization

```bash
# Build optimized WASM
cargo build --target wasm32-unknown-unknown --release

# Further optimize with wasm-opt
wasm-opt -Oz --enable-simd target/wasm32-unknown-unknown/release/moon_shine.wasm -o moon_shine_optimized.wasm

# Check size
ls -la *.wasm
# Before: 2.1MB
# After:  680KB (68% reduction)
```

### Distribution

```bash
# Package for distribution
extism-cli generate --wasm moon_shine_optimized.wasm > moon-shine-plugin.json

# Test plugin
extism call moon-shine-plugin.json analyze --input '{"code": "console.log()", "language": "javascript"}'
```

## 🔍 Debugging

### Debug Configuration

```rust
#[cfg(debug_assertions)]
pub fn enable_debug_logging() {
    env_logger::Builder::from_env(Env::default().default_filter_or("moon_shine=debug"))
        .init();
}

// Debug-specific features
#[cfg(debug_assertions)]
impl AIProvider for DebugProvider {
    async fn analyze_code(&self, ...) -> Result<AIAnalysisResult> {
        log::debug!("AI analysis request: {} chars of {} code", code.len(), language);
        let start = Instant::now();

        let result = self.inner.analyze_code(context, code, language).await?;

        log::debug!("AI analysis completed in {:?}", start.elapsed());
        log::debug!("Found {} issues with avg confidence {:.2}",
                   result.issues.len(),
                   result.confidence);

        Ok(result)
    }
}
```

### Performance Profiling

```rust
use moon_shine::profiling::{ProfiledAnalyzer, ProfilingConfig};

pub async fn profile_analysis() -> Result<()> {
    let config = ProfilingConfig {
        sample_rate: 1000, // Sample every 1000 operations
        track_memory: true,
        track_ai_calls: true,
    };

    let analyzer = ProfiledAnalyzer::new(config);

    // Run analysis with profiling
    let result = analyzer.analyze_with_profiling("test.ts", sample_code).await?;

    // Export profiling data
    let profile = analyzer.export_profile();
    fs::write("profile.json", serde_json::to_string_pretty(&profile)?)?;

    println!("Profiling data saved to profile.json");
    Ok(())
}
```

## 🤝 Contributing

### Development Guidelines

1. **Code Style**: Use `rustfmt` and `clippy`
2. **Testing**: Maintain >90% test coverage
3. **Documentation**: Document all public APIs
4. **Performance**: Benchmark critical paths
5. **Security**: No hardcoded secrets or unsafe code

### Adding Features

```bash
# Create feature branch
git checkout -b feature/new-ai-provider

# Implement with tests
cargo test
cargo bench

# Check code quality
cargo fmt
cargo clippy

# Submit PR with documentation
```

---

*For more information, see the [API Reference](./API_REFERENCE.md) and [Examples](./examples/)*