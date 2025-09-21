# 🤖 Moon Shine - AI Agent Integration Guide

## 📖 Overview

Moon Shine implements a sophisticated AI agent integration system that coordinates between WASM-based analysis and Moon task execution to provide intelligent code improvements. This document details how AI agents are integrated and orchestrated within the Moon Shine ecosystem.

## 🏗️ Agent Architecture

### **Hybrid AI Processing Model**

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   WASM Agent    │───▶│  Session Manager │───▶│  Moon Task      │
│   Coordinator   │    │  JSON Protocol   │    │  Agents         │
└─────────────────┘    └──────────────────┘    └─────────────────┘
        │                        │                       │
        ▼                        ▼                       ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│ Static Analysis │    │ Request/Response │    │ Claude AI Agent │
│ Pattern Matching│    │ File Coordination│    │ Native Tools    │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## 🧠 AI Agent Types

### **1. WASM Coordination Agent** (`lib.rs:464`)

**Role**: Primary orchestrator and basic static analysis
**Capabilities**:
- Language detection and file pattern matching
- Basic TypeScript/JavaScript static analysis
- Session management and cleanup coordination
- Moon task pipeline orchestration

**Code Location**: `src/lib.rs:ai_lint_file()`

```rust
#[plugin_fn]
pub fn ai_lint_file(Json(request): Json<AiLintRequest>) -> FnResult<Json<AiLintResponse>>
```

### **2. Static Analysis Agent** (`linter.rs:282`)

**Role**: Enhanced file-path-aware static analysis
**Capabilities**:
- Context-aware code pattern detection
- File-path-specific suggestions
- Multi-language syntax analysis

**Code Location**: `src/linter.rs:static_analysis_with_path()`

```rust
pub fn static_analysis_with_path(&self, content: &str, language: &str, file_path: &str) -> Vec<AiSuggestion>
```

### **3. Claude AI Agent** (Moon Task Integration)

**Role**: Intelligent code fixing and improvement
**Capabilities**:
- Natural language code analysis
- Comprehensive TypeScript/ESLint/Prettier fixes
- TSDoc generation and improvement
- Modern pattern application

**Integration**: Moon task with JSON communication

```bash
# Claude AI processing via Moon task
echo "$USER_PROMPT" | ~/.local/bin/claude --print --output-format json --disallowed-tools "Write,Edit,MultiEdit"
```

### **4. Strict TypeScript Agent** (`strict_ts_integration.rs`)

**Role**: Advanced TypeScript compilation analysis
**Capabilities**:
- Strict compilation checking
- Type safety analysis
- Modern TypeScript pattern detection

**Optional**: Enabled with `strict-ts` feature flag

```rust
#[cfg(feature = "strict-ts")]
let strict_checker = StrictTypeScriptChecker::new(strict_config);
```

## 🏗️ Workflow Engine

Moon Shine now features a **Workflow Engine** that serves as the central coordination system for all AI agents and workflow phases. This engine replaces the legacy parallel lint runner and provides a sophisticated execution model using petgraph-based DAG execution.

### **Engine Architecture**

The workflow engine models analysis pipelines as directed acyclic graphs (DAGs) where:
- Each node is a [`WorkflowStep`] with specific actions (OXC parsing, AI enhancement, etc.)
- Edges represent dependencies between steps
- Execution follows topological order with parallelization where possible

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   WASM Agent    │───▶│  Workflow        │───▶│  Moon Task      │
│   Coordinator   │    │  Engine          │    │  Agents         │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                        │                       │
         ▼                        ▼                       ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│ Step Execution  │    │ Result           │    │ Agent           │
│ Engine          │    │ Aggregation      │    │ Coordination    │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### **Step Execution Model**

The engine supports comprehensive analysis pipelines:

#### **Foundation Steps** (Sequential Execution)
- **OXC Parsing & Analysis**: AST parsing, semantic analysis, type checking
- **Language Detection**: File type identification and validation
- **Session Management**: Directory creation and coordination setup

#### **Analysis Steps** (Parallel Execution)
- **OXC Rules**: 582+ linting rules with AI enhancement
- **SunLinter Behavioral**: 192 behavioral patterns with hybrid analysis
- **Type Analysis**: TypeScript type checking and inference

#### **Enhancement Steps** (Conditional Execution)
- **AI Enhancement**: Claude AI-powered code improvements and suggestions
- **Code Generation**: Apply fixes and generate final output
- **Formatting**: Code formatting with style preservation

### **Key Features**

#### **DAG-based Execution**
```rust
// Automatic dependency resolution and parallel execution
let steps = create_moonshine_oxc_workflow();
let mut engine = WorkflowEngine::new(
    steps,
    "function foo() { return 42; }".to_string(),
    "src/main.ts".to_string(),
    MoonShineConfig::default(),
)?;

let result = engine.execute().await?;
```

#### **Cost-Aware AI**
- **Intelligent AI Usage**: Quick assessment to determine optimal AI strategy
- **Dynamic Workflow Modification**: Adjusts pipeline based on code complexity
- **Budget Optimization**: Skip AI when static analysis is sufficient

#### **Async Coordination**
- **Tokio Integration**: Full async/await support with cancellation
- **Timeout Handling**: Configurable timeouts for each step
- **Retry Logic**: Exponential backoff for resilient operation

### **Configuration**

```rust
pub struct MoonShineConfig {
    pub max_iterations: u32,                    // Maximum workflow iterations
    pub quality_threshold: f64,                 // Convergence quality threshold
    pub enable_claude_ai: bool,                 // Enable AI enhancement
    pub enable_parallel_execution: bool,        // Enable concurrent steps
    pub timeout_per_step_ms: u64,              // Per-step timeout
    pub retry_failed_steps: bool,              // Retry failed steps
    pub keep_debug_sessions: bool,             // Preserve session data
}
```

### **Usage Examples**

#### **Basic Workflow Execution**
```rust
use moon_shine::workflow_engine::{WorkflowEngine, create_moonshine_oxc_workflow};
use moon_shine::config::MoonShineConfig;

let steps = create_moonshine_oxc_workflow();
let mut engine = WorkflowEngine::new(
    steps,
    "function foo() { return 42; }".to_string(),
    "src/main.ts".to_string(),
    MoonShineConfig::default(),
)?;

let result = engine.execute().await?;
println!("Workflow completed: {}", result.success);
```

#### **Intelligent Workflow**
```rust
let mut engine = WorkflowEngine::create_intelligent_workflow(
    source_code,
    file_path,
    config,
)?;

// Execute with cost-aware AI assessment
let result = engine.execute().await?;

// Optionally modify workflow based on assessment
engine.modify_workflow_based_on_assessment().await?;
```

#### **Debug and Monitoring**
```bash
# Execute with debug session preservation
moon ext moon-shine --keep-debug-sessions src/

# Check workflow steps
moon ext moon-shine --dry-run src/
```

### **Migration from Legacy Systems**

The workflow engine replaces both the legacy `workflow.rs` and `parallel_lint_runner.rs` systems:

**Legacy Parallel Runner:**
```bash
moon run moon-shine -- --mode parallel-lint src/
```

**New Workflow Engine:**
```bash
moon ext moon-shine src/
```

**Benefits:**
- ✅ **Unified Architecture**: Single entry point for all workflow types
- ✅ **Better Performance**: DAG-based execution with intelligent parallelization
- ✅ **Enhanced Error Handling**: Robust recovery and retry mechanisms
- ✅ **Comprehensive Monitoring**: Detailed metrics and debugging support
- ✅ **Future-Proof**: Extensible design for new steps and agents

### **Performance Characteristics**

| Step Type | Execution Time | Concurrency | Use Case |
|-----------|----------------|-------------|----------|
| **Foundation** | 50-500ms | Sequential | OXC parsing, type analysis |
| **Analysis** | 100-800ms | Parallel | Rules, behavioral patterns |
| **Enhancement** | 2-10s | Conditional | AI enhancement, code generation |

### **Error Handling & Recovery**

- **Step-Level Isolation**: Individual step failures don't break the pipeline
- **Configurable Retries**: Automatic retry with exponential backoff
- **Graceful Degradation**: Continues execution with non-critical step failures
- **Comprehensive Logging**: Detailed error context and debugging information

## 🔄 Agent Communication Protocol

### **Session-Based JSON Protocol**

Each file processing creates a session directory with structured communication:

```
/tmp/moon-shine/20241219/session-143052-a1b2c3d4/
├── request.json           # WASM → Moon tasks
├── typescript-response.json  # TypeScript agent → WASM
├── eslint-response.json      # ESLint agent → WASM
├── prettier-response.json    # Prettier agent → WASM
├── tsdoc-response.json       # TSDoc agent → WASM
└── claude-response.json      # Claude AI agent → WASM
```

### **Agent Request Structure**

```json
{
  "file_path": "src/component.tsx",
  "language": "typescript",
  "content": "...",
  "wasm_analysis": {
    "suggestions": [...],
    "tsdoc_coverage": 75.0,
    "quality_score": 85.2,
    "parse_errors": []
  },
  "task_config": {
    "enable_strict_typescript": true,
    "enable_eslint": true,
    "enable_prettier": true,
    "enable_tsdoc": true,
    "enable_claude_ai": true
  },
  "session_dir": "/tmp/moon-shine/20241219/session-143052-a1b2c3d4",
  "request_id": "uuid",
  "timestamp": "2024-12-19T14:30:52Z"
}
```

### **Agent Response Structure**

```json
{
  "request_id": "uuid",
  "task_name": "claude-json",
  "success": true,
  "error": null,
  "results": {
    "claude": {
      "success": true,
      "fixed_content": "...",
      "improvements": ["TypeScript fixes", "ESLint resolution", "TSDoc added"],
      "issues_resolved": 5,
      "claude_processing_time_ms": 2500
    }
  },
  "processing_time_ms": 2500,
  "completed_at": "2024-12-19T14:30:55Z"
}
```

## 🎯 Agent Orchestration Flow

### **Multi-Stage Processing Pipeline**

1. **WASM Coordination Phase**
   - File analysis and language detection
   - Basic static analysis and pattern matching
   - Session directory creation
   - Moon task coordination setup

2. **Native Tool Agents Phase** (Sequential)
   - TypeScript compilation analysis
   - ESLint rule checking and auto-fixing
   - Prettier formatting
   - TSDoc coverage analysis

3. **AI Enhancement Phase**
   - Claude AI comprehensive analysis
   - Intelligent code improvements
   - Modern pattern suggestions
   - Quality score calculation

4. **Response Aggregation Phase**
   - Session response collection
   - Final result compilation
   - Session cleanup (if configured)

## 🔧 Agent Configuration

### **AI Agent Settings**

```rust
pub struct AiLinterConfig {
    pub ai_model: String,                           // "claude-3-5-sonnet"
    pub enable_ai_suggestions: bool,                // Enable Claude AI
    pub quality_threshold: f64,                     // 0.8
    pub enable_tsdoc: bool,                        // TSDoc analysis
    pub keep_sessions_for_debug: bool,             // Debug mode
    pub cleanup_sessions_older_than_hours: u32,    // 24
}
```

### **Command Line Agent Control**

```bash
# Enable all agents (default)
moon ext moon-shine src/

# Debug agent sessions
moon ext moon-shine --keep-debug-sessions src/

# Disable specific agents via config
moon ext moon-shine --config '{"enable_claude_ai": false}' src/

# Use Moon commands for development workflow
moon run moon-shine:build     # Compile WASM (uses Moon's intelligent caching)
moon run moon-shine:test      # Run tests (cached, faster subsequent runs)
moon run moon-shine:lint      # Lint code (parallel execution via Moon)
moon run moon-shine:type-check # Type checking (dependency-aware caching)

# DON'T use direct commands (no caching benefits):
# cargo build --target wasm32-wasip1 --release  ❌
# cargo test                                     ❌
# Use Moon instead for intelligent caching! ✅
```

## 🛠️ Agent Development

### **Adding New Agents**

1. **WASM Agent** (Rust)
   - Add to `src/lib.rs` for coordination logic
   - Implement `perform_wasm_analysis()` extensions

2. **Moon Task Agent** (Shell)
   - Add task definition to `moon.yml`
   - Follow JSON protocol structure
   - Use `$MOON_SESSION_DIR` for file coordination

3. **Integration Points**
   - Update `MoonTaskResults` structure
   - Add response aggregation logic
   - Configure in `AiLinterConfig`

### **Agent Performance Optimization**

- **Parallel Execution**: Moon tasks run sequentially but can be parallelized
- **Caching Integration**: Leverage Moon's caching for agent results
- **Session Management**: Efficient cleanup and debugging support
- **Error Isolation**: Individual agent failures don't break the pipeline

## 🔍 Agent Debugging

### **Session Analysis**

```bash
# List all agent sessions
ls -la /tmp/moon-shine/$(date +%Y%m%d)/

# Examine agent request
cat /tmp/moon-shine/20241219/session-*/request.json

# Check agent responses
cat /tmp/moon-shine/20241219/session-*/claude-response.json
cat /tmp/moon-shine/20241219/session-*/typescript-response.json

# Development workflow using Moon (recommended for agents)
moon run moon-shine:build     # Build with caching
moon run moon-shine:test      # Test with dependency caching
moon query projects --id moon-shine --json  # Query project state
moon sync moon-shine          # Sync project state

# For debugging WASM extension specifically
moon ext moon-shine --help    # Extension help
moon ext moon-shine src/      # Run extension on sources
```

### **Agent Monitoring**

- **Processing Times**: Each agent reports execution time
- **Success Rates**: Agent success/failure tracking
- **Quality Metrics**: Code quality improvements per agent
- **Session Correlation**: Complete pipeline visibility

## 🚀 Agent Performance Metrics

### **Typical Agent Performance**

| Agent | Processing Time | Capabilities |
|-------|----------------|--------------|
| WASM Coordinator | <50ms | Static analysis, orchestration |
| TypeScript Agent | 100-500ms | Compilation checking |
| ESLint Agent | 200-800ms | Rule checking, auto-fixing |
| Prettier Agent | 50-200ms | Code formatting |
| TSDoc Agent | 100-300ms | Documentation analysis |
| Claude AI Agent | 2-10s | Intelligent improvements |

### **Quality Improvements**

- **Code Quality Score**: 65-95% typical range
- **TSDoc Coverage**: 0-100% measurement and improvement
- **Issue Resolution**: 3-15 issues per file average
- **Modern Patterns**: Automatic TypeScript modernization

---

**Moon Shine Agent System - Production-Ready AI Integration for Code Excellence**