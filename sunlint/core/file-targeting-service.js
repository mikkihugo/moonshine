const path = require('path');
const fs = require('fs');
const chalk = require('chalk');
const { minimatch } = require('minimatch');

/**
 * File Targeting Service
 * Handles complex file inclusion/exclusion logic for multi-language support
 * Rule C005: Single responsibility - only handle file targeting
 * Rule C006: getTargetFiles - verb-noun naming pattern
 */
class FileTargetingService {
  constructor() {
    this.supportedLanguages = ['typescript', 'javascript', 'dart', 'kotlin', 'java', 'swift'];
  }

  /**
   * Get target files based on enhanced configuration
   * ENHANCED: Uses metadata for intelligent file targeting
   */
  async getTargetFiles(inputPaths, config, cliOptions = {}) {
    try {
      const startTime = Date.now();
      const metadata = config._metadata;
      
      if (cliOptions.verbose) {
        console.log(chalk.cyan(`📁 File Targeting: ${this.getTargetingMode(metadata)}`));
        if (metadata?.shouldBypassProjectDiscovery) {
          console.log(chalk.blue(`🎯 Optimized targeting for ${metadata.analysisScope}`));
        }
      }
      
      let allFiles = [];
      
      // Use enhanced targeting based on metadata
      if (metadata?.shouldBypassProjectDiscovery) {
        allFiles = await this.collectTargetedFiles(inputPaths, config, cliOptions);
      } else {
        allFiles = await this.collectProjectFiles(inputPaths, config, cliOptions);
      }

      // Apply filtering logic
      const targetFiles = this.applyFiltering(allFiles, config, cliOptions);
      
      const duration = Date.now() - startTime;
      
      if (cliOptions.verbose) {
        console.log(chalk.green(`✅ File targeting completed in ${duration}ms (${targetFiles.length} files)`));
      }
      
      return {
        files: targetFiles,
        stats: this.generateStats(targetFiles, config),
        timing: { duration, filesPerMs: targetFiles.length / Math.max(duration, 1) }
      };
    } catch (error) {
      console.error('❌ FileTargetingService error:', error);
      throw error;
    }
  }

  /**
   * Get targeting mode description
   */
  getTargetingMode(metadata) {
    if (!metadata) return 'legacy';
    
    if (metadata.shouldBypassProjectDiscovery) {
      return metadata.analysisScope === 'file' ? 'single_file' : 'folder_targeted';
    } else {
      return 'project_wide';
    }
  }

  /**
   * Collect files with targeted approach (bypassing project discovery)
   */
  async collectTargetedFiles(inputPaths, config, cliOptions) {
    const files = [];
    
    for (const inputPath of inputPaths) {
      const resolvedPath = path.resolve(inputPath);
      
      if (!fs.existsSync(resolvedPath)) {
        if (cliOptions.verbose) {
          console.log(chalk.yellow(`⚠️  Path not found: ${inputPath}`));
        }
        continue;
      }
      
      const stat = fs.statSync(resolvedPath);
      
      if (stat.isFile()) {
        // Single file targeting
        files.push(resolvedPath);
      } else if (stat.isDirectory()) {
        // Folder-only targeting (no recursive project scan)
        const folderFiles = await this.collectFolderFiles(resolvedPath);
        files.push(...folderFiles);
      }
    }
    
    return files;
  }

  /**
   * Collect files from specific folder (no project-wide scanning)
   */
  async collectFolderFiles(folderPath) {
    const files = [];
    const targetExtensions = ['.ts', '.tsx', '.js', '.jsx', '.dart', '.kt', '.kts'];
    
    try {
      const entries = fs.readdirSync(folderPath);
      
      for (const entry of entries) {
        const fullPath = path.join(folderPath, entry);
        const stat = fs.statSync(fullPath);
        
        if (stat.isFile()) {
          const ext = path.extname(fullPath);
          if (targetExtensions.includes(ext)) {
            files.push(path.resolve(fullPath));
          }
        } else if (stat.isDirectory() && !this.shouldSkipDirectory(entry)) {
          // Recursive collection within target folder only
          const subFiles = await this.collectFolderFiles(fullPath);
          files.push(...subFiles);
        }
      }
    } catch (error) {
      console.warn(`⚠️ Error reading folder ${folderPath}: ${error.message}`);
    }
    
    return files;
  }

  /**
   * Collect files with project-wide approach (original logic)
   */
  async collectProjectFiles(inputPaths, config, cliOptions) {
    const allFiles = [];
    
    // Use original collection logic for project-wide analysis
    for (const inputPath of inputPaths) {
      const files = await this.collectFiles(inputPath);
      allFiles.push(...files);
    }
    
    return allFiles;
  }

  /**
   * Check if directory should be skipped
   */
  shouldSkipDirectory(dirName) {
    const skipDirs = [
      'node_modules', '.git', 'dist', 'build', 'coverage', 
      '.next', '.nuxt', 'vendor', 'target', 'generated'
    ];
    return skipDirs.includes(dirName);
  }

  /**
   * Apply comprehensive filtering logic
   * Priority: CLI > Config > Default
   * Rule C005: Single responsibility - only filtering logic
   */
  applyFiltering(files, config, cliOptions) {
    let filteredFiles = [...files];
    const debug = cliOptions?.debug || false;

    if (debug) console.log(`🔍 [DEBUG] applyFiltering: start with ${filteredFiles.length} files`);
    if (debug) console.log(`🔍 [DEBUG] config.include:`, config.include);
    if (debug) console.log(`🔍 [DEBUG] cliOptions.include:`, cliOptions.include);

    // 1. Apply config include patterns first (medium priority)
    if (config.include && config.include.length > 0) {
      filteredFiles = this.applyIncludePatterns(filteredFiles, config.include, debug);
      if (debug) console.log(`🔍 [DEBUG] After config include: ${filteredFiles.length} files`);
    }

    // 2. Apply CLI include overrides (highest priority - completely overrides config)
    if (cliOptions.include) {
      // CLI include completely replaces config include - start fresh from all files
      filteredFiles = this.applyIncludePatterns([...files], cliOptions.include, debug);
    }

    // 3. Apply config exclude patterns
    if (config.exclude && config.exclude.length > 0) {
      if (debug) console.log(`🔍 [DEBUG] About to apply config exclude patterns: ${config.exclude}`);
      if (debug) console.log(`🔍 [DEBUG] Files before config exclude: ${filteredFiles.length}`);
      filteredFiles = this.applyExcludePatterns(filteredFiles, config.exclude, debug);
      if (debug) console.log(`🔍 [DEBUG] Files after config exclude: ${filteredFiles.length}`);
    }

    // 4. Apply CLI exclude overrides (highest priority)
    if (cliOptions.exclude) {
      if (debug) console.log(`🔍 [DEBUG] About to apply CLI exclude patterns: ${cliOptions.exclude}`);
      if (debug) console.log(`🔍 [DEBUG] Files before CLI exclude: ${filteredFiles.length}`);
      filteredFiles = this.applyExcludePatterns(filteredFiles, cliOptions.exclude, debug);
      if (debug) console.log(`🔍 [DEBUG] Files after CLI exclude: ${filteredFiles.length}`);
    }

    // 5. Apply language-specific filtering
    if (cliOptions.languages || config.languages) {
      filteredFiles = this.applyLanguageFiltering(filteredFiles, config, cliOptions, debug);
      if (debug) console.log(`🔍 [DEBUG] After language filtering: ${filteredFiles.length} files`);
    }

    // 6. Apply only-source filtering (exclude tests, configs, etc.)
    if (cliOptions.onlySource) {
      filteredFiles = this.applyOnlySourceFiltering(filteredFiles);
      if (debug) console.log(`🔍 [DEBUG] After onlySource filtering: ${filteredFiles.length} files`);
    } else {
      // 8. Handle test files normally
      if (config.testPatterns) {
        filteredFiles = this.handleTestFiles(filteredFiles, config.testPatterns, cliOptions, config);
        if (debug) console.log(`🔍 [DEBUG] After test files handling: ${filteredFiles.length} files`);
      }
    }

    if (debug) console.log(`🔍 [DEBUG] Final filtered files: ${filteredFiles.length}`);
    return filteredFiles;
  }

  /**
   * Apply language-specific filtering
   * Rule C005: Single responsibility - language filtering only
   */
  applyLanguageFiltering(files, config, cliOptions, debug = false) {
    if (debug) console.log(`🔍 [DEBUG] === applyLanguageFiltering ENTRY ===`);
    if (debug) console.log(`🔍 [DEBUG] Input files.length: ${files.length}`);
    if (debug) console.log(`🔍 [DEBUG] Sample input files:`, files.slice(0, 3));
    
    // Determine target languages from CLI or config
    let targetLanguages;
    if (cliOptions.languages) {
      targetLanguages = cliOptions.languages.split(',').map(l => l.trim());
    } else if (Array.isArray(config.languages)) {
      targetLanguages = config.languages;
    } else {
      targetLanguages = Object.keys(config.languages || {});
    }

    if (debug) console.log(`🔍 [DEBUG] applyLanguageFiltering: cliOptions.languages = ${cliOptions.languages}`);
    if (debug) console.log(`🔍 [DEBUG] applyLanguageFiltering: config.languages =`, config.languages);
    if (debug) console.log(`🔍 [DEBUG] applyLanguageFiltering: targetLanguages =`, targetLanguages);

    if (targetLanguages.length === 0) {
      if (debug) console.log(`🔍 [DEBUG] applyLanguageFiltering: No language filtering, returning all files`);
      return files; // No language filtering
    }

    let languageFiles = [];

    for (const language of targetLanguages) {
      if (debug) console.log(`🔍 [DEBUG] Processing language: ${language}`);
      if (Array.isArray(config.languages)) {
        // New array format - use isLanguageFile method
        const langFiles = files.filter(file => this.isLanguageFile(file, language));
        languageFiles.push(...langFiles);
        if (debug) console.log(`🔍 [DEBUG] Array format - found ${langFiles.length} files for ${language}`);
      } else {
        // Legacy object format - use include/exclude patterns
        const langConfig = config.languages[language];
        if (!langConfig) {
          if (debug) console.log(`🔍 [DEBUG] No config for language: ${language}`);
          continue;
        }

        let langFiles = [...files];
        if (debug) console.log(`🔍 [DEBUG] Starting with ${langFiles.length} files for ${language}`);

        // Apply language-specific include patterns
        if (langConfig.include && langConfig.include.length > 0) {
          langFiles = this.applyIncludePatterns(langFiles, langConfig.include, debug);
          if (debug) console.log(`🔍 [DEBUG] After include patterns ${langConfig.include}: ${langFiles.length} files`);
        }

        // Apply language-specific exclude patterns
        if (langConfig.exclude && langConfig.exclude.length > 0) {
          langFiles = this.applyExcludePatterns(langFiles, langConfig.exclude, debug);
          if (debug) console.log(`🔍 [DEBUG] After exclude patterns ${langConfig.exclude}: ${langFiles.length} files`);
        }

        languageFiles.push(...langFiles);
        if (debug) console.log(`🔍 [DEBUG] Added ${langFiles.length} files for ${language}, total: ${languageFiles.length}`);
      }
    }

    // Remove duplicates
    const finalFiles = [...new Set(languageFiles)];
    if (debug) console.log(`🔍 [DEBUG] Final language files after dedup: ${finalFiles.length}`);
    return finalFiles;
  }

  /**
   * Apply include patterns using minimatch
   * Rule C006: applyIncludePatterns - verb-noun naming
   */
  applyIncludePatterns(files, patterns, debug = false) {
    if (!patterns) return files;
    
    // Normalize patterns to array
    const patternArray = Array.isArray(patterns) ? patterns : [patterns];
    if (patternArray.length === 0) return files;
    
    if (debug) console.log(`🔍 [DEBUG] applyIncludePatterns - input files:`, files.length);
    if (debug) console.log(`🔍 [DEBUG] applyIncludePatterns - patterns:`, patternArray);
    if (debug) console.log(`🔍 [DEBUG] applyIncludePatterns - sample input files:`, files.slice(0, 3));
    
    const result = files.filter(file => {
      return patternArray.some(pattern => {
        const normalizedFile = this.normalizePath(file);
        const match = minimatch(normalizedFile, pattern, { dot: true });
        if (debug && file.includes('.ts') && !file.includes('.test.')) {
          console.log(`🔍 [DEBUG] Testing: '${file}' -> '${normalizedFile}' vs '${pattern}' = ${match}`);
        }
        return match;
      });
    });
    
    if (debug) console.log(`🔍 [DEBUG] applyIncludePatterns - result:`, result.length);
    return result;
  }

  /**
   * Apply exclude patterns using minimatch
   * Rule C006: applyExcludePatterns - verb-noun naming
   */
  applyExcludePatterns(files, patterns, debug = false) {
    if (!patterns) return files;
    
    // Normalize patterns to array
    const patternArray = Array.isArray(patterns) ? patterns : [patterns];
    if (patternArray.length === 0) return files;
    
    // Filter out negation patterns (starting with !) - these should not be in exclude patterns
    const excludePatterns = patternArray.filter(pattern => !pattern.startsWith('!'));
    
    if (debug) console.log(`🔍 [DEBUG] applyExcludePatterns - input files: ${files.length}`);
    if (debug) console.log(`🔍 [DEBUG] applyExcludePatterns - original patterns:`, patternArray);
    if (debug) console.log(`🔍 [DEBUG] applyExcludePatterns - filtered patterns:`, excludePatterns);
    
    if (excludePatterns.length === 0) return files;
    
    const result = files.filter(file => {
      return !excludePatterns.some(pattern => {
        const normalizedFile = this.normalizePath(file);
        const match = minimatch(normalizedFile, pattern, { dot: true });
        return match;
      });
    });
    
    if (debug) console.log(`🔍 [DEBUG] applyExcludePatterns - result: ${result.length}`);
    return result;
  }

  /**
   * Handle test files with special rules
   * Rule C005: Single responsibility - test file handling only
   */
  handleTestFiles(files, testPatterns, cliOptions, config = {}) {
    // Normalize testPatterns - can be array or object with include property
    const patterns = Array.isArray(testPatterns) ? testPatterns : testPatterns.include || [];
    
    // Check CLI options first (highest priority)
    if (cliOptions.excludeTests === true) {
      return this.applyExcludePatterns(files, patterns);
    }
    
    if (cliOptions.includeTests === true) {
      return files;
    }
    
    // Check config includeTests setting
    if (config.includeTests === false) {
      return this.applyExcludePatterns(files, patterns);
    }
    
    // Default behavior - include tests
    return files;
  }

  /**
   * Collect files recursively from input path
   * Rule C006: collectFiles - verb-noun naming
   */
  async collectFiles(inputPath) {
    const files = [];
    
    try {
      const stats = fs.statSync(inputPath);

      if (stats.isFile()) {
        files.push(path.resolve(inputPath));
      } else if (stats.isDirectory()) {
        const dirFiles = await this.collectFilesFromDirectory(inputPath);
        files.push(...dirFiles);
      }
    } catch (error) {
      if (error.code === 'ENOENT') {
        console.warn(`⚠️ Path not found: ${inputPath}`);
        return files; // Return empty array instead of throwing
      }
      throw error; // Re-throw other errors
    }

    return files;
  }

  /**
   * Collect files from directory recursively
   * Rule C006: collectFilesFromDirectory - verb-noun naming
   */
  async collectFilesFromDirectory(dirPath) {
    const files = [];
    const entries = fs.readdirSync(dirPath);

    for (const entry of entries) {
      const fullPath = path.join(dirPath, entry);
      const stats = fs.statSync(fullPath);

      if (stats.isFile()) {
        files.push(path.resolve(fullPath));
      } else if (stats.isDirectory()) {
        const subFiles = await this.collectFilesFromDirectory(fullPath);
        files.push(...subFiles);
      }
    }

    return files;
  }

  /**
   * Normalize file path for cross-platform compatibility and pattern matching
   * Rule C006: normalizePath - verb-noun naming
   */
  normalizePath(filePath) {
    // Convert to relative path from current working directory for pattern matching
    const relativePath = path.relative(process.cwd(), filePath);
    // Normalize path separators for cross-platform compatibility
    return relativePath.replace(/\\/g, '/');
  }

  /**
   * Generate targeting statistics
   * Rule C006: generateStats - verb-noun naming
   */
  generateStats(files, config) {
    const stats = {
      totalFiles: files.length,
      byLanguage: {},
      byCategory: {
        source: 0,
        test: 0,
        config: 0,
        other: 0
      }
    };

    // Count by language - handle both array and object formats
    if (config.languages) {
      if (Array.isArray(config.languages)) {
        // New format: array of language names
        for (const language of config.languages) {
          const langFiles = files.filter(file => this.isLanguageFile(file, language));
          stats.byLanguage[language] = langFiles.length;
        }
      } else {
        // Legacy format: object with language configs
        for (const [language, langConfig] of Object.entries(config.languages)) {
          const langFiles = this.applyIncludePatterns(files, langConfig.include);
          stats.byLanguage[language] = langFiles.length;
        }
      }
    }

    // Count by category
    const testPatterns = config.testPatterns?.include || [];
    const configPatterns = ['**/*.config.*', '**/config/**', '**/.env*'];

    for (const file of files) {
      const normalizedFile = this.normalizePath(file);
      
      if (testPatterns.some(pattern => minimatch(normalizedFile, pattern))) {
        stats.byCategory.test++;
      } else if (configPatterns.some(pattern => minimatch(normalizedFile, pattern))) {
        stats.byCategory.config++;
      } else if (this.isSourceFile(normalizedFile, config)) {
        stats.byCategory.source++;
      } else {
        stats.byCategory.other++;
      }
    }

    return stats;
  }

  /**
   * Check if file is a source file (not test/config)
   * Rule C012: Query method - returns boolean
   */
  isSourceFile(filePath, config) {
    const sourceExtensions = ['.ts', '.tsx', '.js', '.jsx', '.dart', '.kt', '.java', '.swift'];
    const ext = path.extname(filePath);
    return sourceExtensions.includes(ext);
  }

  /**
   * Check if file matches language type
   * Rule C006: isLanguageFile - verb-noun naming
   */
  isLanguageFile(filePath, language) {
    const normalizedPath = this.normalizePath(filePath);
    const ext = path.extname(normalizedPath).toLowerCase();
    
    switch (language) {
      case 'typescript':
        return ['.ts', '.tsx', '.mts', '.cts'].includes(ext) && !normalizedPath.includes('.d.ts');
      case 'javascript':
        return ['.js', '.jsx', '.mjs', '.cjs'].includes(ext) && !normalizedPath.includes('.min.js');
      case 'dart':
        return ext === '.dart' && !normalizedPath.match(/\.(g|freezed|mocks)\.dart$/);
      case 'kotlin':
        return ['.kt', '.kts'].includes(ext);
      case 'swift':
        return ext === '.swift';
      case 'python':
        return ext === '.py';
      default:
        return false;
    }
  }

  /**
   * Apply only-source filtering: exclude tests, configs, generated files
   * Rule C012: Command method - filters array
   */
  applyOnlySourceFiltering(files) {
    const sourceOnlyPatterns = [
      '**/*.test.*',
      '**/*.spec.*',
      '**/*Test.*',
      '**/*Spec.*',
      '**/*.config.*',
      '**/*.generated.*',
      '**/*.d.ts',
      '**/test/**',
      '**/tests/**',
      '**/spec/**',
      '**/specs/**',
      '**/config/**',
      '**/configs/**',
      '**/dist/**',
      '**/build/**',
      '**/coverage/**',
      '**/.next/**',
      '**/node_modules/**'
    ];

    return files.filter(file => {
      const relativePath = this.normalizePath(file);
      const shouldExclude = sourceOnlyPatterns.some(pattern => minimatch(relativePath, pattern));
      return !shouldExclude;
    });
  }
}

module.exports = FileTargetingService;
