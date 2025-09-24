/**
 * Heuristic analyzer for S006 - No Plaintext Recovery/Activation Codes
 * Purpose: Detect sending recovery codes, activation codes, or reset codes in plaintext
 * Based on OWASP A02:2021 - Cryptographic Failures
 */

class S006Analyzer {
  constructor() {
    this.ruleId = 'S006';
    this.ruleName = 'No Plaintext Recovery/Activation Codes';
    this.description = 'Do not send recovery or activation codes in plaintext';
    
    // Keywords that indicate sensitive codes
    this.sensitiveCodeKeywords = [
      'recovery', 'activation', 'reset', 'verification', 'confirm', 'verify',
      'otp', 'totp', 'code', 'pin', 'token', 'secret', 'key', 'password'
    ];
    
    // Keywords that indicate code sending/transmission
    this.sendingKeywords = [
      'send', 'email', 'sms', 'text', 'message', 'mail', 'push', 'notify',
      'transmit', 'deliver', 'dispatch', 'forward', 'post', 'put', 'create',
      'response', 'body', 'content', 'payload', 'data'
    ];
    
    // Patterns that indicate plaintext transmission
    this.plaintextPatterns = [
      // Email/SMS sending with codes
      /(?:send|email|sms|text|message).*(?:recovery|activation|reset|verification|otp|code|pin)/i,
      /(?:recovery|activation|reset|verification|otp|code|pin).*(?:send|email|sms|text|message)/i,
      
      // HTTP responses with codes in body
      /(?:response|body|json|data|payload).*(?:recovery|activation|reset|verification|otp|code|pin)/i,
      /(?:recovery|activation|reset|verification|otp|code|pin).*(?:response|body|json|data|payload)/i,
      
      // Direct code exposure in strings
      /".*(?:recovery|activation|reset|verification|otp|code|pin).*"/i,
      /'.*(?:recovery|activation|reset|verification|otp|code|pin).*'/i,
      /`.*(?:recovery|activation|reset|verification|otp|code|pin).*`/i,
      
      // Template strings with codes
      /\$\{.*(?:recovery|activation|reset|verification|otp|code|pin).*\}/i,
      
      // API endpoint responses
      /return.*(?:recovery|activation|reset|verification|otp|code|pin)/i,
      /res\.(?:send|json|end).*(?:recovery|activation|reset|verification|otp|code|pin)/i,
    ];
    
    // Patterns that should be excluded (safe practices)
    this.safePatterns = [
      // Hashed or encrypted codes
      /hash|encrypt|cipher|bcrypt|crypto|secure/i,
      
      // Environment variables or config
      /process\.env|config\.|getenv/i,
      
      // Database storage (not transmission)
      /save|store|insert|update|database|db\./i,
      
      // Logging patterns (depends on context but often acceptable for debugging)
      /log|debug|trace|console/i,
      
      // Comments and documentation
      /\/\/|\/\*|\*\/|@param|@return|@example/,
      
      // Type definitions and interfaces
      /interface|type|enum|class.*\{/i,
      
      // Import/export statements
      /import|export|require|module\.exports/i,
      
      // Safe message patterns (no actual codes exposed)
      /instructions sent|sent to|check your|please enter|has been sent|successfully sent|we've sent|click the link|enter the code|will expire/i,
      
      // Configuration and constants
      /const\s+\w+\s*=|enum\s+\w+|type\s+\w+/i,
      
      // Function definitions
      /function\s+\w+|async\s+\w+|\w+\s*\(/i,
      
      // Return statements with safe messages
      /return\s*\{[^}]*success[^}]*\}/i,
    ];
    
    // Common safe variable names that might contain keywords
    this.safeVariableNames = [
      /^(is|has|can|should|will|enable|disable|show|hide|display).*code/i,
      /^.*type$/i,
      /^.*config$/i,
      /^.*setting$/i,
      /^.*option$/i,
      /^.*flag$/i,
    ];
  }

  async analyze(files, language, options = {}) {
    const violations = [];
    
    for (const filePath of files) {
      // Skip test files, build directories, and node_modules
      if (this.shouldSkipFile(filePath)) {
        continue;
      }
      
      try {
        const content = require('fs').readFileSync(filePath, 'utf8');
        const fileViolations = this.analyzeFile(content, filePath, options);
        violations.push(...fileViolations);
      } catch (error) {
        if (options.verbose) {
          console.warn(`⚠️ Failed to analyze ${filePath}: ${error.message}`);
        }
      }
    }
    
    return violations;
  }

  shouldSkipFile(filePath) {
    const skipPatterns = [
      'test/', 'tests/', '__tests__/', '.test.', '.spec.',
      'node_modules/', 'build/', 'dist/', '.next/', 'coverage/',
      'vendor/', 'mocks/', '.mock.'
      // Removed 'fixtures/' to allow testing
    ];
    
    return skipPatterns.some(pattern => filePath.includes(pattern));
  }

  analyzeFile(content, filePath, options = {}) {
    const violations = [];
    const lines = content.split('\n');
    
    lines.forEach((line, index) => {
      const lineNumber = index + 1;
      const trimmedLine = line.trim();
      
      // Skip comments, imports, and empty lines
      if (this.shouldSkipLine(trimmedLine)) {
        return;
      }
      
      // Check for potential plaintext code transmission
      const violation = this.checkForPlaintextCodeTransmission(line, lineNumber, filePath);
      if (violation) {
        violations.push(violation);
      }
    });
    
    return violations;
  }

  shouldSkipLine(line) {
    // Skip comments, imports, and other non-code lines
    return (
      line.length === 0 ||
      line.startsWith('//') ||
      line.startsWith('/*') ||
      line.startsWith('*') ||
      line.startsWith('import ') ||
      line.startsWith('export ') ||
      line.startsWith('require(') ||
      line.includes('module.exports')
    );
  }

  checkForPlaintextCodeTransmission(line, lineNumber, filePath) {
    const lowerLine = line.toLowerCase();
    
    // First check if line contains safe patterns (early exit)
    if (this.containsSafePattern(line)) {
      return null;
    }
    
    // Check for variable assignments with sensitive names
    const sensitiveAssignment = this.checkSensitiveAssignment(line, lineNumber, filePath);
    if (sensitiveAssignment) {
      return sensitiveAssignment;
    }
    
    // Check for direct plaintext patterns
    for (const pattern of this.plaintextPatterns) {
      if (pattern.test(line)) {
        // Additional context check to reduce false positives
        if (this.hasTransmissionContext(line)) {
          return {
            ruleId: this.ruleId,
            severity: 'error',
            message: 'Recovery/activation codes should not be transmitted in plaintext. Use encrypted channels or hash the codes.',
            line: lineNumber,
            column: this.findPatternColumn(line, pattern),
            filePath: filePath,
            type: 'plaintext_code_transmission',
            details: 'Consider using encrypted communication or sending only hashed/masked versions of sensitive codes.'
          };
        }
      }
    }
    
    return null;
  }

  containsSafePattern(line) {
    return this.safePatterns.some(pattern => pattern.test(line));
  }

  checkSensitiveAssignment(line, lineNumber, filePath) {
    // Look for variable assignments that combine sensitive codes with transmission
    const assignmentMatch = line.match(/(?:const|let|var)\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*=\s*(.+)/);
    if (!assignmentMatch) {
      return null;
    }
    
    const [, variableName, valueExpr] = assignmentMatch;
    const lowerVarName = variableName.toLowerCase();
    const lowerValueExpr = valueExpr.toLowerCase();
    
    // Skip safe variable names
    if (this.safeVariableNames.some(pattern => pattern.test(lowerVarName))) {
      return null;
    }
    
    // Check if variable name suggests code transmission
    const hasSensitiveCodeKeyword = this.sensitiveCodeKeywords.some(keyword => 
      lowerVarName.includes(keyword)
    );
    
    const hasSendingKeyword = this.sendingKeywords.some(keyword => 
      lowerVarName.includes(keyword) || lowerValueExpr.includes(keyword)
    );
    
    if (hasSensitiveCodeKeyword && hasSendingKeyword) {
      // Check if the value looks like it contains actual codes or sensitive data
      if (this.valueContainsCodes(valueExpr)) {
        return {
          ruleId: this.ruleId,
          severity: 'warning',
          message: `Variable '${variableName}' appears to handle sensitive codes for transmission. Ensure codes are encrypted or hashed.`,
          line: lineNumber,
          column: line.indexOf(variableName) + 1,
          filePath: filePath,
          type: 'sensitive_code_variable',
          variableName: variableName,
          details: 'Consider encrypting sensitive codes before transmission or use secure communication channels.'
        };
      }
    }
    
    return null;
  }

  hasTransmissionContext(line) {
    const transmissionIndicators = [
      // HTTP response methods
      /res\.(?:send|json|status|end)/i,
      /response\.(?:send|json|status|end)/i,
      /return.*(?:json|response|status)/i,
      
      // Email/SMS functions
      /(?:sendEmail|sendSMS|sendMessage|notify|mail)/i,
      
      // Template rendering
      /render|template|view|html|email/i,
      
      // API responses
      /\.json\(|\.send\(|\.end\(/,
      
      // String concatenation or template literals with codes
      /\+.*['"`]|['"`].*\+|\$\{.*\}/,
    ];
    
    return transmissionIndicators.some(indicator => indicator.test(line));
  }

  valueContainsCodes(valueExpr) {
    // Check if the value expression contains actual code patterns or user data
    const codePatterns = [
      // Template strings with variables
      /\$\{[^}]+\}/,
      
      // String concatenation
      /\+\s*[a-zA-Z_$]/,
      
      // Function calls that might return codes
      /\w+\([^)]*\)/,
      
      // Property access that might be codes
      /\w+\.\w+/,
      
      // Array/object access
      /\[.*\]/,
      
      // Direct string literals that look like codes (6+ chars with mixed case/numbers)
      /['"`][a-zA-Z0-9]{6,}['"`]/,
    ];
    
    return codePatterns.some(pattern => pattern.test(valueExpr));
  }

  findPatternColumn(line, pattern) {
    const match = pattern.exec(line);
    return match ? match.index + 1 : 1;
  }
}

module.exports = S006Analyzer;
