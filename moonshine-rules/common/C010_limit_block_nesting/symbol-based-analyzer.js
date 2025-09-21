/**
 * Symbol-based analyzer for C010 - Block Nesting Detection using AST
 * Purpose: Use AST traversal to accurately detect nested block statements
 * Advantage: More accurate than regex, handles complex syntax naturally
 */

const { SyntaxKind } = require('ts-morph');

class C010SymbolBasedAnalyzer {
  constructor(semanticEngine = null) {
    this.ruleId = 'C010';
    this.ruleName = 'Limit Block Nesting (Symbol-Based)';
    this.semanticEngine = semanticEngine;
    this.verbose = false;
    
    // Configuration
    this.maxNestingLevel = 3;
    
    // Block statement kinds that count toward nesting (use ts-morph SyntaxKind)
    this.blockStatementKinds = new Set([
      SyntaxKind.IfStatement,
      SyntaxKind.ForStatement,
      SyntaxKind.ForInStatement,
      SyntaxKind.ForOfStatement,
      SyntaxKind.WhileStatement,
      SyntaxKind.DoStatement,
      SyntaxKind.SwitchStatement,
      SyntaxKind.TryStatement,
      SyntaxKind.CatchClause,
      SyntaxKind.Block
    ]);
    
    // Statements that DON'T count toward nesting
    this.nonNestingKinds = new Set([
      SyntaxKind.FunctionDeclaration,
      SyntaxKind.MethodDeclaration,
      SyntaxKind.ArrowFunction,
      SyntaxKind.FunctionExpression,
      SyntaxKind.Constructor,
      SyntaxKind.GetAccessor,
      SyntaxKind.SetAccessor,
      SyntaxKind.ClassDeclaration,
      SyntaxKind.InterfaceDeclaration,
      SyntaxKind.ObjectLiteralExpression,
      SyntaxKind.ArrayLiteralExpression
    ]);
  }

  async initialize(semanticEngine = null) {
    if (semanticEngine) {
      this.semanticEngine = semanticEngine;
    }
    this.verbose = semanticEngine?.verbose || false;
    
    if (this.verbose) {
      console.log(`[DEBUG] 🔧 C010 Symbol-Based: Analyzer initialized with max nesting level: ${this.maxNestingLevel}`);
    }
  }

  async analyze(files, language, options = {}) {
    const violations = [];
    
    if (process.env.SUNLINT_DEBUG) {
      console.log(`[C010 Symbol-Based] Starting analysis for ${files.length} files`);
      console.log(`[C010 Symbol-Based] Semantic engine available: ${!!this.semanticEngine}`);
      console.log(`[C010 Symbol-Based] Project available: ${!!this.semanticEngine?.project}`);
    }
    
    if (!this.semanticEngine?.project) {
      if (this.verbose || process.env.SUNLINT_DEBUG) {
        console.warn('[C010 Symbol-Based] No semantic engine available, skipping analysis');
      }
      return violations;
    }
    
    for (const filePath of files) {
      try {
        if (process.env.SUNLINT_DEBUG) {
          console.log(`[C010 Symbol-Based] Analyzing file: ${filePath}`);
        }
        const fileViolations = await this.analyzeFileWithSymbols(filePath, options);
        violations.push(...fileViolations);
      } catch (error) {
        if (this.verbose || process.env.SUNLINT_DEBUG) {
          console.warn(`[C010 Symbol-Based] Analysis failed for ${filePath}:`, error.message);
          console.warn(`[C010 Symbol-Based] Error stack:`, error.stack);
        }
      }
    }
    
    if (process.env.SUNLINT_DEBUG) {
      console.log(`[C010 Symbol-Based] Analysis complete: ${violations.length} violations found`);
    }
    
    return violations;
  }

  async analyzeFileWithSymbols(filePath, options = {}) {
    const violations = [];
        const sourceFile = this.semanticEngine.project.getSourceFile(filePath);    if (!sourceFile) {
      if (this.verbose) {
        console.warn(`[C010 Symbol-Based] Source file not found: ${filePath}`);
      }
      return violations;
    }

    // Traverse AST and track nesting depth
    this.traverseNode(sourceFile, 0, violations, filePath);
    
    if (this.verbose) {
      console.log(`[C010 Symbol-Based] Found ${violations.length} violations in ${filePath}`);
    }
    
    return violations;
  }

  /**
   * Recursively traverse AST nodes and track block nesting depth
   */
  traverseNode(node, currentDepth, violations, filePath) {
    const nodeKind = node.getKind();
    
    // Check if this node starts a new nesting level
    let newDepth = currentDepth;
    let isBlockStatement = false;
    
    if (this.isNestingStatement(node)) {
      newDepth = currentDepth + 1;
      isBlockStatement = true;
      
      // Check if nesting exceeds maximum allowed
      if (newDepth > this.maxNestingLevel) {
        const startPos = node.getStart();
        const sourceFile = node.getSourceFile();
        const lineAndChar = sourceFile.getLineAndColumnAtPos(startPos);
        
        violations.push({
          ruleId: this.ruleId,
          severity: 'warning',
          message: `Block nesting is too deep (level ${newDepth}). Maximum allowed is ${this.maxNestingLevel} levels.`,
          filePath: filePath,
          line: lineAndChar.line + 1,
          column: lineAndChar.column + 1,
          context: this.getNodeContext(node)
        });
      }
    }
    
    // Recursively analyze child nodes
    node.forEachChild(child => {
      // Don't increase depth for function boundaries
      if (this.isFunctionBoundary(child)) {
        // Reset depth for function/method/class boundaries
        this.traverseNode(child, 0, violations, filePath);
      } else {
        this.traverseNode(child, newDepth, violations, filePath);
      }
    });
  }

  /**
   * Check if a node represents a statement that increases nesting depth
   */
  isNestingStatement(node) {
    const kind = node.getKind();
    
    // Basic block statements
    if (this.blockStatementKinds.has(kind)) {
      return true;
    }
    
    // Special case: Block statements inside other constructs
    if (kind === SyntaxKind.Block) {
      const parent = node.getParent();
      if (parent && this.blockStatementKinds.has(parent.getKind())) {
        return false; // Block is part of the parent statement
      }
      return true;
    }
    
    return false;
  }

  /**
   * Check if a node represents a function boundary (resets nesting depth)
   */
  isFunctionBoundary(node) {
    return this.nonNestingKinds.has(node.getKind());
  }

  /**
   * Get context information for a violation
   */
  getNodeContext(node) {
    const text = node.getText();
    const lines = text.split('\n');
    const firstLine = lines[0].trim();
    
    // Return first line or statement type
    if (firstLine.length > 0) {
      return firstLine.length > 50 ? firstLine.substring(0, 47) + '...' : firstLine;
    }
    
    // Fallback to node kind
    const kind = node.getKind();
    return SyntaxKind[kind] || 'Unknown';
  }

  /**
   * Get detailed information about nesting violation
   */
  getNestingPath(node) {
    const path = [];
    let current = node.getParent();
    
    while (current && path.length < 10) { // Limit to prevent infinite loops
      if (this.isNestingStatement(current)) {
        const kind = current.getKind();
        const kindName = SyntaxKind[kind] || 'Unknown';
        path.unshift(kindName);
      } else if (this.isFunctionBoundary(current)) {
        break; // Stop at function boundary
      }
      current = current.getParent();
    }
    
    return path;
  }
}

module.exports = C010SymbolBasedAnalyzer;
