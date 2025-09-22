//! # Security Module Tests
//!
//! Comprehensive tests for AST-based security vulnerability detection.
//!
//! @category testing
//! @safe program
//! @complexity high
//! @since 2.0.0

use super::*;
use crate::testing::builders::AiSuggestionBuilder;
use crate::testing::fixtures::{JAVASCRIPT_WITH_ISSUES, TYPESCRIPT_WITH_ISSUES};

#[test]
fn test_security_analyzer_creation() {
    let analyzer = SecurityAnalyzer::new();
    assert!(!analyzer.rules.is_empty());
    assert!(analyzer.rules.len() >= 10); // Should have multiple security rules
}

#[test]
fn test_xss_vulnerability_detection() {
    let analyzer = SecurityAnalyzer::new();

    let vulnerable_code = r#"
        const userInput = req.body.content;
        document.innerHTML = userInput; // XSS vulnerability
        element.outerHTML = "<div>" + userInput + "</div>"; // Another XSS
    "#;

    let vulnerabilities = analyzer.scan_for_vulnerabilities(vulnerable_code, "test.js");
    assert!(!vulnerabilities.is_empty());

    let xss_vulns: Vec<_> = vulnerabilities.iter().filter(|v| v.vulnerability_type == VulnerabilityType::XSS).collect();
    assert!(!xss_vulns.is_empty());
}

#[test]
fn test_sql_injection_detection() {
    let analyzer = SecurityAnalyzer::new();

    let vulnerable_code = r#"
        const userId = req.params.id;
        const query = "SELECT * FROM users WHERE id = " + userId; // SQL Injection
        db.query(query);

        const email = req.body.email;
        connection.execute(`DELETE FROM users WHERE email = '${email}'`); // Template injection
    "#;

    let vulnerabilities = analyzer.scan_for_vulnerabilities(vulnerable_code, "test.js");

    let sql_vulns: Vec<_> = vulnerabilities
        .iter()
        .filter(|v| v.vulnerability_type == VulnerabilityType::SQLInjection)
        .collect();
    assert!(!sql_vulns.is_empty());
}

#[test]
fn test_csrf_vulnerability_detection() {
    let analyzer = SecurityAnalyzer::new();

    let vulnerable_code = r#"
        app.post('/transfer', (req, res) => {
            // Missing CSRF protection
            const amount = req.body.amount;
            const recipient = req.body.recipient;
            transferMoney(amount, recipient);
        });

        fetch('/api/delete-account', {
            method: 'POST',
            // Missing CSRF token
            body: JSON.stringify(data)
        });
    "#;

    let vulnerabilities = analyzer.scan_for_vulnerabilities(vulnerable_code, "test.js");

    let csrf_vulns: Vec<_> = vulnerabilities.iter().filter(|v| v.vulnerability_type == VulnerabilityType::CSRF).collect();
    assert!(!csrf_vulns.is_empty());
}

#[test]
fn test_insecure_random_detection() {
    let analyzer = SecurityAnalyzer::new();

    let vulnerable_code = r#"
        const sessionId = Math.random().toString(36); // Insecure random
        const token = Math.floor(Math.random() * 1000000); // Predictable token

        // Should suggest crypto.randomBytes or similar
    "#;

    let vulnerabilities = analyzer.scan_for_vulnerabilities(vulnerable_code, "test.js");

    let random_vulns: Vec<_> = vulnerabilities
        .iter()
        .filter(|v| v.vulnerability_type == VulnerabilityType::InsecureRandom)
        .collect();
    assert!(!random_vulns.is_empty());
}

#[test]
fn test_hardcoded_secrets_detection() {
    let analyzer = SecurityAnalyzer::new();

    let vulnerable_code = r#"
        const API_KEY = "sk-1234567890abcdef"; // Hardcoded API key
        const PASSWORD = "admin123"; // Hardcoded password
        const connectionString = "mongodb://user:password@localhost/db"; // Hardcoded credentials

        const config = {
            aws_access_key: "AKIAIOSFODNN7EXAMPLE", // AWS key pattern
            secret: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
        };
    "#;

    let vulnerabilities = analyzer.scan_for_vulnerabilities(vulnerable_code, "test.js");

    let secret_vulns: Vec<_> = vulnerabilities
        .iter()
        .filter(|v| v.vulnerability_type == VulnerabilityType::HardcodedSecrets)
        .collect();
    assert!(!secret_vulns.is_empty());
    assert!(secret_vulns.len() >= 3); // Should detect multiple secrets
}

#[test]
fn test_path_traversal_detection() {
    let analyzer = SecurityAnalyzer::new();

    let vulnerable_code = r#"
        const fileName = req.params.file;
        const filePath = path.join(__dirname, fileName); // Path traversal risk
        fs.readFile(filePath, callback);

        app.get('/download/:filename', (req, res) => {
            const filename = req.params.filename;
            res.sendFile(__dirname + '/' + filename); // Direct path concatenation
        });
    "#;

    let vulnerabilities = analyzer.scan_for_vulnerabilities(vulnerable_code, "test.js");

    let path_vulns: Vec<_> = vulnerabilities
        .iter()
        .filter(|v| v.vulnerability_type == VulnerabilityType::PathTraversal)
        .collect();
    assert!(!path_vulns.is_empty());
}

#[test]
fn test_typescript_specific_vulnerabilities() {
    let analyzer = SecurityAnalyzer::new();

    let vulnerable_code = r#"
        declare global {
            interface Window {
                myGlobalVar: any; // Global pollution
            }
        }

        // Type assertion bypassing safety
        const userData = req.body as UserData;

        // Unsafe any usage in security context
        function authenticateUser(credentials: any): boolean {
            return credentials.password === "admin";
        }

        // Prototype pollution risk
        function merge(target: any, source: any) {
            for (let key in source) {
                target[key] = source[key]; // No prototype check
            }
        }
    "#;

    let vulnerabilities = analyzer.scan_for_vulnerabilities(vulnerable_code, "test.ts");
    assert!(!vulnerabilities.is_empty());

    // Should detect prototype pollution and type safety issues
    let proto_vulns: Vec<_> = vulnerabilities
        .iter()
        .filter(|v| v.vulnerability_type == VulnerabilityType::PrototypePollution)
        .collect();
    assert!(!proto_vulns.is_empty());
}

#[test]
fn test_vulnerability_severity_classification() {
    let analyzer = SecurityAnalyzer::new();

    let code_with_mixed_severities = r#"
        // Critical: SQL injection
        const query = "SELECT * FROM users WHERE id = " + userId;

        // High: XSS vulnerability
        document.innerHTML = userInput;

        // Medium: Insecure random
        const token = Math.random().toString();

        // Low: Missing security header
        res.setHeader('X-Frame-Options', 'SAMEORIGIN');
    "#;

    let vulnerabilities = analyzer.scan_for_vulnerabilities(code_with_mixed_severities, "test.js");

    // Should have vulnerabilities of different severities
    let critical = vulnerabilities.iter().filter(|v| v.severity == VulnerabilitySeverity::Critical).count();
    let high = vulnerabilities.iter().filter(|v| v.severity == VulnerabilitySeverity::High).count();
    let medium = vulnerabilities.iter().filter(|v| v.severity == VulnerabilitySeverity::Medium).count();

    assert!(critical > 0);
    assert!(high > 0);
    assert!(medium > 0);
}

#[test]
fn test_security_suggestions_generation() {
    let analyzer = SecurityAnalyzer::new();

    let vulnerable_code = r#"
        const password = "hardcoded123";
        document.innerHTML = userInput;
    "#;

    let vulnerabilities = analyzer.scan_for_vulnerabilities(vulnerable_code, "test.js");

    for vuln in &vulnerabilities {
        assert!(!vuln.description.is_empty());
        assert!(!vuln.recommendation.is_empty());
        assert!(vuln.line_number > 0);

        // Should provide actionable recommendations
        assert!(vuln.recommendation.contains("Use") || vuln.recommendation.contains("Implement") || vuln.recommendation.contains("Replace"));
    }
}

#[test]
fn test_false_positive_filtering() {
    let analyzer = SecurityAnalyzer::new();

    let safe_code = r#"
        // These should NOT trigger vulnerabilities

        // Safe DOM manipulation
        element.textContent = userInput;

        // Parameterized query
        const query = "SELECT * FROM users WHERE id = ?";
        db.query(query, [userId]);

        // Crypto random
        const token = crypto.randomBytes(32).toString('hex');

        // Environment variable
        const apiKey = process.env.API_KEY;

        // Safe path handling
        const safePath = path.resolve(publicDir, path.normalize(fileName));
        if (!safePath.startsWith(publicDir)) {
            throw new Error('Invalid path');
        }
    "#;

    let vulnerabilities = analyzer.scan_for_vulnerabilities(safe_code, "test.js");

    // Should have very few or no false positives
    assert!(vulnerabilities.len() <= 1); // Allow for potential edge cases
}

#[test]
fn test_security_rule_customization() {
    let mut analyzer = SecurityAnalyzer::new();

    // Add custom security rule
    analyzer.add_custom_rule(SecurityRule {
        name: "custom-test-rule".to_string(),
        description: "Test custom security rule".to_string(),
        severity: VulnerabilitySeverity::Medium,
        pattern: r"\.dangerous\(\)".to_string(),
        vulnerability_type: VulnerabilityType::Other("CustomVulnerability".to_string()),
        recommendation: "Avoid using dangerous() method".to_string(),
    });

    let code_with_custom_vuln = r#"
        something.dangerous(); // Should trigger custom rule
        safeThing.safe(); // Should not trigger
    "#;

    let vulnerabilities = analyzer.scan_for_vulnerabilities(code_with_custom_vuln, "test.js");

    let custom_vulns: Vec<_> = vulnerabilities
        .iter()
        .filter(|v| matches!(v.vulnerability_type, VulnerabilityType::Other(ref s) if s == "CustomVulnerability"))
        .collect();
    assert!(!custom_vulns.is_empty());
}

#[test]
fn test_security_reporting() {
    let analyzer = SecurityAnalyzer::new();

    let vulnerable_code = TYPESCRIPT_WITH_ISSUES;
    let vulnerabilities = analyzer.scan_for_vulnerabilities(vulnerable_code, "test.ts");

    let report = analyzer.generate_security_report(&vulnerabilities);

    assert!(!report.summary.is_empty());
    assert!(report.total_vulnerabilities >= vulnerabilities.len());
    assert!(report.critical_count + report.high_count + report.medium_count + report.low_count == report.total_vulnerabilities);

    // Should include recommendations
    assert!(!report.recommendations.is_empty());
}

#[test]
fn test_security_integration_with_ai_suggestions() {
    let analyzer = SecurityAnalyzer::new();

    let vulnerable_code = r#"
        const query = "SELECT * FROM users WHERE email = '" + email + "'";
        document.innerHTML = content;
    "#;

    let vulnerabilities = analyzer.scan_for_vulnerabilities(vulnerable_code, "test.js");

    // Convert to AI suggestions
    let ai_suggestions: Vec<_> = vulnerabilities
        .iter()
        .map(|vuln| {
            AiSuggestionBuilder::error()
                .message(&format!("Security: {}", vuln.description))
                .file_path("test.js")
                .line_number(vuln.line_number)
                .category(crate::linter::SuggestionCategory::Security)
                .confidence_score(0.95)
                .build()
        })
        .collect();

    assert_eq!(ai_suggestions.len(), vulnerabilities.len());

    for suggestion in &ai_suggestions {
        assert!(suggestion.message.starts_with("Security:"));
        assert!(matches!(suggestion.category, crate::linter::SuggestionCategory::Security));
        assert!(suggestion.confidence_score > 0.9);
    }
}

#[test]
fn test_performance_with_large_files() {
    let analyzer = SecurityAnalyzer::new();

    // Create large file content
    let large_content = (0..1000)
        .map(|i| {
            format!(
                r#"
            function func{}() {{
                const data = "some data {}";
                return process(data);
            }}
        "#,
                i, i
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let start_time = std::time::Instant::now();
    let vulnerabilities = analyzer.scan_for_vulnerabilities(&large_content, "large.js");
    let execution_time = start_time.elapsed();

    // Should complete in reasonable time even for large files
    assert!(execution_time.as_millis() < 5000); // 5 seconds max

    // May or may not find vulnerabilities, but should not crash
    assert!(vulnerabilities.len() >= 0);
}

#[test]
fn test_concurrent_security_analysis() {
    let analyzer = std::sync::Arc::new(SecurityAnalyzer::new());

    let test_codes = vec![
        "const x = Math.random();",
        "document.innerHTML = input;",
        "const query = 'SELECT * FROM users WHERE id = ' + id;",
        "const key = 'hardcoded123';",
    ];

    let handles: Vec<_> = test_codes
        .into_iter()
        .enumerate()
        .map(|(i, code)| {
            let analyzer = analyzer.clone();
            let code = code.to_string();
            std::thread::spawn(move || analyzer.scan_for_vulnerabilities(&code, &format!("test{}.js", i)))
        })
        .collect();

    // Wait for all threads to complete
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All should complete successfully
    assert_eq!(results.len(), 4);

    // Should find at least some vulnerabilities across all tests
    let total_vulns: usize = results.iter().map(|r| r.len()).sum();
    assert!(total_vulns > 0);
}
