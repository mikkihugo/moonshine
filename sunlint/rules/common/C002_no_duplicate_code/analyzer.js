/**
 * C002_no_duplicate_code - Enhanced Regex-based Rule Analyzer
 * Category: coding
 * 
 * Detects duplicate code blocks longer than specified threshold (default: 10 lines)
 * Uses regex-based approach with proper comment filtering for multi-language support
 */

const fs = require('fs');
const path = require('path');
const { CommentDetector } = require('../../utils/rule-helpers');

class C002_no_duplicate_codeAnalyzer {
    constructor(config = {}) {
        this.config = {
            minLines: config.minLines || 5,
            ignoreComments: config.ignoreComments !== false,
            ignoreWhitespace: config.ignoreWhitespace !== false,
            ignoreEmptyLines: config.ignoreEmptyLines !== false,
            similarityThreshold: config.similarityThreshold || 0.80, // 80% similarity
            ...config
        };
        this.codeBlocks = new Map();
        this.reportedBlocks = new Set();
    }

    /**
     * Analyze files for duplicate code violations (heuristic engine interface)
     * @param {Array} files - Array of file paths
     * @param {string} language - Programming language
     * @param {Object} options - Analysis options
     * @returns {Array} Array of violations
     */
    analyze(files, language, options = {}) {
        const violations = [];

        try {
                console.log(`[C002 DEBUG] Analyzing ${files.length} files for duplicate code`);
            
            // Reset state for new analysis
            this.reset();
            
            // Collect all code blocks from all files
            const allCodeBlocks = [];
            
            for (const filePath of files) {
                console.log(`[C002 DEBUG] Processing file: ${filePath}`);
                const content = this.readFileContent(filePath);
                if (content) {
                    console.log(`[C002 DEBUG] File content length: ${content.length}`);
                    const codeBlocks = this.extractCodeBlocks(content, filePath);
                    console.log(`[C002 DEBUG] Extracted ${codeBlocks.length} code blocks from ${filePath}`);
                    codeBlocks.forEach((block, i) => {
                        console.log(`[C002 DEBUG] Block ${i}: ${block.type} at lines ${block.startLine}-${block.endLine} (${block.lineCount} lines)`);
                    });
                    allCodeBlocks.push(...codeBlocks);
                }
            }
            
            console.log(`[C002 DEBUG] Total code blocks: ${allCodeBlocks.length}`);
            
            // Find duplicates across all files
            const duplicates = this.findDuplicates(allCodeBlocks);
            console.log(`[C002 DEBUG] Found ${duplicates.length} duplicate groups`);
            
            // Generate violations for each file
            files.forEach(filePath => {
                duplicates.forEach(duplicate => {
                    const fileViolations = this.createViolations(duplicate, filePath);
                    console.log(`[C002 DEBUG] Created ${fileViolations.length} violations for ${filePath}`);
                    violations.push(...fileViolations);
                });
            });

        } catch (error) {
            console.warn(`Error analyzing files with C002:`, error.message, error.stack);
        }

        console.log(`[C002 DEBUG] Total violations: ${violations.length}`);
        return violations;
    }

    /**
     * Read file content safely
     * @param {string} filePath - Path to file
     * @returns {string|null} File content or null if error
     */
    readFileContent(filePath) {
        try {
            return fs.readFileSync(filePath, 'utf8');
        } catch (error) {
            console.warn(`C002: Cannot read file ${filePath}:`, error.message);
            return null;
        }
    }

    /**
     * Extract code blocks from content
     * @param {string} content - File content
     * @param {string} filePath - File path for context
     * @returns {Array} Array of code blocks with metadata
     */
    extractCodeBlocks(content, filePath) {
        const lines = content.split('\n');
        const blocks = [];
        
        // Extract function blocks, class methods, etc.
        const functionPattern = /^\s*(function\s+\w+|const\s+\w+\s*=\s*(async\s+)?\([^)]*\)\s*=>|class\s+\w+|\w+\s*\([^)]*\)\s*:\s*[^{]*\{)/;
        let currentBlock = null;
        let braceLevel = 0;
        
        lines.forEach((line, index) => {
            const lineNum = index + 1;
            const trimmedLine = line.trim();
            
            // Use CommentDetector to filter out comments
            const filteredLines = CommentDetector.filterCommentLines([line]);
            if (filteredLines[0].isComment) {
                return;
            }
            
            // Skip empty lines if configured
            if (this.config.ignoreEmptyLines && !trimmedLine) {
                return;
            }
            
            // Detect function/method/class start
            if (functionPattern.test(trimmedLine)) {
                currentBlock = {
                    startLine: lineNum,
                    lines: [line],
                    filePath: filePath,
                    type: this.detectBlockType(trimmedLine)
                };
                braceLevel = (line.match(/{/g) || []).length - (line.match(/}/g) || []).length;
            } else if (currentBlock) {
                currentBlock.lines.push(line);
                braceLevel += (line.match(/{/g) || []).length - (line.match(/}/g) || []).length;
                
                // End of block
                if (braceLevel <= 0) {
                    currentBlock.endLine = lineNum;
                    currentBlock.lineCount = currentBlock.lines.length;
                    
                    // Only consider blocks that meet minimum line requirement
                    if (currentBlock.lineCount >= this.config.minLines) {
                        currentBlock.normalizedCode = this.normalizeCode(currentBlock.lines.join('\n'));
                        if (currentBlock.normalizedCode.length > 20) { // Skip if too short after normalization
                            blocks.push(currentBlock);
                        }
                    }
                    currentBlock = null;
                    braceLevel = 0;
                }
            }
        });
        
        return blocks;
    }

    /**
     * Detect the type of code block
     * @param {string} line - First line of the block
     * @returns {string} Block type
     */
    detectBlockType(line) {
        if (line.includes('function')) return 'function';
        if (line.includes('class')) return 'class';
        if (line.includes('interface')) return 'interface';
        if (line.includes('=>')) return 'arrow-function';
        return 'method';
    }

    /**
     * Normalize code for comparison
     * @param {string} code - Raw code
     * @returns {string} Normalized code
     */
    normalizeCode(code) {
        let normalized = code;
        
        if (this.config.ignoreComments) {
            // Remove single line comments (// comments)
            normalized = normalized.replace(/\/\/.*$/gm, '');
            // Remove multi-line comments (/* comments */)
            normalized = normalized.replace(/\/\*[\s\S]*?\*\//g, '');
            // Remove # comments (for other languages)
            normalized = normalized.replace(/#.*$/gm, '');
        }
        
        if (this.config.ignoreWhitespace) {
            // Normalize whitespace
            normalized = normalized
                .replace(/\s+/g, ' ')           // Multiple spaces to single space
                .replace(/\s*{\s*/g, '{')       // Remove spaces around braces
                .replace(/\s*}\s*/g, '}')
                .replace(/\s*;\s*/g, ';')       // Remove spaces around semicolons
                .replace(/\s*,\s*/g, ',')       // Remove spaces around commas
                .trim();
        }
        
        if (this.config.ignoreEmptyLines) {
            // Remove empty lines
            normalized = normalized
                .split('\n')
                .filter(line => line.trim().length > 0)
                .join('\n');
        }
        
        console.log(`[C002 DEBUG] Normalized code block:
${normalized}
---`);
        
        return normalized;
    }

    /**
     * Find duplicate code blocks
     * @param {Array} blocks - Array of code blocks
     * @returns {Array} Array of duplicate groups
     */
    findDuplicates(blocks) {
        const duplicateGroups = [];
        const processedBlocks = new Set();
        
        for (let i = 0; i < blocks.length; i++) {
            if (processedBlocks.has(i)) continue;
            
            const currentBlock = blocks[i];
            const duplicates = [currentBlock];
            
            for (let j = i + 1; j < blocks.length; j++) {
                if (processedBlocks.has(j)) continue;
                
                const otherBlock = blocks[j];
                const similarity = this.calculateSimilarity(
                    currentBlock.normalizedCode, 
                    otherBlock.normalizedCode
                );
                
                if (similarity >= this.config.similarityThreshold) {
                    duplicates.push(otherBlock);
                    processedBlocks.add(j);
                }
            }
            
            if (duplicates.length > 1) {
                duplicateGroups.push(duplicates);
                processedBlocks.add(i);
            }
        }
        
        return duplicateGroups;
    }

    /**
     * Calculate similarity between two code strings
     * @param {string} code1 - First code string
     * @param {string} code2 - Second code string
     * @returns {number} Similarity ratio (0-1)
     */
    calculateSimilarity(code1, code2) {
        if (code1 === code2) return 1.0;
        
        // Use Levenshtein distance for similarity calculation
        const longer = code1.length > code2.length ? code1 : code2;
        const shorter = code1.length > code2.length ? code2 : code1;
        
        if (longer.length === 0) return 1.0;
        
        const distance = this.levenshteinDistance(longer, shorter);
        return (longer.length - distance) / longer.length;
    }

    /**
     * Calculate Levenshtein distance between two strings
     * @param {string} str1 - First string
     * @param {string} str2 - Second string
     * @returns {number} Edit distance
     */
    levenshteinDistance(str1, str2) {
        const matrix = Array(str2.length + 1).fill().map(() => Array(str1.length + 1).fill(0));
        
        for (let i = 0; i <= str1.length; i++) matrix[0][i] = i;
        for (let j = 0; j <= str2.length; j++) matrix[j][0] = j;
        
        for (let j = 1; j <= str2.length; j++) {
            for (let i = 1; i <= str1.length; i++) {
                const cost = str1[i - 1] === str2[j - 1] ? 0 : 1;
                matrix[j][i] = Math.min(
                    matrix[j - 1][i] + 1,     // deletion
                    matrix[j][i - 1] + 1,     // insertion
                    matrix[j - 1][i - 1] + cost // substitution
                );
            }
        }
        
        return matrix[str2.length][str1.length];
    }

    /**
     * Create violation objects for duplicate code
     * @param {Array} duplicateGroup - Group of duplicate blocks
     * @param {string} filePath - Current file path
     * @returns {Array} Array of violation objects
     */
    createViolations(duplicateGroup, filePath) {
        const violations = [];
        
        duplicateGroup.forEach((block, index) => {
            // Skip if not in current file or already reported
            if (block.filePath !== filePath) return;
            
            const blockId = `${block.filePath}:${block.startLine}-${block.endLine}`;
            if (this.reportedBlocks.has(blockId)) return;
            
            this.reportedBlocks.add(blockId);
            
            violations.push({
                ruleId: 'C002',
                severity: 'error',
                message: `Duplicate ${block.type} found (${block.lineCount} lines). Consider extracting into a shared function or module. Found ${duplicateGroup.length} similar blocks.`,
                line: block.startLine,
                column: 1,
                endLine: block.endLine,
                endColumn: 1,
                filePath: filePath, // Add filePath field for engine compatibility
                data: {
                    lineCount: block.lineCount,
                    blockType: block.type,
                    duplicateCount: duplicateGroup.length,
                    locations: duplicateGroup.map(b => `${path.basename(b.filePath)}:${b.startLine}-${b.endLine}`)
                }
            });
        });
        
        return violations;
    }

    /**
     * Reset analyzer state for new analysis session
     */
    reset() {
        this.codeBlocks.clear();
        this.reportedBlocks.clear();
    }

    /**
     * Get configuration for this rule
     * @returns {Object} Configuration object
     */
    getConfig() {
        return {
            minLines: this.config.minLines,
            ignoreComments: this.config.ignoreComments,
            ignoreWhitespace: this.config.ignoreWhitespace,
            ignoreEmptyLines: this.config.ignoreEmptyLines,
            similarityThreshold: this.config.similarityThreshold
        };
    }
}

module.exports = C002_no_duplicate_codeAnalyzer;
