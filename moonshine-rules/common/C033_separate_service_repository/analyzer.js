/**
 * C033 Main Analyzer - Symbol-based with minimal regex fallback
 * Primary: Symbol-based analysis (95% cases)
 * Fallback: Regex-based only when symbol analysis completely fails
 */

const C033SymbolBasedAnalyzer = require('./symbol-based-analyzer');
const C033RegexBasedAnalyzer = require('./regex-based-analyzer');

class C033Analyzer {
  constructor(semanticEngine = null) {
    this.ruleId = 'C033';
    this.ruleName = 'Separate Service and Repository Logic';
    this.description = 'Tách logic xử lý và truy vấn dữ liệu trong service layer - Repository chỉ chứa CRUD, Service chứa business logic';
    this.semanticEngine = semanticEngine;
    this.verbose = false;
    
    // Initialize analyzers
    this.symbolBasedAnalyzer = new C033SymbolBasedAnalyzer(semanticEngine);
    this.regexBasedAnalyzer = new C033RegexBasedAnalyzer(semanticEngine);
    
    // Configuration
    this.config = {
      useSymbolBased: true,      // Primary approach
      fallbackToRegex: true,     // Only when symbol fails completely
      symbolBasedOnly: false     // Can be set to true for pure mode
    };
  }

  /**
   * Initialize with semantic engine
   */
  async initialize(semanticEngine = null) {
    if (semanticEngine) {
      this.semanticEngine = semanticEngine;
    }
    this.verbose = semanticEngine?.verbose || false;
    
    // Initialize both analyzers
    await this.symbolBasedAnalyzer.initialize(semanticEngine);
    await this.regexBasedAnalyzer.initialize(semanticEngine);
    
    if (this.verbose) {
      console.log(`[DEBUG] 🔧 C033: Analyzer initialized - Symbol-based: ✅, Regex fallback: ${this.config.fallbackToRegex ? '✅' : '❌'}`);
    }
  }

  async analyze(files, language, options = {}) {
    const violations = [];
    let symbolCount = 0;
    let regexCount = 0;
    
    for (const filePath of files) {
      try {
        const fileViolations = await this.analyzeFile(filePath, options);
        violations.push(...fileViolations);
        
        // Count strategy usage
        const strategy = fileViolations[0]?.analysisStrategy;
        if (strategy === 'symbol-based') symbolCount++;
        else if (strategy === 'regex-fallback') regexCount++;
        
      } catch (error) {
        if (this.verbose) {
          console.warn(`[C033] Analysis failed for ${filePath}:`, error.message);
        }
      }
    }
    
    // Summary of strategy usage
    if (this.verbose && (symbolCount > 0 || regexCount > 0)) {
      console.log(`📊 [C033-SUMMARY] Analysis strategy usage:`);
      console.log(`   🧠 Symbol-based: ${symbolCount} files`);
      console.log(`   🔄 Regex-fallback: ${regexCount} files`);
      console.log(`   📈 Coverage: ${symbolCount}/${symbolCount + regexCount} files used primary strategy`);
    }
    
    return violations;
  }

  async analyzeFile(filePath, options = {}) {
    // 1. Try Symbol-based analysis first (primary)
    if (this.config.useSymbolBased && this.semanticEngine?.project) {
      try {
        const sourceFile = this.semanticEngine.project.getSourceFileByFilePath(filePath);
        if (sourceFile) {
          const violations = await this.symbolBasedAnalyzer.analyzeFileWithSymbols(filePath, options);
          
          if (this.verbose) {
            console.log(`🧠 [C033-SYMBOL] ${filePath}: Found ${violations.length} violations`);
          }
          
          return violations.map(v => ({ ...v, analysisStrategy: 'symbol-based' }));
        } else {
          if (this.verbose) {
            console.log(`⚠️  [C033-SYMBOL] ${filePath}: Source file not found in ts-morph project, falling back to regex`);
          }
        }
      } catch (error) {
        if (this.verbose) {
          console.warn(`❌ [C033-SYMBOL] ${filePath}: Symbol analysis failed, falling back to regex:`, error.message);
        }
      }
    } else {
      if (this.verbose) {
        const reason = !this.config.useSymbolBased ? 'Symbol-based disabled' : 'No semantic engine';
        console.log(`⚠️  [C033] ${filePath}: Skipping symbol analysis (${reason}), using regex`);
      }
    }

    // 2. Fallback to Regex-based analysis (only if symbol fails or unavailable)
    if (this.config.fallbackToRegex && !this.config.symbolBasedOnly) {
      try {
        const violations = await this.regexBasedAnalyzer.analyzeFileBasic(filePath, options);
        
        if (this.verbose) {
          console.log(`🔄 [C033-REGEX] ${filePath}: Found ${violations.length} violations`);
        }
        
        return violations.map(v => ({ ...v, analysisStrategy: 'regex-fallback' }));
      } catch (error) {
        if (this.verbose) {
          console.warn(`❌ [C033-REGEX] ${filePath}: Regex fallback also failed:`, error.message);
        }
      }
    }
    
    return [];
  }

  // Legacy compatibility methods
  async analyzeWithSemantics(filePath, options = {}) {
    return await this.analyzeFile(filePath, options);
  }

  async analyzeFileBasic(filePath, options = {}) {
    // Force regex-based for legacy compatibility
    const violations = await this.regexBasedAnalyzer.analyzeFileBasic(filePath, options);
    return violations.map(v => ({ ...v, analysisStrategy: 'regex-legacy' }));
  }

  // Configuration methods
  enableSymbolBasedOnly() {
    this.config.symbolBasedOnly = true;
    this.config.fallbackToRegex = false;
    if (this.verbose) {
      console.log(`[C033] Switched to symbol-based only mode`);
    }
  }

  enableHybridMode() {
    this.config.symbolBasedOnly = false;
    this.config.fallbackToRegex = true;
    if (this.verbose) {
      console.log(`[C033] Switched to hybrid mode (symbol-based + regex fallback)`);
    }
  }
}

module.exports = C033Analyzer;
