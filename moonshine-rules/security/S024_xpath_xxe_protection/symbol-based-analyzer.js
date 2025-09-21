/**
 * S024 Symbol-Based Analyzer - Protect against XPath Injection and XML External Entity (XXE)
 * Uses TypeScript compiler API for semantic analysis
 */

const ts = require("typescript");

class S024SymbolBasedAnalyzer {
  constructor(semanticEngine = null) {
    this.semanticEngine = semanticEngine;
    this.ruleId = "S024";
    this.category = "security";

    // XPath-related method names that can be vulnerable
    this.xpathMethods = [
      "evaluate",
      "select", 
      "selectText",
      "selectValue",
      "selectNodes",
      "selectSingleNode"
    ];

    // XML parsing methods that can have XXE vulnerabilities
    this.xmlParsingMethods = [
      "parseString",
      "parseXml", 
      "parseFromString",
      "parse",
      "parseXmlString",
      "transform"
    ];

    // XML parser constructors that need XXE protection
    this.xmlParserConstructors = [
      "DOMParser",
      "XSLTProcessor", 
      "SAXParser"
    ];

    // User input sources that could lead to injection
    this.userInputSources = [
      "req",
      "request", 
      "params",
      "query",
      "body",
      "headers",
      "cookies"
    ];

    // Secure XPath/XML patterns
    this.securePatterns = [
      "parameterized",
      "escaped", 
      "sanitized",
      "validate"
    ];
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
        console.log(`🔍 [${this.ruleId}] Symbol: Starting symbol-based analysis`);
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
          // Analyze XPath injection vulnerabilities
          const xpathViolation = this.analyzeXPathCall(callNode, sourceFile);
          if (xpathViolation) {
            violations.push(xpathViolation);
          }

          // Analyze XXE vulnerabilities  
          const xxeViolation = this.analyzeXXEVulnerability(callNode, sourceFile);
          if (xxeViolation) {
            violations.push(xxeViolation);
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

      // Also check for new expressions (constructors)
      const newExpressions = sourceFile.getDescendantsOfKind
        ? sourceFile.getDescendantsOfKind(
            require("typescript").SyntaxKind.NewExpression
          )
        : [];

      for (const newNode of newExpressions) {
        try {
          const xxeViolation = this.analyzeXMLParserConstructor(newNode, sourceFile);
          if (xxeViolation) {
            violations.push(xxeViolation);
          }
        } catch (error) {
          if (this.verbose) {
            console.log(
              `🔍 [${this.ruleId}] Symbol: Error analyzing new expression:`,
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

  analyzeXPathCall(callNode, sourceFile) {
    try {
      const expression = callNode.getExpression();
      const methodName = this.getMethodName(expression);

      if (!this.xpathMethods.includes(methodName)) {
        return null;
      }

      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: XPath method call detected: ${methodName}`
        );
      }

      // Check if user input is used in XPath arguments
      const args = callNode.getArguments();
      if (args.length === 0) {
        return null;
      }

      const firstArg = args[0];
      if (this.containsUserInput(firstArg)) {
        return this.createViolation(
          sourceFile,
          callNode,
          `XPath Injection vulnerability: User input used directly in ${methodName}() without proper sanitization`
        );
      }

      // Check for string concatenation with user input
      if (this.containsStringConcatenationWithUserInput(firstArg)) {
        return this.createViolation(
          sourceFile,
          callNode,
          `XPath Injection vulnerability: XPath query constructed using string concatenation with user input`
        );
      }

      return null;
    } catch (error) {
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: Error analyzing XPath call:`,
          error.message
        );
      }
      return null;
    }
  }

  analyzeXXEVulnerability(callNode, sourceFile) {
    try {
      const expression = callNode.getExpression();
      const methodName = this.getMethodName(expression);

      if (!this.xmlParsingMethods.includes(methodName)) {
        return null;
      }

      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: XML parsing method detected: ${methodName}`
        );
      }

      // Check if XXE protection is implemented
      const hasProtection = this.hasXXEProtectionInContext(callNode, sourceFile);
      if (!hasProtection) {
        return this.createViolation(
          sourceFile,
          callNode,
          `XXE vulnerability: ${methodName}() used without disabling external entity processing`
        );
      }

      return null;
    } catch (error) {
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: Error analyzing XXE vulnerability:`,
          error.message
        );
      }
      return null;
    }
  }

  analyzeXMLParserConstructor(newNode, sourceFile) {
    try {
      const expression = newNode.getExpression();
      const constructorName = expression.getText();

      const isXMLParser = this.xmlParserConstructors.some(parser =>
        constructorName.includes(parser)
      );

      if (!isXMLParser) {
        return null;
      }

      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: XML parser constructor detected: ${constructorName}`
        );
      }

      // Check if XXE protection is configured
      const hasProtection = this.hasXXEProtectionInContext(newNode, sourceFile);
      if (!hasProtection) {
        return this.createViolation(
          sourceFile,
          newNode,
          `XXE vulnerability: ${constructorName} instantiated without XXE protection`
        );
      }

      return null;
    } catch (error) {
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Symbol: Error analyzing XML parser constructor:`,
          error.message
        );
      }
      return null;
    }
  }

  containsUserInput(node) {
    try {
      const nodeText = node.getText();
      return this.userInputSources.some(source => 
        nodeText.includes(`${source}.`) || nodeText.includes(`${source}[`)
      );
    } catch (error) {
      return false;
    }
  }

  containsStringConcatenationWithUserInput(node) {
    try {
      const nodeText = node.getText();
      
      // Check for binary expressions with +
      if (nodeText.includes('+')) {
        return this.userInputSources.some(source =>
          nodeText.includes(`${source}.`) || nodeText.includes(`${source}[`)
        );
      }

      // Check for template literals with user input
      if (nodeText.includes('`') && nodeText.includes('${')) {
        return this.userInputSources.some(source =>
          nodeText.includes(`\${${source}.`) || nodeText.includes(`\${${source}[`)
        );
      }

      return false;
    } catch (error) {
      return false;
    }
  }

  hasXXEProtectionInContext(node, sourceFile) {
    try {
      // Get the parent scope (function/method) to check for XXE protection
      let parent = node.getParent();
      while (parent && !this.isFunctionLike(parent)) {
        parent = parent.getParent();
      }

      if (!parent) {
        return false;
      }

      const functionText = parent.getText();
      
      // Check for XXE protection patterns
      const protectionPatterns = [
        /resolveExternalEntities\s*:\s*false/,
        /setFeature.*disallow-doctype-decl.*true/,
        /setFeature.*external-general-entities.*false/,
        /setFeature.*external-parameter-entities.*false/,
        /explicitChildren\s*:\s*false/,
        /ignoreAttrs\s*:\s*true/,
        /parseDoctype\s*:\s*false/
      ];

      return protectionPatterns.some(pattern => pattern.test(functionText));
    } catch (error) {
      return false;
    }
  }

  isFunctionLike(node) {
    try {
      const SyntaxKind = require("typescript").SyntaxKind;
      const kind = node.getKind();
      
      return kind === SyntaxKind.FunctionDeclaration ||
             kind === SyntaxKind.FunctionExpression ||
             kind === SyntaxKind.ArrowFunction ||
             kind === SyntaxKind.MethodDeclaration;
    } catch (error) {
      return false;
    }
  }

  getMethodName(expression) {
    try {
      const ts = require("typescript");

      if (expression.getKind() === ts.SyntaxKind.PropertyAccessExpression) {
        return expression.getNameNode().getText();
      }

      if (expression.getKind() === ts.SyntaxKind.Identifier) {
        return expression.getText();
      }

      return "";
    } catch (error) {
      return "";
    }
  }

  createViolation(sourceFile, node, message) {
    try {
      const start = node.getStart();
      const lineAndChar = sourceFile.getLineAndColumnAtPos(start);

      return {
        rule: this.ruleId,
        source: sourceFile.getFilePath(),
        category: this.category,
        line: lineAndChar.line,
        column: lineAndChar.column,
        message: message,
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
}

module.exports = S024SymbolBasedAnalyzer;
