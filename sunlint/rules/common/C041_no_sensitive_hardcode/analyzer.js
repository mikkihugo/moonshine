const fs = require('fs');
const path = require('path');

class C041Analyzer {
  constructor() {
    this.ruleId = 'C041';
    this.ruleName = 'No Hardcoded Sensitive Information';
    this.description = 'Không hardcode hoặc push thông tin nhạy cảm vào repo';
  }

  async analyze(files, language, options = {}) {
    const violations = [];
    
    for (const filePath of files) {
      if (options.verbose) {
        console.log(`🔍 Running C041 analysis on ${path.basename(filePath)}`);
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
        return this.analyzeTypeScript(filePath, content, config);
      default:
        return [];
    }
  }

  async analyzeTypeScript(filePath, content, config) {
    const violations = [];
    const lines = content.split('\n');

    lines.forEach((line, index) => {
      const lineNumber = index + 1;
      const trimmedLine = line.trim();

      // Skip comments and imports
      if (this.isCommentOrImport(trimmedLine)) {
        return;
      }

      // Find potential hardcoded sensitive values
      const sensitiveMatches = this.findSensitiveHardcode(trimmedLine, line);
      
      sensitiveMatches.forEach(match => {
        violations.push({
          ruleId: this.ruleId,
          file: filePath,
          line: lineNumber,
          column: match.column,
          message: match.message,
          severity: 'error',
          code: trimmedLine,
          type: match.type,
          confidence: match.confidence,
          suggestion: match.suggestion
        });
      });
    });

    return violations;
  }

  isCommentOrImport(line) {
    const trimmed = line.trim();
    return trimmed.startsWith('//') || 
           trimmed.startsWith('/*') || 
           trimmed.startsWith('*') ||
           trimmed.startsWith('import ') ||
           trimmed.startsWith('export ');
  }

  findSensitiveHardcode(line, originalLine) {
    const matches = [];
    
    // Skip template literals with variables - they are dynamic, not hardcoded
    if (line.includes('${') || line.includes('`')) {
      return matches;
    }
    
    // Skip if line is clearly configuration, type definition, or UI-related
    if (this.isConfigOrUIContext(line)) {
      return matches;
    }
    
    // Look for suspicious patterns with better context awareness
    const patterns = [
      {
        name: 'suspicious_password_variable',
        regex: /(const|let|var)\s+\w*[Pp]ass[Ww]ord\w*\s*=\s*['"`]([^'"`]{4,})['"`]/g,
        severity: 'error',
        message: 'Potential hardcoded password in variable assignment',
        suggestion: 'Move sensitive values to environment variables or secure config files'
      },
      {
        name: 'suspicious_secret_variable',
        regex: /(const|let|var)\s+\w*[Ss]ecret\w*\s*=\s*['"`]([^'"`]{6,})['"`]/g,
        severity: 'error',
        message: 'Potential hardcoded secret in variable assignment',
        suggestion: 'Use environment variables for secrets'
      },
      {
        name: 'suspicious_short_password',
        regex: /(const|let|var)\s+(?!use)\w*([Pp]ass|[Dd]b[Pp]ass|[Aa]dmin)(?!word[A-Z])\w*\s*=\s*['"`]([^'"`]{4,})['"`]/g,
        severity: 'error',
        message: 'Potential hardcoded password or admin credential',
        suggestion: 'Use environment variables for credentials'
      },
      {
        name: 'api_key',
        regex: /(const|let|var)\s+\w*[Aa]pi[Kk]ey\w*\s*=\s*['"`]([^'"`]{10,})['"`]/g,
        severity: 'error',
        message: 'Potential hardcoded API key detected',
        suggestion: 'Use environment variables for API keys'
      },
      {
        name: 'auth_token',
        regex: /(const|let|var)\s+\w*[Tt]oken\w*\s*=\s*['"`]([^'"`]{16,})['"`]/g,
        severity: 'error', 
        message: 'Potential hardcoded authentication token detected',
        suggestion: 'Store tokens in secure storage, not in source code'
      },
      {
        name: 'database_url',
        regex: /['"`](mongodb|mysql|postgres|redis):\/\/[^'"`]+['"`]/gi,
        severity: 'error',
        message: 'Hardcoded database connection string detected',
        suggestion: 'Use environment variables for database connections'
      },
      {
        name: 'suspicious_url',
        regex: /['"`]https?:\/\/(?!localhost|127\.0\.0\.1|example\.com|test\.com|www\.w3\.org|www\.google\.com|googleapis\.com)[^'"`]{20,}['"`]/gi,
        severity: 'warning',
        message: 'Hardcoded external URL detected (consider configuration)',
        suggestion: 'Consider moving URLs to configuration files'
      }
    ];

    // Additional context-aware checks
    patterns.forEach(pattern => {
      let match;
      while ((match = pattern.regex.exec(line)) !== null) {
        // Skip false positives
        if (this.isFalsePositive(line, match[0], pattern.name)) {
          continue;
        }

        matches.push({
          type: pattern.name,
          column: match.index + 1,
          message: pattern.message,
          confidence: this.calculateConfidence(line, match[0], pattern.name),
          suggestion: pattern.suggestion
        });
      }
    });

    return matches;
  }

  isConfigOrUIContext(line) {
    const lowerLine = line.toLowerCase();
    
    // UI/Component contexts - likely false positives
    const uiContexts = [
      'inputtype', 'type:', 'type =', 'type:', 'inputtype=',
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
    
    return uiContexts.some(context => lowerLine.includes(context));
  }

  isFalsePositive(line, matchedText, patternName) {
    const lowerLine = line.toLowerCase();
    const lowerMatch = matchedText.toLowerCase();

    // Global false positive indicators
    const globalFalsePositives = [
      'test', 'mock', 'example', 'demo', 'sample', 'placeholder', 'dummy', 'fake',
      'xmlns', 'namespace', 'schema', 'w3.org', 'google.com', 'googleapis.com',
      'error', 'message', 'missing', 'invalid', 'failed'
    ];

    // Check if the line contains any global false positive indicators
    const hasGlobalFalsePositive = globalFalsePositives.some(pattern => 
      lowerLine.includes(pattern) || lowerMatch.includes(pattern)
    );

    if (hasGlobalFalsePositive) {
      return true;
    }

    // Common false positive patterns
    const falsePositivePatterns = {
      'suspicious_password_variable': [
        'inputtype', 'type:', 'type =', 'activation', 'forgot_password', 'reset_password',
        'setup_password', 'route', 'path', 'hook', 'use', 'change', 'confirm',
        'validation', 'component', 'page', 'screen', 'textinput', 'input',
        'trigger', 'useeffect', 'password.*trigger', 'renewpassword'
      ],
      'suspicious_short_password': [
        'inputtype', 'type:', 'type =', 'activation', 'forgot_password', 'reset_password',
        'setup_password', 'route', 'path', 'hook', 'use', 'change', 'confirm',
        'validation', 'component', 'page', 'screen', 'textinput'
      ],
      'suspicious_secret_variable': [
        'component', 'props', 'state', 'hook', 'use'
      ],
      'suspicious_url': [
        'localhost', '127.0.0.1', 'example.com', 'test.com', 'placeholder',
        'mock', 'w3.org', 'google.com', 'recaptcha', 'googleapis.com'
      ],
      'api_key': [
        'test-', 'mock-', 'example-', 'demo-', 'missing', 'error', 'message'
      ]
    };

    const patterns = falsePositivePatterns[patternName] || [];
    
    // Check if line contains any pattern-specific false positive indicators
    const hasPatternFalsePositive = patterns.some(pattern => 
      lowerLine.includes(pattern) || lowerMatch.includes(pattern)
    );

    // Special handling for password-related patterns
    if (patternName === 'hardcoded_password') {
      // Allow if it's clearly UI/component related
      if (lowerLine.includes('input') || 
          lowerLine.includes('field') || 
          lowerLine.includes('form') ||
          lowerLine.includes('component') ||
          lowerLine.includes('type') ||
          lowerLine.includes('route') ||
          lowerLine.includes('path') ||
          lowerMatch.includes('activation') ||
          lowerMatch.includes('forgot_password') ||
          lowerMatch.includes('reset_password') ||
          lowerMatch.includes('setup_password')) {
        return true;
      }
    }

    return hasPatternFalsePositive;
  }

  calculateConfidence(line, match, patternName) {
    let confidence = 0.8; // Base confidence
    
    // Reduce confidence for potential false positives
    const lowerLine = line.toLowerCase();
    
    if (lowerLine.includes('test') || lowerLine.includes('mock') || lowerLine.includes('example')) {
      confidence -= 0.3;
    }

    if (lowerLine.includes('const') || lowerLine.includes('let') || lowerLine.includes('var')) {
      confidence += 0.1; // Variable assignments more likely to be hardcode
    }

    if (lowerLine.includes('type') || lowerLine.includes('component') || lowerLine.includes('props')) {
      confidence -= 0.2; // UI-related less likely to be sensitive
    }

    return Math.max(0.3, Math.min(1.0, confidence));
  }
}

module.exports = new C041Analyzer();
