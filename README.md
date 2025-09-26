# ✨ Moon Shine

Moon Shine is a production-grade AI agent workflow for [moonrepo](https://moonrepo.dev) projects. The
extension runs as a WebAssembly (WASM) module inside Moon, delegating heavyweight work to native Moon
tasks while coordinating multi-agent analysis, code generation, and formatting passes. The current
release focuses on TypeScript and JavaScript, but the architecture is designed to extend across
languages and provider stacks.

---

## 📖 Overview

- **Hybrid AI pipeline** – WASM orchestrates intelligent workflows while Moon tasks run TypeScript,
  ESLint, Prettier, and Claude CLI tooling.
- **DAG-based workflow engine** – `src/workflow.rs` models end-to-end analysis as a petgraph DAG with
  configurable phases, conditional steps, and retries.
- **Agent federation** – Specialized agents handle static pattern analysis, TypeScript checking,
  Claude-powered refinement, and optional strict-mode validation. See [`agents.md`](./agents.md).
- **Session-based coordination** – Each run exchanges JSON payloads via structured session directories
  under `/tmp/moon-shine/<date>/session-*/`.
- **Cost-aware AI** – Adaptive heuristics decide when to invoke Claude versus relying on static
  heuristics to manage latency and spend.

Moon Shine ships with an embedded rulebase (582 static, 192 behavioral, 50 hybrid rules) plus
AI-enhanced behavioral detectors defined under `src/oxc_adapter/`.

---

## 🚀 Quick Start

```bash
./setup.sh                         # Install proto toolchain (Moon, Rust, Node)
moon run moon-shine:build          # Compile WASM bundle with caching
moon run moon-shine:test           # Execute unit & integration tests
moon ext moon-shine src/           # Run the extension on your workspace
```

Core CLI flags (parsed in `src/extension.rs`):

- `--mode <fix|lint-only|reporting-only|parallel-lint>` – Select the workflow profile. Defaults to
  `fix` unless overridden in configuration.
- `--lint-only`, `--reporting-only` – Convenience shorthands for the matching mode.
- `--force-init`, `--install-prompts` – Refresh provisioning payloads that Moon materializes on disk.

When `--mode parallel-lint` is selected the WASM module emits JSON metrics; all other heavy work
remains inside Moon tasks.

---

## 🧠 Architecture

```text
┌─────────────────┐    ┌──────────────────┐    ┌──────────────────┐
│ WASM Coordinator│───▶│ Workflow Engine  │───▶│ Moon Task Agents │
│ (Extism + Moon) │    │ (petgraph DAG)   │    │ (tsc/eslint/etc) │
└─────────────────┘    └──────────────────┘    └──────────────────┘
        │                        │                        │
        ▼                        ▼                        ▼
┌─────────────────┐    ┌──────────────────┐    ┌──────────────────┐
│ Static Analysis │    │ Result Aggregator│    │ Claude AI Agent │
│ Pattern Pass    │    │ Cost Heuristics  │    │ Strict TS Agent │
└─────────────────┘    └──────────────────┘    └──────────────────┘
```

### Workflow Phases

1. **Foundation** – Session setup, language detection, OXC parsing, and semantic analysis.
2. **Analysis** – OXC rule execution and behavioral detectors (parallelized where dependencies allow).
3. **Enhancement** – Claude AI guidance, code generation, formatting, and optional strict TypeScript
   validation.
4. **Aggregation** – Collect agent responses, merge diffs, score quality metrics, and clean sessions
   (configurable).

The workflow engine performs topological scheduling, per-step timeouts, exponential backoff retries,
and skip logic when prerequisites fail.

---

## ⚠️ Implementation Status

Moon Shine's codebase contains comprehensive scaffolding that needs integration work before production deployment. Key areas to complete:

### **Critical Infrastructure**

- **Moon PDK Integration**: `src/moon_pdk_interface.rs` returns mock results for host operations (`execute_command`, `read_file_content`, `write_file_to_host`). Replace with real Moon PDK bindings.
- **Workflow Engine Activation**: `src/workflow.rs` implements the petgraph DAG engine but is commented out in `src/lib.rs` exports. Enable and integrate the workflow engine.
- **Extension Execution Pipeline**: `src/extension.rs` prepares workflow requests but never calls Moon tasks or workflow engine. Wire the entrypoint to real executors.

### **Tool Integration Strategy**

- **OXC Integration**: OXC should be called as external CLI commands via Moon PDK adapters, not embedded as Rust libraries. Create adapters that invoke `oxc` CLI for parsing, linting, and formatting.
- **Tool Coordination**: All tools (TypeScript, ESLint, Prettier, Claude CLI) should use the adapter pattern via `execute_command()` rather than Moon tasks for lightweight coordination.
- **Provider Router**: `src/provider_router/` requires actual CLI binaries (`claude`, `gemini`, `codex`) and configuration plumbing before dispatching AI calls.

### **Configuration and Analysis**

- **TSDoc Integration**: TSDoc analysis references `tsdoc.json` settings but uses placeholder data paths. Wire real config before enabling by default.
- **Session Management**: Session-based JSON protocol is designed but needs full implementation of directory creation, cleanup, and agent coordination.
- **AI Behavioral Analysis**: `src/oxc_adapter/ai_behavioral.rs` and neural pattern models are stubbed and need full implementation.

### **System Integration**

- **Rule Registry**: `src/rule_registry.rs` exists but needs connection to workflow engine and execution pipeline for 582+ static and 192 behavioral rules.
- **Error Handling**: Need robust error handling for Moon task failures, AI provider timeouts, and graceful degradation.
- **Testing**: `src/testing/` has comprehensive structure but needs integration tests for Moon task execution, AI provider routing, and end-to-end workflows.

This documentation reflects the current state and provides a clear roadmap for production readiness.

---

## 🤖 Agent Catalog

- **WASM Coordination** (`src/lib.rs::ai_lint_file`) – Entry point for language detection, session
  orchestration, and host-side Moon PDK execution.
- **Static Analysis** (`src/linter.rs::static_analysis_with_path`) – Rule-aware analyzer that
  surfaces contextual suggestions per file path.
- **AI Provider Router** (Moon PDK `execute_command`) – Dispatches to Claude, Gemini, or other
  configured providers for holistic fixes, lint harmonization, and documentation improvements.
- **TypeScript Semantics via OXC** – Uses OXC’s TypeScript parsing and semantic analysis. Future
  strictness toggles will be derived from project configuration (e.g., `tsconfig.json`) when host-side
  TypeScript integration is implemented.

See [`agents.md`](./agents.md) for the full integration guide, including JSON contract examples and
debugging tips.

---

## 🔄 Agent Communication Protocol

Each processed file produces a session directory containing the request, intermediate tool output,
and final AI results:

```text
/tmp/moon-shine/<date>/session-<uuid>/
├── request.json
├── typescript-response.json
├── eslint-response.json
├── prettier-response.json
├── tsdoc-response.json
└── claude-response.json
```

Requests encapsulate file content, WASM analysis summaries, quality scores, and agent enablement
toggles. Responses echo the `request_id`, identify the task (`claude-json`, `eslint-native`, …), and
supply structured results plus timing metadata. Session retention is configurable via
`MoonShineConfig::keep_debug_sessions`.

---

## ⚙️ Configuration

`MoonShineConfig` (deserialized through Moon’s config schema) controls workflow depth, AI budget
heuristics, and session behavior:

```rust
pub struct MoonShineConfig {
    pub max_iterations: u32,
    pub quality_threshold: f64,
    pub enable_claude_ai: bool,
    pub enable_parallel_execution: bool,
    pub timeout_per_step_ms: u64,
    pub retry_failed_steps: bool,
    pub keep_debug_sessions: bool,
    // ... provider and COPRO options elided
}
```

Runtime AI behaviour is tuned through `AiLinterConfig`, enabling or disabling specific agents and
defining session retention for debugging. Configuration lives in your Moon `workspace.yml` or
`project.yml`; Moon validates it using the schema emitted from `create_config_schema()`. Defaults are
designed for practical operations (e.g., keep debug sessions for 12 hours, clean up stale sessions
after 48 hours). Strict TypeScript semantics flow from your project’s `tsconfig.json`, while TSDoc
analysis will respect your TSDoc configuration (`tsdoc.json`) when the host integration is complete.

---

## 📏 Code Quality & Naming Conventions

**Moon Shine follows Google TypeScript naming conventions adapted for Rust** to ensure clarity.
This approach emphasizes descriptive, fully qualified names that make code intent immediately obvious.

### **Naming Philosophy**

- **Self-Documenting Names**: `LanguageModelUsageMetrics` instead of `LmUsage`
- **Full Descriptive Terms**: `MultiLanguageAnalysisResult` instead of `Result`
- **Explicit Purpose**: `TypeScriptSemanticAnalyzer` instead of `TSAnalyzer`
- **Behavioral Clarity**: `RepetitivePatternLearner` instead of `AdaptiveAnalyzer`

### **Key Renamed Components**

- `LmUsage` → `LanguageModelUsageMetrics` - Token usage tracking with comprehensive metadata
- `CodePatternDetector` → Maintains descriptive naming for StarCoder-1B integration
- `MultiLanguageAnalyzer` → Unified analysis system for TypeScript/JavaScript and Rust
- `RepetitivePatternLearner` → AI-powered pattern detection for custom rule generation

### **Benefits**

- **Onboarding Speed**: New contributors immediately understand component purposes
- **IDE Experience**: IntelliSense provides clear context for every symbol
- **Code Review Quality**: Reviewers can focus on logic rather than deciphering abbreviated names
- **Documentation Alignment**: Code structure mirrors architectural documentation

This convention applies throughout the codebase - from high-level workflow coordinators to low-level diagnostic structures.

---

## 🧪 Development & Testing

- `moon run moon-shine:build` – WASM build (wraps `cargo build --target wasm32-wasip1`).
- `moon run moon-shine:test` – Runs crate tests with Moon’s caching.
- `moon run moon-shine:lint` – Executes strict `cargo clippy` configuration.
- `moon run moon-shine:type-check` – Performs `cargo check` for the WASM target.
- `moon ext moon-shine --keep-debug-sessions src/` – Run analysis while retaining session artifacts.
- `moon ext moon-shine --dry-run src/` – Inspect planned workflow steps without executing external
  agents.

Prefer Moon tasks over direct `cargo` or `tsc` invocation—Moon provides dependency-aware caching and
environment consistency.

---

## 🛠️ Debugging & Monitoring

- Inspect session directories under `/tmp/moon-shine/<date>/session-*` for agent I/O.
- Use `moon query projects --id moon-shine --json` to verify task registration.
- Run `moon sync moon-shine` to ensure task metadata matches your workspace configuration.
- Provider routing telemetry and rule execution stats are exported via `TelemetryCollector` in
  `src/telemetry.rs`.

---

## 📚 Additional Resources

- [`agents.md`](./agents.md) – Deep dive on agent orchestration, workflow DAGs, and protocol examples.
- `src/oxc_adapter/` – High-performance OXC integration, adaptive pattern analysis, and behavioral AI
  detectors.
- `rulebase/` – Embedded rule definitions bundled with the WASM module.
- `tests/` – Snapshot, property, and integration tests covering workflow scenarios.

---

## 🤝 Contributing

1. Keep documentation synchronized with executable behaviour.
2. Extend coverage when enabling new workflow phases or AI providers.
3. Follow the existing JSON protocol when adding Moon tasks or agents.

Moon Shine is distributed under the MIT License. See `LICENSE` for details.
