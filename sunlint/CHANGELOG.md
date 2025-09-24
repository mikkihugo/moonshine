# 🎉 SunLint Changelog

---

## � **v1.3.7 - File Count Reporting & Performance Fixes (September 11, 2025)**

**Release Date**: September 11, 2025  
**Type**: Bug Fix & Enhancement  
**Branch**: `fix.sunlint.report`

### 🐛 **Critical Bug Fixes**
- **FIXED**: File count reporting accuracy in summary
  - **Issue**: Summary showed incorrect file counts when performance filtering applied
  - **Before**: `Files loaded: 1322` but summary `Files: 1000` (misleading)  
  - **After**: Summary accurately reflects files actually analyzed
- **FIXED**: File count multiplication in batch processing
  - **Issue**: Multiple batches incorrectly accumulated file counts  
  - **Before**: 1322 files → reported as 3000 files in batched analysis
  - **After**: Consistent file count regardless of batch strategy

### ⚡ **Performance Enhancements**
- **ENHANCED**: `--max-files=-1` unlimited file processing
  - **Issue**: `-1` flag was ignored, still limited to 1000 files
  - **Solution**: Proper unlimited file processing support
  - **Usage**: `sunlint --max-files=-1` now analyzes all files without limits

### 🎯 **Rule Improvements**
- **ENHANCED**: S057 UTC Logging rule precision (100% accuracy)
  - Fixed false positive detection for `pino.stdTimeFunctions.isoTime`
  - Added timezone indicator support: `'Z'`, `"Z"`, `+00:00`, `.l'Z'`
  - Enhanced config variable tracing for complex logging setups
  - Cleaned up test fixtures and moved to proper location

### 📊 **Validation Results**
- **File Processing**: `--max-files=-1` → 1322 files analyzed ✅
- **Limited Analysis**: `--max-files=500` → 500 files analyzed ✅  
- **Batch Analysis**: Multi-rule analysis maintains accurate counts ✅
- **S057 Precision**: 0 false positives on real projects ✅

---

## �🔧 **v1.3.6 - C067 False Positive Reduction (September 8, 2025)**

**Release Date**: September 8, 2025  
**Type**: Bug Fix & Improvement

### 🐛 **Bug Fixes**
- **FIXED**: C067 "no hardcoded config" rule - Massive false positive reduction
  - **replace-fe**: From 296 → 2 violations (-99.3%)
  - **replace-be**: From 171 → 3 violations (-98.2%) 
  - **jmb-app-be**: From 121 → 5 violations (-95.9%)
  - **mdx-cycle-hack**: From 8 → 6 violations (-25%)

### 🔧 **Technical Improvements**
- **ENHANCED**: C067 analyzer logic improvements
  - Skip dummy/test files and entity files completely
  - Exclude field mapping objects and ORM configurations
  - Skip database constraint names (primaryKeyConstraintName, etc.)
  - Focus only on truly environment-dependent configurations
  - Exclude business logic constants and UI field mappings
- **IMPROVED**: Rule precision - Only flag real environment config issues
  - API endpoints, AWS service URLs, application keys
  - Credential values and connection strings
  - Environment-dependent timeouts and ports

### 📊 **Performance**
- **OPTIMIZED**: Reduced analysis noise by 95%+ on large projects
- **ENHANCED**: Better developer experience with fewer false alarms

---

## 🔧 **v1.3.5 - Preset System Refactor (September 8, 2025)**

**Release Date**: September 8, 2025  
**Type**: Feature Enhancement

### ✨ **New Features**
- **ENHANCED**: Complete preset system overhaul
  - **Data-driven presets**: All presets now generated from actual rule sources
  - **Accurate rule counting**: Presets contain only activated rules with tool support
  - **New preset categories**: Added beginner, ci, strict, maintainability, performance presets
  - **Comprehensive "all" preset**: 88 activated rules from common and security files

### 🔧 **Technical Improvements**
- **ADDED**: Automated preset generation scripts
  - `scripts/generate-presets.js` - Generate all preset configurations
  - `scripts/analyze-core-rules.js` - Analyze rules from markdown sources
- **UPDATED**: ConfigPresetResolver now supports all 9 presets
- **ORGANIZED**: Scripts directory with clear categorization and documentation
- **FIXED**: Preset-to-rule mapping accuracy

### 🎯 **Preset System**
- **9 total presets**: all, recommended, security, quality, beginner, ci, strict, maintainability, performance
- **Focus**: Only common-en.md and security-en.md rules (no language-specific rules)
- **Validation**: Tested with real demo project showing 135 violations detected

### 📦 **Upgrade Notes**
- **Zero breaking changes** - all existing configurations work
- **New presets available** - can now use @sun/sunlint/all and other new presets
- **Improved accuracy** - presets now contain only rules that actually work

---

## � **v1.3.4 - Engine Auto Hotfix (September 5, 2025)**

**Release Date**: September 5, 2025  
**Type**: Critical Hotfix

### 🚨 **Critical Bug Fix**
- **FIXED**: Engine "auto" validation and selection logic
  - **Issue**: `--engine=auto` causing "Invalid engine: auto" error in v1.3.3
  - **Root Cause**: Missing auto engine support in validation and orchestrator
  - **Solution**: Comprehensive auto engine implementation
    - Added "auto" case to engine factory with heuristic fallback
    - Updated CLI validation to include "auto" in valid engines
    - Enhanced orchestrator to resolve "auto" to actual engines (heuristic + eslint)
    - Fixed CLI action handler auto-detection logic

### 🧪 **Validation Results**
- **✅ Auto engine**: Works correctly (auto-selects heuristic + eslint)
- **✅ Heuristic engine**: Unchanged, working properly
- **✅ ESLint engine**: Unchanged, working properly
- **✅ CLI help**: Shows all engines including auto option

### 📦 **Upgrade Notes**
- **Zero breaking changes** - seamless upgrade from v1.3.3
- **Default `--engine=auto`** now works as intended
- **All existing commands** continue to work unchanged

---

## �🚀 **v1.3.3 - Performance & File Limits Optimization (September 4, 2025)**

**Release Date**: September 4, 2025  
**Type**: Performance Enhancement & User Experience

### ⚡ **Performance Engineering**
- **ENHANCED**: Heuristic Engine v4.0 with integrated performance optimizations
  - **Smart file limits**: Auto-detection prevents memory issues
  - **Batch processing**: Optimized rule execution for large projects
  - **Memory management**: Symbol table limits for TypeScript projects
  - **Timeout protection**: Graceful handling of long-running analysis

### 🎛️ **CLI Enhancement & Clarity**
- **CLARIFIED**: File limit options with comprehensive documentation
  - **`--max-files`**: Controls total analysis workload (performance)
  - **`--max-semantic-files`**: Controls TypeScript symbol table memory
  - **Auto-detection**: Smart defaults for 90% of use cases
  - **Manual tuning**: Fine control for enterprise projects

### � **Bug Fixes**
- **FIXED**: Engine "auto" validation and selection logic
  - **Engine Factory**: Added "auto" case with fallback to heuristic engine
  - **CLI Validation**: Added "auto" to valid engines list
  - **Orchestrator**: Auto-resolve "auto" to actual engines (heuristic + eslint)
  - **Engine Selection**: Auto-detection works correctly for rule preferences

### �📚 **Documentation Expansion**
- **NEW**: [FILE_LIMITS_EXPLANATION.md](./docs/FILE_LIMITS_EXPLANATION.md) - Comprehensive guide (5.7KB)
- **NEW**: [QUICK_FILE_LIMITS.md](./docs/QUICK_FILE_LIMITS.md) - Quick reference (1.8KB)
- **ENHANCED**: CLI help with clear usage examples
- **INTEGRATED**: Performance docs in README.md

### 🧠 **Architecture Improvements**
- **INTEGRATED**: Performance logic into heuristic engine (no separate files)
- **ENHANCED**: Auto-performance-manager for intelligent limit calculation
- **OPTIMIZED**: Memory usage patterns for large codebases
- **TESTED**: GitHub Actions compatibility with resource constraints

### 🎯 **User Experience**
- **90/10 Rule**: Auto-detection works for most cases, manual tuning available
- **Progressive disclosure**: Quick ref → detailed guide → implementation details
- **CI/CD Ready**: Optimized for memory-constrained environments

---

## 🏆 **v1.3.2 - Precision Engineering & Rule Maturity (August 21, 2025)**

**Release Date**: August 21, 2025  
**Type**: Precision Enhancement & Architecture-Aware Analysis

### 🎯 **Precision Engineering Achievements**
- **BREAKTHROUGH**: Rule **C019** - Log Level Usage 
  - **97.5% false positive reduction** across real projects (315+ → 8 violations)
  - **Architecture-aware detection**: Frontend/backend, client/server, test exclusions
  - **Framework-aware patterns**: NestJS DI, Redux slices, ORM operations  
  - **Context-aware analysis**: Centralized logging, error handling, internal vs external calls
  - **Production-ready precision**: Only high-value violations remain

### 🔧 **Rules Enhanced with Production-Grade Precision**
- **ENHANCED**: Rule **C002** - Code Organization & Structure
- **ENHANCED**: Rule **C003** - Function Complexity Management  
- **ENHANCED**: Rule **C006** - Error Handling Patterns
- **ENHANCED**: Rule **C010** - Performance Optimization
- **ENHANCED**: Rule **C012** - Security Best Practices
- **ENHANCED**: Rule **C014** - API Design Standards

### 🌟 **New Rules Portfolio**
- **NEW**: Rule **S005** - Security Vulnerability Detection
- **NEW**: Rule **S006** - Authentication & Authorization Patterns
- **NEW**: Rule **S007** - Data Protection & Privacy
- **NEW**: Rule **S009** - Input Validation & Sanitization
- **NEW**: Rule **S010** - Cryptographic Implementation
- **NEW**: Rule **S016** - Secure Communication Protocols
- **NEW**: Rule **C018** - Code Documentation Standards
- **NEW**: Rule **C023** - Database Query Optimization
- **NEW**: Rule **C024** - Memory Management Patterns

### 🏗️ **Architecture & Detection Improvements**
- **Smart exclusion patterns**: Config services, local libraries, internal dependencies
- **Centralized logging detection**: Redux error handling, API interceptors, global handlers
- **Duplicate log intelligence**: Different functions, error handling contexts
- **Business logic awareness**: Higher thresholds for complex functions
- **Framework-specific patterns**: NestJS, React, Redux, ORM recognition

### 📊 **Precision Metrics**
- **External service calls**: 99.7% false positive elimination
- **Payment transactions**: Redux slice exclusion, actual processing detection
- **Duplicate logs**: Context-aware, cross-function intelligent filtering
- **Log levels**: Architecture-aware suggestions and enforcement

---

## 🚀 **v1.3.1 - Advanced Rules & Performance Optimization (August 18, 2025)**

**Release Date**: August 18, 2025  
**Type**: Feature Enhancement & Performance Optimization

### 🎯 **New Rules Added**
- **NEW**: Rule **C076** - Explicit Function Argument Types (Semantic-only)
  - Enforces explicit type annotations on all public function parameters
  - Detects `any`, `unknown`, and missing type annotations
  - Semantic-only analysis (no regex fallback) for maximum accuracy
  - Config-driven with customizable allowed/disallowed types

### 🔧 **Rules Enhanced**
- **ENHANCED**: Rule **C033** - Separate Service and Repository Logic
  - Improved symbol-based analysis with regex fallback
  - Better business logic pattern detection
  - Enhanced service/repository boundary enforcement

- **ENHANCED**: Rule **C035** - Error Logging Context  
  - Advanced semantic analysis for error handling patterns
  - Better context detection in catch blocks
  - Improved logging recommendation accuracy

- **ENHANCED**: Rule **C040** - Centralized Validation
  - Symbol-based validation pattern detection
  - Enhanced inline validation detection
  - Better configuration options

- **ENHANCED**: Rule **C017** - Consistent Error Response Format
  - Improved semantic analysis capabilities
  - Better error response format detection
  - Enhanced cross-file analysis

### 🎯 **Semantic Rules Added**
- **NEW**: Rule **S005** - AST-based analysis capabilities
- **NEW**: Rule **S006** - Advanced regex pattern matching  
- **NEW**: Rule **S007** - Semantic analysis with symbol resolution

### ⚡ **Performance Improvements**
- **OPTIMIZED**: Lazy initialization for semantic rules
  - Rules only initialize when actually needed
  - Reduced startup time and memory usage
  - Eliminated unnecessary rule initialization logs

- **IMPROVED**: Semantic engine memory optimization
  - Better handling of large projects (1000+ files)
  - Optimized ts-morph project loading
  - Enhanced file targeting for semantic analysis

### 🐛 **Bug Fixes**
- **FIXED**: Rule ID confusion between C072 and C076
- **FIXED**: Verbose logging only shows when `--verbose` flag is used
- **FIXED**: Semantic rules initialization spam in logs
- **FIXED**: File ignore patterns for ESLint integration rules

### 📦 **Packaging Improvements**
- **UPDATED**: .npmignore to preserve important ESLint rule implementations
- **IMPROVED**: Package size optimization while maintaining functionality
- **ENHANCED**: Build process to include all necessary rule files

### 🔄 **Backward Compatibility**
- **MAINTAINED**: Full backward compatibility with existing configurations
- **PRESERVED**: All existing rule IDs and behavior
- **ENSURED**: ESLint integration continues to work seamlessly

---

## 🌟 **v1.3.0 - Enhanced Engine Architecture (August 13, 2025)**

**Release Date**: August 13, 2025  
**Type**: Major Engine Enhancement & Rule Mapping Improvements

### 🏗️ **Engine Architecture Enhancements**

#### **Strict Engine Mode vs Fallback Mode**
- **NEW**: `--engine=eslint` strict mode - only runs specified engine, skips unsupported rules
- **NEW**: Auto fallback mode when no engine specified (ESLint → Heuristic → OpenAI)
- **IMPROVED**: Enhanced orchestrator with requestedEngine support
- **FIXED**: TypeScript ESLint rules requiring type information removed from mapping

#### **ESLint Integration Improvements**
- **FIXED**: Removed type-dependent rules: `@typescript-eslint/strict-boolean-expressions`, `@typescript-eslint/no-floating-promises`, `@typescript-eslint/prefer-readonly`
- **UPDATED**: ESLint rule mapping cleanup for better stability
- **ENHANCED**: Graceful handling of missing ESLint plugins
- **ADDED**: Support for `eslint-plugin-import` in dependencies documentation

#### **Rule System Enhancements**
- **IMPROVED**: Rule skip logic with detailed reporting
- **ENHANCED**: Engine-specific rule filtering and compatibility
- **FIXED**: ESLint engine stability issues with TypeScript projects
- **UPDATED**: Documentation to reflect current architecture

### 📚 **Documentation Updates**
- **UPDATED**: README.md with complete dependency information
- **UPDATED**: CONTRIBUTING.md to match current architecture
- **REMOVED**: Outdated documentation files (REFACTOR_PLAN.md, RULE_MIGRATION_SUMMARY.md, etc.)
- **ENHANCED**: Clear setup instructions for TypeScript projects

### 🧹 **Cleanup & Maintenance**
- **REMOVED**: Deprecated documentation files
- **REMOVED**: Temporary test files and cache files
- **UPDATED**: Package version to 1.3.0
- **IMPROVED**: File structure organization

---

## 🔥 **v1.2.0 - Architecture Refactor (July 30, 2025)**

**Release Date**: July 30, 2025  
**Type**: Major Architecture Update (Adapter Pattern Implementation)

### 🏗️ **Major Architecture Changes**

#### **Unified Adapter Pattern**
- **NEW**: `SunlintRuleAdapter` - Unified rule access layer for CLI
- **IMPROVED**: Same adapter pattern as VSCode extension (`RuleReaderService`)
- **ELIMINATED**: Direct parser/registry access across core modules
- **PERFORMANCE**: 0.07ms average per rule query with singleton caching

#### **Refactored Core Modules**
- **UPDATED**: `core/rule-selection-service.js` - Now uses adapter exclusively
- **UPDATED**: `core/config-manager.js` - Adapter-driven config validation
- **UPDATED**: `core/analysis-orchestrator.js` - Unified rule initialization
- **UPDATED**: `engines/heuristic-engine.js` - Adapter-based rule access

#### **Enhanced Rule Management**
- **IMPROVED**: 256 rules loaded from registry with fallback to origin-rules
- **ADDED**: AI context generation via `generateAIContext()` method
- **ENHANCED**: Engine compatibility checking (heuristic: 244, eslint: 17, ai: 256)
- **OPTIMIZED**: Memory usage with singleton pattern

### 🎯 **Benefits**
- **No Rule Model Duplication**: Single source of truth across CLI and VSCode
- **Extensible Architecture**: Easy to add new engines or rule sources
- **Consistent OpenAI Integration**: Proper context extraction from origin-rules
- **Maintainable Codebase**: Centralized rule logic through adapter layer

### 📊 **Performance Metrics**
- **Rule Loading**: 256 rules in ~10ms
- **Query Performance**: 0.07ms average per `getAllRules()` call
- **Engine Coverage**: Heuristic (95.3%), ESLint (6.6%), AI (100%)
- **Memory Efficiency**: Singleton prevents duplicate instances

### 🧪 **Testing & Validation**
- **ADDED**: `test-adapter.js` - Comprehensive adapter testing
- **UPDATED**: Integration tests now use adapter methods
- **VERIFIED**: All 3/3 integration tests pass
- **VALIDATED**: Engine orchestration and rule compatibility

---

## 🎉 **v1.1.8 Release Notes**

**Release Date**: July 24, 2025  
**Type**: Minor Release (ESLint 9.x Compatibility & Enhanced Error Handling)

---

## 🚀 **Key Improvements**

### 🔧 **ESLint 9.x Full Compatibility**
- **Fixed**: `context.getSource is not a function` error with React Hooks plugin
- **Enhanced**: Robust plugin compatibility detection and fallback mechanisms
- **Improved**: Legacy config to flat config conversion for ESLint 9.x projects
- **Added**: Graceful degradation when plugins fail to load

### 🛡️ **Enhanced Error Handling**
- **Smart**: Plugin version detection with upgrade guidance
- **Robust**: Fallback to minimal ESLint configuration when plugins fail
- **Clear**: Detailed error messages for troubleshooting plugin issues
- **Stable**: Continue analysis even with incompatible plugins

### ✅ **Real-World Validation**
- **Tested**: Successfully validated on 3 production projects (NestJS, Next.js)
- **Verified**: 820+ files analyzed without crashes
- **Proven**: Handles ESLint 8.x, 9.x, and mixed configurations

### 🎯 **Plugin Compatibility**
- **React Hooks**: Fixed compatibility issues with outdated versions
- **TypeScript ESLint**: Enhanced support for v5.x and v8.x
- **Security Plugins**: Graceful handling of missing security rules
- **Custom Plugins**: Better error recovery for third-party plugins

---

# 🎉 SunLint v1.1.7 Release Notes

**Release Date**: July 24, 2025  
**Type**: Minor Release (ESLint Engine Enhancement & Smart Installation Guidance)

---

## 🚀 **Key Improvements**

### 🧠 **ESLint Engine Enhancement**
- **Enhanced**: ESLint v9+ flat config support with automatic legacy config conversion
- **Improved**: Dynamic plugin loading with availability detection (React, TypeScript, React Hooks)
- **Robust**: Better error handling and parsing error filtering for TypeScript files
- **Smart**: Temporary flat config generation for legacy compatibility

### 🎯 **Smart Installation Guidance**
- **Intelligent**: Project type detection (NestJS, React, Next.js, Node.js)
- **Targeted**: Package manager detection (npm, yarn, pnpm) from package.json
- **Conditional**: Smart `--legacy-peer-deps` suggestion only when dependency conflicts detected
- **Clear**: Descriptive project-specific installation instructions

### 🔧 **Project Type Detection**
- **NestJS Projects**: `pnpm install --save-dev @typescript-eslint/parser @typescript-eslint/eslint-plugin`
- **React Projects**: `npm install --save-dev @typescript-eslint/parser @typescript-eslint/eslint-plugin eslint-plugin-react eslint-plugin-react-hooks`
- **Conflict Detection**: Automatic detection of date-fns, React version conflicts, ESLint v8 issues

### 📦 **Dependency Management**
- **Aggregated Warnings**: Consolidated messages for missing plugins instead of spam
- **Graceful Fallback**: Analysis continues even with missing plugins, filtering parsing errors
- **Cleanup**: Automatic temporary config file cleanup after analysis

---

## 🛠 **Technical Details**

### **ESLint Integration**
- **Config Detection**: Automatic detection of flat config vs legacy config
- **Plugin Availability**: Runtime detection of React, TypeScript, React Hooks plugins
- **Parser Support**: Conditional TypeScript parser loading based on availability
- **Rule Filtering**: Skip rules for unavailable plugins with clear messaging

### **Smart Guidance Logic**
- **Package Manager**: Detects preferred package manager from scripts and preinstall hooks
- **Conflict Detection**: Analyzes package.json for known dependency conflicts
- **Project Classification**: Distinguishes between frontend (React/Next.js) and backend (NestJS/Node.js) projects

---

## 📋 **Usage Examples**

### **Minimal Installation (Works for basic analysis)**
```bash
npm install --save-dev @sun-asterisk/sunlint
```

### **TypeScript Projects (Recommended)**
```bash
npm install --save-dev @sun-asterisk/sunlint typescript
```

### **Full Installation (All project types)**
```bash
npm install --save-dev @sun-asterisk/sunlint eslint @typescript-eslint/parser @typescript-eslint/eslint-plugin eslint-plugin-react eslint-plugin-react-hooks typescript
```

---

## 🎉 **What's Next**

SunLint v1.1.7 makes ESLint integration more robust and user-friendly with intelligent project detection and clear installation guidance. No more guessing what dependencies to install! 🚀

---

# 🎉 SunLint v1.1.0 Release Notes

**Release Date**: July 23, 2025  
**Type**: Minor Release (AST Enhancement & CLI Options Fix)

---

## 🚀 **Key Improvements**

### 🧠 **AST-Enhanced Analysis**
- **Enhanced**: Heuristic engine now supports AST-based analysis using ESLint's parser infrastructure
- **Improved**: Rule C010 (block nesting) now uses AST for accurate detection
- **Modular**: AST modules integrated with silent fallback to regex when parsing fails
- **Performance**: ESLint-based parsers (@babel/parser, @typescript-eslint/parser) for JS/TS analysis

### 🎯 **CLI Options Fix**
- **Fixed**: `--quality` option now correctly selects quality rules (30 rules)
- **Fixed**: `--security` option now correctly selects security rules (41 rules)
- **Enhanced**: Rule selection service properly filters by category
- **Validated**: Both options tested and working correctly

### 📦 **Package Optimization**
- **Reduced**: Package size from 8MB to 243KB by excluding nested node_modules
- **Clean**: Updated .npmignore to exclude development files
- **Dependencies**: Moved AST parser dependencies to root package.json

---

## 📋 **Previous Changes (v1.0.7)**

### 🔧 **Configuration Cleanup**

---

## 🚀 **Key Improvements**

### 🔧 **Configuration Cleanup** 
- **BREAKING**: Deprecated `ignorePatterns` in favor of `exclude` for better consistency
- **Auto-migration**: Existing configs with `ignorePatterns` will auto-migrate with deprecation warning
- **Unified logic**: Removed duplicate pattern processing for better performance

### 🎯 **File Targeting Fixes**
- **Fixed**: Specific file input (`--input=file.js`) now works correctly with config patterns
- **Enhanced**: Better include/exclude pattern resolution for both CLI and config
- **Improved**: Default include patterns for JavaScript/TypeScript files

### 🛡️ **Security Rules Enhancement**
- **Verified**: All security rules (S001, S002, S007, S013, etc.) working correctly
- **Tested**: Comprehensive rule detection across TypeScript and JavaScript files
- **Stable**: 20,000+ violation detection capability validated

---

## 📋 **Changes in Detail**

### ✅ **Configuration Changes**
- **Deprecated**: `ignorePatterns` → Use `exclude` instead
- **New**: Default include patterns: `["**/*.js", "**/*.ts", "**/*.jsx", "**/*.tsx"]`
- **Migration**: Automatic conversion with warning for backward compatibility

**Before (Deprecated):**
```json
{
  "ignorePatterns": ["node_modules/**", "dist/**"]
}
```

**After (Recommended):**
```json
{
  "include": ["**/*.js", "**/*.ts", "**/*.jsx", "**/*.tsx"],
  "exclude": ["node_modules/**", "dist/**"]
}
```

### 🐛 **Bug Fixes**
- Fixed file targeting when using specific file input (`--input=cli.js`)
- Resolved circular symlink issues in `node_modules` traversal
- Eliminated duplicate ignore pattern processing

### 🏗️ **Internal Improvements**
- Cleaner file targeting service logic
- Better config merger with deprecation warnings
- Updated preset configurations to use `exclude`

---

## 📦 **Updated Files**

### **Core Components**
- `core/file-targeting-service.js` - Simplified pattern processing
- `core/config-merger.js` - Added deprecation handling
- `core/config-manager.js` - Updated default config structure

### **Configuration**
- `config/presets/*.json` - Updated all presets to use `exclude`
- `config/sunlint-schema.json` - Removed deprecated `ignorePatterns`
- `.sunlint.json` - Updated with include patterns

### **Documentation**
- `README.md` - Added breaking change notice and migration guide

---

## 🧪 **Validation Results**

✅ **Global Installation**: `npm install -g @sun-asterisk/sunlint`  
✅ **Project Installation**: `npm install --save-dev @sun-asterisk/sunlint`  
✅ **CLI Commands**: All CLI options tested and working  
✅ **Rule Detection**: 20,263 violations detected across 4,272 files  
✅ **Performance**: 17s analysis time for large codebase  

---

## 🔄 **Migration Guide**

### **For Existing Users**
1. **Update your `.sunlint.json`:**
   ```bash
   # Replace ignorePatterns with exclude
   sed -i 's/ignorePatterns/exclude/g' .sunlint.json
   ```

2. **Add include patterns (recommended):**
   ```json
   {
     "include": ["**/*.js", "**/*.ts", "**/*.jsx", "**/*.tsx"],
     "exclude": ["node_modules/**", "dist/**", "**/*.min.*"]
   }
   ```

3. **Test your configuration:**
   ```bash
   sunlint --dry-run --verbose
   ```

### **No Action Required**
- Existing configs with `ignorePatterns` will continue to work
- Automatic migration with deprecation warning
- Remove deprecated properties when convenient

---

## 📈 **Statistics**

| Metric | Value |
|--------|-------|
| **Rules Available** | 97+ (Security + Quality) |
| **File Processing** | 4,272 files analyzed |
| **Violation Detection** | 20,263 issues found |
| **Performance** | ~17 seconds for full analysis |
| **Languages Supported** | TypeScript, JavaScript, Dart |

---

## 🎯 **Next Steps**

- **v1.0.8**: Enhanced TypeScript analysis engine
- **v1.1.0**: Dart language support expansion
- **v1.2.0**: Custom rule authoring framework

---

## 💫 **Acknowledgments**

Thanks to the Sun* Engineering team for continuous feedback and testing. Special recognition for helping identify and resolve the file targeting issues.

**Happy Linting!** ☀️
