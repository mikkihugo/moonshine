const { Project, SyntaxKind } = require("ts-morph");

/**
 * S017 Regex-Based Analyzer - Always use parameterized queries
 * Uses regex patterns and TypeScript AST to detect SQL injection vulnerabilities
 */
class S017RegexBasedAnalyzer {
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
        `🔧 [S017-Regex] Constructor - databaseLibraries:`,
        this.databaseLibraries.length
      );
      console.log(
        `🔧 [S017-Regex] Constructor - sqlMethods:`,
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
        `🔧 [S017 Regex-Based] Analyzer initialized, verbose: ${this.verbose}`
      );
    }
  }

  /**
   * Analyze file using AST
   */
  async analyzeFile(filePath, fileContent) {
    if (this.debug) {
      console.log(`🔍 [S017-AST] Analyzing: ${filePath}`);
    }

    const violations = [];

    try {
      const project = new Project({
        useInMemoryFileSystem: true,
        compilerOptions: {
          allowJs: true,
          target: "ES2020",
        },
      });

      const sourceFile = project.createSourceFile(filePath, fileContent);

      // Find SQL-related method calls
      const sqlMethodCalls = this.findSqlMethodCalls(sourceFile);

      for (const methodCall of sqlMethodCalls) {
        const sqlViolations = this.analyzeSqlMethodCall(methodCall, filePath);
        violations.push(...sqlViolations);
      }

      // Find template literals with SQL content
      const templateLiterals = this.findSqlTemplateLiterals(sourceFile);

      for (const template of templateLiterals) {
        const templateViolations = this.analyzeTemplateLiteral(
          template,
          filePath
        );
        violations.push(...templateViolations);
      }

      if (this.debug) {
        console.log(
          `🔍 [S017-AST] Found ${violations.length} violations in ${filePath}`
        );
      }
    } catch (error) {
      if (this.debug) {
        console.error(`❌ [S017-AST] Error analyzing ${filePath}:`, error);
      }
    }

    return violations;
  }

  /**
   * Find method calls that might execute SQL
   */
  findSqlMethodCalls(sourceFile) {
    const methodCalls = [];

    sourceFile.forEachDescendant((node) => {
      if (node.getKind() === SyntaxKind.CallExpression) {
        const callExpr = node;
        const expression = callExpr.getExpression();

        let methodName = "";

        if (expression.getKind() === SyntaxKind.PropertyAccessExpression) {
          const propAccess = expression;
          methodName = propAccess.getName();
        } else if (expression.getKind() === SyntaxKind.Identifier) {
          methodName = expression.getText();
        }

        // Check if method name matches SQL execution methods
        if (this.sqlMethods.includes(methodName)) {
          methodCalls.push({
            node: callExpr,
            methodName,
            line: callExpr.getStartLineNumber(),
            column: callExpr.getStart(),
          });
        }
      }
    });

    return methodCalls;
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
   * Find template literals that might contain SQL
   */
  findSqlTemplateLiterals(sourceFile) {
    const templateLiterals = [];

    sourceFile.forEachDescendant((node) => {
      if (node.getKind() === SyntaxKind.TemplateExpression) {
        const template = node;
        const text = template.getText();

        // Check if template contains SQL keywords using improved logic
        const containsSql = this.containsSqlKeywords(text);

        if (containsSql) {
          templateLiterals.push({
            node: template,
            text,
            line: template.getStartLineNumber(),
            column: template.getStart(),
          });
        }
      }
    });

    return templateLiterals;
  }

  /**
   * Analyze SQL method call for vulnerabilities
   */
  analyzeSqlMethodCall(methodCall, filePath) {
    const violations = [];
    const { node, methodName, line } = methodCall;
    const args = node.getArguments();

    if (args.length === 0) return violations;

    const firstArg = args[0];

    // Check if first argument is a string concatenation or template literal
    if (this.isSuspiciousSqlArgument(firstArg)) {
      const argText = firstArg.getText();
      const evidence =
        node.getText().length > 100
          ? node.getText().substring(0, 100) + "..."
          : node.getText();

      violations.push({
        ruleId: this.ruleId,
        severity: "error",
        message: `SQL injection risk in ${methodName}(): avoid string concatenation or template literals in SQL queries`,
        source: this.ruleId,
        file: filePath,
        line: line,
        column: firstArg.getStart(),
        evidence: evidence,
        suggestion: `Use parameterized queries with ${methodName}() method instead of string concatenation`,
        category: "security",
      });

      if (this.debug) {
        console.log(
          `🚨 [S017-AST] Unsafe SQL method call at line ${line}: ${methodName}`
        );
      }
    }

    return violations;
  }

  /**
   * Analyze template literal for SQL injection risks
   */
  analyzeTemplateLiteral(template, filePath) {
    const violations = [];
    const { node, text, line } = template;

    // Check if template has variable interpolation
    if (node.getTemplateSpans().length > 0) {
      const evidence =
        text.length > 100 ? text.substring(0, 100) + "..." : text;

      violations.push({
        ruleId: this.ruleId,
        severity: "error",
        message:
          "SQL injection risk: template literal with variable interpolation in SQL query",
        source: this.ruleId,
        file: filePath,
        line: line,
        column: node.getStart(),
        evidence: evidence,
        suggestion:
          "Use parameterized queries instead of template literals for SQL statements",
        category: "security",
      });

      if (this.debug) {
        console.log(`🚨 [S017-AST] Unsafe SQL template at line ${line}`);
      }
    }

    return violations;
  }

  /**
   * Check if argument is suspicious for SQL injection
   */
  isSuspiciousSqlArgument(argNode) {
    const kind = argNode.getKind();

    // Template expressions with interpolation
    if (kind === SyntaxKind.TemplateExpression) {
      return argNode.getTemplateSpans().length > 0;
    }

    // Binary expressions (string concatenation)
    if (kind === SyntaxKind.BinaryExpression) {
      const binExpr = argNode;
      return binExpr.getOperatorToken().getKind() === SyntaxKind.PlusToken;
    }

    // Check if it's a template literal with SQL keywords
    if (kind === SyntaxKind.NoSubstitutionTemplateLiteral) {
      const text = argNode.getText();
      return this.sqlKeywords.some((keyword) =>
        text.toUpperCase().includes(keyword.toUpperCase())
      );
    }

    return false;
  }

  /**
   * Get analyzer metadata
   */
  getMetadata() {
    return {
      rule: "S017",
      name: "Always use parameterized queries",
      category: "security",
      type: "regex-based",
      description:
        "Uses regex patterns and AST analysis to detect SQL injection vulnerabilities",
    };
  }
}

module.exports = S017RegexBasedAnalyzer;
