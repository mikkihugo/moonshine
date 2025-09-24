/**
 * Heuristic analyzer for: C047 – Logic retry không được viết lặp lại nhiều nơi
 * Purpose: Detect duplicate retry logic patterns and suggest centralized retry utilities
 */

class C047Analyzer {
  constructor() {
    this.ruleId = 'C047';
    this.ruleName = 'No Duplicate Retry Logic';
    this.description = 'Logic retry không được viết lặp lại nhiều nơi - use centralized retry utility instead';
    
    // Enhanced patterns that indicate retry logic
    this.retryIndicators = [
      'maxretries', 'maxattempts', 'maxtries',
      'attempt', 'retry', 'tries', 'retries',
      'backoff', 'delay', 'timeout',
      'exponential', 'linear'
    ];
    
    // Architectural layer detection patterns
    this.layerPatterns = {
      ui: ['component', 'view', 'page', 'modal', 'form', 'screen', 'widget', 'button'],
      logic: ['service', 'usecase', 'viewmodel', 'controller', 'handler', 'manager', 'business'],
      repository: ['repository', 'dao', 'store', 'cache', 'persistence', 'data'],
      infrastructure: ['client', 'adapter', 'gateway', 'connector', 'network', 'http', 'api']
    };
    
    // Retry purpose classification
    this.purposeIndicators = {
      network: ['fetch', 'axios', 'request', 'http', 'api', 'ajax', 'xhr'],
      database: ['query', 'transaction', 'connection', 'db', 'sql', 'insert', 'update'],
      validation: ['validate', 'check', 'verify', 'confirm', 'assert'],
      ui: ['click', 'submit', 'load', 'render', 'update', 'refresh'],
      auth: ['login', 'authenticate', 'authorize', 'token', 'session']
    };
    
    // Allowed centralized retry utilities
    this.allowedRetryUtils = [
      'RetryUtil', 'retryWithBackoff', 'withRetry', 'retry',
      'retryAsync', 'retryPromise', 'retryOperation',
      'exponentialBackoff', 'linearBackoff'
    ];
    
    // Detected retry patterns for duplicate checking with context
    this.retryPatterns = [];
  }

  async analyze(files, language, options = {}) {
    const violations = [];
    
    for (const filePath of files) {
      if (options.verbose) {
        console.log(`🔍 Running C047 analysis on ${require('path').basename(filePath)}`);
      }
      
      try {
        const content = require('fs').readFileSync(filePath, 'utf8');
        this.retryPatterns = []; // Reset for each file
        const fileViolations = this.analyzeFile(content, filePath);
        violations.push(...fileViolations);
      } catch (error) {
        console.warn(`⚠️ Failed to analyze ${filePath}: ${error.message}`);
      }
    }
    
    return violations;
  }

  analyzeFile(content, filePath) {
    const violations = [];
    const lines = content.split('\n');
    
    // Find all retry patterns in the file
    this.findRetryPatterns(lines, filePath);
    
    // Add architectural context to patterns
    this.retryPatterns.forEach(pattern => {
      if (!pattern.context) {
        const contextLines = lines.slice(Math.max(0, pattern.line - 10), pattern.line + 10);
        const contextContent = contextLines.join('\n');
        pattern.context = this.analyzeArchitecturalContext(filePath, contextContent);
      }
    });
    
    // Enhanced duplicate detection with architectural intelligence
    const duplicateGroups = this.enhancedDuplicateDetection();
    
    // Generate violations with architectural context
    const enhancedViolations = this.generateEnhancedViolations(duplicateGroups, filePath);
    violations.push(...enhancedViolations);
    
    return violations;
  }
  
  findRetryPatterns(lines, filePath) {
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      const trimmedLine = line.trim().toLowerCase();
      
      // Skip comments and empty lines
      if (!trimmedLine || trimmedLine.startsWith('//') || trimmedLine.startsWith('/*')) {
        continue;
      }
      
      // Skip if using allowed retry utilities
      if (this.usesAllowedRetryUtil(line)) {
        continue;
      }
      
      // Pattern 1: For/while loop with retry indicators
      if (this.isRetryLoopPattern(line, lines, i)) {
        const pattern = this.extractRetryPattern(lines, i, 'loop');
        if (pattern) {
          this.retryPatterns.push({
            ...pattern,
            line: i + 1,
            column: line.indexOf(line.trim()) + 1
          });
        }
      }
      
      // Pattern 2: Variable declarations with retry indicators
      else if (this.isRetryVariableDeclaration(trimmedLine)) {
        // Additional context check for false positives
        const contextLines = lines.slice(Math.max(0, i - 5), Math.min(lines.length, i + 10));
        const contextText = contextLines.join('\n').toLowerCase();
        
        // Skip if it's clearly data processing context
        if (contextText.includes('filter(') && contextText.includes('map(') && 
            !contextText.includes('try') && !contextText.includes('catch') &&
            !contextText.includes('timeout') && !contextText.includes('delay')) {
          continue;
        }
        
        const pattern = this.extractRetryPattern(lines, i, 'variable');
        if (pattern) {
          this.retryPatterns.push({
            ...pattern,
            line: i + 1,
            column: line.indexOf(line.trim()) + 1
          });
        }
      }
      
      // Pattern 3: Recursive function with retry logic
      else if (this.isRetryFunctionPattern(lines, i)) {
        // Additional check for actual retry vs. recursive data processing
        const contextLines = lines.slice(i, Math.min(lines.length, i + 20));
        const contextText = contextLines.join('\n').toLowerCase();
        
        // Skip recursive data processing functions
        if (contextText.includes('flatmap') || contextText.includes('children') ||
            (contextText.includes('recursive') && !contextText.includes('retry'))) {
          continue;
        }
        
        const pattern = this.extractRetryPattern(lines, i, 'recursive');
        if (pattern) {
          this.retryPatterns.push({
            ...pattern,
            line: i + 1,
            column: line.indexOf(line.trim()) + 1
          });
        }
      }
    }
  }
  
  isRetryLoopPattern(line, lines, index) {
    const trimmedLine = line.trim().toLowerCase();
    
    // Check for for/while loops with retry indicators
    if ((trimmedLine.includes('for') || trimmedLine.includes('while')) &&
        (trimmedLine.includes('(') && trimmedLine.includes(')'))) {
      
      // Look in the loop condition and surrounding context for retry indicators
      const contextLines = lines.slice(Math.max(0, index - 2), Math.min(lines.length, index + 10));
      const contextText = contextLines.join('\n').toLowerCase();
      
      return this.retryIndicators.some(indicator => contextText.includes(indicator)) &&
             (contextText.includes('try') || contextText.includes('catch') || contextText.includes('error'));
    }
    
    return false;
  }
  
  isRetryVariableDeclaration(line) {
    // Skip simple constant definitions (export const X = number)
    if (/^export\s+const\s+\w+\s*=\s*\d+/.test(line.trim())) {
      return false;
    }
    
    // Skip test-related patterns (jest mocks, test descriptions)
    if (/it\(|describe\(|test\(|mock\w*\(|\.mock/.test(line)) {
      return false;
    }
    
    // Skip HTTP response patterns
    if (/response\.status\(|\.json\(|return.*errors/.test(line)) {
      return false;
    }
    
    // Skip simple array declarations
    if (/(?:const|let|var)\s+\w+Array\s*=\s*\[\s*\]/.test(line.trim())) {
      return false;
    }
    
    // Check for variable declarations with retry-related names that involve logic
    const declarationPatterns = [
      /(?:const|let|var)\s+.*(?:retry|attempt|tries|maxretries|maxattempts)/,
      /(?:retry|attempt|tries|maxretries|maxattempts)\s*[:=]/
    ];
    
    // Only consider it a retry pattern if it has logical complexity
    if (declarationPatterns.some(pattern => pattern.test(line))) {
      // Exclude simple constant assignments to numbers
      if (/=\s*\d+\s*[;,]?\s*$/.test(line.trim())) {
        return false;
      }
      // Exclude simple array initializations
      if (/=\s*\[\s*\]\s*[;,]?\s*$/.test(line.trim())) {
        return false;
      }
      return true;
    }
    
    return false;
  }
  
  isRetryFunctionPattern(lines, index) {
    const line = lines[index].trim().toLowerCase();
    
    // Skip test functions
    if (line.includes('it(') || line.includes('describe(') || line.includes('test(')) {
      return false;
    }
    
    // Check for function declarations with retry indicators
    if ((line.includes('function') || line.includes('=>')) &&
        this.retryIndicators.some(indicator => line.includes(indicator))) {
      
      // Skip obvious non-retry functions (formatters, validators, mappers)
      if (line.includes('format') || line.includes('validate') || line.includes('map') || 
          line.includes('filter') || line.includes('transform')) {
        return false;
      }
      
      // Look for retry logic in function body
      const functionBody = this.getFunctionBody(lines, index);
      if (!functionBody) return false;
      
      // Check if it's a recursive function (calls itself) without actual retry patterns
      const functionName = this.extractFunctionName(line);
      if (functionName && functionBody.includes(functionName) && 
          !functionBody.includes('try') && !functionBody.includes('catch') &&
          !functionBody.includes('timeout') && !functionBody.includes('delay')) {
        return false;
      }
      
      return functionBody && 
             this.retryIndicators.some(indicator => functionBody.includes(indicator)) &&
             (functionBody.includes('try') || functionBody.includes('catch') || 
              functionBody.includes('throw') || functionBody.includes('error'));
    }
    
    return false;
  }
  
  usesAllowedRetryUtil(line) {
    return this.allowedRetryUtils.some(util => 
      line.includes(util) && (line.includes('.') || line.includes('('))
    );
  }
  
  extractRetryPattern(lines, startIndex, type) {
    // Extract the pattern characteristics for similarity comparison
    const contextLines = lines.slice(startIndex, Math.min(lines.length, startIndex + 20));
    const contextText = contextLines.join('\n').toLowerCase();
    
    // Extract key characteristics
    const characteristics = {
      type: type,
      hasForLoop: contextText.includes('for'),
      hasWhileLoop: contextText.includes('while'),
      hasDoWhile: contextText.includes('do') && contextText.includes('while'),
      hasTryCatch: contextText.includes('try') && contextText.includes('catch'),
      hasMaxRetries: /max.*(?:retry|attempt|tries)/.test(contextText),
      hasBackoff: contextText.includes('backoff') || contextText.includes('delay') || contextText.includes('timeout'),
      hasExponential: contextText.includes('exponential') || /math\.pow|2\s*\*\*|\*\s*2/.test(contextText),
      hasLinear: contextText.includes('linear') || /\*\s*(?:attempt|retry)/.test(contextText),
      hasSetTimeout: contextText.includes('settimeout') || contextText.includes('promise') && contextText.includes('resolve'),
      signature: this.generatePatternSignature(contextText)
    };
    
    return characteristics;
  }
  
  generatePatternSignature(contextText) {
    // Create a normalized signature for pattern matching
    let signature = '';
    
    if (contextText.includes('for')) signature += 'FOR_';
    if (contextText.includes('while')) signature += 'WHILE_';
    if (/max.*(?:retry|attempt)/.test(contextText)) signature += 'MAX_';
    if (contextText.includes('try') && contextText.includes('catch')) signature += 'TRYCATCH_';
    if (contextText.includes('settimeout') || contextText.includes('delay')) signature += 'DELAY_';
    if (contextText.includes('exponential') || /math\.pow/.test(contextText)) signature += 'EXPONENTIAL_';
    if (contextText.includes('throw')) signature += 'THROW_';
    
    return signature || 'GENERIC_RETRY';
  }
  
  extractFunctionName(line) {
    // Extract function name from function declaration
    const functionMatch = line.match(/(?:function\s+(\w+)|(\w+)\s*(?:\([^)]*\))?\s*=>|(\w+)\s*:\s*(?:async\s+)?function)/);
    if (functionMatch) {
      return functionMatch[1] || functionMatch[2] || functionMatch[3];
    }
    
    // Try to extract from method declaration
    const methodMatch = line.match(/(\w+)\s*\(/);
    if (methodMatch) {
      return methodMatch[1];
    }
    
    return null;
  }
  
  getFunctionBody(lines, startIndex) {
    // Extract function body for analysis
    let braceDepth = 0;
    let foundStartBrace = false;
    const bodyLines = [];
    
    for (let i = startIndex; i < lines.length; i++) {
      const line = lines[i];
      
      for (const char of line) {
        if (char === '{') {
          braceDepth++;
          foundStartBrace = true;
        } else if (char === '}') {
          braceDepth--;
        }
      }
      
      if (foundStartBrace) {
        bodyLines.push(line);
        
        if (braceDepth === 0) {
          break;
        }
      }
    }
    
    return bodyLines.join('\n').toLowerCase();
  }
  
  findDuplicateRetryLogic() {
    const groups = [];
    const processed = new Set();
    
    this.retryPatterns.forEach((pattern, index) => {
      if (processed.has(index)) return;
      
      const similarPatterns = [pattern];
      processed.add(index);
      
      // Find similar patterns
      this.retryPatterns.forEach((otherPattern, otherIndex) => {
        if (otherIndex !== index && !processed.has(otherIndex)) {
          if (this.areSimilarPatterns(pattern, otherPattern)) {
            similarPatterns.push(otherPattern);
            processed.add(otherIndex);
          }
        }
      });
      
      if (similarPatterns.length > 0) {
        groups.push({
          signature: pattern.signature,
          patterns: similarPatterns
        });
      }
    });
    
    return groups;
  }
  
  areSimilarPatterns(pattern1, pattern2) {
    // Check if two patterns are similar enough to be considered duplicates
    
    // Same signature indicates very similar patterns
    if (pattern1.signature === pattern2.signature) {
      return true;
    }
    
    // Compare characteristics
    const similarities = [
      pattern1.hasForLoop === pattern2.hasForLoop,
      pattern1.hasWhileLoop === pattern2.hasWhileLoop,
      pattern1.hasTryCatch === pattern2.hasTryCatch,
      pattern1.hasMaxRetries === pattern2.hasMaxRetries,
      pattern1.hasBackoff === pattern2.hasBackoff,
      pattern1.hasSetTimeout === pattern2.hasSetTimeout
    ].filter(Boolean).length;
    
    // Consider patterns similar if they share at least 4 out of 6 characteristics
    return similarities >= 4;
  }
  
  getFunctionNameForLine(lines, lineIndex) {
    // Look backwards to find the function declaration for this line
    for (let i = lineIndex; i >= 0; i--) {
      const line = lines[i].trim();
      
      // Match function declarations
      const functionMatch = line.match(/(?:function|async\s+function)\s+(\w+)/);
      if (functionMatch) {
        return functionMatch[1];
      }
      
      // Match arrow functions assigned to variables
      const arrowMatch = line.match(/(?:const|let|var)\s+(\w+)\s*=.*=>/);
      if (arrowMatch) {
        return arrowMatch[1];
      }
      
      // Stop at class/module boundaries
      if (line.includes('class ') || line.includes('module.exports') || line.includes('export')) {
        break;
      }
    }
    
    return 'anonymous';
  }

  // 🧠 ARCHITECTURAL INTELLIGENCE METHODS

  analyzeArchitecturalContext(filePath, content) {
    const fileName = require('path').basename(filePath).toLowerCase();
    const dirPath = require('path').dirname(filePath).toLowerCase();
    
    // Determine architectural layer
    let layer = 'unknown';
    for (const [layerName, patterns] of Object.entries(this.layerPatterns)) {
      if (patterns.some(pattern => fileName.includes(pattern) || dirPath.includes(pattern))) {
        layer = layerName;
        break;
      }
    }
    
    // Extract retry purpose/scope
    const purpose = this.extractRetryPurpose(content);
    
    return { layer, purpose, filePath: fileName };
  }

  extractRetryPurpose(content) {
    const contentLower = content.toLowerCase();
    
    for (const [purpose, indicators] of Object.entries(this.purposeIndicators)) {
      if (indicators.some(indicator => contentLower.includes(indicator))) {
        return purpose;
      }
    }
    
    return 'general';
  }

  enhancedDuplicateDetection() {
    const groups = [];
    const processed = new Set();
    
    // Add architectural context to each pattern
    this.retryPatterns.forEach((pattern, index) => {
      if (!pattern.context) {
        // This would be called during pattern extraction in a real implementation
        pattern.context = { layer: 'unknown', purpose: 'general' };
      }
    });
    
    this.retryPatterns.forEach((pattern, index) => {
      if (processed.has(index)) return;
      
      const similarPatterns = [pattern];
      processed.add(index);
      
      this.retryPatterns.forEach((otherPattern, otherIndex) => {
        if (otherIndex !== index && !processed.has(otherIndex)) {
          if (this.areSimilarPatternsWithContext(pattern, otherPattern)) {
            similarPatterns.push(otherPattern);
            processed.add(otherIndex);
          }
        }
      });
      
      if (similarPatterns.length > 1) {
        const legitimacy = this.assessDuplicateLegitimacy(similarPatterns);
        
        if (!legitimacy.isLegitimate) {
          groups.push({
            signature: pattern.signature,
            patterns: similarPatterns,
            legitimacy: legitimacy
          });
        }
      }
    });
    
    return groups;
  }

  areSimilarPatternsWithContext(pattern1, pattern2) {
    // First check basic similarity
    if (!this.areSimilarPatterns(pattern1, pattern2)) {
      return false;
    }
    
    // Enhanced context-aware similarity
    const context1 = pattern1.context || { layer: 'unknown', purpose: 'general' };
    const context2 = pattern2.context || { layer: 'unknown', purpose: 'general' };
    
    // Same layer AND same purpose = likely duplicate
    // Different layer OR different purpose = likely legitimate
    return context1.layer === context2.layer && context1.purpose === context2.purpose;
  }

  assessDuplicateLegitimacy(patterns) {
    const layers = new Set(patterns.map(p => p.context?.layer || 'unknown'));
    const purposes = new Set(patterns.map(p => p.context?.purpose || 'general'));
    
    // Cross-layer retries are often legitimate
    if (layers.size > 1) {
      return {
        isLegitimate: true,
        reason: 'Cross-layer retry patterns are architecturally valid',
        confidence: 'high'
      };
    }
    
    // Different purposes in same layer can be legitimate
    if (purposes.size > 1) {
      return {
        isLegitimate: true,
        reason: 'Different retry purposes in same layer',
        confidence: 'medium'
      };
    }
    
    // Same layer, same purpose - likely duplicate
    const layer = [...layers][0];
    const purpose = [...purposes][0];
    
    return {
      isLegitimate: false,
      reason: `Duplicate ${purpose} retry logic in ${layer} layer`,
      confidence: 'high',
      severity: 'warning'
    };
  }

  generateEnhancedViolations(duplicateGroups, filePath) {
    const violations = [];
    
    duplicateGroups.forEach(group => {
      const firstPattern = group.patterns[0];
      
      violations.push({
        file: filePath,
        line: firstPattern.line,
        column: firstPattern.column || 1,
        message: `${group.legitimacy.reason} (${group.patterns.length} similar patterns found). Consider using a centralized retry utility.`,
        severity: group.legitimacy.severity || 'warning',
        ruleId: this.ruleId,
        type: 'duplicate_retry_logic',
        duplicateCount: group.patterns.length,
        architecturalContext: {
          layers: [...new Set(group.patterns.map(p => p.context?.layer))],
          purposes: [...new Set(group.patterns.map(p => p.context?.purpose))],
          confidence: group.legitimacy.confidence
        }
      });
    });
    
    return violations;
  }
}

module.exports = C047Analyzer;
