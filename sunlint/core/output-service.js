/**
 * Output Service
 * Following Rule C005: Single responsibility - handle output operations
 */

const fs = require('fs');
const path = require('path');
const chalk = require('chalk');

class OutputService {
  constructor() {}

  async outputResults(results, options, metadata = {}) {
    // Generate report based on format
    const report = this.generateReport(results, metadata, options);

    // Console output
    if (!options.quiet) {
      console.log(report.formatted);
    }

    // File output
    if (options.output) {
      const outputData = options.format === 'json' ? report.raw : report.formatted;
      const content = typeof outputData === 'string' ? outputData : JSON.stringify(outputData, null, 2);
      fs.writeFileSync(options.output, content);
      console.log(chalk.green(`📄 Report saved to: ${options.output}`));
    }

    // Summary (skip for JSON format)
    if (!options.quiet && options.format !== 'json') {
      console.log(report.summary);
    }
  }

  generateReport(results, metadata, options = {}) {
    const allViolations = [];
    let totalFiles = results.filesAnalyzed || results.summary?.totalFiles || results.totalFiles || results.fileCount || 0;

    // Collect all violations - handle both file-based and rule-based results
    if (results.results) {
      results.results.forEach(result => {
        if (result.violations) {
          // Handle rule-based format (MultiRuleRunner)
          if (result.ruleId) {
            result.violations.forEach(violation => {
              allViolations.push(violation); // violation already has file path
            });
          } 
          // Handle file-based format (legacy)
          else {
            result.violations.forEach(violation => {
              allViolations.push({
                ...violation,
                file: result.filePath || result.file // Use filePath first, then file
              });
            });
          }
        }
        
        // Handle ESLint format (messages array)
        if (result.messages) {
          result.messages.forEach(message => {
            allViolations.push({
              file: result.filePath || message.file,
              ruleId: message.ruleId,
              severity: message.severity === 2 ? 'error' : 'warning',
              message: message.message,
              line: message.line,
              column: message.column,
              source: message.source || 'eslint'
            });
          });
        }
      });
    }

    // Generate output based on format
    let formatted;
    let raw;

    if (options.format === 'json') {
      // ESLint-compatible JSON format
      raw = this.generateJsonFormat(results, allViolations, options);
      formatted = JSON.stringify(raw, null, 2);
    } else {
      // Default text format
      formatted = this.formatViolations(allViolations);
      raw = {
        violations: allViolations,
        filesAnalyzed: totalFiles,
        metadata
      };
    }

    const summary = this.generateSummary(allViolations, totalFiles, metadata);

    return {
      formatted,
      summary,
      raw
    };
  }

  formatViolations(violations) {
    if (violations.length === 0) {
      return chalk.green('✅ No violations found!');
    }

    let output = '';
    const fileGroups = {};

    // Group violations by file
    violations.forEach(violation => {
      let file = violation.file || violation.filePath || 'unknown';
      
      // Convert absolute path to relative path for better display
      if (file !== 'unknown' && path.isAbsolute(file)) {
        const cwd = process.cwd();
        if (file.startsWith(cwd)) {
          file = path.relative(cwd, file);
        }
      }
      
      if (!fileGroups[file]) {
        fileGroups[file] = [];
      }
      fileGroups[file].push(violation);
    });

    // Format each file's violations (ESLint-compatible format)
    Object.keys(fileGroups).forEach(file => {
      output += `\n${chalk.underline(path.resolve(file))}\n`;
      fileGroups[file].forEach(violation => {
        const line = (violation.line || 1).toString();
        const column = (violation.column || 1).toString();
        const severityText = violation.severity === 'error' ? 'error' : 'warning';
        const severityColor = violation.severity === 'error' ? chalk.red : chalk.yellow;

        output += ` ${chalk.dim(`${line}:${column}`)} ${severityColor(severityText)}  ${violation.message}  ${chalk.gray(violation.ruleId)}\n`;
      });
    });

    // Add violation count (ESLint-compatible)
    const errorCount = violations.filter(v => v.severity === 'error').length;
    const warningCount = violations.filter(v => v.severity === 'warning').length;

    output += `\n${chalk.red('✖')} ${violations.length} problems `;
    output += `(${errorCount} errors, ${warningCount} warnings)\n`;

    return output;
  }

  generateSummary(violations, filesAnalyzed, metadata) {
    const duration = metadata.duration || 0;
    const errorCount = violations.filter(v => v.severity === 'error').length;
    const warningCount = violations.filter(v => v.severity === 'warning').length;

    let summary = chalk.blue('\n📊 Sun Lint Summary:\n');
    summary += `Analysis completed in ${duration}ms\n`;
    summary += `Files: ${filesAnalyzed} | Total: ${violations.length}\n`;
    
    if (errorCount > 0) {
      summary += chalk.red(`Errors: ${errorCount} `);
    }
    if (warningCount > 0) {
      summary += chalk.yellow(`Warnings: ${warningCount} `);
    }
    if (violations.length === 0) {
      summary += chalk.green('All checks passed! ✅');
    }

    return summary;
  }

  generateJsonFormat(results, allViolations, options = {}) {
    // ESLint-compatible JSON format
    const jsonResults = [];
    const fileGroups = {};

    // Group violations by file
    allViolations.forEach(violation => {
      let file = violation.file || violation.filePath || 'unknown';
      
      // Convert absolute path to relative path for better display
      if (file !== 'unknown' && path.isAbsolute(file)) {
        const cwd = process.cwd();
        if (file.startsWith(cwd)) {
          file = path.relative(cwd, file);
        }
      }
      
      if (!fileGroups[file]) {
        fileGroups[file] = [];
      }
      fileGroups[file].push(violation);
    });

    // Add files with violations
    Object.keys(fileGroups).forEach(filePath => {
      const messages = fileGroups[filePath].map(violation => ({
        ruleId: violation.ruleId,
        severity: violation.severity === 'error' ? 2 : 1, // ESLint: 1=warning, 2=error
        message: violation.message,
        line: violation.line || 1,
        column: violation.column || 1,
        nodeType: violation.nodeType || null,
        messageId: violation.messageId || null,
        endLine: violation.endLine || null,
        endColumn: violation.endColumn || null
      }));

      jsonResults.push({
        filePath: filePath,
        messages: messages,
        suppressedMessages: [],
        errorCount: messages.filter(m => m.severity === 2).length,
        warningCount: messages.filter(m => m.severity === 1).length,
        fatalErrorCount: 0,
        fixableErrorCount: 0,
        fixableWarningCount: 0,
        source: null
      });
    });

    // Add files without violations (if any were analyzed)
    if (results.results) {
      results.results.forEach(fileResult => {
        if (!fileGroups[fileResult.file] && fileResult.violations.length === 0) {
          jsonResults.push({
            filePath: fileResult.file,
            messages: [],
            suppressedMessages: [],
            errorCount: 0,
            warningCount: 0,
            fatalErrorCount: 0,
            fixableErrorCount: 0,
            fixableWarningCount: 0,
            source: null
          });
        }
      });
    }

    return jsonResults;
  }
}

module.exports = OutputService;
