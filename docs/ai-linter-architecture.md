# AI-Supported Linter Architecture for MoonRepo

## Executive Summary

This document outlines a comprehensive AI-supported linter architecture that integrates seamlessly with the existing Moon Shine infrastructure. The design leverages the current OXC adapter, DSPy framework, provider router, and workflow engine to create an intelligent linting system that combines traditional static analysis with AI-powered pattern detection and code enhancement.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Core Components](#core-components)
3. [AI Enhancement Points](#ai-enhancement-points)
4. [Integration with Existing Infrastructure](#integration-with-existing-infrastructure)
5. [Component Specifications](#component-specifications)
6. [Configuration Schema](#configuration-schema)
7. [Caching and Performance](#caching-and-performance)
8. [Implementation Roadmap](#implementation-roadmap)

## Architecture Overview

### High-Level Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           Moon Shine AI Linter                         │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐     │
│  │   File Input    │───▶│  OXC Adapter    │───▶│ Static Analysis │     │
│  │   Dispatcher    │    │   Pipeline      │    │    Results      │     │
│  └─────────────────┘    └─────────────────┘    └─────────────────┘     │
│           │                       │                       │             │
│           ▼                       ▼                       ▼             │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐     │
│  │ AI Enhancement │───▶│ Provider Router │───▶│ Pattern Cache   │     │
│  │   Coordinator   │    │   (DSPy Core)   │    │   Manager       │     │
│  └─────────────────┘    └─────────────────┘    └─────────────────┘     │
│           │                       │                       │             │
│           ▼                       ▼                       ▼             │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐     │
│  │   AI Models     │    │ Workflow Engine │    │ Result Merger   │     │
│  │ (Claude/GPT/etc)│    │   Orchestrator  │    │ & Formatter     │     │
│  └─────────────────┘    └─────────────────┘    └─────────────────┘     │
├─────────────────────────────────────────────────────────────────────────┤
│                         Existing Infrastructure                         │
│ ┌─────────┐ ┌──────────────┐ ┌──────────────┐ ┌─────────────────────┐ │
│ │   OXC   │ │    DSPy      │ │   Provider   │ │     Workflow        │ │
│ │Adapter  │ │  Framework   │ │    Router    │ │     Engine          │ │
│ └─────────┘ └──────────────┘ └──────────────┘ └─────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────┘
```

### Data Flow Architecture

```
Input Files
    │
    ▼
┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐
│   File Watch    │─────▶│   Incremental   │─────▶│   OXC Parser    │
│   & Discovery   │      │    Analysis     │      │   & Semantic    │
└─────────────────┘      └─────────────────┘      └─────────────────┘
                                                           │
                                                           ▼
┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐
│  AI Enhancement │◀─────│  Complexity     │◀─────│   Traditional   │
│   Decision      │      │   Assessment    │      │   Linting       │
│   Engine        │      │   (Quick)       │      │   (Fast)        │
└─────────────────┘      └─────────────────┘      └─────────────────┘
         │
         ▼
┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐
│   AI Model      │─────▶│   Pattern       │─────▶│   Cache         │
│   Invocation    │      │   Recognition   │      │   Storage       │
│   (DSPy)        │      │   & Analysis    │      │   Layer         │
└─────────────────┘      └─────────────────┘      └─────────────────┘
         │
         ▼
┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐
│   Results       │─────▶│   Format &      │─────▶│   Output        │
│   Integration   │      │   Validation    │      │   Generation    │
│   & Merging     │      │                 │      │                 │
└─────────────────┘      └─────────────────┘      └─────────────────┘
```

## Core Components

### 1. AI Enhancement Coordinator

The central orchestrator that decides when and how to apply AI enhancements to the linting process.

**Responsibilities:**
- Analyze complexity scores from OXC static analysis
- Determine AI enhancement strategy based on file characteristics
- Coordinate between traditional linting and AI-powered analysis
- Manage AI model selection and provider routing

### 2. Enhanced OXC Pipeline

Extended version of the existing OXC adapter with AI integration points.

**Key Features:**
- Maintains existing incremental analysis capabilities
- Adds hooks for AI enhancement at specific pipeline stages
- Preserves performance for simple files (skip AI when unnecessary)
- Integrates with pattern cache for known issue recognition

### 3. AI Pattern Recognition Engine

Leverages the DSPy framework to provide sophisticated pattern detection.

**Capabilities:**
- Complex code smell detection beyond static analysis
- Architectural pattern violations
- Context-aware code suggestions
- Cross-file dependency analysis

### 4. Intelligent Cache System

Multi-layer caching strategy optimized for AI-powered linting.

**Cache Levels:**
- File-level: Traditional OXC incremental cache
- Pattern-level: AI-recognized patterns and their fixes
- Model-level: Cached AI model responses for similar code patterns
- Context-level: Project-specific learning and adaptations

## AI Enhancement Points

### 1. Pre-Analysis Enhancement

**Trigger Conditions:**
- High cyclomatic complexity (>10)
- Large file size (>500 lines)
- High change frequency
- Previous AI-detected patterns

**AI Tasks:**
- Architectural assessment
- Complexity hotspot identification
- Risk area highlighting

### 2. Post-Static Analysis Enhancement

**Trigger Conditions:**
- Multiple traditional lint warnings
- Security-sensitive code patterns
- Performance-critical sections
- External API interactions

**AI Tasks:**
- Context-aware issue prioritization
- Suggested refactoring strategies
- Cross-file impact analysis
- Performance optimization suggestions

### 3. Continuous Learning Enhancement

**Trigger Conditions:**
- User feedback on suggestions
- Repetitive pattern detection
- Project-specific conventions
- Team coding style evolution

**AI Tasks:**
- Pattern learning and adaptation
- Custom rule generation
- Style guide enforcement
- Team preference learning

## Integration with Existing Infrastructure

### OXC Adapter Integration

```rust
// Enhanced OXC adapter with AI integration points
pub struct AiEnhancedOxcAnalyzer {
    base_analyzer: OxcMoonAnalyzer,
    ai_coordinator: AiEnhancementCoordinator,
    pattern_cache: Arc<PatternCacheManager>,
}

impl AiEnhancedOxcAnalyzer {
    pub async fn analyze_with_ai_enhancement(
        &mut self,
        source_code: &str,
        file_path: &str,
    ) -> Result<EnhancedAnalysisResult, Error> {
        // Phase 1: Traditional OXC analysis
        let base_result = self.base_analyzer
            .analyze_incremental()
            .await?;

        // Phase 2: AI enhancement decision
        let enhancement_strategy = self.ai_coordinator
            .assess_enhancement_need(&base_result)
            .await?;

        // Phase 3: Apply AI enhancements if needed
        match enhancement_strategy {
            AiStrategy::SkipAI { .. } => {
                Ok(EnhancedAnalysisResult::from_base(base_result))
            }
            _ => {
                let ai_enhancements = self.ai_coordinator
                    .apply_enhancements(source_code, &base_result, enhancement_strategy)
                    .await?;

                Ok(EnhancedAnalysisResult::merge(base_result, ai_enhancements))
            }
        }
    }
}
```

### DSPy Framework Integration

```rust
// AI pattern recognition using existing DSPy infrastructure
#[derive(Signature)]
struct CodePatternAnalysis {
    #[input]
    source_code: String,

    #[input]
    static_analysis_results: String,

    #[input]
    file_context: String,

    #[output]
    detected_patterns: Vec<CodePattern>,

    #[output]
    suggested_improvements: Vec<CodeSuggestion>,

    #[output]
    complexity_assessment: ComplexityAssessment,
}

impl CodePatternAnalysis {
    pub async fn analyze_patterns(
        &self,
        provider: &dyn ProviderTrait,
    ) -> Result<Prediction, Error> {
        // Leverage existing DSPy prediction pipeline
        // with specialized prompts for code analysis
    }
}
```

### Provider Router Integration

```rust
// Enhanced provider routing with AI model capabilities
pub struct AiLinterProviderRouter {
    base_router: ProviderRouter,
    model_capabilities: HashMap<String, AiModelCapabilities>,
}

impl AiLinterProviderRouter {
    pub async fn select_optimal_provider(
        &self,
        task_type: AiLinterTask,
        complexity_score: f64,
        budget_constraints: Option<f64>,
    ) -> Result<String, Error> {
        // Select provider based on:
        // - Task complexity requirements
        // - Model specialization (code analysis vs generation)
        // - Performance requirements (speed vs accuracy)
        // - Budget constraints
    }
}
```

## Component Specifications

### AI Enhancement Coordinator

```rust
pub struct AiEnhancementCoordinator {
    config: AiLinterConfig,
    provider_router: Arc<AiLinterProviderRouter>,
    pattern_cache: Arc<PatternCacheManager>,
    complexity_analyzer: ComplexityAnalyzer,
    usage_tracker: UsageTracker,
}

impl AiEnhancementCoordinator {
    /// Assess whether a file needs AI enhancement
    pub async fn assess_enhancement_need(
        &self,
        analysis_result: &MoonAnalysisResult,
    ) -> Result<AiStrategy, Error> {
        let complexity = self.complexity_analyzer
            .calculate_complexity(&analysis_result.diagnostics);

        let budget_estimate = self.estimate_cost(complexity);

        match (complexity.score, self.config.ai_threshold) {
            (score, threshold) if score < threshold => {
                Ok(AiStrategy::SkipAI {
                    reason: "Low complexity, traditional linting sufficient".to_string(),
                })
            }
            (score, _) if score < 0.7 => {
                Ok(AiStrategy::LightAI {
                    target_issues: 3,
                    budget_estimate,
                })
            }
            (score, _) if score < 0.9 => {
                Ok(AiStrategy::StandardAI {
                    passes: 2,
                    budget_estimate,
                })
            }
            _ => {
                Ok(AiStrategy::HeavyAI {
                    passes: 3,
                    specialized_models: vec!["claude-3.5-sonnet".to_string()],
                    budget_estimate,
                })
            }
        }
    }

    /// Apply AI enhancements based on strategy
    pub async fn apply_enhancements(
        &self,
        source_code: &str,
        base_result: &MoonAnalysisResult,
        strategy: AiStrategy,
    ) -> Result<AiEnhancementResult, Error> {
        match strategy {
            AiStrategy::LightAI { target_issues, .. } => {
                self.apply_light_enhancement(source_code, base_result, target_issues).await
            }
            AiStrategy::StandardAI { passes, .. } => {
                self.apply_standard_enhancement(source_code, base_result, passes).await
            }
            AiStrategy::HeavyAI { passes, specialized_models, .. } => {
                self.apply_heavy_enhancement(source_code, base_result, passes, specialized_models).await
            }
            _ => Ok(AiEnhancementResult::empty()),
        }
    }
}
```

### Pattern Cache Manager

```rust
pub struct PatternCacheManager {
    memory_cache: Arc<DashMap<String, CachedPattern>>,
    persistent_cache: Arc<dyn PersistentCache>,
    cache_config: CacheConfig,
}

impl PatternCacheManager {
    /// Check if a pattern is already cached
    pub async fn get_cached_pattern(
        &self,
        code_hash: &str,
        context_hash: &str,
    ) -> Option<CachedPattern> {
        // Multi-level cache lookup:
        // 1. Memory cache (fastest)
        // 2. Persistent cache (file-based)
        // 3. Distributed cache (if configured)
    }

    /// Store pattern recognition results
    pub async fn cache_pattern(
        &self,
        code_hash: String,
        context_hash: String,
        pattern: DetectedPattern,
        ttl: Duration,
    ) -> Result<(), Error> {
        // Store in multiple cache levels with appropriate TTL
    }

    /// Invalidate cache when files change
    pub async fn invalidate_file_patterns(
        &self,
        file_path: &str,
    ) -> Result<(), Error> {
        // Smart invalidation based on file dependencies
    }
}
```

### AI Model Interface

```rust
#[async_trait]
pub trait AiLinterModel {
    /// Analyze code patterns and suggest improvements
    async fn analyze_patterns(
        &self,
        request: PatternAnalysisRequest,
    ) -> Result<PatternAnalysisResponse, Error>;

    /// Generate code fixes for detected issues
    async fn generate_fixes(
        &self,
        request: FixGenerationRequest,
    ) -> Result<FixGenerationResponse, Error>;

    /// Assess code complexity and architectural patterns
    async fn assess_architecture(
        &self,
        request: ArchitectureAssessmentRequest,
    ) -> Result<ArchitectureAssessmentResponse, Error>;

    /// Learn from user feedback to improve suggestions
    async fn learn_from_feedback(
        &self,
        feedback: UserFeedback,
    ) -> Result<(), Error>;
}

// Concrete implementations for different providers
pub struct ClaudeAiLinterModel {
    client: ClaudeClient,
    config: ClaudeConfig,
}

pub struct OpenAiLinterModel {
    client: OpenAiClient,
    config: OpenAiConfig,
}

pub struct LocalAiLinterModel {
    model: LocalModel,
    config: LocalConfig,
}
```

## Configuration Schema

### AI Linter Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiLinterConfig {
    /// Global AI enablement
    pub enabled: bool,

    /// Complexity threshold for AI enhancement (0.0-1.0)
    pub ai_threshold: f64,

    /// Maximum budget per file analysis (in tokens or cost)
    pub max_budget_per_file: Option<f64>,

    /// Preferred AI providers in order of preference
    pub preferred_providers: Vec<String>,

    /// Model-specific configurations
    pub model_configs: HashMap<String, ModelConfig>,

    /// Caching configuration
    pub cache_config: CacheConfig,

    /// Pattern learning configuration
    pub learning_config: LearningConfig,

    /// Integration points configuration
    pub integration_config: IntegrationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Model name/identifier
    pub model_name: String,

    /// Temperature for creativity vs consistency
    pub temperature: f64,

    /// Maximum tokens per request
    pub max_tokens: u32,

    /// Timeout for requests
    pub timeout_seconds: u64,

    /// Retry configuration
    pub retry_config: RetryConfig,

    /// Model-specific parameters
    pub model_params: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Enable/disable caching
    pub enabled: bool,

    /// Maximum memory cache size (in MB)
    pub max_memory_cache_size: usize,

    /// Persistent cache directory
    pub cache_directory: Option<PathBuf>,

    /// Default TTL for cached patterns (in seconds)
    pub default_ttl: u64,

    /// Cache compression
    pub compression_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConfig {
    /// Enable learning from user feedback
    pub feedback_learning_enabled: bool,

    /// Enable project-specific pattern learning
    pub project_pattern_learning: bool,

    /// Minimum feedback count before pattern adaptation
    pub min_feedback_threshold: u32,

    /// Learning rate for pattern adaptation
    pub learning_rate: f64,
}
```

### JSON Schema for Configuration

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "AI Linter Configuration",
  "type": "object",
  "properties": {
    "aiLinter": {
      "type": "object",
      "properties": {
        "enabled": {
          "type": "boolean",
          "default": true,
          "description": "Enable AI-powered linting enhancements"
        },
        "aiThreshold": {
          "type": "number",
          "minimum": 0.0,
          "maximum": 1.0,
          "default": 0.5,
          "description": "Complexity threshold for triggering AI analysis"
        },
        "maxBudgetPerFile": {
          "type": "number",
          "minimum": 0,
          "description": "Maximum cost budget per file analysis"
        },
        "preferredProviders": {
          "type": "array",
          "items": {
            "type": "string",
            "enum": ["claude", "openai", "google", "local"]
          },
          "default": ["claude", "openai"],
          "description": "AI providers in order of preference"
        },
        "modelConfigs": {
          "type": "object",
          "additionalProperties": {
            "$ref": "#/definitions/ModelConfig"
          }
        },
        "cacheConfig": {
          "$ref": "#/definitions/CacheConfig"
        }
      },
      "required": ["enabled", "aiThreshold"],
      "additionalProperties": false
    }
  },
  "definitions": {
    "ModelConfig": {
      "type": "object",
      "properties": {
        "modelName": {
          "type": "string",
          "description": "Specific model identifier"
        },
        "temperature": {
          "type": "number",
          "minimum": 0.0,
          "maximum": 2.0,
          "default": 0.3,
          "description": "Sampling temperature for model responses"
        },
        "maxTokens": {
          "type": "integer",
          "minimum": 1,
          "maximum": 32000,
          "default": 4000,
          "description": "Maximum tokens per AI request"
        },
        "timeoutSeconds": {
          "type": "integer",
          "minimum": 1,
          "maximum": 300,
          "default": 30,
          "description": "Request timeout in seconds"
        }
      },
      "required": ["modelName"],
      "additionalProperties": false
    },
    "CacheConfig": {
      "type": "object",
      "properties": {
        "enabled": {
          "type": "boolean",
          "default": true,
          "description": "Enable pattern caching"
        },
        "maxMemoryCacheSize": {
          "type": "integer",
          "minimum": 1,
          "default": 100,
          "description": "Memory cache size in MB"
        },
        "cacheDirectory": {
          "type": "string",
          "description": "Directory for persistent cache storage"
        },
        "defaultTtl": {
          "type": "integer",
          "minimum": 60,
          "default": 3600,
          "description": "Default cache TTL in seconds"
        },
        "compressionEnabled": {
          "type": "boolean",
          "default": true,
          "description": "Enable cache compression"
        }
      },
      "additionalProperties": false
    }
  }
}
```

## Caching and Performance Optimization

### Multi-Level Caching Strategy

#### Level 1: Memory Cache (Fastest)
- **Storage**: In-memory hash map with LRU eviction
- **Scope**: Recently analyzed patterns and AI responses
- **TTL**: 1-4 hours depending on file change frequency
- **Size Limit**: Configurable (default 100MB)

#### Level 2: Persistent File Cache
- **Storage**: Compressed JSON files with metadata
- **Scope**: Project-specific patterns and learned behaviors
- **TTL**: 24-168 hours based on project activity
- **Features**: Atomic writes, corruption recovery, migration support

#### Level 3: Distributed Cache (Optional)
- **Storage**: Redis/Memcached for team sharing
- **Scope**: Team-wide patterns and common fixes
- **TTL**: 1-4 weeks based on team feedback
- **Features**: Cross-developer pattern sharing, anonymized learning

### Performance Optimization Strategies

#### Smart AI Invocation
```rust
pub struct AiInvocationOptimizer {
    complexity_threshold: f64,
    cost_tracker: CostTracker,
    pattern_matcher: PatternMatcher,
}

impl AiInvocationOptimizer {
    /// Decide if AI analysis is warranted
    pub fn should_invoke_ai(
        &self,
        file_stats: &FileStats,
        analysis_result: &MoonAnalysisResult,
    ) -> AiInvocationDecision {
        // Skip AI for:
        // - Simple files with no existing issues
        // - Files with cached patterns (high confidence)
        // - Test files with repetitive patterns
        // - Generated code (package-lock.json, etc.)

        // Prioritize AI for:
        // - Complex business logic files
        // - Files with security implications
        // - High-change frequency files
        // - Files with previous AI-detected issues
    }
}
```

#### Incremental AI Analysis
```rust
pub struct IncrementalAiAnalyzer {
    file_dependencies: DependencyGraph,
    change_detector: ChangeDetector,
    pattern_cache: Arc<PatternCacheManager>,
}

impl IncrementalAiAnalyzer {
    /// Analyze only changed sections and their dependencies
    pub async fn analyze_incremental_changes(
        &self,
        changed_lines: &[LineRange],
        full_content: &str,
        previous_analysis: &Option<AiAnalysisResult>,
    ) -> Result<AiAnalysisResult, Error> {
        // Only re-analyze:
        // - Changed functions/classes
        // - Dependencies of changed code
        // - Cross-cutting concerns (imports, exports)

        // Preserve cached results for:
        // - Unchanged functions
        // - Unrelated code sections
        // - Stable architectural patterns
    }
}
```

#### Parallel Processing Pipeline
```rust
pub struct ParallelAiPipeline {
    thread_pool: ThreadPool,
    rate_limiter: RateLimiter,
    batch_processor: BatchProcessor,
}

impl ParallelAiPipeline {
    /// Process multiple files concurrently with rate limiting
    pub async fn process_files_batch(
        &self,
        files: Vec<FileAnalysisRequest>,
    ) -> Result<Vec<FileAnalysisResult>, Error> {
        // Batch similar files for efficiency
        // Respect API rate limits
        // Load balance across multiple providers
        // Fail gracefully with circuit breaker pattern
    }
}
```

## Implementation Roadmap

### Phase 1: Foundation (Weeks 1-2)
**Goal**: Establish core infrastructure and basic AI integration

**Deliverables**:
- [ ] Enhanced OXC adapter with AI integration points
- [ ] Basic AI enhancement coordinator
- [ ] Simple pattern cache implementation
- [ ] Configuration schema and validation
- [ ] Integration tests with existing Moon PDK

**Success Criteria**:
- AI coordinator can assess when to apply enhancements
- Basic pattern caching works for simple cases
- Configuration is properly validated and loaded
- No regression in existing OXC functionality

### Phase 2: AI Model Integration (Weeks 3-4)
**Goal**: Implement AI model interfaces and provider integration

**Deliverables**:
- [ ] AI model interface traits and implementations
- [ ] DSPy signature definitions for code analysis
- [ ] Provider router enhancements for AI models
- [ ] Error handling and fallback mechanisms
- [ ] Basic pattern recognition workflows

**Success Criteria**:
- Successfully analyze code patterns with Claude/GPT models
- Provider routing works with model capability matching
- Graceful degradation when AI providers are unavailable
- Cost tracking and budget enforcement functional

### Phase 3: Advanced Features (Weeks 5-6)
**Goal**: Implement sophisticated caching, learning, and optimization

**Deliverables**:
- [ ] Multi-level caching system implementation
- [ ] Incremental analysis optimizations
- [ ] User feedback learning mechanisms
- [ ] Performance monitoring and optimization
- [ ] Advanced configuration options

**Success Criteria**:
- Significant performance improvement for repeated analyses
- User feedback improves suggestion quality over time
- Memory and cost usage within acceptable limits
- Advanced configuration options work correctly

### Phase 4: Integration and Polish (Weeks 7-8)
**Goal**: Complete integration with Moon ecosystem and production readiness

**Deliverables**:
- [ ] Complete Moon PDK integration
- [ ] Comprehensive documentation and examples
- [ ] Performance benchmarking and optimization
- [ ] Security audit and hardening
- [ ] Migration tools for existing projects

**Success Criteria**:
- Seamless integration with Moon workflow
- Production-ready performance and reliability
- Security best practices implemented
- Clear migration path for existing users
- Comprehensive documentation available

### Validation and Testing Strategy

#### Unit Testing
- Component isolation testing
- AI model interface mocking
- Cache behavior verification
- Configuration validation testing

#### Integration Testing
- End-to-end workflow testing
- Multiple provider integration
- Performance regression testing
- Error scenario handling

#### Performance Testing
- Large codebase analysis benchmarks
- Memory usage profiling
- AI provider response time analysis
- Cache efficiency measurement

#### Security Testing
- Input sanitization verification
- AI prompt injection prevention
- Sensitive data handling audit
- Access control validation

## Conclusion

This AI-supported linter architecture provides a robust foundation for enhancing the Moon Shine linting capabilities while maintaining compatibility with the existing infrastructure. The design emphasizes:

1. **Incremental Adoption**: Teams can gradually enable AI features
2. **Performance First**: Traditional fast linting for simple cases
3. **Cost Awareness**: Budget controls and intelligent AI invocation
4. **Learning Capability**: Continuous improvement through feedback
5. **Reliability**: Graceful degradation and comprehensive error handling

The modular design allows for independent development and testing of components while ensuring seamless integration with the existing Moon PDK ecosystem.