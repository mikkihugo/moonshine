# C033: Separate Service and Repository Logic

## Rule Description

Enforces proper separation between Service and Repository layers in your application architecture:

- **Services** should contain business logic and use repositories for data access
- **Repositories** should contain only data access logic, no business rules

## Architecture

This rule uses a **hybrid analysis approach**:

1. **Primary: Symbol-based Analysis** (using ts-morph)
   - AST parsing and symbol resolution
   - Accurate detection of database operations
   - Excludes queue/job operations automatically

2. **Fallback: Regex-based Analysis**
   - Pattern matching for when symbol analysis fails
   - Handles edge cases and complex code structures

## Files Structure

```
C033_separate_service_repository/
├── analyzer.js              # Main hybrid orchestrator
├── symbol-based-analyzer.js # Primary AST-based analysis
├── regex-based-analyzer.js  # Fallback pattern matching
├── config.json             # Rule configuration
└── README.md               # This documentation
```

## What this rule detects

### Violations in Service files:
- Direct database calls (`repository.createQueryBuilder()`, `dataSource.createQueryBuilder()`)
- Direct ORM operations (`entity.save()`, `entity.find()`)
- SQL queries embedded in service methods
- **Excludes**: Queue/job operations (`job.remove()`, `job.isFailed()`, etc.)

### Violations in Repository files:
- Complex business logic (filtering, calculations, validations)
- Business rules and workflows
- Complex conditional logic for data processing

## Examples

See test cases in the standard test fixtures location:

- **Violations**: `examples/rule-test-fixtures/rules/C033_separate_service_repository/violations/test-cases.js`
- **Clean code**: `examples/rule-test-fixtures/rules/C033_separate_service_repository/clean/good-examples.js`

## Usage

```bash
# Test violations
node cli.js --rule=C033 --input=examples/rule-test-fixtures/rules/C033_separate_service_repository/violations --engine=heuristic

# Test clean code
node cli.js --rule=C033 --input=examples/rule-test-fixtures/rules/C033_separate_service_repository/clean --engine=heuristic
```

## Technical Implementation

- **Primary Analysis**: Semantic analysis using ts-morph for AST traversal
- **Fallback**: Regex pattern matching for environments without ts-morph
- **Engine**: Heuristic engine (registered in enhanced-rules-registry.js)
- **File Detection**: Classifies files as Service/Repository based on naming patterns

## Philosophy

This rule enforces the Repository Pattern and Domain-Driven Design principles:

1. **Separation of Concerns**: Business logic in Services, data access in Repositories
2. **Testability**: Each layer can be tested independently
3. **Maintainability**: Changes to business rules don't affect data access code
4. **Flexibility**: Data storage can be changed without affecting business logic
