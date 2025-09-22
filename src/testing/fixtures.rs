//! # Test Fixtures and Sample Data
//!
//! Realistic test data and sample code files for comprehensive testing.
//! Provides consistent test scenarios across all test types.
//!
//! @category testing
//! @safe program
//! @complexity medium
//! @since 2.0.0

use std::collections::HashMap;

/// TypeScript code sample with various issues for testing
pub const TYPESCRIPT_WITH_ISSUES: &str = r#"
import React, { useState, useEffect } from 'react';

// Issue: any type usage
interface UserProps {
    data: any;
    callback?: any;
}

// Issue: console.log usage
const UserComponent: React.FC<UserProps> = ({ data, callback }) => {
    const [state, setState] = useState<any>(null);

    useEffect(() => {
        console.log('Component mounted with data:', data);

        // Issue: unhandled promise
        fetchUserData().then(result => {
            setState(result);
        });
    }, [data]);

    // Issue: non-null assertion without proper check
    const handleClick = () => {
        callback!();
        console.log('Button clicked');
    };

    return (
        <div>
            <h1>{data.title}</h1>
            <p>{state?.description || 'Loading...'}</p>
            <button onClick={handleClick}>Click me</button>
        </div>
    );
};

async function fetchUserData(): Promise<any> {
    // Issue: any return type
    const response = await fetch('/api/user');
    return response.json();
}

export default UserComponent;
"#;

/// Clean TypeScript code sample without issues
pub const CLEAN_TYPESCRIPT: &str = r#"
import React, { useState, useEffect, useCallback } from 'react';

interface User {
    id: number;
    name: string;
    email: string;
}

interface UserComponentProps {
    userId: number;
    onUserLoad?: (user: User) => void;
}

const UserComponent: React.FC<UserComponentProps> = ({ userId, onUserLoad }) => {
    const [user, setUser] = useState<User | null>(null);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);

    const fetchUser = useCallback(async () => {
        try {
            setLoading(true);
            setError(null);

            const response = await fetch(`/api/users/${userId}`);
            if (!response.ok) {
                throw new Error(`Failed to fetch user: ${response.statusText}`);
            }

            const userData: User = await response.json();
            setUser(userData);

            if (onUserLoad) {
                onUserLoad(userData);
            }
        } catch (err) {
            setError(err instanceof Error ? err.message : 'Unknown error');
        } finally {
            setLoading(false);
        }
    }, [userId, onUserLoad]);

    useEffect(() => {
        fetchUser();
    }, [fetchUser]);

    if (loading) return <div>Loading user...</div>;
    if (error) return <div>Error: {error}</div>;
    if (!user) return <div>User not found</div>;

    return (
        <div>
            <h1>{user.name}</h1>
            <p>Email: {user.email}</p>
            <button onClick={fetchUser}>Refresh</button>
        </div>
    );
};

export default UserComponent;
"#;

/// JavaScript code with common issues
pub const JAVASCRIPT_WITH_ISSUES: &str = r#"
// Issue: var usage instead of const/let
var globalData = {};

// Issue: function declaration in block scope
function processData(data) {
    if (data) {
        function helper(item) {
            return item.value;
        }

        // Issue: == instead of ===
        if (data.type == 'user') {
            console.log('Processing user data');

            // Issue: missing error handling
            var result = data.items.map(helper);
            return result;
        }
    }
}

// Issue: missing semicolon
var userData = {
    name: 'John',
    age: 30
}

// Issue: implicit global
function updateUser() {
    userName = 'Jane'; // Missing var/let/const
    console.log('User updated');
}

module.exports = { processData, updateUser };
"#;

/// React component with performance issues
pub const REACT_PERFORMANCE_ISSUES: &str = r#"
import React from 'react';

// Issue: component recreated on every render
const ExpensiveComponent = ({ data, onUpdate }) => {
    // Issue: inline object creation
    const style = {
        width: '100%',
        height: '200px',
        backgroundColor: '#f0f0f0'
    };

    // Issue: inline function creation
    const handleClick = () => {
        console.log('clicked');
        onUpdate(data.id);
    };

    // Issue: no key prop in list
    return (
        <div style={style}>
            {data.items.map(item => (
                <div onClick={handleClick}>
                    {item.name}
                </div>
            ))}
        </div>
    );
};

// Issue: no memo for pure component
export default ExpensiveComponent;
"#;

/// Configuration file with issues
pub const CONFIG_WITH_ISSUES: &str = r#"
{
  "compilerOptions": {
    "target": "es5",
    "strict": false,
    "skipLibCheck": true,
    "allowJs": true,
    "noImplicitAny": false
  },
  "include": ["src/**/*"],
  "exclude": []
}
"#;

/// Test data builder for creating structured test scenarios
pub struct TestDataBuilder {
    files: HashMap<String, String>,
    expected_issues: Vec<ExpectedIssue>,
    metadata: HashMap<String, String>,
}

impl TestDataBuilder {
    /// Create new test data builder
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            expected_issues: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add file to test scenario
    pub fn with_file(mut self, path: &str, content: &str) -> Self {
        self.files.insert(path.to_string(), content.to_string());
        self
    }

    /// Add expected issue
    pub fn expect_issue(mut self, issue: ExpectedIssue) -> Self {
        self.expected_issues.push(issue);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    /// Build test scenario
    pub fn build(self) -> TestScenario {
        TestScenario {
            files: self.files,
            expected_issues: self.expected_issues,
            metadata: self.metadata,
        }
    }

    /// Create TypeScript scenario with issues
    pub fn typescript_with_issues() -> TestScenario {
        Self::new()
            .with_file("src/component.tsx", TYPESCRIPT_WITH_ISSUES)
            .expect_issue(ExpectedIssue {
                file_path: "src/component.tsx".to_string(),
                message_pattern: "any.*type".to_string(),
                severity: "error".to_string(),
                category: "type_safety".to_string(),
            })
            .expect_issue(ExpectedIssue {
                file_path: "src/component.tsx".to_string(),
                message_pattern: "console\\.log".to_string(),
                severity: "warning".to_string(),
                category: "best_practices".to_string(),
            })
            .with_metadata("test_type", "typescript")
            .with_metadata("complexity", "high")
            .build()
    }

    /// Create clean TypeScript scenario
    pub fn clean_typescript() -> TestScenario {
        Self::new()
            .with_file("src/component.tsx", CLEAN_TYPESCRIPT)
            .with_metadata("test_type", "typescript")
            .with_metadata("complexity", "low")
            .build()
    }

    /// Create JavaScript scenario with issues
    pub fn javascript_with_issues() -> TestScenario {
        Self::new()
            .with_file("src/legacy.js", JAVASCRIPT_WITH_ISSUES)
            .expect_issue(ExpectedIssue {
                file_path: "src/legacy.js".to_string(),
                message_pattern: "var.*declaration".to_string(),
                severity: "warning".to_string(),
                category: "best_practices".to_string(),
            })
            .with_metadata("test_type", "javascript")
            .with_metadata("complexity", "medium")
            .build()
    }

    /// Create React performance scenario
    pub fn react_performance_issues() -> TestScenario {
        Self::new()
            .with_file("src/expensive.tsx", REACT_PERFORMANCE_ISSUES)
            .expect_issue(ExpectedIssue {
                file_path: "src/expensive.tsx".to_string(),
                message_pattern: "inline.*function".to_string(),
                severity: "warning".to_string(),
                category: "performance".to_string(),
            })
            .with_metadata("test_type", "react")
            .with_metadata("complexity", "performance")
            .build()
    }

    /// Create large codebase scenario for performance testing
    pub fn large_codebase(file_count: usize) -> TestScenario {
        let mut builder = Self::new();

        for i in 0..file_count {
            let file_path = format!("src/file_{}.ts", i);
            let content = format!(
                r#"
export interface Data{} {{
    id: number;
    value: any; // Issue: any type
}}

export const process{}Data = (data: Data{}) => {{
    console.log('Processing data {}'); // Issue: console.log
    return data.value;
}};
"#,
                i, i, i, i
            );

            builder = builder.with_file(&file_path, &content);

            // Add expected issues for each file
            builder = builder.expect_issue(ExpectedIssue {
                file_path: file_path.clone(),
                message_pattern: "any.*type".to_string(),
                severity: "error".to_string(),
                category: "type_safety".to_string(),
            });

            builder = builder.expect_issue(ExpectedIssue {
                file_path,
                message_pattern: "console\\.log".to_string(),
                severity: "warning".to_string(),
                category: "best_practices".to_string(),
            });
        }

        builder
            .with_metadata("test_type", "performance")
            .with_metadata("file_count", &file_count.to_string())
            .build()
    }
}

/// Expected issue for test validation
#[derive(Debug, Clone)]
pub struct ExpectedIssue {
    pub file_path: String,
    pub message_pattern: String,
    pub severity: String,
    pub category: String,
}

/// Complete test scenario with files and expectations
#[derive(Debug, Clone)]
pub struct TestScenario {
    pub files: HashMap<String, String>,
    pub expected_issues: Vec<ExpectedIssue>,
    pub metadata: HashMap<String, String>,
}

impl TestScenario {
    /// Get total number of files
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Get expected issue count
    pub fn expected_issue_count(&self) -> usize {
        self.expected_issues.len()
    }

    /// Get test complexity from metadata
    pub fn complexity(&self) -> String {
        self.metadata.get("complexity").cloned().unwrap_or_else(|| "medium".to_string())
    }

    /// Check if scenario is performance-focused
    pub fn is_performance_test(&self) -> bool {
        self.metadata.get("test_type").map(|t| t == "performance").unwrap_or(false)
    }

    /// Get files as vector of tuples
    pub fn files_vec(&self) -> Vec<(String, String)> {
        self.files.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}

/// Predefined test scenarios
pub struct TestFixtures;

impl TestFixtures {
    /// Get all standard test scenarios
    pub fn all_scenarios() -> Vec<TestScenario> {
        vec![
            TestDataBuilder::typescript_with_issues(),
            TestDataBuilder::clean_typescript(),
            TestDataBuilder::javascript_with_issues(),
            TestDataBuilder::react_performance_issues(),
        ]
    }

    /// Get scenarios for specific test type
    pub fn scenarios_for_type(test_type: &str) -> Vec<TestScenario> {
        Self::all_scenarios()
            .into_iter()
            .filter(|scenario| scenario.metadata.get("test_type").map(|t| t == test_type).unwrap_or(false))
            .collect()
    }

    /// Get performance test scenarios
    pub fn performance_scenarios() -> Vec<TestScenario> {
        vec![
            TestDataBuilder::large_codebase(10),
            TestDataBuilder::large_codebase(50),
            TestDataBuilder::react_performance_issues(),
        ]
    }

    /// Get simple scenarios for unit testing
    pub fn unit_test_scenarios() -> Vec<TestScenario> {
        vec![TestDataBuilder::typescript_with_issues(), TestDataBuilder::clean_typescript()]
    }

    /// Get complex scenarios for integration testing
    pub fn integration_test_scenarios() -> Vec<TestScenario> {
        vec![
            TestDataBuilder::large_codebase(20),
            TestDataBuilder::javascript_with_issues(),
            TestDataBuilder::react_performance_issues(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typescript_with_issues_fixture() {
        assert!(TYPESCRIPT_WITH_ISSUES.contains("any"));
        assert!(TYPESCRIPT_WITH_ISSUES.contains("console.log"));
        assert!(TYPESCRIPT_WITH_ISSUES.len() > 100);
    }

    #[test]
    fn test_clean_typescript_fixture() {
        assert!(!CLEAN_TYPESCRIPT.contains("any"));
        assert!(!CLEAN_TYPESCRIPT.contains("console.log"));
        assert!(CLEAN_TYPESCRIPT.contains("User"));
    }

    #[test]
    fn test_data_builder() {
        let scenario = TestDataBuilder::new()
            .with_file("test.ts", "const x: any = 1;")
            .expect_issue(ExpectedIssue {
                file_path: "test.ts".to_string(),
                message_pattern: "any".to_string(),
                severity: "error".to_string(),
                category: "type_safety".to_string(),
            })
            .build();

        assert_eq!(scenario.file_count(), 1);
        assert_eq!(scenario.expected_issue_count(), 1);
    }

    #[test]
    fn test_typescript_scenario() {
        let scenario = TestDataBuilder::typescript_with_issues();
        assert_eq!(scenario.file_count(), 1);
        assert!(scenario.expected_issue_count() >= 2);
        assert_eq!(scenario.complexity(), "high");
    }

    #[test]
    fn test_large_codebase_scenario() {
        let scenario = TestDataBuilder::large_codebase(5);
        assert_eq!(scenario.file_count(), 5);
        assert_eq!(scenario.expected_issue_count(), 10); // 2 issues per file
        assert!(scenario.is_performance_test());
    }

    #[test]
    fn test_fixtures_scenarios() {
        let all = TestFixtures::all_scenarios();
        assert!(all.len() >= 4);

        let typescript = TestFixtures::scenarios_for_type("typescript");
        assert!(!typescript.is_empty());

        let performance = TestFixtures::performance_scenarios();
        assert!(!performance.is_empty());
    }
}
