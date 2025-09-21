/**
 * S017 Main Analyzer - Always use parameterized queries
 * Primary: Symbol-based analysis (when available)
 * Fallback: Regex-based for all other cases
 * Command: node cli.js --rule=S017 --input=examples/rule-test-fixtures/rules/S017_use_parameterized_queries --engine=heuristic
 */

const S017SymbolBasedAnalyzer = require("./symbol-based-analyzer.js");
const S017RegexBasedAnalyzer = require("./regex-based-analyzer.js");

class S017Analyzer {
  constructor(options = {}) {
    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔧 [S017] Constructor called with options:`, !!options);
      console.log(
        `🔧 [S017] Options type:`,
        typeof options,
        Object.keys(options || {})
      );
    }

    this.ruleId = "S017";
    this.ruleName = "Always use parameterized queries";
    this.description =
      "Always use parameterized queries instead of string concatenation to build SQL queries. This prevents SQL injection attacks by separating SQL logic from data";
    this.semanticEngine = options.semanticEngine || null;
    this.verbose = options.verbose || false;

    // Configuration
    this.config = {
      useSymbolBased: true, // Primary approach
      fallbackToRegex: true, // Secondary approach
      regexBasedOnly: false, // Can be set to true for pure mode
    };

    // Initialize analyzers
    try {
      this.symbolAnalyzer = new S017SymbolBasedAnalyzer(this.semanticEngine);
      if (process.env.SUNLINT_DEBUG) {
        console.log(`🔧 [S017] Symbol analyzer created successfully`);
      }
    } catch (error) {
      console.error(`🔧 [S017] Error creating symbol analyzer:`, error);
    }

    try {
      this.regexAnalyzer = new S017RegexBasedAnalyzer(this.semanticEngine);
      if (process.env.SUNLINT_DEBUG) {
        console.log(`🔧 [S017] Regex analyzer created successfully`);
      }
    } catch (error) {
      console.error(`🔧 [S017] Error creating regex analyzer:`, error);
    }

    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔧 [S017] Constructor completed`);
    }
  }

  /**
   * Initialize with semantic engine
   */
  async initialize(semanticEngine = null) {
    if (semanticEngine) {
      this.semanticEngine = semanticEngine;
    }
    this.verbose = semanticEngine?.verbose || false;

    // Initialize both analyzers
    if (this.symbolAnalyzer) {
      await this.symbolAnalyzer.initialize?.(semanticEngine);
    }
    if (this.regexAnalyzer) {
      await this.regexAnalyzer.initialize?.(semanticEngine);
    }

    // Ensure verbose flag is propagated
    if (this.regexAnalyzer) {
      this.regexAnalyzer.verbose = this.verbose;
    }
    if (this.symbolAnalyzer) {
      this.symbolAnalyzer.verbose = this.verbose;
    }

    if (this.verbose) {
      console.log(
        `🔧 [S017 Hybrid] Analyzer initialized - verbose: ${this.verbose}`
      );
    }
  }

  /**
   * Single file analysis method for testing
   */
  analyzeSingle(filePath, options = {}) {
    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔍 [S017] analyzeSingle() called for: ${filePath}`);
    }

    // Return result using same format as analyze method
    return this.analyze([filePath], "typescript", options);
  }

  async analyze(files, language, options = {}) {
    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔧 [S017] analyze() method called with ${files.length} files, language: ${language}`
      );
    }

    const violations = [];

    for (const filePath of files) {
      try {
        if (process.env.SUNLINT_DEBUG) {
          console.log(`🔧 [S017] Processing file: ${filePath}`);
        }

        const fileViolations = await this.analyzeFile(filePath, options);
        violations.push(...fileViolations);

        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔧 [S017] File ${filePath}: Found ${fileViolations.length} violations`
          );
        }
      } catch (error) {
        console.warn(
          `⚠ [S017] Analysis failed for ${filePath}:`,
          error.message
        );
      }
    }

    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔧 [S017] Total violations found: ${violations.length}`);
    }

    return violations;
  }

  async analyzeFile(filePath, options = {}) {
    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔍 [S017] analyzeFile() called for: ${filePath}`);
    }

    // Create a Set to track unique violations and prevent duplicates
    const violationMap = new Map();

    // 1. Try Symbol-based analysis first (primary)
    if (
      this.config.useSymbolBased &&
      this.semanticEngine?.project &&
      this.semanticEngine?.initialized
    ) {
      try {
        if (process.env.SUNLINT_DEBUG) {
          console.log(`🔧 [S017] Trying symbol-based analysis...`);
        }
        const sourceFile = this.semanticEngine.project.getSourceFile(filePath);
        if (sourceFile) {
          if (process.env.SUNLINT_DEBUG) {
            console.log(
              `🔧 [S017] Source file found, analyzing with symbol-based...`
            );
          }

          // Read file content for symbol analyzer
          const fs = require("fs");
          const fileContent = fs.readFileSync(filePath, "utf8");

          const violations = await this.symbolAnalyzer.analyzeFile(
            filePath,
            fileContent,
            { ...options, verbose: options.verbose }
          );

          // Add violations to map to deduplicate
          violations.forEach((v) => {
            const key = `${v.line}:${v.column}:${v.message}`;
            if (!violationMap.has(key)) {
              v.analysisStrategy = "symbol-based";
              violationMap.set(key, v);
            }
          });

          if (process.env.SUNLINT_DEBUG) {
            console.log(
              `✅ [S017] Symbol-based analysis: ${violations.length} violations`
            );
          }
          return Array.from(violationMap.values()); // Return deduplicated violations
        } else {
          if (process.env.SUNLINT_DEBUG) {
            console.log(`⚠️ [S017] Source file not found in project`);
          }
        }
      } catch (error) {
        console.warn(`⚠️ [S017] Symbol analysis failed: ${error.message}`);
        // Continue to fallback
      }
    } else {
      if (process.env.SUNLINT_DEBUG) {
        console.log(`🔄 [S017] Symbol analysis conditions check:`);
        console.log(`  - useSymbolBased: ${this.config.useSymbolBased}`);
        console.log(`  - semanticEngine: ${!!this.semanticEngine}`);
        console.log(
          `  - semanticEngine.project: ${!!this.semanticEngine?.project}`
        );
        console.log(
          `  - semanticEngine.initialized: ${this.semanticEngine?.initialized}`
        );
        console.log(
          `🔄 [S017] Symbol analysis unavailable, using regex fallback`
        );
      }
    }

    // 2. Fallback to regex-based analysis (only if symbol-based failed or unavailable)
    if (this.config.fallbackToRegex) {
      try {
        if (process.env.SUNLINT_DEBUG) {
          console.log(`🔧 [S017] Trying regex-based analysis...`);
        }

        // Read file content for regex analyzer
        const fs = require("fs");
        const fileContent = fs.readFileSync(filePath, "utf8");

        const violations = await this.regexAnalyzer.analyzeFile(
          filePath,
          fileContent,
          options
        );

        // Add violations to map to deduplicate
        violations.forEach((v) => {
          const key = `${v.line}:${v.column}:${v.message}`;
          if (!violationMap.has(key)) {
            v.analysisStrategy = "regex-fallback";
            violationMap.set(key, v);
          }
        });

        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔄 [S017] Regex-based analysis: ${violations.length} violations`
          );
        }
        return Array.from(violationMap.values()); // Return deduplicated violations
      } catch (error) {
        console.error(`⚠ [S017] Regex analysis failed: ${error.message}`);
      }
    }

    console.log(`🔧 [S017] No analysis methods succeeded, returning empty`);
    return [];
  }

  /**
   * Methods for compatibility with different engine invocation patterns
   */
  async analyzeFileWithSymbols(filePath, options = {}) {
    return this.analyzeFile(filePath, options);
  }

  async analyzeWithSemantics(filePath, options = {}) {
    return this.analyzeFile(filePath, options);
  }

  /**
   * Get analyzer metadata
   */
  getMetadata() {
    return {
      rule: "S017",
      name: "Always use parameterized queries",
      category: "security",
      type: "hybrid",
      description:
        "Uses symbol-based and regex analysis to detect SQL injection vulnerabilities",
    };
  }
}

module.exports = S017Analyzer;
