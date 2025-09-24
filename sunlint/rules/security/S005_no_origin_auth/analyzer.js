/**
 * Hybrid analyzer for S005 - No Origin Header Authentication
 * Uses AST analysis with regex fallback for comprehensive coverage
 * Detects usage of Origin header for authentication/access control
 */

const S005ASTAnalyzer = require('./ast-analyzer');

class S005Analyzer {
  constructor() {
    this.ruleId = 'S005';
    this.ruleName = 'No Origin Header Authentication';
    this.description = 'Do not use Origin header for authentication or access control';
    this.astAnalyzer = new S005ASTAnalyzer();
  }

  async analyze(files, language, options = {}) {
    const violations = [];
    
    if (options.verbose) {
      console.log(`🔍 Running S005 analysis on ${files.length} files...`);
    }

    // Use AST analysis as primary method
    const astViolations = await this.astAnalyzer.analyze(files, language, options);
    violations.push(...astViolations);

    // Add regex-based patterns for edge cases AST might miss
    for (const filePath of files) {
      try {
        const content = require('fs').readFileSync(filePath, 'utf8');
        const regexViolations = this.analyzeWithRegexPatterns(content, filePath, options);
        
        // Filter out duplicates (same line, same type)
        const filteredRegexViolations = regexViolations.filter(regexViolation => 
          !astViolations.some(astViolation => 
            astViolation.line === regexViolation.line && 
            astViolation.filePath === regexViolation.filePath
          )
        );
        
        violations.push(...filteredRegexViolations);
      } catch (error) {
        if (options.verbose) {
          console.warn(`⚠️ S005 regex analysis failed for ${filePath}: ${error.message}`);
        }
      }
    }

    if (options.verbose && violations.length > 0) {
      console.log(`📊 S005 found ${violations.length} violations`);
    }

    return violations;
  }

  analyzeWithRegexPatterns(content, filePath, options = {}) {
    const violations = [];
    const lines = content.split('\n');

    lines.forEach((line, index) => {
      const lineNumber = index + 1;
      
      // Pattern 1: Direct origin header access for authentication
      // req.headers.origin, req.get('origin'), req.header('origin')
      const originHeaderPattern = /(?:req\.headers\.origin|req\.get\s*\(\s*['"`]origin['"`]\s*\)|req\.header\s*\(\s*['"`]origin['"`]\s*\)|headers\[['"`]origin['"`]\])/i;
      if (originHeaderPattern.test(line)) {
        // Check if this line is used for authentication/authorization
        const authContextPattern = /(?:auth|login|verify|check|validate|permission|access|allow|deny|secure|token|session)/i;
        if (authContextPattern.test(line) || this.isInAuthContext(lines, index)) {
          violations.push({
            ruleId: this.ruleId,
            severity: 'error',
            message: 'Origin header should not be used for authentication. Origin can be spoofed and is not secure for access control.',
            line: lineNumber,
            column: line.search(originHeaderPattern) + 1,
            filePath: filePath,
            type: 'origin_header_auth'
          });
        }
      }

      // Pattern 2: Origin-based CORS validation for authentication
      const corsOriginAuthPattern = /(?:cors|origin).*(?:auth|login|permission|access|allow|token)/i;
      if (corsOriginAuthPattern.test(line) && !line.includes('//') && !line.includes('*')) {
        violations.push({
          ruleId: this.ruleId,
          severity: 'warning',
          message: 'CORS origin validation should not replace proper authentication mechanisms.',
          line: lineNumber,
          column: line.search(corsOriginAuthPattern) + 1,
          filePath: filePath,
          type: 'cors_origin_auth'
        });
      }

      // Pattern 3: Origin header in conditional authentication logic
      const conditionalAuthPattern = /if\s*\([^)]*origin[^)]*\)\s*\{[^}]*(?:auth|login|token|permission|access)/i;
      if (conditionalAuthPattern.test(line)) {
        violations.push({
          ruleId: this.ruleId,
          severity: 'error',
          message: 'Conditional authentication based on Origin header is insecure. Use proper authentication tokens.',
          line: lineNumber,
          column: line.search(/origin/i) + 1,
          filePath: filePath,
          type: 'conditional_origin_auth'
        });
      }

      // Pattern 4: Origin in middleware authentication
      const middlewarePattern = /(middleware|auth|guard).*origin.*(?:next\(\)|return|allow|permit)/i;
      if (middlewarePattern.test(line)) {
        violations.push({
          ruleId: this.ruleId,
          severity: 'error',
          message: 'Authentication middleware should not rely on Origin header. Use proper authentication mechanisms.',
          line: lineNumber,
          column: line.search(/origin/i) + 1,
          filePath: filePath,
          type: 'middleware_origin_auth'
        });
      }

      // Pattern 5: Origin header whitelisting for access control
      const whitelistPattern = /(?:whitelist|allowlist|allowed.*origins?).*(?:auth|access|permission)/i;
      if (whitelistPattern.test(line) && /origin/i.test(line)) {
        violations.push({
          ruleId: this.ruleId,
          severity: 'warning',
          message: 'Origin whitelisting should complement, not replace, proper authentication and authorization.',
          line: lineNumber,
          column: line.search(/origin/i) + 1,
          filePath: filePath,
          type: 'origin_whitelist_auth'
        });
      }

      // Pattern 6: Express.js specific patterns
      const expressPattern = /(?:app\.use|router\.).*origin.*(?:auth|protect|secure)/i;
      if (expressPattern.test(line)) {
        violations.push({
          ruleId: this.ruleId,
          severity: 'error',
          message: 'Express routes should not use Origin header for authentication or authorization.',
          line: lineNumber,
          column: line.search(/origin/i) + 1,
          filePath: filePath,
          type: 'express_origin_auth'
        });
      }
    });

    return violations;
  }

  /**
   * Check if the current line is within an authentication context
   * by looking at surrounding lines
   */
  isInAuthContext(lines, currentIndex) {
    const contextRange = 3; // Check 3 lines before and after
    const startIndex = Math.max(0, currentIndex - contextRange);
    const endIndex = Math.min(lines.length - 1, currentIndex + contextRange);
    
    const authKeywords = [
      'authenticate', 'authorize', 'login', 'logout', 'auth',
      'permission', 'access', 'token', 'session', 'user',
      'verify', 'validate', 'check', 'guard', 'protect',
      'middleware', 'passport', 'jwt', 'bearer'
    ];
    
    for (let i = startIndex; i <= endIndex; i++) {
      const line = lines[i].toLowerCase();
      if (authKeywords.some(keyword => line.includes(keyword))) {
        return true;
      }
    }
    
    return false;
  }
}

module.exports = S005Analyzer;
