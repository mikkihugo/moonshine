/**
 * Analysis Orchestrator - Plugin-Based Architecture
 * Following Rule C005: Single responsibility - orchestrate analysis plugins
 * Following Rule C014: Dependency injection - inject analysis engines
 * Following Rule C015: Use domain language - clear orchestration terms
 */

const chalk = require('chalk');
const path = require('path');
const fs = require('fs');
const AnalysisEngineInterface = require('./interfaces/analysis-engine.interface');
const SunlintRuleAdapter = require('./adapters/sunlint-rule-adapter');
const PerformanceOptimizer = require('./performance-optimizer');

class AnalysisOrchestrator {
  constructor() {
    this.engines = new Map(); // Plugin registry
    this.initialized = false;
    this.defaultTimeout = 30000; // 30 seconds default timeout
    this.ruleAdapter = SunlintRuleAdapter.getInstance();
    this.enginesConfigPath = path.join(__dirname, '..', 'config', 'engines', 'engines.json');
    this.performanceOptimizer = new PerformanceOptimizer();
  }

  /**
   * Auto-load engines from configuration
   * Following Rule C006: Verb-noun naming
   * @param {Object} config - Configuration for engines
   */
  async loadEnginesFromConfig(config = {}) {
    try {
      // Load engines config
      let enginesConfig = {};
      if (fs.existsSync(this.enginesConfigPath)) {
        const configData = fs.readFileSync(this.enginesConfigPath, 'utf8');
        enginesConfig = JSON.parse(configData);
      }

      // Load each configured engine
      const enabledEngines = config.enabledEngines || 
        enginesConfig.defaultEngines || 
        Object.keys(enginesConfig.engines || {});

      for (const engineName of enabledEngines) {
        const engineConfig = enginesConfig.engines?.[engineName];
        if (!engineConfig || !engineConfig.enabled) {
          continue;
        }

        try {
          // Load engine module
          const enginePath = path.resolve(__dirname, '..', engineConfig.path);
          const EngineClass = require(enginePath);
          
          // Create and register engine instance
          const engine = new EngineClass();
          this.registerEngine(engine);
          
        } catch (error) {
          console.warn(chalk.yellow(`⚠️ Failed to load engine ${engineName}:`), error.message);
        }
      }
      
    } catch (error) {
      console.warn(chalk.yellow(`⚠️ Failed to load engines config:`), error.message);
      // Fall back to manual registration if config fails
    }
  }

  /**
   * Register an analysis engine plugin
   * Following Rule C014: Dependency injection - register engines
   * Following Rule C006: Verb-noun naming
   * @param {AnalysisEngineInterface} engine - Engine to register
   * @param {Object} options - Options including verbose flag
   */
  registerEngine(engine, options = {}) {
    if (!(engine instanceof AnalysisEngineInterface)) {
      throw new Error('Engine must implement AnalysisEngineInterface');
    }
    
    if (this.engines.has(engine.id)) {
      if (options.verbose) {
        console.warn(chalk.yellow(`⚠️ Engine ${engine.id} already registered, replacing...`));
      }
    }
    
    this.engines.set(engine.id, engine);
    
    if (options.verbose) {
      console.log(chalk.green(`✅ Registered engine: ${engine.id} v${engine.version}`));
      
      if (!this.initialized) {
        console.log(chalk.gray(`   Supported languages: ${engine.supportedLanguages.join(', ')}`));
        // Note: Supported rules count will be shown after initialization
      }
    }
  }

  /**
   * Initialize all registered engines
   * Following Rule C006: Verb-noun naming
   * @param {Object} config - Configuration for engines
   */
  async initialize(config) {
    if (this.initialized) {
      return;
    }

    // Auto-load engines from config if none are registered
    if (this.engines.size === 0) {
      await this.loadEnginesFromConfig(config);
    }

    if (config.verbose) {
      console.log(chalk.blue(`🔧 Initializing ${this.engines.size} analysis engines...`));
    }

    // Initialize rule adapter
    await this.ruleAdapter.initialize();
    
    // Initialize enabled engines
    const enabledEngines = config.enabledEngines || Array.from(this.engines.keys());
    
    for (const engineName of enabledEngines) {
      const engine = this.engines.get(engineName);
      if (!engine) {
        if (config.verbose) {
          console.warn(chalk.yellow(`⚠️ Engine ${engineName} not registered, skipping...`));
        }
        continue;
      }

      try {
        await engine.initialize(config[`${engineName}Config`] || {});
        if (config.verbose) {
          console.log(chalk.green(`✅ ${engineName} engine initialized`));
          console.log(chalk.gray(`   Supported rules: ${engine.getSupportedRules().length} rules`));
        }
      } catch (error) {
        console.error(chalk.red(`❌ Failed to initialize ${engineName} engine:`), error.message);
        
        // Remove failed engine from registry
        this.engines.delete(engineName);
      }
    }

    if (this.engines.size === 0) {
      throw new Error('No analysis engines successfully initialized');
    }

    this.initialized = true;
    if (config.verbose) {
      console.log(chalk.blue(`🚀 All engines initialized successfully`));
    }
  }  /**
   * Run analysis using appropriate engines
   * Following Rule C005: Single responsibility - orchestrate analysis
   * Following Rule C006: Verb-noun naming
   * @param {Object[]} rulesToRun - Rules to analyze
   * @param {Object} options - Analysis options
   * @param {Object} config - Configuration
   * @returns {Promise<Object>} Combined analysis results
   */
  async runAnalysis(rulesToRun, options, config = {}) {
    try {
      // Initialize engines if not already done
      if (!this.initialized) {
        await this.initialize(config);
      }
      
      if (this.engines.size === 0) {
        throw new Error('No analysis engines registered');
      }

      // Initialize performance optimizer
      await this.performanceOptimizer.initialize(config);

      // Apply performance optimizations to files and rules
      const { optimizedFiles, optimizedRules, performanceMetrics } = 
        await this.performanceOptimizer.optimizeAnalysis(
          options.files || [options.input], 
          rulesToRun, 
          config
        );

      if (!options.quiet) {
        console.log(chalk.cyan(`🔍 Analyzing ${optimizedRules.length} rules on ${optimizedFiles.length} files...`));
        if (performanceMetrics.filteredFiles > 0) {
          console.log(chalk.gray(`   📦 Filtered ${performanceMetrics.filteredFiles} files for performance`));
        }
        if (performanceMetrics.ruleBatches > 1) {
          console.log(chalk.gray(`   🔄 Using ${performanceMetrics.ruleBatches} rule batches`));
        }
      }

      // Group rules by their preferred engines
      const engineGroups = this.groupRulesByEngine(optimizedRules, config);
      
      if (!options.quiet) {
        console.log(chalk.cyan(`🚀 Running analysis across ${engineGroups.size} engines...`));
      }

      // Run analysis on each engine with batching
      const results = [];
      for (const [engineName, rules] of engineGroups) {
        const engine = this.engines.get(engineName);
        if (!engine) {
          console.warn(chalk.yellow(`⚠️ Engine ${engineName} not found, skipping ${rules.length} rules`));
          continue;
        }

        // Process rules in batches for performance
        const ruleBatches = this.performanceOptimizer.createRuleBatches(rules, config);
        
        for (let i = 0; i < ruleBatches.length; i++) {
          const batch = ruleBatches[i];
          const batchNumber = i + 1;
          
          if (!options.quiet && ruleBatches.length > 1) {
            console.log(chalk.blue(`⚙️ ${engineName} - Batch ${batchNumber}/${ruleBatches.length}: ${batch.length} rules`));
          } else if (!options.quiet) {
            console.log(chalk.blue(`⚙️ Running ${batch.length} rules on ${engineName} engine...`));
          }

          try {
            const engineResult = await this.runEngineWithOptimizations(
              engine, 
              optimizedFiles, 
              batch, 
              options,
              { batchNumber, totalBatches: ruleBatches.length }
            );
            
            results.push({
              engine: engineName,
              batch: batchNumber,
              rules: batch.map(r => r.id),
              ...engineResult
            });
            
            if (!options.quiet) {
              const violationCount = this.countViolations(engineResult);
              console.log(chalk.blue(`✅ ${engineName} batch ${batchNumber}: ${violationCount} violations found`));
            }
          } catch (error) {
            // Enhanced error recovery with batch context
            const errorInfo = this.performanceOptimizer.handleAnalysisError(error, {
              engine: engineName,
              batch: batchNumber,
              rules: batch.map(r => r.id),
              files: optimizedFiles.length
            });
            
            console.error(chalk.red(`❌ Engine ${engineName} batch ${batchNumber} failed:`), errorInfo.message);
            
            if (errorInfo.shouldRetry && !options.noRetry) {
              console.log(chalk.yellow(`🔄 Retrying with reduced batch size...`));
              // Split batch and retry - implement recursive retry logic here
            }
            // Continue with other batches instead of failing completely
          }
        }
      }

      // Merge results and add performance metrics
      const mergedResults = this.mergeEngineResults(results, options, optimizedFiles.length);
      mergedResults.performance = performanceMetrics;
      
      return mergedResults;
      
    } catch (error) {
      console.error(chalk.red('❌ Analysis orchestration failed:'), error.message);
      throw error;
    }
  }

  /**
   * Group rules by their preferred analysis engines
   * Following Rule C005: Single responsibility
   * Following Rule C006: Verb-noun naming
   * @param {Object[]} rulesToRun - Rules to group
   * @param {Object} config - Configuration with engine preferences
   * @returns {Map} Map of engine names to rule arrays
   */
  groupRulesByEngine(rulesToRun, config) {
    const groups = new Map();
    const skippedRules = [];
    
    if (config.verbose) {
      console.log(`📊 [Orchestrator] Grouping ${rulesToRun.length} rules by engine...`);
    }
    
    for (const rule of rulesToRun) {
      if (config.verbose) {
        console.log(`🔄 [Orchestrator] Processing rule ${rule.id}`);
      }
      
      // Determine engine preference order
      const enginePreference = this.getEnginePreference(rule, config);
      if (config.verbose) {
        console.log(`🎯 [Orchestrator] Engine preference for ${rule.id}:`, enginePreference);
      }
      
      const selectedEngine = this.selectBestEngine(enginePreference, rule, config);
      
      // If rule is skipped (no engine supports it when specific engine requested)
      if (selectedEngine === null) {
        skippedRules.push(rule);
        if (config.verbose) {
          console.log(`⚠️ [Orchestrator] Skipped rule ${rule.id} - not supported by requested engine`);
        }
        continue;
      }
      
      if (config.verbose) {
        console.log(`✅ [Orchestrator] Selected engine for ${rule.id}: ${selectedEngine}`);
      }
      
      if (!groups.has(selectedEngine)) {
        groups.set(selectedEngine, []);
      }
      groups.get(selectedEngine).push(rule);
    }
    
    // Report skipped rules if any
    if (skippedRules.length > 0) {
      const skippedRuleIds = skippedRules.map(r => r.id).join(', ');
      console.warn(`⚠️ [Orchestrator] Skipped ${skippedRules.length} rules not supported by requested engine: ${skippedRuleIds}`);
    }
    
    if (config.verbose) {
      console.log(`📋 [Orchestrator] Final groups:`, Array.from(groups.entries()).map(([k, v]) => [k, v.length]));
    }
    return groups;
  }

  /**
   * Get engine preference for a rule
   * Following Rule C006: Verb-noun naming
   * @param {Object} rule - Rule object
   * @param {Object} config - Configuration
   * @returns {string[]} Array of engine names in preference order
   */
  getEnginePreference(rule, config) {
    // If user specified a specific engine via --engine option, use only that engine
    if (config.requestedEngine) {
      // Handle "auto" engine selection
      if (config.requestedEngine === 'auto') {
        // Auto-select best engines: default to heuristic, add eslint for JS/TS
        return ['heuristic', 'eslint'];
      }
      
      return [config.requestedEngine];
    }

    // Special preference for C047: Always use semantic analysis (heuristic engine)
    if (rule.id === 'C047') {
      return ['heuristic', 'openai'];
    }

    // Check config-level rule preferences
    const ruleConfig = config.rules?.[rule.id];
    if (ruleConfig?.engines) {
      return ruleConfig.engines;
    }

    // Check CLI --eslint-integration flag (high priority)
    if (config.eslintIntegration) {
      // ESLint integration: prioritize ESLint engine for all rules
      // ESLint engine can handle JS/TS rules, heuristic handles others
      return ['eslint', 'heuristic', 'openai'];
    }
    
    // Check rule analyzer field for compatibility
    if (rule.analyzer) {
      if (rule.analyzer === 'eslint' || rule.analyzer === 'typescript') {
        return ['eslint', 'heuristic'];
      }
      if (rule.analyzer.includes('heuristic')) {
        return ['heuristic', 'openai'];
      }
    }
    
    // Default preference order
    return ['heuristic', 'openai', 'eslint'];
  }

  /**
   * Select best available engine for a rule
   * Following Rule C006: Verb-noun naming
   * @param {string[]} preferences - Engine preferences in order
   * @param {Object} rule - Rule object
   * @param {Object} config - Configuration with verbose flag
   * @returns {string|null} Selected engine name or null if no engine supports the rule
   */
  selectBestEngine(preferences, rule, config) {
    if (config.verbose) {
      console.log(`🎯 [Orchestrator] Selecting engine for rule ${rule.id}, preferences:`, preferences);
    }
    
    // If user specified a specific engine (--engine=eslint), only use that engine
    // Don't fallback to other engines to maintain consistency
    const isSpecificEngineRequested = config.requestedEngine && preferences.length === 1;
    
    for (const engineName of preferences) {
      const engine = this.engines.get(engineName);
      if (config.verbose) {
        console.log(`🔍 [Orchestrator] Checking engine ${engineName}: exists=${!!engine}`);
      }
      
      if (engine && engine.isRuleSupported(rule.id)) {
        if (config.verbose) {
          console.log(`✅ [Orchestrator] Selected engine ${engineName} for rule ${rule.id}`);
        }
        return engineName;
      }
    }
    
    // If specific engine is requested and it doesn't support the rule, skip fallback
    if (isSpecificEngineRequested) {
      if (config.verbose) {
        console.log(`⚠️ [Orchestrator] Rule ${rule.id} not supported by requested engine ${preferences[0]}, skipping`);
      }
      return null; // Skip this rule
    }
    
    if (config.verbose) {
      console.log(`🔄 [Orchestrator] No preferred engine supports ${rule.id}, checking all engines...`);
    }
    
    // Fallback to first available engine that supports the rule (only when no specific engine requested)
    for (const [engineName, engine] of this.engines) {
      if (config.verbose) {
        console.log(`🔍 [Orchestrator] Fallback checking engine ${engineName}`);
      }
      if (engine.isRuleSupported(rule.id)) {
        if (config.verbose) {
          console.log(`✅ [Orchestrator] Fallback selected engine ${engineName} for rule ${rule.id}`);
        }
        return engineName;
      }
    }
    
    if (config.verbose) {
      console.log(`🚨 [Orchestrator] No engine supports rule ${rule.id}, falling back to heuristic`);
    }
    
    // Final fallback to 'heuristic' (most flexible)
    return 'heuristic';
  }

  /**
   * Run engine analysis with timeout protection and performance optimizations
   * Following Rule C006: Verb-noun naming
   * @param {AnalysisEngineInterface} engine - Engine to run
   * @param {string[]} files - Files to analyze
   * @param {Object[]} rules - Rules to apply
   * @param {Object} options - Analysis options
   * @param {Object} batchInfo - Batch context information
   * @returns {Promise<Object>} Engine results
   */
  async runEngineWithOptimizations(engine, files, rules, options, batchInfo = {}) {
    // Dynamic timeout based on file count and rules
    const adaptiveTimeout = this.performanceOptimizer.calculateAdaptiveTimeout(
      files.length, 
      rules.length, 
      options.timeout || this.defaultTimeout
    );
    
    const enhancedOptions = {
      ...options,
      timeout: adaptiveTimeout,
      batchInfo
    };

    try {
      return await Promise.race([
        engine.analyze(files, rules, enhancedOptions),
        new Promise((_, reject) => 
          setTimeout(() => reject(new Error(
            `Engine ${engine.name} batch ${batchInfo.batchNumber || 1} timed out after ${adaptiveTimeout}ms`
          )), adaptiveTimeout)
        )
      ]);
    } catch (error) {
      // Enhanced error context for debugging
      const errorContext = {
        engine: engine.name,
        filesCount: files.length,
        rulesCount: rules.length,
        timeout: adaptiveTimeout,
        batch: batchInfo
      };
      
      // Wrap error with context
      const enhancedError = new Error(`${error.message} (Context: ${JSON.stringify(errorContext)})`);
      enhancedError.originalError = error;
      enhancedError.context = errorContext;
      
      throw enhancedError;
    }
  }

  /**
   * Run engine analysis with timeout protection (legacy method for backward compatibility)
   * Following Rule C006: Verb-noun naming
   * @param {AnalysisEngineInterface} engine - Engine to run
   * @param {string[]} files - Files to analyze
   * @param {Object[]} rules - Rules to apply
   * @param {Object} options - Analysis options
   * @returns {Promise<Object>} Engine results
   */
  async runEngineWithTimeout(engine, files, rules, options) {
    return this.runEngineWithOptimizations(engine, files, rules, options);
  }

  /**
   * Merge results from multiple engines
   * Following Rule C005: Single responsibility
   * Following Rule C006: Verb-noun naming
   * @param {Object[]} engineResults - Results from all engines
   * @param {Object} options - Analysis options
   * @returns {Object} Merged results
   */
  mergeEngineResults(engineResults, options, actualFilesCount = 0) {
    const mergedResults = {
      results: [],
      summary: {
        totalEngines: engineResults.length,
        totalBatches: engineResults.length,
        totalViolations: 0,
        totalFiles: 0,
        engines: {}
      },
      metadata: {
        timestamp: new Date().toISOString(),
        orchestrator: 'sunlint-v2',
        version: '2.0.0'
      }
    };

    // Track unique engines for summary
    const uniqueEngines = new Set();

    // Combine results from all engines
    for (const engineResult of engineResults) {
      uniqueEngines.add(engineResult.engine);
      
      // Add engine-specific results
      if (engineResult.results) {
        mergedResults.results.push(...engineResult.results);
      }
      
      // Track engine statistics
      const violationCount = this.countViolations(engineResult);
      const engineName = engineResult.engine;
      
      if (!mergedResults.summary.engines[engineName]) {
        mergedResults.summary.engines[engineName] = {
          rules: [],
          violations: 0,
          files: 0,
          batches: 0
        };
      }
      
      // Accumulate engine statistics across batches
      mergedResults.summary.engines[engineName].rules.push(...(engineResult.rules || []));
      mergedResults.summary.engines[engineName].violations += violationCount;
      // Don't accumulate filesAnalyzed for each batch - use actual unique file count
      if (!mergedResults.summary.engines[engineName].filesSet) {
        mergedResults.summary.engines[engineName].files = actualFilesCount;
        mergedResults.summary.engines[engineName].filesSet = true;
      }
      mergedResults.summary.engines[engineName].batches += 1;
      
      mergedResults.summary.totalViolations += violationCount;
    }

    // Update unique engine count and correct total files count
    mergedResults.summary.totalEngines = uniqueEngines.size;
    mergedResults.summary.totalFiles = actualFilesCount;

    return mergedResults;
  }

  /**
   * Count violations in engine results
   * Following Rule C006: Verb-noun naming
   * @param {Object} engineResult - Result from an engine
   * @returns {number} Number of violations
   */
  countViolations(engineResult) {
    if (!engineResult.results) return 0;
    
    return engineResult.results.reduce((total, fileResult) => {
      return total + (fileResult.violations?.length || 0);
    }, 0);
  }

  /**
   * Get information about registered engines
   * Following Rule C006: Verb-noun naming
   * @returns {Object} Engine information
   */
  getEngineInfo() {
    const engines = {};
    for (const [name, engine] of this.engines) {
      engines[name] = engine.getEngineInfo();
    }
    return engines;
  }

  /**
   * Get available engines
   * Following Rule C006: Verb-noun naming
   * @returns {string[]} Array of available engine IDs
   */
  getAvailableEngines() {
    return Array.from(this.engines.keys());
  }

  /**
   * Analyze files with rules (main analysis interface)
   * Following Rule C006: Verb-noun naming
   * @param {string[]} files - Files to analyze
   * @param {Object[]} rules - Rules to apply
   * @param {Object} options - Analysis options
   * @returns {Promise<Object>} Analysis results
   */
  async analyze(files, rules, options = {}) {
    // Ensure files are in options for compatibility
    const analysisOptions = { ...options, files };
    
    // Ensure verbose/quiet flags are available in config
    const config = {
      ...options.config || {},
      verbose: options.verbose || options.config?.verbose,
      quiet: options.quiet || options.config?.quiet
    };
    
    return await this.runAnalysis(rules, analysisOptions, config);
  }

  /**
   * Cleanup all engines and performance optimizer
   * Following Rule C006: Verb-noun naming
   * @returns {Promise<void>}
   */
  async cleanup() {
    for (const engine of this.engines.values()) {
      try {
        await engine.cleanup();
      } catch (error) {
        console.warn(chalk.yellow(`⚠️ Failed to cleanup engine ${engine.id}:`), error.message);
      }
    }
    
    // Cleanup performance optimizer
    try {
      await this.performanceOptimizer.cleanup();
    } catch (error) {
      console.warn(chalk.yellow(`⚠️ Failed to cleanup performance optimizer:`), error.message);
    }
    
    this.initialized = false;
    console.log(chalk.blue('🧹 Engine cleanup completed'));
  }
}

module.exports = AnalysisOrchestrator;
