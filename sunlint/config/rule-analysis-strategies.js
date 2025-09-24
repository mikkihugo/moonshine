/**
 * Rule Analysis Strategy Configuration
 * Defines optimal analysis methods for each rule type
 */

module.exports = {
  // Rules that benefit significantly from AST analysis
  astPreferred: {
    'C003': {
      reason: 'Variable naming requires context awareness (types, scopes, conventions)',
      methods: ['ast', 'regex'],
      accuracy: { ast: 95, regex: 75 }
    },
    'C010': {
      reason: 'Block nesting requires precise scope tracking',
      methods: ['ast', 'regex'],
      accuracy: { ast: 95, regex: 75 }
    },
    'C012': {
      reason: 'Command Query Separation requires function behavior analysis',
      methods: ['ast', 'regex'],
      accuracy: { ast: 95, regex: 80 }
    },
    'C015': {
      reason: 'Function parameter counting benefits from AST',
      methods: ['ast', 'regex'],
      accuracy: { ast: 95, regex: 85 }
    },
    'C017': {
      reason: 'Constructor logic analysis needs semantic context - Phase 2 with symbol-based analysis',
      methods: ['semantic', 'ast', 'regex'],
      accuracy: { semantic: 95, ast: 85, regex: 70 },
      strategy: 'semantic-primary'
    },
    'S015': {
      reason: 'TLS certificate validation requires AST context analysis',
      methods: ['ast', 'regex'],
      accuracy: { ast: 95, regex: 80 }
    },
    'S023': {
      reason: 'JSON injection detection requires AST context analysis',
      methods: ['ast', 'regex'],
      accuracy: { ast: 95, regex: 60 }
    }
  },

  // Rules where regex is sufficient and efficient
  regexOptimal: {
    'C001': {
      reason: 'Naming patterns are string-based',
      methods: ['regex'],
      accuracy: { regex: 95 }
    },
    'C002': {
      reason: 'Duplicate code detection requires cross-file analysis',
      methods: ['regex'],
      accuracy: { regex: 85 }
    },
    'C043': {
      reason: 'Console/print detection via simple patterns',
      methods: ['regex'],
      accuracy: { regex: 90 }
    },
    'C070': {
      reason: 'Real-time dependencies detection via timer/sleep patterns',
      methods: ['regex'],
      accuracy: { regex: 95 }
    },
    'S001': {
      reason: 'Security patterns are often string-based',
      methods: ['regex', 'ast'],
      accuracy: { regex: 85, ast: 90 }
    }
  },

  // Rules that require hybrid approach
  hybridOptimal: {
    'C018': {
      reason: 'Do not throw generic errors',
      methods: ['semantic', 'regex'],
      strategy: 'semantic-primary-regex-fallback',
      accuracy: { semantic: 90, regex: 70, combined: 95 }
    },
    'C029': {
      reason: 'Catch block analysis needs context + patterns',
      methods: ['ast', 'regex', 'semantic'],
      strategy: 'ast-primary-regex-fallback',
      accuracy: { ast: 90, regex: 75, combined: 95 }
    },
    'C035': {
      reason: 'Error logging context requires symbol-based + regex analysis',
      methods: ['semantic', 'regex'],
      strategy: 'semantic-primary-regex-fallback',
      accuracy: { semantic: 90, regex: 70, combined: 95 }
    },
    'C040': {
      reason: 'Validation centralization requires project-wide symbol analysis + data flow tracking',
      methods: ['semantic', 'regex'],
      strategy: 'semantic-primary-regex-fallback',
      accuracy: { semantic: 95, regex: 75, combined: 97 }
    },
    'C076': {
      reason: 'Public API type enforcement requires symbol-based analysis for export boundaries',
      methods: ['semantic'],
      strategy: 'semantic-primary',
      accuracy: { semantic: 95 }
    },
    'C041': {
      reason: 'Hardcoded secrets need AST literal analysis like ESLint',
      methods: ['ast', 'regex'],
      strategy: 'ast-primary-regex-fallback',
      accuracy: { ast: 95, regex: 70, combined: 95 }
    },
    'C047': {
      reason: 'Retry logic detection needs pattern + structure',
      methods: ['regex', 'ast'],
      strategy: 'regex-primary-ast-enhancement',
      accuracy: { regex: 80, ast: 85, combined: 92 }
    }
  },

  // Rules that may need future enhancement
  experimental: {
    'C072': {
      reason: 'Test assertion counting - exploring AI enhancement',
      methods: ['regex', 'ast', 'ai'],
      strategy: 'progressive-enhancement'
    }
  }
};
