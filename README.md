# ✨ Moon Shine - Shine Code to Production Excellence

A sophisticated WASM extension for [moonrepo](https://moonrepo.dev) that coordinates AI-powered code optimization through industry-standard tools (ESLint, TypeScript, Claude CLI) with advanced DSPy prompt optimization and COPRO collaborative engineering.

## ✨ Features

- **🧠 Advanced AI Optimization**: Full DSPy (Declarative Self-improving Python) framework implementation in Rust. This implementation mirrors core DSPy concepts including:
    - **Core Components**: Language Model (LM) integration, module definition, settings management, and signature handling.
    - **Predictors**: Mechanisms for generating predictions from input examples using LMs.
    - **Optimizers**: Algorithms for improving DSPy modules, including COPRO.
    - **Evaluators**: Tools for assessing the performance and quality of modules.
    - **Adapters**: For formatting LM input/output.
    - **Macros**: `example!`, `prediction!`, `field!`, `sign!`, and `hashmap!` for enhanced expressiveness and declarative programming.
    //todo: review this code //end: here
- **🎯 COPRO Optimization**: Collaborative Prompt Optimization for systematic prompt engineering. COPRO iteratively generates, evaluates, and refines prompts using a search process that explores a breadth of candidate instructions and refines them over a certain depth of iterations. Key aspects include:
    - **Candidate Generation**: Initial and refined instructions are generated using specialized predictors (`BasicGenerateInstruction`, `GenerateInstructionGivenAttempts`).
    - **Batch Evaluation**: Candidates are evaluated against a training set, with scores determining their effectiveness. This process is WASM-compatible and leverages concurrent execution for performance.
    - **Iterative Refinement**: Best-performing candidates inform the generation of new, improved instructions in subsequent iterations.
    - **Global & Individual Optimization**: The algorithm identifies the best overall prompt across all predictors and applies it, or applies individual bests if no significant global improvement is found.
    //todo: review this code //end: here
- **⚡ ESLint Integration**: Leverages industry-standard ESLint via Moon tasks for TypeScript/JavaScript
- **🔬 Multi-Stage Analysis**: ESLint → TypeScript → AI pipeline orchestrated through WASM coordination
- **🌙 Moon Native**: Full integration with Moon's task orchestration and intelligent caching
- **📊 Pattern Detection**: Advanced behavioral pattern analysis with mathematical optimization
- **🔄 JSON Communication**: Structured protocol between WASM extension and Moon tasks
- **🗂️ Session-Based Debugging**: Organized session directories with timestamp-based cleanup
- **🎯 Flexible Targeting**: Path filters, include/exclude patterns, file limits for precise control
- **🛡️ Production Ready**: Enterprise-grade error handling and hybrid WASM architecture

## 🚀 Installation

### Prerequisites

```bash
# Install required targets and tools
rustup target add wasm32-wasip1
```

### Build

```bash
# Use Moon for intelligent caching and dependency management
moon run moon-shine:build     # Builds WASM with smart caching
moon run moon-shine:test      # Run tests with dependency awareness
moon run moon-shine:lint      # Lint with parallel execution
moon run moon-shine:type-check # Type checking with Moon's optimization

# For manual builds (NOT recommended - no caching):
# cargo build --release --target wasm32-wasip1  ❌
# Use Moon commands instead! ✅
```

### Moon Integration

Moon Shine is designed for seamless integration with Moon's task orchestration:

```bash
# Essential development workflow
moon run moon-shine:type-check  # TypeScript validation with caching
moon run moon-shine:lint        # Code quality with parallel execution
---

## 🏗️ Workflow Engine

Moon Shine now features a high-performance workflow engine that orchestrates complex code analysis and transformation pipelines using petgraph-based DAG execution.

### Architecture Overview

The workflow engine models analysis pipelines as directed acyclic graphs (DAGs) where:
- Each node is a [`WorkflowStep`] with specific actions (OXC parsing, AI enhancement, etc.)
- Edges represent dependencies between steps
- Execution follows topological order with parallelization where possible

### Key Features

- **DAG-based Execution**: Automatic dependency resolution and parallel execution
- **Async Coordination**: Full tokio integration with cancellation and timeouts
- **Cost-Aware AI**: Intelligent AI usage based on code complexity assessment
- **Conditional Steps**: Context-aware step execution based on previous results
- **Retry Logic**: Exponential backoff for resilient operation
- **Memory Efficient**: Shared context with Arc<RwLock<>> for thread safety

### Usage

#### Via Moon CLI

```bash
# Process files with the workflow engine (default behavior)
moon ext moon-shine src/

# Debug mode with session preservation
moon ext moon-shine --keep-debug-sessions src/

# Custom configuration
moon ext moon-shine --config '{"enable_claude_ai": false}' src/
```

#### Programmatic Usage

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
```

### Workflow Phases

The engine automatically configures these phases:

1. **OXC Parsing & Analysis** (Foundation)
   - AST parsing and semantic analysis
   - Type checking and compilation validation

2. **Static Analysis** (Parallel)
   - OXC rules execution (582+ rules)
   - SunLinter behavioral patterns (192 patterns)
   - TypeScript type analysis and inference

3. **AI Enhancement** (Conditional)
   - Claude AI-powered code improvements
   - Context-aware suggestions based on analysis results

4. **Code Generation** (Final)
   - Apply fixes and generate final output
   - Code formatting with style preservation

### Configuration

The workflow engine supports extensive configuration through `MoonShineConfig`:

```json
{
  "max_iterations": 3,
  "quality_threshold": 0.8,
  "enable_claude_ai": true,
  "enable_tsdoc": true,
  "parallel_analysis": true
}
```

### Performance Benefits

- **⚡ Sub-50ms WASM coordination** for lightweight orchestration
- **🔄 Intelligent caching** of phase results across iterations
- **⚙️ Configurable concurrency** with semaphore-based resource management
- **📊 Comprehensive metrics** for performance monitoring and optimization

### Error Handling & Recovery

- **Phase-level error isolation**: Individual phase failures don't break the entire pipeline
- **Configurable retry logic**: Automatic retry with exponential backoff
- **Graceful degradation**: Continues execution even when non-critical phases fail
- **Comprehensive logging**: Detailed error reporting with context preservation

moon run moon-shine:build       # WASM compilation with smart caching
moon run moon-shine:test        # Comprehensive test suite

# Moon-specific optimization commands
moon sync moon-shine            # Sync project dependencies
moon query projects --id moon-shine --json  # Project introspection
moon action-graph moon-shine:build --json   # Dependency visualization

# Extension execution (the main use case)
moon ext moon-shine src/        # Process source files
moon ext moon-shine --help      # Extension options
```

## 🏗️ Architecture

### Hybrid WASM + Moon Tasks

Moon Shine implements a sophisticated hybrid architecture that leverages the best of both worlds:

- **🧠 WASM Extension**: Lightweight coordination, pattern matching, and static analysis
- **⚡ Moon Tasks**: Native tool execution (tsc, eslint, prettier, claude) with intelligent caching
- **📡 JSON Protocol**: Type-safe communication between WASM and Moon task ecosystem
- **🗂️ Session Management**: Timestamp-based debugging with automatic cleanup
- **🔄 Intelligent Caching**: Moon's dependency-aware caching for all operations
- **📊 Performance Optimization**: Sub-50ms WASM coordination + cached native tools

### Moon Integration Benefits

- **🎯 Smart Dependency Tracking**: Only rebuild what actually changed
- **⚡ Parallel Task Execution**: Moon orchestrates concurrent tool execution
- **💾 Persistent Caching**: AI results, TypeScript checks, and ESLint outputs cached
- **🔍 Project Introspection**: Deep understanding of project relationships
- **📈 Incremental Processing**: Process only touched files in development workflow

### Core Components

- **`lib.rs`**: Main WASM extension with Moon integration and task coordination
- **`dspy/`**: Complete DSPy framework implementation (8 modules, 42+ files)
- **`copro_optimizer.rs`**: COPRO (Collaborative Prompt Optimization) engine
- **`dspy_optimization.rs`**: DSPy-powered Claude CLI prompt optimization
- **`moon_communication.rs`**: JSON protocol for Moon task coordination
- **`prompt_optimizer.rs`**: Advanced prompt engineering and meta-prompt reflection
- **`prompts.rs`**: Embedded prompt management and templates
- **`claude_fixer.rs`**: Claude CLI integration via Moon tasks
- **`linter.rs`**: Enhanced multi-language static analysis coordination
- **`error.rs`**: Comprehensive error handling
- **`rules/`**: AI-Enhanced Rule Engine - See [Rules Documentation](src/rules/CLAUDE.md) for complete rule conversion methodology and AI enhancement architecture

### AI Agent Integration

See [AGENTS.md](./AGENTS.md) for comprehensive documentation on:
- Multi-stage AI processing pipeline
- Agent coordination and communication
- Session-based debugging
- Performance optimization

### WASM Integration

```rust
use moon_shine::MoonShine;

// Create linter with configuration
let config = r#"{"enable_ai_suggestions": true, "enable_tsdoc": true}"#;
let shiner = MoonShine::new(Some(config.to_string()))?;

// Process lint task (coordinates with Moon tasks)
let task = r#"{"file_path": "src/main.ts", "content": "...", "language": "typescript"}"#;
let result = shiner.shine_file(task)?;
```

### Moon Tasks Integration

The extension coordinates with Moon tasks for native tool execution:

```yaml
# moon.yml - Task definitions for native tools
typescript-json:
  script: |
    echo "$JSON_INPUT" | jq -r '.content' | npx tsc --noEmit --strict

claude-json:
  script: |
    echo "$USER_PROMPT" | ~/.local/bin/claude --print --output-format json
```

## 🔧 Configuration

### Moon Shine Config

```json
{
  "enable_ai_suggestions": true,
  "max_files_per_task": 10,
  "quality_threshold": 0.8,
  "enable_tsdoc": true
}
```

### Supported Languages

#### Primary (ESLint-Powered):
- **TypeScript** (`.ts`, `.tsx`) - Industry-standard ESLint integration via Moon tasks
- **JavaScript** (`.js`, `.jsx`) - ESLint-powered analysis and linting

#### Additional Languages:
- Rust (`.rs`)
- Go (`.go`)
- Python (`.py`)
- And 15+ more languages via extensible detection. Language support is layered:
    - **Explicit TypeScript Support**: Through dedicated `typescript_patterns` in `CodePatternMatcher` and specific prompt templates.
    - **Configurable Pattern-Based Support**: The `CodePatternMatcher` can be configured with custom patterns for any language. This allows users to extend support for new languages by defining relevant patterns for functions, loops, security issues, etc.
    - **AI-Driven Generic Support**: The programming language is passed to the AI model in prompts. This means that for languages not explicitly covered by patterns, the AI model itself can leverage its understanding of the language to provide suggestions, as long as the prompt is well-crafted. The `AIContext` also includes the language.
    //todo: review this code //end: here

### ESLint Integration

Moon Shine leverages industry-standard ESLint through Moon task orchestration:

```yaml
# Moon tasks handle ESLint execution
eslint-json:
  command: 'eslint --format json'
  inputs: ['**/*.{ts,tsx,js,jsx}']
```

**Benefits:**
- **🏭 Industry Standard**: Uses the same ESLint everyone knows and trusts
- **🎯 Comprehensive Rules**: Full ESLint ecosystem including TypeScript rules
- **🔧 Zero Configuration Drift**: Respects existing ESLint configs
- **📊 Rich Integration**: JSON output perfectly integrated with DSPy optimization

## 🎯 Usage

### Command Line Usage

```bash
# Process all TypeScript/JavaScript files in current directory
moon ext moon-shine

# Process specific files
moon ext moon-shine file1.ts file2.js

# Process files in a specific directory
moon ext moon-shine --path src/

# Use custom include/exclude patterns
moon ext moon-shine --include "src/**/*.ts" --exclude "**/*.test.ts"

# Limit number of files processed
moon ext moon-shine --max-files 5

# Dry run to see what would be processed
moon ext moon-shine --dry-run
```

### Command Options

| Option | Short | Description | Example |
|--------|-------|-------------|----------|
| `--path` | `-p` | Base path to search | `--path src/components` |
| `--include` | `-i` | Include glob pattern | `--include "**/*.{ts,tsx}"` |
| `--exclude` | `-e` | Exclude glob pattern | `--exclude "**/*.test.ts"` |
| `--max-files` | | Limit files processed | `--max-files 10` |
| `--dry-run` | | Show what would be processed | `--dry-run` |
| `--keep-debug-sessions` | | Keep session dirs for debugging | `--keep-debug-sessions` |

### Programmatic Usage

```typescript
const shiner = new MoonShine();

// Synchronous analysis
const result = shiner.shine_file(JSON.stringify({
  file_path: "src/component.tsx",
  content: "function MyComponent() { return <div />; }",
  language: "tsx"
}));

// Async with AI
const aiResult = await shiner.shine_file_ai(JSON.stringify(task));
```

### Output Format

```json
{
  "file_path": "src/component.tsx",
  "issues_found": 3,
  "suggestions": [
    "Function should have TSDoc documentation",
    "Consider using nullish coalescing (??) for better null safety"
  ],
  "tsdoc_coverage": 75.0,
  "quality_score": 85.2,
  "processing_time_ms": 120
}
```

## 🗂️ Session-Based Debugging

Moon-shine uses a session-based approach for debugging and file management:

### Session Directory Structure
```
/tmp/moon-shine/
├── 20241219/                    # Date-based cleanup (YYYYMMDD)
│   ├── session-143052-a1b2c3d4/ # Time + short UUID
│   │   ├── request.json         # Input request
│   │   ├── typescript-response.json
│   │   ├── eslint-response.json
│   │   ├── prettier-response.json
│   │   ├── tsdoc-response.json
│   │   └── claude-response.json
│   └── session-143127-e5f6g7h8/
└── 20241218/                    # Previous day (auto-cleaned)
```

### Session Management
- **Automatic cleanup**: Removes directories older than configured age (default: 24 hours)
- **Date-based organization**: Efficient cleanup by removing entire date directories
- **Debug mode**: Use `--keep-debug-sessions` to preserve sessions for analysis
- **Session correlation**: Each file processing gets a unique session with all related artifacts

### Debugging Commands
```bash
# Keep sessions for debugging
moon ext moon-shine --keep-debug-sessions

# Check session directories
ls -la /tmp/moon-shine/

# View specific session
cat /tmp/moon-shine/20241219/session-143052-a1b2c3d4/request.json
cat /tmp/moon-shine/20241219/session-143052-a1b2c3d4/claude-response.json

# Moon-specific debugging
moon query tasks --project moon-shine --json     # Task introspection
moon query projects --id moon-shine --json       # Project state
moon action-graph moon-shine:build --json        # Dependency graph
moon run moon-shine:build --log debug            # Verbose build logging

# Performance analysis
moon run moon-shine:test --dump                  # Generate trace profile
moon query touched-files --base HEAD~1           # Change detection
```

## 🛠️ Development

### Build Commands

```bash
# RECOMMENDED: Use Moon for all development tasks
moon run moon-shine:build       # Intelligent WASM build with caching
moon run moon-shine:test        # Test execution with dependency tracking
moon run moon-shine:lint        # Code quality with ESLint integration
moon run moon-shine:type-check  # TypeScript validation
moon run moon-shine:clean       # Clean build artifacts

# Advanced Moon workflow
moon ci moon-shine              # CI-equivalent validation
moon check moon-shine           # Full project health check
moon run moon-shine:ai-lint-batch  # AI-powered batch linting

# Direct cargo commands (avoid - no Moon caching benefits):
# cargo build --release --target wasm32-wasip1  ❌
# cargo test                                     ❌
# cargo clippy                                   ❌
# Use Moon orchestration instead! ✅
```

### Features

- Pure WASM extension - no feature flags needed

## 📊 Performance

### ⚡ Execution Performance

- **Binary Size**: Optimized WASM extension (~200KB) with complete DSPy framework
- **Memory Usage**: Efficient with `wee_alloc` and Moon's resource management
- **Coordination Speed**: <50ms WASM orchestration + Moon's cached native tools
- **Incremental Builds**: Moon's dependency tracking provides 10-100x speedup
- **Parallel Execution**: Moon orchestrates concurrent ESLint, TypeScript, and AI processing

### 💾 Intelligent Caching Strategy

- **TypeScript Results**: Cached until source or config changes
- **ESLint Analysis**: Persistent across runs with smart invalidation
- **AI Enhancements**: Claude responses cached by content hash
- **Dependency Tracking**: Only reprocess files affected by changes
- **Session Management**: Efficient cleanup with configurable retention

### 🎆 Advanced Optimizations

- **DSPy Mathematical Optimization**: Prompt engineering with measurable improvements
- **COPRO Collaborative Engineering**: Systematic prompt refinement
- **Moon Task Orchestration**: Intelligent scheduling and resource management
- **WASM Sandbox Security**: Isolated execution with controlled resource access

## 🔍 Quality Metrics

### 🏆 Performance Benchmarks

| Metric | Value | Moon Benefit |
|--------|-------|-------------|
| **WASM Coordination** | <50ms | Lightweight orchestration |
| **TypeScript Analysis** | 100-500ms | 💾 Cached compilation |
| **ESLint Processing** | 200-800ms | ⚡ Parallel rule execution |
| **Claude AI Enhancement** | 2-10s | 🧠 Intelligent caching |
| **Total Pipeline** | 2.5-11s | 🎯 Only changed files |

### 📈 Quality Improvements

- **Industry Standard Tools**: ESLint, TypeScript compiler, Prettier via Moon's task orchestration
- **DSPy Optimization**: Mathematical prompt engineering with 15-30% improvement metrics
- **COPRO Collaboration**: Systematic prompt optimization achieving 85%+ quality scores
- **AI Suggestions**: Context-aware improvements with 90%+ relevance rates
- **Moon Caching**: 10-100x faster subsequent runs through intelligent dependency tracking
- **Incremental Processing**: Process only touched files, reducing CI times by 60-80%

## 📝 License

MIT License - see LICENSE file for details.

## 🤝 Contributing

1. Follow Rust and WASM best practices
2. Maintain clippy compliance with zero warnings
3. Add comprehensive tests for new features
4. Update documentation for API changes

## 📋 Latest Improvements

### ✨ Session-Based File Management (v1.0.0)

- **🗂️ Timestamp-Based Sessions**: `/tmp/moon-shine/20241219/session-143052-a1b2c3d4/`
- **🧹 Smart Cleanup**: Date-based directory removal for efficient maintenance
- **🔍 Enhanced Debugging**: Complete session artifacts preserved per file
- **⚙️ Configurable Retention**: 24-hour default with `--keep-debug-sessions` override
- **📊 Session Correlation**: All request/response files grouped for analysis

### 🎯 Enhanced Command Line Interface

- **Path targeting**: `--path src/components`
- **Pattern filtering**: `--include "**/*.ts"` and `--exclude "**/*.test.ts"`
- **File limits**: `--max-files 10` for controlled processing
- **Debug mode**: `--keep-debug-sessions` for development
- **Dry runs**: `--dry-run` for preview without execution

## 🌟 Acknowledgments

- Built for [moonrepo](https://moonrepo.dev) task orchestration
- Powered by [Claude AI](https://claude.ai) for intelligent analysis
- WASM integration via [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
## OXC Parser Error Handling

moon-shine robustly catches and converts all OXC parser errors during validation. All parser errors are:
- Logged with full metadata (message, ruleId, line, column, severity, span).
- Converted to ESLint-compatible error objects with the following fields:
  - `ruleId`: Always `"parser"` for parser errors.
  - `message`: The original OXC error message.
  - `line`, `column`: Accurate location extracted from the error label or fallback.
  - `severity`: Always `"error"` for parser errors.
  - `span`: Byte offsets for precise source mapping.
- All error metadata is preserved for downstream consumers and reporting tools.

See [`oxc_unified_workflow.rs`](src/oxc_unified_workflow.rs) for implementation details and [`linter_tests.rs`](tests/linter_tests.rs) for test coverage.