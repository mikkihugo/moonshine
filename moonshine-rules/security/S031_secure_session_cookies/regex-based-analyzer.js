/**
 * S031 Regex-Based Analyzer - Set Secure flag for Session Cookies
 * Fallback analysis using regex patterns

 */

const fs = require("fs");

class S031RegexBasedAnalyzer {
  constructor(semanticEngine = null) {
    this.semanticEngine = semanticEngine;
    this.ruleId = "S031";
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
    ];

    // Regex patterns for cookie detection
    this.cookiePatterns = [
      // Express/Node.js patterns
      /res\.cookie\s*\(\s*['"`]([^'"`]+)['"`]\s*,([^)]+)\)/gi,
      /response\.cookie\s*\(\s*['"`]([^'"`]+)['"`]\s*,([^)]+)\)/gi,
      /\.setCookie\s*\(\s*['"`]([^'"`]+)['"`]\s*,([^)]+)\)/gi,

      // Set-Cookie header patterns
      /setHeader\s*\(\s*['"`]Set-Cookie['"`]\s*,\s*['"`]([^'"`]+)['"`]\s*\)/gi,
      /writeHead\s*\([^,]*,\s*{[^}]*['"`]Set-Cookie['"`]\s*:\s*['"`]([^'"`]+)['"`]/gi,

      // Document.cookie assignments
      /document\.cookie\s*=\s*['"`]([^'"`]+)['"`]/gi,

      // Session middleware patterns
      /session\s*\(\s*{([^}]+)}/gi,
      /\.use\s*\(\s*session\s*\(\s*{([^}]+)}/gi,
    ];
  }

  /**
   * Initialize analyzer

   */
  async initialize(semanticEngine) {
    this.semanticEngine = semanticEngine;
    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔧 [S031] Regex-based analyzer initialized`);
    }
  }

  /**
   * Analyze file content using regex patterns

   */
  async analyze(filePath) {
    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔍 [S031] Regex-based analysis for: ${filePath}`);
    }

    let content;
    try {
      content = fs.readFileSync(filePath, "utf8");
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.error(`❌ [S031] File read error:`, error);
      }
      throw error;
    }

    const violations = [];
    const lines = content.split("\n");

    // Check each pattern
    for (const pattern of this.cookiePatterns) {
      this.checkPattern(pattern, content, lines, violations, filePath);
    }

    return violations;
  }

  /**
   * Check specific regex pattern for violations

   */
  checkPattern(pattern, content, lines, violations, filePath) {
    let match;
    pattern.lastIndex = 0; // Reset regex state

    while ((match = pattern.exec(content)) !== null) {
      const matchText = match[0];
      const cookieName = match[1] || "";
      const cookieOptions = match[2] || match[1] || "";

      // Check if this is a session cookie
      if (!this.isSessionCookie(cookieName, matchText)) {
        continue;
      }

      // Check if secure flag is present
      if (!this.hasSecureFlag(cookieOptions, matchText)) {
        const lineNumber = this.getLineNumber(content, match.index);

        this.addViolation(
          matchText,
          lineNumber,
          violations,
          `Session cookie "${cookieName || "unknown"}" missing Secure flag`
        );
      }
    }
  }

  /**
   * Check if cookie name or context indicates session cookie

   */
  isSessionCookie(cookieName, matchText) {
    const textToCheck = (cookieName + " " + matchText).toLowerCase();
    return this.sessionIndicators.some((indicator) =>
      textToCheck.includes(indicator.toLowerCase())
    );
  }

  /**
   * Check if secure flag is present in cookie options
   */
  hasSecureFlag(cookieOptions, fullMatch) {
    const textToCheck = cookieOptions + " " + fullMatch;

    // Check for secure config references (likely safe)
    const secureConfigPatterns = [
      /\bcookieConfig\b/i,
      /\bsecureConfig\b/i,
      /\bsafeConfig\b/i,
      /\bdefaultConfig\b/i,
      /\.\.\..*config/i, // spread operator with config
      /config.*secure/i,
    ];

    // If using a secure config reference, assume it's safe
    if (secureConfigPatterns.some((pattern) => pattern.test(textToCheck))) {
      return true;
    }

    // Check for various secure flag patterns
    const securePatterns = [
      /secure\s*:\s*true/i,
      /secure\s*=\s*true/i,
      /;\s*secure\s*[;\s]/i,
      /;\s*secure$/i,
      /['"`]\s*secure\s*['"`]/i,
      /"secure"\s*:\s*true/i,
      /'secure'\s*:\s*true/i,
      /\bsecure\b/i, // Simple secure keyword
    ];

    return securePatterns.some((pattern) => pattern.test(textToCheck));
  }

  /**
   * Get line number from content position

   */
  getLineNumber(content, position) {
    const beforeMatch = content.substring(0, position);
    return beforeMatch.split("\n").length;
  }

  /**
   * Add violation to results

   */
  addViolation(source, lineNumber, violations, message) {
    violations.push({
      ruleId: this.ruleId,
      source: source.trim(),
      category: this.category,
      line: lineNumber,
      column: 1,
      message: `Insecure session cookie: ${message}`,
      severity: "error",
    });
  }
}

module.exports = S031RegexBasedAnalyzer;
