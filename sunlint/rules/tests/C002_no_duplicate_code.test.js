/**
 * C002_no_duplicate_code - Rule Tests
 * Tests for heuristic rule analyzer
 */

const C002_no_duplicate_codeAnalyzer = require('./analyzer');

describe('C002_no_duplicate_code Heuristic Rule', () => {
    let analyzer;

    beforeEach(() => {
        analyzer = new C002_no_duplicate_codeAnalyzer();
    });

    describe('Valid Code', () => {
        test('should not report violations for valid code', () => {
            const code = `
                // TODO: Add valid code examples
            `;

            const violations = analyzer.analyze(code, 'test.js');
            expect(violations).toHaveLength(0);
        });
    });

    describe('Invalid Code', () => {
        test('should report violations for invalid code', () => {
            const code = `
                // TODO: Add invalid code examples
            `;

            const violations = analyzer.analyze(code, 'test.js');
            expect(violations.length).toBeGreaterThan(0);
            expect(violations[0].ruleId).toBe('C002_no_duplicate_code');
        });
    });

    describe('Edge Cases', () => {
        test('should handle empty code', () => {
            const violations = analyzer.analyze('', 'test.js');
            expect(violations).toHaveLength(0);
        });

        test('should handle syntax errors gracefully', () => {
            const code = 'invalid javascript syntax {{{';
            const violations = analyzer.analyze(code, 'test.js');
            expect(Array.isArray(violations)).toBe(true);
        });
    });
});
