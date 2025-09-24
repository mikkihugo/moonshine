# SunLint Deployment Strategies

This document outlines different deployment strategies for SunLint, demonstrating the modular approach that allows selective inclusion/exclusion of features.

## Overview

SunLint supports flexible deployment strategies through its modular architecture:

- **Core Features**: Always included (heuristic regex, ESLint integration)
- **AST Enhancement**: Optional Tree-sitter modules for improved accuracy
- **Language-Specific**: Can include/exclude language-specific parsers
- **AI Features**: Optional OpenAI integration

## Deployment Strategies

### 1. Full Distribution (Development/Enterprise)

**Target**: Development teams, enterprise installations
**Size**: Large (~50MB+ with all AST parsers)
**Accuracy**: Highest

```bash
# Include everything
npm install sunlint

# Available engines: eslint, heuristic (with AST), openai
sunlint --rule=C010 --engine=heuristic src/
# Uses AST when available, falls back to regex
```

**Files Included**:
```
sunlint/
├── engines/
│   ├── eslint-engine.js        ✅ ESLint integration
│   ├── heuristic-engine.js     ✅ Enhanced with AST
│   └── openai-engine.js        ✅ AI-powered analysis
├── core/ast-modules/           ✅ Full AST support
│   ├── parsers/
│   │   ├── javascript-parser.js   ✅ JS AST
│   │   ├── typescript-parser.js   ✅ TS AST  
│   │   ├── dart-parser.js         ✅ Dart AST
│   │   ├── java-parser.js         ✅ Java AST
│   │   └── ...                    ✅ All languages
│   └── package.json               ✅ Tree-sitter deps
└── ...
```

### 2. TypeScript-Only Distribution

**Target**: TypeScript-only projects, Next.js, React
**Size**: Medium (~15MB)
**Accuracy**: High for TS/JS, regex for others

```bash
# Build script for TypeScript distribution
npm run build:typescript

# Available engines: eslint, heuristic (TS/JS AST only)
sunlint --rule=C010 --engine=heuristic src/
# Uses AST for .ts/.js files, regex for others
```

**Build Process**:
```bash
#!/bin/bash
# build-typescript.sh

# Copy base files
cp -r engines/ dist/
cp -r rules/ dist/
cp -r core/ dist/

# Keep only TypeScript/JavaScript AST parsers
mkdir -p dist/core/ast-modules/parsers/
cp core/ast-modules/index.js dist/core/ast-modules/
cp core/ast-modules/base-parser.js dist/core/ast-modules/
cp core/ast-modules/parsers/javascript-parser.js dist/core/ast-modules/parsers/
cp core/ast-modules/parsers/typescript-parser.js dist/core/ast-modules/parsers/

# Update package.json to include only JS/TS Tree-sitter deps
cat > dist/core/ast-modules/package.json << 'EOF'
{
  "name": "@sunlint/ast-typescript",
  "dependencies": {
    "tree-sitter": "^0.20.0",
    "tree-sitter-javascript": "^0.20.0", 
    "tree-sitter-typescript": "^0.20.0"
  }
}
EOF

# Remove OpenAI engine for smaller bundle
rm dist/engines/openai-engine.js

npm pack dist/
```

### 3. Minimal Distribution (Regex-Only)

**Target**: CI/CD, embedded systems, minimal installs
**Size**: Small (~2MB)
**Accuracy**: Basic regex patterns only

```bash
# Build script for minimal distribution
npm run build:minimal

# Available engines: eslint, heuristic (regex-only)
sunlint --rule=C010 --engine=heuristic src/
# Uses only regex patterns, no AST enhancement
```

**Build Process**:
```bash
#!/bin/bash
# build-minimal.sh

# Copy only essential files
cp -r engines/ dist/
cp -r rules/ dist/
cp -r core/ dist/

# Remove entire AST modules directory
rm -rf dist/core/ast-modules/

# Remove OpenAI engine
rm dist/engines/openai-engine.js

# Update heuristic engine to not reference AST modules
sed -i 's/const ASTModuleRegistry = require.*//g' dist/engines/heuristic-engine.js
sed -i 's/this.astRegistry = ASTModuleRegistry;//g' dist/engines/heuristic-engine.js

npm pack dist/
```

### 4. Language-Specific Distributions

**Example: Java-Only Distribution**

```bash
#!/bin/bash
# build-java.sh

# Base files
cp -r engines/ dist/
cp -r rules/ dist/
cp -r core/ dist/

# Keep only Java AST parser
mkdir -p dist/core/ast-modules/parsers/
cp core/ast-modules/index.js dist/core/ast-modules/
cp core/ast-modules/base-parser.js dist/core/ast-modules/
cp core/ast-modules/parsers/java-parser.js dist/core/ast-modules/parsers/

# Java-specific package.json
cat > dist/core/ast-modules/package.json << 'EOF'
{
  "name": "@sunlint/ast-java",
  "dependencies": {
    "tree-sitter": "^0.20.0",
    "tree-sitter-java": "^0.20.0"
  }
}
EOF

# Remove ESLint engine (Java doesn't need it)
rm dist/engines/eslint-engine.js

# Update engine registry
cat > dist/config/engines/engines.json << 'EOF'
{
  "engines": {
    "heuristic": {
      "enabled": true,
      "path": "./engines/heuristic-engine.js",
      "version": "2.0",
      "supportedLanguages": ["java"],
      "priority": 1,
      "description": "Enhanced Java analyzer with AST support"
    }
  },
  "defaultEngines": ["heuristic"],
  "fallbackEngine": "heuristic"
}
EOF

npm pack dist/
```

## Runtime Behavior

### With AST Support Available
```bash
$ sunlint --rule=C010 --engine=heuristic --debug src/
🌳 [AST-Enhanced] Analyzing C010 for typescript with AST support  
🌳 [AST-Enhanced] Found 12 violations via AST, 3 via regex
✅ heuristic: 15 violations found
```

### AST Support Not Available (Fallback)
```bash
$ sunlint --rule=C010 --engine=heuristic --debug src/
⚠️ Tree-sitter TypeScript parser not available, falling back to regex
✅ heuristic: 8 violations found
```

### Minimal Installation
```bash
$ sunlint --rule=C010 --engine=heuristic --debug src/
# No AST warnings - AST modules not included
✅ heuristic: 8 violations found
```

## Package.json Configurations

### Full Distribution
```json
{
  "name": "sunlint",
  "dependencies": {
    "minimatch": "^9.0.0",
    "eslint": "^8.0.0"
  },
  "optionalDependencies": {
    "tree-sitter": "^0.20.0",
    "tree-sitter-javascript": "^0.20.0",
    "tree-sitter-typescript": "^0.20.0",
    "tree-sitter-dart": "^0.1.0",
    "tree-sitter-java": "^0.20.0",
    "tree-sitter-kotlin": "^0.3.0",
    "tree-sitter-swift": "^0.4.0",
    "tree-sitter-python": "^0.20.0",
    "tree-sitter-go": "^0.20.0",
    "tree-sitter-rust": "^0.20.0"
  }
}
```

### TypeScript-Only Distribution
```json
{
  "name": "@sunlint/typescript",
  "dependencies": {
    "minimatch": "^9.0.0",
    "eslint": "^8.0.0",
    "tree-sitter": "^0.20.0",
    "tree-sitter-javascript": "^0.20.0",
    "tree-sitter-typescript": "^0.20.0"
  }
}
```

### Minimal Distribution
```json
{
  "name": "@sunlint/minimal",
  "dependencies": {
    "minimatch": "^9.0.0"
  }
}
```

## Benefits

1. **Flexibility**: Choose the right distribution for your needs
2. **Performance**: Smaller bundles load faster, fewer dependencies to install
3. **Compatibility**: Minimal distribution works everywhere, enhanced versions provide better accuracy
4. **Gradual Adoption**: Start with minimal, upgrade to AST-enhanced when needed
5. **Deployment Options**: Different distributions for different environments (CI vs development)
