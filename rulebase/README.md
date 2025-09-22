# MoonShine Rulebase

Organized structure for MoonShine's 800+ rules using static analysis, behavioral analysis, and hybrid approaches.

## Directory Structure

```
rulebase/
├── definitions/          # Rule definition files by analysis type
│   ├── static-security-rules.json     # Security rules (no-eval, detect-sql-injection, etc.)
│   ├── static-performance-rules.json  # Performance rules (prefer-const, object-shorthand, etc.)
│   ├── static-codequality-rules.json  # Code quality rules (no-unused-vars, eqeqeq, etc.)
│   ├── behavioral-analysis-rules.json # AI behavioral analysis rules
│   └── hybrid-analysis-rules.json     # Hybrid static+AI rules and special rules
├── schemas/              # JSON schemas for validation
│   └── rule-schema.json  # Rule definition validation schema
├── generators/           # Rule generation scripts
│   └── generate-full-rulebase.js    # Generates complete rulebase from definitions
├── examples/            # Configuration examples
│   └── eslint-moonshine-example.config.js  # ESLint config example
└── output/              # Generated files
    └── moonshine-rulebase-complete.json     # Complete generated rulebase
```

## Usage

### Generate Complete Rulebase

```bash
cd rulebase
node generators/generate-full-rulebase.js
```

This reads the organized definition files and generates the complete 832-rule rulebase with:
- 582 Static analysis rules (ESLint-compatible)
- 200 Behavioral analysis rules
- 50 Hybrid analysis rules

### Rule Categories

**Static Analysis Rules (ESLint-compatible)**:
- Security: `no-eval`, `detect-sql-injection`, `no-hardcoded-secrets`
- Performance: `prefer-const`, `object-shorthand`, `prefer-template`
- Code Quality: `no-unused-vars`, `eqeqeq`, `no-console`

**Behavioral Analysis Rules**:
- `@moonshine/aibehavioral-analysis-*` - Behavioral pattern detection
- `@moonshine/cognitive-analysis-*` - Cognitive complexity analysis
- `@moonshine/patterns-analysis-*` - Design pattern detection
- `@moonshine/architecture-analysis-*` - Architectural analysis

**Hybrid Analysis Rules**:
- `@moonshine/hybrid-*` - Combined static+AI rules
- `@moonshine/excellence-*` - Production excellence rules
- Special rules: `@moonshine/ai-code-quality-oracle`, `@moonshine/production-excellence-suite`

### Adding New Rules

1. Edit the appropriate definition file in `definitions/`
2. Validate against `schemas/rule-schema.json`
3. Regenerate the complete rulebase
4. The dynamic rule loader will automatically pick up changes

## Integration

The rulebase integrates with:
- **Dynamic Rule Loader** (`src/rulebase/dynamic_rule_loader.rs`) - Loads rules into KV storage
- **ESLint Configuration** - ESLint-compatible rule names for seamless integration
- **Moon Tasks** - Native tool execution via Moon's task system
- **WASM Extension** - Lightweight coordination and rule orchestration