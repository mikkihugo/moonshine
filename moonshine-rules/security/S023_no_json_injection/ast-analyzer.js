const fs = require('fs');
const path = require('path');

class S023ASTAnalyzer {
  constructor() {
    this.ruleId = 'S023';
    this.ruleName = 'No JSON Injection Prevention (AST-Enhanced)';
    this.description = 'AST-based detection of unsafe JSON parsing and injection vulnerabilities';
  }

  async analyze(files, language, options = {}) {
    const violations = [];
    
    for (const filePath of files) {
      if (options.verbose) {
        console.log(`🎯 Running S023 AST analysis on ${path.basename(filePath)}`);
      }
      
      try {
        const content = fs.readFileSync(filePath, 'utf8');
        const fileViolations = await this.analyzeFile(filePath, content, language, options);
        violations.push(...fileViolations);
      } catch (error) {
        console.warn(`⚠️ Failed to analyze ${filePath}: ${error.message}`);
      }
    }
    
    return violations;
  }

  async analyzeFile(filePath, content, language, config) {
    switch (language) {
      case 'typescript':
      case 'javascript':
        return this.analyzeJSTS(filePath, content, config);
      default:
        return [];
    }
  }

  async analyzeJSTS(filePath, content, config) {
    const violations = [];
    
    try {
      // Try AST analysis first (like ESLint approach)
      const astViolations = await this.analyzeWithAST(filePath, content, config);
      if (astViolations.length > 0) {
        violations.push(...astViolations);
      }
    } catch (astError) {
      if (config.verbose) {
        console.log(`⚠️ AST analysis failed for ${path.basename(filePath)}, falling back to regex`);
      }
      
      // Fallback to regex-based analysis
      const regexViolations = await this.analyzeWithRegex(filePath, content, config);
      violations.push(...regexViolations);
    }
    
    return violations;
  }

  async analyzeWithAST(filePath, content, config) {
    const violations = [];
    
    // Import AST modules dynamically
    let astModules;
    try {
      astModules = require('../../../core/ast-modules');
    } catch (error) {
      throw new Error('AST modules not available');
    }

    // Try to parse with AST
    let ast;
    try {
      ast = await astModules.parseCode(content, 'javascript', filePath);
      if (!ast) {
        throw new Error('AST parsing returned null');
      }
    } catch (parseError) {
      throw new Error(`Parse error: ${parseError.message}`);
    }

    // Traverse AST to find JSON injection vulnerabilities - mimicking ESLint's approach
    const rootNode = ast.program || ast;
    this.traverseAST(rootNode, (node) => {
      // Check JSON.parse() calls
      if (this.isJsonParseCall(node)) {
        const violation = this.checkJsonParseForUnsafeUsage(node, filePath, content);
        if (violation) {
          violations.push(violation);
        }
      }
      
      // Check eval() with JSON patterns
      if (this.isEvalCall(node)) {
        const violation = this.checkEvalForJsonUsage(node, filePath, content);
        if (violation) {
          violations.push(violation);
        }
      }
      
      // Check JSON.stringify in HTML context
      if (this.isJsonStringifyCall(node)) {
        const violation = this.checkJsonStringifyInHtmlContext(node, filePath, content);
        if (violation) {
          violations.push(violation);
        }
      }
    });

    return violations;
  }

  traverseAST(node, callback) {
    if (!node || typeof node !== 'object') return;
    
    callback(node);
    
    for (const key in node) {
      if (key === 'parent' || key === 'leadingComments' || key === 'trailingComments') continue;
      
      const child = node[key];
      if (Array.isArray(child)) {
        child.forEach(item => this.traverseAST(item, callback));
      } else if (child && typeof child === 'object') {
        this.traverseAST(child, callback);
      }
    }
  }

  isJsonParseCall(node) {
    return node.type === 'CallExpression' &&
           node.callee && 
           node.callee.type === 'MemberExpression' &&
           node.callee.object && node.callee.object.name === 'JSON' &&
           node.callee.property && node.callee.property.name === 'parse';
  }

  isEvalCall(node) {
    return node.type === 'CallExpression' &&
           node.callee && 
           node.callee.type === 'Identifier' &&
           node.callee.name === 'eval';
  }

  isJsonStringifyCall(node) {
    return node.type === 'CallExpression' &&
           node.callee && 
           node.callee.type === 'MemberExpression' &&
           node.callee.object && node.callee.object.name === 'JSON' &&
           node.callee.property && node.callee.property.name === 'stringify';
  }

  checkJsonParseForUnsafeUsage(node, filePath, content) {
    if (!node.arguments || node.arguments.length === 0) return null;
    
    const lines = content.split('\n');
    const lineNumber = node.loc.start.line;
    const lineText = lines[lineNumber - 1] || '';
    
    // Check if the argument is from user input (similar to ESLint logic)
    const argument = node.arguments[0];
    if (this.isUserInputSource(argument, content)) {
      // Check if there's validation before JSON.parse
      if (!this.hasValidationBefore(node, content)) {
        return {
          ruleId: this.ruleId,
          file: filePath,
          line: lineNumber,
          column: node.loc.start.column + 1,
          message: 'Unsafe JSON parsing - validate input before parsing',
          severity: 'warning',
          code: lineText.trim(),
          type: 'unsafe_json_parse',
          confidence: 0.8,
          suggestion: 'Validate input before parsing JSON'
        };
      }
    }
    
    return null;
  }

  checkEvalForJsonUsage(node, filePath, content) {
    if (!node.arguments || node.arguments.length === 0) return null;
    
    const lines = content.split('\n');
    const lineNumber = node.loc.start.line;
    const lineText = lines[lineNumber - 1] || '';
    
    // Check if eval contains JSON patterns
    const argument = node.arguments[0];
    if (this.containsJsonPattern(argument, content)) {
      return {
        ruleId: this.ruleId,
        file: filePath,
        line: lineNumber,
        column: node.loc.start.column + 1,
        message: 'Never use eval() to process JSON data - use JSON.parse() instead',
        severity: 'error',
        code: lineText.trim(),
        type: 'eval_json',
        confidence: 0.9,
        suggestion: 'Use JSON.parse() instead of eval()'
      };
    }
    
    return null;
  }

  checkJsonStringifyInHtmlContext(node, filePath, content) {
    const lines = content.split('\n');
    const lineNumber = node.loc.start.line;
    const lineText = lines[lineNumber - 1] || '';
    
    // Check if JSON.stringify is used in HTML context
    if (this.isInHtmlContext(node, content)) {
      return {
        ruleId: this.ruleId,
        file: filePath,
        line: lineNumber,
        column: node.loc.start.column + 1,
        message: 'JSON.stringify output should be escaped when used in HTML context',
        severity: 'warning',
        code: lineText.trim(),
        type: 'json_stringify_html',
        confidence: 0.7,
        suggestion: 'Escape JSON.stringify output in HTML context'
      };
    }
    
    return null;
  }

  isUserInputSource(node, content) {
    if (!node) return false;
    
    // Check for common user input patterns (similar to ESLint)
    const userInputPatterns = [
      /localStorage\.getItem/,
      /sessionStorage\.getItem/,
      /window\.location/,
      /location\.(search|hash)/,
      /URLSearchParams/,
      /req\.(body|query|params)/,
      /request\.(body|query|params)/
    ];
    
    return userInputPatterns.some(pattern => {
      const nodeText = this.getNodeText(node, content);
      return pattern.test(nodeText);
    });
  }

  hasValidationBefore(node, content) {
    // Simple check for validation patterns before JSON.parse
    const lines = content.split('\n');
    const lineNumber = node.loc.start.line;
    
    // Check previous lines for validation patterns
    for (let i = Math.max(0, lineNumber - 5); i < lineNumber - 1; i++) {
      const line = lines[i] || '';
      if (this.containsValidationPattern(line)) {
        return true;
      }
    }
    
    return false;
  }

  containsValidationPattern(line) {
    const validationPatterns = [
      /try\s*{/,
      /catch\s*\(/,
      /if\s*\(/,
      /typeof\s+/,
      /instanceof\s+/,
      /\.length\s*>/,
      /validate/i,
      /check/i,
      /isValid/i
    ];
    
    return validationPatterns.some(pattern => pattern.test(line));
  }

  containsJsonPattern(node, content) {
    const nodeText = this.getNodeText(node, content);
    return /json|JSON|\{|\[/.test(nodeText);
  }

  isInHtmlContext(node, content) {
    // Check if JSON.stringify is used in HTML context
    const htmlPatterns = [
      /innerHTML/,
      /outerHTML/,
      /insertAdjacentHTML/,
      /document\.write/,
      /\.html\(/
    ];
    
    const surroundingText = this.getSurroundingText(node, content, 3);
    return htmlPatterns.some(pattern => pattern.test(surroundingText));
  }

  getNodeText(node, content) {
    if (!node || !node.loc) return '';
    const lines = content.split('\n');
    return lines[node.loc.start.line - 1] || '';
  }

  getSurroundingText(node, content, radius = 2) {
    if (!node || !node.loc) return '';
    const lines = content.split('\n');
    const lineNumber = node.loc.start.line;
    
    const startLine = Math.max(0, lineNumber - radius - 1);
    const endLine = Math.min(lines.length, lineNumber + radius);
    
    return lines.slice(startLine, endLine).join('\n');
  }

  async analyzeWithRegex(filePath, content, config) {
    // Fallback regex analysis for basic JSON.parse detection
    const violations = [];
    const lines = content.split('\n');
    
    const jsonParsePattern = /JSON\.parse\s*\(\s*([^)]+)\)/g;
    let match;
    
    while ((match = jsonParsePattern.exec(content)) !== null) {
      const line = content.substring(0, match.index).split('\n').length;
      const lineText = lines[line - 1] || '';
      
      // Simple check for unsafe patterns
      const argument = match[1];
      if (/localStorage\.getItem|sessionStorage\.getItem/.test(argument)) {
        violations.push({
          ruleId: this.ruleId,
          file: filePath,
          line: line,
          column: match.index - content.lastIndexOf('\n', match.index),
          message: 'Unsafe JSON parsing - validate input before parsing',
          severity: 'warning',
          code: lineText.trim(),
          type: 'unsafe_json_parse_regex',
          confidence: 0.6,
          suggestion: 'Validate input before parsing JSON'
        });
      }
    }
    
    return violations;
  }
}

module.exports = new S023ASTAnalyzer();
