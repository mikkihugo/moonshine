const fs = require('fs');
const path = require('path');

class C017Analyzer {
  constructor() {
    this.ruleId = 'C017';
    this.ruleName = 'Constructor Logic Limitation';
    this.description = 'Không gán logic xử lý vào constructor';
  }

  async analyze(files, language, options = {}) {
    const violations = [];
    
    for (const filePath of files) {
      if (options.verbose) {
        console.log(`🔍 Running C017 analysis on ${path.basename(filePath)}`);
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
    
    // Find constructor blocks and analyze their content
    lines.forEach((line, index) => {
      const lineNumber = index + 1;
      const trimmedLine = line.trim();

      // Detect constructor start
      if (this.isConstructorStart(trimmedLine)) {
        const constructorInfo = this.extractConstructorInfo(lines, index);
        
        // Debug logging to understand boundary detection
        if (config?.verbose) {
          console.log(`[DEBUG] Constructor found at line ${lineNumber}`);
          console.log(`[DEBUG] Constructor body lines: ${constructorInfo.body.length}`);
          console.log(`[DEBUG] Constructor content:`, constructorInfo.body.map((l, i) => `${lineNumber + i}: ${l}`));
        }
        
        const complexLogic = this.findComplexLogic(constructorInfo.body);
        
        complexLogic.forEach(logic => {
          violations.push({
            ruleId: this.ruleId,
            file: filePath,
            line: lineNumber + logic.lineOffset,
            column: logic.column,
            message: `Constructor contains complex logic: ${logic.description}. Move to initialization methods`,
            severity: 'warning',
            code: logic.code,
            type: logic.type,
            confidence: logic.confidence,
            suggestion: 'Move complex logic to separate initialization methods or lifecycle hooks'
          });
        });
      }
    });

    return violations;
  }

  isConstructorStart(line) {
    return line.includes('constructor(') || line.match(/constructor\s*\(/);
  }

  extractConstructorInfo(lines, startIndex) {
    const constructorLines = [];
    let braceDepth = 0;
    let foundConstructorBrace = false;
    let inConstructor = false;
    let parenthesesDepth = 0;
    let inCallback = false;

    for (let i = startIndex; i < lines.length; i++) {
      const line = lines[i];
      const trimmedLine = line.trim();
      
      // Check if this is the constructor line with opening brace
      if (this.isConstructorStart(trimmedLine)) {
        inConstructor = true;
        constructorLines.push(line);
        
        // Special case: empty constructor () {}
        if (trimmedLine.includes(') {}')) {
          foundConstructorBrace = true;
          braceDepth = 0; // Already closed
          break; // Stop immediately for empty constructor
        }
        
        // Count braces and parentheses in the constructor line itself
        for (const char of line) {
          if (char === '{') {
            foundConstructorBrace = true;
            braceDepth = 1; // Start counting from 1 for the constructor block
          } else if (char === '(') {
            parenthesesDepth++;
          } else if (char === ')') {
            parenthesesDepth--;
          }
        }
      } else if (inConstructor && foundConstructorBrace) {
        constructorLines.push(line);
        
        // Track callback functions and nested structures
        for (const char of line) {
          if (char === '(') {
            parenthesesDepth++;
            // Detect callback patterns: .use(, .then(, .catch(, etc.
            if (line.includes('.use(') || line.includes('.then(') || 
                line.includes('.catch(') || line.includes('.finally(') ||
                line.includes('=>')) {
              inCallback = true;
            }
          } else if (char === ')') {
            parenthesesDepth--;
            if (parenthesesDepth <= 0) {
              inCallback = false;
            }
          } else if (char === '{') {
            braceDepth++;
          } else if (char === '}') {
            braceDepth--;
          }
        }
        
        // If we've closed all braces and not in callback, we're done
        if (braceDepth === 0 && !inCallback) {
          break;
        }
      } else if (inConstructor) {
        // Haven't found the opening brace yet, keep looking
        constructorLines.push(line);
        
        // Check for empty constructor pattern: ) {}
        if (trimmedLine.includes(') {}')) {
          foundConstructorBrace = true;
          braceDepth = 0; // Already closed
          break; // Stop immediately
        }
        
        for (const char of line) {
          if (char === '{') {
            foundConstructorBrace = true;
            braceDepth = 1;
            break;
          }
        }
      }
        }
        
        for (const char of line) {
          if (char === '{') {
            foundConstructorBrace = true;
            braceDepth = 1;
            break;
          }
        }
      }
    }

    return {
      body: constructorLines,
      startLine: startIndex
    };
  }

  findComplexLogic(constructorBody) {
    const complexLogic = [];
    let inCallbackFunction = false;
    let callbackDepth = 0;
    
    // Check if this is an empty constructor first
    // TEMPORARILY DISABLED for debugging
    // if (this.isEmptyConstructor(constructorBody)) {
    //   return complexLogic; // Return empty array - no violations for empty constructors
    // }
    
    constructorBody.forEach((line, index) => {
      const trimmedLine = line.trim();
      
      // Skip comments, empty lines, and constructor declaration
      if (!trimmedLine || 
          trimmedLine.startsWith('//') || 
          trimmedLine.startsWith('/*') ||
          trimmedLine.startsWith('*') ||
          this.isConstructorStart(trimmedLine) ||
          trimmedLine === '{' ||
          trimmedLine === '}') {
        return;
      }

      // Track callback functions to avoid flagging their content
      if (this.isCallbackStart(trimmedLine)) {
        inCallbackFunction = true;
        callbackDepth = 1;
        return;
      }

      if (inCallbackFunction) {
        // Count braces to track callback boundaries
        const openBraces = (trimmedLine.match(/\{/g) || []).length;
        const closeBraces = (trimmedLine.match(/\}/g) || []).length;
        callbackDepth += openBraces - closeBraces;
        
        if (callbackDepth <= 0) {
          inCallbackFunction = false;
          callbackDepth = 0;
        }
        return; // Skip analysis inside callbacks
      }

      // Analyze line for complex logic patterns only if not in callback
      const logicType = this.analyzeLineComplexity(trimmedLine);
      
      if (logicType) {
        complexLogic.push({
          lineOffset: index,
          column: line.indexOf(trimmedLine) + 1,
          code: trimmedLine,
          type: logicType.type,
          description: logicType.description,
          confidence: logicType.confidence
        });
      }
    });

    return complexLogic;
  }

  isEmptyConstructor(constructorBody) {
    // Check if constructor body contains only dependency injection parameters and closing brace
    const meaningfulLines = constructorBody.filter(line => {
      const trimmed = line.trim();
      
      // Skip empty lines, comments, and constructor declaration
      if (!trimmed || 
          trimmed.startsWith('//') || 
          trimmed.startsWith('/*') || 
          trimmed.startsWith('*') ||
          this.isConstructorStart(trimmed) ||
          trimmed === '{' || 
          trimmed === '}' ||
          trimmed === ') {}') {
        return false;
      }
      
      // Skip constructor parameter declarations (NestJS style)
      if (trimmed.startsWith('@') || // Decorators like @InjectRepository
          trimmed.match(/^\s*(private|public|protected)\s+readonly\s+\w+/) || // DI parameters
          trimmed.match(/^\s*(private|public|protected)\s+\w+/) || // Other parameters
          trimmed.endsWith(',') || // Parameter continuation
          trimmed.endsWith(') {')) { // Constructor parameter closing
        return false;
      }
      
      // This is a meaningful line (logic, assignments, calls, etc.)
      return true;
    });
    
    return meaningfulLines.length === 0;
  }

  isCallbackStart(line) {
    // Patterns that indicate callback function starts
    const callbackPatterns = [
      /\.(use|then|catch|finally|map|filter|forEach|reduce)\s*\(/,
      /=>\s*\{/,
      /function\s*\(/,
      /\(\s*\w+\s*\)\s*=>/,
      /interceptors\.(request|response)\.use\(/,
      /\.addEventListener\(/,
      /setTimeout\(/,
      /setInterval\(/
    ];

    return callbackPatterns.some(pattern => pattern.test(line));
  }

  analyzeLineComplexity(line) {
    // Simple property assignments are OK
    if (this.isSimpleAssignment(line)) {
      return null;
    }

    // Super calls are OK
    if (this.isSuperCall(line)) {
      return null;
    }

    // Parameter assignments are OK
    if (this.isParameterAssignment(line)) {
      return null;
    }

    // MobX decorators/observables setup are OK
    if (this.isMobXSetup(line)) {
      return null;
    }

    // Allowed method calls are OK
    if (this.isAllowedMethodCall(line)) {
      return null;
    }

    // Configuration setup is OK
    if (this.isConfigurationSetup(line)) {
      return null;
    }

    // Interceptor setup is OK (axios, etc.)
    if (this.isInterceptorSetup(line)) {
      return null;
    }

    // Detect complex logic patterns
    const complexPatterns = [
      {
        pattern: /\bif\s*\(|\belse\s|\bswitch\s*\(/,
        type: 'conditional_logic',
        description: 'conditional statements (if/else/switch)',
        confidence: 0.9
      },
      {
        pattern: /\bfor\s*\(|\bwhile\s*\(|\bdo\s+/,
        type: 'loop_logic',
        description: 'loops (for/while)',
        confidence: 0.95
      },
      {
        pattern: /\btry\s*{|\bcatch\s*\(|\bfinally\s*{/,
        type: 'exception_handling',
        description: 'exception handling (try/catch/finally)',
        confidence: 0.8
      },
      {
        pattern: /\.then\s*\(|\.catch\s*\(|await\s+/,
        type: 'async_logic',
        description: 'asynchronous operations',
        confidence: 0.9
      },
      {
        pattern: /\w+\s*\(\s*[^)]*\)\s*[;{]/,
        type: 'method_call',
        description: 'complex method calls',
        confidence: 0.4 // Reduced confidence to minimize false positives
      },
      {
        pattern: /new\s+\w+\s*\([^)]*\)\s*\./,
        type: 'chained_instantiation',
        description: 'chained object instantiation',
        confidence: 0.8
      }
    ];

    for (const { pattern, type, description, confidence } of complexPatterns) {
      if (pattern.test(line)) {
        return { type, description, confidence };
      }
    }

    // Check for complex expressions
    if (this.hasComplexExpression(line)) {
      return {
        type: 'complex_expression',
        description: 'complex expression or calculation',
        confidence: 0.6
      };
    }

    return null;
  }

  isSimpleAssignment(line) {
    // Patterns for simple assignments - expanded to catch more legitimate patterns
    const simplePatterns = [
      /^this\.\w+\s*=\s*[^;]+;?$/,          // this.property = value;
      /^this\.\w+\s*=\s*\w+;?$/,            // this.property = parameter;
      /^this\.\w+\s*=\s*(null|undefined|true|false|\d+|'[^']*'|"[^"]*");?$/, // this.property = literal;
      /^this\.\w+\s*=\s*this\.\w+\s*\.\s*get\s*\(/,  // this.property = this.configService.get(
      /^this\.\w+\s*=\s*new\s+\w+\s*\(/,             // this.property = new SomeClass(
      /^this\.\w+\s*=\s*\w+\s*\.\s*\w+\s*\(/,       // this.property = service.method(
      // Enhanced patterns for configuration initialization
      /^this\.\w+\s*=\s*new\s+\w+Client\s*\(/,       // this.s3 = new S3Client(
      /^this\.\w+\s*=\s*new\s+\w+\s*\(\s*\{/,        // this.prop = new Class({ config })
      /^this\.\w+\s*=\s*\w+\s*\.\s*create\s*\(/,     // this.prop = Factory.create(
      /^this\.\w+\s*=\s*\w+\s*\.\s*getInstance\s*\(/, // this.prop = Service.getInstance(
    ];

    return simplePatterns.some(pattern => pattern.test(line));
  }

  isConfigurationSetup(line) {
    // Check if this is configuration/options object setup
    const configPatterns = [
      /^\s*\w+:\s*/, // Property definition in object literal
      /level:\s*/, // Log level setting
      /timestamp:\s*/, // Timestamp configuration
      /formatters:\s*/, // Formatter configuration
      /mixin:\s*/, // Mixin configuration
      /transport:\s*/, // Transport configuration
      /options:\s*/, // Options configuration
      /target:\s*/, // Target configuration
      /baseURL:\s*/, // Axios baseURL
      /timeout:\s*/, // Axios timeout
      /region:\s*/, // AWS region configuration
      /credentials:\s*/, // AWS credentials
      /endpoint:\s*/, // API endpoint
      /apiVersion:\s*/, // API version
      /maxRetries:\s*/, // Retry configuration  
      /headers:\s*/, // HTTP headers
      /defaultHeaders:\s*/, // Default headers
      /interceptors:\s*/, // Request/response interceptors
      /transformRequest:\s*/, // Request transformation
      /transformResponse:\s*/, // Response transformation
      /validateStatus:\s*/, // Status validation
      /responseType:\s*/, // Response type
      /maxContentLength:\s*/, // Content length limit
      /ssl:\s*/, // SSL configuration
      /auth:\s*/, // Authentication
      /withCredentials:\s*/, // CORS credentials
      /maxRedirects:\s*/, // Max redirects
    ];

    return configPatterns.some(pattern => pattern.test(line));
  }

  isInterceptorSetup(line) {
    // Check if this is axios interceptor setup (should be allowed in constructor)
    const interceptorPatterns = [
      /interceptors\.(request|response)\.use/,
      /\.use\s*\(/,
      /\.then\s*\(/,
      /\.catch\s*\(/,
      /\.finally\s*\(/,
      /=>\s*\{?/,
      /=>\s*[^{]/, // Arrow function without braces
      /function\s*\(/
    ];

    return interceptorPatterns.some(pattern => pattern.test(line));
  }

  isSuperCall(line) {
    return line.includes('super(') || line.startsWith('super.');
  }

  isParameterAssignment(line) {
    // Check if it's just assigning constructor parameters to properties
    return /^this\.\w+\s*=\s*\w+;?$/.test(line);
  }

  isMobXSetup(line) {
    // MobX patterns that are OK in constructor
    const mobxPatterns = [
      'makeObservable',
      'makeAutoObservable',
      'observable',
      'action',
      'computed',
      'reaction',
      'autorun',
      '@observable',
      '@action',
      '@computed'
    ];

    const lowerLine = line.toLowerCase();
    return mobxPatterns.some(pattern => 
      lowerLine.includes(pattern.toLowerCase())
    );
  }

  isAllowedMethodCall(line) {
    // Method calls that are typically OK in constructors - expanded list
    const allowedMethods = [
      'makeObservable',
      'makeAutoObservable',
      'bind',
      'addEventListener',
      'removeEventListener',
      'Object.assign',
      'Object.defineProperty',
      'console.log',
      'console.warn',
      // Common initialization patterns
      'createLogger',
      'initializeLogger',
      'setupConfiguration',
      'initializeService',
      'configure',
      'init',
      'setup',
      // Common service/config patterns
      'get',
      'getService',
      'getInstance',
      // Dependency injection patterns
      'inject',
      'resolve',
      // Axios/HTTP setup patterns
      'interceptors',
      'use',
      'defaults',
      'timeout',
      'baseURL',
      'headers'
    ];

    const lowerLine = line.toLowerCase();
    return allowedMethods.some(method => 
      lowerLine.includes(method.toLowerCase())
    );
  }

  hasComplexExpression(line) {
    // Check for complex expressions (multiple operators, complex calculations)
    // Made more restrictive to reduce false positives
    const complexityIndicators = [
      /[+\-*/]\s*[+\-*/]/,    // Multiple arithmetic operators
      /\?\s*.*\s*:/,          // Ternary operators
      /&&.*&&|\|\|.*\|\|/,    // Multiple logical operators (more restrictive)
      /\[[^\]]*\].*\[[^\]]*\]/, // Multiple array accesses
      /\.[^.]*\.[^.]*\./      // Triple+ chained property access (more restrictive)
    ];

    return complexityIndicators.some(pattern => pattern.test(line));
  }
}

module.exports = new C017Analyzer();
