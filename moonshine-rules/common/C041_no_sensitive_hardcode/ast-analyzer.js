const fs = require('fs');
const path = require('path');

class C041ASTAnalyzer {
  constructor() {
    this.ruleId = 'C041';
    this.ruleName = 'No Hardcoded Sensitive Information (AST-Enhanced)';
    this.description = 'AST-based detection of hardcoded sensitive information - superior to regex approach';
  }

  async analyze(files, language, options = {}) {
    const violations = [];
    
    for (const filePath of files) {
      if (options.verbose) {
        console.log(`🎯 Running C041 AST analysis on ${path.basename(filePath)}`);
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
      // Use the registry's parseCode method
      ast = await astModules.parseCode(content, 'javascript', filePath);
      if (!ast) {
        throw new Error('AST parsing returned null');
      }
    } catch (parseError) {
      throw new Error(`Parse error: ${parseError.message}`);
    }

    // Traverse AST to find sensitive information - mimicking ESLint's approach
    const rootNode = ast.program || ast; // Handle both Babel and ESLint formats
    this.traverseAST(rootNode, (node) => {
      if (this.isLiteralNode(node)) {
        const violation = this.checkLiteralForSensitiveInfo(node, filePath, content);
        if (violation) {
          violations.push(violation);
        }
      }
      
      if (this.isTemplateLiteralNode(node)) {
        const violation = this.checkTemplateLiteralForSensitiveInfo(node, filePath, content);
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

  isLiteralNode(node) {
    // Support both ESLint format (Literal) and Babel format (StringLiteral)
    return node && (node.type === 'Literal' || node.type === 'StringLiteral') && typeof node.value === 'string';
  }

  isTemplateLiteralNode(node) {
    return node && node.type === 'TemplateLiteral' && 
           node.quasis && node.quasis.length === 1 && // No variable interpolation
           node.expressions && node.expressions.length === 0;
  }

  checkLiteralForSensitiveInfo(node, filePath, content) {
    const value = node.value;
    if (!value || value.length < 4) return null;
    
    const lines = content.split('\n');
    const lineNumber = node.loc.start.line;
    const lineText = lines[lineNumber - 1] || '';
    
    // Skip if it's in UI/component context - same as ESLint
    if (this.isFalsePositive(value, lineText)) {
      return null;
    }
    
    // Check against sensitive patterns - enhanced version of ESLint patterns
    const sensitivePattern = this.detectSensitivePattern(value, lineText);
    if (sensitivePattern) {
      return {
        ruleId: this.ruleId,
        file: filePath,
        line: lineNumber,
        column: node.loc.start.column + 1,
        message: sensitivePattern.message,
        severity: 'warning', // Match ESLint severity
        code: lineText.trim(),
        type: sensitivePattern.type,
        confidence: sensitivePattern.confidence,
        suggestion: sensitivePattern.suggestion
      };
    }
    
    return null;
  }

  checkTemplateLiteralForSensitiveInfo(node, filePath, content) {
    if (!node.quasis || node.quasis.length !== 1) return null;
    
    const value = node.quasis[0].value.raw;
    if (!value || value.length < 4) return null;
    
    // Create a mock literal node for consistent processing
    const mockNode = {
      ...node,
      value: value,
      loc: node.loc
    };
    
    return this.checkLiteralForSensitiveInfo(mockNode, filePath, content);
  }

  detectSensitivePattern(value, lineText) {
    const lowerValue = value.toLowerCase();
    const lowerLine = lineText.toLowerCase();
    
    // Enhanced patterns based on ESLint rule but with better detection
    const sensitivePatterns = [
      {
        type: 'password',
        condition: () => /password/i.test(lineText) && value.length >= 4,
        message: 'Potential hardcoded sensitive information detected. Move sensitive values to environment variables or secure config files.',
        confidence: 0.8,
        suggestion: 'Move sensitive values to environment variables or secure config files'
      },
      {
        type: 'secret',
        condition: () => /secret/i.test(lineText) && value.length >= 6,
        message: 'Potential hardcoded sensitive information detected. Move sensitive values to environment variables or secure config files.',
        confidence: 0.8,
        suggestion: 'Use environment variables for secrets'
      },
      {
        type: 'api_key',
        condition: () => /api[_-]?key/i.test(lineText) && value.length >= 10,
        message: 'Potential hardcoded sensitive information detected. Move sensitive values to environment variables or secure config files.',
        confidence: 0.9,
        suggestion: 'Use environment variables for API keys'
      },
      {
        type: 'auth_token',
        condition: () => /auth[_-]?token/i.test(lineText) && value.length >= 16,
        message: 'Potential hardcoded sensitive information detected. Move sensitive values to environment variables or secure config files.',
        confidence: 0.9,
        suggestion: 'Store tokens in secure storage'
      },
      {
        type: 'access_token',
        condition: () => /access[_-]?token/i.test(lineText) && value.length >= 16,
        message: 'Potential hardcoded sensitive information detected. Move sensitive values to environment variables or secure config files.',
        confidence: 0.9,
        suggestion: 'Store tokens in secure storage'
      },
      {
        type: 'database_url',
        condition: () => /(mongodb|mysql|postgres|redis):\/\//i.test(value) && value.length >= 10,
        message: 'Potential hardcoded sensitive information detected. Move sensitive values to environment variables or secure config files.',
        confidence: 0.95,
        suggestion: 'Use environment variables for database connections'
      }
    ];
    
    for (const pattern of sensitivePatterns) {
      if (pattern.condition()) {
        return pattern;
      }
    }
    
    return null;
  }

  isFalsePositive(value, sourceCode) {
    const lowerValue = value.toLowerCase();
    const lowerLine = sourceCode.toLowerCase();
    
    // Global false positive indicators - same as ESLint
    const globalFalsePositives = [
      'test', 'mock', 'example', 'demo', 'sample', 'placeholder', 'dummy', 'fake',
      'xmlns', 'namespace', 'schema', 'w3.org', 'google.com', 'googleapis.com',
      'error', 'message', 'missing', 'invalid', 'failed', 'localhost', '127.0.0.1'
    ];
    
    // Check global false positives
    if (globalFalsePositives.some(pattern => lowerValue.includes(pattern))) {
      return true;
    }
    
    // Check if line context suggests UI/component usage - same as ESLint
    if (this.isConfigOrUIContext(lowerLine)) {
      return true;
    }
    
    return false;
  }

  isConfigOrUIContext(line) {
    // Same logic as ESLint rule
    const uiContexts = [
      'inputtype', 'type:', 'type =', 'inputtype=',
      'routes =', 'route:', 'path:', 'routes:', 
      'import {', 'export {', 'from ', 'import ',
      'interface', 'type ', 'enum ',
      'props:', 'defaultprops',
      'schema', 'validator',
      'hook', 'use', 'const use', 'import.*use',
      // React/UI specific
      'textinput', 'input ', 'field ', 'form',
      'component', 'page', 'screen', 'modal',
      // Route/navigation specific  
      'navigation', 'route', 'path', 'url:', 'route:',
      'setuppassword', 'resetpassword', 'forgotpassword',
      'changepassword', 'confirmpassword'
    ];
    
    return uiContexts.some(context => line.includes(context));
  }

  async analyzeWithRegex(filePath, content, config) {
    // Fallback to original regex approach if AST fails
    const originalAnalyzer = require('./analyzer.js');
    return originalAnalyzer.analyzeTypeScript(filePath, content, config);
  }
}

module.exports = new C041ASTAnalyzer();
