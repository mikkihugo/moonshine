/**
 * Modern CLI Action Handler with Modular Engine Architecture
 * Following Rule C005: Single responsibility - handle CLI execution flow with new architecture
 * Following Rule C014: Dependency injection - inject orchestrator v2
 * Following Rule C015: Use domain language - clear analysis and orchestration terms
 */

const chalk = require('chalk');
const fs = require('fs');
const ConfigManager = require('./config-manager');
const RuleSelectionService = require('./rule-selection-service');
const AnalysisOrchestrator = require('./analysis-orchestrator');
const OutputService = require('./output-service');
const GitUtils = require('./git-utils');
const FileTargetingService = require('./file-targeting-service');

// Legacy orchestrator for fallback
// const LegacyOrchestrator = require('./legacy-analysis-orchestrator'); // Removed

class CliActionHandler {
  constructor(options = {}) {
    this.options = options;
    this.configManager = null;
    this.ruleSelectionService = new RuleSelectionService();
    
    // Use new orchestrator by default, fallback to legacy if needed
    this.orchestrator = new AnalysisOrchestrator();
      
    this.outputService = new OutputService(options);
    this.fileTargetingService = new FileTargetingService();
    
    this.isModernMode = !options.useLegacy;
  }

  /**
   * Execute CLI analysis with new architecture
   * Following Rule C006: Verb-noun naming
   */
  async execute() {
    try {
      this.displayModernBanner();
      this.handleShortcuts();
      
      // Load configuration
      const config = await this.loadConfiguration();
      
      // Validate input with priority system
      this.validateInput(config);

      // Show dry run preview if requested
      if (this.options.dryRun) {
        // We need to get rules first for accurate dry run info
        const rulesToRun = await this.ruleSelectionService.selectRules(config, this.options);
        await this.showDryRunPreview(config, rulesToRun);
        return;
      }

      // Select rules to run
      const rulesToRun = await this.ruleSelectionService.selectRules(config, this.options);
      
      if (rulesToRun.length === 0) {
        console.log(chalk.yellow('⚠️  No rules to run'));
        return;
      }

      // Apply enhanced file targeting
      const targetingResult = await this.applyFileTargeting(config);
      if (targetingResult.files.length === 0) {
        console.log(chalk.yellow('⚠️  No files to analyze after applying filters'));
        this.displayTargetingStats(targetingResult.stats);
        return;
      }

      // Update options with filtered files
      this.options.targetFiles = targetingResult.files;

      // Display analysis info
      this.displayAnalysisInfo(rulesToRun, targetingResult);

      // Run analysis with appropriate orchestrator
      const startTime = Date.now();
      const results = await this.runModernAnalysis(rulesToRun, targetingResult.files, config);
      const duration = Date.now() - startTime;

      // Output results
      await this.outputService.outputResults(results, this.options, { 
        duration,
        rulesRun: rulesToRun.length 
      });

      // Exit with appropriate code
      this.handleExit(results);
      
    } catch (error) {
      console.error(chalk.red('❌ Sun Lint Error:'), error.message);
      
      // Following Rule C035: Log complete error context
      if (this.options.debug) {
        console.error('Full error context:', {
          message: error.message,
          stack: error.stack,
          options: this.options,
          mode: this.isModernMode ? 'modern' : 'legacy'
        });
      }
      
      process.exit(1);
    }
  }

  /**
   * Run analysis using modern orchestrator
   */
  async runModernAnalysis(rulesToRun, files, config) {
    if (this.isModernMode) {
      console.log(chalk.blue('🚀 Using modern engine architecture'));
      
      // Initialize orchestrator with configuration including targetFiles for optimization
      await this.orchestrator.initialize({
        enabledEngines: this.determineEnabledEngines(config),
        aiConfig: config.ai || {},
        eslintConfig: config.eslint || {},
        heuristicConfig: { 
          ...config.heuristic || {},
          targetFiles: this.options.targetFiles,  // Pass filtered files for semantic optimization
          maxSemanticFiles: this.options.maxSemanticFiles !== undefined ? parseInt(this.options.maxSemanticFiles) : 1000,  // Configurable semantic file limit
          verbose: this.options.verbose  // Pass verbose for debugging
        }
      });
      
      if (this.options.verbose) {
        console.log(`🔧 Debug: maxSemanticFiles option = ${this.options.maxSemanticFiles}`);
        console.log(`🔧 Debug: parsed maxSemanticFiles = ${this.options.maxSemanticFiles !== undefined ? parseInt(this.options.maxSemanticFiles) : 1000}`);
      }

      // Run analysis with new orchestrator
      const results = await this.orchestrator.analyze(files, rulesToRun, {
        ...this.options,
        timeout: parseInt(this.options.timeout) || 30000,
        config: {
          ...config,
          verbose: this.options.verbose,
          quiet: this.options.quiet,
          // Pass requested engine to enable strict engine mode (no fallback)
          requestedEngine: this.options.engine,
          // Performance optimization settings
          performanceMode: this.options.performanceMode,
          ruleBatchSize: parseInt(this.options.ruleBatchSize) || 10,
          fileBatchSize: parseInt(this.options.fileBatchSize) || 50,
          maxFiles: parseInt(this.options.maxFiles) || 1000,
          enableFileFiltering: !this.options.noFileFiltering,
          enableBatching: !this.options.noBatching
        }
      });
      return results;
    }
  }

  /**
   * Determine which engines to enable based on configuration
   * Following Rule C006: Verb-noun naming
   * Following Rule C031: Separate validation logic
   */
  determineEnabledEngines(config) {
    // If specific engine is requested via --engine option, use only that engine
    if (this.options.engine) {
      // Handle "auto" engine selection
      if (this.options.engine === 'auto') {
        // Auto-select best engines: default to heuristic for compatibility
        const autoEngines = ['heuristic'];
        
        // Add ESLint for JS/TS files if available
        if (this.hasJavaScriptTypeScriptFiles() || config.eslint?.enabled !== false) {
          autoEngines.push('eslint');
        }
        
        return autoEngines;
      }
      
      // Return specific engine as requested
      return [this.options.engine];
    }

    const enabledEngines = [];

    // Always enable heuristic engine for pattern-based rules
    enabledEngines.push('heuristic');

    // Enable ESLint engine for JS/TS files
    if (this.hasJavaScriptTypeScriptFiles() || config.eslint?.enabled !== false) {
      enabledEngines.push('eslint');
    }

    // Enable OpenAI engine if AI is configured and requested
    if (this.options.ai || config.ai?.enabled) {
      if (this.validateAIConfiguration(config)) {
        enabledEngines.push('openai');
      } else {
        console.warn(chalk.yellow('⚠️ AI requested but not properly configured, skipping AI analysis'));
      }
    }

    return enabledEngines;
  }

  /**
   * Validate AI configuration
   * Following Rule C006: Verb-noun naming
   * Following Rule C031: Separate validation logic
   */
  validateAIConfiguration(config) {
    const aiConfig = config.ai || {};
    
    // Check for API key
    if (!aiConfig.apiKey && !process.env.OPENAI_API_KEY) {
      console.warn(chalk.yellow('⚠️ No OpenAI API key found in config or environment'));
      return false;
    }

    // Check for model configuration
    if (!aiConfig.model && !process.env.OPENAI_MODEL) {
      // Use default model
      return true;
    }

    return true;
  }

  /**
   * Check if target files contain JS/TS files
   * Following Rule C006: Verb-noun naming
   */
  hasJavaScriptTypeScriptFiles() {
    if (!this.options.targetFiles) return false;
    
    return this.options.targetFiles.some(file => {
      const ext = require('path').extname(file).toLowerCase();
      return ['.js', '.jsx', '.ts', '.tsx', '.mjs', '.cjs'].includes(ext);
    });
  }

  /**
   * Display modern banner with architecture info
   * Following Rule C006: Verb-noun naming
   */
  displayModernBanner() {
    // Skip banner in quiet mode or JSON format
    if (this.options.quiet || this.options.format === 'json') {
      return;
    }
    
    const { version } = require('../package.json');
    console.log(chalk.yellow.bold('☀️  Sun Lint - Modular Analysis Engine'));
    console.log(chalk.gray(`Version: ${version} | Mode: ${this.isModernMode ? 'Modern' : 'Legacy'} | Sun* Engineering`));
    
    if (this.options.debug) {
      console.log(chalk.yellow('Debug mode enabled'));
      console.log('Architecture:', this.isModernMode ? 'Plugin-based' : 'Legacy');
      console.log('Options:', this.options);
    }
    console.log();
  }

  // Delegate methods to base functionality (same as original CliActionHandler)
  
  /**
   * Load configuration using existing config manager
   * Following Rule C006: Verb-noun naming
   */
  async loadConfiguration() {
    this.configManager = new ConfigManager();
    return await this.configManager.loadConfig(this.options.config, this.options);
  }

  /**
   * Validate CLI input
   * Following Rule C006: Verb-noun naming
   * Following Rule C031: Separate validation logic
   */
  /**
   * Validate CLI input with priority system
   * Priority: CLI > Config File > Default
   * Following Rule C006: Verb-noun naming
   * Following Rule C031: Separate validation logic
   */
  validateInput(config) {
    // Validate engine option if specified (check this first, always)
    if (this.options.engine) {
      const validEngines = ['auto', 'eslint', 'heuristic', 'openai'];
      if (!validEngines.includes(this.options.engine)) {
        throw new Error(
          chalk.red(`❌ Invalid engine: ${this.options.engine}\n`) +
          chalk.gray(`Valid engines: ${validEngines.join(', ')}`)
        );
      }
    }

    // Priority 1: CLI --input parameter (highest priority)
    if (this.options.input) {
      // Validate CLI input path exists
      if (!fs.existsSync(this.options.input)) {
        throw new Error(
          chalk.red(`❌ Input path does not exist: ${this.options.input}\n`) +
          chalk.gray('Please check the path and try again.')
        );
      }
      return; // CLI input is valid, use it
    }

    // Priority 2: Config file 'include' field
    if (config && config.include && Array.isArray(config.include) && config.include.length > 0) {
      // Config provides include patterns, use current directory as base
      // Let FileTargetingService handle the include patterns from config
      this.options.input = '.'; 
      
      if (this.options.verbose) {
        console.log(chalk.gray(`ℹ️  Using config include patterns: ${config.include.join(', ')}`));
      }
      return;
    }

    // Priority 3: Default behavior (fallback)
    if (!this.options.input && (!config || !config.include)) {
      // Set default input directory instead of glob patterns
      this.options.input = '.'; // Current directory, let FileTargetingService handle patterns
      
      if (this.options.verbose) {
        console.log(chalk.gray('ℹ️  Using default input: current directory with JS/TS file patterns'));
      }
      return;
    }
  }

  /**
   * Handle CLI shortcuts
   * Following Rule C006: Verb-noun naming
   */
  handleShortcuts() {
    // Handle version shortcut
    if (this.options.version) {
      const { version } = require('../package.json');
      console.log(version);
      process.exit(0);
    }

    // Handle list-rules shortcut  
    if (this.options.listRules) {
      // This could be enhanced to use the new orchestrator's rule discovery
      console.log('Available rules will be listed here...');
      process.exit(0);
    }
  }

  /**
   * Get preset name based on CLI options
   */
  getPresetName() {
    if (this.options.all) return 'all preset';
    if (this.options.quality) return 'quality preset';
    if (this.options.security) return 'security preset';
    if (this.options.category) return `${this.options.category} preset`;
    if (this.options.rule) return 'single rule';
    if (this.options.rules) return 'multiple rules';
    return 'config-based';
  }

  /**
   * Show dry run preview
   * Following Rule C006: Verb-noun naming
   */
  async showDryRunPreview(config, rulesToRun = null) {
    console.log(chalk.blue('🔍 Dry Run Preview'));
    console.log(chalk.gray('This would analyze the following configuration:'));
    
    let rulesInfo;
    if (rulesToRun) {
      rulesInfo = `${rulesToRun.length} rules (${this.getPresetName()})`;
    } else {
      rulesInfo = this.options.rules || 'config-based';
    }
    
    console.log(JSON.stringify({
      rules: rulesInfo,
      files: this.options.targetFiles?.length || 'auto-detected',
      engines: this.determineEnabledEngines(config),
      mode: this.isModernMode ? 'modern' : 'legacy'
    }, null, 2));
  }

  /**
   * Apply file targeting logic
   * Following Rule C006: Verb-noun naming
   */
  async applyFileTargeting(config) {
    // Handle both string and array input patterns
    const inputPaths = Array.isArray(this.options.input) 
      ? this.options.input 
      : [this.options.input];
    
    return await this.fileTargetingService.getTargetFiles(inputPaths, config, this.options);
  }

  /**
   * Display analysis information
   * Following Rule C006: Verb-noun naming
   */
  displayAnalysisInfo(rulesToRun, targetingResult) {
    if (this.options.quiet) return;

    console.log(chalk.blue('📊 Analysis Configuration:'));
    console.log(`• Rules: ${rulesToRun.length} selected`);
    console.log(`• Files: ${targetingResult.files.length} targeted`);
    console.log(`• Architecture: ${this.isModernMode ? 'Modern Plugin-based' : 'Legacy'}`);
    
    if (this.options.debug) {
      console.log(`• Rules: ${rulesToRun.map(r => r.id).join(', ')}`);
    }
    console.log();
  }

  /**
   * Display targeting statistics
   * Following Rule C006: Verb-noun naming
   */
  displayTargetingStats(stats) {
    if (this.options.quiet) return;
    
    console.log(chalk.gray('Targeting Stats:'));
    Object.entries(stats).forEach(([key, value]) => {
      console.log(`• ${key}: ${value}`);
    });
  }

  /**
   * Handle process exit based on results
   * Following Rule C006: Verb-noun naming
   */
  handleExit(results) {
    if (this.options.noExit) return;
    
    // Check if any violations were found
    const hasViolations = results.results?.some(result => 
      result.violations && result.violations.length > 0
    );
    
    if (hasViolations && this.options.failOnViolations !== false) {
      process.exit(1);
    } else {
      process.exit(0);
    }
  }
}

module.exports = CliActionHandler;
