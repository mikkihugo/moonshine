/**
 * CLI Program Definition
 * Following Rule C005: Single responsibility - only handle CLI structure
 */

const { Command } = require('commander');
const { version } = require('../package.json');

function createCliProgram() {
  const program = new Command();

  program
    .name('sunlint')
    .description('☀️ Sun Lint - Coding Standards Checker | Multi-rule Quality & Security Analysis')
    .version(version);

  // Rule selection options
  program
    .option('-r, --rule <rule>', 'Run single rule (e.g., C019)')
    .option('--rules <rules>', 'Run multiple rules (comma-separated, e.g., C019,C006,S005)')
    .option('-a, --all', 'Run all available rules')
    .option('-c, --category <category>', 'Run rules by category (quality,security,logging,naming)')
    .option('--quality', 'Run all code quality rules')
    .option('--security', 'Run all secure coding rules');

  // TypeScript specific options (Phase 1 focus)
  program
    .option('--typescript', 'Enable TypeScript-specific analysis')
    .option('--typescript-engine <engine>', 'TypeScript analysis engine (eslint,heuristic,hybrid)', 'heuristic')
    .option('--ensure-deps', 'Ensure ESLint dependencies are installed');

  // Input/Output options (v1.x: explicit --input required)
  program
    .option('-i, --input <path>', 'Input file or directory to analyze (REQUIRED)')
    .option('-f, --format <format>', 'Output format (eslint,json,summary,table)', 'eslint')
    .option('-o, --output <file>', 'Output file path')
    .option('--config <file>', 'Configuration file path (default: auto-discover)');

  // File targeting options
  program
    .option('--include <patterns>', 'Include file patterns (comma-separated globs)')
    .option('--exclude <patterns>', 'Exclude file patterns (comma-separated globs)')
    .option('--languages <languages>', 'Target specific languages (comma-separated: typescript,dart,kotlin)')
    .option('--include-tests', 'Include test files in analysis (default: true)')
    .option('--exclude-tests', 'Exclude test files from analysis')
    .option('--only-source', 'Only analyze source files (exclude tests, configs, etc.)');

  // CI/CD and Git integration options
  program
    .option('--changed-files', 'Only analyze files changed in current branch (git diff)')
    .option('--staged-files', 'Only analyze staged files (git diff --cached)')
    .option('--diff-base <ref>', 'Compare against specific git reference (e.g., origin/main)')
    .option('--since <commit>', 'Only analyze files changed since specific commit')
    .option('--pr-mode', 'Enable PR mode (changed files + baseline comparison)')
    .option('--baseline <file>', 'Load baseline results to compare against')
    .option('--save-baseline <file>', 'Save current results as baseline')
    .option('--fail-on-new-violations', 'Exit with error only on new violations (not existing)');

  // Performance options (SIMPLIFIED)
  program
    .option('--timeout <milliseconds>', 'Analysis timeout in milliseconds (default: auto)', '0')
    .option('--max-files <count>', 'Maximum files to analyze (default: auto-detect)', '0')
    .option('--performance <mode>', 'Performance mode: auto, fast, careful (default: auto)', 'auto');

  // Advanced options
  program
    .option('--engine <engine>', 'Analysis engine (eslint,heuristic,auto)', 'auto')
    .option('--dry-run', 'Show what would be analyzed without running')
    .option('--verbose', 'Enable verbose logging')
    .option('--quiet', 'Suppress non-error output')
    .option('--debug', 'Enable debug mode')
    .option('--ai', 'Enable AI-powered analysis')
    .option('--no-ai', 'Force disable AI analysis')
    .option('--max-semantic-files <number>', 'Symbol table file limit for TypeScript analysis (default: auto)', '0')
    .option('--list-engines', 'List available analysis engines');

  // ESLint Integration options
  program
    .option('--eslint-integration', 'Enable ESLint integration (merge with existing ESLint config)')
    .option('--no-eslint-integration', 'Disable ESLint integration')
    .option('--eslint-merge-rules', 'Merge SunLint and user ESLint rules (default: true)')
    .option('--eslint-preserve-config', 'Preserve user ESLint configuration (default: true)')
    .option('--eslint-run-after', 'Run ESLint after SunLint (instead of merged execution)');

  // Help examples
  program.addHelpText('after', `
Examples:
  $ sunlint --rule=C019 --input=src
  $ sunlint --rule C019 --input src
  $ sunlint --rules=C019,C006,S005 --input=src
  $ sunlint --all --input=src
  $ sunlint --quality --input=src
  $ sunlint --security --input=src
  $ sunlint --category=logging --input=src

File Targeting:
  $ sunlint --all --include="src/**/*.ts" --exclude="**/*.test.*" --input=.

Performance (SIMPLIFIED):
  $ sunlint --all --input=src --performance=auto        # Auto-detect best settings
  $ sunlint --all --input=src --performance=fast        # Quick scan
  $ sunlint --all --input=src --performance=careful     # Thorough analysis
  $ sunlint --all --input=src --timeout=60000           # Custom timeout (60s)

File Limits (when needed):
  $ sunlint --all --input=src --max-files=500           # Limit total files analyzed
  $ sunlint --all --input=src --max-semantic-files=200  # Limit TypeScript symbol table
  $ sunlint --all --languages=typescript,dart --input=src
  $ sunlint --typescript --exclude-tests --input=src
  $ sunlint --all --only-source --include="src/**,lib/**" --input=.

TypeScript Analysis (Phase 1):
  $ sunlint --typescript --input=src
  $ sunlint --rule=C006 --typescript --input=src
  $ sunlint --rules=C019,S005 --typescript --input=src
  $ sunlint --typescript-engine=eslint --input=src

Version Strategy:
  v1.x: ESLint-first with SunLint fallback (current)
  v2.x: SunLint-first with ESLint integration (--eslint-integration)

Engine Configuration:
  $ sunlint --all --input=src                    # Use config engine setting
  $ sunlint --all --input=src --engine=eslint    # Force ESLint engine 
  $ sunlint --all --input=src --engine=heuristic # Force Heuristic engine

CI/CD Integration:
  $ sunlint --all --changed-files --format=summary --no-ai
  $ sunlint --all --changed-files --diff-base=origin/main --fail-on-new-violations
  $ sunlint --all --staged-files --format=summary
  $ sunlint --all --pr-mode --diff-base=origin/main

ESLint Integration:
  $ sunlint --typescript --eslint-integration --input=src
  $ sunlint --all --eslint-integration --eslint-merge-rules --input=src
  $ sunlint --all --eslint-integration --eslint-run-after --input=src
  $ sunlint --typescript --eslint-integration --changed-files

Advanced File Targeting:
  $ sunlint --all --include="src/**/*.ts,lib/**/*.dart" --exclude="**/*.generated.*" --input=.
  $ sunlint --typescript --exclude="**/*.d.ts,**/*.test.*" --input=src
  $ sunlint --languages=typescript,dart --include="src/**,packages/**" --input=.
  $ sunlint --all --only-source --exclude-tests --languages=typescript --input=.

Large Project Optimization:
  $ sunlint --all --input=. --max-semantic-files=500    # Conservative analysis
  $ sunlint --all --input=. --max-semantic-files=2000   # Comprehensive analysis  
  $ sunlint --all --input=. --max-semantic-files=-1     # Unlimited (all files)
  $ sunlint --all --input=. --max-semantic-files=0      # Disable semantic analysis
  $ sunlint --all --changed-files --max-semantic-files=300  # Fast CI analysis

Sun* Engineering - Coding Standards Made Simple ☀️
`);

  return program;
}

module.exports = { createCliProgram };
