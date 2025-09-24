/**
 * S032 Regex-Based Analyzer - Set HttpOnly attribute for Session Cookies
 * Fallback analysis using regex patterns
 */

const fs = require("fs");

class S032RegexBasedAnalyzer {
  constructor(semanticEngine = null) {
    this.semanticEngine = semanticEngine;
    this.ruleId = "S032";
    this.category = "security";

    // Session cookie indicators
    this.sessionIndicators = [
      "session",
      "sessionid",
      "sessid",
      "jsessionid",
      "phpsessid",
      "asp.net_sessionid",
      "connect.sid",
      "auth",
      "token",
      "jwt",
      "csrf",
      "refresh",
      // NestJS specific
      "nest-session",
      "nest-auth",
      // Next.js specific
      "next-auth.session-token",
      "next-auth.csrf-token",
      "__Host-next-auth.csrf-token",
      "__Secure-next-auth.session-token",
      // Nuxt.js specific
      "nuxt-session",
      "nuxt-auth",
      "auth._token",
      "auth._refresh_token",
      // General framework patterns
      "access_token",
      "refresh_token",
      "id_token",
      "state_token",
      "nonce",
    ];

    // Regex patterns for cookie detection
    this.cookiePatterns = [
      // Express/Node.js cookie patterns
      /res\.cookie\s*\(\s*['"`]([^'"`]+)['"`]\s*,\s*[^,)]+\s*,\s*({[^}]+})/gi,
      /response\.cookie\s*\(\s*['"`]([^'"`]+)['"`]\s*,\s*[^,)]+\s*,\s*({[^}]+})/gi,

      // NestJS specific patterns (more specific to avoid overlap)
      /@Res\(\)\s*\.cookie\s*\(\s*['"`]([^'"`]+)['"`]\s*,\s*[^,)]+\s*,\s*({[^}]+})/gi,

      // Next.js patterns
      /NextResponse\.next\(\)\.cookies\.set\s*\(\s*['"`]([^'"`]+)['"`]\s*,\s*[^,)]+\s*,\s*({[^}]+})/gi,
      /cookies\(\)\.set\s*\(\s*['"`]([^'"`]+)['"`]\s*,\s*[^,)]+\s*,\s*({[^}]+})/gi,
      /\.cookies\.set\s*\(\s*['"`]([^'"`]+)['"`]\s*,\s*[^,)]+\s*,\s*({[^}]+})/gi,

      // Nuxt.js patterns
      /useCookie\s*\(\s*['"`]([^'"`]+)['"`]\s*,\s*({[^}]+})/gi,
      /\$cookies\.set\s*\(\s*['"`]([^'"`]+)['"`]\s*,\s*[^,)]+\s*,\s*({[^}]+})/gi,

      // Set-Cookie header patterns (array format)
      /setHeader\s*\(\s*['"`]Set-Cookie['"`]\s*,\s*\[\s*([^\]]+)\s*\]/gi,

      // Set-Cookie header patterns (single string)
      /setHeader\s*\(\s*['"`]Set-Cookie['"`]\s*,\s*['"`]([^'"`]+)['"`]/gi,

      // Session middleware patterns
      /session\s*\(\s*({[^}]+})/gi,
      /\.use\s*\(\s*session\s*\(\s*({[^}]+})/gi,

      // Framework-specific session patterns
      /NextAuth\s*\(\s*({[^}]+})/gi,

      // Generic cookie method (only if not caught by above patterns)
      /(?<!response\.)(?<!res\.)(?<!@Res\(\)\s*)\.cookie\s*\(\s*['"`]([^'"`]+)['"`]\s*,\s*[^,)]+\s*,\s*({[^}]+})/gi,
    ];

    // NextAuth configuration patterns
    this.nextAuthPatterns = [
      // Individual cookie configuration patterns for sessionToken
      /sessionToken\s*:\s*\{[^{]*?name\s*:\s*['"`]([^'"`]+)['"`][^{]*?options\s*:\s*\{([^}]+)\}/gis,

      // Individual cookie configuration patterns for csrfToken
      /csrfToken\s*:\s*\{[^{]*?name\s*:\s*['"`]([^'"`]+)['"`][^{]*?options\s*:\s*\{([^}]+)\}/gis,

      // Generic cookie configuration pattern (fallback)
      /(\w+Token)\s*:\s*\{[^{]*?name\s*:\s*['"`]([^'"`]+)['"`][^{]*?options\s*:\s*\{([^}]+)\}/gis,
    ];
  }

  /**
   * Initialize analyzer
   */
  async initialize(semanticEngine) {
    this.semanticEngine = semanticEngine;
    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔧 [S032] Regex-based analyzer initialized`);
    }
  }

  /**
   * Main analysis method
   */
  async analyze(filePath) {
    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔍 [S032] Regex-based analysis for: ${filePath}`);
    }

    const violations = [];
    const violationMap = new Map(); // Deduplication map

    try {
      const content = fs.readFileSync(filePath, "utf8");
      const lines = content.split("\n");

      // Check each pattern
      for (const pattern of this.cookiePatterns) {
        this.checkPattern(pattern, content, lines, violations, filePath);
      }

      // Check NextAuth configuration patterns
      for (const pattern of this.nextAuthPatterns) {
        this.checkNextAuthPattern(
          pattern,
          content,
          lines,
          violations,
          filePath
        );
      }

      // Check for session middleware without cookie config
      // This method is now mainly handled by checkPattern, but keep for edge cases
    } catch (error) {
      console.warn(
        `⚠ [S032] Regex analysis failed for ${filePath}:`,
        error.message
      );
    }

    // Deduplicate violations based on line, column, and message
    violations.forEach((violation) => {
      const key = `${violation.line}:${violation.column}:${violation.message}`;
      if (!violationMap.has(key)) {
        violationMap.set(key, violation);
      }
    });

    const uniqueViolations = Array.from(violationMap.values());

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔧 [S032] Regex analysis completed: ${violations.length} total, ${uniqueViolations.length} unique violations`
      );
    }

    return uniqueViolations;
  }

  /**
   * Check a specific regex pattern for violations
   */
  checkPattern(pattern, content, lines, violations, filePath) {
    pattern.lastIndex = 0; // Reset regex state
    let match;

    while ((match = pattern.exec(content)) !== null) {
      const matchText = match[0];
      const cookieName = match[1] || "";
      const cookieConfig = match[2] || match[1] || matchText;

      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Regex: Pattern match - cookieName: "${cookieName}", config: "${cookieConfig.substring(
            0,
            50
          )}..."`
        );
      }

      // Handle different patterns
      if (matchText.includes("setHeader") && matchText.includes("Set-Cookie")) {
        this.checkSetCookieHeaderPattern(match, content, violations, filePath);
      } else if (matchText.includes("session(")) {
        this.checkSessionMiddlewarePattern(
          match,
          content,
          violations,
          filePath
        );
      } else if (
        matchText.includes("NextAuth(") ||
        matchText.includes("useCookie(")
      ) {
        this.checkFrameworkSpecificPattern(
          match,
          content,
          violations,
          filePath
        );
      } else {
        // Regular cookie patterns
        if (this.isSessionCookie(cookieName, matchText)) {
          if (!this.hasHttpOnlyFlag(cookieConfig)) {
            const lineNumber = this.getLineNumber(content, match.index);

            if (process.env.SUNLINT_DEBUG) {
              console.log(
                `🔍 [S032] Regex: ⚠️ VIOLATION FOUND: Line ${lineNumber} - "${cookieName}" missing httpOnly`
              );
            }

            violations.push({
              rule: this.ruleId,
              source: filePath,
              category: this.category,
              line: lineNumber,
              column: 1,
              message: `Insecure session cookie: Session cookie "${cookieName}" missing HttpOnly attribute`,
              severity: "error",
            });
          }
        }
      }
    }
  }

  /**
   * Check NextAuth configuration patterns for missing httpOnly
   */
  checkNextAuthPattern(pattern, content, lines, violations, filePath) {
    pattern.lastIndex = 0; // Reset regex state
    let match;

    while ((match = pattern.exec(content)) !== null) {
      const matchText = match[0];

      // Handle different pattern capture groups
      let cookieName, optionsConfig;
      if (match[3]) {
        // Generic pattern: match[1] = tokenType, match[2] = name, match[3] = options
        cookieName = match[2];
        optionsConfig = match[3];
      } else {
        // Specific patterns: match[1] = name, match[2] = options
        cookieName = match[1] || "session-cookie";
        optionsConfig = match[2] || "";
      }

      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] NextAuth: Pattern match - cookieName: "${cookieName}", options: "${optionsConfig.substring(
            0,
            50
          )}..."`
        );
      }

      // Check if httpOnly is missing or false
      if (!this.hasHttpOnlyTrue(optionsConfig)) {
        const lineNumber = this.getLineNumber(content, match.index);

        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `❌ [S032] NextAuth: Missing HttpOnly for cookie "${cookieName}" at line ${lineNumber}`
          );
        }

        violations.push({
          rule: this.ruleId,
          source: filePath,
          category: this.category,
          line: lineNumber,
          column: 1,
          message: `NextAuth session cookie: Cookie "${cookieName}" missing HttpOnly attribute in authOptions configuration`,
          severity: "error",
        });
      } else {
        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `✅ [S032] NextAuth: Cookie "${cookieName}" has HttpOnly set correctly`
          );
        }
      }
    }
  }

  /**
   * Check Set-Cookie header patterns
   */
  checkSetCookieHeaderPattern(match, content, violations, filePath) {
    const matchText = match[0];

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S032] Regex: Checking Set-Cookie header pattern: ${matchText.substring(
          0,
          100
        )}...`
      );
    }

    // Extract cookie strings from setHeader array format
    if (matchText.includes("[")) {
      const arrayMatch = matchText.match(/\[\s*([^\]]+)\s*\]/);
      if (arrayMatch) {
        const cookiesContent = arrayMatch[1];

        // Split by comma but preserve template literals
        const cookieStrings = this.splitCookieStrings(cookiesContent);

        for (const cookieString of cookieStrings) {
          const cookieName = this.extractCookieNameFromString(cookieString);

          if (process.env.SUNLINT_DEBUG) {
            console.log(
              `🔍 [S032] Regex: Checking Set-Cookie string: "${cookieString.substring(
                0,
                50
              )}..." - name: "${cookieName}"`
            );
          }

          if (this.isSessionCookie(cookieName, cookieString)) {
            const hasHttpOnly = cookieString.toLowerCase().includes("httponly");

            if (!hasHttpOnly) {
              const lineNumber = this.getLineNumber(content, match.index);

              violations.push({
                rule: this.ruleId,
                source: filePath,
                category: this.category,
                line: lineNumber,
                column: 1,
                message: `Insecure session cookie: Session cookie "${cookieName}" in Set-Cookie header missing HttpOnly attribute`,
                severity: "error",
              });
            }
          }
        }
      }
    }
  }

  /**
   * Split cookie strings while preserving template literals
   */
  splitCookieStrings(cookiesContent) {
    const cookieStrings = [];
    let current = "";
    let inTemplate = false;
    let quoteChar = null;

    for (let i = 0; i < cookiesContent.length; i++) {
      const char = cookiesContent[i];

      if ((char === '"' || char === "'" || char === "`") && !quoteChar) {
        quoteChar = char;
        current += char;
      } else if (char === quoteChar) {
        quoteChar = null;
        current += char;
      } else if (char === "," && !quoteChar) {
        if (current.trim()) {
          cookieStrings.push(current.trim());
          current = "";
        }
      } else {
        current += char;
      }
    }

    if (current.trim()) {
      cookieStrings.push(current.trim());
    }

    return cookieStrings;
  }

  /**
   * Extract cookie name from string like "auth=${tokens.auth}; ..." or `auth=${value}; ...`
   */
  extractCookieNameFromString(cookieString) {
    try {
      // Remove quotes and backticks
      const cleaned = cookieString
        .replace(/^[`'"]/g, "")
        .replace(/[`'"]$/g, "");

      // Match cookie name before = sign
      const match = cleaned.match(/^(\w+)=/);
      return match ? match[1] : null;
    } catch (error) {
      return null;
    }
  }

  /**
   * Check session middleware pattern
   */
  checkSessionMiddlewarePattern(match, content, violations, filePath) {
    const sessionConfig = match[1];

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S032] Regex: Checking session middleware: ${sessionConfig.substring(
          0,
          100
        )}...`
      );
    }

    // Check if session has cookie configuration
    if (sessionConfig.includes("cookie:")) {
      // Has cookie config, check for httpOnly
      if (!this.hasHttpOnlyFlag(sessionConfig)) {
        const lineNumber = this.getLineNumber(content, match.index);

        violations.push({
          rule: this.ruleId,
          source: filePath,
          category: this.category,
          line: lineNumber,
          column: 1,
          message:
            "Insecure session cookie: Session middleware missing httpOnly attribute",
          severity: "error",
        });
      }
    } else {
      // No cookie config at all
      const lineNumber = this.getLineNumber(content, match.index);

      violations.push({
        rule: this.ruleId,
        source: filePath,
        category: this.category,
        line: lineNumber,
        column: 1,
        message:
          "Insecure session cookie: Session middleware missing cookie configuration with httpOnly",
        severity: "error",
      });
    }
  }

  /**
   * Check if cookie name indicates session cookie
   */
  isSessionCookie(cookieName, fullMatch) {
    if (!cookieName && fullMatch.includes("session")) {
      return true; // Session middleware
    }

    if (!cookieName) return false;

    const lowerName = cookieName.toLowerCase();
    return this.sessionIndicators.some((indicator) =>
      lowerName.includes(indicator.toLowerCase())
    );
  }

  /**
   * Check if configuration has httpOnly flag
   */
  hasHttpOnlyFlag(configText) {
    // Skip if this is a reference to external config
    if (this.isConfigReference(configText)) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Regex: Skipping config reference: ${configText.substring(
            0,
            30
          )}...`
        );
      }
      return true; // Assume external config is secure (avoid false positives)
    }

    // Remove comments to avoid false positives from "// Missing: httpOnly: true"
    const codeOnly = configText
      .replace(/\/\/.*$/gm, "")
      .replace(/\/\*[\s\S]*?\*\//g, "");

    const httpOnlyPatterns = [
      /httpOnly\s*:\s*true/i,
      /httpOnly\s*=\s*true/i,
      /['"]httpOnly['"]\s*:\s*true/i,
      /HttpOnly/i, // For Set-Cookie header format
    ];

    // Check for explicitly disabled httpOnly (should be treated as violation)
    const httpOnlyDisabledPatterns = [
      /httpOnly\s*:\s*false/i,
      /httpOnly\s*=\s*false/i,
      /['"]httpOnly['"]\s*:\s*false/i,
    ];

    const hasHttpOnlyFalse = httpOnlyDisabledPatterns.some((pattern) =>
      pattern.test(codeOnly)
    );

    const hasHttpOnly = httpOnlyPatterns.some((pattern) =>
      pattern.test(codeOnly)
    );

    // If httpOnly is explicitly set to false, it's a violation
    if (hasHttpOnlyFalse) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Regex: HttpOnly explicitly disabled (violation): ${configText.substring(
            0,
            50
          )}...`
        );
      }
      return false; // Violation: explicitly disabled
    }

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S032] Regex: HttpOnly check result: ${hasHttpOnly} for config (without comments): ${codeOnly.substring(
          0,
          50
        )}...`
      );
    }

    return hasHttpOnly;
  }

  /**
   * Check if the config text is a reference to external configuration
   */
  isConfigReference(configText) {
    const refPatterns = [
      /this\.\w+/, // this.cookieConfig
      /\w+Config/, // someConfig
      /\.\.\.\w+/, // ...spread
      /\w+\.\w+/, // object.property
    ];

    return refPatterns.some((pattern) => pattern.test(configText.trim()));
  }

  /**
   * Get line number from character index
   */
  getLineNumber(content, index) {
    const beforeMatch = content.substring(0, index);
    return beforeMatch.split("\n").length;
  }

  /**
   * Check framework-specific patterns (Next.js, Nuxt.js, etc.)
   */
  checkFrameworkSpecificPattern(match, content, violations, filePath) {
    const matchText = match[0];
    const cookieName = match[1] || "";
    const cookieConfig = match[2] || match[1] || "";

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S032] Regex: Checking framework-specific pattern: ${matchText.substring(
          0,
          100
        )}...`
      );
    }

    // Handle NextAuth patterns
    if (matchText.includes("NextAuth(")) {
      if (!this.hasHttpOnlyInNextAuthConfig(cookieConfig)) {
        const lineNumber = this.getLineNumber(content, match.index);
        violations.push({
          rule: this.ruleId,
          source: filePath,
          category: this.category,
          line: lineNumber,
          column: 1,
          message:
            "Insecure session cookie: NextAuth configuration missing httpOnly for session cookies",
          severity: "error",
        });
      }
      return;
    }

    // Handle Nuxt useCookie patterns
    if (matchText.includes("useCookie(")) {
      if (
        this.isSessionCookie(cookieName, matchText) &&
        !this.hasHttpOnlyFlag(cookieConfig)
      ) {
        const lineNumber = this.getLineNumber(content, match.index);
        violations.push({
          rule: this.ruleId,
          source: filePath,
          category: this.category,
          line: lineNumber,
          column: 1,
          message: `Insecure session cookie: Nuxt useCookie "${cookieName}" missing httpOnly attribute`,
          severity: "error",
        });
      }
      return;
    }

    // Handle other framework patterns
    if (this.isSessionCookie(cookieName, matchText)) {
      if (!this.hasHttpOnlyFlag(cookieConfig)) {
        const lineNumber = this.getLineNumber(content, match.index);
        const framework = this.detectFramework(matchText);

        violations.push({
          rule: this.ruleId,
          source: filePath,
          category: this.category,
          line: lineNumber,
          column: 1,
          message: `Insecure session cookie: ${framework} session cookie "${cookieName}" missing HttpOnly attribute`,
          severity: "error",
        });
      }
    }
  }

  /**
   * Check NextAuth configuration for httpOnly
   */
  hasHttpOnlyInNextAuthConfig(config) {
    // NextAuth typically has cookies.sessionToken.httpOnly configuration
    return (
      config.includes("httpOnly: true") ||
      (config.includes("sessionToken") && config.includes("httpOnly")) ||
      (config.includes("cookies") && config.includes("httpOnly"))
    );
  }

  /**
   * Detect framework from match text
   */
  detectFramework(matchText) {
    if (matchText.includes("@Res()") || matchText.includes("NestJS")) {
      return "NestJS";
    } else if (
      matchText.includes("NextResponse") ||
      matchText.includes("NextAuth")
    ) {
      return "Next.js";
    } else if (
      matchText.includes("useCookie") ||
      matchText.includes("$cookies") ||
      matchText.includes("nuxt")
    ) {
      return "Nuxt.js";
    }
    return "Framework";
  }

  /**
   * Clean up resources
   */
  cleanup() {
    // No resources to clean up for regex analyzer
  }

  /**
   * Check if configuration has httpOnly set to true
   */
  hasHttpOnlyTrue(configText) {
    // Remove comments to avoid false positives
    const codeOnly = configText
      .replace(/\/\/.*$/gm, "")
      .replace(/\/\*[\s\S]*?\*\//g, "");

    const httpOnlyPatterns = [
      /httpOnly\s*:\s*true/i,
      /httpOnly\s*=\s*true/i,
      /['"]httpOnly['"]\s*:\s*true/i,
      /HttpOnly/i, // For Set-Cookie header format
    ];

    // Check for explicitly disabled httpOnly (should be treated as violation)
    const httpOnlyDisabledPatterns = [
      /httpOnly\s*:\s*false/i,
      /httpOnly\s*=\s*false/i,
      /['"]httpOnly['"]\s*:\s*false/i,
    ];

    const hasHttpOnlyFalse = httpOnlyDisabledPatterns.some((pattern) =>
      pattern.test(codeOnly)
    );

    const hasHttpOnly = httpOnlyPatterns.some((pattern) =>
      pattern.test(codeOnly)
    );

    // If httpOnly is explicitly set to false, it's a violation
    if (hasHttpOnlyFalse) {
      return false; // Violation: explicitly disabled
    }

    return hasHttpOnly;
  }
}

module.exports = S032RegexBasedAnalyzer;
