# AI-Enhanced Rule System Usage Guide

This guide demonstrates how to use the AI-enhanced rule system that combines traditional static analysis with AI-powered behavioral pattern detection, adaptive learning, and quality metrics tracking.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Rule Types](#rule-types)
3. [Configuration](#configuration)
4. [Using Templates](#using-templates)
5. [Execution](#execution)
6. [Learning and Adaptation](#learning-and-adaptation)
7. [Quality Metrics](#quality-metrics)
8. [Best Practices](#best-practices)
9. [Examples](#examples)

## Quick Start

### Basic Usage

```rust
use moon_shine::rulebase::{
    IntegratedRuleEngine, IntegratedEngineConfig,
    AIRuleTemplateRegistry, RuleSeverity
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create engine with default configuration
    let config = IntegratedEngineConfig::default();
    let mut engine = IntegratedRuleEngine::new(config);

    // Load rules from templates
    let security_templates = vec![
        "ai-security-xss-detection",
        "ai-security-sql-injection",
        "ai-security-weak-crypto",
    ];
    engine.load_rules_from_templates(&security_templates, RuleSeverity::Error, 50);

    // Analyze code
    let code = r#"
        function processUserInput(input) {
            document.getElementById('output').innerHTML = input; // XSS vulnerability
            const query = `SELECT * FROM users WHERE id = ${input}`; // SQL injection
        }
    "#;

    let result = engine.analyze_code(code, "example.js", None).await?;

    // Display results
    println!("Found {} issues", result.diagnostics.len());
    println!("Confidence: {:.2}", result.execution_summary.average_confidence);

    for diagnostic in &result.diagnostics {
        println!("  {}: {} (line {})", diagnostic.rule_name, diagnostic.message, diagnostic.line);
    }

    Ok(())
}
```

## Rule Types

The AI-enhanced rule system supports several types of rules:

### 1. Static Analysis Rules (OXC-based)

```rust
RuleImplementation::OxcStatic {
    rule_name: "no-unused-vars".to_string(),
}
```

### 2. AI Behavioral Rules

```rust
RuleImplementation::AiBehavioral {
    pattern_type: "xss_vulnerability".to_string(),
    ai_prompt: Some("Analyze this code for XSS vulnerabilities...".to_string()),
    confidence_threshold: Some(0.8),
    model_preference: Some("claude".to_string()),
    context_requirements: Some(AIContextRequirements {
        needs_ast: true,
        needs_dependencies: false,
        needs_project_context: true,
        max_context_tokens: 8000,
    }),
}
```

### 3. AI-Enhanced Static Rules

```rust
RuleImplementation::AIEnhanced {
    base_rule: "no-var".to_string(),
    ai_enhancements: AIEnhancements {
        context_analysis: true,
        smart_fixes: true,
        false_positive_reduction: true,
        severity_adjustment: true,
        pattern_learning: false,
    },
    enhancement_prompt: Some("Provide context-aware fixes for var usage".to_string()),
    fallback_to_static: true,
}
```

### 4. Hybrid Rules

```rust
RuleImplementation::Hybrid {
    implementations: vec![
        RuleImplementation::OxcStatic {
            rule_name: "no-eval".to_string(),
        },
        RuleImplementation::AiBehavioral {
            pattern_type: "dynamic_code_execution".to_string(),
            ai_prompt: Some("Look for dynamic code execution patterns".to_string()),
            confidence_threshold: Some(0.7),
            model_preference: None,
            context_requirements: None,
        },
    ],
    combination_strategy: HybridStrategy::BestConfidence,
    confidence_weights: HashMap::new(),
}
```

### 5. Smart Learning Rules

```rust
RuleImplementation::SmartRule {
    learning_enabled: true,
    adaptation_threshold: 0.8,
    feedback_learning: true,
    pattern_evolution: false,
    base_implementations: vec![
        RuleImplementation::AiBehavioral {
            pattern_type: "performance_issue".to_string(),
            ai_prompt: Some("Detect performance anti-patterns".to_string()),
            confidence_threshold: Some(0.7),
            model_preference: None,
            context_requirements: None,
        },
    ],
}
```

## Configuration

### Engine Configuration

```rust
let config = IntegratedEngineConfig {
    enable_ai_rules: true,
    enable_static_rules: true,
    enable_learning: true,
    enable_metrics: true,
    enable_adaptation: true,
    max_concurrent_ai_requests: 5,
    ai_timeout_seconds: 30,
    fallback_to_static: true,
    learning_config: LearningConfig {
        enabled: true,
        adaptation_threshold: 0.8,
        min_samples_for_learning: 10,
        max_history_size: 1000,
        confidence_decay_factor: 0.95,
        feedback_weight: 0.3,
        performance_window_days: 30,
    },
    metrics_config: MetricsConfig {
        enabled: true,
        collection_interval_seconds: 60,
        retention_days: 90,
        aggregation_window_minutes: 15,
        export_enabled: true,
        real_time_monitoring: true,
    },
};
```

### Rule Schema Configuration

Update your `rule-schema.json` to include AI-specific properties:

```json
{
  "id": "my-ai-rule",
  "name": "Custom AI Security Rule",
  "description": "Detects custom security patterns",
  "category": "Security",
  "severity": "Error",
  "implementation": {
    "type": "AiBehavioral",
    "pattern_type": "custom_security_pattern",
    "ai_prompt": "Look for specific security anti-patterns...",
    "confidence_threshold": 0.85,
    "model_preference": "claude",
    "context_requirements": {
      "needs_ast": true,
      "needs_dependencies": true,
      "needs_project_context": false,
      "max_context_tokens": 6000
    }
  },
  "ai_metadata": {
    "confidence_score": 0.9,
    "training_data_size": 1000,
    "model_version": "claude-3-sonnet"
  }
}
```

## Using Templates

### Available Templates

```rust
let registry = AIRuleTemplateRegistry::new();
let template_ids = registry.list_template_ids();
println!("Available templates: {:#?}", template_ids);

// Get templates by category
let security_templates = registry.get_templates_by_category(&RuleCategory::Security);
println!("Security templates: {}", security_templates.len());
```

### Template Customization

```rust
let registry = AIRuleTemplateRegistry::new();
let mut template = registry.get_template("ai-security-xss-detection").unwrap().clone();

// Customize template parameters
let mut customization = HashMap::new();
customization.insert("confidence_threshold".to_string(), "0.9".to_string());
customization.insert("model_preference".to_string(), "claude".to_string());
customization.insert("max_context_tokens".to_string(), "10000".to_string());

template.customize(customization);

// Convert to rule
let rule = template.to_rule_metadata(RuleSeverity::Error, 60);
```

### Preset Generation

```rust
use moon_shine::rulebase::ai_rule_templates::template_utils;

// Generate security-focused preset
let security_rules = template_utils::generate_security_preset();

// Generate performance-focused preset
let performance_rules = template_utils::generate_performance_preset();

// Load rules into engine
engine.load_rules(security_rules);
engine.load_rules(performance_rules);
```

## Execution

### Single File Analysis

```rust
let result = engine.analyze_code(code, "app.ts", None).await?;

// Access different result components
println!("Execution Summary:");
println!("  Total rules: {}", result.execution_summary.total_rules_executed);
println!("  AI rules: {}", result.execution_summary.ai_rules_executed);
println!("  Issues found: {}", result.execution_summary.issues_found);
println!("  Average confidence: {:.2}", result.execution_summary.average_confidence);

// AI-specific analysis
if let Some(ai_analysis) = result.ai_analysis {
    println!("AI Analysis:");
    println!("  Provider: {}", ai_analysis.provider_used);
    println!("  Patterns found: {:#?}", ai_analysis.behavioral_patterns_found);
    println!("  Suggestions: {:#?}", ai_analysis.suggested_improvements);
}
```

### Batch Analysis

```rust
let files = vec![
    ("src/app.ts".to_string(), std::fs::read_to_string("src/app.ts")?),
    ("src/utils.js".to_string(), std::fs::read_to_string("src/utils.js")?),
    ("src/api.ts".to_string(), std::fs::read_to_string("src/api.ts")?),
];

let results = engine.analyze_batch(&files, None).await?;

for (i, result) in results.iter().enumerate() {
    println!("File {}: {} issues found", i, result.diagnostics.len());
}
```

### With Project Context

```rust
use moon_shine::rulebase::integrated_rule_engine::ProjectContext;

let project_context = ProjectContext {
    framework: Some("react".to_string()),
    build_tool: Some("vite".to_string()),
    testing_framework: Some("jest".to_string()),
    dependencies: {
        let mut deps = HashMap::new();
        deps.insert("react".to_string(), "18.0.0".to_string());
        deps.insert("typescript".to_string(), "5.0.0".to_string());
        deps
    },
    typescript_config: Some("tsconfig.json".to_string()),
    lint_config: Some("eslint.config.js".to_string()),
};

let result = engine.analyze_code(code, "component.tsx", Some(project_context)).await?;
```

## Learning and Adaptation

### Recording Feedback

```rust
// After showing results to user and getting feedback
engine.record_feedback(
    "ai-security-xss-detection",
    execution_timestamp,
    UserFeedback::TruePositive
).await;

// Different feedback types
engine.record_feedback(rule_id, timestamp, UserFeedback::FalsePositive).await;
engine.record_feedback(rule_id, timestamp, UserFeedback::Helpful).await;
engine.record_feedback(rule_id, timestamp, UserFeedback::NotHelpful).await;
```

### Manual Adaptation

```rust
// Check for adaptation suggestions
let suggestions = engine.generate_adaptation_suggestions().await;

for suggestion in suggestions {
    println!("Rule: {}", suggestion.rule_id);
    println!("Type: {}", suggestion.suggestion_type);
    println!("Description: {}", suggestion.description);
    println!("Impact: {:.2}", suggestion.impact_estimate);

    if suggestion.auto_apply {
        println!("  -> Will be auto-applied");
    }
}

// Apply adaptations automatically
let applied = engine.apply_adaptations().await?;
println!("Applied {} adaptations", applied.len());
```

### Viewing Learning Data

```rust
// Get learning engine access (in real code, you'd expose this through the integrated engine)
if let Ok(learning) = engine.learning_engine.lock() {
    let export = learning.export_learning_data("ai-security-xss-detection");
    if let Some(data) = export {
        println!("Learning Data for rule:");
        println!("  Total executions: {}", data.total_executions);
        println!("  Data points: {}", data.data_points.len());

        if let Some(adaptation) = data.adaptation_suggestions {
            println!("  Should adapt: {}", adaptation.should_adapt);
            println!("  Reasoning: {}", adaptation.reasoning);
        }
    }
}
```

## Quality Metrics

### Accessing Metrics

```rust
// Get quality dashboard
let dashboard = engine.get_quality_dashboard().unwrap();

println!("System Overview:");
println!("  Total rules: {}", dashboard.overview.total_rules);
println!("  Average quality: {:.2}", dashboard.overview.average_quality_score);
println!("  User satisfaction: {:.2}", dashboard.overview.user_satisfaction);

println!("Top performing rules:");
for rule in &dashboard.top_performing_rules {
    println!("  {}: {:.2}", rule.rule_id, rule.quality_score);
}

println!("Quality alerts:");
for alert in &dashboard.alerts {
    println!("  {}: {} ({})", alert.severity, alert.message, alert.rule_id);
}
```

### Rule-Specific Analytics

```rust
let analytics = engine.get_rule_analytics("ai-security-xss-detection").unwrap();

println!("Rule Analytics:");
println!("  Total executions: {}", analytics.total_executions);
println!("  Success rate: {:.2}", analytics.success_rate);
println!("  Average execution time: {:?}", analytics.average_execution_time);
println!("  Usage frequency: {:.2}", analytics.usage_frequency);

if let Some(trend) = analytics.quality_trend {
    println!("  Quality trend: {:.2}", trend.confidence_trend);
    println!("  Satisfaction trend: {:.2}", trend.satisfaction_trend);
}
```

### Exporting Analytics

```rust
let export = engine.export_analytics()?;

// Save to file
let json = serde_json::to_string_pretty(&export)?;
std::fs::write("analytics_export.json", json)?;

println!("Exported analytics for {} rules", export.rule_analytics.len());
```

## Best Practices

### 1. Rule Configuration

- Start with higher confidence thresholds (0.8+) for production
- Use lower thresholds (0.6-0.7) for development and learning
- Enable fallback to static analysis for critical rules
- Set appropriate context token limits based on file sizes

### 2. Learning and Adaptation

- Collect feedback consistently from developers
- Review adaptation suggestions before auto-applying
- Monitor quality metrics regularly
- Use learning data to improve rule prompts

### 3. Performance Optimization

- Limit concurrent AI requests based on your API limits
- Use caching for repeated analysis of similar code
- Consider batching small files for better throughput
- Monitor resource usage and adjust accordingly

### 4. Quality Assurance

- Set up alerts for declining rule performance
- Regular review of false positive rates
- Monitor user satisfaction scores
- Validate AI rule effectiveness against static analysis

## Examples

### Custom Security Rule

```rust
use moon_shine::rulebase::{RuleImplementation, AIContextRequirements};
use moon_shine::rule_types::{RuleMetadata, RuleCategory, RuleSeverity, FixStatus};

let custom_security_rule = RuleMetadata {
    id: "custom-auth-bypass".to_string(),
    name: "Authentication Bypass Detection".to_string(),
    description: "Detects patterns that might bypass authentication".to_string(),
    category: RuleCategory::Security,
    severity: RuleSeverity::Error,
    fix_status: FixStatus::Manual,
    ai_enhanced: true,
    cost: 70,
    tags: vec!["security".to_string(), "authentication".to_string()],
    dependencies: vec![],
    implementation: RuleImplementation::AiBehavioral {
        pattern_type: "auth_bypass".to_string(),
        ai_prompt: Some(r#"
            Analyze this code for authentication bypass vulnerabilities. Look for:
            1. Direct access to protected resources without auth checks
            2. Bypassable authentication conditions
            3. Hardcoded credentials or tokens
            4. Insecure session management
            5. Missing authorization checks

            Focus on actual security vulnerabilities, not style issues.
        "#.to_string()),
        confidence_threshold: Some(0.85),
        model_preference: Some("claude".to_string()),
        context_requirements: Some(AIContextRequirements {
            needs_ast: true,
            needs_dependencies: true,
            needs_project_context: true,
            max_context_tokens: 10000,
        }),
    },
    config_schema: None,
};

engine.load_rules(vec![custom_security_rule]);
```

### Performance Monitoring Setup

```rust
use moon_shine::rulebase::{MetricsConfig, QualityMetricsTracker};

let metrics_config = MetricsConfig {
    enabled: true,
    collection_interval_seconds: 30,
    retention_days: 180, // Keep 6 months of data
    aggregation_window_minutes: 5,
    export_enabled: true,
    real_time_monitoring: true,
};

// Monitor performance continuously
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(60));

    loop {
        interval.tick().await;

        if let Some(dashboard) = engine.get_quality_dashboard() {
            // Check for performance issues
            for alert in dashboard.alerts {
                if matches!(alert.severity, AlertSeverity::High | AlertSeverity::Critical) {
                    eprintln!("ALERT: {}", alert.message);
                    // Send notification, log, etc.
                }
            }

            // Auto-apply low-risk adaptations
            if let Ok(applied) = engine.apply_adaptations().await {
                if !applied.is_empty() {
                    println!("Auto-applied {} rule adaptations", applied.len());
                }
            }
        }
    }
});
```

### Integration with CI/CD

```rust
// ci_integration.rs
use moon_shine::rulebase::{IntegratedRuleEngine, IntegratedEngineConfig};

pub async fn run_ci_analysis() -> Result<bool, Box<dyn std::error::Error>> {
    let config = IntegratedEngineConfig {
        enable_ai_rules: true,
        enable_learning: false, // Disable learning in CI
        enable_adaptation: false, // Disable adaptation in CI
        ai_timeout_seconds: 60, // Longer timeout for CI
        ..IntegratedEngineConfig::default()
    };

    let mut engine = IntegratedRuleEngine::new(config);

    // Load production rule set
    engine.load_rules_from_templates(&[
        "ai-security-xss-detection",
        "ai-security-sql-injection",
        "ai-performance-memory-leak",
        "ai-quality-code-duplication",
    ], RuleSeverity::Error, 50);

    // Analyze all TypeScript/JavaScript files
    let files = find_source_files()?;
    let results = engine.analyze_batch(&files, None).await?;

    let mut has_errors = false;
    let mut total_issues = 0;

    for (i, result) in results.iter().enumerate() {
        let file_path = &files[i].0;
        let errors: Vec<_> = result.diagnostics.iter()
            .filter(|d| matches!(d.severity, DiagnosticSeverity::Error))
            .collect();

        if !errors.is_empty() {
            has_errors = true;
            println!("❌ {}: {} errors", file_path, errors.len());

            for error in errors {
                println!("   {}:{} - {} ({})",
                    error.line, error.column, error.message, error.rule_name);
            }
        }

        total_issues += result.diagnostics.len();
    }

    println!("Analysis complete: {} total issues found across {} files",
        total_issues, results.len());

    if has_errors {
        println!("❌ CI check failed due to rule violations");
        return Ok(false);
    }

    println!("✅ All checks passed");
    Ok(true)
}

fn find_source_files() -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    // Implementation to find and read source files
    todo!("Implement file discovery")
}
```

This comprehensive guide should help you get started with the AI-enhanced rule system and understand how to leverage its full capabilities for improved code analysis.