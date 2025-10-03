# ✨ Moon Shine - Advanced AI-Powered Linter and Code Improvement Tool

Moon Shine is an experimental WebAssembly (WASM) extension for [moonrepo](https://moonrepo.dev) that uses the full power of the OXC (JavaScript Oxidation Compiler) toolchain to provide production-grade static analysis, complexity analysis, and automated code fixing for TypeScript and JavaScript. It is designed to be a high-performance, semantically-aware alternative to traditional linters and formatters like ESLint and Prettier.

## 🚀 Features

- **AST-Based Analysis**: Full scope and symbol resolution for deep, semantic understanding of code, far beyond what's possible with regex-based heuristics.
- **Automated Code Fixing**: A powerful AST-based auto-fix engine that can resolve a wide range of issues, from type safety improvements to performance optimizations.
- **Comprehensive Complexity Analysis**: In-depth complexity metrics, including cyclomatic and cognitive complexity, Halstead metrics, and maintainability index, to help you identify and refactor complex code.
- **Security Scanning**: Built-in security scanning to detect common vulnerabilities like unsafe `eval` usage, prototype pollution, and hardcoded secrets.
- **DSPy-Inspired Prompt Optimization**: A framework for optimizing AI prompts to improve the quality and accuracy of AI-assisted code analysis and fixing.
- **High-Performance Formatting**: A lightning-fast code formatter that serves as a replacement for Prettier, with full integration into the AST analysis pipeline for single-pass processing.
- **Extensible Rulebase**: A flexible rulebase system that allows you to define custom linting rules and auto-fixes.

## ✅ What Works Today

- **Moon Extension Entrypoint**: Implemented in `src/lib.rs`/`src/extension.rs`, parsing a range of CLI flags (`--mode`, `--lint-only`, `--reporting-only`, `--force-init`, `--install-prompts`) and forwarding positional arguments as file targets.
- **Installation Bootstrap**: The `install_moonshine_extension` function returns a JSON payload with default prompt/training data and Moon task templates, which Moon writes to the host.
- **Configuration Loading**: `MoonShineConfig` loads configuration with a generated JSON schema, allowing `workspace.yml`/`project.yml` to provide settings without the WASM module accessing the disk.
- **Advanced Workflow Engine**: A sophisticated workflow engine in `src/workflow.rs` and `src/engine.rs` that orchestrates complex analysis and fixing tasks, including parallel linting.
- **Embedded Resources**: Prompts, pattern configurations, and DSPy-inspired helpers are embedded for consumption by workflows.

## 🧭 CLI Usage

```
moon ext moon-shine [flags] [files...]
```

Supported flags (see `parse_moon_args` in `src/extension.rs`):

| Flag | Description |
| ---- | ----------- |
| `--mode <value>` | Selects operation mode. Supported values: `fix`, `lint-only`, `reporting-only`, `parallel-lint`. The default is `fix` or whatever `MoonShineConfig.operationMode` provides. |
| `--lint-only` | A shortcut that forces `mode = lint-only`. |
| `--reporting-only` | Reports issues without running installation logic. |
| `--force-init` | Forces regeneration of installation payloads, even if prompts already exist. |
| `--install-prompts` | Triggers the installation flow without running a workflow. |

Any argument that does not start with `--` is treated as a file or directory target.

## 🛠️ Installation & Build

### Environment Setup

First, set up the development environment:

```bash
# Activate proto toolchain (Rust, Moon, etc.)
./setup.sh

# Or manually activate proto in your shell:
eval "$(proto activate --shell bash)"
```

The project uses [proto](https://moonrepo.dev/docs/proto) for tool management. Tools are configured in `.prototools`:
- Rust 1.89.0
- Moon 1.40.4
- Node 20.0.0

### Build Commands

Moon repo tasks defined in `moon.yml` wrap the usual Cargo commands:

```bash
moon run moon-shine:build       # cargo build --target wasm32-wasip1 --release
moon run moon-shine:test        # cargo test --all --release
moon run moon-shine:lint        # cargo clippy with strict settings
moon run moon-shine:type-check  # cargo check --target wasm32-wasip1
moon run moon-shine:format      # dprint fmt
```

You can still call Cargo directly if needed, but you will lose Moon's caching benefits.

### Prerequisites

```bash
rustup target add wasm32-wasip1
```

## 🧩 Configuration

`MoonShineConfig` is exposed to Moon via the `create_config_schema` helper. You can add the extension to `.moon/workspace.yml` or a project file and set fields such as:

- `aiModel`, `enableCoproOptimization`, and other AI-related toggles
- `enableSemanticAnalysis`, `enableTypeChecking`, `enablePerformanceFixes`, `enableSecurityFixes`
- `includePatterns` / `excludePatterns`
- `moonTaskName` or `moonTaskMapping`
- `format_config` for the built-in Prettier replacement

## 🗂️ Installation Payload Structure

Calling the extension with `--install-prompts` (or running it for the first time) produces a payload similar to:

```json
{
  "action": "install_moonshine_extension",
  "moonshine_dir": ".moon/moonshine",
  "initial_files": {
    "prompts.json": { ... },
    "training.json": { ... },
    "config.json": { ... }
  },
  "task_templates": {
    "shine": { ... },
    "shine-lint": { ... }
  }
}
```

Moon is responsible for materializing these artifacts on disk.

## 🧪 Development Notes

- Unit tests cover serialization helpers, configuration defaults, and core functionality.
- The DSPy-inspired modules under `src/dspy/` provide macros and utilities that are exercised in tests and examples.
- When evolving the project, prefer adding executable code first, then refreshing the README so it continues to mirror the repository state.

## 📄 License

MIT License – see `LICENSE`.

## 🙌 Contributing

1. Keep documentation in sync with behavior; the code is treated as the source of truth.
2. Add tests when enabling new workflow steps.
3. Update or add TODOs in code if you adopt ideas from older documentation that are not yet implemented.

## 🧩 Rulebase

This build ships with a comprehensive set of compiled rule definitions derived from `rulebase/output/moonshine-rulebase-complete.json`. They are embedded via the `embedded_rulebase` feature and exposed through `rulebase::iter_builtin_rules()`.
