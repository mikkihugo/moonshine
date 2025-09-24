/**
 * SunLint Default Values and Configuration Constants
 * Centralized location for all default values used across the system
 */

/**
 * Default rule sets for different scenarios
 */
const DEFAULT_RULE_SETS = {
  // Minimal set for quick checks
  MINIMAL: ['C006', 'C019'],
  
  // Essential rules for any project
  ESSENTIAL: [
    'C001', 'C002', 'C003', 'C004', 'C005', 'C006', 
    'C007', 'C008', 'C009', 'C010', 'C019'
  ],
  
  // Security-focused rules
  SECURITY: [
    'S001', 'S002', 'S003', 'S004', 'S005',
    'S010', 'S015', 'S020', 'S025'
  ],
  
  // Performance-focused rules
  PERFORMANCE: [
    'C015', 'C020', 'C025', 'C030',
    'T010', 'T015', 'T020'
  ]
};

/**
 * Default severity levels
 */
const DEFAULT_SEVERITIES = {
  ERROR: 'error',
  WARNING: 'warning', 
  INFO: 'info',
  HINT: 'hint'
};

/**
 * Default configuration values
 */
const DEFAULT_CONFIG = {
  // Analysis options
  verbose: false,
  useRegistry: true,
  skipTests: false,
  includeTests: false,
  
  // File targeting
  maxFileSize: 1024 * 1024, // 1MB
  includePatterns: ['**/*.{js,ts,jsx,tsx,java,kt,dart,swift}'],
  excludePatterns: [
    '**/node_modules/**',
    '**/dist/**', 
    '**/build/**',
    '**/.git/**',
    '**/coverage/**'
  ],
  
  // Rule selection
  ruleSet: 'ESSENTIAL',
  categories: [],
  excludeRules: [],
  
  // Output options
  outputFormat: 'console',
  reportFile: null,
  showStats: true
};

/**
 * Default timeout values (in milliseconds)
 */
const DEFAULT_TIMEOUTS = {
  RULE_EXECUTION: 30000,      // 30 seconds per rule
  FILE_ANALYSIS: 5000,        // 5 seconds per file
  ENGINE_INITIALIZATION: 10000, // 10 seconds for engine init
  TOTAL_ANALYSIS: 300000      // 5 minutes total
};

/**
 * Default file size limits
 */
const DEFAULT_LIMITS = {
  MAX_FILE_SIZE: 1024 * 1024,    // 1MB
  MAX_FILES_PER_BATCH: 100,      // Process 100 files at once
  MAX_CONCURRENT_RULES: 5,       // Run 5 rules concurrently
  MAX_OUTPUT_LINES: 1000         // Limit output to 1000 lines
};

/**
 * Performance optimization defaults
 */
const DEFAULT_PERFORMANCE = {
  // File filtering
  ENABLE_FILE_FILTERING: true,
  MAX_FILE_SIZE: 2 * 1024 * 1024,  // 2MB per file
  MAX_TOTAL_FILES: 1000,           // Max 1000 files per analysis
  
  // Batch processing
  ENABLE_BATCHING: true,
  RULE_BATCH_SIZE: 10,             // Process 10 rules per batch
  FILE_BATCH_SIZE: 50,             // Process 50 files per batch
  
  // Concurrency
  MAX_CONCURRENT_BATCHES: 3,       // Max 3 batches running simultaneously
  
  // Memory management
  ENABLE_MEMORY_MONITORING: true,
  MAX_HEAP_SIZE_MB: 512,           // 512MB heap limit
  GC_THRESHOLD_MB: 256,            // Trigger GC at 256MB
  
  // Timeouts (adaptive)
  BASE_TIMEOUT_MS: 30000,          // 30s base timeout
  TIMEOUT_PER_FILE_MS: 100,        // +100ms per file
  TIMEOUT_PER_RULE_MS: 1000,       // +1s per rule
  MAX_TIMEOUT_MS: 120000,          // 2 minutes max timeout
  
  // Error recovery
  ENABLE_ERROR_RECOVERY: true,
  MAX_RETRIES: 2,                  // Retry failed batches up to 2 times
  RETRY_DELAY_MS: 1000,            // 1s delay between retries
  
  // Exclusion patterns for performance
  HIGH_PERFORMANCE_EXCLUDES: [
    '**/node_modules/**',
    '**/.next/**',
    '**/dist/**',
    '**/build/**',
    '**/coverage/**',
    '**/.git/**',
    '**/target/**',
    '**/out/**',
    '**/*.min.js',
    '**/*.bundle.js',
    '**/vendor/**',
    '**/lib/**',
    '**/libs/**',
    '**/.vscode/**',
    '**/.idea/**',
    '**/tmp/**',
    '**/temp/**'
  ]
};

/**
 * Default language extensions mapping
 */
const DEFAULT_LANGUAGE_EXTENSIONS = {
  javascript: ['.js', '.jsx', '.mjs'],
  typescript: ['.ts', '.tsx'],
  java: ['.java'],
  kotlin: ['.kt', '.kts'],
  dart: ['.dart'],
  swift: ['.swift'],
  python: ['.py'],
  go: ['.go'],
  rust: ['.rs'],
  php: ['.php']
};

/**
 * Get default rule set by name
 * @param {string} setName - Name of the rule set
 * @returns {Array<string>} Array of rule IDs
 */
function getDefaultRuleSet(setName = 'ESSENTIAL') {
  return DEFAULT_RULE_SETS[setName.toUpperCase()] || DEFAULT_RULE_SETS.ESSENTIAL;
}

/**
 * Get default configuration with overrides
 * @param {Object} overrides - Configuration overrides
 * @returns {Object} Merged configuration
 */
function getDefaultConfig(overrides = {}) {
  return {
    ...DEFAULT_CONFIG,
    ...overrides
  };
}

/**
 * Get file extensions for a language
 * @param {string} language - Programming language
 * @returns {Array<string>} Array of file extensions
 */
function getLanguageExtensions(language) {
  return DEFAULT_LANGUAGE_EXTENSIONS[language?.toLowerCase()] || [];
}

/**
 * Check if file size is within limits
 * @param {number} fileSize - File size in bytes
 * @returns {boolean} True if within limits
 */
function isFileSizeValid(fileSize) {
  return fileSize <= DEFAULT_LIMITS.MAX_FILE_SIZE;
}

module.exports = {
  // Rule sets
  DEFAULT_RULE_SETS,
  
  // Configuration
  DEFAULT_CONFIG,
  DEFAULT_SEVERITIES,
  DEFAULT_TIMEOUTS,
  DEFAULT_LIMITS,
  DEFAULT_PERFORMANCE,
  DEFAULT_LANGUAGE_EXTENSIONS,
  
  // Utility functions
  getDefaultRuleSet,
  getDefaultConfig,
  getLanguageExtensions,
  isFileSizeValid
};
