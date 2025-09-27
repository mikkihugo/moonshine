# Moon-Shine Comprehensive Test Suite

This directory contains both traditional OXC rule tests and comprehensive AI linting tests for Moon-Shine.

## Traditional Rule Tests

### OXC Integration Tests
Tests validate the full linting pipeline on representative JavaScript/TypeScript code samples, ensuring correct diagnostics (including span/line/column info) for both compliant and violation cases.

#### Test Coverage
- **C002: No Duplicate Code**
  - Positive: Ensures no diagnostics for unique code blocks.
  - Negative: Detects duplicate code, validates diagnostic location/severity.

- **C006: Function Naming**
  - Positive: Accepts compliant function names (verbs/verb-noun).
  - Negative: Flags non-compliant names, checks diagnostic info.

#### Test Files
- [`oxc_rule_template_test.rs`](oxc_rule_template_test.rs): Integration tests for OXC-compatible rules
- [`linter_tests.rs`](linter_tests.rs): Unit and property-based tests for core linter data structures
- [`ai_linting_integration_test.rs`](ai_linting_integration_test.rs): Basic AI linting integration

## AI Linting Test Suite

### Comprehensive AI Testing
The AI linting system includes extensive testing across multiple dimensions:

#### 1. Unit Tests
- **AI Provider Router Tests** (`ai_provider_router_tests.rs`)
  - Provider selection logic and scoring algorithms
  - Rate limiting and concurrency controls
  - Mock AI provider communication
  - Error handling and fallback scenarios

- **AI Behavioral Analysis Tests** (`ai_behavioral_analysis_tests.rs`)
  - Behavioral pattern configuration and validation
  - Heuristic analysis algorithms (cognitive complexity, memory leaks)
  - AI client integration with mocking
  - Pattern detection accuracy and confidence scoring

#### 2. Integration Tests
- **End-to-End AI Linting** (`ai_linting_end_to_end_tests.rs`)
  - Complete AI linting pipeline testing
  - OXC adapter integration with AI behavioral analysis
  - Workflow engine AI linting steps
  - Combined static and AI analysis results

#### 3. Performance & Load Tests
- **Performance Benchmarks** (`ai_performance_benchmarks.rs`)
  - AI vs static analysis performance comparison
  - Provider speed and accuracy benchmarks
  - Memory usage and resource consumption analysis

- **Load & WASM Tests** (`ai_load_and_wasm_tests.rs`)
  - High-volume concurrent AI analysis
  - WASM runtime compatibility validation
  - Resource cleanup verification

#### 4. Advanced Testing
- **Property-Based Tests** (`ai_property_based_tests.rs`)
  - Provider selection determinism
  - Rule consistency validation
  - Configuration serialization preservation

- **Edge Case Tests** (`ai_edge_case_tests.rs`)
  - AI provider failures and network timeouts
  - Malformed input handling
  - Resource exhaustion scenarios

- **Snapshot Tests** (`ai_snapshot_and_fixtures_tests.rs`)
  - AI response format validation with golden files
  - Regression testing for AI analysis outputs
  - Comprehensive test fixture builders

## How to Run

### Traditional Tests
```sh
moon run :test
cargo test --test oxc_rule_template_test
cargo test --test linter_tests
```

### AI Linting Tests
```sh
# All AI tests
cargo test

# Specific categories
cargo test --test ai_provider_router_tests
cargo test --test ai_behavioral_analysis_tests
cargo test --test ai_linting_end_to_end_tests

# Performance benchmarks
cargo bench

# WASM compatibility
cargo test --target wasm32-wasip1
```

### Test Configuration
Environment variables for optimal testing:
- `RUST_TEST_THREADS=4` - Parallel execution
- `RUST_BACKTRACE=1` - Debug backtraces
- `RUST_LOG=debug` - Detailed logging

## Test Quality & Coverage

### Coverage Goals
- **Unit Tests**: >95% code coverage for AI components
- **Integration Tests**: End-to-end workflow validation
- **Property Tests**: Mathematical properties and invariants
- **Performance Tests**: Regression prevention for speed/memory
- **Edge Cases**: Graceful handling of all failure modes

### Quality Metrics
- All tests pass on both native and WASM targets
- Performance benchmarks establish baseline metrics
- Property tests validate mathematical correctness
- Snapshot tests prevent regression in AI output formats
- Mock tests ensure reliability without external dependencies

## Adding New Tests

### Traditional Rule Tests
1. Import the rule's check function and dependencies
2. Add positive and negative code samples
3. Assert on diagnostics, including line/column/severity
4. Document new coverage in this README

### AI Linting Tests
1. Follow existing test organization patterns
2. Use appropriate test fixtures and builders (`TestFixtures::*`)
3. Include both success and failure scenarios
4. Add performance considerations for large-scale tests
5. Ensure WASM compatibility for all new test code
6. Update snapshots when AI output formats change

For detailed information about each test category, see the individual test files and their comprehensive documentation.
