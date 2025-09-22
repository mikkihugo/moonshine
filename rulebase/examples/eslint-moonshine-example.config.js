/**
 * ESLint Configuration with MoonShine Rules
 *
 * Shows how to use MoonShine's 835 rules with standard ESLint configuration.
 * Uses ESLint-compatible naming for seamless integration.
 */

module.exports = {
  extends: [
    'eslint:recommended',
    '@typescript-eslint/recommended'
  ],
  plugins: [
    '@moonshine'  // MoonShine plugin with 835 rules
  ],
  rules: {
    // Standard ESLint rules (now powered by OXC)
    'no-eval': 'error',
    'no-console': 'warn',
    'no-debugger': 'error',
    'no-unused-vars': 'warn',
    'prefer-const': 'warn',
    'prefer-template': 'warn',
    'eqeqeq': 'warn',
    'curly': 'warn',

    // Security rules (OXC-powered)
    'no-implied-eval': 'error',
    'no-script-url': 'error',
    'no-unsafe-finally': 'error',
    'detect-sql-injection': 'error',
    'no-hardcoded-secrets': 'error',
    'crypto-secure-random': 'warn',

    // Performance rules (OXC-powered)
    'no-array-constructor': 'warn',
    'no-new-object': 'warn',
    'prefer-object-spread': 'warn',
    'prefer-spread': 'warn',
    'object-shorthand': 'warn',
    'prefer-destructuring': 'warn',

    // MoonShine AI-powered rules (scoped)
    '@moonshine/aibehavioral-analysis-1': 'info',
    '@moonshine/cognitive-analysis-1': 'info',
    '@moonshine/ai-code-quality-oracle': 'info',
    '@moonshine/production-excellence-suite': 'warn',

    // MoonShine hybrid rules
    '@moonshine/hybrid-1': 'warn',
    '@moonshine/hybrid-2': 'warn',
    '@moonshine/excellence-1': 'warn',

    // Comprehensive security suite
    'no-security-vulnerabilities': 'error',

    // AI-enhanced pattern detection
    '@moonshine/patterns-analysis-1': 'info',
    '@moonshine/architecture-analysis-1': 'info',
    '@moonshine/aienhanced-analysis-1': 'warn'
  },

  // MoonShine-specific settings
  settings: {
    moonshine: {
      ai_enabled: true,
      max_parallel: 8,
      cache_enabled: true,

      // AI model preferences
      ai_provider: 'claude',
      ai_model: 'claude-3-sonnet',

      // Rule categories to enable
      categories: [
        'Security',
        'Performance',
        'CodeQuality',
        'TypeScript',
        'AIBehavioral',
        'Hybrid'
      ],

      // Performance settings
      execution_timeout: 10000,
      enable_caching: true,

      // Ultra-architecture features
      enable_correlation_engine: true,
      enable_adaptive_learning: true,
      enable_intelligent_pipeline: true
    }
  },

  overrides: [
    {
      files: ['*.ts', '*.tsx'],
      rules: {
        // TypeScript-specific rules
        '@moonshine/typescript-analysis-1': 'warn',
        '@moonshine/typescript-analysis-2': 'warn'
      }
    },
    {
      files: ['*.test.js', '*.spec.js'],
      rules: {
        // Disable some rules for tests
        'no-console': 'off',
        '@moonshine/production-excellence-suite': 'off'
      }
    },
    {
      files: ['src/production/**'],
      rules: {
        // Stricter rules for production code
        '@moonshine/production-excellence-suite': 'error',
        '@moonshine/excellence-1': 'error',
        'no-security-vulnerabilities': 'error'
      }
    }
  ]
};