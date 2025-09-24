/**
 * S031 Symbol-Based Analyzer - Set Secure flag for Session Cookies
 * Uses TypeScript compiler API for semantic analysis
 */

const ts = require("typescript");

class S031SymbolBasedAnalyzer {
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
      "refresh",
    ];

    // Cookie methods that need security checking
    this.cookieMethods = [
      "setCookie",
      "cookie",
      "set",
      "append",
      "session",
      "setHeader",
      "writeHead",
    ];
  }

  /**
   * Initialize analyzer with semantic engine
   */
  async initialize(semanticEngine) {
    this.semanticEngine = semanticEngine;
    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔧 [S031] Symbol-based analyzer initialized`);
    }
  }

  /**
   * Analyze source file for insecure session cookies
   */
  async analyze(sourceFile, filePath) {
    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔍 [S031] Symbol-based analysis for: ${filePath}`);
    }

    const violations = [];

    try {
      // Get the TypeScript compiler SourceFile from ts-morph
      const compilerNode = sourceFile.compilerNode || sourceFile._compilerNode;
      if (!compilerNode) {
        if (process.env.SUNLINT_DEBUG) {
          console.log(`⚠️ [S031] No compiler node found, using ts-morph API`);
        }
        // Use ts-morph API instead
        this.visitMorphNode(sourceFile, violations, sourceFile);
      } else {
        if (process.env.SUNLINT_DEBUG) {
          console.log(`✅ [S031] Using TypeScript compiler API`);
        }
        // Traverse AST to find cookie-related code
        this.visitNode(compilerNode, violations, compilerNode);
      }
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.error(`❌ [S031] Symbol analysis error:`, error);
      }
      throw error;
    }

    return violations;
  }

  /**
   * Visit ts-morph nodes recursively
   */
  visitMorphNode(node, violations, sourceFile) {
    if (process.env.SUNLINT_DEBUG) {
      const nodeKind = node.getKindName ? node.getKindName() : "Unknown";
      if (
        nodeKind &&
        (nodeKind.includes("Call") || nodeKind.includes("Property"))
      ) {
        console.log(`🔍 [S031] Symbol: Visiting ${nodeKind} node (ts-morph)`);
      }
    }

    // Check for call expressions
    if (node.getKind && node.getKind() === 208) {
      // CallExpression
      this.checkMorphCookieMethodCall(node, violations, sourceFile);
    }

    // Continue traversing children
    if (node.getChildren) {
      node.getChildren().forEach((child) => {
        this.visitMorphNode(child, violations, sourceFile);
      });
    }
  }

  /**
   * Check cookie method calls using ts-morph API
   */
  checkMorphCookieMethodCall(callNode, violations, sourceFile) {
    const methodName = this.getMorphMethodName(callNode);

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S031] Symbol: ts-morph Method call detected: "${methodName}"`
      );
    }

    if (!this.cookieMethods.includes(methodName)) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: Method "${methodName}" not in cookieMethods list`
        );
      }
      return;
    }

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S031] Symbol: Method "${methodName}" found in cookieMethods, proceeding...`
      );
    }

    // Special handling for setHeader("Set-Cookie", [...]) pattern
    if (methodName === "setHeader") {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: Special setHeader handling triggered for line ${
            callNode.getStartLineNumber?.() || "unknown"
          }`
        );
      }
      this.checkSetHeaderCookies(callNode, violations, sourceFile);
      return;
    }

    // Check if this is setting a session-related cookie
    const cookieName = this.extractMorphCookieName(callNode);
    if (!this.isSessionCookie(cookieName, callNode)) {
      return;
    }

    // Check for secure flag in options
    const hasSecureFlag = this.checkMorphSecureFlag(callNode);

    if (!hasSecureFlag) {
      this.addMorphViolation(
        callNode,
        violations,
        sourceFile,
        `Session cookie "${cookieName || "unknown"}" missing Secure flag`
      );
    }
  }

  /**
   * Extract method name from ts-morph call expression
   */
  getMorphMethodName(callNode) {
    try {
      const expression = callNode.getExpression();
      if (expression && expression.getKind() === 201) {
        // PropertyAccessExpression
        return expression.getName();
      }
      if (expression && expression.getText) {
        return expression.getText().split(".").pop();
      }
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: Error getting method name:`,
          error.message
        );
      }
    }
    return "";
  }

  /**
   * Check setHeader("Set-Cookie", [...]) pattern for insecure session cookies
   */
  checkSetHeaderCookies(callNode, violations, sourceFile) {
    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S031] Symbol: checkSetHeaderCookies called for line ${
          callNode.getStartLineNumber?.() || "unknown"
        }`
      );
    }

    try {
      const args = callNode.getArguments();
      if (!args || args.length < 2) {
        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔍 [S031] Symbol: setHeader insufficient args: ${
              args?.length || 0
            }`
          );
        }
        return;
      }

      // Check if first argument is "Set-Cookie"
      const firstArg = args[0];
      const headerName = firstArg.getText().replace(/['"]/g, "");

      if (headerName !== "Set-Cookie") {
        return;
      }

      // Get the array of cookie strings from second argument
      const secondArg = args[1];
      if (!secondArg) {
        return;
      }

      // Parse cookie strings from array
      const cookieStrings = this.extractCookieStringsFromArray(secondArg);

      for (const cookieString of cookieStrings) {
        const cookieName = this.extractCookieNameFromString(cookieString);

        if (this.isSessionCookie(cookieName, null)) {
          const hasSecure = cookieString.toLowerCase().includes("secure");

          if (!hasSecure) {
            this.addMorphViolation(
              callNode,
              violations,
              sourceFile,
              `Session cookie "${cookieName}" in Set-Cookie header missing Secure attribute`
            );
          }
        }
      }
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: Error checking setHeader cookies:`,
          error.message
        );
      }
    }
  }

  /**
   * Extract cookie strings from array literal or template strings
   */
  extractCookieStringsFromArray(arrayNode) {
    const cookieStrings = [];

    try {
      if (arrayNode.getKind() === 196) {
        // ArrayLiteralExpression
        const elements = arrayNode.getElements();

        for (const element of elements) {
          let cookieString = element.getText();

          // Remove quotes and template literal markers
          cookieString = cookieString
            .replace(/^[`'"]/g, "")
            .replace(/[`'"]$/g, "");

          // Handle template literals with variables
          if (cookieString.includes("${")) {
            // Extract cookie name from template pattern like `auth=${tokens.auth}; ...`
            const match = cookieString.match(/^(\w+)=/);
            if (match) {
              cookieStrings.push(cookieString);
            }
          } else {
            cookieStrings.push(cookieString);
          }
        }
      }
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: Error extracting cookie strings:`,
          error.message
        );
      }
    }

    return cookieStrings;
  }

  /**
   * Extract cookie name from cookie string like "auth=value; HttpOnly; ..."
   */
  extractCookieNameFromString(cookieString) {
    try {
      const match = cookieString.match(/^(\w+)=/);
      return match ? match[1] : null;
    } catch (error) {
      return null;
    }
  }

  /**
   * Extract cookie name from ts-morph method call
   */
  extractMorphCookieName(callNode) {
    try {
      const args = callNode.getArguments();
      if (args && args.length > 0) {
        const firstArg = args[0];
        if (firstArg && firstArg.getText) {
          const text = firstArg.getText();
          return text.replace(/['"]/g, ""); // Remove quotes
        }
      }
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: Error extracting cookie name:`,
          error.message
        );
      }
    }
    return null;
  }

  /**
   * Check for secure flag in ts-morph method call options
   */
  checkMorphSecureFlag(callNode) {
    try {
      const args = callNode.getArguments();
      if (!args || args.length < 2) {
        return false;
      }

      // Check options object (usually second or third argument)
      for (let i = 1; i < args.length; i++) {
        const arg = args[i];
        if (arg && arg.getKind && arg.getKind() === 195) {
          // ObjectLiteralExpression
          const text = arg.getText();
          if (
            text.includes("secure") &&
            (text.includes("true") || text.includes(": true"))
          ) {
            return true;
          }
        }
      }
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: Error checking secure flag:`,
          error.message
        );
      }
    }
    return false;
  }

  /**
   * Add violation using ts-morph API
   */
  addMorphViolation(node, violations, sourceFile, message) {
    try {
      const start = node.getStart();
      const lineAndColumn = sourceFile.getLineAndColumnAtPos(start);
      const source = node.getText();

      violations.push({
        ruleId: this.ruleId,
        source: source,
        category: this.category,
        line: lineAndColumn.line,
        column: lineAndColumn.column,
        message: `Insecure session cookie: ${message}`,
        severity: "error",
      });

      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: Added violation at line ${lineAndColumn.line}`
        );
      }
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.error(
          `🔍 [S031] Symbol: Error adding violation:`,
          error.message
        );
      }
    }
  }

  /**
   * Visit AST nodes recursively
   */
  visitNode(node, violations, sourceFile) {
    if (process.env.SUNLINT_DEBUG) {
      const nodeKind = ts.SyntaxKind[node.kind];
      if (
        nodeKind &&
        (nodeKind.includes("Call") || nodeKind.includes("Property"))
      ) {
        console.log(`🔍 [S031] Symbol: Visiting ${nodeKind} node`);
      }
    }

    // Check for cookie setting method calls
    if (ts.isCallExpression(node)) {
      this.checkCookieMethodCall(node, violations, sourceFile);
    }

    // Check for property assignments (e.g., response.cookie = ...)
    if (ts.isPropertyAssignment(node) || ts.isBinaryExpression(node)) {
      this.checkCookiePropertyAssignment(node, violations, sourceFile);
    }

    // Continue traversing
    ts.forEachChild(node, (child) => {
      this.visitNode(child, violations, sourceFile);
    });
  }

  /**
   * Check cookie method calls for security flags
   */
  checkCookieMethodCall(callNode, violations, sourceFile) {
    const methodName = this.getMethodName(callNode);

    // Get line number for debugging
    let lineNumber = "unknown";
    try {
      const start = sourceFile.getLineAndCharacterOfPosition(
        callNode.getStart(sourceFile)
      );
      lineNumber = start.line + 1;
    } catch (error) {
      // Ignore line number errors
    }

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S031] Symbol: Line ${lineNumber} - Method call detected: "${methodName}"`
      );
    }

    if (!this.cookieMethods.includes(methodName)) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: Line ${lineNumber} - Method "${methodName}" not in cookieMethods list`
        );
      }
      return;
    }

    // Special handling for setHeader("Set-Cookie", [...]) pattern
    if (methodName === "setHeader") {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: Line ${lineNumber} - Special setHeader handling triggered`
        );
      }
      this.checkSetHeaderCookiesTS(callNode, violations, sourceFile);
      return;
    }

    // Skip middleware setup patterns
    const callText = callNode.getText();
    if (this.isMiddlewareSetup(callText, methodName)) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: Line ${lineNumber} - Skipping middleware setup for "${methodName}"`
        );
      }
      return;
    }

    // Check if this is setting a session-related cookie
    const cookieName = this.extractCookieName(callNode);

    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔍 [S031] Symbol: Extracted cookie name: "${cookieName}"`);
    }

    if (!this.isSessionCookie(cookieName, callNode)) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: Cookie "${cookieName}" not identified as session cookie`
        );
      }
      return;
    }

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S031] Symbol: Cookie "${cookieName}" IS a session cookie, checking secure flag...`
      );
    }

    // Check for secure flag in options
    const hasSecureFlag = this.checkSecureFlag(callNode);

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S031] Symbol: Secure flag check result: ${hasSecureFlag}`
      );
    }

    if (!hasSecureFlag) {
      // Improve message for session middleware
      let violationMessage;
      if (methodName === "session" && (!cookieName || cookieName === "null")) {
        violationMessage = `Session middleware missing secure cookie configuration`;
      } else {
        violationMessage = `Session cookie "${
          cookieName || "unknown"
        }" missing Secure flag`;
      }

      this.addViolation(callNode, violations, sourceFile, violationMessage);

      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: ⚠️ VIOLATION ADDED: ${violationMessage}`
        );
      }
    } else {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: ✅ Cookie "${cookieName}" has secure flag`
        );
      }
    }
  }

  /**
   * Check if this is middleware setup rather than direct cookie setting
   */
  isMiddlewareSetup(callText, methodName) {
    if (process.env.SUNLINT_DEBUG && methodName === "session") {
      console.log(`🔍 [S031] Symbol: Checking middleware for session call`);
      console.log(`🔍 [S031] Symbol: Full callText: "${callText}"`);
      console.log(
        `🔍 [S031] Symbol: Contains "cookie:": ${callText.includes("cookie:")}`
      );
    }

    // session() calls inside app.use() with proper cookie config can be skipped
    if (methodName === "session" && callText.includes("cookie:")) {
      // Remove comments to avoid false matches
      const codeOnly = callText
        .replace(/\/\/.*$/gm, "")
        .replace(/\/\*[\s\S]*?\*\//g, "");

      if (process.env.SUNLINT_DEBUG) {
        console.log(`🔍 [S031] Symbol: Code only (no comments): "${codeOnly}"`);
      }

      // Check if the cookie config has secure: true (in actual code, not comments)
      const cookieConfigMatch = codeOnly.match(/cookie:\s*{[^}]*}/s);
      if (cookieConfigMatch) {
        const cookieConfig = cookieConfigMatch[0];

        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔍 [S031] Symbol: Found cookie config: "${cookieConfig}"`
          );
          console.log(
            `🔍 [S031] Symbol: Contains "secure:": ${cookieConfig.includes(
              "secure:"
            )}`
          );
        }

        if (
          cookieConfig.includes("secure:") &&
          (cookieConfig.includes("secure: true") ||
            cookieConfig.includes("secure:true"))
        ) {
          if (process.env.SUNLINT_DEBUG) {
            console.log(
              `🔍 [S031] Symbol: ✅ Skipping secure session middleware`
            );
          }
          return true; // Skip secure middleware setup
        }
      }
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: ❌ Not skipping - has cookie config but no secure: true`
        );
      }
      return false; // Don't skip insecure middleware setup
    }

    // For session() without cookie config, check if it's a violation case
    if (methodName === "session") {
      // If it's in app.use() but has no cookie config, it's likely a violation
      if (callText.includes("app.use(") || callText.includes(".use(")) {
        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔍 [S031] Symbol: ❌ Not skipping - session middleware without cookie config (violation)`
          );
        }
        return false; // Don't skip - needs to be checked for missing cookie config
      }
    }

    // Other non-session middleware patterns can be skipped
    const nonSessionMiddlewarePatterns = [
      /middleware.*(?!session)/i, // middleware but not session
      /setup.*(?!session)/i, // setup but not session
    ];

    const shouldSkip = nonSessionMiddlewarePatterns.some((pattern) =>
      pattern.test(callText)
    );
    if (process.env.SUNLINT_DEBUG && shouldSkip) {
      console.log(`🔍 [S031] Symbol: ✅ Skipping non-session middleware`);
    }

    return shouldSkip;
  }

  /**
   * Check property assignments for cookie security
   */
  checkCookiePropertyAssignment(node, violations, sourceFile) {
    const nodeText = node.getText(sourceFile);

    // Check for document.cookie assignments
    if (
      nodeText.includes("document.cookie") ||
      nodeText.includes("Set-Cookie")
    ) {
      if (
        this.isSessionCookieString(nodeText) &&
        !this.hasSecureInString(nodeText)
      ) {
        this.addViolation(
          node,
          violations,
          sourceFile,
          "Session cookie assignment missing Secure flag"
        );
      }
    }
  }

  /**
   * Extract method name from call expression
   */
  getMethodName(callNode) {
    if (ts.isPropertyAccessExpression(callNode.expression)) {
      return callNode.expression.name.text;
    }
    if (ts.isIdentifier(callNode.expression)) {
      return callNode.expression.text;
    }
    return "";
  }

  /**
   * Extract cookie name from method call
   */
  extractCookieName(callNode) {
    if (callNode.arguments && callNode.arguments.length > 0) {
      const firstArg = callNode.arguments[0];
      if (ts.isStringLiteral(firstArg)) {
        return firstArg.text;
      }
      if (ts.isIdentifier(firstArg)) {
        return firstArg.text;
      }
    }
    return null;
  }

  /**
   * Check if cookie name indicates session cookie
   */
  isSessionCookie(cookieName, callNode) {
    const methodName = this.getMethodName(callNode);

    if (process.env.SUNLINT_DEBUG && methodName === "session") {
      console.log(
        `🔍 [S031] Symbol: Checking isSessionCookie for session() call with cookieName: "${cookieName}"`
      );
    }

    // For session() method calls, they ARE always session-related
    if (methodName === "session") {
      if (process.env.SUNLINT_DEBUG) {
        console.log(`🔍 [S031] Symbol: ✅ session() IS a session cookie setup`);
      }
      return true;
    }

    if (!cookieName || cookieName === "null" || cookieName === "unknown") {
      // If no explicit name, check call context more carefully
      const callText = callNode.getText();

      // Skip if it's obviously not a session cookie setting
      if (
        callText.includes(".json(") ||
        callText.includes(".status(") ||
        callText.includes("generateToken") ||
        callText.includes("authenticateUser")
      ) {
        return false;
      }

      return this.sessionIndicators.some((indicator) =>
        callText.toLowerCase().includes(indicator.toLowerCase())
      );
    }

    return this.sessionIndicators.some((indicator) =>
      cookieName.toLowerCase().includes(indicator.toLowerCase())
    );
  }

  /**
   * Check if string contains session cookie indicators
   */
  isSessionCookieString(text) {
    const lowerText = text.toLowerCase();
    return this.sessionIndicators.some((indicator) =>
      lowerText.includes(indicator.toLowerCase())
    );
  }

  /**
   * Check for Secure flag in method call options
   */
  checkSecureFlag(callNode) {
    if (!callNode.arguments || callNode.arguments.length < 1) {
      return false;
    }

    // For session() middleware, check if cookie config exists and has secure
    const methodName = this.getMethodName(callNode);
    if (methodName === "session") {
      return this.checkSessionMiddlewareSecure(callNode);
    }

    // For regular cookie methods, check options object (usually second or third argument)
    if (callNode.arguments.length < 2) {
      return false;
    }

    for (let i = 1; i < callNode.arguments.length; i++) {
      const arg = callNode.arguments[i];
      if (ts.isObjectLiteralExpression(arg)) {
        if (this.hasSecureProperty(arg)) {
          return true;
        }

        // Check for config references
        const argText = arg.getText();
        if (this.hasSecureConfigReference(argText)) {
          return true;
        }
      } else {
        // Check if argument references a secure config
        const argText = arg.getText();
        if (this.hasSecureConfigReference(argText)) {
          return true;
        }
      }
    }

    return false;
  }

  /**
   * Check session middleware for cookie.secure configuration
   */
  checkSessionMiddlewareSecure(callNode) {
    if (!callNode.arguments || callNode.arguments.length === 0) {
      return false;
    }

    // Session config is usually the first argument
    const configArg = callNode.arguments[0];
    if (!ts.isObjectLiteralExpression(configArg)) {
      return false;
    }

    // Look for cookie property
    for (const property of configArg.properties) {
      if (ts.isPropertyAssignment(property)) {
        const propName = property.name;
        if (ts.isIdentifier(propName) && propName.text === "cookie") {
          // Check cookie object for secure property
          if (ts.isObjectLiteralExpression(property.initializer)) {
            const cookieObj = property.initializer;

            // Look for secure property in cookie config
            for (const cookieProp of cookieObj.properties) {
              if (ts.isPropertyAssignment(cookieProp)) {
                const cookiePropName = cookieProp.name;
                if (
                  ts.isIdentifier(cookiePropName) &&
                  cookiePropName.text === "secure"
                ) {
                  // Check if secure is true, or a variable (like isProduction)
                  const secureValue = cookieProp.initializer;
                  if (secureValue.kind === ts.SyntaxKind.TrueKeyword) {
                    return true; // explicit secure: true
                  }
                  if (ts.isIdentifier(secureValue)) {
                    // Variable like isProduction - assume secure
                    return true;
                  }
                  if (secureValue.kind === ts.SyntaxKind.FalseKeyword) {
                    return false; // explicit secure: false
                  }
                  // Any other expression - assume secure (conservative)
                  return true;
                }
              }
            }
            return false; // cookie config exists but no secure property
          }

          // Check if cookie references a secure config
          const cookieText = property.initializer.getText();
          if (this.hasSecureConfigReference(cookieText)) {
            return true;
          }
          return false; // cookie property exists but not secure
        }
      }
    }

    // No cookie property found = missing cookie config = violation
    return false;
  }

  /**
   * Check if text contains secure config references
   */
  hasSecureConfigReference(text) {
    const secureConfigPatterns = [
      /\bcookieConfig\b/i,
      /\bsecureConfig\b/i,
      /\.\.\..*config/i,
      /secureOptions/i,
      /cookieDefaults/i,
      /httpOnly.*secure/i,
      /secure.*httpOnly/i,
    ];

    return secureConfigPatterns.some((pattern) => pattern.test(text));
  }

  /**
   * Check if object has secure property set to true
   */
  hasSecureProperty(objectNode) {
    for (const property of objectNode.properties) {
      if (ts.isPropertyAssignment(property)) {
        const propName = property.name;
        if (ts.isIdentifier(propName) && propName.text === "secure") {
          // Check if value is true
          if (property.initializer.kind === ts.SyntaxKind.TrueKeyword) {
            return true;
          }
        }
      }
    }
    return false;
  }

  /**
   * Check if string contains Secure flag
   */
  hasSecureInString(text) {
    const securePatterns = [
      /secure\s*[:=]\s*true/i,
      /;\s*secure\s*[;\s]/i,
      /;\s*secure$/i,
      /secure\s*=\s*true/i,
    ];

    return securePatterns.some((pattern) => pattern.test(text));
  }

  /**
   * Add violation to results
   */
  addViolation(node, violations, sourceFile, message) {
    const start = sourceFile.getLineAndCharacterOfPosition(
      node.getStart(sourceFile)
    );
    const source = node.getText(sourceFile);

    violations.push({
      ruleId: this.ruleId,
      source: source,
      category: this.category,
      line: start.line + 1,
      column: start.character + 1,
      message: `Insecure session cookie: ${message}`,
      severity: "error",
    });
  }

  /**
   * Check setHeader("Set-Cookie", [...]) pattern for insecure session cookies (TypeScript compiler API)
   */
  checkSetHeaderCookiesTS(callNode, violations, sourceFile) {
    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔍 [S031] Symbol: checkSetHeaderCookiesTS called`);
    }

    try {
      const args = callNode.arguments;
      if (!args || args.length < 2) {
        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔍 [S031] Symbol: setHeader insufficient args: ${
              args?.length || 0
            }`
          );
        }
        return;
      }

      // Check if first argument is "Set-Cookie"
      const firstArg = args[0];
      let headerName = "";
      if (firstArg.kind === ts.SyntaxKind.StringLiteral) {
        headerName = firstArg.text;
      }

      if (headerName !== "Set-Cookie") {
        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔍 [S031] Symbol: Not Set-Cookie header: "${headerName}"`
          );
        }
        return;
      }

      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: Set-Cookie header detected, checking array...`
        );
      }

      // Get the array of cookie strings from second argument
      const secondArg = args[1];
      if (!secondArg) {
        return;
      }

      // Parse cookie strings from array
      const cookieStrings = this.extractCookieStringsFromArrayTS(secondArg);

      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: Extracted ${cookieStrings.length} cookie strings`
        );
      }

      for (const cookieString of cookieStrings) {
        const cookieName = this.extractCookieNameFromString(cookieString);

        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔍 [S031] Symbol: Checking cookie "${cookieName}" from string: "${cookieString}"`
          );
        }

        if (this.isSessionCookieName(cookieName)) {
          const hasSecure = cookieString.toLowerCase().includes("secure");

          if (process.env.SUNLINT_DEBUG) {
            console.log(
              `🔍 [S031] Symbol: Session cookie "${cookieName}" has secure: ${hasSecure}`
            );
          }

          if (!hasSecure) {
            this.addViolation(
              callNode,
              violations,
              sourceFile,
              `Session cookie "${cookieName}" in Set-Cookie header missing Secure attribute`
            );
          }
        }
      }
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: Error checking setHeader cookies:`,
          error.message
        );
      }
    }
  }

  /**
   * Extract cookie strings from array literal (TypeScript compiler API)
   */
  extractCookieStringsFromArrayTS(arrayNode) {
    const cookieStrings = [];

    try {
      if (arrayNode.kind === ts.SyntaxKind.ArrayLiteralExpression) {
        const elements = arrayNode.elements;

        for (const element of elements) {
          let cookieString = "";

          if (element.kind === ts.SyntaxKind.StringLiteral) {
            cookieString = element.text;
          } else if (
            element.kind === ts.SyntaxKind.TemplateExpression ||
            element.kind === ts.SyntaxKind.NoSubstitutionTemplateLiteral
          ) {
            // Handle template literals
            cookieString = element.getText();
            // Remove backticks
            cookieString = cookieString.replace(/^`/, "").replace(/`$/, "");
          }

          if (cookieString) {
            cookieStrings.push(cookieString);
          }
        }
      }
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S031] Symbol: Error extracting cookie strings:`,
          error.message
        );
      }
    }

    return cookieStrings;
  }

  /**
   * Check if cookie name indicates session cookie (for setHeader pattern)
   */
  isSessionCookieName(cookieName) {
    if (!cookieName) return false;

    const lowerName = cookieName.toLowerCase();

    // Check against session cookie patterns
    return this.sessionIndicators.some((keyword) =>
      lowerName.includes(keyword.toLowerCase())
    );
  }
}

module.exports = S031SymbolBasedAnalyzer;
