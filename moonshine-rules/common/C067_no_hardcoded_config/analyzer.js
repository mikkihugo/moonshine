// rules/common/C067_no_hardcoded_config/analyzer.js
const C067SymbolBasedAnalyzer = require('./symbol-based-analyzer.js');

class C067Analyzer {
  constructor(semanticEngine = null) {
    this.ruleId = 'C067';
    this.ruleName = 'No Hardcoded Configuration';
    this.description = 'Improve configurability, reduce risk when changing environments, and make configuration management flexible and maintainable.';
    this.semanticEngine = semanticEngine;
    this.verbose = false;
    
    // Use symbol-based only for accuracy
    this.symbolAnalyzer = new C067SymbolBasedAnalyzer(semanticEngine);
  }

  async initialize(semanticEngine = null) {
    if (semanticEngine) {
      this.semanticEngine = semanticEngine;
    }
    this.verbose = semanticEngine?.verbose || false;
    await this.symbolAnalyzer.initialize(semanticEngine);
  }

  // Main analyze method required by heuristic engine
  async analyze(files, language, options = {}) {
    const violations = [];
    
    for (const filePath of files) {
      if (options.verbose) {
        console.log(`[DEBUG] 🎯 C067: Analyzing ${filePath.split('/').pop()}`);
      }
      
      try {
        const fileViolations = await this.analyzeFileBasic(filePath, options);
        violations.push(...fileViolations);
      } catch (error) {
        console.warn(`C067: Skipping ${filePath}: ${error.message}`);
      }
    }
    
    return violations;
  }

  async analyzeFileBasic(filePath, options = {}) {
    try {
      // Try semantic engine first, fallback to standalone ts-morph
      if (this.semanticEngine?.isSymbolEngineReady?.() && this.semanticEngine.project) {
        if (this.verbose) {
          console.log(`[DEBUG] 🎯 C067: Using semantic engine for ${filePath.split('/').pop()}`);
        }
        
        const violations = await this.symbolAnalyzer.analyzeFileBasic(filePath, options);
        
        if (this.verbose) {
          console.log(`[DEBUG] 🎯 C067: Symbol-based analysis found ${violations.length} violations`);
        }

        return violations;
      } else {
        // Fallback to standalone analysis
        if (this.verbose) {
          console.log(`[DEBUG] 🎯 C067: Using standalone analysis for ${filePath.split('/').pop()}`);
        }
        
        const violations = await this.symbolAnalyzer.analyzeFileStandalone(filePath, options);
        
        if (this.verbose) {
          console.log(`[DEBUG] 🎯 C067: Standalone analysis found ${violations.length} violations`);
        }

        return violations;
      }
    } catch (error) {
      if (this.verbose) {
        console.error(`[DEBUG] ❌ C067: Analysis failed: ${error.message}`);
      }
      throw new Error(`C067 analysis failed: ${error.message}`);
    }
  }

  async analyzeFiles(files, options = {}) {
    const allViolations = [];
    for (const filePath of files) {
      try {
        const violations = await this.analyzeFileBasic(filePath, options);
        allViolations.push(...violations);
      } catch (error) {
        console.warn(`C067: Skipping ${filePath}: ${error.message}`);
      }
    }
    return allViolations;
  }
}

module.exports = C067Analyzer;
