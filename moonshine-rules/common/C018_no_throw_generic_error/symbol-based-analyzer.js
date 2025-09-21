/**
 * C018 Symbol-based Analyzer - Advanced Do not throw generic errors
 * Purpose: Use AST + Symbol Resolution to analyze log content quality in catch blocks
 */

const { SyntaxKind } = require('ts-morph');

class C018SymbolBasedAnalyzer {
  constructor(semanticEngine = null) {
    this.ruleId = 'C018';
    this.ruleName = 'Error Always provide detailed messages and context. (Symbol-Based)';
    this.semanticEngine = semanticEngine;
    this.verbose = false;

    // Sensitive data patterns to flag (more specific to avoid false positives)
    this.sensitivePatterns = [
      'password', 'passwd', 'pwd', 'pass',
      'token', 'jwt', 'secret', 'privatekey', 'publickey', 'apikey', 'accesskey',
      'ssn', 'social', 'creditcard', 'cardnumber', 'cvv', 'pin',
      'authorization', 'bearer'
    ];

    // Ensure error messages should explain what happened, why, and in what context
    this.explanationPatterns = [
      'because', 'due to', 'failed to', 'cannot', 'invalid', 'missing', 'not found',
    ];
    this.guidancePatterns = [
      'please', 'ensure', 'make sure', 'check', 'try', 'use',
    ];
  }

  async initialize(semanticEngine = null) {
    if (semanticEngine) {
      this.semanticEngine = semanticEngine;
    }
    this.verbose = semanticEngine?.verbose || false;

    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔧 [C018 Symbol-Based] Analyzer initialized, verbose: ${this.verbose}`);
    }
  }

  async analyzeFileBasic(filePath, options = {}) {
    // This is the main entry point called by the hybrid analyzer
    return await this.analyzeFileWithSymbols(filePath, options);
  }

  async analyzeFileWithSymbols(filePath, options = {}) {
    const violations = [];

    // Enable verbose mode if requested
    const verbose = options.verbose || this.verbose;

    if (!this.semanticEngine?.project) {
      if (verbose) {
        console.warn('[C018 Symbol-Based] No semantic engine available, skipping analysis');
      }
      return violations;
    }

    if (verbose) {
      console.log(`🔍 [C018 Symbol-Based] Starting analysis for ${filePath}`);
    }

    try {
      const sourceFile = this.semanticEngine.project.getSourceFile(filePath);
      if (!sourceFile) {
        return violations;
      }

      // Find all try-catch statements in the file
      const tryCatchStatements = sourceFile.getDescendantsOfKind(SyntaxKind.TryStatement);

      if (verbose) {
        console.log(`🔍 [C018 Symbol-Based] Found ${tryCatchStatements.length} try-catch statements`);
      }

      for (const tryStatement of tryCatchStatements) {
        const catchClause = tryStatement.getCatchClause();
        if (catchClause) {
          const catchViolations = this.analyzeCatchBlock(catchClause, sourceFile, filePath, verbose);
          violations.push(...catchViolations);
        }
      }

      if (verbose) {
        console.log(`🔍 [C018 Symbol-Based] Total violations found: ${violations.length}`);
      }

      return violations;
    } catch (error) {
      if (verbose) {
        console.warn(`[C018 Symbol-Based] Analysis failed for ${filePath}:`, error.message);
      }
      return violations;
    }
  }

  /**
   * Analyze catch block for logging context violations
   */
  analyzeCatchBlock(catchClause, sourceFile, filePath, verbose = false) {
    const violations = [];

    if (verbose) {
      console.log(`🔍 [C018 Symbol-Based] Analyzing catch block in ${filePath}`);
    }

    // Get catch parameter (e, error, err, etc.)
    const catchParameter = catchClause.getVariableDeclaration();
    const errorVarName = catchParameter?.getName() || 'e';

    if (verbose) {
      console.log(`🔍 [C018 Symbol-Based] Error variable name: ${errorVarName}`);
    }

    // Find all log calls within catch block
    const catchBlock = catchClause.getBlock();
    const throwStatements = catchBlock.getDescendantsOfKind(SyntaxKind.ThrowStatement);

    if (verbose) {
      console.log(`🔍 [C018 Symbol-Based] Error variable name: ${errorVarName}`);
    }

    if (throwStatements.length === 0) {
      // No logging found - but this is C029's concern, not C018
      // We only analyze existing logs for quality
      return violations;
    }

    // Analyze each log call for context quality
    for (const throwStatement of throwStatements) {
      if (verbose) {
        console.log(`🔍 [C018 Symbol-Based] Analyzing throwStatement call: ${throwStatement.getText()}`);
      }

      const throwViolations = this.analyzeThrowCall(throwStatement, errorVarName, sourceFile, filePath, verbose);
      violations.push(...throwViolations);
    }

    return violations;
  }

  /**
   * Analyze individual log call for context quality
   */
  analyzeThrowCall(throwStatement, errorVarName, sourceFile, filePath, verbose = false) {
    const violations = [];
    const lineNumber = throwStatement.getStartLineNumber();
    const columnNumber = throwStatement.getStart() - throwStatement.getStartLinePos();
    const exp = throwStatement.getExpression();

    if (!exp) {
      return violations; // No arguments to analyze;
    }

    // Case: throw e (identifier)
    if (exp.getKind() === SyntaxKind.Identifier) {
      violations.push({
        ruleId: this.ruleId,
        severity: 'error',
        message: 'Throwing caught error directly without context',
        source: this.ruleId,
        file: filePath,
        line: lineNumber,
        column: columnNumber,
        description: `[SYMBOL-BASED] Caught error thrown directly without additional context. Use structured error objects.`,
        suggestion: 'Use structured error objects with context instead of throwing caught errors directly',
        category: 'error-handling'
      });
    }

    const args = [];

    // Case: throw new Error("...")
    if (exp.getKind() === SyntaxKind.NewExpression) {
      const newExp = exp.asKind(SyntaxKind.NewExpression);
      const arg = newExp.getArguments().map(arg => arg);

      args.push(...arg);
    }

    const analysis = this.analyzeThrowArguments(args, errorVarName, verbose);

    // Analyze throw structure and content

    // Check for violations
    if (!analysis.isStructured) {
      violations.push({
        ruleId: this.ruleId,
        severity: 'warning',
        message: 'Error logging should use structured format (object) instead of string concatenation',
        source: this.ruleId,
        file: filePath,
        line: lineNumber,
        column: columnNumber,
        description: `[SYMBOL-BASED] Non-structured logging detected. Use object format for better parsing and monitoring.`,
        suggestion: 'Use logger.error("message", { error: e.message, context: {...} }) instead of string concatenation',
        category: 'error-handling'
      });
    }

    if (analysis.hasSensitiveData) {
      violations.push({
        ruleId: this.ruleId,
        severity: 'error',
        message: 'Error logging contains potentially sensitive data',
        source: this.ruleId,
        file: filePath,
        line: lineNumber,
        column: columnNumber,
        description: `[SYMBOL-BASED] Sensitive patterns detected: ${analysis.sensitivePatterns.join(', ')}. Mask or exclude sensitive data.`,
        suggestion: 'Mask sensitive data: password.substring(0,2) + "***" or exclude entirely',
        category: 'security'
      });
    }

    if (!analysis.hasExplanation) {
      violations.push({
        ruleId: this.ruleId,
        severity: 'warning',
        message: 'Error logging should explain what happened',
        source: this.ruleId,
        file: filePath,
        line: lineNumber,
        column: columnNumber,
        description: `[SYMBOL-BASED] Error message should explain what happened, why, and in what context.`,
        suggestion: 'Use structured error objects with context: { message: "Error occurred", context: "Request failed because todo something." } }',
        category: 'error-handling'
      });
    }

    if (!analysis.hasGuidance) {
      violations.push({
        ruleId: this.ruleId,
        severity: 'warning',
        message: 'Error logging should provide guidance on what to do next',
        source: this.ruleId,
        file: filePath,
        line: lineNumber,
        column: columnNumber,
        description: `[SYMBOL-BASED] Error message should provide guidance on what to do next.`,
        suggestion: 'Use structured error objects with guidance: { message: "Error occurred", guidance: "Please check the input data and try again." }',
        category: 'error-handling'
      });
    }

    return violations;
  }

  /**
   * Analyze log arguments for structure, context, and sensitive data
   */
  analyzeThrowArguments(args, errorVarName, verbose = false) {
    const analysis = {
      isStructured: false,
      hasSensitiveData: false,
      hasExplanation: false,
      hasGuidance: false,
      sensitivePatterns: []
    };

    // Check if any argument is an object (structured logging)
    for (const arg of args) {
      if (arg.getKind() === SyntaxKind.ObjectLiteralExpression) {
        analysis.isStructured = true;
        analysis.hasExplanation = true; // Assume structured logs have explanations
        analysis.hasGuidance = true; // Assume structured logs have guidance
        break;
      }
    }

    // If not structured, check for string concatenation patterns
    if (!analysis.isStructured) {
      for (const arg of args) {
        const argText = arg.getText().toLowerCase();
        this.validateForSensitiveDataInText(argText, analysis);
        this.validateErrorMessage(argText, analysis);
      }
    }

    return analysis;
  }

  /**
   * Check text for sensitive data patterns
   */
  validateForSensitiveDataInText(text, analysis) {
    for (const pattern of this.sensitivePatterns) {
      if (text.includes(pattern)) {
        analysis.hasSensitiveData = true;
        analysis.sensitivePatterns.push(pattern);
      }
    }
  }

  validateErrorMessage(text, analysis) {
    // Rule 1: Explanation
    for (const patternE of this.explanationPatterns) {
      if (text.includes(patternE)) {
        analysis.hasExplanation = true;
      }
    }

    // Rule 2: Guidance
    for (const patternG of this.guidancePatterns) {
      if (text.includes(patternG)) {
        analysis.hasGuidance = true;
      }
    }
  }
}

module.exports = C018SymbolBasedAnalyzer;
