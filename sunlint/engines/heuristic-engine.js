/**
 * Heuristic Analysis Engine Plugin with ts-morph Core Integration
 * Following Rule C005: Single responsibility - Pattern-based analysis with ts-morph core
 * Following Rule C014: Dependency injection - implements interface
 * Following Rule C015: Use domain language - clear heuristic analysis terms
 */

const AnalysisEngineInterface = require('../core/interfaces/analysis-engine.interface');
const ASTModuleRegistry = require('../core/ast-modules/index');
const dependencyChecker = require('../core/dependency-checker');
const SunlintRuleAdapter = require('../core/adapters/sunlint-rule-adapter');
const SemanticEngine = require('../core/semantic-engine');
const SemanticRuleBase = require('../core/semantic-rule-base');
const { getInstance: getUnifiedRegistry } = require('../core/unified-rule-registry');
const AutoPerformanceManager = require('../core/auto-performance-manager');
const fs = require('fs');
const path = require('path');

class HeuristicEngine extends AnalysisEngineInterface {
  constructor() {
    super('heuristic', '4.0', ['typescript', 'javascript', 'dart', 'swift', 'kotlin', 'java', 'python', 'go', 'rust', 'all']);
    
    this.ruleAnalyzers = new Map();
    this.supportedRulesList = [];
    this.ruleAdapter = SunlintRuleAdapter.getInstance();
    this.astRegistry = ASTModuleRegistry;
    
    // ts-morph as core technology for heuristic engine
    // Note: semantic engine will be initialized in initialize() with proper config
    this.semanticEngine = null;
    this.semanticRules = new Map();
    this.symbolTableEnabled = false;
    
    // Unified rule registry
    this.unifiedRegistry = getUnifiedRegistry();
    
    // ✅ PERFORMANCE OPTIMIZATIONS (Integrated)
    this.performanceManager = new AutoPerformanceManager();
    this.performanceConfig = null;
    this.metrics = {
      startTime: null,
      filesProcessed: 0,
      rulesProcessed: 0,
      violationsFound: 0,
      memoryUsage: 0
    };
  }

  /**
   * Initialize Heuristic engine with ts-morph core and configuration
   * ✅ ENHANCED: Now includes performance optimization
   * Following Rule C006: Verb-noun naming
   * @param {Object} config - Engine configuration
   */
  async initialize(config) {
    try {
      // ✅ PERFORMANCE: Get optimal settings based on project
      this.performanceConfig = this.performanceManager.getOptimalSettings(config, config?.targetFiles || []);
      
      // Store verbosity setting
      this.verbose = config?.verbose || false;
      
      if (this.verbose && this.performanceConfig.autoDetected) {
        console.log(`🤖 [HeuristicEngine] Auto-detected performance profile: ${this.performanceConfig.name}`);
        console.log(`   ⚡ Settings: ${this.performanceConfig.timeout/1000}s timeout, ${this.performanceConfig.batchSize || 'auto'} batch size`);
      }
      
      // Initialize unified rule registry
      await this.unifiedRegistry.initialize({ verbose: this.verbose });
      
      // Check for optional AST dependencies
      dependencyChecker.checkAndNotify('ast');
      
      // Initialize ts-morph Symbol Table (core requirement)
      await this.initializeSymbolTable(config);
      
      // Initialize rule adapter
      await this.ruleAdapter.initialize();
      
      // Load available rules from unified registry (OPTIMIZED: skip for performance)
      // Rules will be loaded on-demand in analyze() method 
      if (config.loadAllRules) {
        await this.loadRulesFromRegistry(config);
      } else if (this.verbose) {
        console.log(`⚡ [HeuristicEngine] Skipping bulk rule loading for performance - will load on-demand`);
      }
      
      this.initialized = true;
      if (this.verbose) {
        console.log(`🔍 Heuristic engine v4.0 initialized:`);
        console.log(`   📊 Total rules: ${this.supportedRulesList.length}`);
        console.log(`   🧠 Symbol Table: ${this.symbolTableInitialized ? 'enabled' : 'disabled'}`);
        console.log(`   🔧 Semantic rules: ${this.semanticRules.size}`);
        console.log(`   ⚡ Performance: ${this.performanceConfig.name || 'standard'}`);
      }
      
    } catch (error) {
      console.error('Failed to initialize Heuristic engine:', error.message);
      throw error;
    }
  }

  /**
   * Initialize ts-morph Symbol Table as core requirement
   * OPTIMIZED: Use targeted files instead of entire project for better performance
   */
  async initializeSymbolTable(config) {
    const projectPath = config?.projectPath || process.cwd();
    
    try {
      // Initialize semantic engine with config options including maxSemanticFiles
      const semanticOptions = {
        maxSemanticFiles: config?.maxSemanticFiles,
        verbose: this.verbose,
        ...config?.semanticOptions
      };
      
      this.semanticEngine = new SemanticEngine(semanticOptions);
      // Pass verbose option to semantic engine
      this.semanticEngine.verbose = this.verbose;
      
      // ts-morph is now a core dependency - but optimized for targeted files
      const success = await this.semanticEngine.initialize(projectPath, config?.targetFiles);
      
      if (success) {
        this.semanticEnabled = true;
        this.symbolTableInitialized = true;
        if (this.verbose) {
          console.log(`🧠 Symbol Table initialized for: ${projectPath}`);
        }
      } else {
        if (this.verbose) {
          console.warn('⚠️  Symbol Table initialization failed, using fallback mode');
        }
      }
      
    } catch (error) {
      if (this.verbose) {
        console.warn('⚠️  ts-morph Symbol Table unavailable:', error.message);
        console.warn('⚠️  Falling back to traditional AST/regex analysis only');
      }
    }
  }

  /**
   * Load rules from unified registry instead of scanning directories
   * Following Rule C006: Verb-noun naming
   * @param {Object} config - Engine configuration
   */
  async loadRulesFromRegistry(config = {}) {
    try {
      // Get rules supported by heuristic engine from unified registry
      const supportedRules = this.unifiedRegistry.getRulesForEngine('heuristic');
      
      if (this.verbose) {
        console.log(`🔍 [HeuristicEngine] Found ${supportedRules.length} rules from unified registry`);
      }
      
      // Load each rule
      for (const ruleDefinition of supportedRules) {
        await this.loadRuleFromDefinition(ruleDefinition);
      }
      
      // Manually load C047 if needed (DEPRECATED - C047 now in enhanced registry)
      // if (!this.semanticRules.has('C047') && !this.ruleAnalyzers.has('C047')) {
      //   await this.manuallyLoadC047();
      // }
      
      if (this.verbose) {
        console.log(`✅ [HeuristicEngine] Loaded ${this.supportedRulesList.length} rules from unified registry`);
      }
      
    } catch (error) {
      console.error('Failed to load rules from registry:', error.message);
      // Fallback to old scanning method
      await this.scanRuleAnalyzers(config);
    }
  }

  /**
   * Load a single rule from its definition
   * @param {Object} ruleDefinition - Rule definition from unified registry
   */
  async loadRuleFromDefinition(ruleDefinition) {
    const ruleId = ruleDefinition.id;
    
    try {
      // Resolve best analyzer path for this engine
      const analyzerPath = this.unifiedRegistry.resolveAnalyzerPath(ruleId, 'heuristic');
      
      if (!analyzerPath) {
        if (this.verbose) {
          console.warn(`⚠️ [HeuristicEngine] No compatible analyzer found for ${ruleId}`);
        }
        return;
      }
      
      // Determine analyzer type from path and strategy
      const strategy = ruleDefinition.strategy.preferred;
      const category = ruleDefinition.category;
      
      if (strategy === 'semantic' && this.symbolTableInitialized) {
        // Load as semantic rule
        await this.loadSemanticRule(ruleId, analyzerPath, { category });
      } else {
        // Load as traditional rule (ast/regex)
        await this.loadTraditionalRule(ruleId, analyzerPath, { category, type: strategy });
      }
      
    } catch (error) {
      if (this.verbose) {
        console.warn(`⚠️ [HeuristicEngine] Failed to load rule ${ruleId}:`, error.message);
      }
    }
  }

  /**
   * Load semantic rule from analyzer path
   * @param {string} ruleId - Rule ID
   * @param {string} analyzerPath - Path to analyzer file
   * @param {Object} metadata - Rule metadata
   */
  async loadSemanticRule(ruleId, analyzerPath, metadata) {
    try {
      const SemanticRuleClass = require(analyzerPath);
      
      // Verify it extends SemanticRuleBase
      if (this.isSemanticRule(SemanticRuleClass)) {
        await this.registerSemanticRule(ruleId, SemanticRuleClass, {
          path: analyzerPath,
          category: metadata.category
        });
        
        if (this.verbose) {
          console.log(`🧠 [HeuristicEngine] Loaded semantic rule: ${ruleId}`);
        }
      } else {
        // Not a semantic rule, fallback to traditional
        await this.loadTraditionalRule(ruleId, analyzerPath, metadata);
      }
      
    } catch (error) {
      if (this.verbose) {
        console.warn(`⚠️ [HeuristicEngine] Failed to load semantic rule ${ruleId}:`, error.message);
      }
    }
  }

  /**
   * Load traditional rule (ast/regex) from analyzer path
   * @param {string} ruleId - Rule ID
   * @param {string} analyzerPath - Path to analyzer file
   * @param {Object} metadata - Rule metadata
   */
  async loadTraditionalRule(ruleId, analyzerPath, metadata) {
    try {
      const analyzerModule = require(analyzerPath);
      const AnalyzerClass = analyzerModule.default || analyzerModule;
      
      this.registerTraditionalRule(ruleId, AnalyzerClass, {
        path: analyzerPath,
        category: metadata.category,
        folder: ruleId, // Add folder name for config loading
        type: metadata.type || 'regex'
      });
      
      if (this.verbose) {
        console.log(`🔧 [HeuristicEngine] Loaded ${metadata.type} rule: ${ruleId}`);
      }
      
    } catch (error) {
      if (this.verbose) {
        console.warn(`⚠️ [HeuristicEngine] Failed to load traditional rule ${ruleId}:`, error.message);
      }
    }
  }

  /**
   * Scan for available rule analyzers with semantic support
   * Priority: semantic > ast > regex
   * Following Rule C006: Verb-noun naming
   */
  async scanRuleAnalyzers(config = {}) {
    const rulesDir = path.resolve(__dirname, '../rules');
    
    if (!fs.existsSync(rulesDir)) {
      console.warn('⚠️ Rules directory not found');
      return;
    }

    try {
      // Scan category folders (common, security, typescript, etc.)
      const categoryFolders = fs.readdirSync(rulesDir, { withFileTypes: true })
        .filter(dirent => dirent.isDirectory())
        .filter(dirent => !['tests', 'docs', 'utils', 'migration'].includes(dirent.name))
        .map(dirent => dirent.name);

      for (const categoryFolder of categoryFolders) {
        const categoryPath = path.join(rulesDir, categoryFolder);
        
        // Scan rule folders within category
        const ruleFolders = fs.readdirSync(categoryPath, { withFileTypes: true })
          .filter(dirent => dirent.isDirectory())
          .map(dirent => dirent.name);

        for (const ruleFolder of ruleFolders) {
          const ruleId = ruleFolder; // Use folder name directly as rule ID
          const rulePath = path.join(categoryPath, ruleFolder);
          
          await this.loadRuleAnalyzer(ruleId, rulePath, categoryFolder);
        }
      }
      
    } catch (error) {
      console.warn('⚠️ Error scanning rule analyzers:', error.message);
    }
  }

  /**
   * Lazy load a single rule on-demand
   * @param {string} ruleId - Rule ID to load
   * @param {Object} options - Loading options
   */
  async lazyLoadRule(ruleId, options = {}) {
    try {
      const ruleDefinition = this.unifiedRegistry.getRuleDefinition(ruleId);
      
      if (!ruleDefinition) {
        if (options.verbose) {
          console.warn(`⚠️ [HeuristicEngine] Rule definition not found for ${ruleId}`);
        }
        return;
      }

      // Check if rule supports heuristic engine
      if (!this.unifiedRegistry.isRuleSupported(ruleId, 'heuristic')) {
        if (options.verbose) {
          console.warn(`⚠️ [HeuristicEngine] Rule ${ruleId} not supported by heuristic engine`);
        }
        return;
      }

      if (options.verbose) {
        console.log(`🔄 [HeuristicEngine] Lazy loading rule ${ruleId}...`);
      }

      await this.loadRuleFromDefinition(ruleDefinition);
      
    } catch (error) {
      if (options.verbose) {
        console.warn(`⚠️ [HeuristicEngine] Failed to lazy load rule ${ruleId}:`, error.message);
      }
    }
  }

  /**
   * Manually load C047 semantic rule (special case)
   */
  async manuallyLoadC047() {
    try {
      if (this.verbose) {
        console.log(`[DEBUG] 🔬 Manually loading C047 semantic rule...`);
      }
      
      const c047Path = path.resolve(__dirname, '../rules/common/C047_no_duplicate_retry_logic/c047-semantic-rule.js');
      
      if (fs.existsSync(c047Path)) {
        const C047SemanticRule = require(c047Path);
        const instance = new C047SemanticRule();
        
        // Register as semantic rule
        await this.registerSemanticRule('C047', C047SemanticRule, {
          path: c047Path,
          category: 'common',
          type: 'semantic',
          description: 'C047 - No Duplicate Retry Logic (Semantic Analysis)'
        });
        
        if (this.verbose) {
          console.log(`[DEBUG] ✅ C047 semantic rule loaded successfully`);
        }
      } else {
        console.warn(`⚠️ C047 semantic rule not found at: ${c047Path}`);
      }
      
    } catch (error) {
      console.warn(`⚠️ Failed to manually load C047:`, error.message);
    }
  }

  /**
   * Load rule analyzer with semantic priority
   */
  async loadRuleAnalyzer(ruleId, rulePath, categoryFolder) {
    // Analyzer priority: semantic > ast > regex
    const analyzerCandidates = [
      { path: path.join(rulePath, 'semantic-analyzer.js'), type: 'semantic' },
      { path: path.join(rulePath, 'ast-analyzer.js'), type: 'ast' },
      { path: path.join(rulePath, 'regex-analyzer.js'), type: 'regex' },
      { path: path.join(rulePath, 'analyzer.js'), type: 'regex' } // legacy fallback
    ];
    
    let selectedAnalyzer = null;
    let analyzerPath = null;
    let analyzerType = null;
    
    // Try semantic analyzer first if Symbol Table available
    if (this.symbolTableInitialized) {
      const semanticCandidate = analyzerCandidates[0];
      if (fs.existsSync(semanticCandidate.path)) {
        try {
          const analyzerModule = require(semanticCandidate.path);
          selectedAnalyzer = analyzerModule.default || analyzerModule;
          analyzerPath = semanticCandidate.path;
          analyzerType = 'semantic';
          
          // Verify it extends SemanticRuleBase
          if (this.isSemanticRule(selectedAnalyzer)) {
            await this.registerSemanticRule(ruleId, selectedAnalyzer, {
              path: analyzerPath,
              category: categoryFolder
            });
            return; // Successfully registered semantic rule
          }
        } catch (error) {
          console.debug(`Semantic analyzer for ${ruleId} failed to load:`, error.message);
        }
      }
    }
    
    // Fall back to AST analyzer
    const astCandidate = analyzerCandidates[1];
    if (fs.existsSync(astCandidate.path)) {
      try {
        const analyzerModule = require(astCandidate.path);
        selectedAnalyzer = analyzerModule.default || analyzerModule;
        analyzerPath = astCandidate.path;
        analyzerType = 'ast';
      } catch (error) {
        console.debug(`AST analyzer for ${ruleId} failed to load:`, error.message);
      }
    }
    
    // Fall back to regex analyzer
    if (!selectedAnalyzer) {
      for (const regexCandidate of analyzerCandidates.slice(2)) {
        if (fs.existsSync(regexCandidate.path)) {
          try {
            const analyzerModule = require(regexCandidate.path);
            selectedAnalyzer = analyzerModule.default || analyzerModule;
            analyzerPath = regexCandidate.path;
            analyzerType = 'regex';
            break;
          } catch (error) {
            console.debug(`Regex analyzer for ${ruleId} failed to load:`, error.message);
          }
        }
      }
    }
    
    // Register traditional (non-semantic) analyzer
    if (selectedAnalyzer) {
      this.registerTraditionalRule(ruleId, selectedAnalyzer, {
        path: analyzerPath,
        category: categoryFolder,
        folder: fullRuleId, // Add folder name for config loading
        type: analyzerType
      });
    }
  }

  /**
   * Check if analyzer is a semantic rule
   */
  isSemanticRule(analyzerClass) {
    if (typeof analyzerClass !== 'function') return false;
    
    try {
      const instance = new analyzerClass(analyzerClass.name);
      return instance instanceof SemanticRuleBase;
    } catch (error) {
      return false;
    }
  }

  /**
   * Register semantic rule (lazy initialization)
   */
  async registerSemanticRule(ruleId, analyzerClass, metadata) {
    try {
      // Store rule class and metadata for lazy initialization
      this.semanticRules.set(ruleId, {
        analyzerClass,
        metadata,
        type: 'semantic',
        initialized: false,
        instance: null
      });
      
      this.supportedRulesList.push(ruleId);
      
      if (this.verbose) {
        console.log(`🧠 Registered semantic rule: ${ruleId} (lazy initialization)`);
      }
      
    } catch (error) {
      console.warn(`⚠️ Failed to register semantic rule ${ruleId}:`, error.message);
    }
  }

  /**
   * Initialize semantic rule on-demand
   */
  async initializeSemanticRule(ruleId) {
    const ruleEntry = this.semanticRules.get(ruleId);
    if (!ruleEntry || ruleEntry.initialized) {
      return ruleEntry?.instance;
    }

    try {
      const instance = new ruleEntry.analyzerClass(ruleId);
      instance.initialize(this.semanticEngine, { verbose: this.verbose });
      
      // Update entry with initialized instance
      ruleEntry.instance = instance;
      ruleEntry.initialized = true;
      
      if (this.verbose) {
        console.log(`🔧 Rule ${ruleId} initialized with semantic analysis`);
      }
      
      return instance;
    } catch (error) {
      console.warn(`⚠️ Failed to initialize semantic rule ${ruleId}:`, error.message);
      return null;
    }
  }

  /**
   * Register traditional (heuristic/AST/regex) rule
   */
  registerTraditionalRule(ruleId, analyzer, metadata) {
    if (typeof analyzer === 'function') {
      // Class constructor
      this.ruleAnalyzers.set(ruleId, {
        ...metadata,
        class: analyzer,
        type: 'class'
      });
      this.supportedRulesList.push(ruleId);
    } else if (analyzer && typeof analyzer === 'object' && analyzer.analyze) {
      // Instance with analyze method
      this.ruleAnalyzers.set(ruleId, {
        ...metadata,
        instance: analyzer,
        type: 'instance'
      });
      this.supportedRulesList.push(ruleId);
    } else {
      console.warn(`⚠️ Analyzer for ${ruleId} has unsupported format:`, typeof analyzer);
    }
  }

  /**
   * Extract rule ID from folder name
   * Following Rule C006: Verb-noun naming
   * @param {string} folderName - Rule folder name
   * @returns {string} Rule ID
   */
  extractRuleIdFromFolder(folderName) {
    // Extract rule ID from patterns like "C019_log_level_usage"
    const match = folderName.match(/^([CST]\d{3})/);
    return match ? match[1] : folderName;
  }

  /**
   * Get full rule ID from short rule ID (C029 -> C029_catch_block_logging)
   * @param {string} ruleId - Short rule ID 
   * @returns {string} Full rule ID or original if not found
   */
  getFullRuleId(ruleId) {
    // Check exact match first
    if (this.ruleAnalyzers.has(ruleId)) {
      return ruleId;
    }
    
    // Find full rule ID that starts with short rule ID
    const shortRulePattern = new RegExp(`^${ruleId}_`);
    const fullRuleId = Array.from(this.ruleAnalyzers.keys()).find(fullId => shortRulePattern.test(fullId));
    
    return fullRuleId || ruleId; // Return original if not found
  }

  /**
   * Check if a rule is supported by this engine
   * Following Rule C006: Verb-noun naming
   * @param {string} ruleId - Rule ID to check
   * @returns {boolean} True if rule is supported
   */
  isRuleSupported(ruleId) {
    // Special case: C047 is always supported (loaded on-demand)
    if (ruleId === 'C047') {
      return true;
    }
    
    // Use unified registry for primary lookup
    if (this.unifiedRegistry && this.unifiedRegistry.initialized) {
      return this.unifiedRegistry.isRuleSupported(ruleId, 'heuristic');
    }
    
    // Fallback to original logic for backward compatibility
    return this.supportedRulesList.includes(ruleId) || 
           this.semanticRules.has(ruleId) ||
           this.ruleAnalyzers.has(ruleId) ||
           this.checkShortRuleIdMatch(ruleId);
  }

  /**
   * Check short rule ID matches (backward compatibility)
   * @param {string} ruleId - Short rule ID (e.g., C029)
   * @returns {boolean} True if matches any full rule ID
   */
  checkShortRuleIdMatch(ruleId) {
    const shortRulePattern = new RegExp(`^${ruleId}_`);
    return this.supportedRulesList.some(fullRuleId => shortRulePattern.test(fullRuleId)) ||
           Array.from(this.semanticRules.keys()).some(fullRuleId => shortRulePattern.test(fullRuleId)) ||
           Array.from(this.ruleAnalyzers.keys()).some(fullRuleId => shortRulePattern.test(fullRuleId));
  }

  /**
   * Analyze files using heuristic patterns
   * ✅ ENHANCED: Now includes performance optimizations and batch processing
   * Following Rule C006: Verb-noun naming
   * @param {string[]} files - Files to analyze
   * @param {Object[]} rules - Rules to apply
   * @param {Object} options - Analysis options
   * @returns {Promise<Object>} Analysis results
   */
  async analyze(files, rules, options) {
    if (!this.initialized) {
      throw new Error('Heuristic engine not initialized');
    }

    // ✅ PERFORMANCE: Apply file limits and timeout protection
    const startTime = Date.now();
    this.metrics.startTime = startTime;
    
    // Apply analysis file limits (different from semantic file limits)
    const maxFiles = this.getAnalysisFileLimit(options);
    const limitedFiles = files.slice(0, maxFiles);
    
    if (files.length > maxFiles && this.verbose) {
      console.warn(`⚠️  [HeuristicEngine] Analysis file limit: ${limitedFiles.length}/${files.length} files`);
      console.log(`   💡 Note: Symbol table uses separate limit (--max-semantic-files)`);
    }

    // Set up timeout if configured
    const timeout = this.performanceConfig?.timeout || parseInt(options.timeout) || 0;
    let timeoutId = null;
    
    if (timeout > 0) {
      timeoutId = setTimeout(() => {
        throw new Error(`Analysis timeout after ${timeout}ms`);
      }, timeout);
    }

    if (options.verbose) {
      console.log(`🔍 [HeuristicEngine] Analyzing ${limitedFiles.length} files with ${rules.length} rules`);
      if (this.performanceConfig?.name) {
        console.log(`⚡ [Performance] Using ${this.performanceConfig.name} profile`);
      }
      if (timeout > 0) {
        console.log(`⏰ [Timeout] Analysis will timeout after ${timeout/1000}s`);
      }
    }

    try {
      // Check if we should use batch processing
      if (this.shouldUseBatchProcessing(limitedFiles, rules)) {
        return await this.analyzeBatched(limitedFiles, rules, options);
      } else {
        return await this.analyzeStandard(limitedFiles, rules, options);
      }
    } finally {
      // Clear timeout
      if (timeoutId) {
        clearTimeout(timeoutId);
      }
      
      // Log performance metrics
      const duration = Date.now() - startTime;
      this.metrics.filesProcessed = limitedFiles.length;
      this.metrics.rulesProcessed = rules.length;
      
      if (options.verbose) {
        console.log(`✅ [HeuristicEngine] Analysis completed in ${duration}ms`);
      }
    }
  }

  /**
   * ✅ NEW: Get analysis file limit (separate from semantic file limit)
   */
  getAnalysisFileLimit(options) {
    // User-specified limit
    if (options.maxFiles && parseInt(options.maxFiles) > 0) {
      return parseInt(options.maxFiles);
    }
    
    // Performance config limit
    if (this.performanceConfig?.maxFiles) {
      return this.performanceConfig.maxFiles;
    }
    
    // Default based on performance mode
    const mode = options.performance || 'auto';
    const defaults = {
      fast: 500,
      auto: 1000,
      careful: 1500
    };
    
    return defaults[mode] || 1000;
  }

  /**
   * ✅ NEW: Determine if batch processing should be used
   */
  shouldUseBatchProcessing(files, rules) {
    const batchThreshold = this.performanceConfig?.batchThreshold || 100;
    const totalWorkload = files.length * rules.length;
    
    return totalWorkload > batchThreshold || 
           files.length > 200 || 
           rules.length > 30;
  }

  /**
   * ✅ NEW: Batch processing for large workloads
   */
  async analyzeBatched(files, rules, options) {
    if (options.verbose) {
      console.log(`� [HeuristicEngine] Using batch processing for large workload`);
    }

    const results = {
      results: [],
      filesAnalyzed: files.length,
      engine: 'heuristic',
      metadata: {
        rulesAnalyzed: rules.map(r => r.id),
        analyzersUsed: [],
        batchProcessing: true
      }
    };

    // Create rule batches
    const batchSize = this.performanceConfig?.batchSize || 10;
    const ruleBatches = [];
    
    for (let i = 0; i < rules.length; i += batchSize) {
      ruleBatches.push(rules.slice(i, i + batchSize));
    }

    if (options.verbose) {
      console.log(`📦 [Batch] Processing ${ruleBatches.length} rule batches (${batchSize} rules each)`);
    }

    // Process each batch
    for (let batchIndex = 0; batchIndex < ruleBatches.length; batchIndex++) {
      const ruleBatch = ruleBatches[batchIndex];
      
      if (options.verbose) {
        console.log(`⚡ [Batch ${batchIndex + 1}/${ruleBatches.length}] Processing ${ruleBatch.length} rules...`);
      }

      const batchResults = await this.analyzeStandard(files, ruleBatch, options);
      
      // Merge batch results
      for (const fileResult of batchResults.results) {
        let existingFile = results.results.find(r => r.file === fileResult.file);
        if (!existingFile) {
          existingFile = { file: fileResult.file, violations: [] };
          results.results.push(existingFile);
        }
        existingFile.violations.push(...fileResult.violations);
      }
      
      results.metadata.analyzersUsed.push(...batchResults.metadata.analyzersUsed);
      
      // Memory management
      if (batchIndex % 3 === 0 && global.gc) {
        global.gc(); // Trigger garbage collection every 3 batches
      }
    }

    return results;
  }

  /**
   * ✅ REFACTORED: Standard analysis method (extracted from original analyze)
   */
  async analyzeStandard(files, rules, options) {
    const results = {
      results: [],
      filesAnalyzed: files.length,
      engine: 'heuristic',
      metadata: {
        rulesAnalyzed: rules.map(r => r.id),
        analyzersUsed: []
      }
    };

    // Group files by language for efficient processing
    const filesByLanguage = this.groupFilesByLanguage(files);

    for (const rule of rules) {
      // Special case: Load C047 semantic rule on-demand
      if (rule.id === 'C047' && !this.semanticRules.has('C047')) {
        if (options.verbose) {
          console.log(`🔬 [HeuristicEngine] Loading C047 semantic rule on-demand...`);
        }
        await this.manuallyLoadC047();
      }
      
      // Lazy load rule if not already loaded
      if (!this.isRuleSupported(rule.id)) {
        if (options.verbose) {
          console.log(`🔄 [HeuristicEngine] Lazy loading rule ${rule.id}...`);
        }
        await this.lazyLoadRule(rule.id, options);
      }
      
      if (!this.isRuleSupported(rule.id)) {
        if (options.verbose) {
          console.warn(`⚠️ Rule ${rule.id} not supported by Heuristic engine, skipping...`);
        }
        continue;
      }

      try {
        let ruleViolations = [];
        
        // Check if this is a semantic rule first (higher priority)
        if (this.semanticRules.has(rule.id)) {
          if (options.verbose) {
            console.log(`🧠 [HeuristicEngine] Running semantic analysis for rule ${rule.id}`);
          }
          ruleViolations = await this.analyzeSemanticRule(rule, files, options);
        } else {
          // Fallback to traditional analysis
          if (options.verbose) {
            console.log(`🔧 [HeuristicEngine] Running traditional analysis for rule ${rule.id}`);
          }
          ruleViolations = await this.analyzeRule(rule, filesByLanguage, options);
        }
        
        if (ruleViolations.length > 0) {
          // Group violations by file
          const violationsByFile = this.groupViolationsByFile(ruleViolations);
          
          for (const [filePath, violations] of violationsByFile) {
            // Find or create file result
            let fileResult = results.results.find(r => r.file === filePath);
            if (!fileResult) {
              fileResult = { file: filePath, violations: [] };
              results.results.push(fileResult);
            }
            fileResult.violations.push(...violations);
          }
        }

        results.metadata.analyzersUsed.push(rule.id);
        
      } catch (error) {
        console.error(`❌ Failed to analyze rule ${rule.id}:`, error.message);
        // Continue with other rules
      }
    }

    return results;
  }

  /**
   * Analyze semantic rule across files
   * Following Rule C006: Verb-noun naming
   * @param {Object} rule - Rule to analyze
   * @param {string[]} files - Files to analyze
   * @param {Object} options - Analysis options
   * @returns {Promise<Object[]>} Rule violations
   */
  async analyzeSemanticRule(rule, files, options) {
    const semanticRuleInfo = this.semanticRules.get(rule.id);
    if (!semanticRuleInfo) {
      console.warn(`⚠️ Semantic rule ${rule.id} not found`);
      return [];
    }

    try {
      // Initialize rule on-demand (lazy initialization)
      const ruleInstance = await this.initializeSemanticRule(rule.id);
      if (!ruleInstance) {
        console.warn(`⚠️ Failed to initialize semantic rule ${rule.id}`);
        return [];
      }

      const allViolations = [];

      // Run semantic analysis for each file
      for (const filePath of files) {
        try {
          if (options.verbose) {
            console.log(`🧠 [SemanticRule] Analyzing ${path.basename(filePath)} with ${rule.id}`);
          }
          
          // Call semantic rule's analyzeFile method
          await ruleInstance.analyzeFile(filePath, options);
          
          // Get violations from the rule instance
          const fileViolations = ruleInstance.getViolations();
          allViolations.push(...fileViolations);
          
          // Clear violations for next file
          ruleInstance.clearViolations();
          
        } catch (fileError) {
          console.warn(`⚠️ Semantic rule ${rule.id} failed for ${filePath}:`, fileError.message);
        }
      }

      if (options.verbose && allViolations.length > 0) {
        console.log(`🧠 [SemanticRule] Found ${allViolations.length} violations for ${rule.id}`);
      }

      return allViolations;
      
    } catch (error) {
      console.error(`❌ Failed to run semantic rule ${rule.id}:`, error.message);
      return [];
    }
  }

  /**
   * Analyze a specific rule across files
   * Following Rule C006: Verb-noun naming
   * @param {Object} rule - Rule to analyze
   * @param {Object} filesByLanguage - Files grouped by language
   * @param {Object} options - Analysis options
   * @returns {Promise<Object[]>} Rule violations
   */
  async analyzeRule(rule, filesByLanguage, options) {
    // Get full rule ID (C029 -> C029_catch_block_logging)
    const fullRuleId = this.getFullRuleId(rule.id);
    
    // Lazy load rule if not already loaded
    if (!this.ruleAnalyzers.has(fullRuleId)) {
      if (options.verbose) {
        console.log(`🔄 [HeuristicEngine] Lazy loading rule ${rule.id} for analysis...`);
      }
      await this.lazyLoadRule(rule.id, options);
    }
    
    const analyzerInfo = this.ruleAnalyzers.get(fullRuleId);
    
    if (!analyzerInfo) {
      return [];
    }

    try {
      // Get analyzer - handle both class and instance types
      let analyzer;
      
      if (analyzerInfo.type === 'class') {
        // Create analyzer instance from class
        const AnalyzerClass = analyzerInfo.class;
        try {
          analyzer = new AnalyzerClass({ 
            verbose: options.verbose,
            semanticEngine: this.semanticEngine 
          });
          
          // Initialize with semantic engine if method exists
          if (analyzer.initialize && typeof analyzer.initialize === 'function') {
            await analyzer.initialize(this.semanticEngine);
          }
        } catch (constructorError) {
          throw new Error(`Failed to instantiate analyzer class: ${constructorError.message}`);
        }
      } else if (analyzerInfo.type === 'instance') {
        // Use existing analyzer instance
        analyzer = analyzerInfo.instance;
        
        // Initialize existing instance with semantic engine if method exists
        if (analyzer.initialize && typeof analyzer.initialize === 'function') {
          await analyzer.initialize(this.semanticEngine);
        }
      } else {
        throw new Error(`Unknown analyzer type: ${analyzerInfo.type}`);
      }
      
      // Verify analyzer has required methods
      if (!analyzer.analyze || typeof analyzer.analyze !== 'function') {
        console.warn(`⚠️ Analyzer for ${rule.id} missing analyze method`);
        return [];
      }

      const allViolations = [];

      // Run analyzer for each supported language
      const ruleLanguages = this.getRuleLanguages(rule);
      for (const language of ruleLanguages) {
        const languageFiles = filesByLanguage[language] || [];
        if (languageFiles.length === 0) continue;

        try {
          // Load rule config
          const ruleConfig = await this.loadRuleConfig(rule.id, analyzerInfo.folder, analyzerInfo.category, options.verbose);
          
          // Run analysis with AST enhancement
          if (options.verbose) {
            console.log(`🔧 [DEBUG] About to call runEnhancedAnalysis for rule ${rule.id}, language ${language}`);
          }
          
          const languageViolations = await this.runEnhancedAnalysis(
            analyzer,
            rule.id,
            languageFiles, 
            language, 
            { ...ruleConfig, ...options, semanticEngine: this.semanticEngine }
          );

          allViolations.push(...languageViolations);
          
        } catch (error) {
          console.error(`❌ Rule ${rule.id} failed for ${language}:`, error.message);
        }
      }

      return allViolations;
      
    } catch (error) {
      console.error(`❌ Failed to create analyzer for rule ${rule.id}:`, error.message);
      return [];
    }
  }

  /**
   * Get supported languages for a rule
   * Following Rule C006: Verb-noun naming
   * @param {Object} rule - Rule object
   * @returns {string[]} Supported languages
   */
  getRuleLanguages(rule) {
    // Get from rule adapter
    const adapterRule = this.ruleAdapter.getRuleById(rule.id);
    if (adapterRule?.languages) {
      // If rule supports 'All languages', return all available languages
      if (adapterRule.languages.includes('All languages')) {
        return ['typescript', 'javascript', 'java', 'python', 'ruby', 'php', 'dart', 'kotlin', 'swift'];
      }
      return adapterRule.languages;
    }

    // Fallback to rule object
    if (rule.languages) {
      // If rule supports 'All languages', return all available languages
      if (rule.languages.includes('All languages')) {
        return ['typescript', 'javascript', 'java', 'python', 'ruby', 'php', 'dart', 'kotlin', 'swift'];
      }
      return rule.languages;
    }

    // Default to common languages
    return ['typescript', 'javascript'];
  }

  /**
   * Load rule configuration
   * Following Rule C006: Verb-noun naming
   * @param {string} ruleId - Rule ID
   * @param {string} ruleFolder - Rule folder name
   * @param {string} category - Rule category (common, security, etc)
   * @param {boolean} verbose - Enable verbose logging
   * @returns {Promise<Object>} Rule configuration
   */
  async loadRuleConfig(ruleId, ruleFolder, category = 'common', verbose = false) {
    try {
      const configPath = path.resolve(__dirname, '../rules', category, ruleFolder, 'config.json');
      if (fs.existsSync(configPath)) {
        return require(configPath);
      }
    } catch (error) {
      if (verbose) {
        console.warn(`[DEBUG] ⚠️ Failed to load config for ${ruleId}:`, error.message);
      }
    }

    // Return minimal config
    return {
      ruleId,
      name: `Rule ${ruleId}`,
      description: `Analysis for rule ${ruleId}`,
      severity: 'warning'
    };
  }

  /**
   * Group files by programming language
   * Following Rule C006: Verb-noun naming
   * @param {string[]} files - Files to group
   * @returns {Object} Files grouped by language
   */
  groupFilesByLanguage(files) {
    const groups = {};

    for (const file of files) {
      const language = this.detectLanguage(file);
      if (!groups[language]) {
        groups[language] = [];
      }
      groups[language].push(file);
    }

    return groups;
  }

  /**
   * Detect programming language from file extension
   * Following Rule C006: Verb-noun naming
   * @param {string} filePath - File path
   * @returns {string} Detected language
   */
  detectLanguage(filePath) {
    const ext = path.extname(filePath).toLowerCase();
    
    switch (ext) {
      case '.ts': case '.tsx': return 'typescript';
      case '.js': case '.jsx': return 'javascript';
      case '.dart': return 'dart';
      case '.swift': return 'swift';
      case '.kt': case '.kts': return 'kotlin';
      default: return 'unknown';
    }
  }

  /**
   * Group violations by file path
   * Following Rule C006: Verb-noun naming
   * @param {Object[]} violations - Array of violations
   * @returns {Map} Violations grouped by file
   */
  groupViolationsByFile(violations) {
    const groups = new Map();

    for (const violation of violations) {
      const filePath = violation.file || violation.filePath;
      if (!filePath) continue;

      if (!groups.has(filePath)) {
        groups.set(filePath, []);
      }
      groups.get(filePath).push(violation);
    }

    return groups;
  }

  /**
   * Get supported rules
   * Following Rule C006: Verb-noun naming
   * @returns {string[]} Supported rule IDs
   */
  getSupportedRules() {
    return this.supportedRulesList;
  }

  /**
   * Run enhanced analysis with multiple strategies
   * Automatically selects optimal analysis method per rule
   * Following Rule C006: Verb-noun naming
   */
  async runEnhancedAnalysis(analyzer, ruleId, files, language, options) {
    // Create debug config from options
    const debugConfig = {
      enabled: options.debug || options.verbose || false,
      logger: (component, message) => console.log(`🔧 [${component}] ${message}`)
    };
    
    // Debug logging based on debug flag
    if (debugConfig.enabled) {
      debugConfig.logger(this.constructor.name, `runEnhancedAnalysis called: rule=${ruleId}, language=${language}, files=${files.length}`);
    }
    
    if (options.verbose) {
      console.log(`🔧 [DEBUG] runEnhancedAnalysis called: rule=${ruleId}, language=${language}, files=${files.length}`);
    }
    
    // Load rule analysis strategy
    const strategy = await this.getRuleAnalysisStrategy(ruleId, language);
    
    // Debug logging based on debug flag
    if (debugConfig.enabled) {
      debugConfig.logger(this.constructor.name, `Rule ${ruleId}: ${strategy.primary} (approach: ${strategy.approach})`);
    }
    
    if (options.verbose) {
      console.log(`🔧 [Strategy] Rule ${ruleId}: ${strategy.primary} (fallback: ${strategy.fallback || 'none'})`);
    }

    let violations = [];
    let analysisResults = {
      methods: [],
      totalViolations: 0,
      accuracy: 'unknown'
    };

    // Execute analysis based on strategy
    switch (strategy.approach) {
      case 'ast-primary':
        violations = await this.runASTPrimaryAnalysis(analyzer, ruleId, files, language, options, strategy, debugConfig);
        break;
      
      case 'regex-optimal':
        violations = await this.runRegexOptimalAnalysis(analyzer, ruleId, files, language, options);
        break;
      
      case 'hybrid-combined':
        violations = await this.runHybridAnalysis(analyzer, ruleId, files, language, options, strategy);
        break;
      
      case 'progressive-enhancement':
        violations = await this.runProgressiveAnalysis(analyzer, ruleId, files, language, options, strategy);
        break;
      
      default:
        // Fallback to existing logic
        violations = await this.runLegacyAnalysis(analyzer, ruleId, files, language, options);
    }

    if (options.verbose && violations.length > 0) {
      console.log(`📊 [Analysis] Found ${violations.length} violations using ${strategy.approach}`);
    }

    return violations;
  }

  /**
   * Get optimal analysis strategy for a rule
   */
  async getRuleAnalysisStrategy(ruleId, language) {
    try {
      const strategies = require('../config/rule-analysis-strategies');
      
      // Check AST-preferred rules
      if (strategies.astPreferred[ruleId]) {
        const astAvailable = this.astRegistry.isASTSupportAvailable(language);
        return {
          approach: 'ast-primary',
          primary: 'ast',
          fallback: 'regex',
          astAvailable,
          config: strategies.astPreferred[ruleId]
        };
      }
      
      // Check regex-optimal rules
      if (strategies.regexOptimal[ruleId]) {
        return {
          approach: 'regex-optimal',
          primary: 'regex',
          config: strategies.regexOptimal[ruleId]
        };
      }
      
      // Check hybrid rules
      if (strategies.hybridOptimal[ruleId]) {
        return {
          approach: 'hybrid-combined',
          primary: strategies.hybridOptimal[ruleId].strategy.split('-')[0],
          config: strategies.hybridOptimal[ruleId]
        };
      }
      
      // Check experimental rules
      if (strategies.experimental[ruleId]) {
        return {
          approach: 'progressive-enhancement',
          primary: 'regex',
          fallback: 'ast',
          config: strategies.experimental[ruleId]
        };
      }
      
      // Default strategy
      return {
        approach: 'ast-primary',
        primary: 'ast',
        fallback: 'regex',
        astAvailable: this.astRegistry.isASTSupportAvailable(language)
      };
      
    } catch (error) {
      // Fallback if strategy config is not available
      return {
        approach: 'ast-primary',
        primary: 'ast',
        fallback: 'regex',
        astAvailable: this.astRegistry.isASTSupportAvailable(language)
      };
    }
  }

  /**
   * AST-primary analysis: Try AST first, fallback to regex
   */
  async runASTPrimaryAnalysis(analyzer, ruleId, files, language, options, strategy, debugConfig) {
    const violations = [];
    
    if (debugConfig.enabled) {
      debugConfig.logger(this.constructor.name, `Starting AST-primary analysis for ${ruleId}, AST available: ${strategy.astAvailable}`);
    }
    
    for (const filePath of files) {
      try {
        const code = fs.readFileSync(filePath, 'utf8');
        let analysisResult = null;
        
        // Try AST analysis first if available
        if (strategy.astAvailable) {
          if (debugConfig.enabled) {
            debugConfig.logger(this.constructor.name, `Attempting AST for file: ${filePath}`);
          }
          
          try {
            const astResult = await this.astRegistry.analyzeRule(ruleId, code, language, filePath);
            if (astResult && astResult.length > 0) {
              analysisResult = astResult.map(violation => ({
                ...violation,
                filePath,
                analysisMethod: 'ast',
                confidence: strategy.config?.accuracy?.ast || 90
              }));
              
              if (debugConfig.enabled) {
                debugConfig.logger(this.constructor.name, `AST found ${astResult.length} violations in ${filePath}`);
              }
            }
          } catch (astError) {
            if (debugConfig.enabled) {
              debugConfig.logger(this.constructor.name, `AST failed for ${filePath}: ${astError.message}`);
            }
            
            if (options.verbose) {
              console.warn(`⚠️ AST analysis failed for ${filePath}, falling back to regex`);
            }
          }
        } else {
          if (debugConfig.enabled) {
            debugConfig.logger(this.constructor.name, `AST not available, skipping to fallback for ${filePath}`);
          }
        }

        // Fallback to regex if AST failed or not available
        if (!analysisResult) {
          if (debugConfig.enabled) {
            debugConfig.logger(this.constructor.name, `Using regex fallback for ${filePath}`);
          }
          const regexResult = await analyzer.analyze([filePath], language, options);
          if (regexResult && regexResult.length > 0) {
            analysisResult = regexResult.map(violation => ({
              ...violation,
              analysisMethod: 'regex',
              confidence: strategy.config?.accuracy?.regex || 75
            }));
          }
        }

        if (analysisResult) {
          violations.push(...analysisResult);
        }
        
      } catch (error) {
        if (options.verbose) {
          console.error(`❌ Analysis failed for ${filePath}:`, error.message);
        }
      }
    }

    return violations;
  }

  /**
   * Regex-optimal analysis: Use regex as primary method
   */
  async runRegexOptimalAnalysis(analyzer, ruleId, files, language, options) {
    const violations = [];
    
    const regexResult = await analyzer.analyze(files, language, options);
    if (regexResult && regexResult.length > 0) {
      violations.push(...regexResult.map(violation => ({
        ...violation,
        analysisMethod: 'regex',
        confidence: 95 // High confidence for regex-optimal rules
      })));
    }

    return violations;
  }

  /**
   * Hybrid analysis: Combine multiple methods for best results
   */
  async runHybridAnalysis(analyzer, ruleId, files, language, options, strategy) {
    const violations = [];
    const astViolations = [];
    const regexViolations = [];
    
    for (const filePath of files) {
      try {
        const code = fs.readFileSync(filePath, 'utf8');
        
        // Run both AST and regex analysis
        const analysisPromises = [];
        
        // AST analysis
        if (this.astRegistry.isASTSupportAvailable(language)) {
          analysisPromises.push(
            this.astRegistry.analyzeRule(ruleId, code, language, filePath)
              .then(result => ({ type: 'ast', result, filePath }))
              .catch(() => ({ type: 'ast', result: null, filePath }))
          );
        }
        
        // Regex analysis
        analysisPromises.push(
          analyzer.analyze([filePath], language, options)
            .then(result => ({ type: 'regex', result, filePath }))
            .catch(() => ({ type: 'regex', result: null, filePath }))
        );

        const results = await Promise.all(analysisPromises);
        
        // Process results based on strategy
        for (const { type, result, filePath } of results) {
          if (result && result.length > 0) {
            const violations = result.map(violation => ({
              ...violation,
              filePath,
              analysisMethod: type,
              confidence: strategy.config?.accuracy?.[type] || 85
            }));
            
            if (type === 'ast') {
              astViolations.push(...violations);
            } else {
              regexViolations.push(...violations);
            }
          }
        }
        
      } catch (error) {
        if (options.verbose) {
          console.error(`❌ Hybrid analysis failed for ${filePath}:`, error.message);
        }
      }
    }

    // Combine results intelligently
    const combinedViolations = this.combineHybridResults(
      astViolations, 
      regexViolations, 
      strategy.config?.strategy || 'ast-primary-regex-fallback'
    );

    return combinedViolations;
  }

  /**
   * Progressive enhancement: Start simple, enhance with advanced methods
   */
  async runProgressiveAnalysis(analyzer, ruleId, files, language, options, strategy) {
    const violations = [];
    
    // Start with basic regex analysis
    const regexResult = await analyzer.analyze(files, language, options);
    if (regexResult) {
      violations.push(...regexResult.map(violation => ({
        ...violation,
        analysisMethod: 'regex',
        confidence: 75
      })));
    }

    // Enhance with AST if available and beneficial
    if (this.astRegistry.isASTSupportAvailable(language) && violations.length > 0) {
      // TODO: Implement AST enhancement for specific violation types
      // This could involve re-analyzing files with violations for better precision
    }

    return violations;
  }

  /**
   * Legacy analysis method (for backward compatibility)
   */
  async runLegacyAnalysis(analyzer, ruleId, files, language, options) {
    const regexResult = await analyzer.analyze(files, language, options);
    return regexResult || [];
  }

  /**
   * Intelligently combine AST and regex results
   */
  combineHybridResults(astViolations, regexViolations, strategy) {
    switch (strategy) {
      case 'ast-primary-regex-fallback':
        // Use AST results where available, fill gaps with regex
        return this.mergeViolationsWithPriority(astViolations, regexViolations, 'ast');
        
      case 'regex-primary-ast-enhancement':
        // Use regex as base, enhance with AST insights
        return this.mergeViolationsWithPriority(regexViolations, astViolations, 'regex');
        
      case 'union':
        // Combine all violations (may have duplicates)
        return [...astViolations, ...regexViolations];
        
      case 'intersection':
        // Only violations found by both methods
        return this.findIntersectionViolations(astViolations, regexViolations);
        
      default:
        return astViolations.length > 0 ? astViolations : regexViolations;
    }
  }

  /**
   * Merge violations with priority given to one method
   */
  mergeViolationsWithPriority(primary, secondary, primaryType) {
    const merged = [...primary];
    const primaryLocations = new Set(
      primary.map(v => `${v.filePath}:${v.line}:${v.column}`)
    );

    // Add secondary violations that don't conflict with primary
    for (const violation of secondary) {
      const location = `${violation.filePath}:${violation.line}:${violation.column}`;
      if (!primaryLocations.has(location)) {
        merged.push({
          ...violation,
          isSecondary: true,
          primaryMethod: primaryType
        });
      }
    }

    return merged;
  }

  /**
   * Find violations detected by both methods (high confidence)
   */
  findIntersectionViolations(astViolations, regexViolations) {
    const intersection = [];
    
    for (const astViolation of astViolations) {
      const matching = regexViolations.find(regexViolation => 
        astViolation.filePath === regexViolation.filePath &&
        Math.abs(astViolation.line - regexViolation.line) <= 2 // Allow small line differences
      );
      
      if (matching) {
        intersection.push({
          ...astViolation,
          analysisMethod: 'hybrid-intersection',
          confidence: 98, // Very high confidence
          confirmedBy: ['ast', 'regex']
        });
      }
    }

    return intersection;
  }

  /**
   * Cleanup Heuristic engine resources
   * Following Rule C006: Verb-noun naming
   */
  async cleanup() {
    // Clear analyzer cache
    this.ruleAnalyzers.clear();
    this.supportedRulesList = [];
    
    await super.cleanup();
    if (this.verbose) {
      console.log('🔍 Heuristic engine cleanup completed');
    }
  }
}

module.exports = HeuristicEngine;