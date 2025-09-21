/**
 * Heuristic analyzer for: S023 – No JSON Injection
 * Purpose: Prevent JSON injection attacks and unsafe JSON handling
 * Detects: unsafe JSON.parse(), eval() with JSON, JSON.stringify in HTML context
 */

class S023Analyzer {
  constructor() {
    this.ruleId = 'S023';
    this.ruleName = 'No JSON Injection';
    this.description = 'Prevent JSON injection attacks and unsafe JSON handling';
    
    // Patterns that indicate user input sources
    this.userInputPatterns = [
      /localStorage\.getItem/,
      /sessionStorage\.getItem/,
      /window\.location/,
      /location\.(search|hash|href)/,
      /URLSearchParams/,
      /req\.(body|query|params)/,
      /request\.(body|query|params)/,
      /document\.cookie/,
      /window\.name/,
      /postMessage/,
      /fetch\(/,
      /axios\./,
      /xhr\./
    ];
    
    // Patterns that indicate validation/safety measures
    this.validationPatterns = [
      /try\s*\{/,
      /catch\s*\(/,
      /if\s*\(/,
      /typeof\s+/,
      /instanceof\s+/,
      /\.length\s*[><=]/,
      /validate/i,
      /check/i,
      /isValid/i,
      /sanitize/i,
      /escape/i,
      /filter/i
    ];
    
    // HTML context patterns
    this.htmlContextPatterns = [
      /innerHTML/,
      /outerHTML/,
      /insertAdjacentHTML/,
      /document\.write/,
      /\.html\(/,
      /<script/i,
      /<\/script>/i
    ];
    
    // JSON patterns for eval detection
    this.jsonPatterns = [
      /json/i,
      /\{.*\}/,
      /\[.*\]/,
      /parse/i,
      /stringify/i
    ];
  }

  async analyze(files, language, options = {}) {
    const violations = [];
    
    for (const filePath of files) {
      try {
        const fileViolations = await this.analyzeFile(filePath, language, options);
        violations.push(...fileViolations);
      } catch (error) {
        if (options.verbose) {
          console.warn(`⚠️ Failed to analyze ${filePath}: ${error.message}`);
        }
      }
    }
    
    return violations;
  }

  async analyzeFile(filePath, language, options = {}) {
    switch (language) {
      case 'typescript':
      case 'javascript':
        return this.analyzeJavaScript(filePath, options);
      default:
        return [];
    }
  }

  async analyzeJavaScript(filePath, options = {}) {
    try {
      // Try AST analysis first (preferred method)
      const astAnalyzer = require('./ast-analyzer.js');
      const astViolations = await astAnalyzer.analyze([filePath], 'javascript', options);
      if (astViolations.length > 0) {
        return astViolations;
      }
    } catch (astError) {
      if (options.verbose) {
        console.log(`⚠️ AST analysis failed for ${filePath}, falling back to regex`);
      }
    }
    
    // Fallback to regex analysis
    return this.analyzeWithRegex(filePath, options);
  }

  async analyzeWithRegex(filePath, options = {}) {
    const fs = require('fs');
    const path = require('path');
    
    try {
      const content = fs.readFileSync(filePath, 'utf8');
      const violations = [];
      const lines = content.split('\n');
      
      // 1. Check JSON.parse() calls
      const jsonParseViolations = this.checkJsonParseCalls(content, lines, filePath);
      violations.push(...jsonParseViolations);
      
      // 2. Check eval() with JSON patterns
      const evalViolations = this.checkEvalWithJson(content, lines, filePath);
      violations.push(...evalViolations);
      
      // 3. Check JSON.stringify in HTML context
      const htmlViolations = this.checkJsonStringifyInHtml(content, lines, filePath);
      violations.push(...htmlViolations);
      
      return violations;
    } catch (error) {
      if (options.verbose) {
        console.warn(`⚠️ Failed to read file ${filePath}: ${error.message}`);
      }
      return [];
    }
  }

  checkJsonParseCalls(content, lines, filePath) {
    const violations = [];
    const jsonParsePattern = /JSON\.parse\s*\(\s*([^)]+)\)/g;
    let match;
    
    while ((match = jsonParsePattern.exec(content)) !== null) {
      const lineNumber = content.substring(0, match.index).split('\n').length;
      const lineText = lines[lineNumber - 1] || '';
      const argument = match[1].trim();
      
      // Check if argument is from user input
      if (this.isUserInputArgument(argument)) {
        // Check if there's validation around this call
        if (!this.hasValidationContext(content, match.index, lineNumber, lines)) {
          violations.push({
            ruleId: this.ruleId,
            file: filePath,
            line: lineNumber,
            column: match.index - content.lastIndexOf('\n', match.index),
            message: 'Unsafe JSON parsing - validate input before parsing',
            severity: 'warning',
            code: lineText.trim(),
            type: 'unsafe_json_parse',
            confidence: 0.8,
            suggestion: 'Validate input before parsing JSON or use try-catch block'
          });
        }
      }
    }
    
    return violations;
  }

  checkEvalWithJson(content, lines, filePath) {
    const violations = [];
    const evalPattern = /eval\s*\(\s*([^)]+)\)/g;
    let match;
    
    while ((match = evalPattern.exec(content)) !== null) {
      const lineNumber = content.substring(0, match.index).split('\n').length;
      const lineText = lines[lineNumber - 1] || '';
      const argument = match[1].trim();
      
      // Check if eval contains JSON patterns
      if (this.containsJsonPattern(argument)) {
        violations.push({
          ruleId: this.ruleId,
          file: filePath,
          line: lineNumber,
          column: match.index - content.lastIndexOf('\n', match.index),
          message: 'Never use eval() to process JSON data - use JSON.parse() instead',
          severity: 'error',
          code: lineText.trim(),
          type: 'eval_json',
          confidence: 0.9,
          suggestion: 'Use JSON.parse() instead of eval() for parsing JSON'
        });
      }
    }
    
    return violations;
  }

  checkJsonStringifyInHtml(content, lines, filePath) {
    const violations = [];
    const jsonStringifyPattern = /JSON\.stringify\s*\([^)]+\)/g;
    let match;
    
    while ((match = jsonStringifyPattern.exec(content)) !== null) {
      const lineNumber = content.substring(0, match.index).split('\n').length;
      const lineText = lines[lineNumber - 1] || '';
      
      // Check if JSON.stringify is used in HTML context
      if (this.isInHtmlContext(content, match.index)) {
        violations.push({
          ruleId: this.ruleId,
          file: filePath,
          line: lineNumber,
          column: match.index - content.lastIndexOf('\n', match.index),
          message: 'JSON.stringify output should be escaped when used in HTML context',
          severity: 'warning',
          code: lineText.trim(),
          type: 'json_stringify_html',
          confidence: 0.7,
          suggestion: 'Escape JSON.stringify output when inserting into HTML'
        });
      }
    }
    
    return violations;
  }

  isUserInputArgument(argument) {
    return this.userInputPatterns.some(pattern => pattern.test(argument));
  }

  hasValidationContext(content, matchIndex, lineNumber, lines) {
    // Check surrounding lines for validation patterns
    const startLine = Math.max(0, lineNumber - 3);
    const endLine = Math.min(lines.length, lineNumber + 2);
    
    for (let i = startLine; i < endLine; i++) {
      const line = lines[i] || '';
      if (this.validationPatterns.some(pattern => pattern.test(line))) {
        return true;
      }
    }
    
    // Check if the call is inside a try block
    const beforeText = content.substring(Math.max(0, matchIndex - 200), matchIndex);
    const afterText = content.substring(matchIndex, Math.min(content.length, matchIndex + 100));
    
    return /try\s*\{[^}]*$/.test(beforeText) || /catch\s*\(/.test(afterText);
  }

  containsJsonPattern(text) {
    return this.jsonPatterns.some(pattern => pattern.test(text));
  }

  isInHtmlContext(content, matchIndex) {
    // Check surrounding context for HTML patterns
    const contextStart = Math.max(0, matchIndex - 100);
    const contextEnd = Math.min(content.length, matchIndex + 100);
    const context = content.substring(contextStart, contextEnd);
    
    return this.htmlContextPatterns.some(pattern => pattern.test(context));
  }

  // Utility method for file extension checking
  isSupportedFile(filePath) {
    const supportedExtensions = ['.js', '.ts', '.jsx', '.tsx', '.mjs', '.cjs'];
    const path = require('path');
    return supportedExtensions.includes(path.extname(filePath));
  }
}

module.exports = new S023Analyzer();
