/**
 * C024 Symbol-based Analyzer - Advanced Do not scatter hardcoded constants throughout the logic
 * Purpose: The rule prevents scattering hardcoded constants throughout the logic. Instead, constants should be defined in a single place to improve maintainability and readability.
 */

const { SyntaxKind } = require('ts-morph');

class C024SymbolBasedAnalyzer {
  constructor(semanticEngine = null) {
    this.ruleId = 'C024';
    this.ruleName = 'Error Scatter hardcoded constants throughout the logic (Symbol-Based)';
    this.semanticEngine = semanticEngine;
    this.verbose = false;
    this.safeStrings = ["UNKNOWN", "N/A"]; // allowlist of special fallback values
  }

  async initialize(semanticEngine = null) {
    if (semanticEngine) {
      this.semanticEngine = semanticEngine;
    }
    this.verbose = semanticEngine?.verbose || false;

    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔧 [C024 Symbol-Based] Analyzer initialized, verbose: ${this.verbose}`);
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
        console.warn('[C024 Symbol-Based] No semantic engine available, skipping analysis');
      }
      return violations;
    }

    if (verbose) {
      console.log(`🔍 [C024 Symbol-Based] Starting analysis for ${filePath}`);
    }

    try {
      const sourceFile = this.semanticEngine.project.getSourceFile(filePath);
      if (!sourceFile) {
        return violations;
      }

      // skip constants files
      if (this.isConstantsFile(filePath)) return violations;
      // Detect hardcoded constants
      sourceFile.forEachDescendant((node) => {
        this.checkLiterals(node, sourceFile, violations);
        this.checkConstDeclaration(node, sourceFile, violations);
        this.checkStaticReadonly(node, sourceFile, violations);
      });


      if (verbose) {
        console.log(`🔍 [C024 Symbol-Based] Total violations found: ${violations.length}`);
      }

      return violations;
    } catch (error) {
      if (verbose) {
        console.warn(`[C024 Symbol-Based] Analysis failed for ${filePath}:`, error.message);
      }

      return violations;
    }
  }

  // --- push violation object ---
  pushViolation(violations, node, filePath, text, message) {
    violations.push({
      ruleId: this.ruleId,
      severity: "warning",
      message: message || `Hardcoded constant found: "${text}"`,
      source: this.ruleId,
      file: filePath,
      line: node.getStartLineNumber(),
      column: node.getStart() - node.getStartLinePos(),
      description:
        "[SYMBOL-BASED] Hardcoded constants should be defined in a single place to improve maintainability.",
      suggestion: "Define constants in a dedicated file or section",
      category: "constants",
    });
  }

  // --- check literals like "ADMIN", 123, true ---
  checkLiterals(node, sourceFile, violations) {
    const kind = node.getKind();
    if (
      kind === SyntaxKind.StringLiteral ||
      kind === SyntaxKind.NumericLiteral ||
      kind === SyntaxKind.TrueKeyword ||
      kind === SyntaxKind.FalseKeyword
    ) {
      const text = node.getText().replace(/['"`]/g, ""); // strip quotes
      if (this.isAllowedLiteral(node, text)) return;

      this.pushViolation(
        violations,
        node,
        sourceFile.getFilePath(),
        node.getText()
      );
    }
  }

  // --- check const declarations outside constants.ts ---
  checkConstDeclaration(node, sourceFile, violations) {
    const kind = node.getKind();
    if (kind === SyntaxKind.VariableDeclaration) {
      const parentKind = node.getParent()?.getKind();
      if (
        parentKind === SyntaxKind.VariableDeclarationList &&
        node.getParent().getDeclarationKind() === "const"
      ) {
        this.pushViolation(
          violations,
          node,
          sourceFile.getFilePath(),
          node.getName(),
          `Const declaration "${node.getName()}" should be moved into constants file`
        );
      }
    }
  }

  // --- check static readonly properties inside classes ---
  checkStaticReadonly(node, sourceFile, violations) {
    const kind = node.getKind();
    if (kind === SyntaxKind.PropertyDeclaration) {
      const modifiers = node.getModifiers().map((m) => m.getText());
      if (modifiers.includes("static") && modifiers.includes("readonly")) {
        this.pushViolation(
          violations,
          node,
          sourceFile.getFilePath(),
          node.getName(),
          `Static readonly property "${node.getName()}" should be moved into constants file`
        );
      }
    }
  }

  // --- helper: allow safe literals ---
  isAllowedLiteral(node, text) {
    // skip imports
    if (node.getParent()?.getKind() === SyntaxKind.ImportDeclaration) {
      return true;
    }

    // allow short strings
    if (typeof text === "string" && text.length <= 1) return true;

    // allow sentinel numbers
    if (text === "0" || text === "1" || text === "-1") return true;

    // allow known safe strings (like "UNKNOWN")
    if (this.safeStrings.includes(text)) return true;

    return false;
  }

  // helper to check if file is a constants file
  isConstantsFile(filePath) {
    const lower = filePath.toLowerCase();
    return lower.endsWith("constants.ts") || lower.includes("/constants/");
  }
}

module.exports = C024SymbolBasedAnalyzer;
