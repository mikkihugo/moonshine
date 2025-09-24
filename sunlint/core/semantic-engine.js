/**
 * SunLint Semantic Engine
 * Core Symbol Table Manager using ts-morph
 * 
 * Provides shared semantic analysis capabilities for SunLint rules
 * Manages project-wide Symbol Table and AST caching
 */

const path = require('path');
const fs = require('fs').promises;
const { Project, SyntaxKind } = require('ts-morph');

class SemanticEngine {
  constructor(options = {}) {
    this.options = {
      // Compiler options
      compilerOptions: {
        target: 99, // ScriptTarget.Latest
        allowJs: true,
        checkJs: false,
        skipLibCheck: true,
        skipDefaultLibCheck: true,
        ...options.compilerOptions
      },
      
      // Performance options
      enableCaching: options.enableCaching !== false,
      maxCacheSize: options.maxCacheSize || 100, // files
      memoryLimit: options.memoryLimit || 500 * 1024 * 1024, // 500MB
      
      // Analysis options
      crossFileAnalysis: options.crossFileAnalysis !== false,
      enableTypeChecker: options.enableTypeChecker || false,
      
      ...options
    };
    
    this.project = null;
    this.symbolTable = new Map();
    this.fileCache = new Map();
    this.initialized = false;
    this.stats = {
      filesAnalyzed: 0,
      cacheHits: 0,
      cacheMisses: 0,
      memoryUsage: 0
    };
  }

  /**
   * Initialize ts-morph project with optimized memory configuration
   * Designed for large projects (3000+ files, 800-1000 lines each)
   * OPTIMIZED: Accept targetFiles parameter to avoid loading unnecessary files
   */
  async initialize(projectPath, targetFiles = null) {
    try {
      // Load ts-morph conditionally
      const { Project } = await import('ts-morph');
      
      // Discover TypeScript configuration
      const tsConfigPath = await this.findTsConfig(projectPath);
      
      // Initialize project with memory-optimized settings
      // When using targetFiles, skip tsconfig to avoid auto-discovery
      const projectOptions = {
        compilerOptions: {
          ...this.options.compilerOptions,
          // Memory optimization flags
          skipLibCheck: true,
          skipDefaultLibCheck: true,
          noLib: true, // Don't load standard libraries
          allowJs: true,
          checkJs: false,
        },
        // Critical memory optimizations for large projects
        skipFileDependencyResolution: true,  // Don't resolve dependencies
        skipLoadingLibFiles: true,          // Don't load .d.ts lib files
        useInMemoryFileSystem: false,       // Use disk for large projects
        
        // Performance settings for large codebases
        resolutionHost: undefined,          // Disable resolution host
        libFolderPath: undefined,           // Don't load TypeScript libs
      };
      
      // NEVER use project tsconfig.json to avoid file resolution issues
      // Instead, load files explicitly to ensure they can be found
      if (this.options.verbose) {
        console.log(`🔧 SemanticEngine: Skipping project tsconfig.json to avoid file resolution issues`);
        if (tsConfigPath) {
          console.log(`   📋 Found tsconfig: ${tsConfigPath} (ignored for better compatibility)`);
        }
      }
      
      this.project = new Project(projectOptions);
      
      // Use provided targetFiles if available, otherwise discover
      const sourceFiles = targetFiles || await this.discoverTargetFiles(projectPath);
      
      // Filter to TypeScript/JavaScript files only for semantic analysis
      const semanticFiles = sourceFiles.filter(filePath => 
        /\.(ts|tsx|js|jsx)$/i.test(filePath)
      );
      
      if (targetFiles) {
        console.log(`🎯 Targeted files received: ${targetFiles.length} total, ${semanticFiles.length} TS/JS files`);
        if (semanticFiles.length < 10) {
          console.log(`   Files: ${semanticFiles.map(f => path.basename(f)).join(', ')}`);
        }
      }
      
      // Adaptive loading strategy based on project size and user preference
      const userMaxFiles = this.options.maxSemanticFiles;
      let maxFiles;
      
      if (userMaxFiles === -1) {
        // Unlimited: Load all files
        maxFiles = semanticFiles.length;
        console.log(`🔧 Semantic Engine config: UNLIMITED analysis (all ${semanticFiles.length} files)`);
      } else if (userMaxFiles === 0) {
        // Disable semantic analysis
        maxFiles = 0;
        console.log(`🔧 Semantic Engine config: DISABLED semantic analysis (heuristic only)`);
      } else if (userMaxFiles > 0) {
        // User-specified limit
        maxFiles = Math.min(userMaxFiles, semanticFiles.length);
        console.log(`🔧 Semantic Engine config: USER limit ${maxFiles} files (requested: ${userMaxFiles})`);
      } else {
        // Auto-detect based on project size
        maxFiles = semanticFiles.length > 1000 ? 1000 : semanticFiles.length;
        console.log(`🔧 Semantic Engine config: AUTO limit ${maxFiles} files (project has ${semanticFiles.length} files)`);
      }
      
      if (this.options.verbose) {
        console.log(`🔧 Semantic Engine detailed config:`);
        console.log(`   📊 maxSemanticFiles option: ${this.options.maxSemanticFiles}`);
        console.log(`   📈 Total semantic files: ${semanticFiles.length}`);
        console.log(`   🎯 Files to load: ${maxFiles}`);
        console.log(`   📉 Coverage: ${maxFiles > 0 ? Math.round(maxFiles/semanticFiles.length*100) : 0}%`);
      }
      
      // Skip semantic analysis if disabled
      if (maxFiles === 0) {
        console.log(`⚠️  Semantic analysis DISABLED - using heuristic rules only`);
        console.log(`💡 To enable semantic analysis, use --max-semantic-files=1000 (or higher)`);
        this.initialized = true;
        return true;
      }
      
      if (semanticFiles.length > maxFiles && maxFiles !== semanticFiles.length) {
        console.warn(`⚠️  Large semantic project detected (${semanticFiles.length} files)`);
        console.warn(`⚠️  Loading ${maxFiles} files for memory optimization (${Math.round(maxFiles/semanticFiles.length*100)}% coverage)`);
        if (userMaxFiles !== -1) {
          console.warn(`⚠️  Use --max-semantic-files=-1 to analyze ALL files (unlimited)`);
          console.warn(`⚠️  Use --max-semantic-files=${semanticFiles.length} to analyze exactly this project`);
        }
        
        const filesToLoad = semanticFiles.slice(0, maxFiles);
        
        // Load files one by one to handle any parse errors gracefully
        let successCount = 0;
        let errorCount = 0;
        
        for (const filePath of filesToLoad) {
          try {
            if (require('fs').existsSync(filePath)) {
              this.project.addSourceFileAtPath(filePath);
              successCount++;
            } else {
              errorCount++;
            }
          } catch (error) {
            if (this.options.verbose) {
              console.warn(`❌ Failed to load: ${path.basename(filePath)} - ${error.message}`);
            }
            errorCount++;
          }
        }
        
        console.log(`📊 Semantic analysis: ${successCount} files loaded, ${errorCount} skipped`);
        
      } else {
        console.log(`📊 Loading all ${semanticFiles.length} files for complete semantic analysis`);
        // For projects within limits, load all files
        this.project.addSourceFilesAtPaths(semanticFiles);
      }
      
      // Debug what ts-morph actually loaded
      const actualFiles = this.project.getSourceFiles();
      console.log(`📊 ts-morph loaded: ${actualFiles.length} files (expected: ${semanticFiles.length})`);
      if (actualFiles.length > semanticFiles.length * 2) {
        console.warn(`⚠️  ts-morph auto-discovered additional files (dependency resolution)`);
      }
      
      console.log(`🔧 Semantic Engine initialized (Memory Optimized):`);
      console.log(`   📁 Project: ${projectPath}`);
      console.log(`   📋 TS Config: ${tsConfigPath || 'default (minimal)'}`);
      console.log(`   📄 Files loaded: ${this.project.getSourceFiles().length}`);
      console.log(`   🎯 Targeting mode: ${targetFiles ? 'Filtered files' : 'Auto-discovery'}`);
      console.log(`   💾 Memory mode: Optimized for large projects`);
      
      this.initialized = true;
      return true;
      
    } catch (error) {
      console.warn(`⚠️  ts-morph not available or initialization failed:`, error.message);
      return false;
    }
  }

  /**
   * Get or create Symbol Table for a file
   */
  async getSymbolTable(filePath) {
    if (!this.initialized) {
      throw new Error('Semantic Engine not initialized');
    }

    const absolutePath = path.resolve(filePath);
    
    // Check cache first
    if (this.fileCache.has(absolutePath)) {
      this.stats.cacheHits++;
      return this.fileCache.get(absolutePath);
    }
    
    this.stats.cacheMisses++;
    
    // Get source file
    const sourceFile = this.project.getSourceFile(absolutePath);
    if (!sourceFile) {
      console.warn(`⚠️  File not found in project: ${filePath}`);
      return null;
    }
    
    // Build symbol table
    const symbolTable = await this.buildSymbolTable(sourceFile);
    
    // Cache the result
    if (this.options.enableCaching) {
      this.cacheSymbolTable(absolutePath, symbolTable);
    }
    
    this.stats.filesAnalyzed++;
    return symbolTable;
  }

  /**
   * Build comprehensive symbol table for a file
   */
  async buildSymbolTable(sourceFile) {
    const symbols = {
      // File metadata
      filePath: sourceFile.getFilePath(),
      fileName: sourceFile.getBaseName(),
      
      // Imports and exports
      imports: this.extractImports(sourceFile),
      exports: this.extractExports(sourceFile),
      
      // Declarations
      functions: this.extractFunctions(sourceFile),
      classes: this.extractClasses(sourceFile),
      interfaces: this.extractInterfaces(sourceFile),
      variables: this.extractVariables(sourceFile),
      constants: this.extractConstants(sourceFile),
      
      // React specific (if applicable)
      hooks: this.extractHooks(sourceFile),
      components: this.extractComponents(sourceFile),
      
      // Call analysis
      functionCalls: this.extractFunctionCalls(sourceFile),
      methodCalls: this.extractMethodCalls(sourceFile),
      
      // Cross-file references
      crossFileReferences: this.extractCrossFileReferences(sourceFile),
      
      // Metadata
      lastModified: Date.now(),
      analysisTime: 0
    };
    
    // Add cross-file dependency information
    if (this.options.crossFileAnalysis) {
      symbols.dependencies = await this.analyzeDependencies(sourceFile);
    }
    
    return symbols;
  }

  /**
   * Extract import statements
   */
  extractImports(sourceFile) {
    const imports = [];
    
    sourceFile.getImportDeclarations().forEach(importDecl => {
      const moduleSpecifier = importDecl.getModuleSpecifierValue();
      
      // Named imports
      const namedImports = importDecl.getNamedImports().map(namedImport => ({
        name: namedImport.getName(),
        alias: namedImport.getAliasNode()?.getText(),
        line: sourceFile.getLineAndColumnAtPos(namedImport.getStart()).line
      }));
      
      // Default import
      const defaultImport = importDecl.getDefaultImport();
      
      imports.push({
        module: moduleSpecifier,
        defaultImport: defaultImport?.getText(),
        namedImports,
        line: sourceFile.getLineAndColumnAtPos(importDecl.getStart()).line,
        isTypeOnly: importDecl.isTypeOnly(),
        resolvedPath: this.resolveModule(moduleSpecifier, sourceFile)
      });
    });
    
    return imports;
  }

  /**
   * Extract function calls (cho C047 analysis)
   */
  extractFunctionCalls(sourceFile) {
    const calls = [];
    
    sourceFile.getDescendantsOfKind(SyntaxKind.CallExpression).forEach(callExpr => {
      const expression = callExpr.getExpression();
      
      calls.push({
        functionName: expression.getText(),
        arguments: callExpr.getArguments().map(arg => ({
          text: arg.getText(),
          type: this.getExpressionType(arg),
          line: sourceFile.getLineAndColumnAtPos(arg.getStart()).line
        })),
        line: sourceFile.getLineAndColumnAtPos(callExpr.getStart()).line,
        column: sourceFile.getLineAndColumnAtPos(callExpr.getStart()).column,
        
        // Detailed analysis for retry patterns
        isRetryPattern: this.isRetryPattern(callExpr),
        isConditionalCall: this.isConditionalCall(callExpr),
        parentContext: this.getParentContext(callExpr)
      });
    });
    
    return calls;
  }

  /**
   * Extract React hooks usage
   */
  extractHooks(sourceFile) {
    const hooks = [];
    
    sourceFile.getDescendantsOfKind(SyntaxKind.CallExpression).forEach(callExpr => {
      const expression = callExpr.getExpression();
      const functionName = expression.getText();
      
      // Detect hook patterns
      if (functionName.startsWith('use') || this.isKnownHook(functionName)) {
        hooks.push({
          hookName: functionName,
          arguments: callExpr.getArguments().map(arg => arg.getText()),
          line: sourceFile.getLineAndColumnAtPos(callExpr.getStart()).line,
          
          // Special analysis for useQuery, useMutation, etc.
          isQueryHook: this.isQueryHook(functionName),
          retryConfig: this.extractRetryConfig(callExpr)
        });
      }
    });
    
    return hooks;
  }

  /**
   * Analyze cross-file dependencies
   */
  async analyzeDependencies(sourceFile) {
    const dependencies = [];
    
    // Analyze imported symbols usage
    sourceFile.getImportDeclarations().forEach(importDecl => {
      const moduleSpecifier = importDecl.getModuleSpecifierValue();
      const resolvedPath = this.resolveModule(moduleSpecifier, sourceFile);
      
      if (resolvedPath && this.project.getSourceFile(resolvedPath)) {
        dependencies.push({
          type: 'import',
          module: moduleSpecifier,
          resolvedPath,
          usages: this.findSymbolUsages(sourceFile, importDecl.getNamedImports())
        });
      }
    });
    
    return dependencies;
  }

  /**
   * Utility methods for pattern detection
   */
  isRetryPattern(callExpr) {
    const functionName = callExpr.getExpression().getText();
    
    // Known retry functions
    const retryFunctions = ['retry', 'retries', 'withRetry', 'retryWhen'];
    if (retryFunctions.some(fn => functionName.includes(fn))) {
      return true;
    }
    
    // Check for retry configuration in arguments
    const args = callExpr.getArguments();
    return args.some(arg => {
      const argText = arg.getText();
      return /retry|retries/i.test(argText);
    });
  }

  isQueryHook(functionName) {
    const queryHooks = ['useQuery', 'useMutation', 'useInfiniteQuery', 'useSuspenseQuery'];
    return queryHooks.includes(functionName);
  }

  extractRetryConfig(callExpr) {
    const args = callExpr.getArguments();
    
    // Look for retry configuration in arguments
    for (const arg of args) {
      const argText = arg.getText();
      
      // Object literal with retry config
      if (arg.getKind() === 204) { // ObjectLiteralExpression
        const retryProperty = arg.getProperties().find(prop => 
          prop.getName && prop.getName() === 'retry'
        );
        
        if (retryProperty) {
          return {
            hasRetryConfig: true,
            retryValue: retryProperty.getValueNode()?.getText(),
            line: retryProperty.getStartLineNumber()
          };
        }
      }
    }
    
    return { hasRetryConfig: false };
  }

  /**
   * Resolve module path
   */
  resolveModule(moduleSpecifier, sourceFile) {
    try {
      // Use ts-morph's resolution if available
      if (this.options.enableTypeChecker && sourceFile.getProject().getTypeChecker) {
        const symbol = sourceFile.getProject().getTypeChecker()
          .getSymbolAtLocation(sourceFile.getImportDeclarations()
            .find(imp => imp.getModuleSpecifierValue() === moduleSpecifier)
            ?.getModuleSpecifier());
        
        if (symbol?.getDeclarations()?.[0]) {
          return symbol.getDeclarations()[0].getSourceFile().getFilePath();
        }
      }
      
      // Basic resolution
      if (moduleSpecifier.startsWith('.')) {
        const dir = path.dirname(sourceFile.getFilePath());
        return path.resolve(dir, moduleSpecifier);
      }
      
      return null;
    } catch (error) {
      return null;
    }
  }

  /**
   * Memory and cache management
   */
  cacheSymbolTable(filePath, symbolTable) {
    // Check memory limits
    if (this.fileCache.size >= this.options.maxCacheSize) {
      this.evictOldestCache();
    }
    
    this.fileCache.set(filePath, symbolTable);
    this.updateMemoryStats();
  }

  evictOldestCache() {
    // Simple LRU eviction
    const oldest = this.fileCache.keys().next().value;
    this.fileCache.delete(oldest);
  }

  updateMemoryStats() {
    this.stats.memoryUsage = process.memoryUsage().heapUsed;
  }

  /**
   * Configuration discovery
   */
  async findTsConfig(projectPath) {
    const candidates = [
      path.join(projectPath, 'tsconfig.json'),
      path.join(projectPath, 'jsconfig.json'),
      path.join(projectPath, '..', 'tsconfig.json')
    ];
    
    for (const candidate of candidates) {
      try {
        await fs.access(candidate);
        return candidate;
      } catch (error) {
        continue;
      }
    }
    
    return null;
  }

  /**
   * Discover target files with intelligent filtering for large projects
   * Optimized for projects with 3000+ files, 800-1000 lines each
   */
  async discoverTargetFiles(projectPath) {
    const fs = await import('fs');
    const glob = require('glob');
    
    try {
      const patterns = [
        '**/*.ts',
        '**/*.tsx',
        '**/*.js',      // Include JS files for semantic analysis
        '**/*.jsx'      // Include JSX files  
        // Both TS and JS files for comprehensive analysis
      ];
      
      // Exclude common directories and large files
      const excludePatterns = [
        '**/node_modules/**',
        '**/dist/**',
        '**/build/**',
        '**/coverage/**',
        '**/.git/**',
        '**/.next/**',
        '**/out/**',
        '**/*.min.js',
        '**/*.min.ts',
        '**/*.d.ts',        // Skip declaration files
        '**/vendor/**',
        '**/third-party/**'
      ];
      
      // Find all matching files
      const allFiles = [];
      for (const pattern of patterns) {
        const globPattern = path.join(projectPath, pattern);
        const files = glob.sync(globPattern, {
          ignore: excludePatterns.map(exclude => path.join(projectPath, exclude))
        });
        allFiles.push(...files);
      }
      
      // Filter by file size for memory optimization
      const targetFiles = [];
      for (const filePath of allFiles) {
        try {
          const stats = await fs.stat(filePath);
          // Skip files larger than 100KB (typically auto-generated)
          if (stats.size < 100 * 1024) {
            targetFiles.push(filePath);
          } else {
            console.debug(`⚠️  Skipping large file: ${path.basename(filePath)} (${Math.round(stats.size / 1024)}KB)`);
          }
        } catch (error) {
          // Skip files that can't be stat'd
          continue;
        }
      }
      
      console.log(`📁 File discovery: ${targetFiles.length}/${allFiles.length} files selected (memory optimized)`);
      return targetFiles;
      
    } catch (error) {
      console.warn(`⚠️  File discovery failed, using basic patterns:`, error.message);
      return this.discoverSourceFiles(projectPath);
    }
  }

  async discoverSourceFiles(projectPath) {
    const patterns = [
      '**/*.ts',
      '**/*.tsx', 
      '**/*.js',
      '**/*.jsx'
    ];
    
    // Exclude common directories
    const excludePatterns = [
      '**/node_modules/**',
      '**/dist/**',
      '**/build/**',
      '**/.git/**'
    ];
    
    return patterns.map(pattern => path.join(projectPath, pattern))
      .filter(filePath => !excludePatterns.some(exclude => 
        filePath.includes(exclude.replace('**/', ''))
      ));
  }

  /**
   * Cleanup and statistics
   */
  async cleanup() {
    if (this.project) {
      // Clear caches
      this.fileCache.clear();
      this.symbolTable.clear();
      
      console.log(`📊 Semantic Engine Stats:`);
      console.log(`   📄 Files analyzed: ${this.stats.filesAnalyzed}`);
      console.log(`   🎯 Cache hits: ${this.stats.cacheHits}`);
      console.log(`   ❌ Cache misses: ${this.stats.cacheMisses}`);
      console.log(`   💾 Memory usage: ${Math.round(this.stats.memoryUsage / 1024 / 1024)}MB`);
    }
  }

  getStats() {
    return {
      ...this.stats,
      cacheSize: this.fileCache.size,
      symbolTableSize: this.symbolTable.size,
      isInitialized: this.initialized
    };
  }

  // Stub methods for full extraction implementation
  extractExports(sourceFile) { return []; }
  extractFunctions(sourceFile) { return []; }
  extractClasses(sourceFile) { return []; }
  extractInterfaces(sourceFile) { return []; }
  extractVariables(sourceFile) { return []; }
  extractConstants(sourceFile) { return []; }
  extractComponents(sourceFile) { return []; }
  extractMethodCalls(sourceFile) { return []; }
  extractCrossFileReferences(sourceFile) { return []; }
  getExpressionType(expr) { return 'unknown'; }
  isConditionalCall(callExpr) { return false; }
  getParentContext(callExpr) { return null; }
  isKnownHook(functionName) { return false; }
  findSymbolUsages(sourceFile, namedImports) { return []; }

  /**
   * Check if symbol engine is ready for symbol-based analysis
   * @returns {boolean} true if project is initialized and ready
   */
  isSymbolEngineReady() {
    return this.initialized && this.project !== null;
  }
}

module.exports = SemanticEngine;
