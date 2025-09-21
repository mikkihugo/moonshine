/**
 * C003_no_vague_abbreviations - Enhanced Regex-based Rule Analyzer
 * Category: coding
 * 
 * Detects vague variable names and unclear abbreviations
 * Uses enhanced regex patterns with proper comment filtering
 */

const fs = require('fs');
const path = require('path');
const { CommentDetector } = require('../../utils/rule-helpers');

class C003NoVagueAbbreviations {
  constructor(options = {}) {
    this.options = {
      allowedSingleChar: new Set(options.allowedSingleChar || [
        'i', 'j', 'k', 'x', 'y', 'z', 'n', 'm', 't', 'v', 'r', 'e', 'p', 'w', 'h'
      ]),
      allowedAbbreviations: new Set(options.allowedAbbreviations || [
        // Technical abbreviations from user feedback
        'id', 'url', 'uri', 'api', 'ui', 'db', 'fs', 'os', 'io', 'ai', 'ml', 'qa', 'ci', 'cd', 'pr',
        'jwt', 'uuid', 'json', 'xml', 'html', 'css', 'sql', 'http', 'https', 'ftp', 'smtp', 'tcp', 'udp',
        'pdf', 'csv', 'tsv', 'png', 'jpg', 'gif', 'svg', 'mp4', 'mp3', 'zip', 'tar',
        'js', 'ts', 'py', 'rb', 'go', 'rs', 'kt', 'cs', 'vb', 'sh',
        'dom', 'xhr', 'spa', 'pwa', 'seo', 'cdn', 'ssl', 'tls',
        'orm', 'ddl', 'dml', 'etl', 'olap', 'oltp',
        'kpi', 'roi', 'sla', 'poc', 'mvp', 'b2b', 'b2c', 'crm', 'erp',
        'jsx', 'tsx', 'vue', 'scss', 'less',
        'it', 'ut', 'e2e',
        // Common development terms
        'config', 'env', 'app', 'btn', 'img', 'src', 'dest', 'req', 'res', 'ctx',
        'min', 'max', 'len', 'num', 'str', 'auth', 'log', 'err', 'msg', 'key',
        // Add the variants from user feedback cases
        'qa1', 'ci1', 'tsx2', 'it2', 'qa2', 'ci2',
        // Common test/function context terms
        'value', 'result', 'response', 'request', 'data', 'item', 'element', 'object',
        // Common programming terms
        'async', 'length', 'ms'
      ]),
      minLength: options.minLength || 2,
      strictMode: options.strictMode || false
    };

    // Patterns for suspicious abbreviations
    this.suspiciousPatterns = [
      /^[a-z]{1,2}[0-9]*$/, // e.g., 'u', 'usr', 'n1', 'v2' 
      /^[a-z]*[aeiou]*[bcdfghjklmnpqrstvwxyz]{4,}$/, // too many consonants
      /^(tmp|temp|val|var|data|info|item|elem|el|obj|arr)([A-Z0-9].*)?$/, // generic names
    ];

    // Unclear names that should be avoided in most contexts (but not in specific contexts)
    this.unclearNames = new Set([
      'stuff', 'thing', 'something', 'anything', 'everything',
      'flag', 'check', 'test', 'validate', 'process', 'handle',
      'obj', 'arg', 'val', 'fn'
      // Removed: 'data', 'info', 'element', 'object', 'value', 'result', 'response', 'request', 'temp', 'tmp', 'variable'
      // These are often acceptable in specific contexts
    ]);

    // Context patterns where single letters are acceptable
    this.loopPatterns = [
      /for\s*\(\s*(?:let|const|var)\s+([a-z])\s*[=;]/i,
      /\.forEach\s*\(\s*(?:\([^)]*\)|\w+)\s*=>/,
      /\.map\s*\(\s*(?:\([^)]*\)|\w+)\s*=>/,
      /\.filter\s*\(\s*(?:\([^)]*\)|\w+)\s*=>/
    ];
  }

  /**
   * Check if variable is in loop context
   */
  isLoopContext(content, variableName, line) {
    const lines = content.split('\n');
    const currentLine = lines[line - 1] || '';
    const prevLine = lines[line - 2] || '';
    const nextLine = lines[line] || '';
    
    const contextLines = [prevLine, currentLine, nextLine].join(' ');
    
    // Check for for loops
    if (/for\s*\(\s*(?:let|const|var)\s+\w+/i.test(contextLines)) {
      return true;
    }
    
    // Check for array methods
    if (/\.(forEach|map|filter|reduce|some|every|find|findIndex)\s*\(/i.test(contextLines)) {
      return true;
    }
    
    return false;
  }

  /**
   * Check if variable is a generic type parameter
   */
  isGenericTypeParameter(content, variableName, line) {
    // Single uppercase letters T, U, V, K, etc. are common generic type names
    if (!/^[T-Z]$/i.test(variableName)) {
      return false;
    }
    
    const lines = content.split('\n');
    const currentLine = lines[line - 1] || '';
    const prevLine = lines[line - 2] || '';
    const nextLine = lines[line] || '';
    
    const contextLines = [prevLine, currentLine, nextLine].join(' ');
    
    // Check for generic type context
    const genericPatterns = [
      /function\s*<[^>]*>/i,           // function<T, U>
      /class\s+\w+\s*<[^>]*>/i,       // class Foo<T>
      /interface\s+\w+\s*<[^>]*>/i,   // interface Bar<T>
      /type\s+\w+\s*<[^>]*>/i,        // type Baz<T>
      /<[^>]*>.*=>/i,                 // <T>(param: T) =>
      /:\s*[T-Z]\s*[,)]/i             // parameter: T,
    ];
    
    return genericPatterns.some(pattern => pattern.test(contextLines));
  }

  /**
   * Check if variable is a TypeScript type annotation context
   */
  isTypeAnnotationContext(content, variableName, line) {
    const lines = content.split('\n');
    const currentLine = lines[line - 1] || '';
    
    // Check if variable is in type position: param: Type, variable: Type
    if (/:\s*\w+\s*[,)=]/.test(currentLine)) {
      return true;
    }
    
    return false;
  }

  /**
   * Check if variable is function parameter context
   */
  isFunctionParameter(content, variableName, line) {
    const lines = content.split('\n');
    const currentLine = lines[line - 1] || '';
    
    // Check if we're in a function parameter list
    const functionPatterns = [
      /function.*\(.*\w+.*\)/,           // function foo(param)
      /\(.*\w+.*\)\s*=>/,               // (param) => 
      /\w+\s*=>\s*/,                    // param =>
      /\w+\s*:\s*\w+\s*[,)]/,          // param: Type,
      /\w+\?\s*:\s*\w+\s*[,)]/         // param?: Type,
    ];
    
    return functionPatterns.some(pattern => pattern.test(currentLine));
  }

  /**
   * Check if variable has clear type information
   */
  hasTypeInformation(content, variableName, line) {
    const lines = content.split('\n');
    const currentLine = lines[line - 1] || '';
    
    // TypeScript type annotations make variables clearer
    // Example: function process(u: User) - 'u' is clear from User type
    const typePatterns = [
      new RegExp(`${variableName}\\s*:\\s*[A-Z]\\w+`),  // param: TypeName
      new RegExp(`${variableName}\\s*:\\s*\\w+\\[\\]`), // param: Type[]
      new RegExp(`${variableName}\\s*:\\s*string|number|boolean`) // primitive types
    ];
    
    return typePatterns.some(pattern => pattern.test(currentLine));
  }

  /**
   * Check if variable is coordinate-like or math notation
   */
  isCoordinate(variableName) {
    return /^[xyz](\d+)?$/i.test(variableName) || 
           /^(width|height|top|left|right|bottom)$/i.test(variableName) ||
           /^[wh](\d+)?$/i.test(variableName);
  }

  /**
   * Check if variable is math/algorithm context
   */
  isMathContext(content, variableName, line) {
    const lines = content.split('\n');
    const currentLine = lines[line - 1] || '';
    const prevLine = lines[line - 2] || '';
    const nextLine = lines[line] || '';
    
    // Check for math variable patterns
    const mathPatterns = [
      // Coordinate pairs: x1, y1, x2, y2
      /^[xyz][12]$/i,
      // Delta notation: dx, dy, dt, dr
      /^d[xyztr]$/i,
      // Math constants: a, b, c in equations
      /^[abc]$/i,
      // Vector components: vx, vy, vz
      /^v[xyz]$/i,
      // Position/point notation: p1, p2
      /^p\d+$/i
    ];
    
    if (mathPatterns.some(pattern => pattern.test(variableName))) {
      return true;
    }
    
    // Context-based detection
    const contextLines = [prevLine, currentLine, nextLine].join(' ');
    
    // Check for math function names in the context
    if (/function\s+(distance|calculate|compute|solve|formula|algorithm|equation|math)/i.test(contextLines)) {
      return true;
    }
    
    // Math operations context
    if (/[+\-*/=]\s*\w+|\w+\s*[+\-*/=]/.test(currentLine)) {
      return true;
    }
    
    // Math functions context
    if (/Math\.|sqrt|pow|abs|sin|cos|tan|distance|calculate/i.test(contextLines)) {
      return true;
    }
    
    // Multiple single-char variables (typical in math)
    if (/const\s+[a-z]\s*=.*[a-z]\s*=/.test(currentLine)) {
      return true;
    }
    
    // Check for function parameters that are clearly coordinates/math
    // Example: function distance(x1: number, y1: number, x2: number, y2: number)
    if (/function.*\([^)]*\b(x1|y1|x2|y2|dx|dy|dz|dt)\b.*\)/.test(contextLines)) {
      return true;
    }
    
    return false;
  }

  /**
   * Check if variable is in callback/iteration context  
   */
  isCallbackContext(content, variableName, line) {
    const lines = content.split('\n');
    const currentLine = lines[line - 1] || '';
    
    // Array methods with callback
    return /\.(map|filter|forEach|reduce|find|some|every)\s*\(/.test(currentLine);
  }

  /**
   * Check if variable is in comparison/sorting context
   */
  isComparisonContext(content, variableName, line) {
    const lines = content.split('\n');
    const currentLine = lines[line - 1] || '';
    const prevLine = lines[line - 2] || '';
    const nextLine = lines[line] || '';
    
    const contextLines = [prevLine, currentLine, nextLine].join(' ');
    
    // Comparison/sorting patterns
    return /\.sort\s*\(/.test(contextLines) ||
           /compare\s*\(/.test(contextLines) ||
           /\w+\s*[<>=]+\s*\w+/.test(contextLines);
  }

  /**
   * Check if variable is in event context
   */
  isEventContext(content, variableName, line) {
    const lines = content.split('\n');
    const contextLines = lines.slice(Math.max(0, line - 2), line + 1).join(' ');
    
    return /(?:event|evt|e)\s*[:=]|addEventListener|onClick|onSubmit|handler/i.test(contextLines);
  }

  /**
   * Check if variable is in destructuring context
   */
  isDestructuring(content, variableName, line) {
    const lines = content.split('\n');
    const currentLine = lines[line - 1] || '';
    
    // Check for object destructuring
    if (/const\s*{[^}]*}\s*=/.test(currentLine) || /{\s*\w+[^}]*}/.test(currentLine)) {
      return true;
    }
    
    // Check for array destructuring  
    if (/const\s*\[[^\]]*\]\s*=/.test(currentLine) || /\[\s*\w+[^\]]*\]/.test(currentLine)) {
      return true;
    }
    
    return false;
  }

  /**
   * Analyze variable names in files
   * @param {string[]} files - Array of file paths to analyze
   * @param {string} language - Programming language
   * @param {Object} options - Analysis options
   * @returns {Object[]} Array of violations
   */
  async analyze(files, language, options) {
    const violations = [];
    
    for (const filePath of files) {
      try {
        // Read file content
        const fs = require('fs');
        const content = fs.readFileSync(filePath, 'utf8');
        
        // Analyze file content
        const fileViolations = this.analyzeContent(content, filePath);
        violations.push(...fileViolations);
        
      } catch (error) {
        console.error(`Error analyzing file ${filePath}:`, error.message);
      }
    }
    
    return violations;
  }

  /**
   * Clean line by removing comments but preserving structure for regex matching
   * @param {string} line - Original line
   * @returns {object} { cleanLine, commentRanges }
   */
  cleanLineForMatching(line) {
    return CommentDetector.cleanLineForMatching(line);
  }

  /**
   * Check if a variable at a specific position is inside a comment
   * @param {string} line - Line content
   * @param {number} position - Position in line where variable was found
   * @returns {boolean} True if position is inside a comment
   */
  isPositionInComment(line, position) {
    return CommentDetector.isPositionInComment(line, position);
  }

  /**
   * Check if a line is a comment or inside a comment block
   * @param {string} line - Line to check
   * @param {number} lineIndex - Current line index
   * @param {string[]} allLines - All lines in content
   * @returns {boolean} True if line is commented
   */
  isCommentedLine(line, lineIndex, allLines) {
    const trimmedLine = line.trim();
    
    // Check single-line comments
    if (trimmedLine.startsWith('//') || trimmedLine.startsWith('#')) {
      return true;
    }
    
    // Check if we're inside a multi-line comment
    let inComment = false;
    for (let i = 0; i <= lineIndex; i++) {
      const currentLine = allLines[i];
      
      // Find /* and */ on the same line or across lines
      let startPos = 0;
      while (startPos < currentLine.length) {
        const commentStart = currentLine.indexOf('/*', startPos);
        const commentEnd = currentLine.indexOf('*/', startPos);
        
        if (commentStart !== -1 && (commentEnd === -1 || commentStart < commentEnd)) {
          inComment = true;
          startPos = commentStart + 2;
        } else if (commentEnd !== -1 && inComment) {
          inComment = false;
          startPos = commentEnd + 2;
        } else {
          break;
        }
      }
      
      // If we're at the target line and inside a comment
      if (i === lineIndex && inComment) {
        return true;
      }
    }
    
    return false;
  }

  /**
   * Remove comments from content
   * @param {string} content - File content
   * @returns {string} Content with comments removed
   */
  removeComments(content) {
    let result = content;
    
    // Remove single line comments (// comments)
    result = result.replace(/\/\/.*$/gm, '');
    
    // Remove multi-line comments (/* comments */)
    result = result.replace(/\/\*[\s\S]*?\*\//g, '');
    
    // Remove # comments (for other languages like Python, Shell)
    result = result.replace(/#.*$/gm, '');
    
    return result;
  }

  /**
   * Analyze variable names in content
   */
  analyzeContent(content, filePath) {
    // Ensure content is a string
    if (typeof content !== 'string') {
      console.error('Content is not a string:', typeof content);
      return [];
    }

    const violations = [];
    const lines = content.split('\n'); // Use original content to preserve line numbers
    
    lines.forEach((line, index) => {
      const lineNumber = index + 1;
      
      // Skip commented lines
      if (this.isCommentedLine(line, index, lines)) {
        return;
      }
      
      // Skip empty lines
      if (!line.trim()) {
        return;
      }
      
      // Clean line for better pattern matching
      const { cleanLine } = this.cleanLineForMatching(line);
      
      // Match variable declarations on cleaned line
      const patterns = [
        // const/let/var declarations
        /(?:const|let|var)\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*[=:]/g,
        // Function parameters in function declarations
        /function\s+\w*\s*\(\s*([^)]+)\s*\)/g,
        // Arrow function parameters
        /\(\s*([^)]+)\s*\)\s*=>/g,
        // Single parameter arrow functions
        /([a-zA-Z_$][a-zA-Z0-9_$]*)\s*=>/g,
      ];
      
      patterns.forEach(pattern => {
        let match;
        while ((match = pattern.exec(cleanLine)) !== null) {
          let variableNames = [];
          
          if (match[1]) {
            // Handle parameter lists
            if (match[1].includes(',')) {
              variableNames = match[1].split(',')
                .map(param => param.trim().split(/[:\s=]/)[0].trim())
                .filter(name => name && /^[a-zA-Z_$][a-zA-Z0-9_$]*$/.test(name));
            } else {
              const cleanName = match[1].trim().split(/[:\s=]/)[0].trim();
              if (cleanName && /^[a-zA-Z_$][a-zA-Z0-9_$]*$/.test(cleanName)) {
                variableNames = [cleanName];
              }
            }
          }
          
          variableNames.forEach(variableName => {
            // No need to check position since we're using cleaned line
            const violation = this.checkVariableName(variableName, content, lineNumber, match.index);
            if (violation) {
              violations.push({
                ruleId: 'C003',
                severity: 'warning',
                message: violation.message,
                line: lineNumber,
                column: match.index + 1,
                filePath: filePath
              });
            }
          });
        }
      });
    });
    
    return violations;
  }

  /**
   * Check individual variable name
   */
  checkVariableName(variableName, content, line, column) {
    // Skip if empty or invalid
    if (!variableName || typeof variableName !== 'string') {
      return null;
    }

    // Skip TypeScript/React specific names
    if (variableName.startsWith('_') || variableName.includes('$')) {
      return null;
    }

    // Skip common React patterns
    if (/^(useState|useEffect|useCallback|useMemo|useRef|useContext)$/.test(variableName)) {
      return null;
    }

    // Check single character variables
    if (variableName.length === 1) {
      const lowerName = variableName.toLowerCase();
      
      // Allow if in allowed single char list
      if (this.options.allowedSingleChar.has(lowerName)) {
        return null;
      }
      
      // Allow if in loop context
      if (this.isLoopContext(content, variableName, line)) {
        return null;
      }
      
      // Allow coordinates
      if (this.isCoordinate(variableName)) {
        return null;
      }
      
      // Allow in event context
      if (lowerName === 'e' && this.isEventContext(content, variableName, line)) {
        return null;
      }
      
      // Allow in math context (a, b, c coefficients)
      if (['a', 'b', 'c'].includes(lowerName) && this.isMathContext(content, variableName, line)) {
        return null;
      }
      
      // Allow a, b in comparison/sorting context
      if (['a', 'b'].includes(lowerName) && this.isComparisonContext(content, variableName, line)) {
        return null;
      }
      
      return {
        message: `Variable '${variableName}' is only 1 character long. Use descriptive names (except for counters like i, j, k).`
      };
    }

    const lowerName = variableName.toLowerCase();

    // Check if it's an allowed abbreviation
    if (this.options.allowedAbbreviations.has(lowerName)) {
      return null;
    }

    // Allow in destructuring context  
    if (this.isDestructuring(content, variableName, line)) {
      return null;
    }

    // Allow in callback/function parameter context for common terms
    if (['item', 'data', 'element'].includes(lowerName) && 
        (this.isCallbackContext(content, variableName, line) || 
         this.isFunctionParameter(content, variableName, line))) {
      return null;
    }

    // Check for minimum length
    if (variableName.length < this.options.minLength) {
      return {
        message: `Variable '${variableName}' is too short (${variableName.length} characters). Use descriptive names with at least ${this.options.minLength} characters.`
      };
    }

    // Check for math context BEFORE suspicious patterns
    if (this.isMathContext(content, variableName, line)) {
      return null;
    }

    // Check for suspicious patterns
    for (const pattern of this.suspiciousPatterns) {
      if (pattern.test(lowerName)) {
        return {
          message: `Variable '${variableName}' appears to be an unclear abbreviation. Use full descriptive names.`
        };
      }
    }

    // Check for unclear names
    if (this.unclearNames.has(lowerName)) {
      return {
        message: `Variable '${variableName}' is unclear or ambiguous. Use more specific descriptive names.`
      };
    }

    return null;
  }
}

module.exports = C003NoVagueAbbreviations;
