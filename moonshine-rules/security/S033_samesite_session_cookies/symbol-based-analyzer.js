/**
 * S033 Symbol-Based Analyzer - Set SameSite attribute for Session Cookies
 * Uses TypeScript compiler API for semantic analysis
 */

const ts = require("typescript");

class S033SymbolBasedAnalyzer {
  constructor(semanticEngine = null) {
    this.semanticEngine = semanticEngine;
    this.ruleId = "S033";
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
      "next-auth",
      "user_session",
      "api_session",
      "login_session",
      "auth_token",
      "csrf_token",
      "refresh_token",
    ];

    // Cookie methods that need security checking (enhanced with framework support)
    this.cookieMethods = [
      "setCookie", // Nuxt.js H3
      "useCookie", // Nuxt.js composable
      "cookie", // Express.js res.cookie
      "set", // Next.js response.cookies.set
      "append", // Express.js res.append
      "session", // Session middleware
      "setHeader", // Node.js res.setHeader
      "writeHead", // Node.js res.writeHead
    ];

    // Acceptable SameSite values
    this.acceptableValues = ["strict", "lax", "none"];
    this.recommendedValues = ["strict", "lax"];
  }

  /**
   * Initialize analyzer with semantic engine
   */
  async initialize(semanticEngine) {
    this.semanticEngine = semanticEngine;
    if (this.verbose) {
      console.log(`🔍 [${this.ruleId}] Symbol: Semantic engine initialized`);
    }
  }

  async analyze(filePath) {
    if (this.verbose) {
      console.log(
        `🔍 [${this.ruleId}] Symbol: Starting analysis for ${filePath}`
      );
    }

    if (!this.semanticEngine) {
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: No semantic engine available, skipping`
        );
      }
      return [];
    }

    try {
      const sourceFile = this.semanticEngine.getSourceFile(filePath);
      if (!sourceFile) {
        if (this.verbose) {
          console.log(
            `🔍 [${this.ruleId}] Symbol: No source file found, trying ts-morph fallback`
          );
        }
        return await this.analyzeTsMorph(filePath);
      }

      if (this.verbose) {
        console.log(`🔧 [${this.ruleId}] Source file found, analyzing...`);
      }

      return await this.analyzeSourceFile(sourceFile, filePath);
    } catch (error) {
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: Error in analysis:`,
          error.message
        );
      }
      return [];
    }
  }

  async analyzeTsMorph(filePath) {
    try {
      if (this.verbose) {
        console.log(`🔍 [${this.ruleId}] Symbol: Starting ts-morph analysis`);
      }

      const { Project } = require("ts-morph");
      const project = new Project();
      const sourceFile = project.addSourceFileAtPath(filePath);

      return await this.analyzeSourceFile(sourceFile, filePath);
    } catch (error) {
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: ts-morph analysis failed:`,
          error.message
        );
      }
      return [];
    }
  }

  async analyzeSourceFile(sourceFile, filePath) {
    const violations = [];

    try {
      if (this.verbose) {
        console.log(`🔍 [${this.ruleId}] Symbol: Starting ts-morph analysis`);
      }

      const callExpressions = sourceFile.getDescendantsOfKind
        ? sourceFile.getDescendantsOfKind(
            require("typescript").SyntaxKind.CallExpression
          )
        : [];

      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: Found ${callExpressions.length} call expressions`
        );
      }

      for (const callNode of callExpressions) {
        try {
          if (this.verbose) {
            const expressionText = callNode.getExpression().getText();
            console.log(
              `🔍 [${this.ruleId}] Symbol: Expression kind: ${callNode
                .getExpression()
                .getKind()}, text: "${expressionText.substring(0, 50)}..."`
            );
          }

          // Handle property access expressions (e.g., res.cookie, res.setHeader)
          if (
            callNode.getExpression().getKind() ===
            require("typescript").SyntaxKind.PropertyAccessExpression
          ) {
            const methodName = callNode.getExpression().getName();

            if (this.verbose) {
              console.log(
                `🔍 [${this.ruleId}] Symbol: PropertyAccess method name: "${methodName}"`
              );
            }

            if (this.verbose) {
              console.log(
                `🔍 [${this.ruleId}] Symbol: ts-morph Method call detected: "${methodName}"`
              );
            }

            if (!this.cookieMethods.includes(methodName)) {
              if (this.verbose) {
                console.log(
                  `🔍 [${this.ruleId}] Symbol: Method "${methodName}" not in cookieMethods list`
                );
              }
              continue;
            }

            if (this.verbose) {
              console.log(
                `🔍 [${this.ruleId}] Symbol: Method "${methodName}" found in cookieMethods, proceeding...`
              );
            }

            // Special handling for setHeader method
            if (methodName === "setHeader") {
              const violation = this.analyzeSetHeaderCall(callNode, sourceFile);
              if (violation) {
                violations.push(violation);
              }
              continue;
            }

            // Analyze cookie method calls
            const violation = this.analyzeCookieCall(
              callNode,
              sourceFile,
              methodName
            );
            if (violation) {
              violations.push(violation);
            }
          }
        } catch (error) {
          if (this.verbose) {
            console.log(
              `🔍 [${this.ruleId}] Symbol: Error analyzing call expression:`,
              error.message
            );
          }
        }
      }

      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: Analysis completed. Found ${violations.length} violations`
        );
      }

      return violations;
    } catch (error) {
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: Error in source file analysis:`,
          error.message
        );
      }
      return [];
    }
  }

  analyzeSetHeaderCall(callNode, sourceFile) {
    try {
      const args = callNode.getArguments();

      if (args.length < 2) return null;

      const headerName = args[0].getText().replace(/['"]/g, "");
      if (headerName.toLowerCase() !== "set-cookie") return null;

      const line = sourceFile.getLineAndColumnAtPos(callNode.getStart()).line;

      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: Special setHeader handling triggered for line ${line}`
        );
      }

      const cookieValue = args[1].getText();

      // Check for SameSite in Set-Cookie header
      if (
        this.isSessionCookieHeader(cookieValue) &&
        !this.hasSameSiteAttribute(cookieValue)
      ) {
        const cookieName = this.extractCookieNameFromHeader(cookieValue);
        return this.createViolation(
          sourceFile,
          callNode,
          `Session cookie "${cookieName}" in Set-Cookie header missing SameSite attribute`
        );
      }

      return null;
    } catch (error) {
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: Error analyzing setHeader:`,
          error.message
        );
      }
      return null;
    }
  }

  analyzeCookieCall(callNode, sourceFile, methodName) {
    try {
      const args = callNode.getArguments();

      if (args.length < 1) return null;

      // Get cookie name using enhanced method
      const cookieName = this.extractMorphCookieName(callNode);

      if (this.verbose) {
        console.log(
          `🔍 [${
            this.ruleId
          }] Symbol: Cookie "${cookieName}" session check: ${this.isSessionCookie(
            cookieName
          )}`
        );
      }

      // Only analyze session cookies
      if (!this.isSessionCookie(cookieName)) {
        return null;
      }

      // Get framework context for better messaging
      const framework = this.detectFramework(callNode, sourceFile);

      // Check if SameSite is configured based on method type
      const hasSameSite = this.checkSameSiteInCall(callNode, methodName);

      if (!hasSameSite) {
        const frameworkMessage =
          framework !== "Framework" ? ` (${framework})` : "";
        return this.createViolation(
          sourceFile,
          callNode,
          `Session cookie "${cookieName}"${frameworkMessage} missing SameSite attribute`
        );
      }

      return null;
    } catch (error) {
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: Error analyzing cookie call:`,
          error.message
        );
      }
      return null;
    }
  }

  /**
   * Check for SameSite attribute in cookie call
   */
  checkSameSiteInCall(callNode, methodName) {
    try {
      const args = callNode.getArguments();

      // For setCookie(event, name, value, options), options is at index 3
      let optionsIndex = 2;
      if (methodName === "setCookie" && args.length >= 4) {
        optionsIndex = 3; // Options argument for setCookie
      } else if (methodName === "useCookie" && args.length >= 2) {
        optionsIndex = 1; // Options argument for useCookie
      }

      // Check if options argument exists
      if (args.length <= optionsIndex) {
        // No options object provided
        return false;
      }

      const optionsArg = args[optionsIndex];
      return this.checkSameSiteInOptions(optionsArg, callNode);
    } catch (error) {
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: Error checking SameSite in call:`,
          error.message
        );
      }
      return false;
    }
  }

  checkSameSiteInOptions(optionsArg, callNode) {
    try {
      const SyntaxKind = require("typescript").SyntaxKind;

      if (optionsArg.getKind() === SyntaxKind.ObjectLiteralExpression) {
        let text = optionsArg.getText();

        if (this.verbose) {
          console.log(
            `🔍 [${
              this.ruleId
            }] Symbol: Checking object literal: ${text.substring(0, 200)}...`
          );
        }

        // Remove comments to avoid false positives
        const textWithoutComments = text
          .replace(/\/\/.*$/gm, "")
          .replace(/\/\*[\s\S]*?\*\//g, "");

        // Check for explicitly configured SameSite
        if (this.hasSameSiteInText(textWithoutComments)) {
          if (this.verbose) {
            console.log(
              `🔍 [${this.ruleId}] Symbol: SameSite found in object literal`
            );
          }
          return true;
        }

        // Check for spread elements within the object literal
        const hasSpreadElements = text.includes("...");
        if (hasSpreadElements) {
          if (this.verbose) {
            console.log(
              `🔍 [${this.ruleId}] Symbol: Object literal contains spread elements, checking each...`
            );
          }

          const spreadMatches = text.match(/\.\.\.([^,}]+)/g);
          if (spreadMatches) {
            for (const spreadMatch of spreadMatches) {
              const reference = spreadMatch.replace(/^\.\.\./g, "").trim();
              if (this.verbose) {
                console.log(
                  `🔍 [${this.ruleId}] Symbol: Checking spread reference: ${reference}`
                );
              }

              if (this.isSecureConfigReference(reference, callNode)) {
                if (this.verbose) {
                  console.log(
                    `🔍 [${this.ruleId}] Symbol: ✅ Secure spread reference detected: ${reference}`
                  );
                }
                return true;
              }
            }
          }
        }

        if (this.verbose) {
          console.log(
            `🔍 [${this.ruleId}] Symbol: Object literal missing SameSite and no secure spreads`
          );
        }
        return false;
      } else if (
        optionsArg.getKind() === SyntaxKind.Identifier ||
        optionsArg.getKind() === SyntaxKind.PropertyAccessExpression
      ) {
        const argText = optionsArg.getText();
        if (this.verbose) {
          console.log(
            `🔍 [${this.ruleId}] Symbol: Found reference: ${argText}`
          );
        }

        if (this.isSecureConfigReference(argText, callNode)) {
          if (this.verbose) {
            console.log(
              `🔍 [${this.ruleId}] Symbol: ✅ Secure config reference detected: ${argText}`
            );
          }
          return true;
        }
      }

      return false;
    } catch (error) {
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: Error checking SameSite in options:`,
          error.message
        );
      }
      return false;
    }
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

        // Look for the exact config definition and check if it contains sameSite
        const configDefPattern = new RegExp(
          `(?:private|public|readonly|const|let|var)\\s+(?:readonly\\s+)?${configName}\\s*=\\s*{[^}]*}`,
          "gis"
        );

        const configMatch = fileText.match(configDefPattern);

        if (this.verbose) {
          console.log(
            `🔍 [${this.ruleId}] Symbol: Looking for config definition of "${configName}"`
          );
          console.log(
            `🔍 [${this.ruleId}] Symbol: Config match found:`,
            configMatch ? configMatch[0] : "none"
          );
        }

        if (configMatch) {
          let configContent = configMatch[0];

          // Remove comments to avoid false positives
          configContent = configContent
            .replace(/\/\/.*$/gm, "")
            .replace(/\/\*[\s\S]*?\*\//g, "");

          const hasSameSite = this.hasSameSiteInText(configContent);

          if (this.verbose) {
            console.log(
              `🔍 [${this.ruleId}] Symbol: Config content (comments removed):`,
              configContent
            );
            console.log(
              `🔍 [${this.ruleId}] Symbol: SameSite found:`,
              hasSameSite
            );
          }

          return hasSameSite;
        }

        if (this.verbose) {
          console.log(
            `🔍 [${this.ruleId}] Symbol: No config definition found for "${configName}"`
          );
        }

        return false;
      }

      // Handle variable references
      const varPattern = new RegExp(
        `(?:const|let|var)\\s+${argText}\\s*=\\s*{[^}]*sameSite\\s*:`,
        "i"
      );
      return varPattern.test(fileText);
    } catch (error) {
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: Error checking config reference:`,
          error.message
        );
      }
      return false;
    }
  }

  hasSameSiteInText(text) {
    // Check for sameSite with acceptable values
    const sameSitePatterns = [
      /sameSite\s*:\s*['"](strict|lax|none)['"]|sameSite\s*:\s*(strict|lax|none)/i,
      /sameSite\s*:\s*.*\?\s*['"](strict|lax|none)['"]\s*:\s*['"](strict|lax|none)['"]/i, // Ternary operator
      /sameSite\s*:\s*.*\?\s*(strict|lax|none)\s*:\s*(strict|lax|none)/i, // Ternary without quotes
    ];

    return sameSitePatterns.some((pattern) => pattern.test(text));
  }

  hasSameSiteAttribute(cookieValue) {
    // Check for SameSite in Set-Cookie header
    const sameSitePattern = /SameSite=(Strict|Lax|None)/i;
    return sameSitePattern.test(cookieValue);
  }

  isSessionCookie(cookieName) {
    const name = cookieName.toLowerCase();
    return this.sessionIndicators.some((indicator) =>
      name.includes(indicator.toLowerCase())
    );
  }

  isSessionCookieHeader(cookieValue) {
    // Extract cookie name from Set-Cookie header value
    const nameMatch = cookieValue.match(/^[^=]+/);
    if (!nameMatch) return false;

    const cookieName = nameMatch[0].replace(/['"]/g, "").trim();
    return this.isSessionCookie(cookieName);
  }

  extractCookieNameFromHeader(cookieValue) {
    const nameMatch = cookieValue.match(/^[^=]+/);
    return nameMatch ? nameMatch[0].replace(/['"]/g, "").trim() : "unknown";
  }

  createViolation(sourceFile, callNode, message) {
    try {
      const start = callNode.getStart();
      const lineAndChar = sourceFile.getLineAndColumnAtPos(start);

      return {
        rule: this.ruleId,
        source: sourceFile.getFilePath(),
        category: this.category,
        line: lineAndChar.line,
        column: lineAndChar.column,
        message: `Insecure session cookie: ${message}`,
        severity: "error",
      };
    } catch (error) {
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: Error creating violation:`,
          error.message
        );
      }
      return null;
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
   * Enhanced cookie name extraction with framework support
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
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: Error extracting cookie name:`,
          error.message
        );
      }
    }
    return null;
  }

  /**
   * Enhanced method name detection with framework support
   */
  getMorphMethodName(callNode) {
    try {
      const expression = callNode.getExpression();
      const ts = require("typescript");

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
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: Error getting method name:`,
          error.message
        );
      }
      return "";
    }
  }
}

module.exports = S033SymbolBasedAnalyzer;
