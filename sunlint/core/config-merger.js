const { minimatch } = require('minimatch');
const chalk = require('chalk');

/**
 * Handles configuration merging and CLI overrides
 * Rule C005: Single responsibility - chỉ merge và override config
 * Rule C015: Domain language - ConfigMerger
 */
class ConfigMerger {
  
  /**
   * Rule C006: mergeConfigurations - verb-noun naming
   * Rule C012: Pure function - no side effects, clear input/output
   */
  mergeConfigurations(base, override) {
    const merged = { ...base };

    for (const [key, value] of Object.entries(override)) {
      if (key === 'rules' && typeof value === 'object') {
        if (Array.isArray(value)) {
          // Convert array of rule names to object format
          const rulesObj = {};
          value.forEach(rule => {
            rulesObj[rule] = 'warn';
          });
          merged.rules = { ...merged.rules, ...rulesObj };
        } else {
          merged.rules = { ...merged.rules, ...value };
        }
      } else if (key === 'categories' && typeof value === 'object') {
        merged.categories = { ...merged.categories, ...value };
      } else if (typeof value === 'object' && !Array.isArray(value)) {
        merged[key] = { ...merged[key], ...value };
      } else {
        merged[key] = value;
      }
    }

    return merged;
  }

  /**
   * Rule C006: applyCLIOverrides - verb-noun naming
   * Rule C012: Pure function - no side effects
   */
  applyCLIOverrides(config, options) {
    const overrides = { ...config };

    // Rules override - only override if config file didn't specify rules
    if (options.rules && (!config.rules || Object.keys(config.rules).length === 0)) {
      if (typeof options.rules === 'string') {
        const ruleList = options.rules.split(',').map(r => r.trim());
        overrides.rules = {};
        ruleList.forEach(rule => {
          overrides.rules[rule] = 'warn';
        });
      } else if (Array.isArray(options.rules)) {
        overrides.rules = {};
        options.rules.forEach(rule => {
          overrides.rules[rule] = 'warn';
        });
      }
    }

    // Include/Input override - only if config file didn't specify
    if (options.include && (!config.include || config.include.length === 0)) {
      overrides.include = Array.isArray(options.include) ? options.include : [options.include];
    }
    if (options.input && (!config.input || config.input.length === 0)) {
      overrides.input = Array.isArray(options.input) ? options.input : [options.input];
    }

    // Exclude override - merge with config file
    if (options.exclude) {
      const excludeList = Array.isArray(options.exclude) ? options.exclude : [options.exclude];
      overrides.exclude = [...new Set([...(config.exclude || []), ...excludeList])];
    }

    // Languages override
    if (options.languages) {
      overrides.languages = options.languages.split(',').map(l => l.trim());
    }

    // Output format override
    if (options.format) {
      overrides.output = { ...overrides.output, format: options.format };
    }

    // AI overrides
    overrides.ai = this.applyAIOverrides(overrides.ai, options);

    // Performance overrides
    overrides.performance = this.applyPerformanceOverrides(overrides.performance, options);

    // Auto-expand include patterns if input doesn't match any existing patterns
    const expandedOverrides = this.autoExpandIncludePatterns(overrides, options);
    // Copy expanded properties back to overrides
    Object.assign(overrides, expandedOverrides);

    return overrides;
  }

  /**
   * Rule C006: applyAIOverrides - verb-noun naming
   * Rule C005: Extracted for single responsibility
   */
  applyAIOverrides(aiConfig, options) {
    const ai = { ...aiConfig };

    if (options.ai === true) {
      ai.enabled = true;
    }
    if (options.ai === false) {
      ai.enabled = false;
    }

    return ai;
  }

  /**
   * Rule C006: applyPerformanceOverrides - verb-noun naming
   * Rule C005: Extracted for single responsibility
   */
  applyPerformanceOverrides(performanceConfig, options) {
    const performance = { ...performanceConfig };

    if (options.maxConcurrent) {
      performance.maxConcurrentRules = parseInt(options.maxConcurrent);
    }

    if (options.timeout) {
      performance.timeoutMs = parseInt(options.timeout);
    }

    if (options.cache === false) {
      performance.cacheEnabled = false;
    }

    return performance;
  }

  /**
   * Rule C006: applyEnvironmentVariables - verb-noun naming
   */
  applyEnvironmentVariables(config) {
    const updatedConfig = { ...config };

    // SUNLINT_RULES environment variable
    if (process.env.SUNLINT_RULES) {
      const envRules = {};
      process.env.SUNLINT_RULES.split(',').forEach(rule => {
        const [ruleId, severity] = rule.trim().split(':');
        envRules[ruleId] = severity || 'error';
      });
      updatedConfig.rules = { ...updatedConfig.rules, ...envRules };
    }

    // SUNLINT_AI_ENABLED environment variable
    if (process.env.SUNLINT_AI_ENABLED) {
      updatedConfig.ai = updatedConfig.ai || {};
      updatedConfig.ai.enabled = process.env.SUNLINT_AI_ENABLED === 'true';
    }

    // SUNLINT_LANGUAGES environment variable
    if (process.env.SUNLINT_LANGUAGES) {
      updatedConfig.languages = process.env.SUNLINT_LANGUAGES.split(',').map(l => l.trim());
    }

    return updatedConfig;
  }

  /**
   * Rule C006: autoExpandIncludePatterns - verb-noun naming
   * Rule C005: Single responsibility - auto-expand include patterns for input paths
   */
  autoExpandIncludePatterns(config, options) {
    if (!options.input) {
      return config;
    }

    const result = { ...config };
    const inputPaths = Array.isArray(options.input) ? options.input : [options.input];
    const currentInclude = result.include || [];
    
    console.log(chalk.gray(`🔍 AUTO-EXPANSION: Checking input paths: ${inputPaths.join(', ')}`));
    console.log(chalk.gray(`🔍 AUTO-EXPANSION: Current include patterns: ${currentInclude.join(', ')}`));
    
    let needsExpansion = false;
    for (const inputPath of inputPaths) {
      const matchesExisting = currentInclude.some(pattern => {
        const match1 = minimatch(inputPath, pattern);
        const match2 = minimatch(inputPath + '/**', pattern);
        const match3 = minimatch('**/' + inputPath, pattern);
        const match4 = minimatch('**/' + inputPath + '/**', pattern);
        
        console.log(chalk.gray(`  AUTO-EXPANSION: ${inputPath} vs ${pattern}: ${match1 || match2 || match3 || match4}`));
        
        return match1 || match2 || match3 || match4;
      });
      
      if (!matchesExisting) {
        needsExpansion = true;
        console.log(chalk.gray(`  🔄 AUTO-EXPANSION: ${inputPath} needs expansion`));
        break;
      }
    }
    
    if (needsExpansion) {
      // Add flexible patterns for input paths
      const expandedInclude = [...currentInclude];
      for (const inputPath of inputPaths) {
        // Check if inputPath is a file or directory
        const fs = require('fs');
        const path = require('path');
        
        try {
          const resolvedPath = path.resolve(inputPath);
          if (fs.existsSync(resolvedPath)) {
            const stat = fs.statSync(resolvedPath);
            if (stat.isFile()) {
              // For files, add the exact path
              expandedInclude.push(inputPath);
              expandedInclude.push('**/' + inputPath);
            } else if (stat.isDirectory()) {
              // For directories, add recursive patterns
              expandedInclude.push(inputPath + '/**');
              expandedInclude.push('**/' + inputPath + '/**');
            }
          } else {
            // If path doesn't exist, assume it's a pattern and add both file and directory variants
            expandedInclude.push(inputPath);
            expandedInclude.push(inputPath + '/**');
            expandedInclude.push('**/' + inputPath);
            expandedInclude.push('**/' + inputPath + '/**');
          }
        } catch (error) {
          // Fallback to original logic if file system check fails
          expandedInclude.push(inputPath + '/**');
          expandedInclude.push('**/' + inputPath + '/**');
        }
      }
      result.include = expandedInclude;
      
      console.log(chalk.gray(`📁 AUTO-EXPANSION: Auto-expanded include patterns: ${expandedInclude.join(', ')}`));
    } else {
      console.log(chalk.gray(`✅ AUTO-EXPANSION: No expansion needed`));
    }

    return result;
  }

  /**
   * Rule C006: processIgnorePatterns - verb-noun naming
   * Convert deprecated ignorePatterns to exclude for backward compatibility
   */
  processIgnorePatterns(config) {
    if (config.ignorePatterns && config.ignorePatterns.length > 0) {
      console.warn('⚠️  DEPRECATED: "ignorePatterns" is deprecated. Please use "exclude" instead.');
      
      // Initialize exclude if it doesn't exist
      if (!config.exclude) {
        config.exclude = [];
      }
      
      // Merge ignorePatterns into exclude and remove duplicates
      config.exclude = [...new Set([...config.exclude, ...config.ignorePatterns])];
      
      // Remove the deprecated property
      delete config.ignorePatterns;
    }
    return config;
  }
}

module.exports = ConfigMerger;
