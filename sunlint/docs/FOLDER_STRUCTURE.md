# Sunlint Folder Structure

## Rules Directory Naming Convention

All rule folders follow the consistent pattern: `C{ID}_{descriptive_name}`

### Current Rules Structure

```
rules/
├── C006_function_naming/           # Function naming conventions
│   ├── analyzer.js                 # Rule implementation
│   └── config.json                 # Rule configuration
├── C019_log_level_usage/           # Log level usage validation
│   ├── analyzer.js
│   └── config.json
├── C029_catch_block_logging/       # Catch block error logging
│   ├── analyzer.js
│   └── config.json
└── C031_validation_separation/     # Validation logic separation (planned)
    ├── analyzer.js
    └── config.json
```

### Benefits of This Naming Convention

1. **Consistency** - All folders follow the same pattern
2. **Resilience** - If rule IDs change, descriptive names provide context
3. **Readability** - Easy to understand rule purpose from folder name
4. **Maintainability** - Clear organization for developers

### Adding New Rules

When adding a new rule, follow this pattern:

1. Create folder: `C{ID}_{snake_case_description}/`
2. Add `analyzer.js` with rule implementation
3. Add `config.json` with rule configuration
4. Update `rules-registry.json` with correct paths
5. Add tests in `test/fixtures/`

### Example

For a new rule C040 about "API Response Format":
```
rules/C040_api_response_format/
├── analyzer.js
└── config.json
```

Registry entry:
```json
"C040": {
  "name": "API Response Format",
  "description": "Hàm xử lý API nên return response object chuẩn",
  "analyzer": "./rules/C040_api_response_format/analyzer.js",
  "config": "./rules/C040_api_response_format/config.json"
}
```
