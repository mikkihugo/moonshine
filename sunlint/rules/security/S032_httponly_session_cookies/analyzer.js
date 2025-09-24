/**
 * S032 Main Analyzer - Set HttpOnly attribute for Session Cookies
 * Primary: Symbol-based analysis (when available)
 * Fallback: Regex-based for all other cases
 * Command: node cli.js --rule=S032 --input=examples/rule-test-fixtures/rules/S032_httponly_session_cookies --engine=heuristic
 */

const S032SymbolBasedAnalyzer = require("./symbol-based-analyzer.js");
const S032RegexBasedAnalyzer = require("./regex-based-analyzer.js");

class S032Analyzer {
  constructor(options = {}) {
    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔧 [S032] Constructor called with options:`, !!options);
      console.log(
        `🔧 [S032] Options type:`,
        typeof options,
        Object.keys(options || {})
      );
    }

    this.ruleId = "S032";
    this.ruleName = "Set HttpOnly attribute for Session Cookies";
    this.description =
      "Set HttpOnly attribute for Session Cookies to prevent JavaScript access. This protects against XSS attacks by preventing client-side script access to sensitive cookies.";
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
      this.symbolAnalyzer = new S032SymbolBasedAnalyzer(this.semanticEngine);
      if (process.env.SUNLINT_DEBUG) {
        console.log(`🔧 [S032] Symbol analyzer created successfully`);
      }
    } catch (error) {
      console.error(`🔧 [S032] Error creating symbol analyzer:`, error);
    }

    try {
      this.regexAnalyzer = new S032RegexBasedAnalyzer(this.semanticEngine);
      if (process.env.SUNLINT_DEBUG) {
        console.log(`🔧 [S032] Regex analyzer created successfully`);
      }
    } catch (error) {
      console.error(`🔧 [S032] Error creating regex analyzer:`, error);
    }
  }

  /**
   * Initialize analyzer with semantic engine
   */
  async initialize(semanticEngine) {
    this.semanticEngine = semanticEngine;

    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔧 [S032] Main analyzer initializing...`);
    }

    // Initialize both analyzers
    if (this.symbolAnalyzer) {
      await this.symbolAnalyzer.initialize?.(semanticEngine);
    }
    if (this.regexAnalyzer) {
      await this.regexAnalyzer.initialize?.(semanticEngine);
    }

    // Clean up if needed
    if (this.regexAnalyzer) {
      this.regexAnalyzer.cleanup?.();
    }

    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔧 [S032] Main analyzer initialized successfully`);
    }
  }

  /**
   * Single file analysis method for testing
   */
  analyzeSingle(filePath, options = {}) {
    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔍 [S032] analyzeSingle() called for: ${filePath}`);
    }

    // Return result using same format as analyze method
    return this.analyze([filePath], "typescript", options);
  }

  async analyze(files, language, options = {}) {
    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔧 [S032] analyze() method called with ${files.length} files, language: ${language}`
      );
    }

    const violations = [];

    for (const filePath of files) {
      try {
        if (process.env.SUNLINT_DEBUG) {
          console.log(`🔧 [S032] Processing file: ${filePath}`);
        }

        const fileViolations = await this.analyzeFile(filePath, options);
        violations.push(...fileViolations);

        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔧 [S032] File ${filePath}: Found ${fileViolations.length} violations`
          );
        }
      } catch (error) {
        console.warn(
          `⚠ [S032] Analysis failed for ${filePath}:`,
          error.message
        );
      }
    }

    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔧 [S032] Total violations found: ${violations.length}`);
    }

    return violations;
  }

  async analyzeFile(filePath, options = {}) {
    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔍 [S032] analyzeFile() called for: ${filePath}`);
    }

    // Create a Map to track unique violations and prevent duplicates
    const violationMap = new Map();

    // 1. Try Symbol-based analysis first (primary)
    if (
      this.config.useSymbolBased &&
      this.semanticEngine?.project &&
      this.semanticEngine?.initialized
    ) {
      try {
        if (process.env.SUNLINT_DEBUG) {
          console.log(`🔧 [S032] Trying symbol-based analysis...`);
        }
        const sourceFile = this.semanticEngine.project.getSourceFile(filePath);
        if (sourceFile) {
          if (process.env.SUNLINT_DEBUG) {
            console.log(`🔧 [S032] Source file found, analyzing...`);
          }
          const symbolViolations = await this.symbolAnalyzer.analyze(
            sourceFile,
            filePath
          );

          // Add to violation map with deduplication
          symbolViolations.forEach((violation) => {
            // Create a location-specific key to allow multiple violations for same cookie at different locations
            const cookieName = this.extractCookieName(violation.message) || "";
            const key = `cookie:${cookieName}:line:${violation.line}:httponly`;
            if (!violationMap.has(key)) {
              violationMap.set(key, violation);
            }
          });
          if (process.env.SUNLINT_DEBUG) {
            console.log(
              `🔧 [S032] Symbol analysis completed: ${symbolViolations.length} violations`
            );
          }
        } else {
          if (process.env.SUNLINT_DEBUG) {
            console.log(`🔧 [S032] Source file not found, falling back...`);
          }
        }
      } catch (error) {
        console.warn(`⚠ [S032] Symbol analysis failed:`, error.message);
      }
    }

    // 2. Try Regex-based analysis (fallback or additional)
    if (this.config.fallbackToRegex || this.config.regexBasedOnly) {
      try {
        if (process.env.SUNLINT_DEBUG) {
          console.log(`🔧 [S032] Trying regex-based analysis...`);
        }
        const regexViolations = await this.regexAnalyzer.analyze(filePath);

        // Add to violation map with deduplication
        regexViolations.forEach((violation) => {
          // Create a location-specific key to allow multiple violations for same cookie at different locations
          const cookieName = this.extractCookieName(violation.message) || "";
          const key = `cookie:${cookieName}:line:${violation.line}:httponly`;

          // Priority: If we already have a violation for this cookie at this line, prefer symbol analyzer result
          if (!violationMap.has(key)) {
            violationMap.set(key, violation);
          } else {
            const existing = violationMap.get(key);
            // Prefer framework-specific messages (Nuxt, NestJS, Next.js) over generic ones
            if (
              this.isFrameworkSpecificMessage(violation.message) &&
              !this.isFrameworkSpecificMessage(existing.message)
            ) {
              violationMap.set(key, violation);
            }
          }
        });

        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔧 [S032] Regex analysis completed: ${regexViolations.length} violations`
          );
        }
      } catch (error) {
        console.warn(`⚠ [S032] Regex analysis failed:`, error.message);
      }
    }

    // Convert Map values to array and add filePath to each violation
    const finalViolations = Array.from(violationMap.values()).map(
      (violation) => ({
        ...violation,
        filePath: filePath,
        file: filePath, // Also add 'file' for compatibility
      })
    );

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔧 [S032] File analysis completed: ${finalViolations.length} unique violations`
      );
    }

    return finalViolations;
  }

  /**
   * Extract cookie name from violation message for better deduplication
   */
  extractCookieName(message) {
    try {
      const match = message.match(
        /Session cookie "([^"]+)"|useCookie "([^"]+)"|Cookie "([^"]+)"/
      );
      return match ? match[1] || match[2] || match[3] : "";
    } catch (error) {
      return "";
    }
  }

  /**
   * Check if message is framework-specific (preferred over generic)
   */
  isFrameworkSpecificMessage(message) {
    return (
      message.includes("Nuxt useCookie") ||
      message.includes("NestJS") ||
      message.includes("Next.js") ||
      message.includes("Framework")
    );
  }

  /**
   * Clean up resources
   */
  cleanup() {
    if (this.symbolAnalyzer?.cleanup) {
      this.symbolAnalyzer.cleanup();
    }
    if (this.regexAnalyzer?.cleanup) {
      this.regexAnalyzer.cleanup();
    }
  }
}

module.exports = S032Analyzer;
