/**
 * Symbol-based analyzer for C013 - No Dead Code
 * Purpose: Detect commented out code, unused variables/functions, and unreachable code using AST
 */

const { SyntaxKind } = require('ts-morph');

class C013SymbolBasedAnalyzer {
  constructor(semanticEngine = null) {
    this.ruleId = 'C013';
    this.ruleName = 'No Dead Code (Symbol-Based)';
    this.semanticEngine = semanticEngine;
    this.verbose = false;
  }

  initialize(options = {}) {
    if (options.semanticEngine) {
      this.semanticEngine = options.semanticEngine;
    }
    this.verbose = options.verbose || false;
    
    if (this.verbose) {
      console.log(`[DEBUG] 🔧 C013 Symbol-Based: Analyzer initialized`);
    }
  }

  async analyze(files, language, options = {}) {
    const violations = [];
    
    if (process.env.SUNLINT_DEBUG) {
      console.log(`[C013 Symbol-Based] Starting analysis for ${files.length} files`);
      console.log(`[C013 Symbol-Based] Semantic engine available: ${!!this.semanticEngine}`);
      console.log(`[C013 Symbol-Based] Options semantic engine: ${!!options.semanticEngine}`);
    }
    
    // Use semantic engine from options if not already set
    if (!this.semanticEngine && options.semanticEngine) {
      this.semanticEngine = options.semanticEngine;
    }
    
    if (!this.semanticEngine?.project) {
      if (this.verbose || process.env.SUNLINT_DEBUG) {
        console.warn('[C013 Symbol-Based] No semantic engine available, skipping analysis');
      }
      return violations;
    }
    
    for (const filePath of files) {
      try {
        if (process.env.SUNLINT_DEBUG) {
          console.log(`[C013 Symbol-Based] Analyzing file: ${filePath}`);
        }
        
        const sourceFile = this.semanticEngine.project.getSourceFile(filePath);
        
        if (!sourceFile) {
          if (process.env.SUNLINT_DEBUG) {
            console.warn(`[C013 Symbol-Based] Could not load source file: ${filePath}`);
          }
          continue;
        }
        
        // 1. Check for commented out code
        const commentedCodeViolations = this.detectCommentedOutCode(sourceFile, filePath);
        violations.push(...commentedCodeViolations);
        
        // 2. Check for unused variables
        const unusedVariableViolations = this.detectUnusedVariables(sourceFile, filePath);
        violations.push(...unusedVariableViolations);
        
        // 3. Check for unused functions
        const unusedFunctionViolations = this.detectUnusedFunctions(sourceFile, filePath);
        violations.push(...unusedFunctionViolations);
        
        // 4. Check for unreachable code
        const unreachableCodeViolations = this.detectUnreachableCode(sourceFile, filePath);
        violations.push(...unreachableCodeViolations);
        
      } catch (error) {
        if (process.env.SUNLINT_DEBUG) {
          console.error(`[C013 Symbol-Based] Error analyzing ${filePath}:`, error);
        }
      }
    }
    
    return violations;
  }

  detectCommentedOutCode(sourceFile, filePath) {
    const violations = [];
    const text = sourceFile.getFullText();
    const lines = text.split('\n');
    
    const codePatterns = [
      /function\s+\w+/,
      /const\s+\w+\s*=/,
      /let\s+\w+\s*=/,
      /var\s+\w+\s*=/,
      /if\s*\(/,
      /for\s*\(/,
      /while\s*\(/,
      /return\s+/,
      /console\./,
      /import\s+/,
      /export\s+/,
      /class\s+\w+/,
      /interface\s+\w+/,
      /type\s+\w+\s*=/
    ];
    
    const processedLines = new Set(); // Track lines we've already processed
    
    for (let i = 0; i < lines.length; i++) {
      // Skip if this line was already processed as part of a block
      if (processedLines.has(i)) {
        continue;
      }
      
      const line = lines[i];
      const trimmedLine = line.trim();
      
      // Check single line comments and group consecutive ones
      if (trimmedLine.startsWith('//')) {
        const startLine = i;
        let endLine = i;
        let blockLines = [];
        
        // Collect consecutive comment lines with their content, including those separated by empty lines
        for (let j = i; j < lines.length; j++) {
          const currentLine = lines[j].trim();
          
          // Stop if we hit a non-comment, non-empty line
          if (!currentLine.startsWith('//') && currentLine !== '') {
            break;
          }
          
          // Skip empty lines but continue processing
          if (currentLine === '') {
            continue;
          }
          
          const content = currentLine.substring(2).trim();
          const isLikelyCode = content.length >= 10 && this.looksLikeCode(content, codePatterns) && !this.isDocumentationComment(content);
          const isShortCodeLine = content.length >= 3 && (
            /^[A-Z_]+:\s*['"][^'"]*['"],?$/.test(content) || // Object property like: JASPA: '0',
            /^[})\]];?$/.test(content) || // Closing braces/brackets
            /^[{(\[]$/.test(content) || // Opening braces/brackets  
            /^return\s+.*;?$/.test(content) || // return statements
            /^default:\s*$/.test(content) || // switch default
            /^case\s+.*:$/.test(content) // switch cases
          );
          
          blockLines.push({
            lineIndex: j,
            content: content,
            fullLine: lines[j],
            isCode: isLikelyCode,
            isShortCodeLine: isShortCodeLine
          });
          
          // Debug log for mappingWorkPartsRow.ts
          if (filePath.includes('mappingWorkPartsRow.ts') && process.env.SUNLINT_DEBUG) {
            console.log(`Line ${j+1}: "${content}" -> isCode: ${content.length >= 10 && this.looksLikeCode(content, codePatterns) && !this.isDocumentationComment(content)}`);
          }
          endLine = j;
          processedLines.add(j); // Mark as processed
        }
        
        // Find consecutive code sections within the block
        // For function-like blocks, group the entire block together
        const blockContent = blockLines.map(line => line.content).join('\n');
        const hasFunction = /\bfunction\s+\w+\s*\(/.test(blockContent) || 
                           /\bconst\s+\w+.*=>\s*\{/s.test(blockContent) ||
                           /\blet\s+\w+.*=>\s*\{/s.test(blockContent) ||
                           /\bvar\s+\w+.*=>\s*\{/s.test(blockContent) ||
                           /\bclass\s+\w+/.test(blockContent) ||
                           /\bit\s*\(\s*['"`].*['"`]\s*,\s*async\s*\(\s*\)\s*=>/s.test(blockContent) || // Jest it() async
                           /\bit\s*\(\s*['"`].*['"`]\s*,\s*\(\s*\)\s*=>/s.test(blockContent) || // Jest it() sync  
                           /\bdescribe\s*\(\s*['"`].*['"`]\s*,\s*\(\s*\)\s*=>/s.test(blockContent) || // Jest describe()
                           /\btest\s*\(\s*['"`].*['"`]\s*,\s*async\s*\(\s*\)\s*=>/s.test(blockContent) || // Jest test() async
                           /\btest\s*\(\s*['"`].*['"`]\s*,\s*\(\s*\)\s*=>/s.test(blockContent); // Jest test() sync
        
        const hasAnyCode = blockLines.some(line => line.isCode);
        
        if (filePath.includes('BillingList.test.tsx') && process.env.SUNLINT_DEBUG) {
          console.log(`[DEBUG] Block analysis: hasFunction=${hasFunction}, hasAnyCode=${hasAnyCode}, blockSize=${blockLines.length}`);
          console.log(`[DEBUG] Block content snippet: "${blockContent.substring(0, 100)}..."`);
          console.log(`[DEBUG] Jest patterns test:
            - it async: ${/\bit\s*\(\s*['"`].*['"`]\s*,\s*async\s*\(\s*\)\s*=>/s.test(blockContent)}
            - it sync: ${/\bit\s*\(\s*['"`].*['"`]\s*,\s*\(\s*\)\s*=>/s.test(blockContent)}
            - test async: ${/\btest\s*\(\s*['"`].*['"`]\s*,\s*async\s*\(\s*\)\s*=>/s.test(blockContent)}
            - describe: ${/\bdescribe\s*\(\s*['"`].*['"`]\s*,\s*\(\s*\)\s*=>/s.test(blockContent)}`);
        }
        
        if (hasFunction && hasAnyCode) {
          // For function blocks, group everything together
          const firstCodeLineIndex = blockLines.findIndex(line => line.isCode);
          const lastCodeLineIndex = blockLines.map((line, idx) => line.isCode ? idx : -1)
                                              .filter(idx => idx !== -1)
                                              .pop();
          
          if (firstCodeLineIndex !== -1 && lastCodeLineIndex !== -1) {
            // Use the very first line of the block instead of first code line for better accuracy
            const startLineIndex = blockLines[0].lineIndex; // Start from beginning of comment block
            const totalLines = lastCodeLineIndex - firstCodeLineIndex + 1;
            
            if (filePath.includes('BillingList.test.tsx') && process.env.SUNLINT_DEBUG) {
              console.log(`[DEBUG] Function block grouped: startLine=${startLineIndex + 1}, firstCodeLine=${blockLines[firstCodeLineIndex].lineIndex + 1}, totalLines=${totalLines}`);
            }
            
            violations.push(this.createViolation(
              filePath,
              startLineIndex + 1, // Use start of comment block, not first code line
              blockLines[0].fullLine.indexOf('//') + 1, // Column of first comment line
              `Commented out code block detected (${totalLines} lines). Remove dead code or use Git for version history.`,
              'commented-code'
            ));
          }
        } else {
          // Original logic for non-function blocks
          let currentCodeStart = -1;
          let currentCodeEnd = -1;
          let hasCodeInBlock = false;
          
          for (let k = 0; k < blockLines.length; k++) {
            const lineInfo = blockLines[k];
            
            if (lineInfo.isCode) {
              hasCodeInBlock = true;
            }
            
            // A line is considered part of code if it's actual code OR short code line in a code context
            const isPartOfCode = lineInfo.isCode || (lineInfo.isShortCodeLine && hasCodeInBlock);
            
            if (isPartOfCode) {
              if (currentCodeStart === -1) {
                // Start new code section
                currentCodeStart = k;
                currentCodeEnd = k;
              } else {
                // Extend current code section
                currentCodeEnd = k;
              }
            } else {
              // Non-code line - if we have a code section, report it
              if (currentCodeStart !== -1) {
                const codeStartLine = blockLines[currentCodeStart].lineIndex;
                const codeCount = currentCodeEnd - currentCodeStart + 1;
                
                const message = codeCount > 1 
                  ? `Commented out code block detected (${codeCount} lines). Remove dead code or use Git for version history.`
                  : `Commented out code detected. Remove dead code or use Git for version history.`;
                  
                violations.push(this.createViolation(
                  filePath,
                  codeStartLine + 1, // Convert to 1-based line number
                  blockLines[currentCodeStart].fullLine.indexOf('//') + 1,
                  message,
                  'commented-code'
                ));
                
                // Reset for next code section
                currentCodeStart = -1;
                currentCodeEnd = -1;
                hasCodeInBlock = false;
              }
            }
          }
          
          // Report any remaining code section
          if (currentCodeStart !== -1) {
            const codeStartLine = blockLines[currentCodeStart].lineIndex;
            const codeCount = currentCodeEnd - currentCodeStart + 1;
            
            const message = codeCount > 1 
              ? `Commented out code block detected (${codeCount} lines). Remove dead code or use Git for version history.`
              : `Commented out code detected. Remove dead code or use Git for version history.`;
              
            violations.push(this.createViolation(
              filePath,
              codeStartLine + 1, // Convert to 1-based line number
              blockLines[currentCodeStart].fullLine.indexOf('//') + 1,
              message,
              'commented-code'
            ));
          }
        }
        
        // Don't forget the last code section from the original logic (this should already be handled above)
        // This section can be removed as it's redundant
      }
      
      // Check multi-line comments (but skip JSDoc)
      if (trimmedLine.startsWith('/*') && !trimmedLine.startsWith('/**') && !trimmedLine.includes('*/')) {
        let commentBlock = '';
        let endLine = i;
        
        // Collect the full comment block
        for (let j = i; j < lines.length; j++) {
          commentBlock += lines[j] + '\n';
          processedLines.add(j); // Mark as processed
          if (lines[j].includes('*/')) {
            endLine = j;
            break;
          }
        }
        
        // Clean the comment block
        const cleanedComment = commentBlock
          .replace(/\/\*|\*\/|\*/g, '')
          .trim();
          
        // Skip if it's documentation
        if (this.isDocumentationComment(cleanedComment)) {
          continue;
        }
          
        if (cleanedComment.length >= 20 && this.looksLikeCode(cleanedComment, codePatterns)) {
          violations.push(this.createViolation(
            filePath,
            i + 1,
            line.indexOf('/*') + 1,
            `Commented out code block detected. Remove dead code or use Git for version history.`,
            'commented-code-block'
          ));
        }
      }
    }
    
    return violations;
  }
  
  isDocumentationComment(text) {
    // Check for JSDoc tags and documentation patterns
    const docPatterns = [
      /@param\b/,
      /@returns?\b/,
      /@example\b/,
      /@description\b/,
      /@see\b/,
      /@throws?\b/,
      /@since\b/,
      /@author\b/,
      /@version\b/,
      /\* Sort an array/,
      /\* Items are sorted/,
      /\* @/,
      /Result:/,
      /Note:/
    ];
    
    // If it contains documentation patterns, it's likely documentation
    if (docPatterns.some(pattern => pattern.test(text))) {
      return true;
    }
    
    // Check for common explanatory phrases
    const explanatoryPhrases = [
      'explanation',
      'description',
      'example',
      'usage',
      'note that',
      'this function',
      'this method',
      'basic usage',
      'with duplicate',
      'items not found'
    ];
    
    const lowerText = text.toLowerCase();
    return explanatoryPhrases.some(phrase => lowerText.includes(phrase));
  }

  looksLikeCode(text, patterns) {
    // Check if text matches code patterns
    const matchCount = patterns.filter(pattern => pattern.test(text)).length;
    
    // If it matches multiple patterns or contains typical code structure
    if (matchCount >= 1) {
      // Additional checks for code-like characteristics
      const hasCodeStructure = (
        text.includes('{') ||
        text.includes(';') ||
        text.includes('()') ||
        text.includes('[]') ||
        /\w+\s*=\s*\w+/.test(text) ||
        /\w+\.\w+/.test(text)
      );
      
      return hasCodeStructure;
    }
    
    return false;
  }

  detectUnusedVariables(sourceFile, filePath) {
    const violations = [];
    
    // Get all variable declarations
    const variableDeclarations = sourceFile.getDescendantsOfKind(SyntaxKind.VariableDeclaration);
    
    for (const declaration of variableDeclarations) {
      const name = declaration.getName();
      
      // Skip variables with underscore prefix (conventional ignore)
      if (name.startsWith('_') || name.startsWith('$')) {
        continue;
      }
      
      // Skip destructured variables for now (complex analysis)
      if (declaration.getNameNode().getKind() !== SyntaxKind.Identifier) {
        continue;
      }
      
      // Skip exported variables (they might be used externally)
      const variableStatement = declaration.getParent()?.getParent();
      if (variableStatement && variableStatement.getKind() === SyntaxKind.VariableStatement) {
        if (variableStatement.isExported()) {
          continue;
        }
      }
      
      // Check if variable is used
      const usages = declaration.getNameNode().findReferences();
      const isUsed = usages.some(ref => 
        ref.getReferences().length > 1 // More than just the declaration itself
      );
      
      if (!isUsed) {
        const line = sourceFile.getLineAndColumnAtPos(declaration.getStart()).line;
        const column = sourceFile.getLineAndColumnAtPos(declaration.getStart()).column;
        
        violations.push(this.createViolation(
          filePath,
          line,
          column,
          `Unused variable '${name}'. Remove dead code to keep codebase clean.`,
          'unused-variable'
        ));
      }
    }
    
    return violations;
  }

  detectUnusedFunctions(sourceFile, filePath) {
    const violations = [];
    
    // Get all function declarations
    const functionDeclarations = sourceFile.getDescendantsOfKind(SyntaxKind.FunctionDeclaration);
    
    for (const func of functionDeclarations) {
      const name = func.getName();
      
      if (!name) continue; // Anonymous functions
      
      // Skip functions with underscore prefix
      if (name.startsWith('_')) {
        continue;
      }
      
      // Skip exported functions (they might be used externally)
      if (func.isExported()) {
        continue;
      }
      
      // Check if function is used
      const nameNode = func.getNameNode();
      if (nameNode) {
        const usages = nameNode.findReferences();
        const isUsed = usages.some(ref => 
          ref.getReferences().length > 1 // More than just the declaration itself
        );
        
        if (!isUsed) {
          const line = sourceFile.getLineAndColumnAtPos(func.getStart()).line;
          const column = sourceFile.getLineAndColumnAtPos(func.getStart()).column;
          
          violations.push(this.createViolation(
            filePath,
            line,
            column,
            `Unused function '${name}'. Remove dead code to keep codebase clean.`,
            'unused-function'
          ));
        }
      }
    }
    
    return violations;
  }

  detectUnreachableCode(sourceFile, filePath) {
    const violations = [];
    
    // Find all return, throw, break, continue statements
    const terminatingStatements = [
      ...sourceFile.getDescendantsOfKind(SyntaxKind.ReturnStatement),
      ...sourceFile.getDescendantsOfKind(SyntaxKind.ThrowStatement),
      ...sourceFile.getDescendantsOfKind(SyntaxKind.BreakStatement),
      ...sourceFile.getDescendantsOfKind(SyntaxKind.ContinueStatement)
    ];
    
    for (const statement of terminatingStatements) {
      // Find the statement that contains this terminating statement
      let containingStatement = statement;
      let parent = statement.getParent();
      
      // Walk up to find the statement that's directly in a block
      while (parent && parent.getKind() !== SyntaxKind.Block && parent.getKind() !== SyntaxKind.SourceFile) {
        containingStatement = parent;
        parent = parent.getParent();
      }
      
      // Get the parent block
      const parentBlock = parent;
      
      if (!parentBlock || parentBlock.getKind() === SyntaxKind.SourceFile) continue;
      
      // Find all statements in the same block after this terminating statement
      const allStatements = parentBlock.getStatements();
      const currentIndex = allStatements.indexOf(containingStatement);
      
      if (currentIndex >= 0 && currentIndex < allStatements.length - 1) {
        // Check statements after the terminating statement
        for (let i = currentIndex + 1; i < allStatements.length; i++) {
          const nextStatement = allStatements[i];
          
          // Skip comments and empty statements
          if (nextStatement.getKind() === SyntaxKind.EmptyStatement) {
            continue;
          }
          
          // Don't flag catch/finally blocks as unreachable
          if (this.isInTryCatchFinally(nextStatement)) {
            continue;
          }
          
          // Skip if this is within a conditional (if/else) or loop that might not execute
          if (this.isConditionallyReachable(containingStatement, nextStatement)) {
            continue;
          }
          
          const line = sourceFile.getLineAndColumnAtPos(nextStatement.getStart()).line;
          const column = sourceFile.getLineAndColumnAtPos(nextStatement.getStart()).column;
          
          violations.push(this.createViolation(
            filePath,
            line,
            column,
            `Unreachable code detected after ${statement.getKindName().toLowerCase()}. Remove dead code.`,
            'unreachable-code'
          ));
          
          break; // Only flag the first unreachable statement to avoid spam
        }
      }
    }
    
    return violations;
  }

  isInTryCatchFinally(node) {
    // Check if the node is inside a try-catch-finally block
    let parent = node.getParent();
    while (parent) {
      if (parent.getKind() === SyntaxKind.TryStatement) {
        return true;
      }
      if (parent.getKind() === SyntaxKind.CatchClause) {
        return true;
      }
      parent = parent.getParent();
    }
    return false;
  }
  
  isConditionallyReachable(terminatingStatement, nextStatement) {
    // Check if the terminating statement is within an arrow function expression
    // or other expression that shouldn't be considered as blocking execution
    let current = terminatingStatement;
    
    while (current) {
      const kind = current.getKind();
      
      // If the terminating statement is in an arrow function or function expression,
      // it doesn't block the execution of subsequent statements in the parent scope
      if (kind === SyntaxKind.ArrowFunction || 
          kind === SyntaxKind.FunctionExpression ||
          kind === SyntaxKind.ConditionalExpression) {
        return true;
      }
      
      // If we're in an if statement, the return might be conditional
      if (kind === SyntaxKind.IfStatement) {
        // Check if this is a complete if-else that covers all paths
        const ifStatement = current;
        const elseStatement = ifStatement.getElseStatement();
        
        // If there's no else, or else doesn't have a return, then subsequent code is reachable
        if (!elseStatement || !this.hasUnconditionalReturn(elseStatement)) {
          return true;
        }
      }
      
      current = current.getParent();
    }
    
    return false;
  }
  
  hasUnconditionalReturn(node) {
    // Check if a node has an unconditional return statement
    if (node.getKind() === SyntaxKind.ReturnStatement) {
      return true;
    }
    
    if (node.getKind() === SyntaxKind.Block) {
      const statements = node.getStatements();
      return statements.some(stmt => this.hasUnconditionalReturn(stmt));
    }
    
    return false;
  }

  createViolation(filePath, line, column, message, type) {
    return {
      file: filePath,
      line: line,
      column: column,
      message: message,
      severity: 'warning',
      ruleId: this.ruleId,
      type: type
    };
  }
}

module.exports = C013SymbolBasedAnalyzer;
