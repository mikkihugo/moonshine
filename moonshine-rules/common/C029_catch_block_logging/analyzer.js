/**
 * C029 Analyzer - Smart Pipeline Integration
 * 
 * This analyzer forwards to the Smart Pipeline for superior accuracy and performance
 */

const fs = require('fs');
const path = require('path');

class C029Analyzer {
  constructor(options = {}) {
    this.ruleId = 'C029';
    this.ruleName = 'Enhanced Catch Block Error Logging';
    this.description = 'Mọi catch block phải log nguyên nhân lỗi đầy đủ và bảo toàn context (Smart Pipeline 3-stage analysis)';
    this.verbose = options.verbose || false;
    
    // Load Smart Pipeline as primary analyzer
    this.smartPipeline = null;
    
    try {
      this.smartPipeline = require('./analyzer-smart-pipeline.js');
      if (this.verbose) {
        console.log('[DEBUG] 🎯 C029: Smart Pipeline loaded (3-stage: Regex → AST → Data Flow)');
      }
    } catch (error) {
      if (this.verbose) {
        console.warn('[DEBUG] ⚠️ C029: Smart Pipeline failed, using fallback:', error.message);
      }
      this.smartPipeline = null;
    }
  }

  async analyze(files, language, options = {}) {
    // Store verbose option for this analysis
    this.verbose = options.verbose || this.verbose || false;
    
    // Use Smart Pipeline as primary choice
    if (this.smartPipeline) {
      if (this.verbose) {
        console.log('[DEBUG] 🎯 C029: Using Smart Pipeline (3-stage analysis)...');
      }
      return await this.smartPipeline.analyze(files, language, options);
    } else {
      if (this.verbose) {
        console.log('[DEBUG] 🔍 C029: Using fallback regex analysis...');
      }
      return await this.analyzeWithRegex(files, language, options);
    }
  }

  async analyzeWithRegex(files, language, options = {}) {
    const violations = [];
    
    for (const filePath of files) {
      if (options.verbose) {
        console.log(`🔍 C029 Regex: Processing ${path.basename(filePath)}...`);
      }
      
      try {
        const content = fs.readFileSync(filePath, 'utf8');
        const fileViolations = await this.analyzeFile(filePath, content, language);
        violations.push(...fileViolations);
      } catch (error) {
        console.warn(`⚠️ C029: Error processing ${filePath}:`, error.message);
      }
    }
    
    return violations;
  }

  async analyzeFile(filePath, content, language) {
    const violations = [];
    const lines = content.split('\n');
    
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      
      // Simple catch block detection
      if (line.includes('catch') && line.includes('(')) {
        const catchBlock = this.extractCatchBlock(lines, i);
        
        if (this.isCatchBlockEmpty(catchBlock.content)) {
          violations.push({
            file: filePath,
            line: i + 1,
            column: line.indexOf('catch') + 1,
            message: 'Empty catch block detected',
            severity: 'error',
            ruleId: this.ruleId,
            type: 'empty_catch'
          });
        }
      }
    }
    
    return violations;
  }

  extractCatchBlock(lines, startIndex) {
    const content = [];
    let braceCount = 0;
    let inBlock = false;
    
    for (let i = startIndex; i < lines.length; i++) {
      const line = lines[i];
      content.push(line);
      
      for (const char of line) {
        if (char === '{') {
          braceCount++;
          inBlock = true;
        } else if (char === '}') {
          braceCount--;
          if (braceCount === 0 && inBlock) {
            return { content, endIndex: i };
          }
        }
      }
    }
    
    return { content, endIndex: startIndex };
  }

  isCatchBlockEmpty(content) {
    const blockContent = content.join('\n');
    
    // Remove comments and whitespace
    const cleanContent = blockContent
      .replace(/\/\*[\s\S]*?\*\//g, '') // Remove multi-line comments
      .replace(/\/\/.*$/gm, '') // Remove single-line comments
      .replace(/\s+/g, ' ') // Normalize whitespace
      .trim();
    
    // Check if only contains catch declaration and braces
    const hasOnlyStructure = /^catch\s*\([^)]*\)\s*\{\s*\}$/.test(cleanContent);
    
    return hasOnlyStructure;
  }
}

module.exports = C029Analyzer;
