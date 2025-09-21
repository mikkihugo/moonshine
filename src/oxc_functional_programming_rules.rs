//! # Functional Programming Rules
//!
//! Comprehensive rules for functional programming patterns, immutability,
//! higher-order functions, and pure functional paradigms in JavaScript/TypeScript.
//!
//! ## Rule Categories:
//! - **Immutability**: Avoiding mutation, immutable data structures, pure functions
//! - **Higher-Order Functions**: Function composition, currying, partial application
//! - **Monadic Patterns**: Maybe/Option types, Either types, error handling
//! - **Functional Composition**: Pipe operators, function chaining, point-free style
//! - **Side Effect Management**: IO isolation, effect systems, referential transparency
//!
//! ## Examples:
//! ```javascript
//! // ✅ Good: Pure function with immutable operations
//! const addToList = (list, item) => [...list, item];
//! const double = x => x * 2;
//! const processNumbers = pipe(
//!   map(double),
//!   filter(x => x > 10),
//!   reduce((sum, x) => sum + x, 0)
//! );
//!
//! // ❌ Bad: Mutating function with side effects
//! function addToList(list, item) {
//!   list.push(item); // Mutation
//!   console.log('Added item'); // Side effect
//!   return list;
//! }
//! ```

use serde::{Deserialize, Serialize};

use crate::unified_rule_registry::{RuleSettings, WasmRuleCategory, WasmFixStatus};

/// Trait for basic WASM-compatible rule implementation
pub trait WasmRule {
    const NAME: &'static str;
    const CATEGORY: WasmRuleCategory;
    const FIX_STATUS: WasmFixStatus;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic>;
}

/// Enhanced trait for AI-powered rule suggestions
pub trait EnhancedWasmRule: WasmRule {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmRuleDiagnostic {
    pub rule_name: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub severity: String,
    pub fix_suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSuggestion {
    pub suggestion_type: String,
    pub confidence: f64,
    pub description: String,
    pub code_example: Option<String>,
}

/// Rule: require-pure-functions
/// Enforces pure function patterns without side effects or mutations
#[derive(Clone)]
pub struct RequirePureFunctions;

impl RequirePureFunctions {
    pub const NAME: &'static str = "require-pure-functions";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequirePureFunctions {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for array mutations
        if code.contains("push(") || code.contains("pop(") || code.contains("splice(") {
            diagnostics.push(create_pure_functions_diagnostic(
                0, 0,
                "Use immutable array operations instead of mutating methods like push, pop, splice"
            ));
        }

        // Check for object mutations
        if code.contains("delete ") && !code.contains("// Intentional mutation") {
            diagnostics.push(create_pure_functions_diagnostic(
                0, 0,
                "Avoid delete operator - use object destructuring or filtering instead"
            ));
        }

        // Check for side effects in functions
        if code.contains("console.log") && code.contains("function") && !code.contains("debug") {
            diagnostics.push(create_pure_functions_diagnostic(
                0, 0,
                "Functions should avoid side effects like console.log - consider using debug functions or effect management"
            ));
        }

        // Check for direct DOM manipulation in business logic
        if code.contains("document.") && !code.contains("effect") && !code.contains("IO") {
            diagnostics.push(create_pure_functions_diagnostic(
                0, 0,
                "DOM manipulation should be isolated from business logic using effect systems"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequirePureFunctions {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.91,
            suggestion: "Create pure functions: Replace `arr.push(item)` with `[...arr, item]`, use `Object.assign({}, obj, updates)` instead of mutation".to_string(),
            fix_code: Some("// Pure function patterns\nconst addItem = (list, item) => [...list, item];\nconst removeItem = (list, index) => list.filter((_, i) => i !== index);\nconst updateObject = (obj, updates) => ({ ...obj, ...updates });\n\n// Effect management\nconst createLogger = () => (message) => console.log(message);\nconst processWithEffects = (data, logger) => {\n  const result = pureTransform(data);\n  logger(`Processed ${data.length} items`);\n  return result;\n};".to_string()),
        }).collect()
    }
}

/// Rule: require-immutable-data-operations
/// Enforces immutable data operations and prevents mutation
#[derive(Clone)]
pub struct RequireImmutableDataOperations;

impl RequireImmutableDataOperations {
    pub const NAME: &'static str = "require-immutable-data-operations";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireImmutableDataOperations {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for direct property assignment
        if code.contains(".property =") && !code.contains("readonly") {
            diagnostics.push(create_immutable_data_operations_diagnostic(
                0, 0,
                "Use immutable update patterns instead of direct property assignment"
            ));
        }

        // Check for Array.sort() without copying
        if code.contains(".sort(") && !code.contains("[...") && !code.contains("slice()") {
            diagnostics.push(create_immutable_data_operations_diagnostic(
                0, 0,
                "Array.sort() mutates the original array - create a copy first"
            ));
        }

        // Check for Array.reverse() without copying
        if code.contains(".reverse(") && !code.contains("slice()") {
            diagnostics.push(create_immutable_data_operations_diagnostic(
                0, 0,
                "Array.reverse() mutates the original array - use immutable alternatives"
            ));
        }

        // Check for missing const assertions
        if code.contains("const") && code.contains("=") && code.contains("[") && !code.contains("as const") {
            diagnostics.push(create_immutable_data_operations_diagnostic(
                0, 0,
                "Consider using 'as const' assertion for immutable array/object literals"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireImmutableDataOperations {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.93,
            suggestion: "Use immutable operations: `const sorted = [...array].sort()`, `const updated = { ...obj, property: newValue }`, `const config = ['a', 'b'] as const`".to_string(),
            fix_code: Some("// Immutable array operations\nconst sortedArray = [...originalArray].sort();\nconst reversedArray = originalArray.slice().reverse();\nconst filteredAndSorted = originalArray.filter(predicate).sort();\n\n// Immutable object updates\nconst updateProperty = (obj, key, value) => ({ ...obj, [key]: value });\nconst updateNested = (obj, path, value) => ({\n  ...obj,\n  [path[0]]: path.length === 1 \n    ? value \n    : updateNested(obj[path[0]], path.slice(1), value)\n});\n\n// Const assertions for type safety\nconst STATUSES = ['pending', 'approved', 'rejected'] as const;\nconst CONFIG = {\n  api: { timeout: 5000 },\n  features: { beta: true }\n} as const;".to_string()),
        }).collect()
    }
}

/// Rule: require-higher-order-functions
/// Enforces higher-order function patterns for code reusability
#[derive(Clone)]
pub struct RequireHigherOrderFunctions;

impl RequireHigherOrderFunctions {
    pub const NAME: &'static str = "require-higher-order-functions";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireHigherOrderFunctions {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for repetitive for loops that could use map/filter/reduce
        if code.contains("for (") && code.contains("push(") && !code.contains("map(") {
            diagnostics.push(create_higher_order_functions_diagnostic(
                0, 0,
                "Consider using map(), filter(), or reduce() instead of imperative for loops"
            ));
        }

        // Check for callback patterns without currying
        if code.contains("callback") && code.contains("function") && !code.contains("curry") {
            diagnostics.push(create_higher_order_functions_diagnostic(
                0, 0,
                "Consider using currying or partial application for reusable callback patterns"
            ));
        }

        // Check for missing function composition
        if code.contains("const result =") && code.contains("function1(function2(") && !code.contains("pipe") && !code.contains("compose") {
            diagnostics.push(create_higher_order_functions_diagnostic(
                0, 0,
                "Consider using function composition (pipe/compose) for cleaner function chaining"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireHigherOrderFunctions {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.88,
            suggestion: "Use higher-order functions: `const processItems = pipe(filter(isValid), map(transform), reduce(accumulate, []))`".to_string(),
            fix_code: Some("// Higher-order function utilities\nconst curry = (fn) => (...args) => \n  args.length >= fn.length \n    ? fn(...args)\n    : (...nextArgs) => curry(fn)(...args, ...nextArgs);\n\nconst pipe = (...fns) => (value) => fns.reduce((acc, fn) => fn(acc), value);\nconst compose = (...fns) => (value) => fns.reduceRight((acc, fn) => fn(acc), value);\n\n// Curried utility functions\nconst map = curry((fn, array) => array.map(fn));\nconst filter = curry((predicate, array) => array.filter(predicate));\nconst reduce = curry((reducer, initial, array) => array.reduce(reducer, initial));\n\n// Example usage\nconst processNumbers = pipe(\n  filter(x => x > 0),\n  map(x => x * 2),\n  reduce((sum, x) => sum + x, 0)\n);\n\n// Partial application\nconst multiply = curry((a, b) => a * b);\nconst double = multiply(2);\nconst triple = multiply(3);".to_string()),
        }).collect()
    }
}

/// Rule: require-monadic-error-handling
/// Enforces monadic patterns for error handling (Maybe, Either types)
#[derive(Clone)]
pub struct RequireMonadicErrorHandling;

impl RequireMonadicErrorHandling {
    pub const NAME: &'static str = "require-monadic-error-handling";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireMonadicErrorHandling {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for null/undefined checks without Maybe types
        if code.contains("!== null") && code.contains("!== undefined") && !code.contains("Maybe") && !code.contains("Option") {
            diagnostics.push(create_monadic_error_handling_diagnostic(
                0, 0,
                "Consider using Maybe/Option types for null safety instead of explicit null checks"
            ));
        }

        // Check for try-catch without Either types
        if code.contains("try {") && code.contains("catch") && !code.contains("Either") && !code.contains("Result") {
            diagnostics.push(create_monadic_error_handling_diagnostic(
                0, 0,
                "Consider using Either/Result types for functional error handling"
            ));
        }

        // Check for nested conditionals that could use flatMap
        if code.contains("if (") && code.contains("if (") && !code.contains("flatMap") && !code.contains("chain") {
            diagnostics.push(create_monadic_error_handling_diagnostic(
                0, 0,
                "Nested conditionals could benefit from monadic chaining with flatMap/chain"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireMonadicErrorHandling {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.89,
            suggestion: "Implement monadic error handling: `const result = Maybe.of(data).map(transform).flatMap(validate).getOrElse(defaultValue)`".to_string(),
            fix_code: Some("// Maybe/Option monad for null safety\nclass Maybe {\n  constructor(value) {\n    this.value = value;\n  }\n  \n  static of(value) {\n    return new Maybe(value);\n  }\n  \n  static none() {\n    return new Maybe(null);\n  }\n  \n  isNone() {\n    return this.value === null || this.value === undefined;\n  }\n  \n  map(fn) {\n    return this.isNone() ? Maybe.none() : Maybe.of(fn(this.value));\n  }\n  \n  flatMap(fn) {\n    return this.isNone() ? Maybe.none() : fn(this.value);\n  }\n  \n  getOrElse(defaultValue) {\n    return this.isNone() ? defaultValue : this.value;\n  }\n}\n\n// Either monad for error handling\nclass Either {\n  constructor(value, isRight = true) {\n    this.value = value;\n    this.isRight = isRight;\n  }\n  \n  static right(value) {\n    return new Either(value, true);\n  }\n  \n  static left(error) {\n    return new Either(error, false);\n  }\n  \n  map(fn) {\n    return this.isRight ? Either.right(fn(this.value)) : this;\n  }\n  \n  flatMap(fn) {\n    return this.isRight ? fn(this.value) : this;\n  }\n  \n  fold(leftFn, rightFn) {\n    return this.isRight ? rightFn(this.value) : leftFn(this.value);\n  }\n}\n\n// Example usage\nconst safeParseInt = (str) => {\n  const num = parseInt(str, 10);\n  return isNaN(num) ? Either.left('Invalid number') : Either.right(num);\n};\n\nconst processUserInput = (input) => \n  Maybe.of(input)\n    .map(s => s.trim())\n    .flatMap(s => s.length > 0 ? Maybe.of(s) : Maybe.none())\n    .map(s => s.toUpperCase())\n    .getOrElse('DEFAULT');".to_string()),
        }).collect()
    }
}

/// Rule: require-function-composition
/// Enforces function composition patterns and point-free style
#[derive(Clone)]
pub struct RequireFunctionComposition;

impl RequireFunctionComposition {
    pub const NAME: &'static str = "require-function-composition";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireFunctionComposition {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for nested function calls without composition
        if code.contains("func1(func2(func3(") && !code.contains("pipe") && !code.contains("compose") {
            diagnostics.push(create_function_composition_diagnostic(
                0, 0,
                "Deeply nested function calls should use composition utilities like pipe() or compose()"
            ));
        }

        // Check for repetitive transformation chains
        if code.contains(".map(").count() > 2 && !code.contains("pipe") {
            diagnostics.push(create_function_composition_diagnostic(
                0, 0,
                "Multiple chained transformations could benefit from function composition"
            ));
        }

        // Check for point-ful style that could be point-free
        if code.contains("x => func(x)") && !code.contains("// Explicit for clarity") {
            diagnostics.push(create_function_composition_diagnostic(
                0, 0,
                "Consider point-free style for simple function applications"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireFunctionComposition {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.87,
            suggestion: "Use function composition: `const transform = pipe(normalize, validate, format)` instead of `format(validate(normalize(data)))`".to_string(),
            fix_code: Some("// Function composition utilities\nconst pipe = (...fns) => (value) => fns.reduce((acc, fn) => fn(acc), value);\nconst compose = (...fns) => (value) => fns.reduceRight((acc, fn) => fn(acc), value);\n\n// Point-free style helpers\nconst map = fn => array => array.map(fn);\nconst filter = predicate => array => array.filter(predicate);\nconst reduce = (reducer, initial) => array => array.reduce(reducer, initial);\n\n// Example transformations\nconst normalize = str => str.toLowerCase().trim();\nconst validate = str => str.length > 0 ? str : null;\nconst format = str => `[${str}]`;\n\n// Composed transformation\nconst processText = pipe(\n  normalize,\n  validate,\n  format\n);\n\n// Array processing pipeline\nconst processNumbers = pipe(\n  filter(x => x > 0),\n  map(x => x * 2),\n  reduce((sum, x) => sum + x, 0)\n);\n\n// Conditional composition\nconst conditionalPipe = (condition, ...fns) => \n  condition ? pipe(...fns) : x => x;\n\n// Async composition\nconst asyncPipe = (...fns) => (value) => \n  fns.reduce(async (acc, fn) => fn(await acc), value);".to_string()),
        }).collect()
    }
}

/// Rule: require-referential-transparency
/// Enforces referential transparency and pure function principles
#[derive(Clone)]
pub struct RequireReferentialTransparency;

impl RequireReferentialTransparency {
    pub const NAME: &'static str = "require-referential-transparency";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireReferentialTransparency {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for functions that depend on external state
        if code.contains("function") && (code.contains("window.") || code.contains("global.")) && !code.contains("inject") {
            diagnostics.push(create_referential_transparency_diagnostic(
                0, 0,
                "Functions should not depend on global state - pass dependencies as parameters"
            ));
        }

        // Check for functions that use Date.now() or Math.random() without isolation
        if code.contains("Date.now()") || code.contains("Math.random()") && !code.contains("IO") && !code.contains("Effect") {
            diagnostics.push(create_referential_transparency_diagnostic(
                0, 0,
                "Non-deterministic functions should be isolated using effect systems or dependency injection"
            ));
        }

        // Check for functions that read from external APIs directly
        if code.contains("fetch(") && code.contains("function") && !code.contains("Effect") && !code.contains("async") {
            diagnostics.push(create_referential_transparency_diagnostic(
                0, 0,
                "IO operations should be isolated from pure business logic"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireReferentialTransparency {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.90,
            suggestion: "Isolate side effects: `const pureLogic = (data, deps) => deps.random() < 0.5 ? process(data) : data; const withEffects = pureLogic(data, { random: Math.random });`".to_string(),
            fix_code: Some("// Dependency injection for pure functions\nconst createLogger = () => ({ log: console.log });\nconst createTimeProvider = () => ({ now: () => Date.now() });\nconst createRandomProvider = () => ({ random: () => Math.random() });\n\n// Pure business logic\nconst calculateScore = (data, timeProvider, randomProvider) => {\n  const timestamp = timeProvider.now();\n  const randomFactor = randomProvider.random();\n  return data.value * randomFactor + timestamp;\n};\n\n// Effect system pattern\nclass IO {\n  constructor(effect) {\n    this.effect = effect;\n  }\n  \n  static of(value) {\n    return new IO(() => value);\n  }\n  \n  map(fn) {\n    return new IO(() => fn(this.effect()));\n  }\n  \n  flatMap(fn) {\n    return new IO(() => fn(this.effect()).effect());\n  }\n  \n  run() {\n    return this.effect();\n  }\n}\n\n// Isolate IO operations\nconst fetchUserIO = (id) => new IO(() => fetch(`/api/users/${id}`));\nconst logIO = (message) => new IO(() => console.log(message));\n\n// Pure computation\nconst processUser = (user) => ({\n  ...user,\n  processedAt: Date.now() // This would be injected in practice\n});\n\n// Compose pure and impure\nconst getUserAndProcess = (id) => \n  fetchUserIO(id)\n    .flatMap(response => new IO(() => response.json()))\n    .map(processUser)\n    .flatMap(user => logIO(`Processed user ${user.id}`).map(() => user));".to_string()),
        }).collect()
    }
}

/// Rule: require-lazy-evaluation
/// Enforces lazy evaluation patterns for performance optimization
#[derive(Clone)]
pub struct RequireLazyEvaluation;

impl RequireLazyEvaluation {
    pub const NAME: &'static str = "require-lazy-evaluation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireLazyEvaluation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for expensive computations without memoization
        if code.contains("expensive") && code.contains("function") && !code.contains("memo") && !code.contains("lazy") {
            diagnostics.push(create_lazy_evaluation_diagnostic(
                0, 0,
                "Expensive computations should use memoization or lazy evaluation"
            ));
        }

        // Check for large data processing without lazy sequences
        if code.contains("map(").count() > 1 && code.contains("filter(") && !code.contains("lazy") && !code.contains("generator") {
            diagnostics.push(create_lazy_evaluation_diagnostic(
                0, 0,
                "Multiple array transformations could benefit from lazy evaluation to avoid intermediate arrays"
            ));
        }

        // Check for conditional expressions that could be lazy
        if code.contains("||") && code.contains("expensive") && !code.contains("()") {
            diagnostics.push(create_lazy_evaluation_diagnostic(
                0, 0,
                "Expensive default values should use lazy evaluation with functions"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireLazyEvaluation {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.86,
            suggestion: "Implement lazy evaluation: `const lazy = (fn) => { let cached; let computed = false; return () => { if (!computed) { cached = fn(); computed = true; } return cached; }; };`".to_string(),
            fix_code: Some("// Lazy evaluation utilities\nconst lazy = (fn) => {\n  let cached;\n  let computed = false;\n  return () => {\n    if (!computed) {\n      cached = fn();\n      computed = true;\n    }\n    return cached;\n  };\n};\n\n// Memoization for expensive functions\nconst memoize = (fn) => {\n  const cache = new Map();\n  return (...args) => {\n    const key = JSON.stringify(args);\n    if (cache.has(key)) {\n      return cache.get(key);\n    }\n    const result = fn(...args);\n    cache.set(key, result);\n    return result;\n  };\n};\n\n// Lazy sequence implementation\nclass LazySequence {\n  constructor(iterable) {\n    this.iterable = iterable;\n  }\n  \n  static of(iterable) {\n    return new LazySequence(iterable);\n  }\n  \n  map(fn) {\n    return new LazySequence(function* () {\n      for (const item of this.iterable) {\n        yield fn(item);\n      }\n    }.call(this));\n  }\n  \n  filter(predicate) {\n    return new LazySequence(function* () {\n      for (const item of this.iterable) {\n        if (predicate(item)) {\n          yield item;\n        }\n      }\n    }.call(this));\n  }\n  \n  take(count) {\n    return new LazySequence(function* () {\n      let taken = 0;\n      for (const item of this.iterable) {\n        if (taken >= count) break;\n        yield item;\n        taken++;\n      }\n    }.call(this));\n  }\n  \n  toArray() {\n    return [...this.iterable];\n  }\n}\n\n// Example usage\nconst expensiveComputation = lazy(() => {\n  console.log('Computing...');\n  return Array.from({ length: 1000000 }, (_, i) => i * i);\n});\n\nconst processLargeDataset = (data) => \n  LazySequence.of(data)\n    .filter(x => x % 2 === 0)\n    .map(x => x * 2)\n    .take(100)\n    .toArray();".to_string()),
        }).collect()
    }
}

/// Rule: require-algebraic-data-types
/// Enforces algebraic data type patterns for type safety
#[derive(Clone)]
pub struct RequireAlgebraicDataTypes;

impl RequireAlgebraicDataTypes {
    pub const NAME: &'static str = "require-algebraic-data-types";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireAlgebraicDataTypes {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for state machines without algebraic data types
        if code.contains("state") && code.contains("switch") && !code.contains("type") && !code.contains("union") {
            diagnostics.push(create_algebraic_data_types_diagnostic(
                0, 0,
                "State machines should use algebraic data types (union types) for type safety"
            ));
        }

        // Check for error handling without sum types
        if code.contains("error") && code.contains("success") && !code.contains("Either") && !code.contains("Result") {
            diagnostics.push(create_algebraic_data_types_diagnostic(
                0, 0,
                "Error handling should use sum types like Either or Result for exhaustive case handling"
            ));
        }

        // Check for optional values without Maybe types
        if code.contains("null") && code.contains("undefined") && !code.contains("Option") && !code.contains("Maybe") {
            diagnostics.push(create_algebraic_data_types_diagnostic(
                0, 0,
                "Optional values should use algebraic data types like Option or Maybe"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireAlgebraicDataTypes {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.88,
            suggestion: "Use algebraic data types: `type LoadingState = { tag: 'loading' } | { tag: 'success', data: T } | { tag: 'error', error: string };`".to_string(),
            fix_code: Some("// Sum types for state management\ntype LoadingState<T> = \n  | { tag: 'idle' }\n  | { tag: 'loading' }\n  | { tag: 'success'; data: T }\n  | { tag: 'error'; error: string };\n\n// Pattern matching helper\nconst match = <T, R>(value: T, patterns: Record<string, (val: any) => R>): R => {\n  const key = (value as any).tag || 'default';\n  const handler = patterns[key] || patterns.default;\n  if (!handler) {\n    throw new Error(`No handler for case: ${key}`);\n  }\n  return handler(value);\n};\n\n// Usage example\nconst renderLoadingState = <T>(state: LoadingState<T>) => \n  match(state, {\n    idle: () => 'Ready to load',\n    loading: () => 'Loading...',\n    success: ({ data }) => `Loaded: ${JSON.stringify(data)}`,\n    error: ({ error }) => `Error: ${error}`\n  });\n\n// Result type for error handling\ntype Result<T, E> = \n  | { tag: 'ok'; value: T }\n  | { tag: 'err'; error: E };\n\nconst Ok = <T>(value: T): Result<T, never> => ({ tag: 'ok', value });\nconst Err = <E>(error: E): Result<never, E> => ({ tag: 'err', error });\n\n// Option type for nullable values\ntype Option<T> = \n  | { tag: 'some'; value: T }\n  | { tag: 'none' };\n\nconst Some = <T>(value: T): Option<T> => ({ tag: 'some', value });\nconst None: Option<never> = { tag: 'none' };\n\n// Helper functions\nconst mapOption = <T, U>(option: Option<T>, fn: (value: T) => U): Option<U> => \n  match(option, {\n    some: ({ value }) => Some(fn(value)),\n    none: () => None\n  });".to_string()),
        }).collect()
    }
}

// Diagnostic creation functions
fn create_pure_functions_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequirePureFunctions::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Style".to_string(),
    }
}

fn create_immutable_data_operations_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireImmutableDataOperations::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_higher_order_functions_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireHigherOrderFunctions::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "info".to_string(),
        category: "Style".to_string(),
    }
}

fn create_monadic_error_handling_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireMonadicErrorHandling::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_function_composition_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireFunctionComposition::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "info".to_string(),
        category: "Style".to_string(),
    }
}

fn create_referential_transparency_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireReferentialTransparency::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Style".to_string(),
    }
}

fn create_lazy_evaluation_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireLazyEvaluation::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Performance".to_string(),
    }
}

fn create_algebraic_data_types_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireAlgebraicDataTypes::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "info".to_string(),
        category: "Style".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_pure_functions_violation() {
        let code = r#"
        function addToList(list, item) {
            list.push(item);
            console.log('Added item');
            return list;
        }
        "#;

        let rule = RequirePureFunctions;
        let diagnostics = rule.run(code);

        assert!(diagnostics.len() >= 2); // push and console.log
        assert!(diagnostics.iter().any(|d| d.message.contains("immutable")));
        assert!(diagnostics.iter().any(|d| d.message.contains("side effects")));
    }

    #[test]
    fn test_require_pure_functions_compliant() {
        let code = r#"
        const addToList = (list, item) => [...list, item];
        const processWithLogger = (data, logger) => {
            const result = transform(data);
            logger('Processing complete');
            return result;
        };
        "#;

        let rule = RequirePureFunctions;
        let diagnostics = rule.run(code);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_require_immutable_data_operations_violation() {
        let code = r#"
        const numbers = [3, 1, 4, 1, 5];
        const sorted = numbers.sort();
        const reversed = numbers.reverse();
        "#;

        let rule = RequireImmutableDataOperations;
        let diagnostics = rule.run(code);

        assert!(diagnostics.len() >= 2); // sort and reverse
        assert!(diagnostics.iter().any(|d| d.message.contains("sort")));
        assert!(diagnostics.iter().any(|d| d.message.contains("reverse")));
    }

    #[test]
    fn test_require_higher_order_functions_violation() {
        let code = r#"
        const results = [];
        for (let i = 0; i < items.length; i++) {
            if (items[i] > 5) {
                results.push(items[i] * 2);
            }
        }
        "#;

        let rule = RequireHigherOrderFunctions;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("map"));
    }

    #[test]
    fn test_require_monadic_error_handling_violation() {
        let code = r#"
        function getUser(id) {
            try {
                const user = fetchUser(id);
                if (user !== null && user !== undefined) {
                    return user;
                }
                return null;
            } catch (error) {
                return null;
            }
        }
        "#;

        let rule = RequireMonadicErrorHandling;
        let diagnostics = rule.run(code);

        assert!(diagnostics.len() >= 2); // null checks and try-catch
    }

    #[test]
    fn test_ai_enhancement_pure_functions() {
        let rule = RequirePureFunctions;
        let diagnostics = vec![create_pure_functions_diagnostic(0, 0, "test")];
        let suggestions = rule.ai_enhance("", &diagnostics);

        assert!(!suggestions.is_empty());
        assert!(suggestions[0].confidence > 0.9);
        assert!(suggestions[0].suggestion.contains("pure"));
    }

    #[test]
    fn test_ai_enhancement_monadic_error_handling() {
        let rule = RequireMonadicErrorHandling;
        let diagnostics = vec![create_monadic_error_handling_diagnostic(0, 0, "test")];
        let suggestions = rule.ai_enhance("", &diagnostics);

        assert!(!suggestions.is_empty());
        assert!(suggestions[0].confidence > 0.8);
        assert!(suggestions[0].suggestion.contains("Maybe"));
    }
}