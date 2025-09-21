//! # Advanced TypeScript Rules
//!
//! Comprehensive rules for advanced TypeScript patterns, complex type systems,
//! metaprogramming, and enterprise-grade type safety.
//!
//! ## Rule Categories:
//! - **Advanced Types**: Conditional types, mapped types, template literals
//! - **Metaprogramming**: Type-level programming, recursive types, type inference
//! - **Generic Constraints**: Complex type constraints, variance, higher-kinded types
//! - **Module Systems**: Declaration merging, namespace management, ambient declarations
//! - **Performance**: Compilation performance, type checking optimization
//!
//! ## Examples:
//! ```typescript
//! // ✅ Good: Proper conditional type usage
//! type ApiResponse<T> = T extends string ? { message: T } : { data: T };
//!
//! // ❌ Bad: Overly complex type that hurts compilation
//! type DeepNested<T> = T extends object ? {
//!   [K in keyof T]: DeepNested<T[K]>
//! } : T; // Can cause infinite recursion
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

/// Rule: require-type-assertion-safety
/// Enforces safe type assertions and prevents dangerous casting
#[derive(Clone)]
pub struct RequireTypeAssertionSafety;

impl RequireTypeAssertionSafety {
    pub const NAME: &'static str = "require-type-assertion-safety";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireTypeAssertionSafety {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for unsafe 'as any' assertions
        if code.contains("as any") {
            diagnostics.push(create_type_assertion_safety_diagnostic(
                0, 0,
                "Avoid 'as any' assertions - use type guards or proper typing instead"
            ));
        }

        // Check for double assertions without justification
        if code.contains("as unknown as") && !code.contains("// Safe:") && !code.contains("// SAFETY:") {
            diagnostics.push(create_type_assertion_safety_diagnostic(
                0, 0,
                "Double assertions should include safety comments explaining why they're necessary"
            ));
        }

        // Check for assertions on complex types without validation
        if code.contains("as") && code.contains("{") && !code.contains("is ") && !code.contains("typeof") {
            diagnostics.push(create_type_assertion_safety_diagnostic(
                0, 0,
                "Complex type assertions should use type guards for runtime validation"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireTypeAssertionSafety {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.93,
            suggestion: "Replace unsafe assertions with type guards: `function isUser(obj: unknown): obj is User { return typeof obj === 'object' && obj !== null && 'id' in obj; }`".to_string(),
            fix_code: Some("// Instead of: const user = data as User;\n// Use type guard:\nfunction isUser(obj: unknown): obj is User {\n  return typeof obj === 'object' && obj !== null &&\n         typeof (obj as any).id === 'string' &&\n         typeof (obj as any).name === 'string';\n}\n\nif (isUser(data)) {\n  // data is now safely typed as User\n  console.log(data.name);\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-conditional-type-bounds
/// Enforces proper bounds checking in conditional types to prevent infinite recursion
#[derive(Clone)]
pub struct RequireConditionalTypeBounds;

impl RequireConditionalTypeBounds {
    pub const NAME: &'static str = "require-conditional-type-bounds";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Perf;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireConditionalTypeBounds {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for recursive conditional types without depth limits
        if code.contains("extends") && code.contains("?") && code.contains("keyof") && !code.contains("Depth") {
            diagnostics.push(create_conditional_type_bounds_diagnostic(
                0, 0,
                "Recursive conditional types should include depth limits to prevent infinite recursion"
            ));
        }

        // Check for complex mapped types without constraints
        if code.contains("[K in keyof T]") && code.contains("extends") && !code.contains("never") {
            diagnostics.push(create_conditional_type_bounds_diagnostic(
                0, 0,
                "Complex mapped types should include termination conditions"
            ));
        }

        // Check for template literal types without bounds
        if code.contains("template literal") && code.contains("${") && !code.contains("length") {
            diagnostics.push(create_conditional_type_bounds_diagnostic(
                0, 0,
                "Template literal types should consider string length bounds for performance"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireConditionalTypeBounds {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.89,
            suggestion: "Add depth bounds to recursive types: `type DeepReadonly<T, Depth extends number = 5> = Depth extends 0 ? T : T extends object ? { readonly [K in keyof T]: DeepReadonly<T[K], Prev<Depth>> } : T;`".to_string(),
            fix_code: Some("// Helper types for depth tracking\ntype Prev<N extends number> = [-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10][N];\n\n// Safe recursive type with depth limit\ntype DeepReadonly<T, Depth extends number = 5> = \n  Depth extends 0 \n    ? T \n    : T extends object \n      ? { readonly [K in keyof T]: DeepReadonly<T[K], Prev<Depth>> }\n      : T;\n\n// Alternative with explicit termination\ntype SafeDeepPick<T, K extends keyof T, Depth extends number = 3> =\n  Depth extends 0\n    ? never  // Terminate recursion\n    : K extends keyof T\n      ? T[K] extends object\n        ? DeepPick<T[K], keyof T[K], Prev<Depth>>\n        : T[K]\n      : never;".to_string()),
        }).collect()
    }
}

/// Rule: require-generic-variance-annotations
/// Enforces proper variance annotations for generic type parameters
#[derive(Clone)]
pub struct RequireGenericVarianceAnnotations;

impl RequireGenericVarianceAnnotations {
    pub const NAME: &'static str = "require-generic-variance-annotations";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireGenericVarianceAnnotations {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for generic interfaces without variance annotations
        if code.contains("interface") && code.contains("<T>") && !code.contains("readonly") && !code.contains("out ") && !code.contains("in ") {
            diagnostics.push(create_generic_variance_annotations_diagnostic(
                0, 0,
                "Generic interfaces should consider variance annotations for type safety"
            ));
        }

        // Check for function type parameters without proper constraints
        if code.contains("function") && code.contains("<T") && !code.contains("extends") {
            diagnostics.push(create_generic_variance_annotations_diagnostic(
                0, 0,
                "Generic function parameters should include appropriate type constraints"
            ));
        }

        // Check for contravariant positions without proper handling
        if code.contains("(arg: T)") && code.contains("interface") && !code.contains("contravariant") {
            diagnostics.push(create_generic_variance_annotations_diagnostic(
                0, 0,
                "Parameters in contravariant positions should be properly documented"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireGenericVarianceAnnotations {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.86,
            suggestion: "Add variance annotations: `interface Container<out T> { get(): T; } interface Consumer<in T> { consume(item: T): void; }`".to_string(),
            fix_code: Some("// Covariant - type parameter appears only in output positions\ninterface Producer<out T> {\n  produce(): T;\n  readonly value: T;\n}\n\n// Contravariant - type parameter appears only in input positions  \ninterface Consumer<in T> {\n  consume(item: T): void;\n  process(items: T[]): void;\n}\n\n// Invariant - type parameter appears in both positions\ninterface Container<T> {\n  get(): T;\n  set(value: T): void;\n}\n\n// With constraints for better type safety\ninterface Repository<T extends { id: string }> {\n  save(entity: T): Promise<void>;\n  findById(id: string): Promise<T | null>;\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-branded-types
/// Enforces branded types for domain-specific primitives
#[derive(Clone)]
pub struct RequireBrandedTypes;

impl RequireBrandedTypes {
    pub const NAME: &'static str = "require-branded-types";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireBrandedTypes {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for ID types without branding
        if (code.contains("userId") || code.contains("orderId") || code.contains("productId")) &&
           code.contains(": string") && !code.contains("Brand") {
            diagnostics.push(create_branded_types_diagnostic(
                0, 0,
                "ID types should use branded types to prevent mixing different ID types"
            ));
        }

        // Check for monetary values without currency branding
        if code.contains("price") && code.contains(": number") && !code.contains("Currency") {
            diagnostics.push(create_branded_types_diagnostic(
                0, 0,
                "Monetary values should use branded types to prevent currency confusion"
            ));
        }

        // Check for domain primitives without type safety
        if code.contains("email") && code.contains(": string") && !code.contains("Email") {
            diagnostics.push(create_branded_types_diagnostic(
                0, 0,
                "Domain primitives like email should use branded types for validation"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireBrandedTypes {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.88,
            suggestion: "Implement branded types: `type UserId = string & { __brand: 'UserId' }; type OrderId = string & { __brand: 'OrderId' };`".to_string(),
            fix_code: Some("// Branded type utility\ntype Brand<T, TBrand> = T & { __brand: TBrand };\n\n// Domain-specific ID types\ntype UserId = Brand<string, 'UserId'>;\ntype OrderId = Brand<string, 'OrderId'>;\ntype ProductId = Brand<string, 'ProductId'>;\n\n// Constructor functions with validation\nfunction createUserId(id: string): UserId {\n  if (!id || id.length < 3) {\n    throw new Error('Invalid user ID');\n  }\n  return id as UserId;\n}\n\n// Currency branded types\ntype USD = Brand<number, 'USD'>;\ntype EUR = Brand<number, 'EUR'>;\n\ninterface Product {\n  id: ProductId;\n  name: string;\n  price: USD;  // Cannot accidentally use EUR\n}\n\n// Email validation with branding\ntype Email = Brand<string, 'Email'>;\n\nfunction createEmail(email: string): Email {\n  if (!/^[^\\s@]+@[^\\s@]+\\.[^\\s@]+$/.test(email)) {\n    throw new Error('Invalid email format');\n  }\n  return email as Email;\n}\n\n// Usage prevents mixing types\nfunction transferMoney(fromUser: UserId, toUser: UserId, amount: USD) {\n  // Cannot pass OrderId where UserId expected\n  // Cannot pass EUR where USD expected\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-exhaustive-switch-cases
/// Enforces exhaustive switch cases for union types and enums
#[derive(Clone)]
pub struct RequireExhaustiveSwitchCases;

impl RequireExhaustiveSwitchCases {
    pub const NAME: &'static str = "require-exhaustive-switch-cases";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireExhaustiveSwitchCases {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for switch statements without exhaustiveness checking
        if code.contains("switch") && !code.contains("default:") && !code.contains("assertNever") {
            diagnostics.push(create_exhaustive_switch_cases_diagnostic(
                0, 0,
                "Switch statements on union types should be exhaustive or include default case with assertNever"
            ));
        }

        // Check for discriminated unions without proper handling
        if code.contains("type") && code.contains("|") && code.contains("kind") && code.contains("switch") && !code.contains("never") {
            diagnostics.push(create_exhaustive_switch_cases_diagnostic(
                0, 0,
                "Discriminated union switches should include exhaustiveness checking"
            ));
        }

        // Check for enum switches without completeness
        if code.contains("enum") && code.contains("switch") && !code.contains("default") {
            diagnostics.push(create_exhaustive_switch_cases_diagnostic(
                0, 0,
                "Enum switches should handle all cases or include default with exhaustiveness check"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireExhaustiveSwitchCases {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.92,
            suggestion: "Add exhaustiveness checking: `function assertNever(x: never): never { throw new Error('Unexpected value: ' + x); } default: return assertNever(value);`".to_string(),
            fix_code: Some("// Exhaustiveness helper\nfunction assertNever(x: never): never {\n  throw new Error(`Unexpected value: ${x}`);\n}\n\n// Discriminated union example\ntype Action = \n  | { type: 'increment'; payload: number }\n  | { type: 'decrement'; payload: number }\n  | { type: 'reset' };\n\nfunction reducer(action: Action): number {\n  switch (action.type) {\n    case 'increment':\n      return action.payload;\n    case 'decrement':\n      return -action.payload;\n    case 'reset':\n      return 0;\n    default:\n      // TypeScript will error if new cases are added but not handled\n      return assertNever(action);\n  }\n}\n\n// Enum example\nenum Status {\n  Pending = 'pending',\n  Approved = 'approved',\n  Rejected = 'rejected'\n}\n\nfunction handleStatus(status: Status): string {\n  switch (status) {\n    case Status.Pending:\n      return 'Processing...';\n    case Status.Approved:\n      return 'Success!';\n    case Status.Rejected:\n      return 'Failed!';\n    default:\n      return assertNever(status);\n  }\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-type-level-documentation
/// Enforces documentation for complex type definitions
#[derive(Clone)]
pub struct RequireTypeLevelDocumentation;

impl RequireTypeLevelDocumentation {
    pub const NAME: &'static str = "require-type-level-documentation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireTypeLevelDocumentation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for complex type aliases without documentation
        if code.contains("type") && code.contains("extends") && code.contains("?") && !code.contains("/**") {
            diagnostics.push(create_type_level_documentation_diagnostic(
                0, 0,
                "Complex type definitions should include JSDoc comments explaining their purpose"
            ));
        }

        // Check for utility types without usage examples
        if code.contains("type") && (code.contains("Utility") || code.contains("Helper")) && !code.contains("@example") {
            diagnostics.push(create_type_level_documentation_diagnostic(
                0, 0,
                "Utility types should include usage examples in documentation"
            ));
        }

        // Check for generic constraints without explanation
        if code.contains("<T extends") && !code.contains("@template") && !code.contains("//") {
            diagnostics.push(create_type_level_documentation_diagnostic(
                0, 0,
                "Generic type constraints should be documented with their reasoning"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireTypeLevelDocumentation {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.85,
            suggestion: "Add type documentation: `/** @description Utility type that makes all properties optional recursively @template T - The type to make deeply optional @example type Optional = DeepPartial<User> */`".to_string(),
            fix_code: Some("/**\n * Utility type that makes all properties optional recursively,\n * useful for partial updates in forms or API patches.\n * \n * @template T - The type to make deeply optional\n * @example\n * ```typescript\n * interface User {\n *   name: string;\n *   address: {\n *     street: string;\n *     city: string;\n *   };\n * }\n * \n * type PartialUser = DeepPartial<User>;\n * // Result: {\n * //   name?: string;\n * //   address?: {\n * //     street?: string;\n * //     city?: string;\n * //   };\n * // }\n * ```\n */\ntype DeepPartial<T> = {\n  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];\n};\n\n/**\n * Creates a new type by picking properties from T that are assignable to U.\n * Commonly used for extracting function properties or specific value types.\n * \n * @template T - Source type to pick from\n * @template U - Target type that picked properties must be assignable to\n * @example\n * ```typescript\n * interface Example {\n *   name: string;\n *   age: number;\n *   isActive: boolean;\n *   onClick: () => void;\n * }\n * \n * type StringProps = PickByType<Example, string>; // { name: string }\n * type FunctionProps = PickByType<Example, Function>; // { onClick: () => void }\n * ```\n */\ntype PickByType<T, U> = {\n  [K in keyof T as T[K] extends U ? K : never]: T[K];\n};".to_string()),
        }).collect()
    }
}

/// Rule: require-immutable-data-structures
/// Enforces immutable data structure patterns using TypeScript types
#[derive(Clone)]
pub struct RequireImmutableDataStructures;

impl RequireImmutableDataStructures {
    pub const NAME: &'static str = "require-immutable-data-structures";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Style;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireImmutableDataStructures {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for mutable arrays in interface definitions
        if code.contains("interface") && code.contains("[]") && !code.contains("readonly") && !code.contains("ReadonlyArray") {
            diagnostics.push(create_immutable_data_structures_diagnostic(
                0, 0,
                "Array properties in interfaces should be readonly to prevent mutation"
            ));
        }

        // Check for mutable object properties
        if code.contains("interface") && code.contains("{") && !code.contains("readonly") && code.contains(": {") {
            diagnostics.push(create_immutable_data_structures_diagnostic(
                0, 0,
                "Object properties should be readonly for immutable data structures"
            ));
        }

        // Check for missing ReadonlySet/ReadonlyMap usage
        if code.contains("Set<") && !code.contains("ReadonlySet") && code.contains("interface") {
            diagnostics.push(create_immutable_data_structures_diagnostic(
                0, 0,
                "Use ReadonlySet instead of Set for immutable data structures"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireImmutableDataStructures {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.87,
            suggestion: "Use immutable types: `interface User { readonly name: string; readonly tags: ReadonlyArray<string>; readonly metadata: Readonly<Record<string, any>>; }`".to_string(),
            fix_code: Some("// Immutable data structure patterns\n\n// Basic immutable interface\ninterface User {\n  readonly id: string;\n  readonly name: string;\n  readonly email: string;\n  readonly tags: ReadonlyArray<string>;\n  readonly preferences: Readonly<{\n    theme: 'light' | 'dark';\n    notifications: boolean;\n  }>;\n}\n\n// Deep readonly utility type\ntype DeepReadonly<T> = {\n  readonly [P in keyof T]: T[P] extends (infer U)[]\n    ? ReadonlyArray<DeepReadonly<U>>\n    : T[P] extends object\n    ? DeepReadonly<T[P]>\n    : T[P];\n};\n\n// Immutable collections\ninterface UserRepository {\n  readonly users: ReadonlyMap<string, User>;\n  readonly activeUserIds: ReadonlySet<string>;\n  readonly metadata: ReadonlyArray<{\n    readonly key: string;\n    readonly value: unknown;\n  }>;\n}\n\n// Update patterns with immutability\nfunction updateUser(user: User, updates: Partial<Pick<User, 'name' | 'email'>>): User {\n  return {\n    ...user,\n    ...updates,\n    // Arrays and objects need explicit reconstruction\n    tags: user.tags, // Already readonly\n    preferences: {\n      ...user.preferences,\n      // Any preference updates would go here\n    }\n  };\n}".to_string()),
        }).collect()
    }
}

/// Rule: require-template-literal-type-validation
/// Enforces validation patterns for template literal types
#[derive(Clone)]
pub struct RequireTemplateLiteralTypeValidation;

impl RequireTemplateLiteralTypeValidation {
    pub const NAME: &'static str = "require-template-literal-type-validation";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Suggestion;
}

impl WasmRule for RequireTemplateLiteralTypeValidation {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for template literal types without runtime validation
        if code.contains("${") && code.contains("type") && !code.contains("validate") && !code.contains("guard") {
            diagnostics.push(create_template_literal_type_validation_diagnostic(
                0, 0,
                "Template literal types should include runtime validation functions"
            ));
        }

        // Check for CSS-in-JS template literals without type safety
        if code.contains("css`") && !code.contains("CSS") && !code.contains("styled") {
            diagnostics.push(create_template_literal_type_validation_diagnostic(
                0, 0,
                "CSS template literals should use typed CSS-in-JS solutions"
            ));
        }

        // Check for SQL template literals without type checking
        if code.contains("sql`") && !code.contains("Query") && !code.contains("typed") {
            diagnostics.push(create_template_literal_type_validation_diagnostic(
                0, 0,
                "SQL template literals should use typed query builders for safety"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireTemplateLiteralTypeValidation {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.86,
            suggestion: "Add template literal validation: `type EmailTemplate = `${string}@${string}.${string}`; function validateEmail(email: string): email is EmailTemplate { return /^.+@.+\\..+$/.test(email); }`".to_string(),
            fix_code: Some("// Template literal type with validation\ntype EventName = `on${Capitalize<string>}`;\ntype CSSProperty = `--${string}`;\ntype APIEndpoint = `/api/${string}`;\n\n// Runtime validation functions\nfunction isEventName(name: string): name is EventName {\n  return name.startsWith('on') && name.length > 2;\n}\n\nfunction isCSSProperty(prop: string): prop is CSSProperty {\n  return prop.startsWith('--');\n}\n\nfunction isAPIEndpoint(path: string): path is APIEndpoint {\n  return path.startsWith('/api/');\n}\n\n// Usage with type guards\nfunction handleEvent(eventName: string, handler: Function) {\n  if (!isEventName(eventName)) {\n    throw new Error(`Invalid event name: ${eventName}`);\n  }\n  // eventName is now typed as EventName\n  element.addEventListener(eventName.slice(2).toLowerCase(), handler);\n}\n\n// Typed CSS custom properties\nfunction setCSSProperty(element: HTMLElement, property: string, value: string) {\n  if (!isCSSProperty(property)) {\n    throw new Error(`Invalid CSS custom property: ${property}`);\n  }\n  // property is now typed as CSSProperty\n  element.style.setProperty(property, value);\n}\n\n// Advanced: Recursive template literal types\ntype Join<T extends readonly string[], Separator extends string = ','> = \n  T extends readonly [infer First, ...infer Rest]\n    ? First extends string\n      ? Rest extends readonly string[]\n        ? Rest['length'] extends 0\n          ? First\n          : `${First}${Separator}${Join<Rest, Separator>}`\n        : never\n      : never\n    : '';\n\ntype CSVRow = Join<['name', 'age', 'email']>; // \"name,age,email\"".to_string()),
        }).collect()
    }
}

/// Rule: require-type-predicate-functions
/// Enforces type predicate functions for runtime type checking
#[derive(Clone)]
pub struct RequireTypePredicateFunctions;

impl RequireTypePredicateFunctions {
    pub const NAME: &'static str = "require-type-predicate-functions";
    pub const CATEGORY: WasmRuleCategory = WasmRuleCategory::Correctness;
    pub const FIX_STATUS: WasmFixStatus = WasmFixStatus::Fix;
}

impl WasmRule for RequireTypePredicateFunctions {
    const NAME: &'static str = Self::NAME;
    const CATEGORY: WasmRuleCategory = Self::CATEGORY;
    const FIX_STATUS: WasmFixStatus = Self::FIX_STATUS;

    fn run(&self, code: &str) -> Vec<WasmRuleDiagnostic> {
        let mut diagnostics = Vec::new();

        // Check for typeof checks without type predicates
        if code.contains("typeof") && code.contains("===") && !code.contains("is ") {
            diagnostics.push(create_type_predicate_functions_diagnostic(
                0, 0,
                "Type checking logic should be encapsulated in type predicate functions"
            ));
        }

        // Check for instanceof without type guards
        if code.contains("instanceof") && !code.contains("is ") && code.contains("function") {
            diagnostics.push(create_type_predicate_functions_diagnostic(
                0, 0,
                "instanceof checks should be wrapped in type predicate functions"
            ));
        }

        // Check for API response validation without type guards
        if code.contains("response") && code.contains("validate") && !code.contains("is ") {
            diagnostics.push(create_type_predicate_functions_diagnostic(
                0, 0,
                "API response validation should use type predicate functions for type safety"
            ));
        }

        diagnostics
    }
}

impl EnhancedWasmRule for RequireTypePredicateFunctions {
    fn ai_enhance(&self, code: &str, diagnostics: &[WasmRuleDiagnostic]) -> Vec<AiSuggestion> {
        diagnostics.iter().map(|_| AiSuggestion {
            confidence: 0.91,
            suggestion: "Create type predicate functions: `function isString(value: unknown): value is string { return typeof value === 'string'; } function isUser(obj: unknown): obj is User { return typeof obj === 'object' && obj !== null && 'id' in obj; }`".to_string(),
            fix_code: Some("// Basic type predicates\nfunction isString(value: unknown): value is string {\n  return typeof value === 'string';\n}\n\nfunction isNumber(value: unknown): value is number {\n  return typeof value === 'number' && !isNaN(value);\n}\n\nfunction isArray<T>(value: unknown, itemGuard: (item: unknown) => item is T): value is T[] {\n  return Array.isArray(value) && value.every(itemGuard);\n}\n\n// Complex object type predicates\ninterface User {\n  id: string;\n  name: string;\n  email: string;\n  age?: number;\n}\n\nfunction isUser(obj: unknown): obj is User {\n  return (\n    typeof obj === 'object' &&\n    obj !== null &&\n    'id' in obj &&\n    'name' in obj &&\n    'email' in obj &&\n    isString((obj as any).id) &&\n    isString((obj as any).name) &&\n    isString((obj as any).email) &&\n    (typeof (obj as any).age === 'undefined' || isNumber((obj as any).age))\n  );\n}\n\n// API response type predicate\ninterface APIResponse<T> {\n  success: boolean;\n  data: T;\n  error?: string;\n}\n\nfunction isAPIResponse<T>(\n  obj: unknown,\n  dataGuard: (data: unknown) => data is T\n): obj is APIResponse<T> {\n  return (\n    typeof obj === 'object' &&\n    obj !== null &&\n    'success' in obj &&\n    'data' in obj &&\n    typeof (obj as any).success === 'boolean' &&\n    dataGuard((obj as any).data) &&\n    (typeof (obj as any).error === 'undefined' || isString((obj as any).error))\n  );\n}\n\n// Usage example\nasync function fetchUser(id: string): Promise<User> {\n  const response = await fetch(`/api/users/${id}`);\n  const data = await response.json();\n  \n  if (isAPIResponse(data, isUser)) {\n    if (data.success) {\n      return data.data; // Properly typed as User\n    } else {\n      throw new Error(data.error || 'Unknown error');\n    }\n  }\n  \n  throw new Error('Invalid API response format');\n}".to_string()),
        }).collect()
    }
}

// Diagnostic creation functions
fn create_type_assertion_safety_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireTypeAssertionSafety::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_conditional_type_bounds_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireConditionalTypeBounds::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Performance".to_string(),
    }
}

fn create_generic_variance_annotations_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireGenericVarianceAnnotations::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "info".to_string(),
        category: "Style".to_string(),
    }
}

fn create_branded_types_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireBrandedTypes::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Style".to_string(),
    }
}

fn create_exhaustive_switch_cases_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireExhaustiveSwitchCases::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_type_level_documentation_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireTypeLevelDocumentation::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "info".to_string(),
        category: "Style".to_string(),
    }
}

fn create_immutable_data_structures_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireImmutableDataStructures::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Style".to_string(),
    }
}

fn create_template_literal_type_validation_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireTemplateLiteralTypeValidation::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "warning".to_string(),
        category: "Correctness".to_string(),
    }
}

fn create_type_predicate_functions_diagnostic(line: usize, column: usize, message: &str) -> WasmRuleDiagnostic {
    WasmRuleDiagnostic {
        rule_name: RequireTypePredicateFunctions::NAME.to_string(),
        message: message.to_string(),
        line,
        column,
        severity: "error".to_string(),
        category: "Correctness".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_type_assertion_safety_violation() {
        let code = r#"
        const user = data as any;
        const config = response as unknown as Config;
        "#;

        let rule = RequireTypeAssertionSafety;
        let diagnostics = rule.run(code);

        assert!(diagnostics.len() >= 1);
        assert!(diagnostics[0].message.contains("as any"));
    }

    #[test]
    fn test_require_exhaustive_switch_cases_violation() {
        let code = r#"
        function handleAction(action: Action) {
            switch (action.type) {
                case 'increment':
                    return action.payload;
                case 'decrement':
                    return -action.payload;
            }
        }
        "#;

        let rule = RequireExhaustiveSwitchCases;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("exhaustive"));
    }

    #[test]
    fn test_require_branded_types_violation() {
        let code = r#"
        interface User {
            userId: string;
            email: string;
            price: number;
        }
        "#;

        let rule = RequireBrandedTypes;
        let diagnostics = rule.run(code);

        assert!(diagnostics.len() >= 2); // userId and price
        assert!(diagnostics.iter().any(|d| d.message.contains("ID types")));
    }

    #[test]
    fn test_require_type_predicate_functions_violation() {
        let code = r#"
        function processData(data: unknown) {
            if (typeof data === 'string') {
                return data.toUpperCase();
            }
        }
        "#;

        let rule = RequireTypePredicateFunctions;
        let diagnostics = rule.run(code);

        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.contains("type predicate"));
    }

    #[test]
    fn test_ai_enhancement_type_assertion_safety() {
        let rule = RequireTypeAssertionSafety;
        let diagnostics = vec![create_type_assertion_safety_diagnostic(0, 0, "test")];
        let suggestions = rule.ai_enhance("", &diagnostics);

        assert!(!suggestions.is_empty());
        assert!(suggestions[0].confidence > 0.9);
        assert!(suggestions[0].suggestion.contains("type guard"));
    }

    #[test]
    fn test_ai_enhancement_exhaustive_switch_cases() {
        let rule = RequireExhaustiveSwitchCases;
        let diagnostics = vec![create_exhaustive_switch_cases_diagnostic(0, 0, "test")];
        let suggestions = rule.ai_enhance("", &diagnostics);

        assert!(!suggestions.is_empty());
        assert!(suggestions[0].confidence > 0.9);
        assert!(suggestions[0].suggestion.contains("assertNever"));
    }
}