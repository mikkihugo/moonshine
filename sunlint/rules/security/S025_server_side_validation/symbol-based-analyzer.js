/**
 * S025 Symbol-Based Analyzer - Always validate client-side data on the server
 * Uses TypeScript compiler API for semantic analysis
 * 
 * Detects patterns where client data is used without server-side validation:
 * 1. Using @Body() without ValidationPipe or DTO validation
 * 2. Trusting sensitive fields from client (userId, role, price, isAdmin)
 * 3. Direct use of req.body, req.query, req.params without validation
 * 4. Missing ValidationPipe configuration
 * 5. SQL injection via string concatenation
 * 6. File upload without server-side validation
 */

const ts = require("typescript");

class S025SymbolBasedAnalyzer {
  constructor(semanticEngine = null) {
    this.semanticEngine = semanticEngine;
    this.ruleId = "S025";
    this.category = "security";

    // Sensitive field patterns that should not come from client
    this.sensitiveFields = [
      "userId", "user_id", "id",
      "role", "roles", "permissions",
      "price", "amount", "total", "cost",
      "isAdmin", "is_admin", "admin",
      "discount", "balance", "credits",
      "isActive", "is_active", "enabled",
      "status", "state"
    ];

    // Client data sources that need validation
    this.clientDataSources = [
      "req.body", "request.body",
      "req.query", "request.query", 
      "req.params", "request.params",
      "ctx.request.body", "ctx.query", "ctx.params",
      "@Body()", "@Query()", "@Param()"
    ];

    // Validation indicators
    this.validationIndicators = [
      "ValidationPipe", "validate", "validator",
      "class-validator", "joi", "yup", "zod",
      "IsString", "IsInt", "IsEmail", "IsUUID",
      "validateOrReject", "plainToClass"
    ];

    // SQL query patterns
    this.sqlQueryPatterns = [
      "query", "exec", "execute", "find", "findOne",
      "createQueryBuilder", "getRepository"
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
      const fileContent = sourceFile.getFullText();

      // Check for NestJS specific patterns
      const isNestJSFile = this.isNestJSFile(fileContent);
      const isExpressFile = this.isExpressFile(fileContent);

      if (this.verbose) {
        console.log(`🔍 [${this.ruleId}] Symbol: Framework detection - NestJS: ${isNestJSFile}, Express: ${isExpressFile}`);
      }

      // 1. Check for missing ValidationPipe in NestJS
      if (isNestJSFile) {
        violations.push(...this.checkValidationPipeUsage(sourceFile));
      }

      // 2. Check for unsafe @Body() usage without DTO
      violations.push(...this.checkUnsafeBodyUsage(sourceFile));

      // 3. Check for sensitive field trusting
      violations.push(...this.checkSensitiveFieldTrusting(sourceFile));

      // 4. Check for SQL injection patterns
      violations.push(...this.checkSQLInjectionPatterns(sourceFile));

      // 5. Check for file upload validation
      violations.push(...this.checkFileUploadValidation(sourceFile));

      // 6. Check for Express req usage without validation
      if (isExpressFile) {
        violations.push(...this.checkExpressReqUsage(sourceFile));
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

  isNestJSFile(content) {
    return content.includes("@nestjs/") || 
           content.includes("@Controller") ||
           content.includes("@Post") ||
           content.includes("@Get") ||
           content.includes("@Body()");
  }

  isExpressFile(content) {
    return content.includes("express") ||
           content.includes("req.body") ||
           content.includes("req.query") ||
           content.includes("res.") ||
           content.includes("app.post") ||
           content.includes("app.get");
  }

  checkValidationPipeUsage(sourceFile) {
    const violations = [];
    const content = sourceFile.getFullText();

    // Check if ValidationPipe is configured globally
    const hasGlobalValidationPipe = content.includes("useGlobalPipes") && 
                                   content.includes("ValidationPipe");

    // If no global ValidationPipe, check individual routes
    if (!hasGlobalValidationPipe) {
      const decorators = sourceFile.getDescendantsOfKind?.(
        require("typescript").SyntaxKind.Decorator
      ) || [];

      for (const decorator of decorators) {
        try {
          const decoratorText = decorator.getText();
          if (decoratorText.includes("@Post") || 
              decoratorText.includes("@Put") || 
              decoratorText.includes("@Patch")) {
            
            // Find the method this decorator is attached to
            const method = decorator.getParent();
            if (method) {
              const methodText = method.getText();
              
              // Check if method uses @Body() without proper validation
              if (methodText.includes("@Body()") && 
                  !this.hasValidationInMethod(methodText)) {
                
                const lineNumber = sourceFile.getLineAndColumnAtPos(decorator.getStart()).line;
                violations.push(this.createViolation(
                  sourceFile,
                  decorator,
                  `Route missing ValidationPipe or DTO validation`
                ));
              }
            }
          }
        } catch (error) {
          if (this.verbose) {
            console.log(`🔍 [${this.ruleId}] Symbol: Error checking decorator:`, error.message);
          }
        }
      }
    }

    return violations;
  }

  hasValidationInMethod(methodText) {
    return this.validationIndicators.some(indicator => 
      methodText.includes(indicator)
    );
  }

  checkUnsafeBodyUsage(sourceFile) {
    const violations = [];

    try {
      const content = sourceFile.getFullText();
      
      // Look for @Body() any or @Body() without DTO
      const bodyUsagePatterns = [
        /@Body\(\)\s+\w+:\s*any/g,
        /@Body\(\)\s+\w+:\s*Record<string,\s*any>/g,
        /@Body\(\)\s+\{[^}]*\}:\s*any/g
      ];

      for (const pattern of bodyUsagePatterns) {
        let match;
        while ((match = pattern.exec(content)) !== null) {
          const lineNumber = this.getLineNumber(content, match.index);
          violations.push({
            rule: this.ruleId,
            source: sourceFile.getFilePath(),
            category: this.category,
            line: lineNumber,
            column: 1,
            message: `Unsafe @Body() usage without proper DTO validation`,
            severity: "error",
          });
        }
      }

    } catch (error) {
      if (this.verbose) {
        console.log(`🔍 [${this.ruleId}] Symbol: Error checking body usage:`, error.message);
      }
    }

    return violations;
  }

  checkSensitiveFieldTrusting(sourceFile) {
    const violations = [];

    try {
      const content = sourceFile.getFullText();

      // Check for destructuring sensitive fields from client data
      for (const sensitiveField of this.sensitiveFields) {
        const patterns = [
          new RegExp(`\\{[^}]*${sensitiveField}[^}]*\\}\\s*=\\s*req\\.body`, 'g'),
          new RegExp(`\\{[^}]*${sensitiveField}[^}]*\\}\\s*=\\s*@Body\\(\\)`, 'g'),
          new RegExp(`const\\s+${sensitiveField}\\s*=\\s*req\\.body\\.${sensitiveField}`, 'g'),
          new RegExp(`let\\s+${sensitiveField}\\s*=\\s*req\\.body\\.${sensitiveField}`, 'g')
        ];

        for (const pattern of patterns) {
          let match;
          while ((match = pattern.exec(content)) !== null) {
            const lineNumber = this.getLineNumber(content, match.index);
            violations.push({
              rule: this.ruleId,
              source: sourceFile.getFilePath(),
              category: this.category,
              line: lineNumber,
              column: 1,
              message: `Sensitive field "${sensitiveField}" should not be trusted from client data`,
              severity: "error",
            });
          }
        }
      }

    } catch (error) {
      if (this.verbose) {
        console.log(`🔍 [${this.ruleId}] Symbol: Error checking sensitive fields:`, error.message);
      }
    }

    return violations;
  }

  checkSQLInjectionPatterns(sourceFile) {
    const violations = [];

    try {
      const content = sourceFile.getFullText();

      // Check for string concatenation in SQL queries
      const sqlInjectionPatterns = [
        /\.query\s*\(\s*[`"'][^`"']*\$\{[^}]+\}[^`"']*[`"']/g,
        /\.query\s*\(\s*[`"'][^`"']*\+[^`"']*[`"']/g,
        /SELECT\s+[^"'`]*\$\{[^}]+\}/gi,
        /WHERE\s+[^"'`]*\$\{[^}]+\}/gi,
        /ORDER\s+BY\s+[^"'`]*\$\{[^}]+\}/gi
      ];

      for (const pattern of sqlInjectionPatterns) {
        let match;
        while ((match = pattern.exec(content)) !== null) {
          const lineNumber = this.getLineNumber(content, match.index);
          violations.push({
            rule: this.ruleId,
            source: sourceFile.getFilePath(),
            category: this.category,
            line: lineNumber,
            column: 1,
            message: `Potential SQL injection: use parameterized queries instead of string concatenation`,
            severity: "error",
          });
        }
      }

    } catch (error) {
      if (this.verbose) {
        console.log(`🔍 [${this.ruleId}] Symbol: Error checking SQL patterns:`, error.message);
      }
    }

    return violations;
  }

  checkFileUploadValidation(sourceFile) {
    const violations = [];

    try {
      const content = sourceFile.getFullText();

      // Check for file upload without validation
      if (content.includes("@UseInterceptors(FileInterceptor") ||
          content.includes("@UploadedFile()")) {
        
        // Check if there's proper file validation
        const hasFileValidation = content.includes("fileFilter") ||
                                 content.includes("file.mimetype") ||
                                 content.includes("file.size") ||
                                 content.includes("multer");

        if (!hasFileValidation) {
          const uploadMatch = content.match(/@UploadedFile\(\)/);
          if (uploadMatch) {
            const lineNumber = this.getLineNumber(content, uploadMatch.index);
            violations.push({
              rule: this.ruleId,
              source: sourceFile.getFilePath(),
              category: this.category,
              line: lineNumber,
              column: 1,
              message: `File upload missing server-side validation (type, size, content)`,
              severity: "error",
            });
          }
        }
      }

    } catch (error) {
      if (this.verbose) {
        console.log(`🔍 [${this.ruleId}] Symbol: Error checking file upload:`, error.message);
      }
    }

    return violations;
  }

  checkExpressReqUsage(sourceFile) {
    const violations = [];

    try {
      const content = sourceFile.getFullText();

      // Check for direct req.body usage without validation
      const patterns = [
        /req\.body\.\w+/g,
        /req\.query\.\w+/g,
        /req\.params\.\w+/g
      ];

      for (const pattern of patterns) {
        let match;
        while ((match = pattern.exec(content)) !== null) {
          // Check if validation is present in the same function
          const functionStart = this.findFunctionStart(content, match.index);
          const functionEnd = this.findFunctionEnd(content, match.index);
          const functionBody = content.substring(functionStart, functionEnd);

          const hasValidation = this.validationIndicators.some(indicator =>
            functionBody.includes(indicator)
          );

          if (!hasValidation) {
            const lineNumber = this.getLineNumber(content, match.index);
            violations.push({
              rule: this.ruleId,
              source: sourceFile.getFilePath(),
              category: this.category,
              line: lineNumber,
              column: 1,
              message: `Direct use of ${match[0]} without server-side validation`,
              severity: "error",
            });
          }
        }
      }

    } catch (error) {
      if (this.verbose) {
        console.log(`🔍 [${this.ruleId}] Symbol: Error checking Express req usage:`, error.message);
      }
    }

    return violations;
  }

  findFunctionStart(content, index) {
    // Simple heuristic: find the previous function declaration
    const beforeIndex = content.lastIndexOf("function", index);
    const beforeArrow = content.lastIndexOf("=>", index);
    const beforeAsync = content.lastIndexOf("async", index);
    
    return Math.max(beforeIndex, beforeArrow, beforeAsync, 0);
  }

  findFunctionEnd(content, index) {
    // Simple heuristic: find the next function or end of file
    const afterFunction = content.indexOf("function", index + 1);
    const afterArrow = content.indexOf("=>", index + 1);
    
    if (afterFunction === -1 && afterArrow === -1) {
      return content.length;
    }
    
    if (afterFunction === -1) return afterArrow;
    if (afterArrow === -1) return afterFunction;
    
    return Math.min(afterFunction, afterArrow);
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
        message: `Server-side validation missing: ${message}`,
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

  getLineNumber(content, index) {
    return content.substring(0, index).split("\n").length;
  }
}

module.exports = S025SymbolBasedAnalyzer;
