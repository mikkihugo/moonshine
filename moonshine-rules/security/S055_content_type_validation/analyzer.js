/**
 * Heuristic analyzer for S055 - Content-Type Validation in REST Services
 * Purpose: Detect REST endpoints that process request body without validating Content-Type
 * Based on OWASP ASVS 13.2.5 - Input Validation
 */

class S055Analyzer {
  constructor() {
    this.ruleId = 'S055';
    this.ruleName = 'Content-Type Validation in REST Services';
    this.description = 'Verify that REST services explicitly check the incoming Content-Type';
    
    // HTTP methods that typically have request bodies
    this.httpMethodsWithBody = [
      'post', 'put', 'patch', 'delete'
    ];
    
    // Patterns that indicate request body usage
    this.requestBodyPatterns = [
      // Express.js patterns
      /req\.body/i,
      /request\.body/i,
      
      // NestJS patterns
      /@Body\(\)/i,
      /@Body\([^)]*\)/i,
      
      // Generic body access patterns
      /\.body\s*[;\.,\]\}]/i,
      /body\s*[:=]/i,
    ];
    
    // Patterns that indicate Content-Type validation
    this.contentTypeValidationPatterns = [
      // Express.js validation methods
      /req\.is\s*\(\s*['"`][^'"`]*application\/[^'"`]*['"`]\s*\)/i,
      /request\.is\s*\(\s*['"`][^'"`]*application\/[^'"`]*['"`]\s*\)/i,
      
      // Direct header checks
      /req\.headers\s*\[\s*['"`]content-type['"`]\s*\]/i,
      /request\.headers\s*\[\s*['"`]content-type['"`]\s*\]/i,
      /req\.get\s*\(\s*['"`]content-type['"`]\s*\)/i,
      /request\.get\s*\(\s*['"`]content-type['"`]\s*\)/i,
      
      // Content-Type comparison
      /content-type\s*[=!]==?\s*['"`]application\//i,
      /['"`]application\/[^'"`]*['"`]\s*[=!]==?\s*.*content-type/i,
      
      // Middleware patterns
      /express\.json\s*\(/i,
      /bodyParser\.json\s*\(/i,
      /app\.use\s*\([^)]*json[^)]*\)/i,
      
      // NestJS decorators
      /@Header\s*\(\s*['"`]Content-Type['"`]/i,
      /@UseInterceptors\s*\([^)]*ContentType[^)]*\)/i,
      
      // Custom validation functions
      /validateContentType/i,
      /checkContentType/i,
      /verifyContentType/i,
    ];
    
    // Patterns that indicate HTTP method handlers
    this.httpHandlerPatterns = [
      // Express.js route definitions
      /app\.(post|put|patch|delete)\s*\(/i,
      /router\.(post|put|patch|delete)\s*\(/i,
      /express\(\)\.(post|put|patch|delete)\s*\(/i,
      
      // NestJS decorators
      /@(Post|Put|Patch|Delete)\s*\(/i,
      
      // Generic handler patterns
      /(post|put|patch|delete)\s*:\s*(async\s+)?function/i,
      /(post|put|patch|delete)\s*:\s*\(/i,
      
      // Function names indicating HTTP handlers
      /function\s+(handle|process)?(Post|Put|Patch|Delete)/i,
      /const\s+\w*(post|put|patch|delete)\w*\s*=/i,
      /let\s+\w*(post|put|patch|delete)\w*\s*=/i,
    ];
    
    // Safe patterns to exclude from violations
    this.safePatterns = [
      // Comments and documentation
      /\/\/|\/\*|\*\/|@param|@return|@example/,
      
      // Import/export statements
      /import|export|require|module\.exports/i,
      
      // Type definitions
      /interface|type|enum|declare/i,
      
      // Test files patterns
      /describe\s*\(|it\s*\(|test\s*\(|expect\s*\(/i,
      
      // Configuration and constants
      /const\s+\w+\s*=\s*['"`]/i,
      
      // Logging and debugging
      /console\.|logger\.|log\(/i,
      
      // Middleware already handling Content-Type
      /express\.json|bodyParser\.json|multer\(/i,
    ];
    
    // Patterns indicating secure implementations
    this.secureImplementationPatterns = [
      // Middleware usage that handles Content-Type
      /app\.use\s*\([^)]*express\.json[^)]*\)/i,
      /app\.use\s*\([^)]*bodyParser\.json[^)]*\)/i,
      
      // Global Content-Type validation
      /app\.use\s*\([^)]*validateContentType[^)]*\)/i,
      /app\.use\s*\([^)]*checkContentType[^)]*\)/i,
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
      'vendor/', 'mocks/', '.mock.',
      // Config files
      'config/', 'configs/', '.config.',
      // Static assets
      'public/', 'static/', 'assets/',
    ];
    
    return skipPatterns.some(pattern => filePath.includes(pattern));
  }

  analyzeFile(content, filePath, options = {}) {
    const violations = [];
    const lines = content.split('\n');
    
    // First, check if file has global Content-Type validation (middleware)
    const hasGlobalValidation = this.hasGlobalContentTypeValidation(content);
    
    lines.forEach((line, index) => {
      const lineNumber = index + 1;
      const trimmedLine = line.trim();
      
      // Skip comments, imports, and empty lines
      if (this.shouldSkipLine(trimmedLine)) {
        return;
      }
      
      // Check for potential Content-Type validation violations
      const violation = this.checkForContentTypeViolation(
        line, 
        lineNumber, 
        filePath, 
        content, 
        hasGlobalValidation
      );
      if (violation) {
        violations.push(violation);
      }
    });
    
    return violations;
  }

  shouldSkipLine(line) {
    return (
      line.length === 0 ||
      this.safePatterns.some(pattern => pattern.test(line))
    );
  }

  hasGlobalContentTypeValidation(content) {
    return this.secureImplementationPatterns.some(pattern => pattern.test(content));
  }

  checkForContentTypeViolation(line, lineNumber, filePath, fullContent, hasGlobalValidation) {
    // Check if line contains request body usage
    const hasRequestBodyUsage = this.requestBodyPatterns.some(pattern => pattern.test(line));
    
    if (!hasRequestBodyUsage) {
      return null;
    }
    
    // Check if this line is part of an HTTP handler
    const isInHttpHandler = this.isInHttpHandlerContext(line, lineNumber, fullContent);
    
    if (!isInHttpHandler) {
      return null;
    }
    
    // Skip if there's global validation
    if (hasGlobalValidation) {
      return null;
    }
    
    // Check if there's local Content-Type validation in the same function/handler
    const hasLocalValidation = this.hasLocalContentTypeValidation(lineNumber, fullContent);
    
    if (hasLocalValidation) {
      return null;
    }
    
    // Check if this is a NestJS handler with proper decorators
    if (this.isSecureNestJSHandler(lineNumber, fullContent)) {
      return null;
    }
    
    return {
      ruleId: this.ruleId,
      severity: 'error',
      message: 'REST endpoint processes request body without validating Content-Type header. This can lead to security vulnerabilities.',
      line: lineNumber,
      column: this.findPatternColumn(line, this.requestBodyPatterns),
      filePath: filePath,
      type: 'missing_content_type_validation',
      details: 'Consider adding Content-Type validation using req.is("application/json") or checking req.headers["content-type"] before processing request body.'
    };
  }

  isInHttpHandlerContext(line, lineNumber, fullContent) {
    const lines = fullContent.split('\n');
    
    // Check previous lines for HTTP handler patterns
    const contextRange = Math.max(0, lineNumber - 10); // Check up to 10 lines back
    
    for (let i = contextRange; i < lineNumber; i++) {
      const contextLine = lines[i];
      if (this.httpHandlerPatterns.some(pattern => pattern.test(contextLine))) {
        return true;
      }
    }
    
    // Check current line
    if (this.httpHandlerPatterns.some(pattern => pattern.test(line))) {
      return true;
    }
    
    return false;
  }

  hasLocalContentTypeValidation(lineNumber, fullContent) {
    const lines = fullContent.split('\n');
    
    // Check surrounding lines for Content-Type validation
    const startLine = Math.max(0, lineNumber - 15);
    const endLine = Math.min(lines.length, lineNumber + 10);
    
    for (let i = startLine; i < endLine; i++) {
      const checkLine = lines[i];
      if (this.contentTypeValidationPatterns.some(pattern => pattern.test(checkLine))) {
        return true;
      }
    }
    
    return false;
  }

  isSecureNestJSHandler(lineNumber, fullContent) {
    const lines = fullContent.split('\n');
    
    // Check previous lines for NestJS decorators that handle Content-Type
    const contextRange = Math.max(0, lineNumber - 5);
    
    for (let i = contextRange; i < lineNumber; i++) {
      const line = lines[i];
      if (/@Header\s*\(\s*['"`]Content-Type['"`]/i.test(line)) {
        return true;
      }
      if (/@UseInterceptors\s*\([^)]*ContentType[^)]*\)/i.test(line)) {
        return true;
      }
    }
    
    return false;
  }

  findPatternColumn(line, patterns) {
    for (const pattern of patterns) {
      const match = pattern.exec(line);
      if (match) {
        return match.index + 1;
      }
    }
    return 1;
  }
}

module.exports = S055Analyzer;
