const { Project, SyntaxKind } = require("ts-morph");

/**
 * S017 Symbol-Based Analyzer - Always use parameterized queries
 * Uses semantic analysis to detect SQL injection vulnerabilities
 */
class S017SymbolBasedAnalyzer {
  constructor(semanticEngine = null) {
    this.ruleId = "S017";
    this.ruleName = "Always use parameterized queries";
    this.semanticEngine = semanticEngine;
    this.verbose = false;
    this.debug = process.env.SUNLINT_DEBUG === "1";

    // SQL execution methods
    this.sqlMethods = [
      "query",
      "execute",
      "exec",
      "run",
      "all",
      "get",
      "prepare",
      "createQuery",
      "executeQuery",
      "executeSql",
      "rawQuery",
    ];

    // SQL keywords that indicate SQL operations
    this.sqlKeywords = [
      "SELECT",
      "INSERT",
      "UPDATE",
      "DELETE",
      "DROP",
      "CREATE",
      "ALTER",
      "UNION",
      "WHERE",
      "ORDER BY",
      "GROUP BY",
      "HAVING",
      "FROM",
      "JOIN",
      "INNER JOIN",
      "LEFT JOIN",
      "RIGHT JOIN",
      "FULL JOIN",
    ];

    // Database libraries to look for
    this.databaseLibraries = [
      "mysql",
      "mysql2",
      "pg",
      "postgres",
      "sqlite3",
      "sqlite",
      "mssql",
      "tedious",
      "oracle",
      "mongodb",
      "mongoose",
      "sequelize",
      "typeorm",
      "prisma",
      "knex",
      "objection",
    ];

    // Safe patterns that indicate parameterized queries
    this.safePatterns = [
      "\\?",
      "\\$1",
      "\\$2",
      "\\$3",
      "\\$4",
      "\\$5",
      "prepare",
      "bind",
      "params",
      "parameters",
      "values",
    ];

    if (this.debug) {
      console.log(
        `🔧 [S017-Symbol] Constructor - databaseLibraries:`,
        this.databaseLibraries.length
      );
      console.log(
        `🔧 [S017-Symbol] Constructor - sqlMethods:`,
        this.sqlMethods.length
      );
    }
  }

  /**
   * Initialize with semantic engine
   */
  async initialize(semanticEngine = null) {
    if (semanticEngine) {
      this.semanticEngine = semanticEngine;
      this.verbose = semanticEngine.verbose || false;
    }

    if (this.verbose) {
      console.log(
        `🔧 [S017 Symbol-Based] Analyzer initialized, verbose: ${this.verbose}`
      );
    }
  }

  /**
   * Analyze file using symbol information
   */
  async analyzeFile(filePath, fileContent) {
    if (this.debug) {
      console.log(`🔍 [S017-Symbol] Analyzing: ${filePath}`);
    }

    const violations = [];
    const violationMap = new Map(); // Track unique violations

    try {
      const project = new Project({
        useInMemoryFileSystem: true,
        compilerOptions: {
          allowJs: true,
          target: "ES2020",
        },
      });

      const sourceFile = project.createSourceFile(filePath, fileContent);

      // Find database-related imports
      const dbImports = this.findDatabaseImports(sourceFile);

      if (this.debug) {
        console.log(
          `🔍 [S017-Symbol] Found ${dbImports.length} database imports:`,
          dbImports.map((i) => i.module)
        );
      }

      if (dbImports.length === 0 && this.debug) {
        console.log(
          `ℹ️ [S017-Symbol] No database imports found in ${filePath}`
        );
      }

      // Analyze method calls in context of database usage
      const violations1 = this.analyzeMethodCallsWithContext(
        sourceFile,
        filePath,
        dbImports
      );
      this.addUniqueViolations(violations1, violationMap);

      // Analyze variable assignments that might contain SQL
      const violations2 = this.analyzeSqlVariableAssignments(
        sourceFile,
        filePath
      );
      this.addUniqueViolations(violations2, violationMap);

      // Analyze function parameters that might be SQL queries
      const violations3 = this.analyzeFunctionParameters(sourceFile, filePath);
      this.addUniqueViolations(violations3, violationMap);

      // Always analyze SQL patterns regardless of imports (catch cases without explicit DB imports)
      const violations4 = this.analyzeUniversalSqlPatterns(
        sourceFile,
        filePath
      );
      this.addUniqueViolations(violations4, violationMap);

      // Convert map to array
      violations.push(...Array.from(violationMap.values()));

      if (this.debug) {
        console.log(
          `🔍 [S017-Symbol] Found ${violations.length} unique violations in ${filePath}`
        );
      }
    } catch (error) {
      if (this.debug) {
        console.error(`❌ [S017-Symbol] Error analyzing ${filePath}:`, error);
      }
    }

    return violations;
  }

  /**
   * Add violations to map, avoiding duplicates
   */
  addUniqueViolations(newViolations, violationMap) {
    newViolations.forEach((v) => {
      const key = `${v.line}:${v.column}:${v.message}`;
      if (!violationMap.has(key)) {
        violationMap.set(key, v);
      }
    });
  }

  /**
   * Find database-related imports
   */
  findDatabaseImports(sourceFile) {
    const imports = [];

    sourceFile.forEachDescendant((node) => {
      // Check for ES6 imports
      if (node.getKind() === SyntaxKind.ImportDeclaration) {
        const importDecl = node;
        const moduleSpecifier = importDecl.getModuleSpecifierValue();

        if (this.databaseLibraries.includes(moduleSpecifier)) {
          imports.push({
            module: moduleSpecifier,
            node: importDecl,
            line: importDecl.getStartLineNumber(),
          });
        }
      }

      // Check for CommonJS require() calls
      if (node.getKind() === SyntaxKind.CallExpression) {
        const callExpr = node;
        const expression = callExpr.getExpression();

        if (
          expression.getKind() === SyntaxKind.Identifier &&
          expression.getText() === "require"
        ) {
          const args = callExpr.getArguments();
          if (args.length > 0) {
            const firstArg = args[0];
            if (firstArg.getKind() === SyntaxKind.StringLiteral) {
              const moduleSpecifier = firstArg.getLiteralValue();

              if (this.databaseLibraries.includes(moduleSpecifier)) {
                imports.push({
                  module: moduleSpecifier,
                  node: callExpr,
                  line: callExpr.getStartLineNumber(),
                });
              }
            }
          }
        }
      }
    });

    return imports;
  }

  /**
   * Analyze method calls with database context
   */
  analyzeMethodCallsWithContext(sourceFile, filePath, dbImports) {
    const violations = [];

    sourceFile.forEachDescendant((node) => {
      if (node.getKind() === SyntaxKind.CallExpression) {
        const callExpr = node;
        const methodName = this.getMethodName(callExpr);

        if (this.sqlMethods.includes(methodName)) {
          const args = callExpr.getArguments();

          if (args.length > 0) {
            const sqlArg = args[0];
            const vulnerability = this.analyzeSqlArgument(sqlArg, methodName);

            if (vulnerability) {
              violations.push({
                ruleId: this.ruleId,
                severity: "error",
                message: vulnerability.message,
                source: this.ruleId,
                file: filePath,
                line: callExpr.getStartLineNumber(),
                column: sqlArg.getStart(),
                evidence: this.getEvidenceText(callExpr),
                suggestion: vulnerability.suggestion,
                category: "security",
              });

              if (this.debug) {
                console.log(
                  `🚨 [S017-Symbol] Vulnerability in ${methodName} at line ${callExpr.getStartLineNumber()}`
                );
              }
            }
          }
        }
      }
    });

    return violations;
  }

  /**
   * Analyze SQL variable assignments
   */
  analyzeSqlVariableAssignments(sourceFile, filePath) {
    const violations = [];

    sourceFile.forEachDescendant((node) => {
      if (node.getKind() === SyntaxKind.VariableDeclaration) {
        const varDecl = node;
        const initializer = varDecl.getInitializer();

        if (initializer) {
          const vulnerability = this.checkForSqlConstruction(initializer);

          if (vulnerability) {
            violations.push({
              ruleId: this.ruleId,
              severity: "error",
              message: `SQL injection risk in variable assignment: ${vulnerability.message}`,
              source: this.ruleId,
              file: filePath,
              line: varDecl.getStartLineNumber(),
              column: initializer.getStart(),
              evidence: this.getEvidenceText(varDecl),
              suggestion: vulnerability.suggestion,
              category: "security",
            });

            if (this.debug) {
              console.log(
                `🚨 [S017-Symbol] SQL variable assignment at line ${varDecl.getStartLineNumber()}`
              );
            }
          }
        }
      }
    });

    return violations;
  }

  /**
   * Analyze function parameters for SQL injection
   */
  analyzeFunctionParameters(sourceFile, filePath) {
    const violations = [];

    sourceFile.forEachDescendant((node) => {
      if (
        node.getKind() === SyntaxKind.FunctionDeclaration ||
        node.getKind() === SyntaxKind.ArrowFunction ||
        node.getKind() === SyntaxKind.FunctionExpression
      ) {
        const func = node;
        const body = func.getBody();

        if (body) {
          // Look for SQL construction within function body
          body.forEachDescendant((childNode) => {
            if (childNode.getKind() === SyntaxKind.BinaryExpression) {
              const binExpr = childNode;
              const vulnerability = this.analyzeBinaryExpression(binExpr);

              if (vulnerability) {
                violations.push({
                  ruleId: this.ruleId,
                  severity: "error",
                  message: vulnerability.message,
                  source: this.ruleId,
                  file: filePath,
                  line: binExpr.getStartLineNumber(),
                  column: binExpr.getStart(),
                  evidence: this.getEvidenceText(binExpr),
                  suggestion: vulnerability.suggestion,
                  category: "security",
                });
              }
            }
          });
        }
      }
    });

    return violations;
  }

  /**
   * Get method name from call expression
   */
  getMethodName(callExpr) {
    const expression = callExpr.getExpression();

    if (expression.getKind() === SyntaxKind.PropertyAccessExpression) {
      return expression.getName();
    } else if (expression.getKind() === SyntaxKind.Identifier) {
      return expression.getText();
    }

    return "";
  }

  /**
   * Analyze SQL argument for vulnerabilities
   */
  analyzeSqlArgument(argNode, methodName) {
    const kind = argNode.getKind();

    // Template expression with interpolation
    if (kind === SyntaxKind.TemplateExpression) {
      const templateSpans = argNode.getTemplateSpans();
      if (templateSpans.length > 0) {
        return {
          message: `Template literal with variable interpolation in ${methodName}() call`,
          suggestion: `Use parameterized queries with ${methodName}() instead of template literals`,
        };
      }
    }

    // Binary expression (string concatenation)
    if (kind === SyntaxKind.BinaryExpression) {
      const vulnerability = this.analyzeBinaryExpression(argNode);
      if (vulnerability) {
        return {
          message: `String concatenation in ${methodName}() call: ${vulnerability.message}`,
          suggestion: `Use parameterized queries with ${methodName}() instead of string concatenation`,
        };
      }
    }

    return null;
  }

  /**
   * Check if text contains SQL keywords in proper SQL context
   */
  containsSqlKeywords(text) {
    // Convert to uppercase for case-insensitive matching
    const upperText = text.toUpperCase();

    // Check for SQL keywords that should be word-bounded
    return this.sqlKeywords.some((keyword) => {
      const upperKeyword = keyword.toUpperCase();

      // For multi-word keywords like "ORDER BY", check exact match
      if (upperKeyword.includes(" ")) {
        return upperText.includes(upperKeyword);
      }

      // For single-word keywords, ensure word boundaries
      // This prevents "FROM" matching "documents from logs" (casual English)
      // but allows "SELECT * FROM users" (SQL context)
      const wordBoundaryRegex = new RegExp(`\\b${upperKeyword}\\b`, "g");
      const matches = upperText.match(wordBoundaryRegex);

      if (!matches) return false;

      // Additional context check: if it's a common English word in non-SQL context, be more strict
      if (["FROM", "WHERE", "ORDER", "GROUP", "JOIN"].includes(upperKeyword)) {
        // Check if it's likely SQL context by looking for other SQL indicators
        const sqlIndicators = [
          "SELECT",
          "INSERT",
          "UPDATE",
          "DELETE",
          "TABLE",
          "DATABASE",
          "\\*",
          "SET ",
          "VALUES",
        ];
        const hasSqlContext = sqlIndicators.some((indicator) =>
          upperText.includes(indicator.toUpperCase())
        );

        // For logging statements, require stronger SQL context
        if (this.isLikelyLoggingStatement(text)) {
          return hasSqlContext && matches.length > 0;
        }

        return hasSqlContext || matches.length > 1; // Multiple SQL keywords suggest SQL context
      }

      return matches.length > 0;
    });
  }

  /**
   * Check if text looks like a logging statement
   */
  isLikelyLoggingStatement(text) {
    const loggingIndicators = [
      "✅",
      "❌",
      "🐝",
      "⚠️",
      "🔧",
      "📊",
      "🔍", // Emoji indicators
      "log:",
      "info:",
      "debug:",
      "warn:",
      "error:", // Log level indicators
      "Step",
      "Start",
      "End",
      "Complete",
      "Success",
      "Failed", // Process indicators
      "We got",
      "We have",
      "Found",
      "Processed",
      "Recovered", // Reporting language
      "[LINE]",
      "[DB]",
      "[Service]",
      "[API]", // System component indicators
      "Delete rich-menu",
      "Create rich-menu",
      "Update rich-menu", // Specific app operations
      "successfully",
      "failed",
      "done",
      "error", // Result indicators
      "Rollback",
      "Upload",
      "Download", // Action verbs in app context
      ".log(",
      ".error(",
      ".warn(",
      ".info(",
      ".debug(", // Method calls
    ];

    return loggingIndicators.some((indicator) => text.includes(indicator));
  }

  /**
   * Check if text contains SQL keywords in proper SQL context
   */
  containsSqlKeywords(text) {
    // Convert to uppercase for case-insensitive matching
    const upperText = text.toUpperCase();

    // Early return if this looks like logging - be more permissive
    if (this.isLikelyLoggingStatement(text)) {
      // For logging statements, require very strong SQL context
      const strongSqlIndicators = [
        "SELECT *",
        "INSERT INTO",
        "UPDATE SET",
        "DELETE FROM",
        "CREATE TABLE",
        "DROP TABLE",
        "ALTER TABLE",
        "WHERE ",
        "JOIN ",
        "UNION ",
        "GROUP BY",
        "ORDER BY",
      ];

      const hasStrongSqlContext = strongSqlIndicators.some((indicator) =>
        upperText.includes(indicator.toUpperCase())
      );

      // Only flag logging statements if they contain strong SQL patterns
      return hasStrongSqlContext;
    }

    // Check for SQL keywords that should be word-bounded
    return this.sqlKeywords.some((keyword) => {
      const upperKeyword = keyword.toUpperCase();

      // For multi-word keywords like "ORDER BY", check exact match
      if (upperKeyword.includes(" ")) {
        return upperText.includes(upperKeyword);
      }

      // For single-word keywords, ensure word boundaries
      const wordBoundaryRegex = new RegExp(`\\b${upperKeyword}\\b`, "g");
      const matches = upperText.match(wordBoundaryRegex);

      if (!matches) return false;

      // Additional context check: if it's a common English word in non-SQL context, be more strict
      if (
        [
          "FROM",
          "WHERE",
          "ORDER",
          "GROUP",
          "JOIN",
          "CREATE",
          "DELETE",
          "UPDATE",
        ].includes(upperKeyword)
      ) {
        // Check if it's likely SQL context by looking for other SQL indicators
        const sqlIndicators = [
          "TABLE",
          "DATABASE",
          "COLUMN",
          "\\*",
          "SET ",
          "VALUES",
          "INTO ",
        ];
        const hasSqlContext = sqlIndicators.some((indicator) =>
          upperText.includes(indicator.toUpperCase())
        );

        return hasSqlContext || matches.length > 1; // Multiple SQL keywords suggest SQL context
      }

      return matches.length > 0;
    });
  }

  /**
   * Check for SQL construction patterns
   */
  checkForSqlConstruction(node) {
    const kind = node.getKind();

    if (kind === SyntaxKind.TemplateExpression) {
      const text = node.getText();
      const hasSqlKeyword = this.containsSqlKeywords(text);

      if (hasSqlKeyword && node.getTemplateSpans().length > 0) {
        return {
          message:
            "template literal with SQL keywords and variable interpolation",
          suggestion: "Use parameterized queries instead of template literals",
        };
      }
    }

    if (kind === SyntaxKind.BinaryExpression) {
      return this.analyzeBinaryExpression(node);
    }

    return null;
  }
  /**
   * Analyze binary expression for SQL concatenation
   */
  analyzeBinaryExpression(binExpr) {
    const operator = binExpr.getOperatorToken();

    if (operator.getKind() === SyntaxKind.PlusToken) {
      const leftText = binExpr.getLeft().getText();
      const rightText = binExpr.getRight().getText();
      const fullText = binExpr.getText();

      const hasSqlKeyword = this.containsSqlKeywords(fullText);

      if (hasSqlKeyword) {
        return {
          message: "string concatenation with SQL keywords detected",
          suggestion:
            "Use parameterized queries with placeholders (?, $1, etc.)",
        };
      }
    }

    return null;
  }

  /**
   * Get evidence text for violation
   */
  getEvidenceText(node) {
    const text = node.getText();
    return text.length > 100 ? text.substring(0, 100) + "..." : text;
  }

  /**
   * Analyze universal SQL patterns regardless of imports
   */
  analyzeUniversalSqlPatterns(sourceFile, filePath) {
    const violations = [];

    sourceFile.forEachDescendant((node) => {
      // Check template literals with SQL keywords
      if (node.getKind() === SyntaxKind.TemplateExpression) {
        const template = node;
        const text = template.getText();

        // Check if template contains SQL keywords and has interpolation
        const containsSql = this.containsSqlKeywords(text);

        if (containsSql && template.getTemplateSpans().length > 0) {
          violations.push({
            ruleId: this.ruleId,
            severity: "error",
            message:
              "SQL injection risk: template literal with variable interpolation in SQL query",
            source: this.ruleId,
            file: filePath,
            line: template.getStartLineNumber(),
            column: template.getStart(),
            evidence: this.getEvidenceText(template),
            suggestion:
              "Use parameterized queries instead of template literals for SQL statements",
            category: "security",
          });

          if (this.debug) {
            console.log(
              `🚨 [S017-Symbol] Universal SQL template at line ${template.getStartLineNumber()}`
            );
          }
        }
      }

      // Check binary expressions with SQL concatenation
      if (node.getKind() === SyntaxKind.BinaryExpression) {
        const binExpr = node;
        const operator = binExpr.getOperatorToken();

        if (operator.getKind() === SyntaxKind.PlusToken) {
          const fullText = binExpr.getText();

          const hasSqlKeyword = this.containsSqlKeywords(fullText);

          if (hasSqlKeyword) {
            violations.push({
              ruleId: this.ruleId,
              severity: "error",
              message:
                "SQL injection risk: string concatenation with SQL keywords detected",
              source: this.ruleId,
              file: filePath,
              line: binExpr.getStartLineNumber(),
              column: binExpr.getStart(),
              evidence: this.getEvidenceText(binExpr),
              suggestion:
                "Use parameterized queries with placeholders (?, $1, etc.)",
              category: "security",
            });

            if (this.debug) {
              console.log(
                `🚨 [S017-Symbol] Universal SQL concatenation at line ${binExpr.getStartLineNumber()}`
              );
            }
          }
        }
      }
    });

    return violations;
  }

  /**
   * Get analyzer metadata
   */
  getMetadata() {
    return {
      rule: "S017",
      name: "Always use parameterized queries",
      category: "security",
      type: "symbol-based",
      description:
        "Uses semantic analysis to detect SQL injection vulnerabilities",
    };
  }
}

module.exports = S017SymbolBasedAnalyzer;
