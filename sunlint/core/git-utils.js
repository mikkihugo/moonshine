/**
 * Git Utils - Handle git operations for file filtering
 * Following Rule C005: Single responsibility - only handle git operations
 * Following Rule C014: Dependency injection for git operations
 */

const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

class GitUtils {
  /**
   * Check if current directory is a git repository
   */
  static isGitRepository(cwd = process.cwd()) {
    try {
      execSync('git rev-parse --git-dir', { cwd, stdio: 'ignore' });
      return true;
    } catch (error) {
      return false;
    }
  }

  /**
   * Get list of changed files compared to base reference
   * @param {string} baseRef - Base git reference (e.g., 'origin/main')
   * @param {string} cwd - Working directory
   * @returns {string[]} Array of changed file paths
   */
  static getChangedFiles(baseRef = 'HEAD', cwd = process.cwd()) {
    if (!this.isGitRepository(cwd)) {
      throw new Error('Not a git repository');
    }

    try {
      // Get git root directory
      const gitRoot = execSync('git rev-parse --show-toplevel', { cwd, encoding: 'utf8' }).trim();
      
      const command = `git diff --name-only ${baseRef}`;
      const output = execSync(command, { cwd: gitRoot, encoding: 'utf8' });
      
      return output
        .split('\n')
        .filter(file => file.trim() !== '')
        .map(file => path.resolve(gitRoot, file))
        .filter(file => fs.existsSync(file)); // Only existing files
    } catch (error) {
      throw new Error(`Failed to get changed files: ${error.message}`);
    }
  }

  /**
   * Get list of staged files
   * @param {string} cwd - Working directory
   * @returns {string[]} Array of staged file paths
   */
  static getStagedFiles(cwd = process.cwd()) {
    if (!this.isGitRepository(cwd)) {
      throw new Error('Not a git repository');
    }

    try {
      // Get git root directory
      const gitRoot = execSync('git rev-parse --show-toplevel', { cwd, encoding: 'utf8' }).trim();
      
      const command = 'git diff --cached --name-only';
      const output = execSync(command, { cwd: gitRoot, encoding: 'utf8' });
      
      return output
        .split('\n')
        .filter(file => file.trim() !== '')
        .map(file => path.resolve(gitRoot, file))
        .filter(file => fs.existsSync(file));
    } catch (error) {
      throw new Error(`Failed to get staged files: ${error.message}`);
    }
  }

  /**
   * Get files changed since specific commit
   * @param {string} commit - Commit hash or reference
   * @param {string} cwd - Working directory
   * @returns {string[]} Array of changed file paths
   */
  static getFilesSince(commit, cwd = process.cwd()) {
    if (!this.isGitRepository(cwd)) {
      throw new Error('Not a git repository');
    }

    try {
      // Get git root directory
      const gitRoot = execSync('git rev-parse --show-toplevel', { cwd, encoding: 'utf8' }).trim();
      
      const command = `git diff --name-only ${commit}..HEAD`;
      const output = execSync(command, { cwd: gitRoot, encoding: 'utf8' });
      
      return output
        .split('\n')
        .filter(file => file.trim() !== '')
        .map(file => path.resolve(gitRoot, file))
        .filter(file => fs.existsSync(file));
    } catch (error) {
      throw new Error(`Failed to get files since ${commit}: ${error.message}`);
    }
  }

  /**
   * Get current branch name
   * @param {string} cwd - Working directory
   * @returns {string} Current branch name
   */
  static getCurrentBranch(cwd = process.cwd()) {
    if (!this.isGitRepository(cwd)) {
      throw new Error('Not a git repository');
    }

    try {
      const command = 'git rev-parse --abbrev-ref HEAD';
      const output = execSync(command, { cwd, encoding: 'utf8' });
      return output.trim();
    } catch (error) {
      throw new Error(`Failed to get current branch: ${error.message}`);
    }
  }

  /**
   * Filter TypeScript/JavaScript files from file list
   * @param {string[]} files - Array of file paths
   * @returns {string[]} Filtered TypeScript/JavaScript files
   */
  static filterSourceFiles(files) {
    const extensions = ['.ts', '.tsx', '.js', '.jsx'];
    return files.filter(file => {
      const ext = path.extname(file);
      return extensions.includes(ext);
    });
  }

  /**
   * Get git diff base reference for PR mode
   * @param {string} targetBranch - Target branch (e.g., 'main', 'develop')
   * @param {string} cwd - Working directory
   * @returns {string} Git reference for comparison
   */
  static getPRDiffBase(targetBranch = 'main', cwd = process.cwd()) {
    if (!this.isGitRepository(cwd)) {
      throw new Error('Not a git repository');
    }

    // Try common remote references
    const candidates = [
      `origin/${targetBranch}`,
      `upstream/${targetBranch}`,
      targetBranch
    ];

    for (const candidate of candidates) {
      try {
        execSync(`git rev-parse --verify ${candidate}`, { cwd, stdio: 'ignore' });
        return candidate;
      } catch (error) {
        // Continue to next candidate
      }
    }

    throw new Error(`No valid git reference found for target branch: ${targetBranch}`);
  }
}

module.exports = GitUtils;
