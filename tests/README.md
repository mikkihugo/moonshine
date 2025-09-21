# Moon-Shine Integration Test Structure

This directory contains integration tests for OXC-native Rust rules in Moon-Shine.  
Tests validate the full linting pipeline on representative JavaScript/TypeScript code samples, ensuring correct diagnostics (including span/line/column info) for both compliant and violation cases.

## Test Coverage

- **C002: No Duplicate Code**
  - Positive: Ensures no diagnostics for unique code blocks.
  - Negative: Detects duplicate code, validates diagnostic location/severity.

- **C006: Function Naming**
  - Positive: Accepts compliant function names (verbs/verb-noun).
  - Negative: Flags non-compliant names, checks diagnostic info.

## Test Files

- [`oxc_rule_template_test.rs`](oxc_rule_template_test.rs):  
  Integration tests for OXC-compatible rules, including C002 and C006, covering both positive and negative scenarios.

- [`linter_tests.rs`](linter_tests.rs):  
  Unit and property-based tests for core linter data structures and logic.

## How to Run

```sh
moon run :test
```

## Adding New Rule Tests

1. Import the rule's check function and dependencies.
2. Add positive and negative code samples.
3. Assert on diagnostics, including line/column/severity.
4. Document new coverage in this README.
