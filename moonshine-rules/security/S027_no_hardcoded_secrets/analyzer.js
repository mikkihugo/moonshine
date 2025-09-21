const fs = require('fs');
const path = require('path');

class S027CategorizedAnalyzer {
  constructor() {
    this.ruleId = 'S027';
    this.ruleName = 'No Hardcoded Secrets (Categorized)';
    this.description = 'Phát hiện thông tin bảo mật theo categories với độ ưu tiên khác nhau';
    
    // Load categories config
    this.config = this.loadConfig();
    this.categories = this.config.categories;
    this.globalExcludePatterns = this.config.global_exclude_patterns.map(p => new RegExp(p, 'i'));
    this.minLength = this.config.min_length || 8;
    this.maxLength = this.config.max_length || 1000;
    
    // Compile patterns for performance
    this.compilePatterns();
  }
  
  loadConfig() {
    const configPath = path.join(__dirname, 'categories.json');
    try {
      const config = JSON.parse(fs.readFileSync(configPath, 'utf8'));
      return config.S027;
    } catch (error) {
      console.error('Failed to load S027 categories config:', error.message);
      return { categories: [], global_exclude_patterns: [] };
    }
  }
  
  compilePatterns() {
    this.categories.forEach(category => {
      category.compiledPatterns = category.patterns.map(p => ({
        regex: new RegExp(p, 'gm'),
        original: p
      }));
      
      if (category.exclude_patterns) {
        category.compiledExcludePatterns = category.exclude_patterns.map(p => new RegExp(p, 'i'));
      }
    });
  }
  
  async analyze(files, language, options = {}) {
    const violations = [];
    this.currentFilePath = '';
    
    for (const filePath of files) {
      // Skip build/dist/node_modules
      if (this.shouldSkipFile(filePath)) {
        continue;
      }
      
      this.currentFilePath = filePath;
      
      try {
        const content = fs.readFileSync(filePath, 'utf8');
        const fileViolations = this.analyzeFile(content, filePath);
        violations.push(...fileViolations);
      } catch (error) {
        if (options.verbose) {
          console.error(`Error analyzing ${filePath}:`, error.message);
        }
      }
    }
    
    return violations;
  }
  
  shouldSkipFile(filePath) {
    const skipPatterns = [
      'build/', 'dist/', 'node_modules/', '.git/',
      'coverage/', '.next/', '.cache/', 'tmp/',
      '.lock', '.log', '.min.js', '.bundle.js'
    ];
    
    return skipPatterns.some(pattern => filePath.includes(pattern));
  }
  
  analyzeFile(content, filePath) {
    const violations = [];
    // Handle different line endings (Windows \r\n, Unix \n, Mac \r)
    const lines = content.split(/\r?\n/);
    
    // Check if this is a test file for context
    const isTestFile = this.isTestFile(filePath);
    
    lines.forEach((line, index) => {
      const lineNumber = index + 1;
      const trimmedLine = line.trim();
      
      // Skip comments and imports
      if (this.isCommentOrImport(trimmedLine)) {
        return;
      }
      
      // Check global exclude patterns first
      if (this.matchesGlobalExcludes(line)) {
        return;
      }
      
      // Check each category
      this.categories.forEach(category => {
        const categoryViolations = this.checkCategory(
          category, line, lineNumber, filePath, isTestFile
        );
        violations.push(...categoryViolations);
      });
    });
    
    return violations;
  }
  
  isTestFile(filePath) {
    const testPatterns = [
      /\.(test|spec)\./i,
      /__tests__/i,
      /\/tests?\//i,
      /\/spec\//i,
      /setupTests/i,
      /testSetup/i,
      /test[-_]/i,  // Matches test- or test_
      /^.*\/test[^\/]*\.js$/i  // Matches files starting with test
    ];
    
    return testPatterns.some(pattern => pattern.test(filePath));
  }
  
  isCommentOrImport(line) {
    return line.startsWith('//') || line.startsWith('/*') || 
           line.startsWith('import') || line.startsWith('export') ||
           line.startsWith('*') || line.startsWith('<');
  }
  
  matchesGlobalExcludes(line) {
    return this.globalExcludePatterns.some(pattern => pattern.test(line));
  }
  
  checkCategory(category, line, lineNumber, filePath, isTestFile) {
    const violations = [];
    
    category.compiledPatterns.forEach(({ regex, original }) => {
      let match;
      
      // Reset regex lastIndex for global patterns
      regex.lastIndex = 0;
      
      while ((match = regex.exec(line)) !== null) {
        const matchedText = match[0];
        const column = match.index + 1;
        
        // Check length constraints
        if (matchedText.length < this.minLength || matchedText.length > this.maxLength) {
          continue;
        }
        
        // Check category-specific excludes
        if (category.compiledExcludePatterns && 
            category.compiledExcludePatterns.some(pattern => pattern.test(matchedText))) {
          continue;
        }
        
        // Be more lenient in test files for lower severity categories
        // But still report critical/high severity issues even in test files
        if (isTestFile && category.severity === 'low') {
          continue;
        }
        
        violations.push({
          file: filePath,
          line: lineNumber,
          column: column,
          message: `[${category.name}] Potential ${category.severity} security risk: '${matchedText}'. ${category.description}`,
          severity: this.mapSeverity(category.severity),
          ruleId: this.ruleId,
          category: category.name,
          categoryDescription: category.description,
          matchedPattern: original,
          matchedText: matchedText
        });
      }
    });
    
    return violations;
  }
  
  mapSeverity(categorySeverity) {
    const severityMap = {
      'critical': 'error',
      'high': 'warning', 
      'medium': 'warning',
      'low': 'info'
    };
    
    return severityMap[categorySeverity] || 'warning';
  }
  
  // Method for getting category statistics
  getCategoryStats(violations) {
    const stats = {};
    
    violations.forEach(violation => {
      const category = violation.category;
      if (!stats[category]) {
        stats[category] = {
          count: 0,
          severity: violation.severity,
          files: new Set()
        };
      }
      stats[category].count++;
      stats[category].files.add(violation.file);
    });
    
    // Convert Set to array for JSON serialization
    Object.keys(stats).forEach(category => {
      stats[category].files = Array.from(stats[category].files);
      stats[category].fileCount = stats[category].files.length;
    });
    
    return stats;
  }
  
  // Method for filtering by category
  filterByCategory(violations, categoryNames) {
    if (!categoryNames || categoryNames.length === 0) {
      return violations;
    }
    
    return violations.filter(violation => 
      categoryNames.includes(violation.category)
    );
  }
  
  // Method for filtering by severity
  filterBySeverity(violations, minSeverity = 'info') {
    const severityOrder = ['info', 'warning', 'error'];
    const minIndex = severityOrder.indexOf(minSeverity);
    
    if (minIndex === -1) return violations;
    
    return violations.filter(violation => {
      const violationIndex = severityOrder.indexOf(violation.severity);
      return violationIndex >= minIndex;
    });
  }
}

module.exports = S027CategorizedAnalyzer;
