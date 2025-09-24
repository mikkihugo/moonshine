const C019SystemLogAnalyzer = require('./system-log-analyzer.js');
const C019PatternAnalyzer = require('./pattern-analyzer.js');

class C019Analyzer {
  constructor(semanticEngine = null) {
    this.ruleId = 'C019';
    this.ruleName = 'Log Level Usage';
    this.description = 'Comprehensive logging analysis: levels, patterns, performance, and system requirements';
    this.semanticEngine = semanticEngine;
    this.verbose = false;
    
    // Initialize analyzers - consolidated architecture
    this.systemAnalyzer = new C019SystemLogAnalyzer(semanticEngine);
    this.patternAnalyzer = new C019PatternAnalyzer();
    this.aiAnalyzer = null;
  }

  async initialize(semanticEngine = null) {
    if (semanticEngine) {
      this.semanticEngine = semanticEngine;
    }
    this.verbose = semanticEngine?.verbose || false;
    
    await this.systemAnalyzer.initialize(semanticEngine);
    await this.patternAnalyzer.initialize({ verbose: this.verbose });
  }

  async analyzeFileBasic(filePath, options = {}) {
    const allViolations = [];
    
    try {
      // Run comprehensive system-level analysis (Primary - AST)
      if (this.semanticEngine?.isSymbolEngineReady?.() && this.semanticEngine.project) {
        if (this.verbose) {
          console.log(`[DEBUG] 🎯 C019: Using comprehensive system-level analysis for ${filePath.split('/').pop()}`);
        }
        
        try {
          const systemViolations = await this.systemAnalyzer.analyzeFileBasic(filePath, options);
          allViolations.push(...systemViolations);
          
          if (this.verbose) {
            console.log(`[DEBUG] 🎯 C019: System analysis found ${systemViolations.length} violations`);
          }
        } catch (systemError) {
          if (this.verbose) {
            console.warn(`[DEBUG] ⚠️ C019: System analysis failed: ${systemError.message}`);
          }
        }
        
        if (allViolations.length > 0) {
          return this.deduplicateViolations(allViolations);
        }
      }
      
      // Fall back to pattern-based analysis (Secondary - Regex)
      if (this.verbose) {
        console.log(`[DEBUG] 🔄 C019: Running pattern-based analysis for ${filePath.split('/').pop()}`);
      }
      
      const patternViolations = await this.patternAnalyzer.analyzeFileBasic(filePath, options);
      allViolations.push(...patternViolations);
      
      if (this.verbose) {
        console.log(`[DEBUG] 🔄 C019: Pattern analysis found ${patternViolations.length} violations`);
      }
      
      return this.deduplicateViolations(allViolations);
      
    } catch (error) {
      if (this.verbose) {
        console.error(`[DEBUG] ❌ C019: Analysis failed: ${error.message}`);
      }
      throw new Error(`C019 analysis failed: ${error.message}`);
    }
  }

  deduplicateViolations(violations) {
    // Remove duplicates based on file, line, and type
    const seen = new Set();
    return violations.filter(violation => {
      const key = `${violation.filePath}:${violation.line}:${violation.type}`;
      if (seen.has(key)) return false;
      seen.add(key);
      return true;
    });
  }

  async analyzeFiles(files, options = {}) {
    const allViolations = [];
    for (const filePath of files) {
      try {
        const violations = await this.analyzeFileBasic(filePath, options);
        allViolations.push(...violations);
      } catch (error) {
        console.warn(`C019: Skipping ${filePath}: ${error.message}`);
      }
    }
    return allViolations;
  }

  // Legacy method for backward compatibility
  async analyze(files, language, config = {}) {
    // Initialize AI analyzer if enabled
    if (config.ai && config.ai.enabled) {
      this.aiAnalyzer = new AIAnalyzer(config.ai);
      console.log('🤖 AI analysis enabled for C019');
    }

    const allViolations = [];

    for (const filePath of files) {
      try {
        const fileContent = require('fs').readFileSync(filePath, 'utf8');
        const fileViolations = await this.analyzeFile(filePath, fileContent, language, config);
        allViolations.push(...fileViolations);
      } catch (error) {
        console.error(`Error analyzing file ${filePath}:`, error.message);
      }
    }

    return allViolations;
  }

  async analyzeFile(filePath, content, language, config) {
    let violations = [];
    
    // Try AI analysis first if enabled
    if (this.aiAnalyzer) {
      try {
        console.log(`🤖 Running AI analysis on ${require('path').basename(filePath)}`);
        const aiViolations = await this.aiAnalyzer.analyzeWithAI(filePath, content, {
          name: this.ruleName,
          description: this.description,
          ruleId: this.ruleId
        });
        
        if (aiViolations && aiViolations.length > 0) {
          violations.push(...aiViolations);
          return violations;
        }
      } catch (error) {
        console.warn(`AI analysis failed for ${filePath}, falling back to heuristic analysis`);
      }
    }
    
    // Use the new analyzer architecture
    const heuristicViolations = await this.analyzeFileBasic(filePath, config);
    violations.push(...heuristicViolations);
    
    return violations;
  }
}

module.exports = C019Analyzer;
