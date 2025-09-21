/**
 * Symbol-based analyzer for C033 - Advanced semantic analysis
 * Purpose: Use AST + Data Flow to detect true database access violations
 */

class C033SymbolBasedAnalyzer {
  constructor(semanticEngine = null) {
    this.ruleId = 'C033';
    this.ruleName = 'Separate Service and Repository Logic (Symbol-Based)';
    this.semanticEngine = semanticEngine;
    this.verbose = false;
    
    // Known database/ORM symbols and interfaces
    this.databaseSymbols = [
      'Repository', 'EntityManager', 'QueryBuilder', 'Connection',
      'PrismaClient', 'Model', 'Collection', 'Table'
    ];
    
    // ORM framework patterns
    this.ormPatterns = {
      typeorm: ['Repository', 'EntityManager', 'QueryRunner', 'QueryBuilder'],
      prisma: ['PrismaClient', 'PrismaService'],
      mongoose: ['Model', 'Document', 'Schema'],
      sequelize: ['Model', 'Sequelize', 'QueryInterface'],
      knex: ['Knex', 'QueryBuilder']
    };
  }

  async initialize(semanticEngine = null) {
    if (semanticEngine) {
      this.semanticEngine = semanticEngine;
    }
    this.verbose = semanticEngine?.verbose || false;
    
    if (this.verbose) {
      console.log(`[DEBUG] 🔧 C033 Symbol-Based: Analyzer initialized`);
    }
  }

  async analyze(files, language, options = {}) {
    const violations = [];
    
    if (!this.semanticEngine?.project) {
      if (this.verbose) {
        console.warn('[C033 Symbol-Based] No semantic engine available, skipping analysis');
      }
      return violations;
    }
    
    for (const filePath of files) {
      try {
        const fileViolations = await this.analyzeFileWithSymbols(filePath, options);
        violations.push(...fileViolations);
      } catch (error) {
        if (this.verbose) {
          console.warn(`[C033 Symbol-Based] Analysis failed for ${filePath}:`, error.message);
        }
      }
    }
    
    return violations;
  }

  async analyzeFileWithSymbols(filePath, options = {}) {
    const violations = [];
    const sourceFile = this.semanticEngine.project.getSourceFileByFilePath(filePath);
    
    if (!sourceFile) {
      return violations;
    }

    // 1. Classify file type using semantic analysis
    const fileType = this.classifyFileSemanticType(sourceFile, filePath);
    
    if (fileType !== 'service') {
      return violations; // Only analyze Service files
    }

    // 2. Analyze call expressions in Service classes
    const classes = sourceFile.getClasses();
    
    for (const cls of classes) {
      const className = cls.getName() || 'UnknownClass';
      
      // Skip if not a Service class
      if (!this.isServiceClass(cls)) {
        continue;
      }
      
      const methods = cls.getMethods();
      
      for (const method of methods) {
        const methodViolations = this.analyzeMethodForDatabaseCalls(
          method, sourceFile, className, filePath
        );
        violations.push(...methodViolations);
      }
    }
    
    return violations;
  }

  /**
   * Analyze method for direct database calls using symbol resolution
   */
  analyzeMethodForDatabaseCalls(method, sourceFile, className, filePath) {
    const violations = [];
    const methodName = method.getName();
    
    // Get all call expressions in the method
    const callExpressions = method.getDescendantsOfKind(this.getKind('CallExpression'));
    
    for (const callExpr of callExpressions) {
      const violation = this.analyzeCallExpression(callExpr, sourceFile, className, methodName, filePath);
      if (violation) {
        violations.push(violation);
      }
    }
    
    return violations;
  }

  /**
   * Analyze individual call expression using symbol resolution
   */
  analyzeCallExpression(callExpr, sourceFile, className, methodName, filePath) {
    const expression = callExpr.getExpression();
    
    // Handle property access (obj.method())
    if (expression.getKind() === this.getKind('PropertyAccessExpression')) {
      const propertyAccess = expression;
      const object = propertyAccess.getExpression();
      const property = propertyAccess.getName();
      
      // Get the symbol/type of the object being called
      const objectSymbol = this.getObjectSymbol(object);
      
      if (this.isDatabaseOperation(objectSymbol, property)) {
        // Check if it's going through repository (acceptable)
        if (this.isRepositoryAccess(objectSymbol)) {
          return null; // OK: Service -> Repository -> Database
        }
        
        // Direct database access in Service (violation)
        const lineNumber = callExpr.getStartLineNumber();
        const columnNumber = callExpr.getStart() - sourceFile.getLineStartPos(lineNumber - 1) + 1;
        
        return {
          ruleId: this.ruleId,
          severity: 'warning',
          message: `Service should not contain direct database calls`,
          source: this.ruleId,
          file: filePath,
          line: lineNumber,
          column: columnNumber,
          description: `[SYMBOL-BASED] Direct database call '${property}()' on '${objectSymbol?.name || 'unknown'}' found in Service`,
          suggestion: 'Use Repository pattern for data access',
          category: 'architecture'
        };
      }
    }
    
    return null;
  }

  /**
   * Get symbol information for an object expression
   */
  getObjectSymbol(objectExpr) {
    try {
      // Try to get the symbol of the expression
      const symbol = objectExpr.getSymbol();
      if (symbol) {
        return {
          name: symbol.getName(),
          type: this.getSymbolType(symbol),
          isDatabase: this.isSymbolDatabase(symbol)
        };
      }
      
      // Fallback: analyze by text patterns
      const text = objectExpr.getText();
      return {
        name: text,
        type: this.inferTypeFromText(text),
        isDatabase: this.isTextDatabase(text)
      };
    } catch (error) {
      return null;
    }
  }

  /**
   * Check if operation is a database operation
   */
  isDatabaseOperation(objectSymbol, methodName) {
    if (!objectSymbol) return false;
    
    // Exclude queue/job operations (Bull.js, agenda, etc.)
    if (this.isQueueOperation(objectSymbol, methodName)) {
      return false;
    }
    
    // Known database method patterns
    const dbMethods = [
      'findOneBy', 'findBy', 'findAndCount', 'findMany', 'findFirst',
      'save', 'insert', 'create', 'upsert',
      'update', 'patch', 'merge', 'set',
      'delete', 'remove', 'destroy',
      'query', 'execute', 'run',
      'createQueryBuilder', 'getRepository'
    ];
    
    return objectSymbol.isDatabase && dbMethods.includes(methodName);
  }

  /**
   * Check if this is a queue/job operation (should be excluded)
   */
  isQueueOperation(objectSymbol, methodName) {
    if (!objectSymbol) return false;
    
    const queueMethods = [
      'remove', 'isFailed', 'isCompleted', 'isActive', 'isWaiting', 'isDelayed',
      'getJob', 'getJobs', 'add', 'process', 'on', 'off',
      'retry', 'moveToCompleted', 'moveToFailed'
    ];
    
    const queueTypes = ['queue', 'job', 'bull'];
    const objectName = objectSymbol.name.toLowerCase();
    
    // Enhanced detection for Bull.js Job objects
    const isQueueMethod = queueMethods.includes(methodName);
    const isQueueObject = queueTypes.some(type => objectName.includes(type)) ||
                          /job/i.test(objectName) || 
                          /queue/i.test(objectName);
    
    if (this.verbose && (isQueueMethod || isQueueObject)) {
      console.log(`[DEBUG] Queue Check: object="${objectName}", method="${methodName}", isQueue=${isQueueMethod && isQueueObject}`);
    }
    
    return isQueueMethod && isQueueObject;
  }

  /**
   * Check if access is through repository (acceptable)
   */
  isRepositoryAccess(objectSymbol) {
    if (!objectSymbol) return false;
    
    const name = objectSymbol.name.toLowerCase();
    return name.includes('repository') || name.includes('repo');
  }

  /**
   * Check if symbol represents database object
   */
  isSymbolDatabase(symbol) {
    try {
      const type = symbol.getType();
      const typeName = type.getSymbol()?.getName() || '';
      
      return this.databaseSymbols.some(dbSymbol => 
        typeName.includes(dbSymbol)
      );
    } catch (error) {
      return false;
    }
  }

  /**
   * Infer type from text patterns (fallback)
   */
  inferTypeFromText(text) {
    const lowerText = text.toLowerCase();
    
    // Check for known database object patterns
    if (/manager|connection|client|prisma/i.test(lowerText)) {
      return 'database';
    }
    
    if (/repository|repo/i.test(lowerText)) {
      return 'repository';
    }
    
    return 'unknown';
  }

  /**
   * Check if text represents database access
   */
  isTextDatabase(text) {
    const lowerText = text.toLowerCase();
    return /manager|connection|client|prisma|entitymanager/i.test(lowerText) &&
           !/repository|repo/i.test(lowerText);
  }

  /**
   * Classify file type using semantic analysis
   */
  classifyFileSemanticType(sourceFile, filePath) {
    const fileName = sourceFile.getBaseName().toLowerCase();
    
    // Check filename patterns
    if (/service\.ts$|service\.js$/i.test(fileName)) return 'service';
    if (/repository\.ts$|repository\.js$/i.test(fileName)) return 'repository';
    
    // Check class patterns
    const classes = sourceFile.getClasses();
    for (const cls of classes) {
      if (this.isServiceClass(cls)) return 'service';
      if (this.isRepositoryClass(cls)) return 'repository';
    }
    
    return 'unknown';
  }

  /**
   * Check if class is a Service class
   */
  isServiceClass(cls) {
    const className = cls.getName()?.toLowerCase() || '';
    
    // Check class name
    if (/service$/.test(className)) return true;
    
    // Check decorators
    const decorators = cls.getDecorators();
    return decorators.some(decorator => {
      const decoratorName = decorator.getName().toLowerCase();
      return decoratorName.includes('service') || decoratorName === 'injectable';
    });
  }

  /**
   * Check if class is a Repository class
   */
  isRepositoryClass(cls) {
    const className = cls.getName()?.toLowerCase() || '';
    return /repository$|repo$/.test(className);
  }

  /**
   * Get TypeScript SyntaxKind
   */
  getKind(kindName) {
    try {
      const ts = require('typescript');
      return ts.SyntaxKind[kindName];
    } catch (error) {
      // Fallback for ts-morph
      return this.semanticEngine?.project?.getTypeChecker()?.compilerObject?.SyntaxKind?.[kindName] || 0;
    }
  }

  /**
   * Get symbol type information
   */
  getSymbolType(symbol) {
    try {
      return symbol.getType().getText();
    } catch (error) {
      return 'unknown';
    }
  }
}

module.exports = C033SymbolBasedAnalyzer;
