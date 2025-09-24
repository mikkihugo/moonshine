/**
 * C023 Symbol-based Analyzer - Advanced Do not declare duplicate variable
 * Purpose: Use AST + Symbol Resolution to analyze log content quality in catch blocks
 */

const { SyntaxKind } = require('ts-morph');

class C023SymbolBasedAnalyzer {
  constructor(semanticEngine = null) {
    this.ruleId = 'C023';
    this.ruleName = 'Error declare duplicate variable names in the same scope. (Symbol-Based)';
    this.semanticEngine = semanticEngine;
    this.verbose = false;
  }

  async initialize(semanticEngine = null) {
    if (semanticEngine) {
      this.semanticEngine = semanticEngine;
    }
    this.verbose = semanticEngine?.verbose || false;

    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔧 [C023 Symbol-Based] Analyzer initialized, verbose: ${this.verbose}`);
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
        console.warn('[C023 Symbol-Based] No semantic engine available, skipping analysis');
      }
      return violations;
    }

    if (verbose) {
      console.log(`🔍 [C023 Symbol-Based] Starting analysis for ${filePath}`);
    }

    try {
      const sourceFile = this.semanticEngine.project.getSourceFile(filePath);
      if (!sourceFile) {
        return violations;
      }
      this.checkScope(sourceFile, violations);
      if (verbose) {
        console.log(`🔍 [C023 Symbol-Based] Total violations found: ${violations.length}`);
      }

      return violations;
    } catch (error) {
      if (verbose) {
        console.warn(`[C023 Symbol-Based] Analysis failed for ${filePath}:`, error.message);
      }

      return violations;
    }
  }

  checkScope(node, violations) {
    const seen = new Map();

    node.forEachChild((child) => {
      switch (child.getKind()) {
        case SyntaxKind.VariableStatement: {
          for (const decl of child.getDeclarations()) {
            this.checkDuplicate(decl, seen, violations);
          }
          break;
        }

        case SyntaxKind.FunctionDeclaration:
        case SyntaxKind.FunctionExpression:
        case SyntaxKind.ArrowFunction:
        case SyntaxKind.MethodDeclaration:
        case SyntaxKind.Constructor: { // ✅ also cover constructors
          const func = child;
          // Params
          func.getParameters().forEach((p) =>
            this.checkDuplicate(p, seen, violations)
          );
          // Body as new scope
          const body = func.getBody?.();
          if (body) this.checkScope(body, violations);
          break;
        }

        case SyntaxKind.Block: {
          this.checkScope(child, violations);
          break;
        }

        case SyntaxKind.CatchClause: {
          const catchVar = child.getVariableDeclaration();
          if (catchVar) {
            // ✅ register catch param in scope
            this.checkDuplicate(catchVar, seen, violations);
          }
          // Traverse catch block (same scope as catch param)
          this.checkScope(child.getBlock(), violations);
          break;
        }

        case SyntaxKind.ForStatement:
        case SyntaxKind.ForOfStatement:
        case SyntaxKind.ForInStatement: {
          const initializer = child.getInitializer?.();
          if (initializer) {
            initializer.getDeclarations?.().forEach((decl) =>
              this.checkDuplicate(decl, seen, violations)
            );
          }
          const statement = child.getStatement?.();
          if (statement) this.checkScope(statement, violations);
          break;
        }

        default: {
          this.checkScope(child, violations);
        }
      }
    });
  }

  checkDuplicate(node, seen, violations) {
    const name = node.getName?.();
    if (!name) return;
    const filePath = node.getSourceFile().getFilePath();

    if (seen.has(name)) {
      violations.push({
        ruleId: this.ruleId,
        severity: 'warning',
        message: `Duplicate variable name found: "${name}"`,
        source: this.ruleId,
        file: filePath,
        line: node.getStartLineNumber(),
        column: node.getStart() - node.getStartLinePos(),
        description: `[SYMBOL-BASED] Duplicate variable names can lead to confusion and bugs. Use unique names.`,
        suggestion: 'Rename variables to ensure uniqueness',
        category: 'naming'
      });
    } else {
      seen.set(name, node);
    }
  }
}

module.exports = C023SymbolBasedAnalyzer;
