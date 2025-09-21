// Flat ESLint v9+ config for moon-shine package
// -------------------------------------------------------------
// - Uses OXC parser for fast JS/TS parsing
// - Integrates oxlint plugin with recommended rules
// - Extends ESLint and TypeScript recommended rules for compatibility
// - Supports both .ts/.tsx and .js/.jsx files
// - Place this file at the root of packages/tools/moon-shine/
// - Usage: `moon run moon-shine:lint` or `eslint .` (with v9+)
// -------------------------------------------------------------

/** @type {import('eslint').FlatConfig[]} */
export default [
  {
    files: ["**/*.{js,jsx,ts,tsx}",],
    languageOptions: {
      parser: "oxc", // OXC parser for JS/TS
      parserOptions: {
        ecmaVersion: 2022,
        sourceType: "module",
        project: ["./tsconfig.json",],
      },
    },
    plugins: {
      oxlint: require("eslint-plugin-oxlint",),
      // Optionally add @typescript-eslint for legacy rules
      "@typescript-eslint": require("@typescript-eslint/eslint-plugin",),
    },
    extends: [
      "eslint:recommended",
      "plugin:@typescript-eslint/recommended",
      "plugin:oxlint/recommended",
    ],
    rules: {
      // Place any moon-shine custom or override rules here
      // Example: 'no-console': 'warn',
    },
  },
];
