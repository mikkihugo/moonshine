# Moon Shine Extension Migration Guide

This guide helps you migrate existing projects to use the Moon Shine AI-powered linting extension with Moon workspaces.

## Overview

Moon Shine is a WebAssembly (WASM) extension for Moon that provides:
- AI-powered TypeScript/JavaScript linting and code fixes
- COPRO (Collaborative Prompt Optimization) for improved AI effectiveness
- Intelligent provider routing (Claude, Gemini, OpenAI)
- Pattern learning and adaptive rule generation
- Integration with existing Moon task orchestration

## Prerequisites

### Required Software

1. **Moon CLI** (v1.0.0 or later)
   ```bash
   # Install Moon
   curl -fsSL https://moonrepo.dev/install/proto.sh | bash
   proto install moon
   ```

2. **Rust** (1.80+ for building from source)
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   ```

3. **Node.js** (for TypeScript/JavaScript projects)
   ```bash
   # Install Node.js via proto
   proto install node 20
   ```

### Optional Tools

- **binaryen** (for WASM optimization)
  ```bash
  # macOS
  brew install binaryen

  # Ubuntu/Debian
  sudo apt install binaryen

  # Windows
  # Download from https://github.com/WebAssembly/binaryen/releases
  ```

## Installation

### Method 1: Quick Install (Recommended)

```bash
# Download and run the installation script
curl -fsSL https://raw.githubusercontent.com/zenflow/moon-shine/main/scripts/install-extension.sh | bash

# Or download and run manually
wget https://raw.githubusercontent.com/zenflow/moon-shine/main/scripts/install-extension.sh
chmod +x install-extension.sh
./install-extension.sh
```

### Method 2: Manual Installation

1. **Download Extension**
   ```bash
   # Create extension directory
   mkdir -p ~/.moon/extensions/moon-shine

   # Download WASM and manifest
   curl -L https://github.com/zenflow/moon-shine/releases/latest/download/moon-shine.wasm \
     -o ~/.moon/extensions/moon-shine/moon-shine.wasm
   curl -L https://github.com/zenflow/moon-shine/releases/latest/download/manifest.json \
     -o ~/.moon/extensions/moon-shine/manifest.json
   ```

2. **Register Extension**
   ```bash
   moon ext install moon-shine
   ```

### Method 3: Build from Source

```bash
# Clone repository
git clone https://github.com/zenflow/moon-shine.git
cd moon-shine

# Build and install
./scripts/build-extension.sh
./scripts/install-extension.sh
```

## Migration Scenarios

### Scenario 1: Existing Moon Workspace

If you already have a Moon workspace:

1. **Install Extension** (see above)

2. **Update Project Configuration**

   Add to your `moon.yml`:
   ```yaml
   # AI-powered linting tasks
   tasks:
     # Main AI linting with fixes
     shine:
       command: 'moon'
       args: ['ext', 'run', 'moon-shine', '--']
       inputs: ['**/*.{ts,tsx,js,jsx}']
       options:
         cache: false
         runFromWorkspaceRoot: true
         affectedFiles: true

     # Lint-only mode (no AI fixes)
     shine-lint:
       command: 'moon'
       args: ['ext', 'run', 'moon-shine', '--', '--lint-only']
       inputs: ['**/*.{ts,tsx,js,jsx}']
       options:
         cache: true
         runFromWorkspaceRoot: true

     # CI reporting mode
     shine-report:
       command: 'moon'
       args: ['ext', 'run', 'moon-shine', '--', '--reporting-only']
       inputs: ['**/*.{ts,tsx,js,jsx}']
       options:
         cache: true
         runFromWorkspaceRoot: true
   ```

3. **Configure Extension** (optional)

   Add to your workspace `.moon/workspace.yml`:
   ```yaml
   extensions:
     moon-shine:
       # AI model preference
       ai:
         ai_model: "sonnet"  # or "haiku", "opus"
         ai_providers: ["claude"]
         max_files_per_task: 50

       # Operation mode
       operation_mode: "fix"  # or "lint-only", "reporting-only"

       # COPRO optimization
       enable_copro_optimization: true
       copro_breadth: 5
       copro_depth: 3
       copro_temperature: 1.0

       # Performance settings
       temperature: 0.7
       max_tokens: 4096

       # Custom prompts (optional)
       custom_prompts:
         typescript_strict: "Custom TypeScript prompt..."
   ```

### Scenario 2: Non-Moon JavaScript/TypeScript Project

If you have an existing project without Moon:

1. **Initialize Moon Workspace**
   ```bash
   # In your project root
   moon init
   ```

2. **Configure Project**

   Create `moon.yml`:
   ```yaml
   $schema: 'https://moonrepo.dev/schemas/project.json'

   type: 'application'  # or 'library'
   language: 'typescript'  # or 'javascript'

   # Existing build tasks
   tasks:
     build:
       command: 'npm'
       args: ['run', 'build']
       inputs: ['src/**/*', 'package.json', 'tsconfig.json']
       outputs: ['dist/**/*']

     test:
       command: 'npm'
       args: ['run', 'test']
       inputs: ['src/**/*', 'tests/**/*']

     # Add Moon Shine tasks
     shine:
       command: 'moon'
       args: ['ext', 'run', 'moon-shine', '--']
       inputs: ['src/**/*.{ts,tsx,js,jsx}']
   ```

3. **Install Extension** (see installation methods above)

### Scenario 3: Migrating from ESLint/Prettier

If you're currently using ESLint and Prettier:

1. **Keep Existing Configuration** (recommended)

   Moon Shine works alongside existing tools:
   ```yaml
   tasks:
     # Traditional linting
     eslint:
       command: 'npx'
       args: ['eslint', 'src/**/*.{ts,tsx,js,jsx}']

     prettier:
       command: 'npx'
       args: ['prettier', '--write', 'src/**/*.{ts,tsx,js,jsx}']

     # AI-enhanced linting
     shine:
       command: 'moon'
       args: ['ext', 'run', 'moon-shine', '--']
       inputs: ['src/**/*.{ts,tsx,js,jsx}']
       deps: ['eslint', 'prettier']  # Run after traditional tools
   ```

2. **Gradual Migration**

   Start with reporting mode to evaluate AI suggestions:
   ```bash
   # Test AI linting without changes
   moon run shine-report src/

   # Apply AI fixes to specific files
   moon run shine src/components/Button.tsx

   # Full project AI linting
   moon run shine src/
   ```

## Configuration Reference

### Extension Configuration

The extension can be configured in `.moon/workspace.yml` under the `extensions.moon-shine` key:

```yaml
extensions:
  moon-shine:
    # Core AI settings
    ai:
      ai_model: "sonnet"                    # AI model (sonnet, haiku, opus)
      ai_providers: ["claude"]              # Available providers
      max_files_per_task: 50               # Batch size

    # Operation mode
    operation_mode: "fix"                   # fix, lint-only, reporting-only

    # COPRO optimization
    enable_copro_optimization: true         # Enable prompt optimization
    copro_breadth: 5                       # Optimization breadth
    copro_depth: 3                         # Optimization depth
    copro_temperature: 1.0                 # Creativity level

    # AI parameters
    temperature: 0.7                       # AI creativity (0.0-2.0)
    max_tokens: 4096                       # Max tokens per request

    # Performance
    max_files_per_task: 50                 # Files per batch
    workflow_timeout_seconds: 300          # Task timeout

    # Custom prompts
    custom_prompts:
      typescript_strict: |
        You are a TypeScript expert. Focus on:
        - Type safety and strict typing
        - Performance optimizations
        - Modern ES2022+ features
```

### Task Configuration

Common task configurations:

```yaml
tasks:
  # Development mode (full AI analysis)
  shine-dev:
    command: 'moon'
    args: ['ext', 'run', 'moon-shine', '--', '--mode', 'fix']
    inputs: ['src/**/*.{ts,tsx,js,jsx}']
    options:
      cache: false
      runFromWorkspaceRoot: true

  # CI mode (reporting only)
  shine-ci:
    command: 'moon'
    args: ['ext', 'run', 'moon-shine', '--', '--reporting-only']
    inputs: ['src/**/*.{ts,tsx,js,jsx}']
    options:
      cache: true
      runFromWorkspaceRoot: true
      runInCI: true

  # Pre-commit hook
  shine-staged:
    command: 'moon'
    args: ['ext', 'run', 'moon-shine', '--', '--lint-only']
    inputs: ['src/**/*.{ts,tsx,js,jsx}']
    options:
      cache: true
      affectedFiles: true
```

## Usage Examples

### Basic Usage

```bash
# Run AI linting on entire project
moon run shine

# Lint specific files
moon run shine src/components/

# Reporting mode (no changes)
moon run shine-report src/

# Lint-only mode (no AI fixes)
moon run shine-lint src/
```

### Advanced Usage

```bash
# Force initialization
moon run shine -- --force-init

# Custom mode
moon run shine -- --mode custom-analysis

# Install custom prompts
moon run shine -- --install-prompts
```

### CI/CD Integration

```yaml
# GitHub Actions example
- name: AI Code Quality Check
  run: moon run shine-ci src/

# GitLab CI example
code_quality:
  script:
    - moon run shine-ci src/
  artifacts:
    reports:
      codequality: shine-report.json
```

## Troubleshooting

### Common Issues

1. **Extension Not Found**
   ```bash
   # Check installation
   moon ext list | grep moon-shine

   # Reinstall if missing
   ./scripts/install-extension.sh --force
   ```

2. **WASM Loading Errors**
   ```bash
   # Check WASM file
   file ~/.moon/extensions/moon-shine/moon-shine.wasm

   # Verify permissions
   ls -la ~/.moon/extensions/moon-shine/
   ```

3. **Memory Issues**
   ```bash
   # Reduce batch size
   moon run shine -- --mode lint-only  # Less memory intensive
   ```

4. **AI Provider Issues**
   ```bash
   # Check Claude CLI installation
   which claude

   # Test connection
   claude --help
   ```

### Performance Optimization

1. **Reduce Scope**
   ```yaml
   # Process only changed files
   tasks:
     shine:
       options:
         affectedFiles: true
   ```

2. **Use Caching**
   ```yaml
   # Enable caching for reporting mode
   tasks:
     shine-report:
       options:
         cache: true
   ```

3. **Parallel Processing**
   ```yaml
   # Configure in workspace
   extensions:
     moon-shine:
       max_files_per_task: 25  # Smaller batches
       workflow_timeout_seconds: 600  # Longer timeout
   ```

## Migration Checklist

- [ ] Moon CLI installed and configured
- [ ] Extension installed via script or manual method
- [ ] Project `moon.yml` updated with shine tasks
- [ ] Workspace configuration added (optional)
- [ ] Test run with `moon run shine-report`
- [ ] CI/CD pipeline updated (if applicable)
- [ ] Team training on new workflow
- [ ] Documentation updated for project

## Support and Resources

- **Documentation**: [GitHub README](https://github.com/zenflow/moon-shine/blob/main/README.md)
- **Issues**: [GitHub Issues](https://github.com/zenflow/moon-shine/issues)
- **Moon Documentation**: [moonrepo.dev](https://moonrepo.dev)
- **Examples**: [examples/](https://github.com/zenflow/moon-shine/tree/main/examples)

## Next Steps

After successful migration:

1. **Customize Configuration**: Adjust AI settings for your team's needs
2. **Create Custom Prompts**: Add project-specific linting rules
3. **Monitor Performance**: Track improvement in code quality
4. **Train Team**: Share best practices for AI-assisted development
5. **Iterate**: Refine configuration based on usage patterns

The Moon Shine extension evolves with your codebase, learning patterns and improving suggestions over time. Regular usage will result in more accurate and helpful AI-powered code improvements.