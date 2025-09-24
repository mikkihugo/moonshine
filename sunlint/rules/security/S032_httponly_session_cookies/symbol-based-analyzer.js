/**
 * S032 Symbol-Based Analyzer - Set HttpOnly attribute for Session Cookies
 * Uses TypeScript compiler API for semantic analysis
 */

const ts = require("typescript");

class S032SymbolBasedAnalyzer {
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

    // Cookie methods that need security checking
    this.cookieMethods = [
      "setCookie",
      "cookie",
      "set",
      "append",
      "session",
      "setHeader",
      "writeHead",
      // Framework-specific methods
      "useCookie", // Nuxt.js
    ];
  }

  /**
   * Initialize analyzer with semantic engine
   */
  async initialize(semanticEngine) {
    this.semanticEngine = semanticEngine;
    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔧 [S032] Symbol-based analyzer initialized`);
    }
  }

  /**
   * Main analysis method for ts-morph source files
   */
  async analyze(sourceFile, filePath) {
    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔍 [S032] Symbol: Starting analysis for ${filePath}`);
    }

    const violations = [];

    try {
      // Use ts-morph API for more detailed analysis
      this.analyzeMorphSyntaxTree(sourceFile, violations);
    } catch (morphError) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: ts-morph analysis failed, trying TypeScript compiler API:`,
          morphError.message
        );
      }

      try {
        // Fallback to TypeScript compiler API
        const sourceCode = sourceFile.getFullText();
        const tsSourceFile = ts.createSourceFile(
          filePath,
          sourceCode,
          ts.ScriptTarget.Latest,
          true
        );
        this.visitNode(tsSourceFile, violations, tsSourceFile);
      } catch (tsError) {
        console.warn(
          `⚠ [S032] Symbol: Both analysis methods failed:`,
          tsError.message
        );
      }
    }

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S032] Symbol: Analysis completed. Found ${violations.length} violations`
      );
    }

    return violations;
  }

  /**
   * Analyze using ts-morph syntax tree (preferred method)
   */
  analyzeMorphSyntaxTree(sourceFile, violations) {
    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔍 [S032] Symbol: Starting ts-morph analysis`);
    }

    // Use SyntaxKind enum from ts-morph
    const SyntaxKind =
      sourceFile.getProject().getTypeChecker().compilerObject.SyntaxKind ||
      require("typescript").SyntaxKind;

    // Find all call expressions using proper SyntaxKind
    const callExpressions = sourceFile.getDescendantsOfKind(
      SyntaxKind.CallExpression
    );

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S032] Symbol: Found ${callExpressions.length} call expressions`
      );
    }

    for (const callNode of callExpressions) {
      this.checkMorphCookieMethodCall(callNode, violations, sourceFile);
    }
  }

  /**
   * Check cookie method calls using ts-morph (more accurate)
   */
  checkMorphCookieMethodCall(callNode, violations, sourceFile) {
    const methodName = this.getMorphMethodName(callNode);

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S032] Symbol: ts-morph Method call detected: "${methodName}"`
      );
    }

    if (!this.cookieMethods.includes(methodName)) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Method "${methodName}" not in cookieMethods list`
        );
      }
      return;
    }

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S032] Symbol: Method "${methodName}" found in cookieMethods, proceeding...`
      );
    }

    // Skip middleware setup patterns
    const callText = callNode.getText();
    if (
      methodName === "session" &&
      this.isMiddlewareSetup(callText, methodName)
    ) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Line ${
            callNode.getStartLineNumber?.() || "unknown"
          } - Skipping properly configured session middleware`
        );
      }
      return;
    }

    // Special handling for setHeader("Set-Cookie", [...]) pattern
    if (methodName === "setHeader") {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Special setHeader handling triggered for line ${
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

    // Check for httpOnly flag in options
    const hasHttpOnlyFlag = this.checkMorphHttpOnlyFlag(callNode);

    if (!hasHttpOnlyFlag) {
      this.addMorphViolation(
        callNode,
        violations,
        sourceFile,
        `Session cookie "${cookieName || "unknown"}" missing HttpOnly attribute`
      );
    }
  }

  /**
   * Check setHeader("Set-Cookie", [...]) pattern for insecure session cookies
   */
  checkSetHeaderCookies(callNode, violations, sourceFile) {
    try {
      const args = callNode.getArguments();
      if (!args || args.length < 2) {
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

        if (this.isSessionCookieName(cookieName)) {
          const hasHttpOnly = cookieString.toLowerCase().includes("httponly");

          if (!hasHttpOnly) {
            this.addMorphViolation(
              callNode,
              violations,
              sourceFile,
              `Session cookie "${cookieName}" in Set-Cookie header missing HttpOnly attribute`
            );
          }
        }
      }
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Error checking setHeader cookies:`,
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
          `🔍 [S032] Symbol: Error extracting cookie strings:`,
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
   * Get method name from ts-morph call expression
   */
  getMorphMethodName(callNode) {
    try {
      const expression = callNode.getExpression();

      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Expression kind: ${expression.getKindName()}, text: "${expression
            .getText()
            .substring(0, 30)}..."`
        );
      }

      // Handle PropertyAccessExpression (e.g., res.cookie)
      if (expression.getKindName() === "PropertyAccessExpression") {
        const name = expression.getName();
        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔍 [S032] Symbol: PropertyAccess method name: "${name}"`
          );
        }
        return name;
      }
      // Handle Identifier (e.g., session)
      else if (expression.getKindName() === "Identifier") {
        const name = expression.getText();
        if (process.env.SUNLINT_DEBUG) {
          console.log(`🔍 [S032] Symbol: Identifier method name: "${name}"`);
        }
        return name;
      }
      // Handle CallExpression chains
      else if (expression.getKindName() === "CallExpression") {
        // This is a chained call, look for the immediate property access
        const parentText = expression.getText();
        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔍 [S032] Symbol: CallExpression chain: "${parentText.substring(
              0,
              50
            )}..."`
          );
        }

        // Try to extract method name from the chain
        const methodMatch = parentText.match(/\.(\w+)\s*\([^)]*\)\s*$/);
        if (methodMatch) {
          const name = methodMatch[1];
          if (process.env.SUNLINT_DEBUG) {
            console.log(`🔍 [S032] Symbol: Extracted from chain: "${name}"`);
          }
          return name;
        }
      }
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Error getting method name:`,
          error.message
        );
      }
    }

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S032] Symbol: Could not extract method name, returning empty string`
      );
    }
    return "";
  }

  /**
   * Extract cookie name from ts-morph method call
   */
  extractMorphCookieName(callNode) {
    try {
      const args = callNode.getArguments();
      if (args && args.length > 0) {
        const methodName = this.getMorphMethodName(callNode);

        // Handle setCookie(event, "cookieName", "value", options) pattern
        if (methodName === "setCookie" && args.length >= 2) {
          const secondArg = args[1]; // Cookie name is second argument
          if (secondArg && secondArg.getText) {
            const text = secondArg.getText();
            return text.replace(/['"]/g, ""); // Remove quotes
          }
        }

        // Handle standard cookie methods (cookieName is first argument)
        const firstArg = args[0];
        if (firstArg && firstArg.getText) {
          const text = firstArg.getText();
          return text.replace(/['"]/g, ""); // Remove quotes
        }
      }
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Error extracting cookie name:`,
          error.message
        );
      }
    }
    return null;
  }

  /**
   * Check for httpOnly flag in ts-morph method call options
   */
  checkMorphHttpOnlyFlag(callNode) {
    try {
      const args = callNode.getArguments();
      if (!args || args.length < 2) {
        return false;
      }

      const methodName = this.getMorphMethodName(callNode);

      // For setCookie(event, name, value, options), options is at index 3
      let startIndex = 1;
      if (methodName === "setCookie" && args.length >= 4) {
        startIndex = 3; // Start checking from the options argument
      }

      // Check options object (usually second or third argument, or fourth for setCookie)
      for (let i = startIndex; i < args.length; i++) {
        const arg = args[i];
        if (arg && arg.getKind) {
          const SyntaxKind = require("typescript").SyntaxKind;

          if (arg.getKind() === SyntaxKind.ObjectLiteralExpression) {
            // ObjectLiteralExpression
            let text = arg.getText();

            if (process.env.SUNLINT_DEBUG) {
              console.log(
                `🔍 [S032] Symbol: Checking object literal: ${text.substring(
                  0,
                  200
                )}...`
              );
            }

            // Remove comments to avoid false positives
            const textWithoutComments = text
              .replace(/\/\/.*$/gm, "")
              .replace(/\/\*[\s\S]*?\*\//g, "");

            // Check for explicitly disabled httpOnly (should be treated as violation)
            if (
              textWithoutComments.includes("httpOnly") &&
              (textWithoutComments.includes("false") ||
                textWithoutComments.includes(": false"))
            ) {
              if (process.env.SUNLINT_DEBUG) {
                console.log(
                  `🔍 [S032] Symbol: HttpOnly explicitly disabled (violation)`
                );
              }
              return false; // Violation: explicitly disabled
            }

            // Check for explicitly enabled httpOnly
            if (
              textWithoutComments.includes("httpOnly") &&
              (textWithoutComments.includes("true") ||
                textWithoutComments.includes(": true"))
            ) {
              if (process.env.SUNLINT_DEBUG) {
                console.log(
                  `🔍 [S032] Symbol: HttpOnly explicitly enabled (secure)`
                );
              }
              return true;
            }

            // Check for spread elements within the object literal
            const hasSpreadElements = text.includes("...");
            if (hasSpreadElements) {
              if (process.env.SUNLINT_DEBUG) {
                console.log(
                  `🔍 [S032] Symbol: Object literal contains spread elements, checking each...`
                );
              }

              // Get spread elements from the object literal
              const spreadMatches = text.match(/\.\.\.([^,}]+)/g);
              if (spreadMatches) {
                for (const spreadMatch of spreadMatches) {
                  const reference = spreadMatch.replace(/^\.\.\./g, "").trim();
                  if (process.env.SUNLINT_DEBUG) {
                    console.log(
                      `🔍 [S032] Symbol: Checking spread reference: ${reference}`
                    );
                  }

                  if (this.isSecureConfigReference(reference, callNode)) {
                    if (process.env.SUNLINT_DEBUG) {
                      console.log(
                        `🔍 [S032] Symbol: ✅ Secure spread reference detected: ${reference}`
                      );
                    }
                    return true;
                  }
                }
              }
            }

            // If no httpOnly found in literal and no secure spread elements, it's a violation
            if (process.env.SUNLINT_DEBUG) {
              console.log(
                `🔍 [S032] Symbol: Object literal missing httpOnly and no secure spreads`
              );
            }
            return false;
          } else if (
            arg.getKind() === SyntaxKind.Identifier ||
            arg.getKind() === SyntaxKind.PropertyAccessExpression
          ) {
            // Handle this.cookieConfig or variable references
            const argText = arg.getText();
            if (process.env.SUNLINT_DEBUG) {
              console.log(`🔍 [S032] Symbol: Found reference: ${argText}`);
            }

            // Check if this refers to a configuration object with httpOnly
            if (this.isSecureConfigReference(argText, callNode)) {
              if (process.env.SUNLINT_DEBUG) {
                console.log(
                  `🔍 [S032] Symbol: ✅ Secure config reference detected: ${argText}`
                );
              }
              return true;
            }
          } else if (arg.getKind() === SyntaxKind.SpreadElement) {
            // Handle spread syntax like { ...this.cookieConfig }
            const spreadText = arg.getText();
            if (process.env.SUNLINT_DEBUG) {
              console.log(
                `🔍 [S032] Symbol: Found spread element: ${spreadText}`
              );
            }

            if (this.isSecureConfigSpread(spreadText, callNode)) {
              if (process.env.SUNLINT_DEBUG) {
                console.log(
                  `🔍 [S032] Symbol: ✅ Secure config spread detected: ${spreadText}`
                );
              }
              return true;
            }
          }
        }
      }
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Error checking httpOnly flag:`,
          error.message
        );
      }
    }
    return false;
  }

  /**
   * Check if reference points to secure configuration
   */
  isSecureConfigReference(argText, callNode) {
    try {
      const sourceFile = callNode.getSourceFile();
      const fileText = sourceFile.getFullText();

      // Handle this.cookieConfig pattern
      if (argText.includes("cookieConfig") || argText.includes("config")) {
        const configName = argText.split(".").pop();

        // Look for the exact config definition and check if it contains httpOnly: true
        // More precise pattern to match the actual config object definition
        const configDefPattern = new RegExp(
          `(?:private|public|readonly|const|let|var)\\s+(?:readonly\\s+)?${configName}\\s*=\\s*{[^}]*}`,
          "gis"
        );

        const configMatch = fileText.match(configDefPattern);

        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔍 [S032] Symbol: Looking for config definition of "${configName}"`
          );
          console.log(
            `🔍 [S032] Symbol: Config match found:`,
            configMatch ? configMatch[0] : "none"
          );
        }

        if (configMatch) {
          let configContent = configMatch[0];

          // Remove comments to avoid false positives from "// Missing: httpOnly: true"
          configContent = configContent
            .replace(/\/\/.*$/gm, "")
            .replace(/\/\*[\s\S]*?\*\//g, "");

          const hasHttpOnlyTrue = /httpOnly\s*:\s*true/i.test(configContent);

          if (process.env.SUNLINT_DEBUG) {
            console.log(
              `🔍 [S032] Symbol: Config content (comments removed):`,
              configContent
            );
            console.log(
              `🔍 [S032] Symbol: httpOnly: true found:`,
              hasHttpOnlyTrue
            );
          }

          return hasHttpOnlyTrue;
        }

        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔍 [S032] Symbol: No config definition found for "${configName}"`
          );
        }

        return false;
      }

      // Handle variable references
      const varPattern = new RegExp(
        `(?:const|let|var)\\s+${argText}\\s*=\\s*{[^}]*httpOnly\\s*:\\s*true`,
        "i"
      );
      return varPattern.test(fileText);
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Error checking config reference:`,
          error.message
        );
      }
      return false;
    }
  }

  /**
   * Check if spread element contains secure configuration
   */
  isSecureConfigSpread(spreadText, callNode) {
    try {
      const sourceFile = callNode.getSourceFile();
      const fileText = sourceFile.getFullText();

      // Extract the reference from spread (e.g., ...this.cookieConfig -> this.cookieConfig)
      const reference = spreadText.replace(/^\.\.\./g, "");

      return this.isSecureConfigReference(reference, callNode);
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Error checking spread config:`,
          error.message
        );
      }
      return false;
    }
  }
  addMorphViolation(callNode, violations, sourceFile, message) {
    try {
      const start = callNode.getStart();
      const lineAndChar = sourceFile.getLineAndColumnAtPos(start);

      violations.push({
        rule: this.ruleId,
        source: sourceFile.getFilePath(),
        category: this.category,
        line: lineAndChar.line,
        column: lineAndChar.column,
        message: `Insecure session cookie: ${message}`,
        severity: "error",
      });
    } catch (error) {
      // Fallback violation without line/column info
      violations.push({
        rule: this.ruleId,
        source: sourceFile.getFilePath ? sourceFile.getFilePath() : "unknown",
        category: this.category,
        line: 1,
        column: 1,
        message: `Insecure session cookie: ${message}`,
        severity: "error",
      });
    }
  }

  // TypeScript compiler API fallback methods

  /**
   * Visit and analyze syntax tree nodes
   */
  visitNode(node, violations, sourceFile) {
    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S032] Symbol: Visiting ${ts.SyntaxKind[node.kind]} node`
      );
    }

    // Check for call expressions
    if (ts.isCallExpression(node)) {
      this.checkCookieMethodCall(node, violations, sourceFile);
    }

    // Continue traversing
    ts.forEachChild(node, (child) => {
      this.visitNode(child, violations, sourceFile);
    });
  }

  /**
   * Check cookie method calls for httpOnly flags
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
        `🔍 [S032] Symbol: Line ${lineNumber} - Method call detected: "${methodName}"`
      );
    }

    if (!this.cookieMethods.includes(methodName)) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Line ${lineNumber} - Method "${methodName}" not in cookieMethods list`
        );
      }
      return;
    }

    // Special handling for setHeader("Set-Cookie", [...]) pattern
    if (methodName === "setHeader") {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Line ${lineNumber} - Special setHeader handling triggered`
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
          `🔍 [S032] Symbol: Line ${lineNumber} - Skipping middleware setup for "${methodName}"`
        );
      }
      return;
    }

    // Check if this is setting a session-related cookie
    const cookieName = this.extractCookieName(callNode);

    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔍 [S032] Symbol: Extracted cookie name: "${cookieName}"`);
    }

    if (!this.isSessionCookie(cookieName, callNode)) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Cookie "${cookieName}" not identified as session cookie`
        );
      }
      return;
    }

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S032] Symbol: Cookie "${cookieName}" IS a session cookie, checking httpOnly flag...`
      );
    }

    // Check for httpOnly flag in options
    const hasHttpOnlyFlag = this.checkHttpOnlyFlag(callNode);

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S032] Symbol: HttpOnly flag check result: ${hasHttpOnlyFlag}`
      );
    }

    if (!hasHttpOnlyFlag) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: ⚠️ VIOLATION ADDED: Session cookie "${cookieName}" missing HttpOnly attribute`
        );
      }

      this.addViolation(
        callNode,
        violations,
        sourceFile,
        `Session cookie "${cookieName}" missing HttpOnly attribute`
      );
    }
  }

  /**
   * Check setHeader("Set-Cookie", [...]) pattern for insecure session cookies (TypeScript compiler API)
   */
  checkSetHeaderCookiesTS(callNode, violations, sourceFile) {
    if (process.env.SUNLINT_DEBUG) {
      console.log(`🔍 [S032] Symbol: checkSetHeaderCookiesTS called`);
    }

    try {
      const args = callNode.arguments;
      if (!args || args.length < 2) {
        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔍 [S032] Symbol: setHeader insufficient args: ${
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
            `🔍 [S032] Symbol: Not Set-Cookie header: "${headerName}"`
          );
        }
        return;
      }

      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Set-Cookie header detected, checking array...`
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
          `🔍 [S032] Symbol: Extracted ${cookieStrings.length} cookie strings`
        );
      }

      for (const cookieString of cookieStrings) {
        const cookieName = this.extractCookieNameFromString(cookieString);

        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔍 [S032] Symbol: Checking cookie "${cookieName}" from string: "${cookieString}"`
          );
        }

        if (this.isSessionCookieName(cookieName)) {
          const hasHttpOnly = cookieString.toLowerCase().includes("httponly");

          if (process.env.SUNLINT_DEBUG) {
            console.log(
              `🔍 [S032] Symbol: Session cookie "${cookieName}" has httpOnly: ${hasHttpOnly}`
            );
          }

          if (!hasHttpOnly) {
            this.addViolation(
              callNode,
              violations,
              sourceFile,
              `Session cookie "${cookieName}" in Set-Cookie header missing HttpOnly attribute`
            );
          }
        }
      }
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Error checking setHeader cookies:`,
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
          `🔍 [S032] Symbol: Error extracting cookie strings:`,
          error.message
        );
      }
    }

    return cookieStrings;
  }

  /**
   * Get method name from call expression
   */
  getMethodName(callNode) {
    try {
      if (callNode.expression && callNode.expression.name) {
        return callNode.expression.name.text;
      } else if (callNode.expression && callNode.expression.property) {
        return callNode.expression.property.text;
      }
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Error getting method name:`,
          error.message
        );
      }
    }
    return "";
  }

  /**
   * Extract cookie name from method call
   */
  extractCookieName(callNode) {
    try {
      if (callNode.arguments && callNode.arguments.length > 0) {
        const firstArg = callNode.arguments[0];
        if (firstArg && ts.isStringLiteral(firstArg)) {
          return firstArg.text;
        }
      }
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Error extracting cookie name:`,
          error.message
        );
      }
    }
    return null;
  }

  /**
   * Check if method setup is middleware configuration
   */
  isMiddlewareSetup(callText, methodName) {
    // Remove comments before checking for cookie configuration
    const codeOnly = callText
      .replace(/\/\/.*$/gm, "")
      .replace(/\/\*[\s\S]*?\*\//g, "");

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S032] Symbol: Checking middleware setup for method: ${methodName}`
      );
      console.log(
        `🔍 [S032] Symbol: Call text (code only): ${codeOnly.substring(
          0,
          100
        )}...`
      );
    }

    // Check for session middleware patterns
    if (methodName === "session" || codeOnly.includes("session(")) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Session middleware detected, checking for cookie config...`
        );
      }

      // Check for existing cookie configuration
      if (codeOnly.includes("cookie:")) {
        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔍 [S032] Symbol: Cookie config found, checking for httpOnly...`
          );
        }

        // Check if httpOnly is properly configured
        const httpOnlyPatterns = [
          /httpOnly\s*:\s*true/i,
          /httpOnly\s*=\s*true/i,
          /['"]httpOnly['"]\s*:\s*true/i,
        ];

        const hasProperHttpOnly = httpOnlyPatterns.some((pattern) =>
          pattern.test(codeOnly)
        );

        if (hasProperHttpOnly) {
          if (process.env.SUNLINT_DEBUG) {
            console.log(
              `🔍 [S032] Symbol: ✅ Skipping - session middleware has proper httpOnly config`
            );
          }
          return true; // Skip - properly configured
        } else {
          if (process.env.SUNLINT_DEBUG) {
            console.log(
              `🔍 [S032] Symbol: ❌ Not skipping - session middleware missing httpOnly: true`
            );
          }
          return false; // Don't skip - needs to be checked for missing httpOnly
        }
      } else {
        if (process.env.SUNLINT_DEBUG) {
          console.log(
            `🔍 [S032] Symbol: ❌ Not skipping - session middleware without cookie config (violation)`
          );
        }
        return false; // Don't skip - needs to be checked for missing cookie config
      }
    }

    // Other non-session middleware patterns can be skipped
    const nonSessionMiddlewarePatterns = [
      /middleware.*(?!session)/i, // middleware but not session
      /use\(.*(?!session)/i, // use() but not session
      /app\.use\((?!.*session)/i, // app.use() but not session
    ];

    const isNonSessionMiddleware = nonSessionMiddlewarePatterns.some(
      (pattern) => pattern.test(codeOnly)
    );

    if (isNonSessionMiddleware) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(`🔍 [S032] Symbol: ✅ Skipping - non-session middleware`);
      }
      return true;
    }

    return false; // Don't skip by default
  }

  /**
   * Check if cookie name indicates session cookie
   */
  isSessionCookie(cookieName, callNode) {
    const methodName = this.getMethodName(callNode);

    if (process.env.SUNLINT_DEBUG && methodName === "session") {
      console.log(
        `🔍 [S032] Symbol: Checking isSessionCookie for session() call with cookieName: "${cookieName}"`
      );
    }

    // For session() method calls, they ARE always session-related
    if (methodName === "session") {
      if (process.env.SUNLINT_DEBUG) {
        console.log(`🔍 [S032] Symbol: ✅ session() IS a session cookie setup`);
      }
      return true;
    }

    // Check cookie name against session indicators
    if (!cookieName) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(`🔍 [S032] Symbol: ❌ No cookie name provided`);
      }
      return false;
    }

    const lowerName = cookieName.toLowerCase();
    const isSession = this.sessionIndicators.some((indicator) =>
      lowerName.includes(indicator.toLowerCase())
    );

    if (process.env.SUNLINT_DEBUG) {
      console.log(
        `🔍 [S032] Symbol: Cookie "${cookieName}" session check: ${isSession}`
      );
    }

    return isSession;
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

  /**
   * Check for httpOnly flag in method call options
   */
  checkHttpOnlyFlag(callNode) {
    try {
      if (!callNode.arguments || callNode.arguments.length < 2) {
        return false;
      }

      // Check options object (usually second or third argument)
      for (let i = 1; i < callNode.arguments.length; i++) {
        const arg = callNode.arguments[i];
        if (ts.isObjectLiteralExpression(arg)) {
          const text = arg.getText();

          // Check for explicitly disabled httpOnly (should be treated as violation)
          if (
            text.includes("httpOnly") &&
            (text.includes("false") || text.includes(": false"))
          ) {
            if (process.env.SUNLINT_DEBUG) {
              console.log(
                `🔍 [S032] Symbol: HttpOnly explicitly disabled (violation) in TypeScript API`
              );
            }
            return false; // Violation: explicitly disabled
          }

          // Check for httpOnly: true patterns
          if (
            text.includes("httpOnly") &&
            (text.includes("true") || text.includes(": true"))
          ) {
            return true;
          }
        }
      }
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Error checking httpOnly flag:`,
          error.message
        );
      }
    }
    return false;
  }

  /**
   * Add violation to results
   */
  addViolation(callNode, violations, sourceFile, message) {
    try {
      const start = sourceFile.getLineAndCharacterOfPosition(
        callNode.getStart(sourceFile)
      );

      violations.push({
        rule: this.ruleId,
        source: sourceFile.fileName,
        category: this.category,
        line: start.line + 1,
        column: start.character + 1,
        message: `Insecure session cookie: ${message}`,
        severity: "error",
      });
    } catch (error) {
      // Fallback violation
      violations.push({
        rule: this.ruleId,
        source: sourceFile.fileName || "unknown",
        category: this.category,
        line: 1,
        column: 1,
        message: `Insecure session cookie: ${message}`,
        severity: "error",
      });
    }
  }

  /**
   * Detect framework from method call context
   */
  detectFramework(callNode, sourceFile) {
    const callText = callNode.getText();
    const fileContent = sourceFile.getFullText();

    // Check imports to detect framework
    if (
      fileContent.includes("@nestjs/common") ||
      fileContent.includes("@Res()")
    ) {
      return "NestJS";
    }

    if (
      fileContent.includes("next/server") ||
      fileContent.includes("NextResponse") ||
      fileContent.includes("NextAuth")
    ) {
      return "Next.js";
    }

    if (
      callText.includes("useCookie") ||
      fileContent.includes("defineEventHandler") ||
      fileContent.includes("setCookie")
    ) {
      return "Nuxt.js";
    }

    return "Framework";
  }

  /**
   * Enhanced method name detection with framework support
   */
  getMorphMethodName(callNode) {
    try {
      const expression = callNode.getExpression();

      // Handle property access expressions (obj.method)
      if (expression.getKind() === ts.SyntaxKind.PropertyAccessExpression) {
        const propertyName = expression.getNameNode().getText();

        // Check for chained method calls like response.cookies.set
        if (propertyName === "set" || propertyName === "cookie") {
          const objectExpression = expression.getExpression();
          if (
            objectExpression.getKind() ===
            ts.SyntaxKind.PropertyAccessExpression
          ) {
            const parentProperty = objectExpression.getNameNode().getText();
            if (parentProperty === "cookies") {
              return "set"; // For cookies.set()
            }
          }
          return propertyName;
        }

        return propertyName;
      }

      // Handle direct function calls
      if (expression.getKind() === ts.SyntaxKind.Identifier) {
        return expression.getText();
      }

      return "";
    } catch (error) {
      if (process.env.SUNLINT_DEBUG) {
        console.log(
          `🔍 [S032] Symbol: Error getting method name:`,
          error.message
        );
      }
      return "";
    }
  }
}

module.exports = S032SymbolBasedAnalyzer;
