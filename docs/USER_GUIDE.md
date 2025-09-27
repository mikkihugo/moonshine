# Moon Shine AI Linter - User Guide

## 🚀 Quick Start

### Installation

1. **Prerequisites:**
   ```bash
   # Ensure Moon CLI is installed
   curl -fsSL https://moonrepo.dev/install/moon.sh | bash

   # Verify installation
   moon --version
   ```

2. **Install Moon Shine AI Linter:**
   ```bash
   # Via Moon CLI
   moon ext install @moonrepo/moon-shine

   # Or build from source
   cargo build --release --target wasm32-unknown-unknown
   ```

3. **Basic Configuration:**
   ```yaml
   # moon.yml
   extensions:
     - id: "@moonrepo/moon-shine"
       config:
         ai:
           providers:
             claude:
               model: "claude-3-sonnet"
               api_key: "${ANTHROPIC_API_KEY}"
           linting:
             enable_ai_behavioral: true
             confidence_threshold: 0.7
   ```

### First Analysis

```bash
# Analyze a single file
moon run @moonrepo/moon-shine analyze src/app.tsx

# Analyze entire project
moon run @moonrepo/moon-shine analyze --recursive

# With AI enhancement
moon run @moonrepo/moon-shine analyze src/app.tsx --ai-enhanced
```

## ⚙️ Configuration Reference

### Core Configuration

```yaml
# .moon/extensions/moon-shine.yml
ai:
  # AI Provider Settings
  providers:
    claude:
      model: "claude-3-sonnet"      # claude-3-sonnet, claude-3-opus
      api_key: "${ANTHROPIC_API_KEY}"
      max_tokens: 4096
      temperature: 0.1

    openai:
      model: "gpt-4"                # gpt-4, gpt-3.5-turbo
      api_key: "${OPENAI_API_KEY}"
      max_tokens: 4096
      temperature: 0.1

    gemini:
      model: "gemini-pro"
      api_key: "${GOOGLE_API_KEY}"

  # Provider Selection
  provider_selection:
    strategy: "intelligent"         # intelligent, round_robin, cost_optimized
    fallback_enabled: true
    fallback_to_static: true

  # Cost Controls
  budget:
    daily_limit_usd: 10.0
    per_analysis_limit_usd: 0.50
    enable_cost_tracking: true

# Linting Configuration
linting:
  # AI Features
  enable_ai_behavioral: true
  enable_ai_enhanced_rules: true
  enable_pattern_learning: true

  # Quality Thresholds
  confidence_threshold: 0.7         # 0.0 - 1.0
  min_severity_for_ai: "warning"    # info, warning, error

  # Performance
  parallel_analysis: true
  max_concurrent_ai_calls: 3
  cache_ai_results: true
  cache_ttl_hours: 24

# Rule Configuration
rules:
  # Static Rules (OXC/ESLint)
  static:
    typescript: "recommended"
    react: "recommended"
    security: "strict"

  # AI-Enhanced Rules
  ai_behavioral:
    - "performance-bottlenecks"
    - "security-vulnerabilities"
    - "code-smells"
    - "architecture-violations"

  # Custom Rules
  custom:
    - pattern: "react-hooks-deps"
      severity: "error"
      ai_enhanced: true

# Output Configuration
output:
  format: "json"                    # json, yaml, table, github
  include_ai_reasoning: true
  include_confidence_scores: true
  group_by_severity: true
```

### Environment Variables

```bash
# Required for AI providers
export ANTHROPIC_API_KEY="your-claude-key"
export OPENAI_API_KEY="your-openai-key"
export GOOGLE_API_KEY="your-gemini-key"

# Optional performance tuning
export MOON_SHINE_CACHE_DIR="/tmp/moon-shine-cache"
export MOON_SHINE_MAX_MEMORY="1GB"
export MOON_SHINE_PARALLEL_JOBS="4"
```

## 📖 Usage Examples

### Basic Analysis

```bash
# Analyze TypeScript files
moon run @moonrepo/moon-shine analyze "src/**/*.{ts,tsx}"

# Analyze with specific rules
moon run @moonrepo/moon-shine analyze src/ --rules="security,performance"

# Output to file
moon run @moonrepo/moon-shine analyze src/ --output="results.json"
```

### AI-Enhanced Analysis

```bash
# Enable AI behavioral analysis
moon run @moonrepo/moon-shine analyze src/ --ai-behavioral

# Use specific AI provider
moon run @moonrepo/moon-shine analyze src/ --provider="claude"

# Adjust confidence threshold
moon run @moonrepo/moon-shine analyze src/ --confidence=0.8
```

### Integration with Moon Tasks

```yaml
# .moon/tasks.yml
tasks:
  lint:
    command: "moon run @moonrepo/moon-shine analyze src/"
    inputs:
      - "src/**/*"
    outputs:
      - "lint-results.json"

  lint-ai:
    command: "moon run @moonrepo/moon-shine analyze src/ --ai-enhanced"
    inputs:
      - "src/**/*"
    deps:
      - "~:type-check"
```

### CI/CD Integration

```yaml
# .github/workflows/lint.yml
name: Lint with Moon Shine AI
on: [push, pull_request]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: moonrepo/setup-moon-action@v1

      - name: AI Linting
        run: moon run @moonrepo/moon-shine analyze src/ --ai-enhanced
        env:
          ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}

      - name: Upload Results
        uses: actions/upload-artifact@v3
        with:
          name: lint-results
          path: lint-results.json
```

## 🎯 AI-Enhanced Features

### Behavioral Pattern Detection

Moon Shine uses AI to detect complex patterns beyond static analysis:

```typescript
// AI detects: Performance bottleneck
function processUsers(users: User[]) {
  return users.map(user => {
    // Expensive operation in loop
    const profile = await fetchUserProfile(user.id);
    return { ...user, profile };
  });
}

// AI suggestion: Use Promise.all for parallel processing
function processUsersOptimized(users: User[]) {
  return Promise.all(
    users.map(async user => {
      const profile = await fetchUserProfile(user.id);
      return { ...user, profile };
    })
  );
}
```

### Security Vulnerability Detection

```typescript
// AI detects: Potential XSS vulnerability
function renderHTML(userInput: string) {
  return `<div>${userInput}</div>`;  // Unsafe
}

// AI suggestion: Use proper sanitization
import DOMPurify from 'dompurify';
function renderHTMLSafe(userInput: string) {
  return `<div>${DOMPurify.sanitize(userInput)}</div>`;
}
```

### Architecture Analysis

```typescript
// AI detects: Circular dependency risk
import { UserService } from './user-service';
export class AuthService {
  constructor(private userService: UserService) {}
}

// In user-service.ts
import { AuthService } from './auth-service';  // Circular!
```

## 🔧 Troubleshooting

### Common Issues

**1. AI Provider Authentication Errors**
```bash
Error: Invalid API key for provider 'claude'
```
Solution: Verify environment variables are set correctly:
```bash
echo $ANTHROPIC_API_KEY
# Should show your API key
```

**2. Rate Limiting**
```bash
Error: Rate limit exceeded for provider 'openai'
```
Solution: Adjust configuration:
```yaml
ai:
  budget:
    per_analysis_limit_usd: 0.25  # Reduce cost limit
  providers:
    openai:
      rate_limit_per_minute: 10   # Reduce rate
```

**3. WASM Memory Issues**
```bash
Error: Memory allocation failed
```
Solution: Increase memory limit:
```bash
export MOON_SHINE_MAX_MEMORY="2GB"
```

**4. Slow Analysis Performance**
```bash
Analysis taking too long...
```
Solution: Optimize configuration:
```yaml
linting:
  parallel_analysis: true
  max_concurrent_ai_calls: 5
  cache_ai_results: true
```

### Debug Mode

Enable detailed logging:
```bash
MOON_SHINE_LOG_LEVEL=debug moon run @moonrepo/moon-shine analyze src/
```

### Performance Profiling

```bash
# Benchmark analysis speed
moon run @moonrepo/moon-shine benchmark src/

# Memory usage analysis
moon run @moonrepo/moon-shine analyze src/ --profile-memory
```

## 📊 Output Formats

### JSON Output

```json
{
  "analysis_summary": {
    "files_analyzed": 42,
    "total_issues": 15,
    "ai_enhanced_issues": 8,
    "analysis_time_ms": 2341
  },
  "issues": [
    {
      "file": "src/components/UserProfile.tsx",
      "line": 23,
      "column": 5,
      "severity": "warning",
      "rule": "performance-bottleneck",
      "message": "Expensive operation in render loop",
      "ai_reasoning": "The fetchUserData call in the render method will execute on every re-render, causing performance issues.",
      "confidence": 0.92,
      "suggestions": [
        "Move data fetching to useEffect",
        "Use React Query for data caching"
      ]
    }
  ]
}
```

### Table Output

```
┌─────────────────────────┬──────┬──────────┬─────────────────────────┬────────────┐
│ File                    │ Line │ Severity │ Rule                    │ Confidence │
├─────────────────────────┼──────┼──────────┼─────────────────────────┼────────────┤
│ src/utils/api.ts        │   15 │ error    │ security-vulnerability  │ 0.95       │
│ src/components/Form.tsx │   42 │ warning  │ performance-bottleneck  │ 0.87       │
│ src/hooks/useAuth.tsx   │   28 │ info     │ code-smell             │ 0.73       │
└─────────────────────────┴──────┴──────────┴─────────────────────────┴────────────┘
```

## 🔗 Integration with Development Tools

### VS Code Extension

Moon Shine integrates with AI CLI tools for enhanced analysis:

```bash
# Install required AI CLI tools
npm install -g @anthropic-ai/claude-code
npm install -g @google/gemini-cli  
npm install -g @openai/codex-cli
```

### Pre-commit Hooks

```yaml
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: moon-shine-lint
        name: Moon Shine AI Linter
        entry: moon run @moonrepo/moon-shine analyze
        language: system
        files: \.(ts|tsx|js|jsx)$
```

### IDE Integration

Configure your IDE to use Moon Shine as the default linter:

```json
// VS Code settings.json
{
  "moon-shine.enableAI": true,
  "moon-shine.confidenceThreshold": 0.7,
  "moon-shine.providers": ["claude", "openai"]
}
```

## 🎓 Best Practices

### 1. Gradual AI Adoption

Start with static analysis, gradually enable AI features:

```yaml
# Week 1: Static only
linting:
  enable_ai_behavioral: false

# Week 2: Add AI behavioral
linting:
  enable_ai_behavioral: true
  confidence_threshold: 0.8  # High confidence only

# Week 3: Lower threshold
linting:
  confidence_threshold: 0.7
```

### 2. Cost Management

Monitor and control AI costs:

```yaml
ai:
  budget:
    daily_limit_usd: 5.0
    enable_cost_tracking: true
    alert_at_percent: 80
```

### 3. Team Configuration

Use shared configuration for consistency:

```yaml
# .moon/extensions/shared.yml
extends: "@moonrepo/moon-shine/configs/team-standard"
ai:
  providers:
    claude:
      model: "claude-3-sonnet"  # Consistent model choice
```

### 4. Performance Optimization

```yaml
linting:
  # Cache AI results aggressively
  cache_ai_results: true
  cache_ttl_hours: 48

  # Optimize for CI
  parallel_analysis: true
  max_concurrent_ai_calls: 2  # Conservative for CI
```

## 🆘 Getting Help

- **Documentation**: [moonrepo.dev/moon-shine](https://moonrepo.dev/moon-shine)
- **GitHub Issues**: [github.com/moonrepo/moon-shine/issues](https://github.com/moonrepo/moon-shine/issues)
- **Discord**: [discord.gg/moonrepo](https://discord.gg/moonrepo)
- **Email**: support@moonrepo.dev

## 📈 Monitoring & Analytics

### Usage Analytics

```bash
# View usage statistics
moon run @moonrepo/moon-shine stats

# Export analytics
moon run @moonrepo/moon-shine stats --export=analytics.json
```

### Cost Tracking

```bash
# View cost breakdown
moon run @moonrepo/moon-shine costs

# Daily usage report
moon run @moonrepo/moon-shine costs --daily
```

---

*Moon Shine AI Linter - Intelligent code analysis for the modern development workflow*