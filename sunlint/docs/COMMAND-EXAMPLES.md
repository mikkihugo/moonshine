# 🎮 SunLint Command Examples & Demos

## 📋 **Tổng hợp đầy đủ các chức năng CLI đã hỗ trợ**

### ✅ **1. Phạm vi kiểm tra (Input Scope)**

```bash
# Kiểm tra 1 file cụ thể
node cli.js --all --input=cli.js --format=summary --no-ai

# Kiểm tra 1 folder/directory  
node cli.js --all --input=core --format=summary --no-ai

# Kiểm tra toàn bộ project/workspace
node cli.js --all --input=. --format=summary --no-ai

# Kiểm tra nhiều folders (comma-separated)
node cli.js --all --input=core,rules --format=summary --no-ai

# Kiểm tra chỉ files đã thay đổi (Git integration)
node cli.js --all --changed-files --format=summary --no-ai

# Kiểm tra chỉ files đã staged (Pre-commit)
node cli.js --all --staged-files --format=summary --no-ai

# Kiểm tra files thay đổi so với branch cụ thể
node cli.js --all --changed-files --diff-base=origin/main --format=summary
```

### ✅ **2. Lựa chọn Rules**

```bash
# Kiểm tra 1 rule cụ thể
node cli.js --rule=C019 --input=. --format=summary --no-ai

# Kiểm tra nhiều rules cụ thể
node cli.js --rules=C019,C006,C029 --input=. --format=summary --no-ai

# Kiểm tra tất cả rules
node cli.js --all --input=. --format=summary --no-ai

# Kiểm tra theo category (quality rules)
node cli.js --quality --input=. --format=summary --no-ai

# Kiểm tra theo category (security rules) 
node cli.js --security --input=. --format=summary --no-ai

# Loại trừ một số rules cụ thể
node cli.js --all --exclude-rules=C031 --input=. --format=summary --no-ai
```

### ✅ **3. Phương pháp phân tích**

```bash
# Pattern-based analysis (free, fast)
node cli.js --all --input=. --format=summary --no-ai

# AI-powered analysis (cost, more accurate)
node cli.js --all --input=. --format=summary --ai

# Hybrid: AI cho rules cụ thể, pattern cho còn lại
node cli.js --rule=C019 --input=. --ai --format=summary
```

### ✅ **4. Output Formats**

```bash
# Human-readable summary
node cli.js --all --input=. --format=summary --no-ai

# ESLint-compatible JSON (for IDEs)
node cli.js --all --input=. --format=eslint --no-ai

# Structured JSON (for processing)
node cli.js --all --input=. --format=json --no-ai

# Table format (for reports)
node cli.js --all --input=. --format=table --no-ai

# GitHub Actions format (for CI)
node cli.js --all --input=. --format=github --no-ai

# Save to file
node cli.js --all --input=. --format=json --output=report.json --no-ai
```

### ✅ **5. CI/CD Features**

```bash
# PR Mode: Chỉ check violations mới
node cli.js --all --changed-files --fail-on-new-violations --format=summary

# Baseline comparison
node cli.js --all --input=. --save-baseline=baseline.json --format=json --no-ai
node cli.js --all --changed-files --baseline=baseline.json --fail-on-new-violations

# Severity filtering
node cli.js --all --input=. --severity=error --format=summary --no-ai

# Language filtering  
node cli.js --all --input=. --languages=typescript,javascript --format=summary
```

### ✅ **6. Performance & Advanced Options**

```bash
# Control concurrent execution
node cli.js --all --input=. --max-concurrent=10 --format=summary --no-ai

# Set timeout for rules
node cli.js --all --input=. --timeout=60000 --format=summary --no-ai

# Disable caching
node cli.js --all --input=. --no-cache --format=summary --no-ai

# **Control semantic analysis for large projects**
# Default limit: 1000 files for performance balance
node cli.js --all --input=. --max-semantic-files=1000 --format=summary

# For small projects: Analyze all files
node cli.js --all --input=. --max-semantic-files=0 --format=summary

# For large projects: Conservative analysis
node cli.js --all --input=. --max-semantic-files=500 --format=summary

# For massive projects: Minimal semantic analysis
node cli.js --all --input=. --max-semantic-files=100 --format=summary

# Unlimited semantic analysis (use with caution!)
node cli.js --all --input=. --max-semantic-files=-1 --format=summary

# Verbose logging
node cli.js --all --input=. --verbose --format=summary --no-ai

# Quiet mode (errors only)
node cli.js --all --input=. --quiet --format=summary --no-ai

# Debug mode
node cli.js --all --input=. --debug --format=summary --no-ai

# Dry run (show what would be analyzed)
node cli.js --all --input=. --dry-run --format=summary --no-ai
```

## 🚀 **Use Cases & Scenarios**

### **Local Development** 🏠

```bash
# Quick check before commit
node cli.js --all --staged-files --format=summary --no-ai

# Check current work
node cli.js --all --changed-files --format=summary --no-ai  

# Focus on specific issue type
node cli.js --rule=C019 --input=. --format=summary --no-ai

# Deep analysis with AI
node cli.js --quality --input=src --ai --format=detailed
```

### **Code Review** 👀

```bash
# Check PR changes
node cli.js --all --changed-files --diff-base=origin/main --format=github

# Focus on security for sensitive changes
node cli.js --security --changed-files --format=summary --no-ai

# New violations only
node cli.js --all --changed-files --baseline=baseline.json --fail-on-new-violations
```

### **CI/CD Pipeline** 🔄

```bash
# Fast PR check
node cli.js --all --changed-files --format=github --no-ai --timeout=30000

# Full scan for main branch
node cli.js --all --input=. --format=json --output=report.json --no-ai

# Security-critical check
node cli.js --security --input=. --severity=error --format=summary --no-ai

# Quality gate
node cli.js --quality --changed-files --max-new-violations=5 --format=summary
```

### **Project Health Monitoring** 📊

```bash
# Full project assessment
node cli.js --all --input=. --format=detailed --output=health-report.json --no-ai

# Trend analysis
node cli.js --all --input=. --baseline=last-month.json --format=trend --no-ai

# Focus areas
node cli.js --rules=C019,C029 --input=core --format=table --no-ai
```

## 🎯 **Practical Examples**

### **Example 1: New Feature Development**
```bash
# Day 1: Start development
node cli.js --all --staged-files --format=summary --no-ai

# Day 2: Check progress  
node cli.js --all --changed-files --format=summary --no-ai

# Day 3: Pre-review check
node cli.js --all --changed-files --diff-base=origin/main --format=github --no-ai

# Day 4: Final validation
node cli.js --all --changed-files --ai --format=detailed
```

## 🏗️ **Large Project Strategies**

> **⚡ Performance Note**: SunLint uses semantic analysis for advanced rules (like C047). For projects with 1000+ files, you can control semantic analysis scope to balance accuracy vs performance.

### **Strategy 1: Incremental Analysis** 📈
```bash
# Start with changed files only (fastest)
node cli.js --all --changed-files --format=summary --no-ai

# Focus on specific directories
node cli.js --all --input=src/critical --max-semantic-files=2000 --format=summary

# Target important file patterns only
node cli.js --all --include="src/**/*.ts" --exclude="**/*.test.*,**/*.d.ts" --input=.

# Use directory-based analysis
node cli.js --all --input=src/auth --format=summary  # Most critical module first
node cli.js --all --input=src/api --format=summary   # Then API layer
node cli.js --all --input=src/utils --format=summary # Finally utilities
```

### **Strategy 2: Semantic Analysis Tuning** 🔧
```bash
# Conservative: 500 files for faster analysis
node cli.js --all --input=. --max-semantic-files=500 --format=summary

# Balanced: 1000 files (default) for medium projects
node cli.js --all --input=. --max-semantic-files=1000 --format=summary

# Comprehensive: 2000+ files for complete analysis
node cli.js --all --input=. --max-semantic-files=2000 --format=summary

# Unlimited: All files (use for final validation)
node cli.js --all --input=. --max-semantic-files=-1 --format=summary

# Disable semantic analysis completely (heuristic only)
node cli.js --all --input=. --max-semantic-files=0 --format=summary
```

### **Strategy 3: Rule-Based Prioritization** 🎯
```bash
# Phase 1: Critical security issues (fast heuristic rules)
node cli.js --security --input=. --max-semantic-files=0 --format=summary

# Phase 2: Code quality basics
node cli.js --rules=C006,C019,C029 --input=. --max-semantic-files=500 --format=summary

# Phase 3: Advanced semantic rules (targeted)
node cli.js --rules=C047 --input=src --max-semantic-files=1000 --format=summary

# Phase 4: Full comprehensive scan
node cli.js --all --input=. --max-semantic-files=-1 --format=detailed
```

### **Strategy 4: CI/CD Optimization** ⚡
```bash
# PR checks: Fast semantic analysis
node cli.js --all --changed-files --max-semantic-files=300 --format=github --no-ai

# Nightly builds: Medium semantic analysis
node cli.js --all --input=. --max-semantic-files=1000 --format=json --output=nightly.json

# Weekly reports: Full semantic analysis
node cli.js --all --input=. --max-semantic-files=-1 --format=detailed --output=weekly.json

# Release validation: Comprehensive with baselines
node cli.js --all --input=. --max-semantic-files=2000 --baseline=last-release.json
```

### **Strategy 5: Memory & Performance Monitoring** 📊
```bash
# Monitor file loading (debug mode)
node cli.js --all --input=. --max-semantic-files=1000 --verbose --debug

# Track performance with different limits
time node cli.js --all --input=. --max-semantic-files=500 --format=summary
time node cli.js --all --input=. --max-semantic-files=1000 --format=summary
time node cli.js --all --input=. --max-semantic-files=2000 --format=summary

# Memory-conscious analysis for CI
node cli.js --all --input=. --max-semantic-files=300 --max-concurrent=2 --format=summary
```

### **📋 Recommended Limits by Project Size**

| Project Size | Files Count | Recommended Limit | Use Case |
|-------------|-------------|-------------------|----------|
| Small | < 100 files | `--max-semantic-files=0` (all) | Complete analysis |
| Medium | 100-500 files | `--max-semantic-files=500` | Balanced |
| Large | 500-2000 files | `--max-semantic-files=1000` | Default recommended |
| Enterprise | 2000-5000 files | `--max-semantic-files=1500` | Conservative |
| Massive | 5000+ files | `--max-semantic-files=500` | Targeted analysis |

> **💡 Pro Tips for Large Projects:**
> 1. Use `--changed-files` for daily development
> 2. Use `--max-semantic-files=500` for CI/CD pipelines  
> 3. Use `--max-semantic-files=-1` for release validation
> 4. Combine with `--include` patterns to focus on critical code
> 5. Monitor analysis time and adjust limits accordingly

### **Example 1: Monorepo with 5000+ Files**
```bash
# Daily development: Changed files only
node cli.js --all --changed-files --max-semantic-files=300 --format=summary

# Module-specific analysis
node cli.js --all --input=packages/core --max-semantic-files=1000 --format=summary
node cli.js --all --input=packages/api --max-semantic-files=1000 --format=summary

# CI pipeline: Conservative semantic analysis
node cli.js --all --changed-files --max-semantic-files=500 --format=github

# Release validation: Full analysis by modules
for dir in packages/*/; do
  node cli.js --all --input="$dir" --max-semantic-files=2000 --format=json --output="${dir//\//-}-report.json"
done
```

### **Example 2: Legacy Code Improvement**
```bash
# Step 1: Baseline assessment
node cli.js --all --input=legacy-module --save-baseline=legacy-baseline.json --no-ai

# Step 2: Focus on critical issues
node cli.js --security --input=legacy-module --severity=error --format=summary

# Step 3: Incremental improvement
node cli.js --rule=C019 --input=legacy-module --format=summary --no-ai

# Step 4: Track progress
node cli.js --all --input=legacy-module --baseline=legacy-baseline.json --format=trend
```

### **Example 3: Team Onboarding**
```bash
# Level 1: Basic checks
node cli.js --rules=C006,C019 --input=. --format=summary --no-ai

# Level 2: Quality focus
node cli.js --quality --input=. --format=table --no-ai

# Level 3: Full analysis
node cli.js --all --input=. --format=detailed --no-ai

# Level 4: AI-assisted learning
node cli.js --all --input=. --ai --verbose --format=detailed
```

## 📝 **Command Cheat Sheet**

| Task | Command |
|------|---------|
| Quick pre-commit check | `node cli.js --all --staged-files --format=summary --no-ai` |
| PR review | `node cli.js --all --changed-files --diff-base=origin/main --format=github` |
| Full project scan | `node cli.js --all --input=. --format=json --output=report.json --no-ai` |
| Security audit | `node cli.js --security --input=. --severity=error --format=summary` |
| New violations only | `node cli.js --all --changed-files --baseline=baseline.json --fail-on-new-violations` |
| AI deep analysis | `node cli.js --quality --input=src --ai --format=detailed` |
| Performance test | `node cli.js --all --input=. --max-concurrent=1 --timeout=10000 --no-ai` |
| Debug issues | `node cli.js --rule=C019 --input=problematic-file.js --debug --verbose` |

## 💡 **Pro Tips**

1. **Start with `--no-ai`** for faster feedback, use `--ai` for complex issues
2. **Use `--changed-files`** in development, `--input=.` for comprehensive checks  
3. **Save baselines** for large projects to track progress over time
4. **Combine `--severity=error`** with CI to focus on critical issues
5. **Use `--dry-run`** to understand what will be analyzed before running
6. **Set `--timeout`** appropriately based on project size and CI time limits
