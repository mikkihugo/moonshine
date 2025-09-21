/**
 * S024 Regex-Based Analyzer - Protect against XPath Injection and XML External Entity (XXE)
 * Fallback analysis using regex patterns for Express.js, NestJS, and TypeScript
 */

const fs = require("fs");

class S024RegexBasedAnalyzer {
  constructor(semanticEngine = null) {
    this.semanticEngine = semanticEngine;
    this.ruleId = "S024";
    this.category = "security";

    // XPath injection patterns - user input in XPath queries
    this.xpathInjectionPatterns = [
      // XPath evaluate methods with user input
      /\.evaluate\s*\(\s*[^,]*(?:req\.|request\.|params\.|query\.|body\.)[^,]*\s*,/gi,
      /xpath\s*\.\s*(?:evaluate|select|selectText|selectValue)\s*\(\s*[^,]*(?:req\.|request\.|params\.|query\.|body\.)[^,]*\s*[,)]/gi,
      
      // XPath string concatenation with user input
      /['"`][^'"`]*\+\s*(?:req\.|request\.|params\.|query\.|body\.)[^'"`]*['"`]/gi,
      /(?:req\.|request\.|params\.|query\.|body\.)[^+]*\+\s*['"`][^'"`]*['"`]/gi,
      
      // XPath libraries with direct user input
      /xpath\s*\(\s*[^,]*(?:req\.|request\.|params\.|query\.|body\.)[^)]*\)/gi,
      /xmldom\s*\.\s*XPath\s*\(\s*[^,]*(?:req\.|request\.|params\.|query\.|body\.)[^)]*\)/gi,
      
      // Template literals with user input in XPath
      /`[^`]*\$\{[^}]*(?:req\.|request\.|params\.|query\.|body\.)[^}]*\}[^`]*`/gi
    ];

    // XXE vulnerability patterns - insecure XML parsing
    this.xxeVulnerabilityPatterns = [
      // XML parsers without XXE protection
      /new\s+DOMParser\s*\(\s*\)/gi,
      /libxmljs\s*\.\s*parseXml\s*\(/gi,
      /xml2js\s*\.\s*parseString\s*\(/gi,
      /xmldom\s*\.\s*DOMParser\s*\(/gi,
      /fast-xml-parser/gi,
      
      // XML parsing without entity resolution disabled
      /\.parseFromString\s*\(/gi,
      /XMLHttpRequest\s*\(\s*\)/gi,
      
      // XSLT processors without XXE protection
      /new\s+XSLTProcessor\s*\(\s*\)/gi,
      /xslt\s*\.\s*transform\s*\(/gi,
      
      // SAX parsers without XXE protection
      /sax\s*\.\s*(?:parser|createStream)\s*\(/gi,
      /new\s+sax\s*\.\s*SAXParser\s*\(/gi
    ];

    // Secure patterns that should NOT be flagged
    this.securePatterns = [
      // XPath with proper parameterization/escaping
      /xpath\s*\.\s*(?:evaluate|select)\s*\(\s*['"`][^'"`${}]*['"`]\s*,/gi,
      /\.evaluate\s*\(\s*['"`][^'"`${}]*['"`]\s*,/gi,
      
      // XML parsers with XXE protection explicitly disabled
      /resolveExternalEntities\s*:\s*false/gi,
      /\.setFeature\s*\(\s*['"`]http:\/\/apache\.org\/xml\/features\/disallow-doctype-decl['"`]\s*,\s*true\s*\)/gi,
      /\.setFeature\s*\(\s*['"`]http:\/\/xml\.org\/sax\/features\/external-general-entities['"`]\s*,\s*false\s*\)/gi,
      /\.setFeature\s*\(\s*['"`]http:\/\/xml\.org\/sax\/features\/external-parameter-entities['"`]\s*,\s*false\s*\)/gi
    ];

    // XPath functions that are commonly misused
    this.vulnerableXPathFunctions = [
      "evaluate",
      "select",
      "selectText", 
      "selectValue",
      "selectNodes",
      "selectSingleNode"
    ];

    // XML parsing libraries that need XXE protection
    this.vulnerableXMLLibraries = [
      "xml2js",
      "libxmljs", 
      "xmldom",
      "fast-xml-parser",
      "node-xml2js",
      "xml-parser",
      "xmldoc",
      "xpath"
    ];
  }

  async analyze(filePath) {
    if (this.verbose) {
      console.log(`🔍 [${this.ruleId}] Regex-based analysis for: ${filePath}`);
    }

    try {
      const content = fs.readFileSync(filePath, "utf8");
      return this.analyzeContent(content, filePath);
    } catch (error) {
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Regex: Error reading file:`,
          error.message
        );
      }
      return [];
    }
  }

  analyzeContent(content, filePath) {
    const violations = [];
    const lines = content.split("\n");

    // Remove comments to avoid false positives
    const cleanContent = this.removeComments(content);

    try {
      // Pattern 1: XPath Injection vulnerabilities
      violations.push(
        ...this.analyzeXPathInjection(cleanContent, lines, filePath)
      );

      // Pattern 2: XXE vulnerabilities in XML parsing
      violations.push(
        ...this.analyzeXXEVulnerabilities(cleanContent, lines, filePath)
      );

      // Pattern 3: Insecure XPath query construction
      violations.push(
        ...this.analyzeInsecureXPathConstruction(cleanContent, lines, filePath)
      );

      // Pattern 4: XML libraries without XXE protection
      violations.push(
        ...this.analyzeXMLLibraryUsage(cleanContent, lines, filePath)
      );

    } catch (error) {
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Regex: Error in analysis:`,
          error.message
        );
      }
    }

    return violations;
  }

  analyzeXPathInjection(content, lines, filePath) {
    const violations = [];

    // Check for XPath injection patterns
    for (const pattern of this.xpathInjectionPatterns) {
      let match;
      pattern.lastIndex = 0; // Reset regex state

      while ((match = pattern.exec(content)) !== null) {
        const lineNumber = this.getLineNumber(content, match.index);
        const lineContent = lines[lineNumber - 1];

        if (this.verbose) {
          console.log(
            `🔍 [${this.ruleId}] Regex: XPath injection pattern found at line ${lineNumber}: ${match[0]}`
          );
        }

        // Skip if this is a secure pattern
        if (this.isSecureXPathPattern(lineContent)) {
          continue;
        }

        violations.push({
          rule: this.ruleId,
          source: filePath,
          category: this.category,
          line: lineNumber,
          column: 1,
          message: `XPath Injection vulnerability: User input used directly in XPath query without proper sanitization`,
          severity: "error",
        });
      }
    }

    return violations;
  }

  analyzeXXEVulnerabilities(content, lines, filePath) {
    const violations = [];

    // Check for XXE vulnerability patterns
    for (const pattern of this.xxeVulnerabilityPatterns) {
      let match;
      pattern.lastIndex = 0; // Reset regex state

      while ((match = pattern.exec(content)) !== null) {
        const lineNumber = this.getLineNumber(content, match.index);
        const lineContent = lines[lineNumber - 1];
        const contextLines = this.getContextLines(lines, lineNumber - 1, 5);

        if (this.verbose) {
          console.log(
            `🔍 [${this.ruleId}] Regex: XXE vulnerability pattern found at line ${lineNumber}: ${match[0]}`
          );
        }

        // Skip if XXE protection is already implemented in context
        if (this.hasXXEProtection(contextLines.join('\n'))) {
          continue;
        }

        violations.push({
          rule: this.ruleId,
          source: filePath,
          category: this.category,
          line: lineNumber,
          column: 1,
          message: `XXE vulnerability: XML parser used without disabling external entity processing`,
          severity: "error",
        });
      }
    }

    return violations;
  }

  analyzeInsecureXPathConstruction(content, lines, filePath) {
    const violations = [];

    // Look for string concatenation or template literals with user input in XPath context
    const xpathContextPattern = /(?:xpath|XPath|XPATH).*(?:\+|`.*\$\{).*(?:req\.|request\.|params\.|query\.|body\.)/gi;
    let match;

    while ((match = xpathContextPattern.exec(content)) !== null) {
      const lineNumber = this.getLineNumber(content, match.index);
      
      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Regex: Insecure XPath construction at line ${lineNumber}: ${match[0]}`
        );
      }

      violations.push({
        rule: this.ruleId,
        source: filePath,
        category: this.category,
        line: lineNumber,
        column: 1,
        message: `XPath Injection vulnerability: XPath query constructed using string concatenation with user input`,
        severity: "error",
      });
    }

    return violations;
  }

  analyzeXMLLibraryUsage(content, lines, filePath) {
    const violations = [];

    // Check imports of vulnerable XML libraries
    const importPattern = /(?:import|require)\s*.*(?:xml2js|libxmljs|xmldom|fast-xml-parser|xpath)/gi;
    let match;

    while ((match = importPattern.exec(content)) !== null) {
      const lineNumber = this.getLineNumber(content, match.index);
      const contextLines = this.getContextLines(lines, lineNumber - 1, 10);
      const contextContent = contextLines.join('\n');

      if (this.verbose) {
        console.log(
          `🔍 [${this.ruleId}] Regex: XML library import found at line ${lineNumber}: ${match[0]}`
        );
      }

      // Check if XXE protection is implemented in the context
      if (!this.hasXXEProtection(contextContent)) {
        violations.push({
          rule: this.ruleId,
          source: filePath,
          category: this.category,
          line: lineNumber,
          column: 1,
          message: `XXE vulnerability: XML parsing library imported without implementing XXE protection measures`,
          severity: "warning",
        });
      }
    }

    return violations;
  }

  isSecureXPathPattern(lineContent) {
    // Check if the line contains secure XPath patterns
    return this.securePatterns.some(pattern => {
      pattern.lastIndex = 0;
      return pattern.test(lineContent);
    });
  }

  hasXXEProtection(content) {
    // Check if content contains XXE protection mechanisms
    const protectionPatterns = [
      /resolveExternalEntities\s*:\s*false/i,
      /setFeature.*disallow-doctype-decl.*true/i,
      /setFeature.*external-general-entities.*false/i,
      /setFeature.*external-parameter-entities.*false/i,
      /explicitChildren\s*:\s*false/i,
      /ignoreAttrs\s*:\s*true/i,
      /parseDoctype\s*:\s*false/i,
      /replaceEntities\s*:\s*false/i,
      /recover\s*:\s*false/i
    ];

    return protectionPatterns.some(pattern => {
      pattern.lastIndex = 0;
      return pattern.test(content);
    });
  }

  getContextLines(lines, centerIndex, contextSize) {
    const start = Math.max(0, centerIndex - contextSize);
    const end = Math.min(lines.length, centerIndex + contextSize + 1);
    return lines.slice(start, end);
  }

  removeComments(content) {
    // Remove single-line comments
    content = content.replace(/\/\/.*$/gm, "");
    // Remove multi-line comments
    content = content.replace(/\/\*[\s\S]*?\*\//g, "");
    return content;
  }

  getLineNumber(content, index) {
    return content.substring(0, index).split("\n").length;
  }
}

module.exports = S024RegexBasedAnalyzer;
