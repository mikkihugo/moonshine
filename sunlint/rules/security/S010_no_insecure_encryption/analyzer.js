/**
 * Heuristic analyzer for S010 - Must use cryptographically secure random number generators (CSPRNG)
 * Purpose: Detect usage of insecure random number generators for security purposes
 * Based on OWASP A02:2021 - Cryptographic Failures
 */

class S010Analyzer {
  constructor() {
    this.ruleId = 'S010';
    this.ruleName = 'Must use cryptographically secure random number generators (CSPRNG)';
    this.description = 'Detect usage of insecure random number generators for security purposes';
    
    // Insecure random functions that should not be used for security
    this.insecureRandomFunctions = [
      // JavaScript/Node.js insecure random functions
      'Math.random',
      'Math.floor(Math.random',
      'Math.ceil(Math.random',
      'Math.round(Math.random',
      
      // Common insecure patterns
      'new Date().getTime()',
      'Date.now()',
      'performance.now()',
      'process.hrtime()',
      
      // Insecure libraries
      'random-js',
      'mersenne-twister',
      'seedrandom',
      
      // Browser APIs (when used for security)
      'window.crypto.getRandomValues', // Actually secure, but context matters
    ];
    
    // Secure random functions (CSPRNG)
    this.secureRandomFunctions = [
      'crypto.randomBytes',
      'crypto.randomUUID',
      'crypto.randomInt',
      'crypto.webcrypto.getRandomValues',
      'window.crypto.getRandomValues',
      'require("crypto").randomBytes',
      'import("crypto").randomBytes',
      'webcrypto.getRandomValues',
      'sodium.randombytes_buf',
      'forge.random.getBytesSync',
      'nanoid',
      'uuid.v4',
      'uuidv4',
    ];
    
    // Security-related contexts where secure random is required
    this.securityContextKeywords = [
      // Authentication
      'password', 'token', 'jwt', 'session', 'auth', 'login', 'signin',
      'activation', 'verification', 'reset', 'recovery', 'otp', 'totp',
      
      // Cryptography
      'encrypt', 'decrypt', 'cipher', 'hash', 'salt', 'key', 'secret',
      'nonce', 'iv', 'seed', 'entropy', 'random', 'secure',
      
      // Security tokens
      'csrf', 'xsrf', 'api_key', 'access_token', 'refresh_token',
      'bearer', 'authorization', 'signature', 'certificate',
      
      // Identifiers
      'id', 'uuid', 'guid', 'code', 'pin', 'challenge',
      
      // File/data security
      'upload', 'filename', 'path', 'temp', 'cache'
    ];
    
    // Patterns that indicate insecure random usage
    this.insecurePatterns = [
      // Math.random() variations
      /Math\.random\(\)/g,
      /Math\.floor\s*\(\s*Math\.random\s*\(\s*\)\s*\*\s*\d+\s*\)/g,
      /Math\.ceil\s*\(\s*Math\.random\s*\(\s*\)\s*\*\s*\d+\s*\)/g,
      /Math\.round\s*\(\s*Math\.random\s*\(\s*\)\s*\*\s*\d+\s*\)/g,
      
      // Date-based random (only when used for randomness, not timestamps)
      /new\s+Date\(\)\.getTime\(\)/g,
      /Date\.now\(\)/g,
      /performance\.now\(\)/g,
      
      // String-based insecure random
      /Math\.random\(\)\.toString\(\d*\)\.substring\(\d+\)/g,
      /Math\.random\(\)\.toString\(\d*\)\.slice\(\d+\)/g,
      
      // Simple increment patterns (only in security contexts)
      /\+\+\s*\w+|--\s*\w+|\w+\s*\+\+|\w+\s*--/g,
    ];
    
    // Patterns that should be excluded (safe contexts)
    this.safePatterns = [
      // Comments and documentation
      /\/\/|\/\*|\*\/|@param|@return|@example/,
      
      // Type definitions and interfaces
      /interface|type|enum|class.*\{/i,
      
      // Import/export statements
      /import|export|require|module\.exports/i,
      
      // Test files and demo code
      /test|spec|demo|example|mock|fixture/i,
      
      // Non-security contexts
      /animation|ui|display|visual|game|chart|graph|color|theme/i,
      
      // Configuration and constants
      /const\s+\w+\s*=|enum\s+\w+|type\s+\w+/i,
      
      // Safe usage patterns - UI/Animation/Game contexts
      /Math\.random\(\).*(?:animation|ui|display|game|demo|test|chart|color|hue)/i,
      /(?:animation|ui|display|game|demo|test|chart|color|hue).*Math\.random\(\)/i,
      
      // Safe class/function contexts
      /class\s+(?:UI|Game|Chart|Mock|Demo|Animation)/i,
      /function\s+(?:get|generate|create).*(?:Color|Animation|Chart|Game|Mock|Demo)/i,
      
      // Safe variable names
      /(?:const|let|var)\s+(?:color|hue|delay|position|chart|game|mock|demo|animation)/i,
    ];
    
    // Function patterns that indicate security context
    this.securityFunctionPatterns = [
      /generate.*(?:token|key|id|code|password|salt|nonce|iv)/i,
      /create.*(?:token|key|id|code|password|salt|nonce|iv)/i,
      /make.*(?:token|key|id|code|password|salt|nonce|iv)/i,
      /random.*(?:token|key|id|code|password|salt|nonce|iv)/i,
      /(?:token|key|id|code|password|salt|nonce|iv).*generator/i,
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
      
      // Check for insecure random usage in security context
      const violation = this.checkForInsecureRandom(line, lineNumber, filePath, content);
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

  checkForInsecureRandom(line, lineNumber, filePath, fullContent) {
    const lowerLine = line.toLowerCase();
    
    // First check if line contains safe patterns (early exit)
    if (this.containsSafePattern(line)) {
      return null;
    }
    
    // Check for insecure random patterns
    for (const pattern of this.insecurePatterns) {
      const matches = [...line.matchAll(pattern)];
      
      for (const match of matches) {
        // Special handling for Date.now() - check if it's legitimate timestamp usage
        if (match[0].includes('Date.now()') && this.isLegitimateTimestampUsage(line)) {
          continue; // Skip this match
        }
        
        // Check if this usage is in a security context
        if (this.isInSecurityContext(line, fullContent, lineNumber)) {
          const column = match.index + 1;
          
          return {
            ruleId: this.ruleId,
            severity: 'error',
            message: 'Must use cryptographically secure random number generators (CSPRNG) for security purposes. Math.random() and similar functions are not secure.',
            line: lineNumber,
            column: column,
            filePath: filePath,
            type: 'insecure_random_usage',
            details: this.getSecureAlternatives(match[0]),
            insecureFunction: match[0]
          };
        }
      }
    }
    
    // Check for insecure function calls
    const insecureFunctionViolation = this.checkInsecureFunctionCall(line, lineNumber, filePath, fullContent);
    if (insecureFunctionViolation) {
      return insecureFunctionViolation;
    }
    
    return null;
  }

  containsSafePattern(line) {
    return this.safePatterns.some(pattern => pattern.test(line));
  }

  isInSecurityContext(line, fullContent, lineNumber) {
    const lowerLine = line.toLowerCase();
    const lowerContent = fullContent.toLowerCase();
    
    // First check if this is explicitly a non-security context
    if (this.isNonSecurityContext(line, fullContent, lineNumber)) {
      return false;
    }
    
    // Check if line contains security keywords
    const hasSecurityKeyword = this.securityContextKeywords.some(keyword => 
      lowerLine.includes(keyword)
    );
    
    if (hasSecurityKeyword) {
      return true;
    }
    
    // Check function context (look at function name)
    const functionContext = this.getFunctionContext(fullContent, lineNumber);
    if (functionContext && this.isSecurityFunction(functionContext)) {
      return true;
    }
    
    // Check variable context
    const variableContext = this.getVariableContext(line);
    if (variableContext && this.isSecurityVariable(variableContext)) {
      return true;
    }
    
    // Check surrounding lines for context
    const contextLines = this.getSurroundingLines(fullContent, lineNumber, 3);
    const contextHasSecurityKeywords = this.securityContextKeywords.some(keyword =>
      contextLines.some(contextLine => contextLine.toLowerCase().includes(keyword))
    );
    
    return contextHasSecurityKeywords;
  }

  isNonSecurityContext(line, fullContent, lineNumber) {
    const lowerLine = line.toLowerCase();
    
    // Check for UI/Game/Animation contexts
    const nonSecurityKeywords = [
      'animation', 'ui', 'display', 'visual', 'game', 'chart', 'graph', 
      'color', 'theme', 'hue', 'rgb', 'hsl', 'position', 'coordinate',
      'mock', 'demo', 'test', 'example', 'sample', 'fixture'
    ];
    
    if (nonSecurityKeywords.some(keyword => lowerLine.includes(keyword))) {
      return true;
    }
    
    // Check class context
    const classContext = this.getClassContext(fullContent, lineNumber);
    if (classContext) {
      const lowerClassName = classContext.toLowerCase();
      if (nonSecurityKeywords.some(keyword => lowerClassName.includes(keyword))) {
        return true;
      }
    }
    
    // Check function context
    const functionContext = this.getFunctionContext(fullContent, lineNumber);
    if (functionContext) {
      const lowerFunctionName = functionContext.toLowerCase();
      if (nonSecurityKeywords.some(keyword => lowerFunctionName.includes(keyword))) {
        return true;
      }
    }
    
    return false;
  }

  getClassContext(content, lineNumber) {
    const lines = content.split('\n');
    
    // Look backwards for class declaration
    for (let i = lineNumber - 1; i >= Math.max(0, lineNumber - 20); i--) {
      const line = lines[i];
      const classMatch = line.match(/class\s+(\w+)/);
      if (classMatch) {
        return classMatch[1];
      }
    }
    
    return null;
  }

  checkInsecureFunctionCall(line, lineNumber, filePath, fullContent) {
    // Look for specific insecure function patterns
    const mathRandomMatch = line.match(/(Math\.random\(\))/);
    if (mathRandomMatch && this.isInSecurityContext(line, fullContent, lineNumber)) {
      return {
        ruleId: this.ruleId,
        severity: 'error',
        message: 'Math.random() is not cryptographically secure. Use crypto.randomBytes() or crypto.randomInt() for security purposes.',
        line: lineNumber,
        column: mathRandomMatch.index + 1,
        filePath: filePath,
        type: 'math_random_insecure',
        details: 'Consider using: crypto.randomBytes(), crypto.randomInt(), crypto.randomUUID(), or nanoid() for secure random generation.',
        insecureFunction: mathRandomMatch[1]
      };
    }
    
    // Check for Date-based random, but exclude legitimate timestamp usage
    const dateRandomMatch = line.match(/(Date\.now\(\)|new\s+Date\(\)\.getTime\(\))/);
    if (dateRandomMatch && this.isInSecurityContext(line, fullContent, lineNumber)) {
      // Check if this is legitimate timestamp usage (JWT iat/exp, logging, etc.)
      if (this.isLegitimateTimestampUsage(line)) {
        return null;
      }
      
      return {
        ruleId: this.ruleId,
        severity: 'warning',
        message: 'Using timestamp for random generation is predictable and insecure.',
        line: lineNumber,
        column: dateRandomMatch.index + 1,
        filePath: filePath,
        type: 'timestamp_random_insecure',
        details: 'Consider using crypto.randomBytes() or crypto.randomUUID() for secure random generation.',
        insecureFunction: dateRandomMatch[1]
      };
    }
    
    return null;
  }

  isLegitimateTimestampUsage(line) {
    // Check for legitimate timestamp usage patterns
    const legitimatePatterns = [
      // JWT timestamp fields - more flexible matching
      /\b(?:iat|exp|nbf)\s*:\s*Math\.floor\s*\(\s*Date\.now\(\)\s*\/\s*1000\s*\)/,
      /Math\.floor\s*\(\s*Date\.now\(\)\s*\/\s*1000\s*\).*(?:iat|exp|nbf)/i,
      
      // JWT timestamp with arithmetic
      /Math\.floor\s*\(\s*Date\.now\(\)\s*\/\s*1000\s*\)\s*[\+\-]\s*\d+/,
      
      // Logging timestamps
      /timestamp\s*:\s*Date\.now\(\)/i,
      /createdAt\s*:\s*new\s+Date\(\)/i,
      /updatedAt\s*:\s*new\s+Date\(\)/i,
      
      // Expiration times
      /expiresAt\s*:\s*new\s+Date\s*\(\s*Date\.now\(\)/i,
      /expiry\s*:\s*Date\.now\(\)/i,
      
      // Performance measurement
      /performance\.now\(\)/,
      
      // Date arithmetic (not for randomness)
      /Date\.now\(\)\s*[\+\-]\s*\d+/,
      /new\s+Date\s*\(\s*Date\.now\(\)\s*[\+\-]/,
      
      // JWT context - check for jwt, token, payload keywords nearby
      /(?:jwt|token|payload).*Date\.now\(\)/i,
      /Date\.now\(\).*(?:jwt|token|payload)/i,
    ];
    
    return legitimatePatterns.some(pattern => pattern.test(line));
  }

  getFunctionContext(content, lineNumber) {
    const lines = content.split('\n');
    
    // Look backwards for function declaration
    for (let i = lineNumber - 1; i >= Math.max(0, lineNumber - 10); i--) {
      const line = lines[i];
      const functionMatch = line.match(/(?:function\s+(\w+)|(?:const|let|var)\s+(\w+)\s*=\s*(?:async\s+)?\w*\s*(?:function\s*)?|\s*(\w+)\s*[:=]\s*(?:async\s+)?(?:function|\w*\s*=>))/);
      if (functionMatch) {
        return functionMatch[1] || functionMatch[2] || functionMatch[3];
      }
    }
    
    return null;
  }

  isSecurityFunction(functionName) {
    if (!functionName) return false;
    
    const lowerFunctionName = functionName.toLowerCase();
    return this.securityFunctionPatterns.some(pattern => pattern.test(lowerFunctionName)) ||
           this.securityContextKeywords.some(keyword => lowerFunctionName.includes(keyword));
  }

  getVariableContext(line) {
    // Extract variable name from assignment
    const assignmentMatch = line.match(/(?:const|let|var)\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*=/);
    if (assignmentMatch) {
      return assignmentMatch[1];
    }
    
    // Extract property assignment
    const propertyMatch = line.match(/(\w+)\s*[:=]/);
    if (propertyMatch) {
      return propertyMatch[1];
    }
    
    return null;
  }

  isSecurityVariable(variableName) {
    if (!variableName) return false;
    
    const lowerVariableName = variableName.toLowerCase();
    return this.securityContextKeywords.some(keyword => lowerVariableName.includes(keyword));
  }

  getSurroundingLines(content, lineNumber, range) {
    const lines = content.split('\n');
    const start = Math.max(0, lineNumber - range - 1);
    const end = Math.min(lines.length, lineNumber + range);
    
    return lines.slice(start, end);
  }

  getSecureAlternatives(insecureFunction) {
    const alternatives = {
      'Math.random()': 'crypto.randomBytes(), crypto.randomInt(), or crypto.randomUUID()',
      'Date.now()': 'crypto.randomBytes() or crypto.randomUUID()',
      'new Date().getTime()': 'crypto.randomBytes() or crypto.randomUUID()',
      'performance.now()': 'crypto.randomBytes() or crypto.randomUUID()'
    };
    
    return alternatives[insecureFunction] || 'Use crypto.randomBytes(), crypto.randomInt(), or crypto.randomUUID() for secure random generation.';
  }

  findPatternColumn(line, pattern) {
    const match = pattern.exec(line);
    return match ? match.index + 1 : 1;
  }
}

module.exports = S010Analyzer;
