// Enhanced symbol-based analyzer for C014
const { SyntaxKind } = require('ts-morph');

class C014SymbolBasedAnalyzer {
  constructor(semanticEngine = null) {
    this.semanticEngine = semanticEngine;
    this.verbose = false;
    
    // Configuration
    this.config = {
      // Built-in classes that are allowed
      allowedBuiltins: [
        'Date', 'Array', 'Object', 'String', 'Number', 'Boolean', 'RegExp',
        'Map', 'Set', 'WeakMap', 'WeakSet', 'Promise', 'Error', 'TypeError',
        'FormData', 'Headers', 'Request', 'Response', 'URLSearchParams',
        'URL', 'Blob', 'File', 'Buffer', 'AbortController', 'AbortSignal',
        'TextEncoder', 'TextDecoder', 'MessageChannel', 'MessagePort',
        'Worker', 'SharedWorker', 'EventSource', 'WebSocket'
      ],
      
      // Value objects/DTOs that are typically safe to instantiate
      allowedValueObjects: [
        'Money', 'Price', 'Currency', 'Quantity', 'Amount',
        'Email', 'Phone', 'Address', 'Name', 'Id', 'UserId',
        'UUID', 'Timestamp', 'Duration', 'Range'
      ],
      
      // Infrastructure patterns that suggest external dependencies
      infraPatterns: [
        'Client', 'Repository', 'Service', 'Gateway', 'Adapter',
        'Provider', 'Factory', 'Builder', 'Manager', 'Handler',
        'Controller', 'Processor', 'Validator', 'Logger'
      ],
      
      // DI decorators that indicate proper injection
      diDecorators: [
        'Injectable', 'Inject', 'Autowired', 'Component',
        'Service', 'Repository', 'Controller', 'autoInjectable'
      ],
      
      // Patterns to exclude from analysis
      excludePatterns: [
        '**/*.test.ts', '**/*.spec.ts', '**/*.test.js', '**/*.spec.js',
        '**/tests/**', '**/test/**', '**/migration/**', '**/scripts/**'
      ]
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
      // Try different approaches to get the source file
      let sourceFile = this.semanticEngine.project.getSourceFile(filePath);
      
      // If not found by full path, try by filename
      if (!sourceFile) {
        const fileName = filePath.split('/').pop();
        const allFiles = this.semanticEngine.project.getSourceFiles();
        sourceFile = allFiles.find(f => f.getBaseName() === fileName);
      }
      
      // If still not found, try to add the file
      if (!sourceFile) {
        try {
          if (require('fs').existsSync(filePath)) {
            sourceFile = this.semanticEngine.project.addSourceFileAtPath(filePath);
          }
        } catch (addError) {
          // Fall through to error below
        }
      }
      
      if (!sourceFile) {
        throw new Error(`Source file not found: ${filePath}`);
      }

      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C014: Analyzing DI violations in ${filePath.split('/').pop()}`);
      }

      // Skip test files and excluded patterns
      if (this.shouldSkipFile(filePath)) {
        if (this.verbose) {
          console.log(`[DEBUG] 🔍 C014: Skipping excluded file ${filePath.split('/').pop()}`);
        }
        return violations;
      }

      // Find all new expressions that might violate DI principles
      const newExpressions = this.findProblematicNewExpressions(sourceFile);
      
      for (const expr of newExpressions) {
        if (this.isDependencyInjectionViolation(expr, sourceFile)) {
          violations.push({
            ruleId: 'C014',
            message: this.buildViolationMessage(expr),
            filePath: filePath,
            line: expr.line,
            column: expr.column,
            severity: 'warning',
            category: 'design',
            confidence: expr.confidence,
            suggestion: this.buildSuggestion(expr)
          });
        }
      }

      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C014: Found ${violations.length} DI violations`);
      }

      return violations;
    } catch (error) {
      if (this.verbose) {
        console.error(`[DEBUG] ❌ C014: Symbol analysis error: ${error.message}`);
      }
      throw error;
    }
  }

  findProblematicNewExpressions(sourceFile) {
    const expressions = [];
    
    function traverse(node) {
      if (node.getKind() === SyntaxKind.NewExpression) {
        const newExpr = node;
        const expression = newExpr.getExpression();
        
        // Get class name and context information
        const className = this.getClassName(expression);
        const position = sourceFile.getLineAndColumnAtPos(newExpr.getStart());
        const context = this.analyzeContext(newExpr);
        
      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C014: Found new expression: ${className} at line ${position.line}`);
      }
        
        if (className) {
          expressions.push({
            node: newExpr,
            className: className,
            line: position.line,
            column: position.column,
            context: context,
            confidence: this.calculateConfidence(className, context)
          });
        }
      }

      // Traverse children
      node.forEachChild(child => traverse.call(this, child));
    }

    traverse.call(this, sourceFile);
    
    if (this.verbose) {
      console.log(`[DEBUG] 🔍 C014: Found ${expressions.length} new expressions total`);
    }
    
    return expressions;
  }

  getClassName(expression) {
    if (expression.getKind() === SyntaxKind.Identifier) {
      return expression.getText();
    }
    
    // Handle qualified names like MyNamespace.MyClass
    if (expression.getKind() === SyntaxKind.PropertyAccessExpression) {
      return expression.getName();
    }
    
    return null;
  }

  analyzeContext(newExpressionNode) {
    const context = {
      isInConstructor: false,
      isAssignedToThis: false,
      isInMethod: false,
      isLocalVariable: false,
      isReturnValue: false,
      isImmediateUse: false,
      parentFunction: null,
      hasDecorators: false
    };

    let current = newExpressionNode.getParent();
    
    while (current) {
      switch (current.getKind()) {
        case SyntaxKind.Constructor:
          context.isInConstructor = true;
          context.parentFunction = current;
          break;
          
        case SyntaxKind.MethodDeclaration:
          context.isInMethod = true;
          context.parentFunction = current;
          break;
          
        case SyntaxKind.BinaryExpression:
          // Check if it's assignment to this.property
          const binaryExpr = current;
          if (binaryExpr.getOperatorToken().getKind() === SyntaxKind.EqualsToken) {
            const left = binaryExpr.getLeft();
            if (left.getKind() === SyntaxKind.PropertyAccessExpression) {
              const propAccess = left;
              if (propAccess.getExpression().getKind() === SyntaxKind.ThisKeyword) {
                context.isAssignedToThis = true;
              }
            }
          }
          break;
          
        case SyntaxKind.PropertyDeclaration:
          // Check if this new expression is in a class property initializer
          const propDecl = current;
          const initializer = propDecl.getInitializer();
          if (initializer && this.containsNewExpression(initializer, newExpressionNode)) {
            context.isAssignedToThis = true; // Class property is effectively "this.property"
          }
          break;
          
        case SyntaxKind.VariableDeclaration:
          context.isLocalVariable = true;
          break;
          
        case SyntaxKind.ReturnStatement:
          context.isReturnValue = true;
          break;
          
        case SyntaxKind.CallExpression:
          // Check for immediate method call like new Date().getTime()
          const callExpr = current;
          if (callExpr.getExpression() === newExpressionNode) {
            context.isImmediateUse = true;
          }
          break;
          
        case SyntaxKind.ClassDeclaration:
          // Check for DI decorators on the class
          context.hasDecorators = this.hasDecorators(current, this.config.diDecorators);
          break;
      }
      
      current = current.getParent();
    }

    return context;
  }

  containsNewExpression(node, targetNewExpr) {
    if (node === targetNewExpr) {
      return true;
    }
    
    let found = false;
    node.forEachChild(child => {
      if (found) return;
      if (this.containsNewExpression(child, targetNewExpr)) {
        found = true;
      }
    });
    
    return found;
  }

  isDependencyInjectionViolation(expr, sourceFile) {
    const { className, context } = expr;

    if (this.verbose) {
      console.log(`[DEBUG] 🔍 C014: Checking violation for ${className}:`, {
        isAssignedToThis: context.isAssignedToThis,
        isInConstructor: context.isInConstructor,
        isInMethod: context.isInMethod,
        isLocalVariable: context.isLocalVariable,
        isImmediateUse: context.isImmediateUse,
        isReturnValue: context.isReturnValue,
        hasDecorators: context.hasDecorators
      });
    }

    // 1. Skip built-in JavaScript/DOM classes
    if (this.config.allowedBuiltins.includes(className)) {
      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C014: Skipping ${className} - allowed builtin`);
      }
      return false;
    }

    // 2. Skip exception/error classes
    if (this.isExceptionClass(className, sourceFile)) {
      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C014: Skipping ${className} - Exception/Error class`);
      }
      return false;
    }

    // 4. Skip entity/model classes (data structures)
    if (this.isEntityClass(className)) {
      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C014: Skipping ${className} - Entity/Model class`);
      }
      return false;
    }

    // 5. Skip command pattern classes (value objects for operations)
    if (this.isCommandPattern(className)) {
      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C014: Skipping ${className} - Command pattern class`);
      }
      return false;
    }

    // 6. Skip value objects/DTOs (configurable)
    if (this.config.allowedValueObjects.includes(className)) {
      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C014: Skipping ${className} - allowed value object`);
      }
      return false;
    }

    // 3. Skip if it's immediate usage (not stored as dependency)
    if (context.isImmediateUse || context.isReturnValue) {
      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C014: Skipping ${className} - immediate use or return value`);
      }
      return false;
    }

    // 4. Skip Singleton pattern (self-instantiation in getInstance-like methods)
    if (this.isSingletonPattern(className, context, sourceFile)) {
      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C014: Skipping ${className} - Singleton pattern`);
      }
      return false;
    }

    // 5. Skip if it's a local variable in method (not dependency field) UNLESS it's infrastructure
    if (context.isInMethod && context.isLocalVariable && !context.isAssignedToThis) {
      // Exception: Still flag if it's infrastructure dependency even as local variable
      if (this.isLikelyExternalDependency(className, sourceFile)) {
        if (this.verbose) {
          console.log(`[DEBUG] ✅ C014: ${className} is violation - infrastructure dependency even as local variable`);
        }
        return true;
      }
      
      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C014: Skipping ${className} - local variable in method`);
      }
      return false;
    }

    // 6. Main heuristic: Flag if assigned to this.* (field or in constructor/method)
    if (context.isAssignedToThis) {
      // Check if target class suggests external dependency
      if (this.isLikelyExternalDependency(className, sourceFile)) {
        if (this.verbose) {
          console.log(`[DEBUG] ✅ C014: ${className} is violation - assigned to this and external dependency`);
        }
        return true;
      }
    }

    // 7. Skip if it's service locator pattern (centralized API client configuration)
    if (this.isServiceLocatorPattern(context, sourceFile)) {
      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C014: Skipping ${className} - Service locator pattern`);
      }
      return false;
    }

    // 8. Flag if class has infrastructure patterns and no DI decorators
    if (this.hasInfraPattern(className) && !context.hasDecorators) {
      if (this.verbose) {
        console.log(`[DEBUG] ✅ C014: ${className} is violation - has infra pattern`);
      }
      return true;
    }

    if (this.verbose) {
      console.log(`[DEBUG] 🔍 C014: ${className} is NOT a violation`);
    }

    return false;
  }

  isLikelyExternalDependency(className, sourceFile) {
    if (this.verbose) {
      console.log(`[DEBUG] 🔍 C014: Checking if ${className} is external dependency`);
    }
    
    // Check if class name suggests infrastructure/external service
    const hasInfraPattern = this.config.infraPatterns.some(pattern => 
      className.includes(pattern)
    );

    if (hasInfraPattern) {
      if (this.verbose) {
        console.log(`[DEBUG] ✅ C014: ${className} has infra pattern`);
      }
      return true;
    }

    // Check import statements to see if it's from external module
    const imports = sourceFile.getImportDeclarations();
    for (const importDecl of imports) {
      const namedImports = importDecl.getNamedImports();
      for (const namedImport of namedImports) {
        if (namedImport.getName() === className) {
          const moduleSpecifier = importDecl.getModuleSpecifierValue();
          if (this.verbose) {
            console.log(`[DEBUG] 🔍 C014: ${className} imported from: ${moduleSpecifier}`);
          }
          // Check if imported from infrastructure/adapter paths
          if (this.isInfrastructurePath(moduleSpecifier)) {
            if (this.verbose) {
              console.log(`[DEBUG] ✅ C014: ${className} from infrastructure path`);
            }
            return true;
          }
        }
      }
    }

    if (this.verbose) {
      console.log(`[DEBUG] 🔍 C014: ${className} is NOT external dependency`);
    }
    return false;
  }

  isInfrastructurePath(modulePath) {
    const infraPaths = [
      'infra', 'infrastructure', 'adapters', 'clients', 
      'repositories', 'services', 'gateways', 'providers'
    ];
    
    // Check explicit infra path keywords
    if (infraPaths.some(path => modulePath.includes(path))) {
      return true;
    }
    
    // Check common external infrastructure packages
    const infraPackages = [
      '@aws-sdk/', 'aws-sdk', 'redis', 'mysql', 'postgresql', 'prisma',
      'mongoose', 'sequelize', 'typeorm', 'knex', 'pg', 'mysql2',
      's3-sync-client', 'firebase', 'googleapis', 'stripe',
      'twilio', 'sendgrid', 'nodemailer', 'kafka', 'rabbitmq',
      'elasticsearch', 'mongodb', 'cassandra'
    ];
    
    return infraPackages.some(pkg => modulePath.includes(pkg));
  }

  hasInfraPattern(className) {
    return this.config.infraPatterns.some(pattern => 
      className.includes(pattern)
    );
  }

  isExceptionClass(className, sourceFile) {
    // First check by naming convention (fast path)
    const errorPatterns = [
      'Error', 'Exception', 'Fault', 'Failure'
    ];
    
    const hasErrorName = errorPatterns.some(pattern => 
      className.endsWith(pattern) || className.includes(pattern)
    );
    
    if (hasErrorName) {
      return true;
    }

    // Check inheritance hierarchy using semantic analysis
    if (this.semanticEngine && sourceFile) {
      try {
        return this.inheritsFromErrorClass(className, sourceFile);
      } catch (error) {
        if (this.verbose) {
          console.log(`[DEBUG] 🔍 C014: Could not check inheritance for ${className}: ${error.message}`);
        }
        // Fall back to name-based check only
        return false;
      }
    }
    
    return false;
  }

  inheritsFromErrorClass(className, sourceFile) {
    // Find class declaration in current file
    const classDecl = sourceFile.getClasses().find(cls => cls.getName() === className);
    
    if (!classDecl) {
      // Class might be imported, try to resolve it
      return this.isImportedErrorClass(className, sourceFile);
    }

    // Check direct inheritance
    const extendsClauses = classDecl.getExtends();
    if (!extendsClauses) {
      return false;
    }

    const baseClassName = extendsClauses.getExpression().getText();
    
    // Check if directly extends Error-like class
    const errorBaseClasses = [
      'Error', 'TypeError', 'ReferenceError', 'SyntaxError', 
      'RangeError', 'EvalError', 'URIError', 'AggregateError'
    ];
    
    if (errorBaseClasses.includes(baseClassName)) {
      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C014: ${className} extends ${baseClassName} (Error class)`);
      }
      return true;
    }

    // Recursively check inheritance chain
    return this.inheritsFromErrorClass(baseClassName, sourceFile);
  }

  isImportedErrorClass(className, sourceFile) {
    // Check imports to see if className is imported from error/exception modules
    const imports = sourceFile.getImportDeclarations();
    
    for (const importDecl of imports) {
      const namedImports = importDecl.getNamedImports();
      for (const namedImport of namedImports) {
        if (namedImport.getName() === className) {
          const moduleSpecifier = importDecl.getModuleSpecifierValue();
          
          // Check if imported from error/exception related modules
          const errorModulePatterns = [
            'error', 'exception', 'http-exception', 'custom-error',
            '../exceptions', './exceptions', '/errors/', '/exceptions/'
          ];
          
          const isFromErrorModule = errorModulePatterns.some(pattern => 
            moduleSpecifier.toLowerCase().includes(pattern)
          );
          
          if (isFromErrorModule) {
            if (this.verbose) {
              console.log(`[DEBUG] 🔍 C014: ${className} imported from error module: ${moduleSpecifier}`);
            }
            return true;
          }
        }
      }
    }
    
    return false;
  }

  isEntityClass(className) {
    // Common entity/model class patterns
    const entityPatterns = [
      'Entity', 'Model', 'Schema', 'Document', 'Dto', 'DTO'
    ];
    
    return entityPatterns.some(pattern => 
      className.endsWith(pattern)
    );
  }

  isCommandPattern(className) {
    // Command pattern classes (value objects for operations)
    const commandPatterns = [
      'Command', 'Request', 'Query', 'Operation', 
      'Action', 'Task', 'Job'
    ];
    
    return commandPatterns.some(pattern => 
      className.endsWith(pattern)
    );
  }

  isServiceLocatorPattern(context, sourceFile) {
    // Check if we're in an object literal assignment that looks like service locator
    if (!context.isLocalVariable) {
      return false;
    }

    // Additional check: if file contains many similar API instantiations, 
    // it's likely a service locator pattern
    const fileText = sourceFile.getFullText();
    const newExpressionCount = (fileText.match(/new \w+Api\(\)/g) || []).length;
    if (newExpressionCount >= 5) {
      // Many API instantiations suggest service locator pattern
      if (this.verbose) {
        console.log(`[DEBUG] 🔍 C014: Found ${newExpressionCount} API instantiations - likely service locator`);
      }
      return true;
    }

    // Check for service locator variable names in file text
    const serviceLocatorPatterns = [
      'apiClient', 'serviceContainer', 'container', 'services',
      'clients', 'providers', 'factories', 'registry'
    ];

    const isServiceLocator = serviceLocatorPatterns.some(pattern => 
      fileText.includes(`export const ${pattern}`) || 
      fileText.includes(`const ${pattern}`)
    );

    if (isServiceLocator && this.verbose) {
      console.log(`[DEBUG] 🔍 C014: Detected service locator pattern from variable name`);
    }

    return isServiceLocator;
  }

  hasDecorators(node, decoratorNames) {
    const decorators = node.getDecorators?.() || [];
    return decorators.some(decorator => {
      const decoratorText = decorator.getText();
      return decoratorNames.some(name => decoratorText.includes(name));
    });
  }

  shouldSkipFile(filePath) {
    return this.config.excludePatterns.some(pattern => {
      const regex = pattern.replace(/\*\*/g, '.*').replace(/\*/g, '[^/]*');
      return new RegExp(regex).test(filePath);
    });
  }

  /**
   * Check if this is a Singleton pattern (self-instantiation)
   */
  isSingletonPattern(className, context, sourceFile) {
    // Must be in a method (not constructor)
    if (!context.isInMethod || context.isInConstructor) {
      return false;
    }

    // Must be instantiating the same class we're in
    const classDeclaration = this.findContainingClass(context, sourceFile);
    if (!classDeclaration) {
      return false;
    }

    const currentClassName = classDeclaration.getName();
    if (currentClassName !== className) {
      return false;
    }

    // Method name should suggest singleton (getInstance, instance, create, etc.)
    const methodName = context.parentFunction?.getName?.() || '';
    const singletonMethods = [
      'getInstance', 'instance', 'getinstance', 'create', 'createInstance',
      'singleton', 'getSingleton', 'getSharedInstance', 'shared'
    ];
    
    const isSingletonMethod = singletonMethods.some(pattern => 
      methodName.toLowerCase().includes(pattern.toLowerCase())
    );

    // Must be a static method
    const isStaticMethod = context.parentFunction?.getModifiers?.()
      ?.some(modifier => modifier.getKind() === SyntaxKind.StaticKeyword) || false;

    if (this.verbose && isSingletonMethod && isStaticMethod) {
      console.log(`[DEBUG] 🔍 C014: Detected Singleton pattern: ${currentClassName}.${methodName}()`);
    }

    return isSingletonMethod && isStaticMethod;
  }

  /**
   * Find the containing class declaration
   */
  findContainingClass(context, sourceFile) {
    let current = context.parentFunction?.getParent();
    
    while (current) {
      if (current.getKind() === SyntaxKind.ClassDeclaration) {
        return current;
      }
      current = current.getParent();
    }
    
    return null;
  }

  calculateConfidence(className, context) {
    let confidence = 0.6; // Base confidence

    // Increase confidence for infrastructure patterns
    if (this.hasInfraPattern(className)) {
      confidence += 0.2;
    }

    // Increase confidence if assigned to this.* (dependency field)
    if (context.isAssignedToThis) {
      confidence += 0.2;
    }

    // Decrease confidence for value objects
    if (this.config.allowedValueObjects.includes(className)) {
      confidence -= 0.3;
    }

    // Decrease confidence if has DI decorators
    if (context.hasDecorators) {
      confidence -= 0.4;
    }

    return Math.max(0.3, Math.min(1.0, confidence));
  }

  buildViolationMessage(expr) {
    const { className, context } = expr;
    
    if (context.isInConstructor && context.isAssignedToThis) {
      return `Direct instantiation of '${className}' in constructor. Consider injecting this dependency instead of creating it directly.`;
    }
    
    if (context.isInMethod && context.isAssignedToThis) {
      return `Direct instantiation of '${className}' assigned to instance field. Consider injecting this dependency.`;
    }
    
    return `Direct instantiation of '${className}'. Consider using dependency injection or factory pattern.`;
  }

  buildSuggestion(expr) {
    const { className, context } = expr;
    
    if (context.isInConstructor) {
      return `Inject ${className} via constructor parameter: constructor(private ${className.toLowerCase()}: ${className})`;
    }
    
    return `Consider injecting ${className} as a dependency or using a factory pattern`;
  }
}

module.exports = C014SymbolBasedAnalyzer;
