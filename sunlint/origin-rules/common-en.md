# 📏 Code Quality Common Rules

> Below is a list of Common Rules, established based on core Principles. Each rule includes a description and recommended tools for enforcement.

### 📘 Rule C001 – Functions should not exceed 50 lines

- **Objective**: Improve readability and maintainability, reduce cognitive load, and enhance testability.
- **Details**:
  - Warn if the function exceeds 30 lines; error if over 50 lines.
  - Encourage breaking functions into smaller ones with meaningful names.
  - Short functions are easier to understand, test, and debug.
  - Adheres to the Single Responsibility Principle.
  - Helps reduce cyclomatic complexity.
- **Applies to**: All languages
- **Tools**: SonarQube, detekt, ESLint, PMD
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C002 – Avoid code duplication > 10 lines

- **Objective**: Prevent messy code, make refactoring easier, and apply the DRY principle.
- **Details**:
    - Warn when duplicate code ≥ 10 lines across functions/classes
    - Extract common logic into functions or utilities
    - Use inheritance or composition when appropriate
    - Create shared libraries for reusable logic
- **Applies to**: All languages
- **Tools**: PMD, SonarQube, jscpd
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C003 – Use clear variable names; avoid arbitrary abbreviations

- **Objective**: Improve readability, searchability, and enable self-documenting code.
- **Details**:
    - Avoid uncommon abbreviations (except i, j in loops)
    - Avoid single-character variables unless used as counters
    - Use descriptive names that clearly indicate purpose
    - Variable names should express *what*, not *how*
- **Applies to**: All languages
- **Tools**: ESLint (custom rule), detekt
- **Principles**: CODE_QUALITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C004 – No TODOs older than 14 days

- **Objective**: Keep the codebase clean and updated, avoid accumulating technical debt.
- **Details**:
    - Warn if a TODO comment is older than 14 days
    - Enforce a standard format with assignee and deadline
    - Use issue tracking for large tasks instead of TODO comments
    - Regularly clean up outdated TODOs
- **Applies to**: All languages
- **Tools**: Custom Git hook, linter TODO scanner
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C005 – Each function should do only one thing

- **Objective**: Ensure single responsibility, better testability and readability.
- **Details**:
    - Validate via review + cyclomatic complexity < 10
    - Function names should clearly reflect a single purpose
    - Break down large functions into smaller ones
    - A function should have only one reason to change
- **Applies to**: All languages
- **Tools**: SonarQube, CodeClimate
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C006 – Function names must be verbs or verb-noun combinations

- **Objective**: Clearly express the purpose of the action and promote self-documenting code.
- **Details**:
    - Avoid vague names like `doSomething()`
    - Use verbs that describe specific actions
    - Boolean functions should start with is/has/can/should
    - Avoid generic names that lack context
- **Applies to**: All languages
- **Tools**: PR review, AI Suggestion (Copilot Review)
- **Principles**: CODE_QUALITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C007 – Avoid comments that just restate the code

- **Objective**: Eliminate redundancy and encourage self-explanatory code.
- **Details**:
    - Warn if a comment duplicates the logic of the line below
    - Comments should explain **why**, not **what**
    - Good code should be self-documenting
    - Reserve comments for complex business logic, not syntax
- **Applies to**: All languages
- **Tools**: AI review, Lint static analyzer
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C008 – Declare variables close to where they are used

- **Objective**: Improve locality, avoid "dangling" variables, and reduce cognitive load.
- **Details**:
    - Warn if a variable is declared far from where it is used
    - Declare variables right before use
    - Use block scope to limit lifetime
    - Minimize the variable scope as much as possible
- **Applies to**: All languages
- **Tools**: Linter (e.g., ktlint, ESLint)
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C009 – Each class should have a single responsibility

- **Objective**: Improve maintainability and scalability, avoid bloated classes.
- **Details**:
    - Warn if a class exceeds 300 lines or has more than 10 methods
    - Apply the Single Responsibility Principle
    - Break large classes into smaller, more focused ones
    - Each class should have only one reason to change
- **Applies to**: All languages
- **Tools**: SonarQube, CodeClimate
- **Principles**: CODE_QUALITY, DESIGN_PATTERNS
- **Version**:
- **Status**: draft

### 📘 Rule C010 – Avoid more than 3 levels of nested blocks

- **Objective**: Reduce code complexity by limiting nesting, improving readability and maintainability.
- **Details**:
    - Warn when nesting exceeds 3 levels for if/for/while/switch
    - Deeply nested code is harder to read and understand
    - Each nested level increases cognitive complexity
    - Use early returns, guard clauses, or split into smaller functions
    - Apply "flattening" to reduce nesting depth
    - 3 nesting levels is the generally accepted maximum
    - Excessive nesting increases the risk of bugs and makes testing harder
- **Applies to**: All languages
- **Tools**: ESLint, SonarQube, TSLint, Detekt, PMD
- **Principles**: CODE_QUALITY, DESIGN_PATTERNS
- **Version**:
- **Status**: draft
- **Severity**: major

### 📘 Rule C011 – Avoid catching generic exceptions (e.g., `catch (Exception)`)

- **Objective**: Prevent hiding real issues and ensure specific and recoverable error handling.
- **Details**:
  - Warn when catching generic exceptions without clearly logging the root cause
  - Catch specific exceptions to handle each error type appropriately
  - Avoid silent failures or overly generic error handling
  - Log sufficient information for debugging and troubleshooting
  - Only catch exceptions that the code is able to handle
- **Applies to**: All languages
- **Tools**: Static analysis, SonarQube
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C012 – Clearly separate Command and Query

- **Objective**: Ensure single responsibility and clear side-effects following the Command Query Separation (CQS) principle.
- **Details**:
  - A function should not both return data and modify state
  - Command functions: change state, return void
  - Query functions: return data, do not modify state
  - Makes code easier to read, test, and avoids unintended side effects
  - Increases the predictability of functions
- **Applies to**: All languages
- **Tools**: PR review, AI code review
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C013 – Do not use dead code

- **Objective**: Keep the codebase clean; rely on Git history instead of commenting out code.
- **Details**:
  - Warn when code is commented out instead of deleted
  - Remove unused functions, variables, and imports
  - Don't keep code "just in case" — Git tracks history
  - Dead code clutters the codebase and hurts maintainability
  - Increases bundle size unnecessarily
- **Applies to**: All languages
- **Tools**: Linter + Git hook
- **Principles**: CODE_QUALITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C014 – Use Dependency Injection instead of directly instantiating dependencies

- **Objective**: Improve testability and decoupling by applying the Dependency Inversion Principle.
- **Details**:
  - Warn if a class/service directly instantiates a dependency inside the constructor
  - Inject dependencies via constructor, setter, or method parameters
  - Makes mocking easier in unit tests
  - Reduces coupling between modules
  - Allows changing implementations without affecting client code
- **Applies to**: All languages
- **Tools**: Static analyzer, PR review
- **Principles**: CODE_QUALITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C015 – Use domain language in class/function names

- **Objective**: Ensure correct domain understanding, reduce cognitive load, and improve communication with domain experts.
- **Details**:
  - Ensure function/class names map to business terminology
  - Use terms that business stakeholders can understand
  - Avoid technical jargon when business terms are available
  - Code should read like business requirements
  - Enables shared understanding between developers and business users
- **Applies to**: All languages
- **Tools**: Review + AI Suggestion
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C016 – TODOs must have a specific reason

- **Objective**: Avoid vague TODOs and ensure traceability and accountability.
- **Details**:
  - Enforce format: `// TODO(username): specific reason`
  - Must include assignee and clear description
  - Should include deadline or issue reference
  - TODOs should not exist for more than 30 days
  - Use `FIXME` for bugs, `TODO` for planned features
- **Applies to**: All languages
- **Tools**: Regex + Linter rule
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C017 – Do not put business logic inside constructors

- **Objective**: Ensure constructors only initialize objects, not perform business logic, to improve testability.
- **Details**:
  - Warn if constructors call APIs, perform logic, or log data
  - Constructors should only assign dependencies and initialize fields
  - Avoid side effects in constructors
  - Business logic should be placed in separate methods
  - Keeps object creation fast and predictable
- **Applies to**: All languages
- **Tools**: Static analyzer / Manual review
- **Principles**: CODE_QUALITY, TESTABILITY, RELIABILITY, INTEGRATION
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C018 – Do not throw generic errors; always provide detailed messages

- **Objective**: Facilitate debugging, ensure traceability, and provide full context.
- **Details**:
  - Always include meaningful messages in exceptions; avoid `throw new RuntimeException()` without context
  - Error messages should explain what happened, why, and in what context
  - Include relevant data in error messages
  - Use custom exception classes for business errors
  - Error messages should guide developers on how to fix the issue
- **Applies to**: All languages
- **Tools**: Linter + Manual review
- **Principles**: CODE_QUALITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C019 – Do not use `error` log level for non-critical issues

- **Objective**: Prevent noisy logs and false alarms; ensure consistent and meaningful log levels across the system.
- **Details**:
  - Reserve `error` for critical failures that require immediate attention or system intervention.
  - Use `warn` for potential issues that may affect functionality but don’t crash the system (e.g., retryable errors).
  - Use `info` for normal business events (e.g., login, order success, expected validation failures).
  - Use `debug` for detailed troubleshooting information; avoid excessive debug logs in production.
  - Avoid using `error` for:
    - Expected business cases (e.g., wrong password, expired card).
    - Normal validation failures.
    - Temporary, recoverable conditions (e.g., network retry).
  - Additional goal: Ensure **logs exist at the right places with the right severity level**, avoiding both over-logging and missing critical logs.
- **Applies to**: All languages
- **Tools**: Log linter / Custom rule
- **Principles**: CODE_QUALITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C020 – Do not import unused modules or libraries

- **Objective**: Reduce noise, improve build performance and code readability, and minimize bundle size.
- **Details**:
  - Use linters to automatically detect unused imports
  - Remove leftover imports after refactoring
  - Automatically clean up unused imports via CI/CD
  - Reduces bundle size and compile time
  - Prevents unnecessary dependency conflicts
  - Helps detect logic bugs (e.g., declared but unused variables)
- **Applies to**: All languages
- **Tools**: Linter (e.g., ESLint, ktlint)
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C021 – Consistently order import statements

- **Objective**: Improve consistency, readability, and manageability of imports; reduce merge conflicts.
- **Details**:
  - Group imports in order: system libraries → third-party libraries → internal project modules
  - Within each group, sort alphabetically
  - Add a blank line between import groups for clarity
  - Avoid wildcard imports (`import *`) unless necessary
  - Helps reduce merge conflicts and improves code review experience
  - Speeds up dependency lookup
- **Applies to**: All languages
- **Tools**: Import sorter (e.g., ESLint sort-imports, IntelliJ organize imports, ktlint)
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C022 – Do not leave unused variables

- **Objective**: Eliminate clutter, improve code clarity, and reduce memory footprint.
- **Details**:
  - Automatically remove unused variables using lint tools or IDE plugins
  - Reduce cognitive load when reading code
  - Prevent confusion over variable purpose
  - Reduce bundle size in JavaScript/TypeScript
  - Allow compilers to optimize more effectively
  - Help detect early logic bugs (variables created but never used)
- **Applies to**: All languages
- **Tools**: Linter / Compiler warning
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C023 – Do not declare duplicate variable names in the same scope, including nested closures

- **Objective**: Avoid confusion and hard-to-trace bugs; prevent variable shadowing.
- **Details**:
  - Do not redeclare a variable already defined in the same scope or within a nested callback
  - Avoid variable shadowing which obscures which variable is actually in use
  - Especially dangerous in closures and nested functions
  - Enables IDEs and tools to analyze code more accurately
  - Prevents hard-to-debug runtime errors
  - Improves code clarity
- **Applies to**: All languages
- **Tools**: Compiler / Linter
- **Principles**: CODE_QUALITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C024 – Do not scatter hardcoded constants throughout the logic

- **Objective**: Improve reusability, readability, and ease of configuration changes.
- **Details**:
  - Extract constants to a separate file or to the top of the class/module
  - Avoid magic numbers and magic strings in logic
  - Centralize constants for easier maintenance
  - Give constants meaningful, descriptive names
  - Makes changing values easier without searching the entire codebase
  - Improves consistency when the same value is used across multiple places
- **Applies to**: All languages
- **Tools**: Linter / Convention
- **Principles**: CODE_QUALITY, MAINTAINABILITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C025 – Each file should contain only one main class

- **Objective**: Reduce cognitive load when reading code; improve searchability and maintainability.
- **Details**:
  - Avoid putting multiple business logic classes in the same file
  - Allow nested classes or supporting DTOs that are directly related
  - File name should match the primary class name
  - Improves IDE navigation and search
  - Reduces merge conflicts in collaborative development
  - Makes refactoring and code movement easier
  - Increases cohesion of the module
- **Applies to**: All languages
- **Tools**: Convention / Linter warning
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C026 – Avoid functions with too many parameters (>6)

- **Objective**: Simplify functions, reduce confusion when calling, and minimize coupling.
- **Details**:
  - If a function has more than 6 parameters, consider converting to an object, DTO, or splitting into smaller functions
  - Too many parameters make functions harder to use and more error-prone
  - Hard to remember parameter order when calling the function
  - Increases likelihood of passing incorrect arguments
  - Use object parameters to group related data
  - Consider builder pattern for complex object creation
- **Applies to**: All languages
- **Tools**: SonarQube, Static Analyzer
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C027 – Each module should have a README.md if it is independent

- **Objective**: Improve onboarding, maintenance, and knowledge sharing.
- **Details**:
  - README.md should describe purpose, usage, and main structure of the module
  - Helps new developers quickly understand the module
  - Document APIs, dependencies, and setup requirements
  - Include examples and common use cases
  - Reduces onboarding time for new team members
  - Acts as a single source of truth for module documentation
- **Applies to**: All languages
- **Tools**: CI check / Manual review
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C028 – Use guard clauses instead of nested ifs

- **Objective**: Improve readability and avoid deep nesting, reducing cognitive complexity.
- **Details**:
  - Use early returns instead of deeply nested `if` blocks
  - Guard clauses check for invalid conditions first and exit early
  - Reduces nesting levels and improves code readability
  - Keeps the happy path (main logic) at the end of the function, not nested
  - Reduces cognitive load while reading code
  - Makes it easier to add new validations without bloating the structure
- **Applies to**: All languages
- **Tools**: PR review, linter
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C029 – All `catch` blocks must log the root cause of the error

- **Objective**: Improve traceability and incident resolution; avoid silent failures.
- **Details**:
  - Always log message and stack trace in `catch`; avoid empty `catch(e) {}`
  - Logs should include context information to assist debugging
  - Never ignore errors silently without action
  - Use appropriate log levels (error, warn, info)
  - Include relevant data like user ID, request ID, input parameters
  - Helps monitoring and alerting systems operate effectively
- **Applies to**: All languages
- **Tools**: Static analyzer / PR review
- **Principles**: CODE_QUALITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C030 – Use custom error classes instead of generic system errors

- **Objective**: Improve error classification and handling; increase maintainability.
- **Details**:
  - Create custom `Error` subclasses with specific codes and messages instead of using `throw new Error()`
  - Custom errors help callers handle different error types appropriately
  - Include error codes, HTTP status codes, and metadata
  - Make it easier to categorize and handle errors in centralized error handlers
  - Ensure consistent error handling throughout the application
  - Enables accurate classification in monitoring and alerting systems
- **Applies to**: All languages
- **Tools**: Linter / Convention
- **Principles**: CODE_QUALITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C031 – Validation logic must be separated

- **Objective**:
  - Clearly separate validation logic from business logic
  - Enable easy unit testing of validation rules
  - Improve reusability of validation logic
  - Reduce complexity in controllers and use cases
  - Simplify maintenance and extension of validation rules

- **Details**:
  - Validation should be placed in separate services or middleware
  - Avoid placing validation logic in controllers or core use cases
  - You can use validation frameworks or create your own validation service
  - Validation services should return clear success/error results with detailed messages
  - Support both field-level and object-level validation
  - Define validation rules declaratively and make them readable

- **Applies to**: All languages
- **Tools**: Convention, PR review
- **Principles**: CODE_QUALITY, TESTABILITY, MAINTAINABILITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C032 – Do not call external APIs in constructors or static blocks

- **Objective**:
  - Avoid unexpected code execution during module loading
  - Prevent unwanted side effects during initialization
  - Increase application predictability
  - Simplify testing and mocking dependencies

- **Details**:
  - Do not call external APIs (HTTP, database, file system) inside constructors or static blocks
  - Warn when `fetch`, `axios`, or similar calls are used in constructors or static blocks
  - Move complex initialization logic outside constructors
  - Use factory pattern or async initialization pattern if necessary

- **Applies to**: All languages
- **Tools**: Static analyzer
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C033 – Separate processing logic and data access in the service layer

- **Objective**:
  - Improve reusability and testability
  - Clearly separate business logic from data access logic
  - Make it easier to change data access implementations
  - Improve maintainability of the code
  - Follow the Single Responsibility Principle

- **Details**:
  - Business logic should not contain query statements; separate Repository and Service
  - Repositories should only contain basic data access methods (CRUD)
  - Services should contain business logic and use Repositories for queries
  - Each Service should correspond to a Repository
  - Do not place business logic inside Repositories
  - Use dependency injection to inject Repositories into Services

- **Applies to**: All languages
- **Tools**: Architectural review
- **Principles**: CODE_QUALITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C034 – Avoid directly accessing global state in domain logic

- **Objective**:
  - Reduce reliance on global state
  - Improve testability of the code
  - Allow easier replacement of dependencies
  - Avoid unwanted side effects
  - Follow the Dependency Inversion Principle

- **Details**:
  - Do not directly access global variables, singletons, or static methods in business logic
  - Use dependency injection to pass in required dependencies
  - Define dependencies clearly through interfaces
  - Avoid using static methods for business logic
  - Separate configuration from business logic

- **Applies to**: All languages
- **Tools**: Static analyzer, Code review
- **Principles**: CODE_QUALITY, DESIGN_PATTERNS
- **Version**:
- **Status**: draft

### 📘 Rule C035 – Log all relevant context when handling errors

- **Objective**:
  - Simplify debugging and troubleshooting
  - Provide full context for error analysis
  - Improve system observability and monitoring
  - Speed up error detection and resolution
  - Follow the principle of observability

- **Details**:
  - Log error message, stack trace, and error code
  - Include error context: request ID, user ID, input data
  - Classify log level appropriately (ERROR, WARN, INFO)
  - Log both at the throw and catch locations
  - Do not log sensitive data (e.g., passwords, tokens)
  - Use structured logging format

- **Applies to**: All languages
- **Tools**: Logging framework
- **Principles**: CODE_QUALITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C036 – Do not throw generic exceptions like `RuntimeException` or `Exception`

- **Objective**:
  - Improve clarity and specificity of error handling
  - Make it easier to classify and handle errors
  - Provide detailed information about the root cause
  - Enhance maintainability and debugging
  - Follow the principles of fail-fast and explicit failure

- **Details**:
  - Avoid using generic exceptions like `Exception`, `RuntimeException`, `Error`
  - Create custom exception classes for specific error types
  - Include useful information in exceptions (error code, context)
  - Name exceptions clearly to reflect the error meaning
  - Inherit from the appropriate base exception class for the language

- **Applies to**: All languages
- **Tools**: Static analyzer
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C037 – API handler functions should return a standardized response object (not raw strings)

- **Objective**:
  - Ensure consistency in API responses
  - Simplify response handling on the client side
  - Support API versioning and extensibility
  - Improve maintainability and debugging
  - Follow REST API best practices

- **Details**:
  - Use a standard response object with fixed fields
  - Include necessary metadata (status, message, timestamp)
  - Separate data from metadata clearly
  - Use appropriate HTTP status codes
  - Support pagination if needed
  - Handle error responses in a consistent way

- **Applies to**: All languages
- **Tools**: API documentation tools
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C038 – Avoid logic that depends on file/module load order

- **Objective**:
  - Increase code independence and reusability
  - Reduce implicit dependencies between modules
  - Make code easier to test and maintain
  - Prevent hard-to-debug execution order issues
  - Follow the principle of loose coupling

- **Details**:
  - Do not rely on the order of module imports/requires
  - Avoid using global state to pass data between modules
  - Use dependency injection instead of direct imports
  - Avoid executing complex logic during module import
  - Clearly separate module definition and initialization
  - Use factory pattern or lazy loading if necessary

- **Applies to**: All languages
- **Tools**: Architectural Review
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C039 – Do not store temporary data in global or static mutable fields

- **Objective**:  
  Prevent issues related to shared state and race conditions in concurrent environments. Ensure thread-safety and testability. Using global or static mutable fields can introduce hard-to-detect and hard-to-fix bugs.
- **Details**:
  - Use context-passing in functions instead of relying on global state
  - Ensure thread-safety using appropriate synchronization mechanisms
  - Prefer dependency injection to manage state and dependencies
  - Avoid mutable static fields in classes
  - Use local or instance variables instead of global/static ones
- **Applies to**: All languages
- **Tools**: Static Analyzer
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C040 – Do not spread validation logic across multiple classes

- **Objective**:  
  Centralize validation logic to simplify maintenance, increase reusability, and ensure consistency. Centralized validation helps reduce bugs and simplifies updating validation rules.
- **Details**:
  - Place validation logic in a dedicated validator or shared service
  - Avoid duplicating the same validation condition in multiple places
  - Use available validation frameworks where possible
  - Clearly separate validation logic from business logic
  - Ensure validation rules are well-documented

- **Applies to**: All languages
- **Tools**: Architectural Refactor Review
- **Principles**: CODE_QUALITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C041 – Do not hardcode or push sensitive information (token, API key, secret, URL) into the repo

- **Objective**: Protect sensitive application data, avoid security risks, and comply with security standards. Exposing sensitive information can lead to serious security and privacy issues.

- **Details**:
  - Use environment variables or separate config files to store secrets
  - Add secret files to `.gitignore` to prevent committing them
  - Use secret management tools such as Vault or AWS Secrets Manager
  - Encrypt sensitive information when necessary
  - Use different environment variables for different environments (dev, staging, prod)

- **Applies to**: All languages
- **Tools**: Git Hooks, Secret Scanner
- **Principles**: SECURITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C042 – Boolean variable names should start with `is`, `has`, or `should`

- **Objective**: Ensure clarity and readability by making boolean variables self-explanatory. This naming convention improves code maintainability and documentation.

- **Details**:
  - Use `is` for state attributes (e.g., `isActive`, `isEnabled`)
  - Use `has` for ownership (e.g., `hasPermission`, `hasChildren`)
  - Use `should` for decision flags (e.g., `shouldUpdate`, `shouldRetry`)
  - Avoid inconsistent prefixes like `can`, `will`, or `does`
  - Ensure the variable name accurately represents its boolean meaning

- **Applies to**: All languages
- **Tools**: Linter (ESLint, SonarQube)
- **Principles**: CODE_QUALITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C043 – Do not use `print` or `console.log` in production code

- **Objective**: Ensure logging is done in a controlled and effective manner in production. Using `print` or `console.log` can lead to performance issues, security risks, and log management difficulties.

- **Details**:
  - Use a dedicated logging framework instead of `print` or `console.log`
  - Set appropriate log levels for each environment (debug, info, warn, error)
  - Ensure logs contain useful metadata like timestamp, level, and context
  - Avoid logging sensitive data like passwords, tokens, or personal information
  - Use structured logging for easier analysis and search

- **Applies to**: All languages
- **Tools**: Linter, Log Analyzer
- **Principles**: CODE_QUALITY, PERFORMANCE
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C044 – Avoid reimplementing functions that already exist in standard libraries or helper utilities

- **Objective**: Leverage well-tested, optimized, and community-maintained libraries to reduce bugs and improve development efficiency.

- **Details**:
  - Prefer using standard language libraries
  - Use trusted and popular community libraries
  - Evaluate library compatibility and performance
  - Ensure the library is actively maintained
  - Only implement custom logic when necessary and justified

- **Applies to**: All languages
- **Tools**: Package Manager, Dependency Analyzer
- **Principles**: CODE_QUALITY, PERFORMANCE
- **Version**:
- **Status**: draft

### 📘 Rule C045 – APIs should not return 500 errors for known business errors

- **Objective**: Ensure APIs return appropriate HTTP status codes so clients can handle errors effectively. HTTP 500 should be reserved for unexpected system errors.

- **Details**:
  - Use specific HTTP status codes based on error type:
    - 400 for validation errors
    - 401 for authentication failures
    - 403 for authorization failures
    - 404 for resource not found
    - 409 for data conflict
    - 422 for business logic errors
    - 500 only for unexpected internal server errors

- **Applies to**: All languages
- **Tools**: API Testing, Error Monitoring
- **Principles**: CODE_QUALITY
- **Version**:
- **Status**: draft

### 📘 Rule C046 – Avoid complex and lengthy regular expressions in core logic

- **Objective**: Keep code readable, maintainable, and efficient by avoiding the use of overly complex regular expressions in business-critical logic.

- **Details**:
  - Move complex regex into constants or helper functions
  - Prefer string manipulation libraries over complex regex
  - Break down complex regex into simpler processing steps
  - Add comments for regex that must be used and are hard to read
  - Use dedicated parsers for complex parsing needs
  - Avoid using regex to parse structured data formats

- **Applies to**: All languages
- **Tools**: Code Review, Static Code Analyzer
- **Principles**: CODE_QUALITY, PERFORMANCE
- **Version**:
- **Status**: draft

### 📘 Rule C047 – Retry logic must not be duplicated in multiple places

- **Objective**: Centralize retry logic to improve consistency, maintainability, and observability of error handling and retry mechanisms.

- **Details**:
  - Create a dedicated utility class or service for retry logic
  - Centralize retry policy configuration (retry count, delay, backoff)
  - Use decorator pattern or AOP to apply retry logic
  - Support different retry strategies (immediate, exponential backoff)
  - Allow customizing retry conditions per use case
  - Log all retry attempts with sufficient context

- **Applies to**: All languages
- **Tools**: Code Review, Static Code Analyzer
- **Principles**: CODE_QUALITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C048 – Do not bypass architectural layers (controller/service/repository)

- **Objective**: Maintain a clear layered architecture, ensuring logic and data flow are well-structured and maintainable.

- **Details**:
  - Controllers should only call Services, not Repositories directly
  - Services should only call Repositories, not Controllers
  - Repositories should only handle data access, not call Services
  - Each layer should only know the layer directly below it
  - Use dependency injection to manage dependencies
  - Define clear interfaces for each layer
  - Avoid circular dependencies between layers

- **Applies to**: All languages
- **Tools**: Static Analyzer, Architectural boundary checker
- **Principles**: DESIGN_PATTERNS, MAINTAINABILITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C049 – Always include a clear default case in switch/case statements

- **Objective**: Avoid missing logic for unexpected values, increasing stability and safety of the application.

- **Details**:
  - Every `switch/case` should include a `default` or `else` branch for unknown values
  - Without a default, the system may silently skip or crash on unexpected input
  - The default case should log the issue or throw an appropriate error
  - Applies to `enum`, `status`, `type`, `command`, especially when handling external/user input

- **Applies to**: All languages
- **Tools**: Linter
- **Principles**: CODE_QUALITY
- **Version**: 
- **Status**: draft
- **Severity**: major

### 📘 Rule C050 – Do not call APIs in loops without batching or throttling

- **Objective**: Prevent system overload, resource contention, API rate limit violations, or the backend being perceived as under attack (DDoS).

- **Details**:
  - Do not call APIs inside loops without concurrency control (`throttle`), grouping (`batching`), or async pooling (`async pool`, `concurrent queue`)
  - When calling multiple APIs, use libraries to manage concurrency and retries
  - Respect API rate limits using client-side throttling mechanisms
  - Prefer using `bulk API` endpoints when available for list-based operations

- **Applies to**: All languages
- **Tools**: Performance review
- **Principles**: CODE_QUALITY, PERFORMANCE
- **Version**: 
- **Status**: draft
- **Severity**: major

### 📘 Rule C051 – Do not use `sleep`, `wait`, or `delay` in business logic

- **Objective**: Avoid uncontrolled delays that cause asynchronous issues, make debugging and testing harder, and increase the risk of race conditions in production environments.

- **Details**:
  - Do not use `sleep`, `wait`, `delay`, `Thread.sleep`, `setTimeout`, `delay()`, etc., directly inside business logic (services, use cases, repositories)
  - Manual delays block threads and increase latency unnecessarily, reducing system throughput
  - Use a proper retry strategy with timeout, backoff, and limit (applied in orchestrators, schedulers, or middleware) instead
  - Do not use arbitrary delay values unless required for security purposes (e.g., brute-force mitigation)
  - Delay is acceptable in tests, demos, or orchestration logic — not core business logic

- **Applies to**: All languages
- **Tools**: Static analyzer, manual review
- **Principles**: CODE_QUALITY, PERFORMANCE
- **Version**: 
- **Status**: draft
- **Severity**: major

### 📘 Rule C052 – Parsing or data transformation logic must be separated from controllers

- **Objective**: Enforce separation of concerns — controllers should only handle requests and delegate processing, improving testability, maintainability, and reuse.

- **Details**:
  - Controllers should not perform heavy processing like JSON parsing, data transformation, or domain mapping
  - Input/output conversions (DTO ↔ domain) should be handled in mappers, transformers, or services
  - This separation allows easy unit testing of controllers by mocking services and reducing duplication
  - Avoid formatting date/time, numeric conversions, or mini business logic in controllers

- **Applies to**: All languages
- **Tools**: Code review / Architecture enforcement
- **Principles**: CODE_QUALITY, DESIGN_PATTERNS, MAINTAINABILITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: major

### 📘 Rule C053 – Avoid vague function names like "handle" or "process"

- **Objective**: Clarify function behavior, reduce hidden logic, and improve code readability and traceability.

- **Details**:
  - Avoid vague names like `handleData()` or `processSomething()` that do not clearly convey intent
  - Prefer descriptive names such as `calculateInvoice()`, `validateUser()`, `fetchProductList()`
  - Clear function names help with debugging, logging, tracing, and onboarding
  - If using `handle`, clarify the context: e.g., `handleUserLoginError()`; but consider `logLoginError()` or `redirectToLoginFailure()` instead

- **Applies to**: All languages
- **Tools**: AI reviewer / naming linter
- **Principles**: CODE_QUALITY
- **Version**: 
- **Status**: draft
- **Severity**: minor

### 📘 Rule C054 – Do not process large datasets without pagination or lazy loading

- **Objective**: Prevent loading all data into memory, avoid out-of-memory errors, and improve performance and response time.

- **Details**:
  - Never use unrestricted queries like `SELECT *`, `findAll()`, or `getAll()` for large datasets
  - APIs returning lists must support pagination: `limit/offset`, `cursor`, or `keyset pagination`
  - Use lazy iterators, stream readers, or batch processing when working with large DB/file/stream data
  - Avoid mapping large datasets into memory when only part of the data is needed

- **Applies to**: All languages
- **Tools**: Code review, ORM warning, API response profiler
- **Principles**: PERFORMANCE
- **Version**: 
- **Status**: draft
- **Severity**: major

### 📘 Rule C055 – Cache results of expensive functions if reused

- **Objective**: Reduce processing time and resource usage by caching results of resource-heavy operations.

- **Details**:
  - If a function performs slow processing, external API calls, or heavy queries with stable results → use caching
  - Caching options: in-memory (e.g., Map/LRU), function memoization, HTTP cache, Redis, service-level cache
  - Ensure:
    - TTL is defined
    - Proper invalidation is implemented when data changes
    - Caching does not break transactional integrity or cause stale data
  - Only cache pure functions with predictable outputs based solely on input

- **Applies to**: All languages
- **Tools**: Code review, performance profiler
- **Principles**: CODE_QUALITY, PERFORMANCE
- **Version**: 
- **Status**: draft
- **Severity**: major

### 📘 Rule C056 – Do not process large datasets without logging or resource monitoring

- **Objective**: Track resource usage (CPU, RAM, I/O), detect anomalies early, and ensure system stability.

- **Details**:
  - When processing large datasets (e.g., >10,000 records), log key metrics: record count, processing time, RAM usage, CPU peak
  - For scheduled jobs (cron, batch, ETL), integrate with monitoring tools like Prometheus, CloudWatch, Grafana
  - Define soft thresholds to log/warn when limits are exceeded (execution time, memory, volume)
  - Logs should include: job ID, name, start/end time, input/output size, and context

- **Applies to**: All languages
- **Tools**: Logging, APM (Application Performance Monitoring)
- **Principles**: PERFORMANCE, RELIABILITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: major

### 📘 Rule C057 – Use optimal data structures instead of arrays for frequent lookups

- **Objective**: Reduce algorithm complexity, improve access speed, and optimize performance.

- **Details**:
  - For repeated key-based lookups, avoid repeated `array.find()` or `array.filter()` — use `Map`, `Set`, `Dictionary`, `HashMap`, etc.
  - Avoid redundant loops to search for the same value multiple times
  - Convert lists to maps for constant-time key access
  - Use `Set` instead of `Array` for membership checks (`includes`, `contains`)
  - Choosing the wrong structure increases complexity from `O(1)` to `O(n)` or `O(n^2)`

- **Applies to**: All languages
- **Tools**: Static analyzer, AI reviewer
- **Principles**: PERFORMANCE
- **Version**: 
- **Status**: draft
- **Severity**: major

### 📘 Rule C058 – Enums must have clear display labels

- **Objective**: Ensure enums shown in logs, UIs, or APIs are understandable and user-friendly.

- **Details**:
  - Avoid showing raw enum values like `STATUS_APPROVED` or `CODE_1` directly
  - Enums should implement `getLabel()`, `getDescription()`, or override `toString()` for meaningful values
  - In TypeScript (frontend), use a clear mapping from enum → label
  - In Java/Go (backend), override `toString()` or include a description field and ensure proper serialization in JSON/logs
  - UI enums must use domain-meaningful, human-readable labels: `DELIVERED` → "Delivered"

- **Applies to**: All languages
- **Tools**: Manual review
- **Principles**: CODE_QUALITY
- **Version**: 
- **Status**: draft
- **Severity**: minor

### 📘 Rule C059 – Do not create abstractions just to group constants

- **Objective**: Avoid unnecessary abstractions (class, enum) that add complexity without behavior or clear domain meaning.

- **Details**:
  - Do not create `class`, `interface`, or `enum` just to group constants if they don't represent a meaningful domain concept
  - If constants are internal to a module, keep them as module-level constants
  - Use `enum` only when values represent distinct domain concepts with potential behavior
  - Avoid generic names like `Constants`, `CommonKeys`, or `GlobalValues`, especially if disconnected from logic

- **Applies to**: All languages
- **Tools**: Manual review
- **Principles**: CODE_QUALITY, MAINTAINABILITY
- **Version**: 
- **Status**: draft
- **Severity**: minor

### 📘 Rule C060 – Do not override superclass methods and ignore critical logic

- **Objective**: Preserve important behavior or lifecycle logic defined in the superclass to ensure correctness and prevent silent errors.

- **Details**:
  - When overriding a superclass method, be cautious if it contains:
    - Resource initialization
    - Precondition checks
    - Logging, auditing, statistics
    - Hooks or extension logic
  - If overriding without calling `super.method()` or equivalent logic, important behavior may be skipped
  - In lifecycle-based frameworks (Spring, React, Android, etc.), omitting `super` calls can break the system

- **Applies to**: All languages
- **Tools**: Manual review
- **Principles**: CODE_QUALITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: major

### 📘 Rule C061 – Write unit tests for business logic

- **Objective**: Ensure that core business flows are verifiable, help catch bugs early, avoid regressions, and increase system reliability.

- **Details**:
  - Each business function (use case, service method, business rule) must have at least one unit test verifying correctness.
  - Cover both common scenarios and edge cases (null, empty, large values, exceptions, etc.)
  - Prioritize testing pure logic (not dependent on DB, network, or I/O)
  - Unit tests should be fast, independent, easy to read, and not require special setup
  - Follow the AAA (Arrange – Act – Assert) or Given – When – Then structure for clarity

- **Applies to**: All languages
- **Tools**: Manual review
- **Principles**: CODE_QUALITY, TESTABILITY, MAINTAINABILITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: major

### 📘 Rule C062 – Interfaces or abstractions should not hold state

- **Objective**: Keep interfaces focused on defining behavior, making them easier to mock, implement, and test.

- **Details**:
  - Interfaces or abstract classes should define method signatures, not hold state (fields, properties, variables)
  - State should reside in the implementation layer, not in the interface
  - Avoid interfaces with default fields or abstract classes with complex logic and member variables unless using the Template Method pattern intentionally
  - Follow the Interface Segregation Principle — keep interfaces simple, clear, and side-effect free

- **Applies to**: All languages
- **Tools**: Manual review
- **Principles**: CODE_QUALITY, DESIGN_PATTERNS
- **Version**: 
- **Status**: draft
- **Severity**: major

### 📘 Rule C063 – Do not repeat the same test logic

- **Objective**: Avoid duplication in tests, making them easier to maintain, read, and extend when business logic changes.

- **Details**:
  - Do not write multiple similar test cases that differ only in input/output
  - Instead, use table-driven tests, parameterized tests, or combine variations into one clear test case
  - Follow the DRY (Don't Repeat Yourself) principle in tests, not just production code
  - If identical test logic appears across files, extract it into helper functions or shared fixtures

- **Applies to**: All languages
- **Tools**: Manual review
- **Principles**: CODE_QUALITY, TESTABILITY, MAINTAINABILITY
- **Version**:
- **Status**: draft
- **Severity**: minor

### 📘 Rule C064 – Interfaces should expose only necessary behavior

- **Objective**: Prevent leaking implementation details, improve encapsulation, and reduce coupling between modules.

- **Details**:
  - Interfaces should only define methods needed by consumers
  - Do not expose internal or helper methods intended only for implementation
  - Fat interfaces make implementation harder, reduce mockability and testability, and introduce unnecessary coupling
  - Follow the Interface Segregation Principle — split interfaces by role so implementations aren't forced to support unrelated methods

- **Applies to**: All languages
- **Tools**: Manual review
- **Principles**: CODE_QUALITY, DESIGN_PATTERNS
- **Version**: 
- **Status**: draft
- **Severity**: major

### 📘 Rule C065 – Each test case should verify only one behavior

- **Objective**: Make test failures easier to diagnose and ensure clarity and maintainability in test code.

- **Details**:
  - Each test should focus on one specific scenario or behavior
  - Do not include multiple logic branches in a single test — failures become harder to trace
  - Split logic with multiple conditions into separate test cases
  - Follow AAA (Arrange – Act – Assert), and avoid unrelated assertions in the same test

- **Applies to**: All languages
- **Tools**: Manual review
- **Principles**: CODE_QUALITY, TESTABILITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: major

### 📘 Rule C066 – Test names should reflect what is being tested

- **Objective**: Help readers quickly understand the purpose of a test, making the test suite self-documenting and easier to trace on failure.

- **Details**:
  - Test function/method names should clearly state the input condition, behavior being tested, and expected result
  - Avoid vague names like `testSomething()`, `testLogic1()`, or `it should work`
  - Recommended formats:
    - `should_<expected_behavior>_when_<condition>`
    - `test_<action>_<expected_result>`
  - Clear naming eliminates the need to read test contents to understand its purpose

- **Applies to**: All languages
- **Tools**: Manual review
- **Principles**: CODE_QUALITY, TESTABILITY
- **Version**:
- **Status**: draft
- **Severity**: minor

### 📘 Rule C067 – Do not hardcode configuration inside code

- **Objective**: Improve configurability, reduce risk when changing environments, and make configuration management flexible and maintainable.

- **Details**:
  - Avoid hardcoding values such as:
    - API URLs, endpoints
    - Timeouts, retry intervals, batch sizes
    - Credentials (username, password, API key)
    - Feature flags, toggles, thresholds
  - Manage these values through:
    - Environment variables
    - Config files (`.env`, `config.yaml`, `application.properties`)
    - Constructor or DI-based injection
  - Centralized configuration simplifies environment-specific overrides and avoids sensitive info leaks
  - Keep all config in one central place (e.g., `config.ts`, `Config.java`, `config.go`, or `settings.py`), not scattered across services/modules

- **Applies to**: All languages
- **Tools**: Manual review
- **Principles**: CODE_QUALITY, MAINTAINABILITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: major

### 📘 Rule C068 – Avoid unclear return types in functions

- **Objective**: Help callers know what data they receive, enable type checking, reduce hidden errors, and improve predictability.

- **Details**:
  - Avoid returning vague types like `any`, `Object`, `dynamic`, `Map<String, dynamic>`, or `interface{}` (in Go) without a clear schema
  - Define return types using interfaces/classes/structs or generics with explicit types
  - Unclear types make property access unsafe, introduce runtime bugs, reduce testability, and make APIs harder to use
  - In large or multi-team systems, strong typing is key to scaling and maintaining quality

- **Applies to**: All languages
- **Tools**: Manual review
- **Principles**: CODE_QUALITY
- **Version**: 
- **Status**: draft
- **Severity**: major

### 📘 Rule C069 – Components should communicate via abstractions

- **Objective**: Reduce module coupling, improve testability, ease mocking, and ensure replaceability without affecting callers.

- **Details**:
  - Services, repositories, clients, and modules should communicate via interfaces, protocols, ports, or abstract classes — not direct implementation
  - Benefits of abstraction:
    - Easier to swap implementations (e.g., mock DB or fake API)
    - Enables testing without infrastructure (real DB/network)
    - Complies with the Dependency Inversion Principle in SOLID
  - Inject dependencies (via constructor or DI container), don't instantiate them directly

- **Applies to**: All languages
- **Tools**: Manual review
- **Principles**: CODE_QUALITY, DESIGN_PATTERNS, TESTABILITY
- **Version**: 
- **Status**: draft
- **Severity**: major

### 📘 Rule C070 – Tests should not rely on real time

- **Objective**: Improve test stability and speed; avoid flaky tests caused by system clock or real-world timing.

- **Details**:
  - Avoid using `delay()`, `sleep()`, `Thread.sleep()`, `setTimeout()`, or `time.Sleep()` in tests to wait for results
  - Don’t write tests like: `someSetup() → wait 5s → assert result`
    - They run slowly
    - Are unstable due to system/CPU/network variability
    - Often fail in CI or under load
  - Instead, mock async components, inject a clock/timer, or observe behavior via hooks, callbacks, or queues

- **Applies to**: All languages
- **Tools**: Manual review
- **Principles**: CODE_QUALITY, TESTABILITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: major

### 📘 Rule C071 – Test class names should reflect the corresponding module

- **Objective**: Make it easy to identify the scope of testing, improve discoverability, and provide clear organization in the test structure.

- **Details**:
  - Test class names should clearly indicate the module or subject under test and use standard suffixes like `Test`, `Spec`, `Tests`, or `TestCase`.
  - This helps:
    - Locate the test associated with a specific module
    - Allow CI/CD tools to detect the correct test suite
    - Enable automatic test detection by naming conventions (e.g., JUnit, Jest, Pytest, etc.)
  - Avoid vague names like `UtilsTest`, `MainTest`, or naming misalignment (e.g., testing controller but naming it `ServiceTest`)

- **Applies to**: All languages
- **Tools**: Manual Review
- **Principles**: CODE_QUALITY, MAINTAINABILITY
- **Version**:
- **Status**: draft
- **Severity**: minor

### 📘 Rule C072 – Each test should assert only one behavior

- **Objective**: Reduce ambiguity when a test fails, ensuring each test case validates a single, specific logic path.

- **Details**:
  - A test case should assert only one distinct behavior — avoid bundling multiple asserts or unrelated checks in the same test
  - When a test includes several assertions, it becomes unclear which one caused the failure, complicating debugging
  - If testing multiple aspects, split them into separate test cases with descriptive names
  - Prioritize isolation and clarity over minimizing lines of code

- **Applies to**: All languages
- **Tools**: Manual Review
- **Principles**: CODE_QUALITY, TESTABILITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: major

### 📘 Rule C073 – All required configurations must be validated at startup

- **Objective**: Prevent unclear runtime errors due to missing or incorrect config. Ensure the app fails fast if essential settings are absent.

- **Details**:
  - All required configurations such as `API key`, `base URL`, `database credentials`, `secrets`, and `feature flags` must be checked at app startup
  - If missing or invalid, the app must clearly log the error and **refuse to run** (panic/exit/throw)
  - Avoid cases where the app starts normally but later fails due to `undefined`, `null`, or `connection failed`
  - Use validation schema tools (e.g., `zod`, `Joi`, `dotenv-safe`) or implement explicit checks

- **Applies to**: All languages
- **Tools**: Manual Review
- **Principles**: CODE_QUALITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: major

### 📘 Rule C074 – Avoid magic numbers/values in code

- **Objective**: Improve clarity and self-documentation in code, so readers can understand the meaning of values without additional context.

- **Details**:
  - Avoid hardcoding numbers (`60`, `1024`, `3600`, `-1`, ...) or strings (`"admin"`, `"success"`, `"N/A"`) without descriptive context
  - Magic values make code harder to read, update, and reuse; they often lead to subtle logic bugs
  - Use named constants, enums, or config variables with meaningful names
  - Applies to business logic, condition checks, default settings, and HTTP/API responses

- **Applies to**: All languages
- **Tools**: Linter (ESLint, PMD, Detekt, etc.)
- **Principles**: CODE_QUALITY, MAINTAINABILITY
- **Version**: 
- **Status**: draft
- **Severity**: major

### 📘 Rule C075 – All functions must explicitly declare return types

- **Objective**: Improve clarity, predictability, and enforce strict type control to avoid silent errors during refactoring or logic changes.

- **Details**:
  - Functions should explicitly declare the return type, especially for public, exported, or widely-used functions
  - Avoid vague return types like `Any`, `interface{}`, `Object`, `Map<String, dynamic>` without a defined schema
  - Clear return types enable:
    - Early detection of type errors
    - Easier understanding of code without reading the entire function body
    - Safer refactoring and branching
  - Not mandatory for inline or narrow-scope lambdas/functions

- **Applies to**: All languages
- **Tools**: Type checker, Linter
- **Principles**: CODE_QUALITY
- **Version**: 1.0
- **Status**: activated

### 📘 Rule C076 – All public functions must declare explicit types for arguments

- **Objective**: Ensure type safety for function inputs, reduce runtime errors, and enable static analysis during compilation or code review.

- **Details**:
  - Public or exported functions must define explicit types for all arguments
  - Avoid generic types like `any`, `Object`, `dynamic`, or `Map<String, dynamic>` unless backed by a clear schema
  - Clearly typed arguments help:
    - Catch incorrect usage early
    - Improve automatic documentation
    - Simplify testing, mocking, and input validation
  - Internal or inline private functions can be more flexible, but public functions require strict control

- **Applies to**: All languages
- **Tools**: Type checker, Linter
- **Principles**: CODE_QUALITY, MAINTAINABILITY
- **Version**: 1.0
- **Status**: activated
