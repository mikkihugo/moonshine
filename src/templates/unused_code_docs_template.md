# {{RULE_ID}}

Detects {{PATTERN_DESCRIPTION}} that should be removed to improve code clarity and maintainability.

## Rule Details

This rule identifies code elements that are declared but never used in your codebase. Unused code can:

- Confuse other developers reading the code
- Indicate incomplete or dead code paths
- Increase bundle size in client-side applications
- Create maintenance burden
- Hide potential bugs

## Examples

### ❌ Incorrect

```typescript
// Unused variable
const data = fetchUserData();
console.log('Processing...');

// Unused function
function calculateTax(amount: number) {
    return amount * 0.1;
}

// Unused import
import { helper, unusedUtility } from './utils';
console.log(helper());

// Unused parameter
function processUser(user: User, unusedOptions: Options) {
    return user.name;
}
```

### ✅ Correct

```typescript
// Used variable
const data = fetchUserData();
console.log('Data:', data);

// Used function
function calculateTax(amount: number) {
    return amount * 0.1;
}
const tax = calculateTax(100);

// Used imports only
import { helper } from './utils';
console.log(helper());

// All parameters used
function processUser(user: User, options: Options) {
    return options.format === 'full' ? user.fullName : user.name;
}
```

## Options

This rule has no configuration options.

## When to Disable

You might want to disable this rule when:

- Working with auto-generated code that may have unused elements
- During development when code is temporarily unused
- For variables that are used by external tools or frameworks (use ESLint disable comments for specific cases)
- In test files where some imports might be used by the test framework

```typescript
// eslint-disable-next-line {{RULE_ID}}
const debugVar = expensiveCalculation(); // Used by debugger
```

## Version

This rule was automatically generated from pattern analysis detecting {{PATTERN_FREQUENCY}} similar violations across your codebase.

## Related Rules

- `@typescript-eslint/no-unused-vars` - TypeScript-specific unused variable detection
- `no-unused-vars` - ESLint's built-in unused variable rule
- `import/no-unused-modules` - Detects unused modules in import/export chains

## Further Reading

- [TypeScript Handbook: Unused Locals](https://www.typescriptlang.org/tsconfig#noUnusedLocals)
- [Clean Code Principles](https://clean-code-developer.com/)