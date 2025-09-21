/**
 * C029 Smart Pipeline Analyzer
 * 
 * Intelligent 3-stage analysis pipeline:
 * 1. REGEX: Fast catch block detection
 * 2. AST: Structure analysis and nesting evaluation  
 * 3. DATA FLOW: Exception usage validation
 * 
 * Goal: >= ESLint accuracy with superior performance
 */

const fs = require('fs');

class C029SmartPipelineAnalyzer {
  constructor() {
    this.ruleId = 'C029';
    this.ruleName = 'Smart Catch Block Analysis';
    this.description = 'Intelligent 3-stage pipeline for catch block validation';
    
    // Performance tracking
    this.stats = {
      totalFiles: 0,
      regexCandidates: 0,
      astAnalyzed: 0,
      dataFlowChecked: 0,
      finalViolations: 0,
      executionTime: 0
    };
  }

  async analyze(files, language, options = {}) {
    if (options.verbose) {
      console.log(`🎯 C029 Smart Pipeline loaded (Regex → AST → Data Flow)`);
      console.log(`🎯 C029 Smart Pipeline: Analyzing ${files.length} files with 3-stage approach...`);
    }
    
    const startTime = Date.now();
    const violations = [];
    this.stats.totalFiles = files.length;

    for (const filePath of files) {
      try {
        const content = fs.readFileSync(filePath, 'utf8');
        const fileViolations = await this.analyzeFileWithPipeline(content, filePath, language);
        violations.push(...fileViolations);
      } catch (error) {
        console.warn(`C029 Smart Pipeline skipping ${filePath}: ${error.message}`);
      }
    }

    this.stats.finalViolations = violations.length;
    this.stats.executionTime = Date.now() - startTime;
    
    if (options.verbose) {
      this.printAnalysisStats();
    }
    return violations;
  }

  /**
   * 3-STAGE SMART PIPELINE
   */
  async analyzeFileWithPipeline(content, filePath, language) {
    const violations = [];

    // STAGE 1: REGEX - Fast catch detection
    const catchCandidates = this.stage1_RegexDetection(content, filePath);
    this.stats.regexCandidates += catchCandidates.length;

    if (catchCandidates.length === 0) {
      return violations; // No catch blocks found, skip expensive analysis
    }

    // STAGE 2: AST - Structure analysis for each candidate
    for (const candidate of catchCandidates) {
      const astResult = this.stage2_ASTAnalysis(candidate, content, filePath);
      this.stats.astAnalyzed++;

      if (astResult.needsDataFlow) {
        // STAGE 3: DATA FLOW - Deep exception usage analysis
        const dataFlowResult = this.stage3_DataFlowAnalysis(candidate, astResult, content, filePath);
        this.stats.dataFlowChecked++;
        
        if (dataFlowResult.isViolation) {
          violations.push(dataFlowResult.violation);
        }
      } else if (astResult.isViolation) {
        // AST already determined it's a violation
        violations.push(astResult.violation);
      }
    }

    return violations;
  }

  /**
   * STAGE 1: REGEX DETECTION
   * Fast identification of catch blocks
   */
  stage1_RegexDetection(content, filePath) {
    const candidates = [];
    const lines = content.split('\n');

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      
      // Detect catch blocks with different patterns
      const catchMatches = [
        /catch\s*\(\s*(\w+)\s*\)\s*\{/.exec(line),           // catch(e) {
        /catch\s*\(\s*\{\s*(\w+)\s*\}\s*\)\s*\{/.exec(line), // catch({error}) {
        /\.catch\s*\(\s*(\w+)\s*=>\s*\{/.exec(line),         // .catch(e => {
        /\.catch\s*\(\s*function\s*\(\s*(\w+)\s*\)/.exec(line) // .catch(function(e)
      ];

      for (const match of catchMatches) {
        if (match) {
          const catchBlock = this.extractCatchBlock(lines, i, match[0]);
          if (catchBlock) {
            candidates.push({
              type: 'try-catch',
              startLine: i + 1,
              endLine: catchBlock.endLine,
              errorVariable: match[1] || 'e',
              content: catchBlock.content,
              rawMatch: match[0],
              context: this.getContext(lines, i)
            });
          }
          break;
        }
      }
    }

    return candidates;
  }

  /**
   * STAGE 2: AST ANALYSIS
   * Structure evaluation and nesting analysis
   */
  stage2_ASTAnalysis(candidate, content, filePath) {
    const astAnalysis = {
      needsDataFlow: false,
      isViolation: false,
      violation: null,
      structureInfo: {}
    };

    // 2.1: Basic emptiness check
    if (this.isEmptyCatchBlock(candidate.content)) {
      astAnalysis.isViolation = true;
      astAnalysis.violation = this.createViolation(
        candidate, filePath, 'empty_catch', 
        'Empty catch block', 
        'Add error handling or explicit documentation', 
        0.9
      );
      return astAnalysis;
    }

    // 2.2: Simple logging check (fast path)
    if (this.hasObviousLogging(candidate.content)) {
      // Has obvious logging, no violation
      return astAnalysis;
    }

    // 2.3: Nesting and structure analysis
    const structureInfo = this.analyzeStructure(candidate.content);
    astAnalysis.structureInfo = structureInfo;

    // 2.4: Context-aware decisions
    if (this.isTestFile(filePath) && structureInfo.hasTestAssertions) {
      // Test file with assertions, likely ok
      return astAnalysis;
    }

    if (structureInfo.hasControlFlow && !structureInfo.hasLogging) {
      // Has control flow but no logging, might be intentional
      astAnalysis.needsDataFlow = true;
      return astAnalysis;
    }

    if (structureInfo.complexity > 3 && !structureInfo.hasLogging) {
      // Complex catch block without logging, likely violation
      astAnalysis.isViolation = true;
      astAnalysis.violation = this.createViolation(
        candidate, filePath, 'complex_no_logging',
        'Complex catch block without error logging',
        'Add error logging for debugging and monitoring',
        0.8
      );
      return astAnalysis;
    }

    // Need deeper analysis
    astAnalysis.needsDataFlow = true;
    return astAnalysis;
  }

  /**
   * STAGE 3: DATA FLOW ANALYSIS
   * Exception usage validation
   */
  stage3_DataFlowAnalysis(candidate, astResult, content, filePath) {
    const dataFlowResult = {
      isViolation: false,
      violation: null,
      usageInfo: {}
    };

    // 3.1: Track exception variable usage
    const usageInfo = this.analyzeExceptionUsage(candidate, content);
    dataFlowResult.usageInfo = usageInfo;

    // 3.2: Decision logic based on usage patterns
    if (usageInfo.isUnused && !usageInfo.hasExplicitIgnore) {
      dataFlowResult.isViolation = true;
      dataFlowResult.violation = this.createViolation(
        candidate, filePath, 'unused_exception',
        `Exception variable '${candidate.errorVariable}' is unused`,
        `Use the exception for logging or add explicit ignore comment`,
        0.85
      );
      return dataFlowResult;
    }

    if (usageInfo.isUsedButNotLogged && !this.isTestFile(filePath)) {
      dataFlowResult.isViolation = true;
      dataFlowResult.violation = this.createViolation(
        candidate, filePath, 'used_not_logged',
        `Exception is used but not logged for debugging`,
        `Add console.error() or logging framework call`,
        0.7
      );
      return dataFlowResult;
    }

    if (usageInfo.isSilentlyReturned && astResult.structureInfo.complexity > 1) {
      dataFlowResult.isViolation = true;
      dataFlowResult.violation = this.createViolation(
        candidate, filePath, 'silent_return',
        `Exception silently handled without logging`,
        `Add error logging before returning fallback value`,
        0.75
      );
      return dataFlowResult;
    }

    return dataFlowResult;
  }

  /**
   * HELPER METHODS
   */

  extractCatchBlock(lines, startIndex, matchString) {
    let braceCount = 0;
    let content = '';
    let inBlock = false;
    let catchBraceCount = 0; // Track braces specifically for the catch block

    for (let i = startIndex; i < lines.length; i++) {
      const line = lines[i];
      
      if (line.includes(matchString)) {
        inBlock = true;
        content += line + '\n';
        
        // Count the opening brace of the catch block
        for (const char of line) {
          if (char === '{') {
            catchBraceCount++;
            break; // Only count the first brace on the catch line
          }
        }
        continue;
      }
      
      if (inBlock) {
        content += line + '\n';
        
        // Count braces to find the end of the catch block
        for (const char of line) {
          if (char === '{') {
            catchBraceCount++;
          } else if (char === '}') {
            catchBraceCount--;
            
            // If we've closed the catch block
            if (catchBraceCount === 0) {
              return {
                endLine: i + 1,
                content: content.trim()
              };
            }
          }
        }
      }
    }
    
    // If we reach here, return what we have
    return content ? {
      endLine: lines.length,
      content: content.trim()
    } : null;
  }

  isEmptyCatchBlock(content) {
    // Don't just remove all comments - check if there's actual code first
    if (this.hasObviousLogging(content)) {
      return false; // Has logging, definitely not empty
    }
    
    const cleaned = content
      .replace(/\/\*[\s\S]*?\*\//g, '') // Remove block comments
      .replace(/\/\/.*$/gm, '')         // Remove line comments  
      .replace(/catch\s*\([^)]*\)\s*\{/, '') // Remove catch declaration
      .replace(/\}/g, '')               // Remove closing brace
      .replace(/\s+/g, '')              // Remove whitespace
      .trim();
    
    return cleaned.length === 0;
  }

  hasObviousLogging(content) {
    const loggingPatterns = [
      /console\.(log|error|warn|info|debug)/,
      /logger?\.(error|warn|info|debug)/,
      /log\.(error|warn|info|debug)/,
      /this\.logger?\.(error|warn|info|debug)/,  // Added: this.logger.method()
      /\w+\.logger?\.(error|warn|info|debug)/,   // Added: obj.logger.method()
      /Logger\.(error|warn|info|debug|log)/,     // Added: Logger.method()
      /this\.logErrors?\s*\(/,                   // Added: this.logError() and this.logErrors()
      /this\.couponLogErrors\s*\(/,              // Added: this.couponLogErrors()
      /print\s*\(/,
      /throw\s+/,
      /rethrow/
    ];
    
    return loggingPatterns.some(pattern => pattern.test(content));
  }

  analyzeStructure(content) {
    const structure = {
      complexity: 0,
      hasLogging: false,
      hasControlFlow: false,
      hasTestAssertions: false,
      nestingLevel: 0
    };

    // Count complexity indicators
    structure.complexity += (content.match(/\bif\b/g) || []).length;
    structure.complexity += (content.match(/\bfor\b/g) || []).length;
    structure.complexity += (content.match(/\bwhile\b/g) || []).length;
    structure.complexity += (content.match(/\btry\b/g) || []).length;

    // Check for control flow
    structure.hasControlFlow = /\b(return|throw|break|continue)\b/.test(content);

    // Check for test assertions
    structure.hasTestAssertions = /\b(expect|assert|should|toBe|toEqual)\b/.test(content);

    // Calculate nesting level
    const lines = content.split('\n');
    let maxNesting = 0;
    let currentNesting = 0;
    
    for (const line of lines) {
      currentNesting += (line.match(/\{/g) || []).length;
      currentNesting -= (line.match(/\}/g) || []).length;
      maxNesting = Math.max(maxNesting, currentNesting);
    }
    structure.nestingLevel = maxNesting;

    // Re-check logging (more thorough)
    structure.hasLogging = this.hasObviousLogging(content);

    return structure;
  }

  analyzeExceptionUsage(candidate, content) {
    const errorVar = candidate.errorVariable;
    const catchContent = candidate.content;
    
    const usage = {
      isUnused: false,
      isUsedButNotLogged: false,
      isSilentlyReturned: false,
      hasExplicitIgnore: false,
      usageCount: 0,
      usageTypes: [],
      dataFlowAnalysis: null
    };

    // Count usages of error variable
    const usageRegex = new RegExp(`\\b${errorVar}\\b`, 'g');
    const matches = catchContent.match(usageRegex) || [];
    
    // More robust counting - exclude the catch declaration itself
    const catchDeclarationRegex = new RegExp(`catch\\s*\\(\\s*${errorVar}\\s*\\)`, 'g');
    const declarationMatches = catchContent.match(catchDeclarationRegex) || [];
    
    usage.usageCount = matches.length - declarationMatches.length;

    // Check for explicit ignore patterns
    usage.hasExplicitIgnore = /\/\/\s*(ignore|TODO|FIXME|eslint-disable)/.test(catchContent);

    if (usage.usageCount === 0) {
      usage.isUnused = true;
      return usage;
    }

    // ENHANCED DATA FLOW ANALYSIS - Track where exception flows
    usage.dataFlowAnalysis = this.traceExceptionDataFlow(errorVar, catchContent, content);

    // Direct logging patterns (immediate logging)
    if (new RegExp(`console\\.\\w+\\s*\\([^)]*\\b${errorVar}\\b`).test(catchContent)) {
      usage.usageTypes.push('direct_logging');
    }
    
    // Logger framework patterns (immediate logging) 
    if (new RegExp(`logger?\\.\\w+\\s*\\([^)]*\\b${errorVar}\\b`).test(catchContent)) {
      usage.usageTypes.push('direct_logging');
    }
    
    // Extract all function calls that use the error variable
    const functionCalls = this.extractFunctionCallsWithError(errorVar, catchContent);
    
    for (const call of functionCalls) {
      // DATA FLOW: Check if this function eventually leads to logging
      const hasEventualLogging = this.doesFunctionEventuallyLog(call.functionName, call.fullCall, content);
      
      if (hasEventualLogging) {
        usage.usageTypes.push('delegated_logging');
      } else {
        usage.usageTypes.push('function_call_no_logging');
      }
    }
    
    if (new RegExp(`throw\\s+\\b${errorVar}\\b`).test(catchContent)) {
      usage.usageTypes.push('rethrowing');
    }
    
    if (new RegExp(`return\\s+[^;]*\\b${errorVar}\\b`).test(catchContent)) {
      usage.usageTypes.push('returning');
    }

    // ENHANCED LOGIC: Based on data flow analysis
    const hasActualLogging = usage.usageTypes.some(type => 
      ['direct_logging', 'delegated_logging'].includes(type)
    );
    
    usage.isUsedButNotLogged = usage.usageCount > 0 && 
                               !hasActualLogging && 
                               !usage.usageTypes.includes('rethrowing');

    usage.isSilentlyReturned = /return\s+(null|undefined|false|\[\]|\{\}|'')/.test(catchContent) &&
                               !hasActualLogging;

    return usage;
  }

  /**
   * ENHANCED DATA FLOW ANALYSIS
   * Trace where exception flows to determine if it eventually gets logged
   */
  traceExceptionDataFlow(errorVar, catchContent, fullFileContent) {
    const flow = {
      directLogging: false,
      functionCalls: [],
      eventualLogging: false,
      traceDepth: 0
    };

    // Check direct logging in catch block
    flow.directLogging = this.hasDirectLogging(errorVar, catchContent);
    
    // Extract function calls and trace them
    flow.functionCalls = this.extractFunctionCallsWithError(errorVar, catchContent);
    
    // For each function call, try to trace if it leads to logging
    for (const call of flow.functionCalls) {
      const hasLogging = this.doesFunctionEventuallyLog(call.functionName, call.fullCall, fullFileContent);
      if (hasLogging) {
        flow.eventualLogging = true;
        break;
      }
    }

    return flow;
  }

  /**
   * Check if exception is directly logged in catch block
   */
  hasDirectLogging(errorVar, catchContent) {
    const directLoggingPatterns = [
      new RegExp(`console\\.(log|error|warn|info|debug)\\s*\\([^)]*\\b${errorVar}\\b`, 'i'),
      new RegExp(`logger?\\.\\w+\\s*\\([^)]*\\b${errorVar}\\b`, 'i'),
      new RegExp(`log\\.\\w+\\s*\\([^)]*\\b${errorVar}\\b`, 'i'),
      new RegExp(`\\.(error|warn|info|debug|log)\\s*\\([^)]*\\b${errorVar}\\b`, 'i')
    ];

    return directLoggingPatterns.some(pattern => pattern.test(catchContent));
  }

  /**
   * Extract all function calls that include the error variable
   */
  extractFunctionCallsWithError(errorVar, catchContent) {
    const calls = [];
    
    // Match various function call patterns
    const patterns = [
      new RegExp(`(\\w+)\\s*\\([^)]*\\b${errorVar}\\b[^)]*\\)`, 'g'),           // func(error)
      new RegExp(`(\\w+\\.\\w+)\\s*\\([^)]*\\b${errorVar}\\b[^)]*\\)`, 'g'),    // this.method(error)
      new RegExp(`(\\w+\\.\\w+\\.\\w+)\\s*\\([^)]*\\b${errorVar}\\b[^)]*\\)`, 'g') // obj.service.method(error)
    ];

    for (const pattern of patterns) {
      let match;
      while ((match = pattern.exec(catchContent)) !== null) {
        calls.push({
          functionName: match[1],
          fullCall: match[0],
          position: match.index
        });
      }
    }

    return calls;
  }

  /**
   * CORE DATA FLOW: Check if a function eventually leads to logging
   * Enhanced with limited multi-level tracing
   */
  doesFunctionEventuallyLog(functionName, functionCall, fullFileContent, depth = 0) {
    try {
      // Prevent infinite recursion and excessive tracing
      if (depth > 2) {
        return false;
      }

      // 1. Try to find the function definition in the current file
      const functionDef = this.findFunctionDefinition(functionName, fullFileContent);
      
      if (functionDef) {
        // 2. Check if the function body contains logging
        const hasDirectLogging = this.hasFunctionLogging(functionDef.body);
        if (hasDirectLogging) {
          return true;
        }

        // 3. MULTI-LEVEL TRACING: Check if this function calls other functions
        if (depth < 2) {
          const nestedCalls = this.extractFunctionCallsFromBody(functionDef.body);
          for (const nestedCall of nestedCalls) {
            const hasNestedLogging = this.doesFunctionEventuallyLog(
              nestedCall.functionName, 
              nestedCall.fullCall, 
              fullFileContent, 
              depth + 1
            );
            if (hasNestedLogging) {
              return true;
            }
          }
        }

        return false;
      }

      // 4. If not found locally, check for common logging patterns
      if (this.isKnownLoggingPattern(functionCall)) {
        return true;
      }

      // 5. Default: assume no logging if we can't trace
      return false;
      
    } catch (error) {
      // If analysis fails, be conservative
      return false;
    }
  }

  /**
   * Extract function calls from function body for multi-level tracing
   */
  extractFunctionCallsFromBody(functionBody) {
    const calls = [];
    
    // Match function calls in the body
    const patterns = [
      /(\w+)\s*\([^)]*\)/g,                    // functionName()
      /(\w+\.\w+)\s*\([^)]*\)/g,               // this.method()
      /(\w+\.\w+\.\w+)\s*\([^)]*\)/g,          // obj.service.method()
      /return\s+(\w+)\s*\([^)]*\)/g,           // return functionName()
      /return\s+(\w+\.\w+)\s*\([^)]*\)/g       // return this.method()
    ];

    for (const pattern of patterns) {
      let match;
      while ((match = pattern.exec(functionBody)) !== null) {
        calls.push({
          functionName: match[1],
          fullCall: match[0],
          position: match.index
        });
      }
    }

    return calls;
  }

  /**
   * Find function definition in file content
   */
  findFunctionDefinition(functionName, content) {
    // Handle method calls like this.methodName
    const methodName = functionName.includes('.') ? 
      functionName.split('.').pop() : functionName;

    // Patterns to match function definitions
    const patterns = [
      new RegExp(`${methodName}\\s*\\([^)]*\\)\\s*\\{([^{}]*(?:\\{[^{}]*\\}[^{}]*)*)\\}`, 's'),  // method() { ... }
      new RegExp(`function\\s+${methodName}\\s*\\([^)]*\\)\\s*\\{([^{}]*(?:\\{[^{}]*\\}[^{}]*)*)\\}`, 's'), // function name() { ... }
      new RegExp(`${methodName}\\s*:\\s*function\\s*\\([^)]*\\)\\s*\\{([^{}]*(?:\\{[^{}]*\\}[^{}]*)*)\\}`, 's'), // name: function() { ... }
      new RegExp(`${methodName}\\s*=\\s*\\([^)]*\\)\\s*=>\\s*\\{([^{}]*(?:\\{[^{}]*\\}[^{}]*)*)\\}`, 's') // name = () => { ... }
    ];

    for (const pattern of patterns) {
      const match = pattern.exec(content);
      if (match) {
        return {
          name: methodName,
          body: match[1],
          fullMatch: match[0]
        };
      }
    }

    return null;
  }

  /**
   * Check if function body contains logging OR valid error handling
   */
  hasFunctionLogging(functionBody) {
    const loggingPatterns = [
      /console\.(log|error|warn|info|debug)/i,
      /logger?\.(error|warn|info|debug|log)/i,
      /log\.(error|warn|info|debug)/i,
      /\.error\s*\(/i,
      /\.warn\s*\(/i,
      /\.info\s*\(/i,
      /\.debug\s*\(/i,
      /print\s*\(/i
    ];

    // ENHANCED: Also consider error propagation as valid handling
    const errorHandlingPatterns = [
      /throw\s+/i,                    // throw error/new Error
      /rethrow/i,                     // explicit rethrow
      /return.*Error/i,               // return error object
      /\.handle\s*\(/i,               // error.handle()
      /ErrorHandler\./i,              // ErrorHandler.method()
      /externalErrorHandler\s*\(/i,   // specific handler functions
      /errorHandler\s*\(/i,           // generic error handlers
      /handleError\s*\(/i             // handle error functions
    ];

    const hasLogging = loggingPatterns.some(pattern => pattern.test(functionBody));
    const hasErrorHandling = errorHandlingPatterns.some(pattern => pattern.test(functionBody));
    
    return hasLogging || hasErrorHandling;
  }

  /**
   * Check if function call matches known logging/error handling patterns
   */
  isKnownLoggingPattern(functionCall) {
    const knownPatterns = [
      // Direct logging
      /\.logger?\./i,        // this.logger.anything, obj.log.anything
      /console\./i,          // console.anything
      /Logger\./i,           // Logger.anything (static)
      /\.log\./i,            // obj.log.anything
      /\.error\s*\(/i,       // anything.error()
      /\.warn\s*\(/i,        // anything.warn()
      /\.info\s*\(/i,        // anything.info()
      /\.debug\s*\(/i,       // anything.debug()
      
      // ENHANCED: Error handling patterns
      /externalErrorHandler\s*\(/i,  // externalErrorHandler()
      /errorHandler\s*\(/i,          // anyErrorHandler()
      /handleError\s*\(/i,           // handleError()
      /ErrorHandler\./i,             // ErrorHandler.method()
      /\.handle\s*\(/i,              // obj.handle()
      /processError\s*\(/i,          // processError() - common pattern
      /logError\s*\(/i,              // logError() - common pattern
      /logErrors\s*\(/i,             // logErrors() - common pattern for base classes
      /reportError\s*\(/i,           // reportError()
      /sendError\s*\(/i,             // sendError()
      /trackError\s*\(/i,            // trackError()
      /couponLogErrors\s*\(/i        // couponLogErrors() - specific pattern
    ];

    return knownPatterns.some(pattern => pattern.test(functionCall));
  }

  getContext(lines, lineIndex) {
    const before = lines.slice(Math.max(0, lineIndex - 3), lineIndex).join('\n');
    const after = lines.slice(lineIndex + 1, Math.min(lines.length, lineIndex + 4)).join('\n');
    
    return { before, after };
  }

  isTestFile(filePath) {
    const testPatterns = ['__tests__', '.test.', '.spec.', '/test/', '/tests/', '.stories.'];
    return testPatterns.some(pattern => filePath.includes(pattern));
  }

  createViolation(candidate, filePath, type, message, suggestion, confidence) {
    return {
      ruleId: this.ruleId,
      file: filePath,
      line: candidate.startLine,
      column: 1,
      message: message,
      severity: 'error',
      code: candidate.content.split('\n')[0].trim(),
      type: type,
      confidence: confidence,
      suggestion: suggestion,
      errorVariable: candidate.errorVariable,
      pipeline: 'smart_3_stage'
    };
  }

  printAnalysisStats() {
    console.log(`📊 C029 Smart Pipeline Stats:`);
    console.log(`  📁 Files analyzed: ${this.stats.totalFiles}`);
    console.log(`  🔍 Regex candidates: ${this.stats.regexCandidates}`);
    console.log(`  🌳 AST analyzed: ${this.stats.astAnalyzed}`);
    console.log(`  🧠 Data flow checked: ${this.stats.dataFlowChecked}`);
    console.log(`  ❌ Final violations: ${this.stats.finalViolations}`);
    console.log(`  ⚡ Execution time: ${this.stats.executionTime}ms`);
    console.log(`  🎯 Efficiency: ${((this.stats.regexCandidates - this.stats.dataFlowChecked) / this.stats.regexCandidates * 100).toFixed(1)}% early exits`);
  }
}

module.exports = new C029SmartPipelineAnalyzer();
