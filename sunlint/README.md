# ☀️ Sun Lint - Universal Coding Standards

**Multi-rule, Multi-language Code Quality & Security Analysis Tool with ESLint Integration**

## 🎯 **Overview**

Sun Lint is a universal coding standards checker providing comprehensive code quality and security analysis. Built by Sun* Engineering Team with integrated security rules from OWASP and industry best practices.

### **✨ Key Features**
- ✅ **256+ Coding Rules**: Quality (161), Security (70), Performance (25)
- ✅ **Unified Architecture**: Same adapter pattern for CLI and VSCode extension
- ✅ **Multi-Engine Support**: Heuristic v4.0 (244 rules) + ESLint (17 rules) + AI (256 rules)
- ✅ **Performance Optimized**: Auto file limits, memory management, GitHub Actions ready
- ✅ **Built-in AST Analysis**: JavaScript/TypeScript parsing out of the box
- ✅ **Git Integration**: `--changed-files`, `--staged-files`, `--pr-mode`
- ✅ **TypeScript Support**: Native TypeScript 5.8+ analysis with smart memory limits
- ✅ **Zero Config**: Works immediately after `npm install`
- ✅ **CI/CD Ready**: Baseline comparison, fail-on-new-violations, timeout protection
- ✅ **Advanced File Targeting**: Include/exclude patterns, language filtering

### **🏗️ Architecture**

SunLint uses a unified adapter pattern ensuring consistency between CLI and VSCode extension:

```
┌─────────────────┬─────────────────┐
│   CLI Tools     │ VSCode Extension│
├─────────────────┼─────────────────┤
│ SunlintRule     │ RuleReader      │
│ Adapter         │ Service         │
├─────────────────┴─────────────────┤
│     Rule Sources & Engines        │
│ • Registry (auto-generated)       │
│ • Origin Rules (markdown)         │
│ • Heuristic Engine (244 rules)    │
│ • ESLint Engine (17 rules)        │
│ • OpenAI Engine (256 rules)       │
└───────────────────────────────────┘
```

**Benefits:**
- **No Rule Model Duplication**: Single source of truth
- **Extensible**: Easy to add new engines or rule sources
- **Performance**: 0.07ms average per rule query
- **AI Integration**: Consistent OpenAI context from origin-rules

### **🚀 Quick Start**
```bash
# Install
npm install -g @sun-asterisk/sunlint

# Basic usage - works immediately!
sunlint --all
sunlint --rules=C019,C006

# With input specification  
sunlint --all --input=src
sunlint --rules=C019,C006 --input=src
sunlint --quality --input=src
sunlint --security --input=src

# ESLint integration (requires eslint dependency)
sunlint --rules=C010,C006 --eslint-integration --input=src

# Git integration  
sunlint --all --changed-files
```

## 📦 **Installation**

### **Global Installation (Recommended)**
```bash
npm install -g @sun-asterisk/sunlint
sunlint --version
```

### **Project Installation**
```bash
npm install --save-dev @sun-asterisk/sunlint
```

**✅ Works immediately** with JavaScript analysis using built-in AST parsers (`@babel/parser`, `espree`)

### **Enhanced TypeScript Support**
For advanced TypeScript analysis with ESLint integration:

```bash
npm install --save-dev @sun-asterisk/sunlint eslint @typescript-eslint/parser @typescript-eslint/eslint-plugin typescript
```

### **Full ESLint Integration Support**
For complete ESLint integration with import analysis:

```bash
npm install --save-dev @sun-asterisk/sunlint eslint @typescript-eslint/parser @typescript-eslint/eslint-plugin eslint-plugin-import typescript
```

### **What's Included by Default**
- ✅ **JavaScript Analysis**: High-accuracy AST analysis out of the box
- ✅ **Basic TypeScript**: Works with built-in Babel parser  
- ✅ **256+ Rules**: All quality and security rules available
- ✅ **Heuristic Engine**: Pattern-based analysis for all languages

### **Optional Dependencies (Install as needed)**
```bash
# For ESLint engine integration
npm install eslint --save-dev

# For import/module analysis (recommended with ESLint)
npm install eslint-plugin-import --save-dev

# For enhanced TypeScript analysis  
npm install @typescript-eslint/parser @typescript-eslint/eslint-plugin --save-dev

# For TypeScript compiler integration
npm install typescript --save-dev

# For import/module analysis (recommended)
npm install eslint-plugin-import --save-dev
```

**Quick setup for TypeScript projects:**
```bash
npm install --save-dev @sun-asterisk/sunlint eslint @typescript-eslint/parser @typescript-eslint/eslint-plugin eslint-plugin-import typescript
```

> 💡 **Note**: SunLint gracefully handles missing dependencies. Install only what your project needs. See [docs/DEPENDENCIES.md](docs/DEPENDENCIES.md) for detailed guidance.

# Package.json scripts
```json
{
  "scripts": {
    "lint": "sunlint --all --input=src",
    "lint:changed": "sunlint --all --changed-files",
    "lint:typescript": "sunlint --all --input=src",
    "lint:eslint-integration": "sunlint --all --eslint-integration --input=src"
  },
  "devDependencies": {
    "@sun-asterisk/sunlint": "^1.2.0"
  }
}
```

**For TypeScript projects, add:**
```json
{
  "devDependencies": {
    "@sun-asterisk/sunlint": "^1.2.0",
    "eslint": "^8.50.0",
    "@typescript-eslint/parser": "^7.2.0",
    "@typescript-eslint/eslint-plugin": "^7.18.0",
    "eslint-plugin-import": "^2.32.0",
    "typescript": "^5.0.0"
  }
}
```

## 🔗 **Multi-Engine Architecture**

SunLint automatically selects the best engine for each rule, providing comprehensive coverage:

### **Engine Coverage & Performance**
```bash
# Show engine compatibility for specific rules
sunlint --show-engines --rules=C010,R001,S005

# Use specific engine
sunlint --engine=heuristic --rules=C010,C020 --input=src
sunlint --engine=eslint --rules=R001,R006 --input=src  
sunlint --engine=openai --rules=C010,S001 --input=src
```

**Engine Stats:**
- **Heuristic Engine**: 244/256 rules (95.3%) - Fast, universal
- **ESLint Engine**: 17/256 rules (6.6%) - JavaScript/TypeScript focused
- **OpenAI Engine**: 256/256 rules (100%) - Context-aware analysis

### **ESLint Integration**
Seamlessly integrate with existing ESLint configurations:

```bash
# Analyze with both SunLint + existing ESLint rules  
sunlint --all --eslint-integration --input=src

# Mix ESLint and heuristic engines based on rule compatibility
sunlint --rules=C010,C006 --eslint-integration --input=src
```

**✅ Current Status:**
- ✅ **Multi-engine orchestration**: Rules automatically routed to optimal engine
- ✅ **ESLint v8/v9 compatibility**: Production-ready with both major versions  
- ✅ **TypeScript support**: Full TS/TSX parsing with custom rule implementation
- ✅ **Custom rule integration**: 17+ SunLint custom rules via ESLint engine  
- ✅ **Smart fallback**: Automatic engine fallback for maximum rule coverage
- ✅ **Production tested**: Successfully processes real projects with mixed violations

**Benefits:**
- ✅ **No workflow disruption**: Existing ESLint continues working
- ✅ **Engine flexibility**: Automatic best-engine selection per rule
- ✅ **Combined reporting**: Unified violation tracking from multiple engines
- ✅ **Adapter Pattern**: Same rule access layer as VSCode extension

## 🔀 **Git Integration**

Optimize CI/CD workflows with Git integration:

```bash
# Analyze only changed files
sunlint --all --changed-files

# Pre-commit validation
sunlint --all --staged-files

# PR mode with failure only on new violations
sunlint --all --pr-mode --fail-on-new-violations
```

## 🎯 **Advanced File Targeting**

Powerful file targeting capabilities:

```bash
# Include specific patterns
sunlint --all --include="src/**/*.ts,lib/**/*.dart" --input=.

# Exclude patterns  
sunlint --all --exclude="**/*.test.*,**/*.generated.*" --input=src

# Language filtering
sunlint --all --languages=typescript,dart --input=src

# Source files only (exclude tests, configs)
sunlint --all --only-source --input=src
```

### **Configuration Priority** (Highest to Lowest)
1. **CLI flags**: `--include`, `--exclude`, `--languages`
2. **Project config**: `.sunlint.json`
3. **Package.json**: `"sunlint"` field  
4. **Default config**: Built-in patterns

## 📋 **Available Rules**

### **Quality Rules** ✨ (30 rules)
| Rule ID | Name | Status |
|---------|------|--------|
| **C002** | No Duplicate Code | ✅ Stable |
| **C003** | No Vague Abbreviations | ✅ Stable |
| **C006** | Function Naming Convention | ✅ Stable |
| **C010** | Limit Block Nesting | ✅ Stable |
| **C013** | No Dead Code | ✅ Stable |
| **C014** | Dependency Injection | ✅ Stable |
| **C017** | Limit Constructor Logic | ✅ Stable |
| **C018** | No Generic Throw | ✅ Stable |
| **C019** | Log Level Usage | ✅ Stable |
| **C023** | No Duplicate Variable Names | ✅ Stable |
| **C029** | Catch Block Logging | ✅ Stable |
| **C030** | Use Custom Error Classes | ✅ Stable |
| **C031** | Validation Separation | ✅ Stable |
| **C041** | No Hardcoded Config | ✅ Stable |
| **C042** | Boolean Name Prefix | ✅ Stable |
| **C043** | No Console or Print | ✅ Stable |
| **C047** | No Duplicate Retry Logic | ✅ Stable |
| **C075** | Explicit Function Return Types | ✅ Stable |
| **T002-T021** | TypeScript-specific rules | ✅ Stable |

### **Security Rules** 🔒 (47 rules)
| Rule ID | Name | Status |
|---------|------|--------|
| **S001** | Fail Securely Access Control | ✅ Stable |
| **S002** | Prevent IDOR Vulnerabilities | ✅ Stable |
| **S003** | URL Redirect Validation | ✅ Stable |
| **S005** | No Origin Header Authentication | ✅ Stable |
| **S006** | Activation Recovery Not Plaintext | ✅ Stable |
| **S007** | Secure OTP Storage | ✅ Stable |
| **S008** | Crypto Agility | ✅ Stable |
| **S009** | No Insecure Crypto | ✅ Stable |
| **S010** | Secure Random Generation | ✅ Stable |
| **S011** | Secure UUID Generation | ✅ Stable |
| **S012** | No Hardcoded Secrets | ✅ Stable |
| **S013** | Always Use TLS | ✅ Stable |
| **S014** | Secure TLS Version | ✅ Stable |
| **S015** | Valid TLS Certificate | ✅ Stable |
| **S016-S058** | *...Additional security rules* | ✅ Stable |

## ⚙️ **Configuration**

Create `.sunlint.json` in your project root:

### **Quick Start Configuration**
```json
{
  "extends": "@sun/sunlint/recommended",
  "input": ["src"],
  "exclude": ["**/*.test.*", "**/*.generated.*"],
  "rules": {
    "C019": "error",
    "C006": "warn", 
    "S005": "error"
  }
}
```

### **Available Presets**
- `@sun/sunlint/recommended` - Balanced rules for all projects
- `@sun/sunlint/security` - Security-focused rules only  
- `@sun/sunlint/quality` - Quality-focused rules only
- `@sun/sunlint/beginner` - Gentle introduction for new teams
- `@sun/sunlint/ci` - Optimized for CI/CD environments

### **Full Configuration Reference**
📖 **[View Complete Configuration Guide](./docs/CONFIGURATION.md)**

Complete reference with all available options:
- File targeting (`include`, `exclude`, `languages`)
- Rule configurations with detailed descriptions
- Git integration settings (`changedFiles`, `baseline`)
- ESLint integration options
- Performance and caching settings
- CI/CD optimizations

> **🚨 MIGRATION NOTE**: `ignorePatterns` is deprecated. Use `exclude` instead. Run `npx sunlint migrate-config` to auto-migrate.

## 🎮 **Usage Examples**

### **Development**
```bash
# Quick start - works immediately
npm install --save-dev @sun-asterisk/sunlint
npx sunlint --all --input=src

# Check specific rules
sunlint --rules=C019,S005 --input=src

# ESLint integration (requires eslint dependency)
npm install --save-dev eslint
sunlint --all --eslint-integration --changed-files
```

### **TypeScript Projects**
```bash
# Enhanced TypeScript setup
npm install --save-dev @sun-asterisk/sunlint eslint @typescript-eslint/parser @typescript-eslint/eslint-plugin typescript

# Full TypeScript analysis
sunlint --all --input=src
sunlint --all --eslint-integration --input=src
```

### **CI/CD**
```bash
# Full project scan
sunlint --all --input=. --format=json --output=report.json

# PR validation
sunlint --all --changed-files --fail-on-new-violations

# Pre-commit hook  
sunlint --all --staged-files --format=summary
```

### **Testing & Debugging**
```bash
# Test adapter performance and coverage
sunlint --test-adapter

# Show detailed engine information
sunlint --show-engines --verbose

# Debug rule selection process
sunlint --rules=C010,R001 --verbose --debug

# Validate configuration
sunlint --validate-config .sunlint.json
```

## 📚 **Documentation**

- **[Configuration Guide](./docs/CONFIGURATION.md)** - Complete config options with examples
- **[Performance & File Limits](./docs/FILE_LIMITS_EXPLANATION.md)** - Understanding `--max-files` vs `--max-semantic-files`
- [ESLint Integration Guide](./docs/ESLINT_INTEGRATION.md)
- [CI/CD Guide](./docs/CI-CD-GUIDE.md)
- [Architecture](./docs/ARCHITECTURE.md)
- [Examples](./examples/README.md)

## 🤝 **Contributing**

See [CONTRIBUTING.md](./CONTRIBUTING.md) for development guidelines.

## 📄 **License**

MIT License - see [LICENSE](./LICENSE) for details.

---

**Built with ❤️ by Sun* Engineering Team**
