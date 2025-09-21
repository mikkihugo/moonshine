const fs = require('fs');
const path = require('path');
const ts = require('typescript');
const { CommentDetector } = require('../../utils/rule-helpers');

/**
 * SMART C006 ANALYZER - INTELLIGENT FUNCTION NAMING ANALYSIS
 * 
 * 🧠 5-TIER ANALYSIS APPROACH:
 * 1. Context Analysis: File path, imports, class context
 * 2. Semantic Analysis: Function body inspection for intent
 * 3. Architectural Layer Detection: UI, Logic, Data, Utils
 * 4. Natural Language Processing: Verb/noun classification
 * 5. Confidence Scoring: Multi-factor violation assessment
 */
class SmartC006Analyzer {
  constructor() {
    this.ruleId = 'C006';
    this.ruleName = 'Smart Function Naming Convention';
    this.description = 'Intelligent verb-noun naming pattern detection with context awareness';
    
    // 🎯 CONFIDENCE THRESHOLDS
    this.confidenceThresholds = {
      HIGH: 0.8,    // Clear violations
      MEDIUM: 0.6,  // Likely violations  
      LOW: 0.4      // Potential violations
    };
    
    // 🗂️ ARCHITECTURAL CONTEXT PATTERNS
    this.architecturalLayers = {
      UI: ['component', 'view', 'page', 'screen', 'modal', 'dialog'],
      DATA: ['service', 'repository', 'dao', 'api', 'client', 'adapter'],
      UTILS: ['util', 'helper', 'tool', 'lib', 'common'],
      LOGIC: ['controller', 'handler', 'processor', 'manager', 'engine']
    };
    
    // 🎭 SEMANTIC INTENT PATTERNS
    this.semanticPatterns = {
      // Return patterns indicate getters
      GETTER: [/return\s+[^;]+/, /=>\s*[^{]/, /\?\s*[^:]+:/],
      // Assignment patterns indicate setters
      SETTER: [/=\s*[^=]/, /\.push\(/, /\.set\(/],
      // Conditional patterns indicate checkers
      CHECKER: [/if\s*\(/, /\?\s*/, /return\s+(true|false)/],
      // Side effect patterns indicate actions
      ACTION: [/console\./, /fetch\(/, /\.send\(/, /\.post\(/]
    };
  }

  async analyze(files, language, config) {
    const violations = [];
    
    if (config.verbose) {
      console.log(`🔧 [DEBUG] Starting Smart C006 Analysis on ${files.length} files...`);
    }

    for (const filePath of files) {
      try {
        const fileContent = fs.readFileSync(filePath, 'utf8');
        const fileViolations = await this.analyzeFile(filePath, fileContent, language, config);
        violations.push(...fileViolations);
      } catch (error) {
        if (config.verbose) {
          console.error(`⚠️ [DEBUG] Error analyzing file ${filePath}:`, error.message);
        }
      }
    }

    if (config.verbose) {
      console.log(`🔧 [DEBUG] Smart C006 Analysis complete: ${violations.length} violations found`);
    }
    
    return violations;
  }

  async analyzeFile(filePath, content, language, config) {
    if (language !== 'typescript' && language !== 'javascript') {
      return []; // Focus on TS/JS for now
    }

    const violations = [];
    const lines = content.split('\n');
    
    // 🏗️ TIER 1: ARCHITECTURAL CONTEXT ANALYSIS
    const architecturalContext = this.analyzeArchitecturalContext(filePath, content);
    
    // Parse TypeScript/JavaScript code
    const sourceFile = ts.createSourceFile(
      filePath,
      content,
      ts.ScriptTarget.Latest,
      true
    );

    const visit = (node) => {
      // Analyze function declarations
      if (ts.isFunctionDeclaration(node) && node.name && node.body) {
        const analysis = this.smartAnalyzeFunctionName(
          node.name.text,
          node,
          sourceFile,
          architecturalContext,
          content
        );
        
        if (analysis.isViolation && analysis.confidence >= this.confidenceThresholds.LOW) {
          const namePosition = sourceFile.getLineAndCharacterOfPosition(node.name.getStart());
          const line = namePosition.line + 1;
          const column = namePosition.character + 1;
          const lineText = lines[line - 1]?.trim() || '';
          
          violations.push({
            ruleId: this.ruleId,
            file: filePath,
            line,
            column,
            message: analysis.reason,
            severity: this.getSeverityFromConfidence(analysis.confidence),
            code: lineText,
            type: analysis.type,
            confidence: analysis.confidence,
            suggestion: analysis.suggestion,
            context: analysis.context
          });
        }
      }

      // Analyze method declarations
      if (ts.isMethodDeclaration(node) && node.name) {
        const analysis = this.smartAnalyzeFunctionName(
          node.name.text,
          node,
          sourceFile,
          architecturalContext,
          content
        );
        
        if (analysis.isViolation && analysis.confidence >= this.confidenceThresholds.LOW) {
          const namePosition = sourceFile.getLineAndCharacterOfPosition(node.name.getStart());
          const line = namePosition.line + 1;
          const column = namePosition.character + 1;
          const lineText = lines[line - 1]?.trim() || '';
          
          violations.push({
            ruleId: this.ruleId,
            file: filePath,
            line,
            column,
            message: analysis.reason,
            severity: this.getSeverityFromConfidence(analysis.confidence),
            code: lineText,
            type: analysis.type,
            confidence: analysis.confidence,
            suggestion: analysis.suggestion,
            context: analysis.context
          });
        }
      }

      // Analyze arrow functions assigned to variables
      if (ts.isVariableDeclaration(node) && node.name && ts.isIdentifier(node.name) && 
          node.initializer && ts.isArrowFunction(node.initializer)) {
        const analysis = this.smartAnalyzeFunctionName(
          node.name.text,
          node.initializer,
          sourceFile,
          architecturalContext,
          content
        );
        
        if (analysis.isViolation && analysis.confidence >= this.confidenceThresholds.LOW) {
          const namePosition = sourceFile.getLineAndCharacterOfPosition(node.name.getStart());
          const line = namePosition.line + 1;
          const column = namePosition.character + 1;
          const lineText = lines[line - 1]?.trim() || '';
          
          violations.push({
            ruleId: this.ruleId,
            file: filePath,
            line,
            column,
            message: analysis.reason,
            severity: this.getSeverityFromConfidence(analysis.confidence),
            code: lineText,
            type: analysis.type,
            confidence: analysis.confidence,
            suggestion: analysis.suggestion,
            context: analysis.context
          });
        }
      }
      
      ts.forEachChild(node, visit);
    };

    visit(sourceFile);
    return violations;
  }

  /**
   * 🏗️ TIER 1: ARCHITECTURAL CONTEXT ANALYSIS
   * Determines what type of file/module this is
   */
  analyzeArchitecturalContext(filePath, content) {
    const fileName = path.basename(filePath, path.extname(filePath)).toLowerCase();
    const fileDir = path.dirname(filePath).toLowerCase();
    
    // Detect architectural layer
    let layer = 'UNKNOWN';
    for (const [layerName, patterns] of Object.entries(this.architecturalLayers)) {
      if (patterns.some(pattern => fileName.includes(pattern) || fileDir.includes(pattern))) {
        layer = layerName;
        break;
      }
    }
    
    // Analyze imports for additional context
    const imports = this.extractImports(content);
    const isReactComponent = imports.some(imp => imp.includes('react')) || content.includes('JSX.Element');
    const isTestFile = fileName.includes('test') || fileName.includes('spec');
    
    return {
      layer,
      isReactComponent,
      isTestFile,
      fileName,
      imports
    };
  }

  /**
   * 🧠 TIER 2: SEMANTIC ANALYSIS
   * Analyzes function body to understand intent
   */
  analyzeSemanticIntent(functionNode, sourceFile, content) {
    if (!functionNode.body) return 'UNKNOWN';
    
    const functionText = content.substring(
      functionNode.body.getStart(),
      functionNode.body.getEnd()
    );
    
    // Check for different semantic patterns
    for (const [intent, patterns] of Object.entries(this.semanticPatterns)) {
      if (patterns.some(pattern => pattern.test(functionText))) {
        return intent;
      }
    }
    
    return 'UNKNOWN';
  }

  /**
   * 🎯 TIER 3: INTELLIGENT VERB DETECTION
   * Uses multiple strategies to detect verbs
   */
  isVerbLikeName(functionName) {
    // Strategy 0: REJECT generic/vague verbs that should be flagged
    const genericVerbs = [
      'do', 'handle', 'process', 'manage', 'execute',
      'something', 'stuff', 'thing', 'work', 'data'
    ];
    
    const isGenericVerb = genericVerbs.some(verb => {
      const verbPattern = new RegExp(`^${verb}([A-Z].*|$)`, 'i');
      return verbPattern.test(functionName);
    });
    
    // Reject names starting with generic verbs
    if (isGenericVerb) return false;
    
    // Strategy 1: Known verb prefixes (expanded beyond static list)
    const verbPrefixes = [
      'get', 'set', 'is', 'has', 'can', 'should', 'will', 'does',
      'create', 'build', 'make', 'generate', 'construct', 'produce',
      'update', 'modify', 'change', 'edit', 'alter', 'transform',
      'delete', 'remove', 'destroy', 'clean', 'clear', 'reset',
      'load', 'save', 'fetch', 'retrieve', 'find', 'search', 'query',
      'validate', 'verify', 'check', 'confirm', 'ensure', 'test',
      'calculate', 'compute', 'parse', 'format', 'convert',
      'send', 'receive', 'transmit', 'broadcast', 'emit', 'publish',
      'map', 'filter', 'sort', 'group', 'merge', 'split',
      'connect', 'disconnect', 'open', 'close', 'start', 'stop', 'run',
      'show', 'hide', 'display', 'render', 'draw', 'paint', 'animate',
      'add', 'append', 'insert', 'push', 'pop', 'shift', 'splice',
      'count', 'measure', 'monitor', 'watch', 'track', 'observe',
      'refresh', 'restore', 'reload', 'retry', 'resume', 'redirect',
      'select', 'toggle', 'switch', 'enable', 'disable', 'activate',
      'expand', 'collapse', 'scroll', 'navigate', 'submit', 'cancel',
      'on', 'trigger', 'fire', 'dispatch', 'invoke', 'call'
    ];
    
    // Strategy 2: Check if starts with known verb
    const startsWithVerb = verbPrefixes.some(verb => {
      const verbPattern = new RegExp(`^${verb}[A-Z]?`, 'i');
      return verbPattern.test(functionName);
    });
    
    if (startsWithVerb) return true;
    
    // Strategy 3: Common verb suffixes that indicate actions
    const actionSuffixes = ['ize', 'ise', 'fy', 'ate', 'en'];
    if (actionSuffixes.some(suffix => functionName.endsWith(suffix))) {
      return true;
    }
    
    // Strategy 4: English verb patterns (basic NLP)
    const verbPatterns = [
      /^(re|un|pre|de|dis)[A-Z]/, // prefixed verbs: revalidate, unload, preprocess
      /^[a-z]+ly[A-Z]/, // adverb-verb patterns: quicklyProcess
    ];
    
    return verbPatterns.some(pattern => pattern.test(functionName));
  }

  /**
   * 🎭 TIER 4: CONTEXT-AWARE NAMING RULES
   * Different rules for different contexts
   */
  getContextSpecificRules(architecturalContext, semanticIntent) {
    const rules = {
      allowedPatterns: [],
      requiredPatterns: [],
      suggestions: []
    };
    
    // React components have different naming conventions
    if (architecturalContext.isReactComponent) {
      rules.allowedPatterns.push(/^[A-Z][a-zA-Z]*$/); // PascalCase components
      rules.allowedPatterns.push(/^use[A-Z][a-zA-Z]*$/); // React hooks
      rules.allowedPatterns.push(/^handle[A-Z][a-zA-Z]*$/); // Event handlers
    }
    
    // Test files have different patterns
    if (architecturalContext.isTestFile) {
      rules.allowedPatterns.push(/^(test|it|describe|should|expect)[A-Z]?/);
    }
    
    // Data layer functions often have CRUD patterns
    if (architecturalContext.layer === 'DATA') {
      rules.suggestions.push('Consider CRUD verbs: create, read, update, delete');
    }
    
    // UI layer functions often have interaction verbs
    if (architecturalContext.layer === 'UI') {
      rules.suggestions.push('Consider UI verbs: show, hide, toggle, render, display');
    }
    
    return rules;
  }

  /**
   * 🎯 TIER 5: COMPREHENSIVE SMART ANALYSIS
   * Combines all tiers for intelligent assessment
   */
  smartAnalyzeFunctionName(functionName, functionNode, sourceFile, architecturalContext, content) {
    // Skip special functions
    if (this.isSpecialFunction(functionName, architecturalContext)) {
      return { isViolation: false };
    }
    
    // Get semantic intent
    const semanticIntent = this.analyzeSemanticIntent(functionNode, sourceFile, content);
    
    // Get context-specific rules
    const contextRules = this.getContextSpecificRules(architecturalContext, semanticIntent);
    
    // Check if allowed by context-specific patterns
    if (contextRules.allowedPatterns.some(pattern => pattern.test(functionName))) {
      return { isViolation: false };
    }
    
    // Check if name follows verb-noun pattern
    const isVerbLike = this.isVerbLikeName(functionName);
    
    if (isVerbLike) {
      return { isViolation: false };
    }
    
    // 🧮 CONFIDENCE CALCULATION
    let confidence = 0.5; // Base confidence
    
    // Boost confidence for clearly generic/vague patterns
    const vagueFunctionNames = [
      'doSomething', 'handleStuff', 'processData', 'processInfo',
      'executeWork', 'manageItems', 'doWork', 'handleData',
      'something', 'stuff', 'thing', 'data', 'info', 'item'
    ];
    
    if (vagueFunctionNames.some(vague => functionName.toLowerCase().includes(vague.toLowerCase()))) {
      confidence += 0.4; // Strongly boost confidence for obviously vague names
    }
    
    // Boost confidence for clear noun-only patterns
    if (/^[a-z]+$/.test(functionName)) {
      confidence += 0.3; // Simple lowercase nouns: user, data
    }
    
    if (/^[a-z]+[A-Z][a-z]+$/.test(functionName)) {
      confidence += 0.2; // Simple camelCase nouns: userData, userInfo
    }
    
    // Reduce confidence for complex names (might have hidden verbs)
    if (functionName.length > 15) {
      confidence -= 0.1;
    }
    
    // Reduce confidence for utils/helpers (more flexible naming)
    if (architecturalContext.layer === 'UTILS') {
      confidence -= 0.2;
    }
    
    // Reduce confidence for test files
    if (architecturalContext.isTestFile) {
      confidence -= 0.3;
    }
    
    // Cap confidence
    confidence = Math.min(Math.max(confidence, 0.1), 1.0);
    
    // 💬 INTELLIGENT MESSAGING
    const context = {
      layer: architecturalContext.layer,
      intent: semanticIntent,
      isReactComponent: architecturalContext.isReactComponent,
      isTestFile: architecturalContext.isTestFile
    };
    
    let reason = `Function '${functionName}' should follow verb-noun naming pattern`;
    let suggestion = this.generateSmartSuggestion(functionName, semanticIntent, architecturalContext);
    
    if (architecturalContext.layer !== 'UNKNOWN') {
      reason += ` (${architecturalContext.layer} layer)`;
    }
    
    return {
      isViolation: true,
      reason,
      type: 'smart_naming_violation',
      confidence,
      suggestion,
      context
    };
  }

  /**
   * 💡 SMART SUGGESTION GENERATOR
   */
  generateSmartSuggestion(functionName, semanticIntent, architecturalContext) {
    const baseNoun = functionName.charAt(0).toUpperCase() + functionName.slice(1);
    
    switch (semanticIntent) {
      case 'GETTER':
        return `get${baseNoun}()`;
      case 'SETTER':
        return `set${baseNoun}()`;
      case 'CHECKER':
        return `is${baseNoun}() or has${baseNoun}()`;
      case 'ACTION':
        return `process${baseNoun}() or handle${baseNoun}()`;
      default:
        if (architecturalContext.layer === 'DATA') {
          return `fetch${baseNoun}() or create${baseNoun}()`;
        }
        if (architecturalContext.layer === 'UI') {
          return `render${baseNoun}() or show${baseNoun}()`;
        }
        return `get${baseNoun}() or process${baseNoun}()`;
    }
  }

  /**
   * 🛡️ ENHANCED SPECIAL FUNCTION DETECTION
   */
  isSpecialFunction(name, architecturalContext) {
    const specialFunctions = [
      'constructor', 'toString', 'valueOf', 'toJSON',
      'main', 'init', 'setup', 'teardown', 'build',
      'onCreate', 'onDestroy', 'onStart', 'onStop',
      'onPause', 'onResume', 'onSaveInstanceState',
      'equals', 'hashCode', 'compareTo', 'clone',
      'finalize', 'notify', 'notifyAll', 'wait'
    ];

    // Basic special function check
    if (specialFunctions.includes(name) || name.startsWith('_') || name.startsWith('$')) {
      return true;
    }
    
    // React component names (PascalCase)
    if (architecturalContext.isReactComponent && /^[A-Z][a-zA-Z]*$/.test(name)) {
      return true;
    }
    
    // React hooks
    if (name.startsWith('use') && /^use[A-Z]/.test(name)) {
      return true;
    }
    
    // Test function names
    if (architecturalContext.isTestFile && /^(test|it|describe|should|expect)/.test(name)) {
      return true;
    }
    
    return false;
  }

  /**
   * 🎯 UTILITY METHODS
   */
  extractImports(content) {
    const importRegex = /import\s+.*?\s+from\s+['"]([^'"]+)['"]/g;
    const imports = [];
    let match;
    
    while ((match = importRegex.exec(content)) !== null) {
      imports.push(match[1]);
    }
    
    return imports;
  }

  getSeverityFromConfidence(confidence) {
    if (confidence >= this.confidenceThresholds.HIGH) return 'warning';
    if (confidence >= this.confidenceThresholds.MEDIUM) return 'info';
    return 'hint';
  }
}

module.exports = new SmartC006Analyzer();
