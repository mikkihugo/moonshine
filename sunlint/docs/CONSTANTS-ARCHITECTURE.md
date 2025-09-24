# SunLint Constants Architecture

## Overview

SunLint now uses a centralized constants sub-package located at `core/constants/` to manage all constants and configuration values. This improves code organization, maintainability, and extensibility.

## Structure

```
core/
  constants/
    categories.js    # Category-principle mappings and functions
    defaults.js      # Default configurations and values  
    engines.js       # Engine capabilities and configurations
    rules.js         # Rule-related constants and utilities
    index.js         # Barrel export for all constants
```

## Migration Guide

### Before (Old approach)
```javascript
// Scattered constants across multiple files
const { getValidCategories } = require('./core/category-constants');
const defaultRules = ['C006', 'C019']; // Hardcoded in various files
const SUPPORTED_ENGINES = { /* scattered */ };
```

### After (New centralized approach)
```javascript
// Option 1: Import specific module
const { getValidCategories } = require('./core/constants/categories');
const { getDefaultRuleSet } = require('./core/constants/defaults');
const { SUPPORTED_ENGINES } = require('./core/constants/engines');

// Option 2: Import from barrel export
const { 
  getValidCategories, 
  getDefaultRuleSet, 
  SUPPORTED_ENGINES 
} = require('./core/constants');

// Option 3: Import entire module
const constants = require('./core/constants');
const categories = constants.getValidCategories();
```

## Modules

### 1. Categories (`core/constants/categories.js`)

**Purpose**: Category-principle mappings, validation, and normalization.

**Key Exports**:
```javascript
// Constants
SUNLINT_PRINCIPLES       // Object with all principle constants
CATEGORY_PRINCIPLE_MAP   // Category to principle mapping
CATEGORY_DESCRIPTIONS    // Human-readable descriptions

// Functions
getValidCategories()     // Get all valid categories
getCategoryPrinciples(category)  // Get principles for category
isValidCategory(category)        // Validate category
normalizeCategory(category)      // Normalize and validate
getCategoryStats()              // Get statistics
```

**Example Usage**:
```javascript
const { getValidCategories, normalizeCategory } = require('./core/constants/categories');

const validCategories = getValidCategories();
// ['security', 'quality', 'performance', ...]

const normalized = normalizeCategory('QUALITY');
// 'quality'
```

### 2. Defaults (`core/constants/defaults.js`)

**Purpose**: Default configurations, rule sets, and standard values.

**Key Exports**:
```javascript
// Constants
DEFAULT_RULE_SETS        // Predefined rule sets (MINIMAL, ESSENTIAL, etc.)
DEFAULT_CONFIG          // Default configuration object
DEFAULT_SEVERITIES      // Severity levels
DEFAULT_TIMEOUTS        // Timeout configurations
DEFAULT_LIMITS          // File size and processing limits

// Functions
getDefaultRuleSet(name)     // Get predefined rule set
getDefaultConfig(overrides) // Get configuration with overrides
getLanguageExtensions(lang) // Get file extensions for language
isFileSizeValid(size)      // Check file size limits
```

**Example Usage**:
```javascript
const { getDefaultRuleSet, getDefaultConfig } = require('./core/constants/defaults');

const essentialRules = getDefaultRuleSet('ESSENTIAL');
// ['C001', 'C002', 'C003', ...]

const config = getDefaultConfig({ verbose: true });
// { verbose: true, useRegistry: true, ... }
```

### 3. Engines (`core/constants/engines.js`)

**Purpose**: Engine capabilities, configurations, and language support.

**Key Exports**:
```javascript
// Constants
SUPPORTED_ENGINES       // Object with all supported engines
ENGINE_CAPABILITIES     // Engine features and language support
ENGINE_MODES           // Execution modes (sequential, parallel, etc.)
ENGINE_PERFORMANCE     // Performance characteristics

// Functions
getEngineLanguages(engine)      // Get supported languages
getEnginesForLanguage(language) // Get engines for language
getRecommendedEngine(language)  // Get best engine for language
isLanguageSupported(engine, lang) // Check support
getEnginePerformance(engine)    // Get performance info
```

**Example Usage**:
```javascript
const { getEnginesForLanguage, getRecommendedEngine } = require('./core/constants/engines');

const jsEngines = getEnginesForLanguage('javascript');
// [{ name: 'heuristic', priority: 1, features: [...] }, ...]

const recommended = getRecommendedEngine('typescript');
// 'heuristic'
```

### 4. Rules (`core/constants/rules.js`)

**Purpose**: Rule-related constants, metadata, and utilities.

**Key Exports**:
```javascript
// Constants
RULE_SEVERITIES         // Severity levels (ERROR, WARNING, etc.)
RULE_STATUS            // Execution status values
RULE_TYPES             // Analysis types (HEURISTIC, AST, etc.)
RULE_SCOPES            // Operation scopes (FILE, PROJECT, etc.)
RULE_LANGUAGE_PATTERNS // Regex patterns for rule IDs
RULE_TIMEOUTS          // Timeout values by rule type

// Functions
getLanguageFromRuleId(ruleId)   // Extract language from rule ID
isValidRuleId(ruleId)          // Validate rule ID format
getRuleTimeout(type)           // Get timeout for rule type
getDefaultRuleMetadata(overrides) // Get rule metadata template
isValidSeverity(severity)      // Validate severity level
```

**Example Usage**:
```javascript
const { getLanguageFromRuleId, isValidRuleId } = require('./core/constants/rules');

const language = getLanguageFromRuleId('C001');
// 'common'

const isValid = isValidRuleId('CUSTOM_RULE');
// true
```

## Backward Compatibility

The following files are maintained for backward compatibility but are deprecated:

- `core/category-constants.js` - Proxies to `core/constants/categories.js`
- `core/categories.js` - Proxies to `core/constants/categories.js`

**Migration Path**:
1. Update imports to use `core/constants/*` directly
2. Replace deprecated file imports gradually
3. Legacy files will be removed in future versions

## Benefits

### 1. **Better Organization**
- Related constants grouped together
- Clear separation of concerns
- Easier to locate and modify constants

### 2. **Improved Maintainability**
- Single source of truth for each type of constant
- Centralized documentation and examples
- Easier to add new constants or modify existing ones

### 3. **Enhanced Extensibility**
- Modular structure supports new constant types
- Barrel export provides flexible import options
- Framework for adding new engines, rules, categories

### 4. **Developer Experience**
- Clearer imports with specific module names
- Better IDE support and autocomplete
- Self-documenting code structure

## Best Practices

### 1. **Import Strategy**
```javascript
// ✅ Good: Import specific functions
const { getValidCategories, normalizeCategory } = require('./core/constants/categories');

// ✅ Good: Import from barrel for multiple modules
const { getValidCategories, getDefaultRuleSet } = require('./core/constants');

// ❌ Avoid: Importing entire modules unnecessarily
const allConstants = require('./core/constants');
```

### 2. **Adding New Constants**
```javascript
// Add to appropriate module (e.g., categories.js)
const NEW_CATEGORY_FEATURE = 'new-feature';

// Export in module
module.exports = {
  NEW_CATEGORY_FEATURE,
  // ... other exports
};

// Update barrel export (index.js) if needed
```

### 3. **Extending Functionality**
```javascript
// Add utility functions to appropriate modules
function getAdvancedCategoryInfo(category) {
  // Implementation
}

// Export with other functions
module.exports = {
  // ... existing exports
  getAdvancedCategoryInfo
};
```

## Testing

Each constants module includes comprehensive tests:

```bash
# Test entire constants structure
node test-constants-structure.js

# Test backward compatibility
node test-centralized-categories.js
```

## Future Enhancements

### Planned Features
1. **Dynamic Configuration Loading** - Load constants from external files
2. **Environment-specific Constants** - Different values for dev/prod
3. **Validation Schemas** - JSON Schema validation for all constants
4. **Hot Reloading** - Update constants without restarting

### Extension Points
- Add new constant modules (e.g., `integrations.js`, `plugins.js`)
- Extend barrel export for new modules
- Add validation functions for new constant types

---

## Quick Reference

| Module | Purpose | Key Functions |
|--------|---------|---------------|
| `categories.js` | Category management | `getValidCategories()`, `normalizeCategory()` |
| `defaults.js` | Default values | `getDefaultConfig()`, `getDefaultRuleSet()` |
| `engines.js` | Engine configuration | `getEnginesForLanguage()`, `getRecommendedEngine()` |
| `rules.js` | Rule utilities | `getLanguageFromRuleId()`, `isValidRuleId()` |
| `index.js` | Barrel export | All functions from all modules |

For detailed API documentation, see the JSDoc comments in each module file.
