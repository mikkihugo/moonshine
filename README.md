# ✨ Moon Shine - Moon Extension Scaffold

Moon Shine is an experimental WebAssembly (WASM) extension for [moonrepo](https://moonrepo.dev). The current codebase focuses on wiring up configuration, installation payloads, and logging so that future AI-assisted workflows can plug into Moon's task runner. The README intentionally tracks the behavior that actually ships in this repository.

## ✅ What Works Today

- **Moon extension entrypoint** implemented in `src/lib.rs`/`src/extension.rs` that parses a limited set of CLI flags (`--mode`, `--lint-only`, `--reporting-only`, `--force-init`, `--install-prompts`) and forwards positional arguments as file targets.
- **Installation bootstrap** (`install_moonshine_extension`) that returns a JSON payload containing default prompt/training data and Moon task templates; Moon is expected to write the files on the host.
- **Configuration loading** through `MoonShineConfig` with a generated JSON schema so `workspace.yml`/`project.yml` can supply settings without the WASM module touching disk.
- **Workflow scaffolding** in `src/workflow.rs` that logs the requested operation mode (including a stub `parallel-lint` mode) and is home to the current orchestrator logic for future Moon task execution.
- **Embedded resources** for prompts, pattern configuration, and experimental DSPy-inspired helpers that can be consumed by future workflows.

## 🚧 Not Implemented Yet

The code does **not** currently execute ESLint, TypeScript, Prettier, Claude, or other native tools. All heavy lifting steps are placeholders that only log intent. Features such as petgraph-based DAG execution, cost-aware AI routing, or JSON responses in `/tmp/moon-shine/...` directories are aspirational and should be treated as TODOs.

If you need those behaviors, follow the existing TODO comments in the source code or expand the scaffolding modules (`workflow`, `cost_aware_ai_orchestrator`, `moon_pdk_interface`, etc.).

## 🧭 CLI Usage

```
moon ext moon-shine [flags] [files...]
```

Supported flags (see `parse_moon_args` in `src/extension.rs`):

| Flag | Description |
| ---- | ----------- |
| `--mode <value>` | Selects operation mode. Supported values today: `fix`, `lint-only`, `reporting-only`, `parallel-lint`. The default is `fix` or whatever `MoonShineConfig.operationMode` provides. |
| `--lint-only` | Shortcut that forces `mode = lint-only`. |
| `--reporting-only` | Reports issues without running installation logic. |
| `--force-init` | Forces regeneration of installation payloads even if prompts already exist. |
| `--install-prompts` | Triggers the installation flow without running a workflow. |

Any argument that does not start with `--` is treated as a file or directory target. In `parallel-lint` mode the extension still emits stub metrics and does not accept extra flags such as `--metrics-file` yet.

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
- `includePatterns` / `excludePatterns`
- `moonTaskName` or `moonTaskMapping`

The current implementation reads these values for logging and future expansion but does not yet change runtime behavior beyond selecting default strings.

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

- Unit tests cover serialization helpers and configuration defaults (`src/extension.rs`, `src/config.rs`).
- The DSPy-inspired modules under `src/dspy/` provide macros/utilities that are currently only exercised in tests/examples.
- Many modules (e.g., `tool_replacements`, `workflow`, `sunlinter_integration`) are scaffolding and may contain TODO comments for future implementation.

When evolving the project, prefer adding executable code first, then refreshing the README so it continues to mirror the repository state.

## 📄 License

MIT License – see `LICENSE`.

## 🙌 Contributing

1. Keep documentation in sync with behavior; the code is treated as the source of truth.
2. Add tests when enabling new workflow steps.
3. Update or add TODOs in code if you adopt ideas from older documentation that are not yet implemented.
## 🧩 Rulebase

This build ships with 832 compiled rule definitions (582 static, 200 behavioral, 50 hybrid) derived from `rulebase/output/moonshine-rulebase-complete.json`. They are embedded via the `embedded_rulebase` feature and exposed through `rulebase::iter_builtin_rules()`.
