// rules/common/C067_no_hardcoded_config/symbol-based-analyzer.js
const { SyntaxKind, Project, Node } = require('ts-morph');

class C067SymbolBasedAnalyzer {
  constructor(semanticEngine = null) {
    this.semanticEngine = semanticEngine;
    this.verbose = false;
    
    // Common UI/framework strings that should be excluded
    this.UI_STRINGS = [
      'checkbox', 'button', 'search', 'remove', 'submit', 'cancel', 'ok', 'close',
      'Authorization', 'User-Agent', 'Content-Type', 'Accept', 'Bearer',
      'ArrowDown', 'ArrowUp', 'ArrowLeft', 'ArrowRight', 'bottom', 'top', 'left', 'right',
      'next-auth/react', '@nestjs/swagger', '@nestjs/common', 'nestjs-pino'
    ];
    
    // Test-related strings to exclude
    this.TEST_PATTERNS = [
      /^(test|mock|example|dummy|placeholder|fixture|stub)/i,
      /^(User \d+|Test User|Admin User)/i,
      /^(group\d+|item\d+|element\d+)/i,
      /^(abcdef\d+|123456|test-\w+)/i
    ];
    
    // Configuration patterns to detect - focused on environment-dependent config
    this.configPatterns = {
      // API endpoints and URLs - only external URLs, not internal endpoints
      urls: {
        regex: /^https?:\/\/(?!localhost|127\.0\.0\.1|0\.0\.0\.0)([a-zA-Z0-9-]+\.[a-zA-Z]{2,}|[^\/\s]+\.[^\/\s]+)(\/[^\s]*)?$/,
        exclude: [
          /^https?:\/\/(localhost|127\.0\.0\.1|0\.0\.0\.0)(:\d+)?/,  // Local development
          /^https?:\/\/(example\.com|test\.com|dummy\.com)/,         // Test domains
          /^(http|https):\/\/\$\{.+\}/                             // Template URLs with variables
        ]
      },
      
      // Environment-dependent numeric values (ports, timeouts that differ by env)
      environmentNumbers: {
        // Only consider numbers that are commonly different between environments
        isEnvironmentDependent: (value, context) => {
          const lowerContext = context.toLowerCase();
          
          // Business logic numbers are NOT environment config
          const businessLogicPatterns = [
            /limit|max|min|size|count|length|threshold/i,
            /page|record|item|batch|chunk|export/i,
            /width|height|margin|padding/i,
            /attempt|retry|step/i
          ];
          
          if (businessLogicPatterns.some(pattern => pattern.test(context))) {
            return false;
          }
          
          // Very specific values that are usually business constants
          const businessConstants = [
            20000, 10000, 5000, 1000, 500, 100, 50, 20, 10, 5,  // Common limits
            404, 500, 200, 201, 400, 401, 403,  // HTTP status codes
            24, 60, 3600, 86400,  // Time constants (hours, minutes, seconds)
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10  // Simple counters
          ];
          
          if (businessConstants.includes(value)) {
            return false;
          }
          
          // Port numbers (except common ones like 80, 443, 3000, 8080)
          if (typeof value === 'number' && value > 1000 && value < 65536) {
            const commonPorts = [3000, 8000, 8080, 9000, 5000, 4200, 4000];
            if (!commonPorts.includes(value)) {
              // Check if context suggests it's a port
              return /port|listen|bind|server/i.test(context);
            }
          }
          
          // Large timeout values that might differ by environment (> 10 seconds)
          if (typeof value === 'number' && value > 10000) {
            return /timeout|interval|delay|duration/i.test(context) && 
                   !businessLogicPatterns.some(pattern => pattern.test(context));
          }
          
          return false;
        }
      },
      
      // Database and connection strings
      connections: {
        regex: /^(mongodb|mysql|postgres|redis|elasticsearch):\/\/|^jdbc:|^Server=|^Data Source=/i
      },
      
      // API Keys and tokens (but exclude validation messages)
      credentials: {
        keywords: ['apikey', 'api_key', 'secret_key', 'access_token', 'client_secret'],
        exclude: [
          /must contain|should contain|invalid|error|message/i,    // Validation messages
          /description|comment|note/i,                             // Descriptions
          /^[a-z\s]{10,}$/i                                       // Long descriptive text
        ]
      }
    };
  }

  async initialize(semanticEngine = null) {
    if (semanticEngine) {
      this.semanticEngine = semanticEngine;
    }
    this.verbose = semanticEngine?.verbose || false;
  }

  async analyzeFileBasic(filePath, options = {}) {
    const violations = [];
    
    try {
      const sourceFile = this.semanticEngine.project.getSourceFile(filePath);
      if (!sourceFile) {
        if (this.verbose) {
          console.log(`[DEBUG] 🔍 C067: File not in semantic project, trying standalone: ${filePath.split('/').pop()}`);
        }
        // Fallback to standalone analysis if file not in semantic project
        return await this.analyzeFileStandalone(filePath, options);
      }

      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C067: Analyzing hardcoded config in ${filePath.split('/').pop()}`);
      }

      // Skip test files and config files themselves
      if (this.isConfigOrTestFile(filePath)) {
        if (this.verbose) {
          console.log(`[DEBUG] 🔍 C067: Skipping config/test file: ${filePath.split('/').pop()}`);
        }
        return violations;
      }

      // Find hardcoded configuration values
      const hardcodedConfigs = this.findHardcodedConfigs(sourceFile);
      
      for (const config of hardcodedConfigs) {
        violations.push({
          ruleId: 'C067',
          message: this.createMessage(config),
          filePath: filePath,
          line: config.line,
          column: config.column,
          severity: 'warning',
          category: 'configuration',
          type: config.type,
          value: config.value,
          suggestion: this.getSuggestion(config.type)
        });
      }

      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C067: Found ${violations.length} hardcoded config violations`);
      }

      return violations;
    } catch (error) {
      if (this.verbose) {
        console.error(`[DEBUG] ❌ C067: Symbol analysis error: ${error.message}`);
      }
      throw error;
    }
  }

  async analyzeFileStandalone(filePath, options = {}) {
    const violations = [];
    
    try {
      // Create a standalone ts-morph project for this analysis
      const project = new Project({
        compilerOptions: {
          target: 'ES2020',
          module: 'CommonJS',
          allowJs: true,
          allowSyntheticDefaultImports: true,
          esModuleInterop: true,
          skipLibCheck: true,
          strict: false
        },
        useInMemoryFileSystem: true
      });

      // Add the source file to the project
      const fs = require('fs');
      const path = require('path');
      
      // Check if file exists first
      if (!fs.existsSync(filePath)) {
        throw new Error(`File not found on filesystem: ${filePath}`);
      }
      
      // Read file content and create source file
      const fileContent = fs.readFileSync(filePath, 'utf8');
      const fileName = path.basename(filePath);
      const sourceFile = project.createSourceFile(fileName, fileContent);
      
      if (!sourceFile) {
        throw new Error(`Source file not found: ${filePath}`);
      }

      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C067: Analyzing hardcoded config in ${filePath.split('/').pop()} (standalone)`);
      }

      // Skip test files and config files themselves
      if (this.isConfigOrTestFile(filePath)) {
        if (this.verbose) {
          console.log(`[DEBUG] 🔍 C067: Skipping config/test file: ${filePath.split('/').pop()}`);
        }
        return violations;
      }

      // Find hardcoded configuration values
      const hardcodedConfigs = this.findHardcodedConfigs(sourceFile);
      
      for (const config of hardcodedConfigs) {
        violations.push({
          ruleId: 'C067',
          message: this.createMessage(config),
          filePath: filePath,
          line: config.line,
          column: config.column,
          severity: 'warning',
          category: 'configuration',
          type: config.type,
          value: config.value,
          suggestion: this.getSuggestion(config.type)
        });
      }

      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C067: Found ${violations.length} hardcoded config violations (standalone)`);
      }

      // Clean up the project
      project.removeSourceFile(sourceFile);

      return violations;
    } catch (error) {
      if (this.verbose) {
        console.error(`[DEBUG] ❌ C067: Standalone analysis error: ${error.message}`);
      }
      throw error;
    }
  }

  isConfigOrTestFile(filePath) {
    // Skip config files themselves and test files, including dummy/test data files
    const fileName = filePath.toLowerCase();
    const configPatterns = [
      /config\.(ts|js|json)$/,
      /\.config\.(ts|js)$/,
      /\.env$/,
      /\.env\./,
      /constants\.(ts|js)$/,
      /settings\.(ts|js)$/,
      /defaults\.(ts|js)$/
    ];
    
    const testPatterns = [
      /\.(test|spec)\.(ts|tsx|js|jsx)$/,
      /\/__tests__\//,
      /\/test\//,
      /\/tests\//,
      /\.stories\.(ts|tsx|js|jsx)$/,
      /\.mock\.(ts|tsx|js|jsx)$/,
      /\/dummy\//,                    // Skip dummy data files
      /dummy\.(ts|js)$/,              // Skip dummy files
      /test-fixtures\//,              // Skip test fixture files
      /\.fixture\.(ts|js)$/,          // Skip fixture files
      /entity\.(ts|js)$/              // Skip entity/ORM files (contain DB constraints)
    ];
    
    return configPatterns.some(pattern => pattern.test(fileName)) ||
           testPatterns.some(pattern => pattern.test(fileName));
  }

  findHardcodedConfigs(sourceFile) {
    const configs = [];
    
    // Traverse all nodes in the source file
    sourceFile.forEachDescendant((node) => {
      // Check string literals
      if (node.getKind() === SyntaxKind.StringLiteral) {
        const config = this.analyzeStringLiteral(node, sourceFile);
        if (config) {
          configs.push(config);
        }
      }
      
      // Check numeric literals
      if (node.getKind() === SyntaxKind.NumericLiteral) {
        const config = this.analyzeNumericLiteral(node, sourceFile);
        if (config) {
          configs.push(config);
        }
      }
      
      // Check template literals (for URLs with variables)
      if (node.getKind() === SyntaxKind.TemplateExpression) {
        const config = this.analyzeTemplateLiteral(node, sourceFile);
        if (config) {
          configs.push(config);
        }
      }
      
      // Check property assignments
      if (node.getKind() === SyntaxKind.PropertyAssignment) {
        const config = this.analyzePropertyAssignment(node, sourceFile);
        if (config) {
          configs.push(config);
        }
      }
      
      // Check variable declarations
      if (node.getKind() === SyntaxKind.VariableDeclaration) {
        const config = this.analyzeVariableDeclaration(node, sourceFile);
        if (config) {
          configs.push(config);
        }
      }
    });
    
    return configs;
  }

  analyzeStringLiteral(node, sourceFile) {
    const value = node.getLiteralValue();
    const position = sourceFile.getLineAndColumnAtPos(node.getStart());
    
    // Skip short strings and common UI values
    if (value.length < 4) return null;
    
    const parentContext = this.getParentContext(node);
    
    // Skip import paths and module names
    if (this.isImportPath(value, node)) return null;
    
    // Skip UI strings and labels
    if (this.isUIString(value)) return null;
    
    // Skip test data and mocks
    if (this.isTestData(value, parentContext)) return null;
    
    // Skip validation messages and error messages
    if (this.isValidationMessage(value, parentContext)) return null;
    
    // Skip file names and descriptions
    if (this.isFileNameOrDescription(value, parentContext)) return null;
    
    // Skip config keys (like 'api.baseUrl', 'features.newUI', etc.)
    if (this.looksLikeConfigKey(value)) {
      return null;
    }
    
    // Skip if this is used in a config service call
    if (parentContext.includes('config.get') || parentContext.includes('config.getString') || 
        parentContext.includes('config.getBoolean') || parentContext.includes('config.getNumber')) {
      return null;
    }
    
    // Skip if this is a property key in an object literal
    if (this.isPropertyKey(node)) {
      return null;
    }
    
    // Check for environment-dependent URLs only
    if (this.configPatterns.urls.regex.test(value)) {
      if (!this.isExcludedUrl(value, node) && this.isEnvironmentDependentUrl(value)) {
        return {
          type: 'url',
          value: value,
          line: position.line,
          column: position.column,
          node: node
        };
      }
    }
    
    // Check for real credentials (not validation messages)
    if (this.isRealCredential(value, parentContext)) {
      return {
        type: 'credential',
        value: value,
        line: position.line,
        column: position.column,
        node: node,
        context: parentContext
      };
    }
    
    // Check for connection strings
    if (this.configPatterns.connections.regex.test(value)) {
      return {
        type: 'connection',
        value: value,
        line: position.line,
        column: position.column,
        node: node
      };
    }
    
    return null;
  }

  analyzeNumericLiteral(node, sourceFile) {
    const value = node.getLiteralValue();
    const position = sourceFile.getLineAndColumnAtPos(node.getStart());
    const parentContext = this.getParentContext(node);
    
    // Only check for environment-dependent numbers
    if (this.configPatterns.environmentNumbers.isEnvironmentDependent(value, parentContext)) {
      return {
        type: 'environment_config',
        value: value,
        line: position.line,
        column: position.column,
        node: node,
        context: parentContext
      };
    }
    
    return null;
  }

  analyzeTemplateLiteral(node, sourceFile) {
    // For now, focus on simple template literals that might contain URLs
    const templateText = node.getFullText();
    if (templateText.includes('http://') || templateText.includes('https://')) {
      const position = sourceFile.getLineAndColumnAtPos(node.getStart());
      
      // Check if it's using environment variables or config
      if (!templateText.includes('process.env') && !templateText.includes('config.')) {
        return {
          type: 'template_url',
          value: templateText.trim(),
          line: position.line,
          column: position.column,
          node: node
        };
      }
    }
    
    return null;
  }

  analyzePropertyAssignment(node, sourceFile) {
    const nameNode = node.getNameNode();
    const valueNode = node.getInitializer();
    
    if (!nameNode || !valueNode) return null;
    
    const propertyName = nameNode.getText();
    const position = sourceFile.getLineAndColumnAtPos(node.getStart());
    
    // Skip ALL field mapping objects and ORM/database entity configurations
    const ancestorObj = node.getParent();
    if (ancestorObj && Node.isObjectLiteralExpression(ancestorObj)) {
      const objParent = ancestorObj.getParent();
      if (objParent && Node.isVariableDeclaration(objParent)) {
        const varName = objParent.getName();
        // Skip field mappings, database schemas, etc.
        if (/mapping|map|field|column|decode|schema|entity|constraint|table/i.test(varName)) {
          return null;
        }
      }
      
      // Check if this looks like a table column definition or field mapping
      const objText = ancestorObj.getText();
      if (/primaryKeyConstraintName|foreignKeyConstraintName|key.*may contain/i.test(objText)) {
        return null; // Skip database constraint definitions
      }
    }
    
    // Skip properties that are clearly field mappings or business data
    const businessLogicProperties = [
      // Field mappings
      'key', 'field', 'dataKey', 'valueKey', 'labelKey', 'sortKey',
      // Business logic
      'endpoint', 'path', 'route', 'method',
      'limit', 'pageSize', 'batchSize', 'maxResults',
      'retry', 'retries', 'maxRetries', 'attempts',
      'count', 'max', 'min', 'size', 'length',
      // UI properties
      'className', 'style', 'disabled', 'readonly',
      // Database/ORM
      'primaryKeyConstraintName', 'foreignKeyConstraintName', 'constraintName',
      'tableName', 'columnName', 'schemaName'
    ];
    
    const lowerPropertyName = propertyName.toLowerCase();
    if (businessLogicProperties.some(prop => lowerPropertyName.includes(prop))) {
      return null; // Skip these completely
    }
    
    // Only check for CLEARLY environment-dependent properties
    const trulyEnvironmentDependentProps = [
      'baseurl', 'baseURL', 'host', 'hostname', 'server', 'endpoint', 
      'apikey', 'api_key', 'secret_key', 'client_secret',
      'database', 'connectionstring', 'dbhost', 'dbport',
      'port', 'timeout', // Only when they have suspicious values
      'bucket', 'region', // Cloud-specific
      'clientid', 'tenantid' // OAuth-specific
    ];
    
    if (!trulyEnvironmentDependentProps.some(prop => lowerPropertyName.includes(prop))) {
      return null; // Not clearly environment-dependent
    }
    
    let value = null;
    let configType = null;
    
    if (valueNode.getKind() === SyntaxKind.StringLiteral) {
      value = valueNode.getLiteralValue();
      
      // Only flag URLs or clearly sensitive values
      if (this.configPatterns.urls.regex.test(value) && this.isEnvironmentDependentUrl(value)) {
        configType = 'url';
      } else if (this.isRealCredential(value, propertyName)) {
        configType = 'credential';
      } else {
        return null; // Skip other string values
      }
    } else if (valueNode.getKind() === SyntaxKind.NumericLiteral) {
      value = valueNode.getLiteralValue();
      const parentContext = this.getParentContext(node);
      
      // Only flag numbers that are clearly environment-dependent
      if (this.configPatterns.environmentNumbers.isEnvironmentDependent(value, parentContext)) {
        configType = 'environment_config';
      } else {
        return null;
      }
    } else {
      return null; // Skip other value types
    }
    
    if (configType) {
      return {
        type: configType,
        value: value,
        line: position.line,
        column: position.column,
        node: node,
        propertyName: propertyName
      };
    }
    
    return null;
  }

  analyzeVariableDeclaration(node, sourceFile) {
    const nameNode = node.getNameNode();
    const initializer = node.getInitializer();
    
    if (!nameNode || !initializer) return null;
    
    const variableName = nameNode.getText();
    const position = sourceFile.getLineAndColumnAtPos(node.getStart());
    
    // Check if variable name suggests environment-dependent configuration
    if (this.isEnvironmentDependentProperty(variableName)) {
      let value = null;
      
      if (initializer.getKind() === SyntaxKind.StringLiteral) {
        value = initializer.getLiteralValue();
      } else if (initializer.getKind() === SyntaxKind.NumericLiteral) {
        value = initializer.getLiteralValue();
      }
      
      if (value !== null && this.looksLikeEnvironmentConfig(variableName, value)) {
        return {
          type: 'variable_config',
          value: value,
          line: position.line,
          column: position.column,
          node: node,
          variableName: variableName
        };
      }
    }
    
    return null;
  }

  getParentContext(node) {
    // Get surrounding context to understand the purpose of the literal
    let parent = node.getParent();
    let context = '';
    
    // Check if this is a method call argument or property access
    while (parent && context.length < 100) {
      const parentText = parent.getText();
      
      // If parent is CallExpression and this node is an argument, it might be a config key
      if (parent.getKind() === SyntaxKind.CallExpression) {
        const callExpr = parent;
        const methodName = this.getMethodName(callExpr);
        if (['get', 'getBoolean', 'getNumber', 'getArray', 'getString'].includes(methodName)) {
          return `config.${methodName}()`;  // This indicates it's a config key
        }
      }
      
      if (parentText.length < 200) {
        context = parentText;
        break;
      }
      parent = parent.getParent();
    }
    
    return context;
  }

  getMethodName(callExpression) {
    const expression = callExpression.getExpression();
    if (expression.getKind() === SyntaxKind.PropertyAccessExpression) {
      return expression.getName();
    }
    if (expression.getKind() === SyntaxKind.Identifier) {
      return expression.getText();
    }
    return '';
  }

  isExcludedUrl(value, node) {
    return this.configPatterns.urls.exclude.some(pattern => pattern.test(value));
  }

  isExcludedCredential(value, node) {
    return this.configPatterns.credentials.exclude.some(pattern => pattern.test(value));
  }

  containsCredentialKeyword(context) {
    const lowerContext = context.toLowerCase();
    
    // Skip if this looks like a header name or property key definition
    if (context.includes("':") || context.includes('": ') || context.includes(' = ')) {
      // This might be a key-value pair where the string is the key
      return false;
    }
    
    return this.configPatterns.credentials.keywords.some(keyword => 
      lowerContext.includes(keyword)
    );
  }

  looksLikeUIValue(value, context) {
    // Check if it's likely a UI-related value (like input type, label, etc.)
    const uiKeywords = ['input', 'type', 'field', 'label', 'placeholder', 'text', 'button'];
    const lowerContext = context.toLowerCase();
    return uiKeywords.some(keyword => lowerContext.includes(keyword));
  }

  looksLikeConfigKey(value) {
    // Check if it looks like a config key path (e.g., 'api.baseUrl', 'features.newUI')
    if (/^[a-zA-Z][a-zA-Z0-9]*\.[a-zA-Z][a-zA-Z0-9]*(\.[a-zA-Z][a-zA-Z0-9]*)*$/.test(value)) {
      return true;
    }
    
    // Check for other config key patterns
    const configKeyPatterns = [
      /^[a-zA-Z][a-zA-Z0-9]*\.[a-zA-Z]/,  // dotted notation like 'api.url'
      /^[A-Z_][A-Z0-9_]*$/,               // CONSTANT_CASE like 'API_URL'
      /^get[A-Z]/,                        // getter methods like 'getApiUrl'
      /^config\./,                        // config namespace
      /^settings\./,                      // settings namespace
      /^env\./                           // env namespace
    ];
    
    return configKeyPatterns.some(pattern => pattern.test(value));
  }

  isPropertyKey(node) {
    // Check if this string literal is used as a property key in an object literal
    const parent = node.getParent();
    
    // If parent is PropertyAssignment and this node is the name, it's a property key
    if (parent && parent.getKind() === SyntaxKind.PropertyAssignment) {
      const nameNode = parent.getNameNode();
      return nameNode === node;
    }
    
    return false;
  }

  isImportPath(value, node) {
    // Check if this is likely an import path or module name
    const parent = node.getParent();
    
    // Check if it's in an import statement
    let currentNode = parent;
    while (currentNode) {
      const kind = currentNode.getKind();
      if (kind === SyntaxKind.ImportDeclaration || 
          kind === SyntaxKind.ExportDeclaration ||
          kind === SyntaxKind.CallExpression) {
        const text = currentNode.getText();
        if (text.includes('require(') || text.includes('import ') || text.includes('from ')) {
          return true;
        }
      }
      currentNode = currentNode.getParent();
    }
    
    // Check for common import path patterns
    return /^[@a-z][a-z0-9\-_]*\/|^[a-z][a-z0-9\-_]*$|^\.{1,2}\//.test(value) ||
           value.endsWith('.js') || value.endsWith('.ts') || 
           value.endsWith('.json') || value.endsWith('.css') ||
           value.endsWith('.scss') || value.endsWith('.html');
  }

  isUIString(value) {
    // Check against predefined UI string patterns, but don't skip credentials
    if (typeof value === 'string' && value.length > 20 && 
        (/token|key|secret|bearer|auth/i.test(value) || /^[a-f0-9-]{30,}$/i.test(value))) {
      // Don't skip potential credentials/tokens even if they contain UI keywords
      return false;
    }
    
    return this.UI_STRINGS.some(pattern => {
      if (typeof pattern === 'string') {
        return value === pattern; // Exact match only, not includes
      } else {
        return pattern.test(value);
      }
    });
  }

  isTestData(value, context) {
    // Don't skip credentials/tokens even in dummy files
    if (typeof value === 'string' && value.length > 15 && 
        (/token|key|secret|auth|bearer|jwt/i.test(value) || 
         /^[a-f0-9-]{20,}$/i.test(value) ||  // Hex tokens
         /^[A-Za-z0-9_-]{20,}$/i.test(value))) { // Base64-like tokens
      return false; // Don't skip potential credentials
    }
    
    // Check for test patterns in value - but be more restrictive
    if (this.TEST_PATTERNS.some(pattern => pattern.test(value))) {
      // Only skip if it's clearly test data, not production dummy data
      const isInTestFile = /\.(test|spec)\.(ts|tsx|js|jsx)$/i.test(context) ||
                          /\/__tests__\//i.test(context) ||
                          /\/test\//i.test(context);
      return isInTestFile;
    }
    
    // Check for test context
    const lowerContext = context.toLowerCase();
    const testKeywords = ['test', 'spec', 'mock', 'fixture', 'stub', 'describe', 'it('];
    return testKeywords.some(keyword => lowerContext.includes(keyword));
  }

  isValidationMessage(value, context) {
    // Skip validation/error messages
    const validationPatterns = [
      /must contain|should contain|invalid|error|required|missing/i,
      /password|username|email/i, // Common validation contexts
      /^[A-Z][a-z\s]{10,}$/,     // Sentence-like messages
      /\s(at least|one|letter|uppercase|lowercase|numeric)/i
    ];
    
    return validationPatterns.some(pattern => pattern.test(value)) ||
           /message|error|validation|description/i.test(context);
  }

  isFileNameOrDescription(value, context) {
    // Skip file names and descriptions
    const filePatterns = [
      /\.(csv|json|xml|txt|md)$/i,
      /^[a-z_\-]+\.(csv|json|xml|txt)$/i,
      /description|comment|note|foreign key|identity/i
    ];
    
    return filePatterns.some(pattern => pattern.test(value)) ||
           /description|comment|note|identity|foreign|table/i.test(context);
  }

  isEnvironmentDependentUrl(value) {
    // Only flag URLs that are likely to differ between environments
    const envDependentPatterns = [
      /\.amazonaws\.com/, // AWS services
      /\.azure\.com/,     // Azure services  
      /\.googleapis\.com/, // Google services
      /api\./,            // API endpoints
      /\.dev|\.staging|\.prod/i // Environment-specific domains
    ];
    
    return envDependentPatterns.some(pattern => pattern.test(value));
  }

  isRealCredential(value, context) {
    // Check for real credentials, not validation messages
    const credentialKeywords = this.configPatterns.credentials.keywords;
    const lowerContext = context.toLowerCase();
    
    // Must have credential keyword in context
    if (!credentialKeywords.some(keyword => lowerContext.includes(keyword))) {
      return false;
    }
    
    // Skip if it's excluded (validation messages, etc.)
    if (this.configPatterns.credentials.exclude.some(pattern => pattern.test(value))) {
      return false;
    }
    
    // Skip validation messages and descriptions
    if (this.isValidationMessage(value, context)) {
      return false;
    }
    
    // Must be reasonably long and not look like UI text
    return value.length >= 6 && !this.looksLikeUIValue(value, context);
  }

  isEnvironmentDependentProperty(propertyName) {
    // Skip UI/framework related property names
    const uiPropertyPatterns = [
      /^key[A-Z]/,           // keyXxx (UI field keys)
      /^field[A-Z]/,         // fieldXxx 
      /^prop[A-Z]/,          // propXxx
      /^data[A-Z]/,          // dataXxx
      /CheckDisplay/,        // UI display control keys
      /InputPossible/,       // UI input control keys
      /Flag$/,               // UI flags
      /Class$/,              // CSS classes
      /^(disabled|readonly|active)Class$/i  // UI state classes
    ];
    
    if (uiPropertyPatterns.some(pattern => pattern.test(propertyName))) {
      return false;
    }
    
    // Properties that are likely to differ between environments
    const envDependentProps = [
      'baseurl', 'baseURL', 'host', 'hostname', 'server',
      'apikey', 'api_key', 'secret', 'token', 'password', 'credential',
      'database', 'db', 'connection', 'connectionstring',
      'timeout', // Only long timeouts
      'port',    // Only non-standard ports
      'authorization', 'auth', 'authentication', // Auth headers and codes
      'apptoken', 'devicetoken', 'accesstoken', 'refreshtoken', // App tokens
      'code', 'hash', 'signature', 'key', // Various security values
      'clientsecret', 'clientid', 'sessionkey', // OAuth and session
      'requestid', 'sessionid', 'transactionid', 'otp' // Request/session tracking
    ];
    
    const lowerName = propertyName.toLowerCase();
    return envDependentProps.some(prop => lowerName.includes(prop));
  }

  looksLikeEnvironmentConfig(propertyName, value) {
    // Check if this property/value combination looks like environment config
    const lowerPropertyName = propertyName.toLowerCase();
    
    if (typeof value === 'string') {
      // Skip test data (common test passwords, etc.)
      const testDataPatterns = [
        /^(password123|test123|admin123|user123|wrongpassword|testpassword)$/i,
        /^(test|mock|dummy|sample|example)/i,
        /^\/(api|mock|test)/,  // Test API paths
        /^[a-z]+\d+$/i         // Simple test values like 'user1', 'test2'
      ];
      
      // Don't skip common test patterns if they appear in credential contexts
      const isCredentialContext = /token|key|secret|auth|otp|code|password|credential/i.test(propertyName);
      
      if (!isCredentialContext && testDataPatterns.some(pattern => pattern.test(value))) {
        return false;
      }
      
      // Skip object property paths and field names
      const propertyPathPatterns = [
        /^[a-zA-Z][a-zA-Z0-9]*(\[[0-9]+\])?\.[a-zA-Z][a-zA-Z0-9]*$/,  // obj[0].prop, obj.prop
        /^[a-zA-Z][a-zA-Z0-9]*\.[a-zA-Z][a-zA-Z0-9]*(\.[a-zA-Z][a-zA-Z0-9]*)*$/,  // obj.prop.subprop
        /^[a-zA-Z][a-zA-Z0-9]*(\[[0-9]+\])+$/,  // obj[0], obj[0][1]
        /^(key|field|prop|data)[A-Z]/,  // keyXxx, fieldXxx, propXxx, dataXxx
        /CheckDisplay|InputPossible|Flag$/i,  // Common UI field patterns
        /^exflg|^flg|Support$/i,  // Business logic flags
      ];
      
      if (propertyPathPatterns.some(pattern => pattern.test(value))) {
        return false;
      }
      
      // Skip CSS classes and UI constants
      const uiPatterns = [
        /^bg-|text-|cursor-|border-|flex-|grid-/,  // CSS classes
        /^(disabled|readonly|active|inactive)$/i,  // UI states
        /class$/i  // className values
      ];
      
      if (uiPatterns.some(pattern => pattern.test(value))) {
        return false;
      }
      
      // Skip internal system identifiers (queue names, service names, route names)
      const systemIdentifierPatterns = [
        /-queue$/i,           // Queue names
        /-task$/i,            // Task names  
        /-activity$/i,        // Activity names
        /-service$/i,         // Service names
        /-worker$/i,          // Worker names
        /^[A-Z_]+_QUEUE$/,    // CONSTANT_QUEUE names
        /^[A-Z_]+_TASK$/,     // CONSTANT_TASK names
        /^(register|login|logout|reset-password|verify|update)$/i, // Route names
        /password|token/i && /invalid|expired|attempts|exceeded/i   // Error messages
      ];
      
      if (systemIdentifierPatterns.some(pattern => pattern.test(value))) {
        return false;
      }
      
      // Skip error messages and validation messages
      const messagePatterns = [
        /invalid|expired|exceeded|failed|error|success/i,
        /attempts|required|missing|not found/i,
        /^[A-Z][a-z\s]{10,}$/,  // Sentence-like messages
        /は|が|を|に|で|と/,      // Japanese particles (UI text)
        /情報|画面|ボタン|入力/    // Japanese UI terms
      ];
      
      if (messagePatterns.some(pattern => pattern.test(value))) {
        return false;
      }
      
      // URLs are environment-dependent
      if (this.configPatterns.urls.regex.test(value)) {
        return this.isEnvironmentDependentUrl(value);
      }
      
      // Credentials - but exclude test data
      if (lowerPropertyName.includes('key') || lowerPropertyName.includes('secret') || 
          lowerPropertyName.includes('token') || lowerPropertyName.includes('password')) {
        return value.length > 10; // Real secrets are usually longer
      }
      
      // Skip short endpoint names or simple strings
      if (value.length < 10 && !value.includes('.') && !value.includes('/')) {
        return false;
      }
    }
    
    if (typeof value === 'number') {
      // Only flag environment-dependent numbers
      return this.configPatterns.environmentNumbers.isEnvironmentDependent(value, propertyName);
    }
    
    return true;
  }

  isCommonConstant(value) {
    // Common constants that are usually OK to hardcode
    const commonConstants = [100, 200, 300, 400, 500, 1000, 2000, 3000, 5000, 8080, 3000];
    return commonConstants.includes(value);
  }

  isConfigProperty(propertyName) {
    const configProps = [
      'url', 'endpoint', 'baseurl', 'apiurl', 'host', 'port',
      'timeout', 'interval', 'delay', 'retry', 'retries',
      'username', 'password', 'apikey', 'secret', 'token',
      'database', 'connection', 'connectionstring',
      'maxsize', 'batchsize', 'pagesize', 'limit'
    ];
    
    const lowerName = propertyName.toLowerCase();
    return configProps.some(prop => lowerName.includes(prop));
  }

  isConfigVariable(variableName) {
    const configVars = [
      'api', 'url', 'endpoint', 'host', 'port',
      'timeout', 'interval', 'delay', 'retry',
      'config', 'setting', 'constant'
    ];
    
    const lowerName = variableName.toLowerCase();
    return configVars.some(var_ => lowerName.includes(var_));
  }

  looksLikeHardcodedConfig(name, value) {
    // Skip obvious constants and UI values
    if (typeof value === 'string') {
      if (value.length < 3) return false;
      if (/^(ok|yes|no|true|false|success|error|info|warn)$/i.test(value)) return false;
    }
    
    if (typeof value === 'number') {
      if (this.isCommonConstant(value)) return false;
    }
    
    return true;
  }

  createMessage(config) {
    const baseMessage = 'Environment-dependent configuration should not be hardcoded.';
    
    switch (config.type) {
      case 'url':
        return `${baseMessage} External URL '${config.value}' should be loaded from environment variables or configuration files.`;
      case 'credential':
        return `${baseMessage} Credential value '${config.value}' should be loaded from secure environment variables.`;
      case 'environment_config':
        return `${baseMessage} Environment-dependent value ${config.value} should be configurable via environment variables or config files.`;
      case 'connection':
        return `${baseMessage} Connection string should be loaded from environment variables.`;
      case 'property_config':
        return `${baseMessage} Property '${config.propertyName}' may contain environment-dependent value '${config.value}'.`;
      case 'variable_config':
        return `${baseMessage} Variable '${config.variableName}' may contain environment-dependent value '${config.value}'.`;
      case 'config_key':
        return `${baseMessage} Configuration key '${config.value}' should not be hardcoded.`;
      default:
        return `${baseMessage} Value '${config.value}' may differ between environments.`;
    }
  }

  getSuggestion(type) {
    const suggestions = {
      'url': 'Use process.env.API_URL or config.get("api.url")',
      'credential': 'Use process.env.SECRET_KEY or secure vault',
      'environment_config': 'Move to environment variables or config service',
      'connection': 'Use process.env.DATABASE_URL',
      'property_config': 'Consider if this differs between dev/staging/production',
      'variable_config': 'Use environment variables if this differs between environments',
      'config_key': 'Use constants or enums for configuration keys'
    };
    
    return suggestions[type] || 'Consider if this value should differ between dev/staging/production environments';
  }
}

module.exports = C067SymbolBasedAnalyzer;
