# MoonShine Rule Presets

Pre-configured rule sets for different development scenarios and requirements.

## Available Presets

### 🔒 **security-critical.json**
**Use for**: Banking, healthcare, government, critical infrastructure
- All security rules at Error level
- Advanced threat detection (CSRF, XXE, SSRF, etc.)
- Strict secret detection
- No debugging aids allowed
- Maximum AI security analysis

### ⚡ **performance-optimized.json**
**Use for**: Gaming, real-time apps, high-throughput systems
- Performance-focused rules
- React optimization patterns
- Async/await best practices
- TypeScript performance hints
- Aggressive caching enabled

### 📝 **typescript-strict.json**
**Use for**: Type-safe codebases, large teams, complex applications
- Strict type checking
- No `any` types allowed
- Comprehensive null checks
- Function signature requirements
- Naming convention enforcement

### 🚀 **development-friendly.json**
**Use for**: Active development, prototyping, learning
- Warnings instead of errors
- Console logging allowed
- Helpful suggestions
- Auto-fix enabled
- AI insights as info level

### 🏢 **enterprise-strict.json**
**Use for**: Large organizations, production systems, compliance requirements
- Comprehensive rule coverage
- Complexity limits enforced
- Accessibility requirements
- Documentation standards
- Architecture analysis

## Usage

### ESLint Integration
```javascript
// .eslintrc.js
module.exports = {
  extends: [
    './rulebase/presets/security-critical.json'
  ],
  // Override specific rules if needed
  rules: {
    'no-console': 'warn' // Relax from error to warn
  }
};
```

### MoonShine Configuration
```javascript
// moonshine.config.js
module.exports = {
  preset: 'security-critical',
  // or
  extends: ['./rulebase/presets/enterprise-strict.json'],
  overrides: {
    rules: {
      '@moonshine/ai-code-quality-oracle': 'warn'
    }
  }
};
```

### Command Line
```bash
# Use specific preset
moon-shine --preset security-critical src/

# Combine presets
moon-shine --extend typescript-strict,performance-optimized src/

# Override preset rules
moon-shine --preset enterprise-strict --rule "no-console:off" src/
```

## Customizing Presets

You can create custom presets by:

1. **Extending existing presets**:
```json
{
  "name": "My Custom Preset",
  "extends": ["./security-critical.json"],
  "rules": {
    "custom-rule": "warn"
  }
}
```

2. **Combining multiple presets**:
```json
{
  "name": "Security + Performance",
  "extends": [
    "./security-critical.json",
    "./performance-optimized.json"
  ]
}
```

3. **Project-specific overrides**:
```json
{
  "name": "My Project",
  "extends": ["./enterprise-strict.json"],
  "overrides": [
    {
      "files": ["src/legacy/**"],
      "rules": {
        "@typescript-eslint/no-explicit-any": "warn"
      }
    }
  ]
}
```

## Preset Selection Guide

| Use Case | Preset | Focus |
|----------|--------|-------|
| Banking/Finance | `security-critical` | Maximum security |
| Gaming/Real-time | `performance-optimized` | Speed & efficiency |
| Enterprise SaaS | `enterprise-strict` | Comprehensive quality |
| Startup MVP | `development-friendly` | Move fast, stay helpful |
| Type-safe App | `typescript-strict` | Type safety |
| Open Source | `development-friendly` + security rules | Community friendly |
| Compliance Required | `enterprise-strict` | Audit-ready |

Choose your preset based on your project's primary concerns and gradually adjust rules as your codebase matures.