/**
 * C017 Constructor Logic - Semantic Analyzer (Phase 2)
 * Uses ts-morph for precise symbol-based analysis
 * Detects complex logic in constructors with high accuracy
 */

const SemanticRuleBase = require('../../../core/semantic-rule-base');
const path = require('path');

class C017SemanticAnalyzer extends SemanticRuleBase {
  constructor(ruleId = 'C017') {
    super(ruleId);
    this.ruleName = 'C017 - No Complex Logic in Constructors (Semantic)';
    this.description = 'Constructors should only handle dependency injection and simple initialization';
  }

  /**
   * Analyze a single file using ts-morph semantic analysis
   * @param {string} filePath - Path to the file to analyze
   * @param {Object} options - Analysis options
   */
  async analyzeFile(filePath, options = {}) {
    if (!this.semanticEngine) {
      throw new Error('Semantic engine not initialized');
    }

    try {
      // Get file's absolute path
      const absolutePath = path.resolve(filePath);
      
      // Get source file from semantic engine's project
      const sourceFile = this.semanticEngine.project.getSourceFile(absolutePath);
      if (!sourceFile) {
        if (options.verbose) {
          console.log(`⚠️ [C017-Semantic] Could not load source file: ${filePath}`);
        }
        return;
      }

      // Find all class declarations
      const classes = sourceFile.getClasses();
      
      for (const classDecl of classes) {
        const constructors = classDecl.getConstructors();
        
        for (const constructor of constructors) {
          await this.analyzeConstructor(constructor, filePath, options);
        }
      }

    } catch (error) {
      if (options.verbose) {
        console.warn(`⚠️ [C017-Semantic] Error analyzing ${filePath}:`, error.message);
      }
    }
  }

  /**
   * Analyze a constructor node for complex logic
   * @param {ts.ConstructorDeclaration} constructor - Constructor node
   * @param {string} filePath - File path
   * @param {Object} options - Analysis options
   */
  async analyzeConstructor(constructor, filePath, options) {
    const body = constructor.getBody();
    if (!body) {
      // Constructor without body (interface, abstract, etc.)
      return;
    }

    const statements = body.getStatements();
    
    for (const statement of statements) {
      const violation = this.analyzeStatement(statement, constructor, filePath);
      if (violation) {
        this.addViolation(violation);
      }
    }
  }

  /**
   * Analyze a statement for complex logic patterns
   * @param {ts.Statement} statement - Statement node
   * @param {ts.ConstructorDeclaration} constructor - Parent constructor
   * @param {string} filePath - File path
   * @returns {Object|null} Violation object or null
   */
  analyzeStatement(statement, constructor, filePath) {
    const statementKind = statement.getKind();
    const line = statement.getStartLineNumber();
    const column = statement.getStart() - statement.getStartLinePos() + 1;

    // Check for complex logic patterns
    switch (statementKind) {
      case this.SyntaxKind.IfStatement:
        return this.createViolation(
          filePath, line, column,
          'Conditional logic (if statement) found in constructor',
          'conditional_logic',
          statement.getText()
        );

      case this.SyntaxKind.ForStatement:
      case this.SyntaxKind.ForInStatement:
      case this.SyntaxKind.ForOfStatement:
      case this.SyntaxKind.WhileStatement:
      case this.SyntaxKind.DoStatement:
        return this.createViolation(
          filePath, line, column,
          'Loop logic found in constructor',
          'loop_logic',
          statement.getText()
        );

      case this.SyntaxKind.SwitchStatement:
        return this.createViolation(
          filePath, line, column,
          'Switch statement found in constructor',
          'switch_logic',
          statement.getText()
        );

      case this.SyntaxKind.TryStatement:
        return this.createViolation(
          filePath, line, column,
          'Exception handling (try/catch) found in constructor',
          'exception_handling',
          statement.getText()
        );

      case this.SyntaxKind.ExpressionStatement:
        return this.analyzeExpressionStatement(statement, filePath);

      default:
        return null;
    }
  }

  /**
   * Analyze expression statements for complex patterns
   * @param {ts.ExpressionStatement} statement - Expression statement
   * @param {string} filePath - File path
   * @returns {Object|null} Violation object or null
   */
  analyzeExpressionStatement(statement, filePath) {
    const expression = statement.getExpression();
    const line = statement.getStartLineNumber();
    const column = statement.getStart() - statement.getStartLinePos() + 1;

    // Check for async operations
    if (this.isAsyncOperation(expression)) {
      return this.createViolation(
        filePath, line, column,
        'Asynchronous operation found in constructor',
        'async_operation',
        statement.getText()
      );
    }

    // Check for complex method calls
    if (this.isComplexMethodCall(expression)) {
      return this.createViolation(
        filePath, line, column,
        'Complex method call found in constructor',
        'complex_method_call',
        statement.getText()
      );
    }

    // Allow simple assignments and DI setup
    if (this.isSimpleAssignment(expression) || this.isConfigurationSetup(expression)) {
      return null;
    }

    return null;
  }

  /**
   * Check if expression is an async operation
   * @param {ts.Expression} expression - Expression node
   * @returns {boolean} True if async operation
   */
  isAsyncOperation(expression) {
    const text = expression.getText();
    
    // Await expressions
    if (expression.getKind() === this.SyntaxKind.AwaitExpression) {
      return true;
    }

    // Promise chains
    if (text.includes('.then(') || text.includes('.catch(') || text.includes('.finally(')) {
      return true;
    }

    return false;
  }

  /**
   * Check if expression is a complex method call
   * @param {ts.Expression} expression - Expression node
   * @returns {boolean} True if complex method call
   */
  isComplexMethodCall(expression) {
    if (expression.getKind() !== this.SyntaxKind.CallExpression) {
      return false;
    }

    const callExpr = expression;
    const methodName = this.getMethodName(callExpr);

    // Allow certain initialization methods
    const allowedMethods = [
      'makeObservable', 'makeAutoObservable', // MobX
      'bind', 'bindAll', // Method binding
      'Object.assign', 'Object.create', // Object utilities
      'Array.from', 'Array.of', // Array utilities
    ];

    if (allowedMethods.some(method => methodName.includes(method))) {
      return false;
    }

    // Check for chained calls (more than 2 levels = complex)
    const chainLength = this.getChainLength(callExpr);
    return chainLength > 2;
  }

  /**
   * Check if expression is a simple assignment
   * @param {ts.Expression} expression - Expression node
   * @returns {boolean} True if simple assignment
   */
  isSimpleAssignment(expression) {
    if (expression.getKind() !== this.SyntaxKind.BinaryExpression) {
      return false;
    }

    const binaryExpr = expression;
    const operator = binaryExpr.getOperatorToken();
    
    return operator.getKind() === this.SyntaxKind.EqualsToken;
  }

  /**
   * Check if expression is configuration setup
   * @param {ts.Expression} expression - Expression node
   * @returns {boolean} True if configuration setup
   */
  isConfigurationSetup(expression) {
    const text = expression.getText();
    
    // Configuration patterns
    const configPatterns = [
      /new\s+\w+Client\s*\(/,  // AWS clients, HTTP clients
      /new\s+\w+\s*\(\s*\{/,   // Configuration objects
      /\.getInstance\s*\(/,     // Singleton patterns
      /\.create\s*\(/,         // Factory patterns
    ];

    return configPatterns.some(pattern => pattern.test(text));
  }

  /**
   * Get method name from call expression
   * @param {ts.CallExpression} callExpr - Call expression
   * @returns {string} Method name
   */
  getMethodName(callExpr) {
    const expression = callExpr.getExpression();
    
    if (expression.getKind() === this.SyntaxKind.PropertyAccessExpression) {
      return expression.getName();
    }
    
    if (expression.getKind() === this.SyntaxKind.Identifier) {
      return expression.getText();
    }
    
    return expression.getText();
  }

  /**
   * Get chain length of method calls
   * @param {ts.CallExpression} callExpr - Call expression
   * @returns {number} Chain length
   */
  getChainLength(callExpr) {
    let current = callExpr.getExpression();
    let length = 1;
    
    while (current.getKind() === this.SyntaxKind.PropertyAccessExpression) {
      const propAccess = current;
      current = propAccess.getExpression();
      length++;
    }
    
    return length;
  }

  /**
   * Create a violation object
   * @param {string} filePath - File path
   * @param {number} line - Line number
   * @param {number} column - Column number
   * @param {string} message - Violation message
   * @param {string} type - Violation type
   * @param {string} code - Code snippet
   * @returns {Object} Violation object
   */
  createViolation(filePath, line, column, message, type, code) {
    return {
      ruleId: this.ruleId,
      file: filePath,
      line: line,
      column: column,
      message: `Constructor contains complex logic: ${message}. Move to initialization methods`,
      severity: 'warning',
      code: code.trim(),
      type: type,
      confidence: 95, // High confidence with semantic analysis
      analysisMethod: 'semantic',
      suggestion: 'Move complex logic to separate initialization methods or lifecycle hooks'
    };
  }

  /**
   * Get SyntaxKind enum from ts-morph
   * @returns {Object} SyntaxKind enum
   */
  get SyntaxKind() {
    if (!this.semanticEngine) {
      throw new Error('Semantic engine not initialized');
    }
    return this.semanticEngine.project.getTypeChecker().compilerObject.SyntaxKind || 
           require('typescript').SyntaxKind;
  }
}

module.exports = C017SemanticAnalyzer;
