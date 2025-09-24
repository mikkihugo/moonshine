# 📘 Security Specific Coding Rules

> This is a comprehensive list of security rules designed to enhance the security posture of applications.
> Each rule includes a title, detailed description, applicable programming languages, and priority level.

### 📘 Rule S001 – Fail securely when access control errors occur

- **Objective**: Ensure the system does not accidentally grant access when errors occur, helping to reduce the risk of unauthorized access and protect sensitive resources.
- **Details**:
  - When errors occur in access control checks (e.g., query errors, runtime exceptions), the system must deny access instead of granting default permissions.
  - Must not fall back to "allow" state if permission data is missing or logic errors occur.
  - Access control mechanisms must fail in a "deny by default" manner.
- **Applies to**: All languages
- **Tools**: SonarQube (S4524), PMD (SecurityCodeGuidelines), Manual Review, Unit Test
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: critical

### 📘 Rule S002 – Avoid IDOR vulnerabilities in CRUD operations

- **Objective**: Prevent unauthorized access to sensitive data by verifying users' actual access rights, avoiding reliance solely on IDs in URLs.
- **Details**:
  - Must not control permissions based solely on `id` sent from the client.
  - Always verify resource ownership at the backend (authorization logic).
  - Prefer using UUID or encrypted IDs to avoid predictable sequential IDs.
  - Check APIs like: `GET/PUT/DELETE /api/resource/{id}` must ensure the id belongs to the current user.
- **Applies to**: All languages
- **Tools**: SonarQube (S6142, S2076), Semgrep (custom rule), Manual Review
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: critical

### 📘 Rule S003 – URL redirects must be within an allow list

- **Objective**: Prevent Open Redirect vulnerabilities, protecting users from being redirected to malicious pages through spoofed input.
- **Details**:
  - Must not redirect to URLs received from user input without validation.
  - If dynamic redirects are needed, must check URLs against an allow list before execution.
  - If the URL is outside the allow list, you can:
    - Reject the redirect (HTTP 400)
    - Or display a clear warning before continuing.
- **Applies to**: All languages
- **Tools**: Semgrep (custom rule), Manual Review, SonarQube (custom rule)
- **Principles**: SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: low

### 📘 Rule S004 – Do not log login credentials, payment information, and unencrypted tokens

- **Objective**: Prevent leakage of sensitive information through log systems – a common attack vector if logs are shared, stored incorrectly, or exploited.
- **Details**:
  - Must not log fields like: `password`, `access_token`, `credit_card`, `cvv`, `secret_key`, etc.
  - Tokens (session tokens, JWT, etc.) if logging is mandatory for debugging, must be hashed or masked (`****`).
  - Avoid logging entire `request.body`, `form-data`, or `headers` containing sensitive information.
  - Ensure logs are configured to exclude sensitive fields.
- **Applies to**: All languages
- **Tools**: SonarQube (S2068, S5334), Semgrep (custom rule), Manual Review
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S005 – Do not use Origin header for authentication or access control

- **Objective**: Prevent authentication or authorization decisions based on `Origin` header – which can be easily spoofed from the client side and is unreliable for security purposes.
- **Details**:
  - `Origin` header can be modified from the client side (via curl, script, proxy), so it must not be used as a basis for:
    - User identification
    - Access permission checks
    - Security business logic routing
  - `Origin` should only be used for source authentication in CSRF checks or CORS policies, **not for authorization decisions**.
  - If behavior needs to be limited by domain, must check against tokens, user sessions, or actually verified scopes.
- **Applies to**: All languages
- **Tools**: Manual Review, Semgrep (custom rule), SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: low

### 📘 Rule S006 – Do not send recovery or activation codes in plaintext

- **Objective**: Prevent leakage of verification codes, recovery codes, or activation tokens if email/SMS is intercepted or exposed – thereby minimizing the risk of account takeover attacks.
- **Details**:
  - Should not send recovery or activation codes in predictable formats, sequential numbers, or containing user identifying information.
  - Codes sent via email/SMS must be random codes with short time-to-live (TTL), verified one-way by the system (hashed) or verified by session.
  - Should not store plaintext codes in database or log sent code content.
  - Prefer verification via temporary encrypted one-time links.
- **Applies to**: All languages
- **Tools**: Manual Review, Semgrep (custom rule), Secret Detection (regex scanner), SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S007 – Do not store OTP codes in plaintext

- **Objective**: Protect the system from OTP reuse attacks if database/logs are accessed without authorization. OTP, magic-links, or reset codes must be treated like passwords – only stored in one-way hashed form, not recoverable.
- **Details**:
  - Must not store OTP codes, reset tokens, or magic-link codes in original form (plaintext) in database or log files.
  - OTP should have short TTL (e.g., 5 minutes) and be stored using hash functions like SHA-256.
  - During verification, should only compare hashed versions.
  - Never resend original OTP to user if already stored – generate new code if needed.
- **Applies to**: All languages
- **Tools**: Manual Review, Semgrep (custom rule), Secret Detection, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S008 – Encryption algorithms and parameters must support flexible configuration and upgrades (crypto agility)

- **Objective**: Avoid binding the system to outdated cryptographic algorithms or parameters (like MD5, SHA-1, DES...), ensuring easy upgrades when standards change or new vulnerabilities emerge.
- **Details**:
  - Do not hardcode algorithms or encryption modes in source code.
  - Components like encryption algorithms, key size, hash rounds, cipher modes... should be configured via files or environment variables.
  - Prefer using modern algorithms like AES-GCM, SHA-256, Argon2, ChaCha20.
  - Algorithm changes should not require modifying core logic in code.
- **Applies to**: All languages
- **Tools**: Manual Review, Semgrep (custom rule), Secret Scanners, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, MAINTAINABILITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: low

### 📘 Rule S009 – Do not use insecure encryption modes, padding, or cryptographic algorithms

- **Objective**: Prevent security vulnerabilities from using outdated encryption or hash algorithms, insecure padding/encryption modes leading to data exposure, pattern leakage, or padding oracle attacks.
- **Details**:
  - Do not use ECB mode in symmetric encryption (AES/3DES/Blowfish), as it reveals identical data patterns.
  - Avoid insecure padding like `PKCS#1 v1.5` in RSA, vulnerable to padding oracle attacks.
  - Do not use encryption algorithms with block size < 128-bit like `Triple-DES`, `Blowfish`, vulnerable to brute-force and collision attacks.
  - Do not use weak hash functions that have been broken like `MD5`, `SHA-1`.
  - In cases requiring backward compatibility (legacy), must isolate and clearly warn.
- **Applies to**: All languages
- **Tools**: SonarQube (S2070, S4790, S5547), Semgrep (crypto rules), Manual Review
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S010 – Must use cryptographically secure random number generators (CSPRNG) for security purposes

- **Objective**: Prevent attackers from guessing security random values like OTP, session ID, recovery tokens... by ensuring they are generated from Cryptographically Secure PRNG provided by cryptographic libraries/modules.
- **Details**:
  - Absolutely do not use `Math.random()` or similar functions for security values.
  - Always use functions designed for cryptographic purposes, e.g.: `crypto.randomBytes()`, `SecureRandom`, `secrets`, `crypto/rand`, etc.
  - Applies to: OTP codes, password reset tokens, session IDs, magic links, temporary file names, security GUIDs...
  - Ensure generated values are sufficiently long (entropy ≥ 128 bits) and non-repeating.
- **Applies to**: All languages
- **Tools**: SonarQube (S2245), Semgrep (random-insecure), Manual Review
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: high

### 📘 Rule S011 – GUIDs used for security purposes must be generated according to UUID v4 standard with CSPRNG

- **Objective**: Prevent guessing, recreation, or exploitation of GUIDs when used as identifiers for sensitive resources, by ensuring GUIDs are generated according to UUID v4 standard with cryptographically secure random number generators (CSPRNG).
- **Details**:
  - Avoid using UUID v1 as it may leak MAC address and timestamp.
  - Do not use UUID v3/v5 as they are based on hash from input → can be recreated if input is known.
  - UUID v4 must be randomly generated (random-based), using CSPRNG.
  - Mandatory when GUID is used as: password reset ID, session ID, magic link, API key, authentication token...
- **Applies to**: All languages
- **Tools**: Manual Review, Semgrep (uuid version rules), Static Analyzer, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: high

### 📘 Rule S012 – Protect secrets and encrypt sensitive data

- **Objective**: Protect encryption keys, passwords, access tokens, API keys... from exposure through source code, `.env` files, or logs. Ensure sensitive data is always encrypted and keys are managed securely through Key Vault or HSM.
- **Details**:
  - Do not store private keys, JWT secrets, passwords in `.env` files, config JSON, YAML without encryption or strict access management.
  - Absolutely do not hardcode secrets in source code (even test keys).
  - Must use secure secret management systems like AWS Secrets Manager, HashiCorp Vault, Azure Key Vault, GCP Secret Manager, or HSM.
  - Sensitive data like payment information, passwords, PII must be encrypted before storing in database or storage.
  - Logs must exclude or obfuscate secret information.
- **Applies to**: All languages
- **Tools**: SonarQube (S2068, S5547), GitLeaks, TruffleHog, Semgrep (hardcoded-secrets), Secret Scanner CI/CD
- **Principles**: SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S013 – Always use TLS for all connections

- **Objective**: Protect data in transit from leakage and Man-In-The-Middle (MITM) attacks by requiring all connections to use TLS (HTTPS), not allowing fallback to unencrypted protocols like HTTP.
- **Details**:
  - Absolutely do not allow HTTP communication for any API, login forms, or personal data (PII).
  - TLS mandatory for all:
    - Frontend → backend communication (browser, mobile)
    - Backend → third-party API communication
    - Service-to-service communication
  - Disable debug/localhost unencrypted modes in production.
  - Enable HSTS (HTTP Strict Transport Security) on frontend to enforce HTTPS.
- **Applies to**: All languages
- **Tools**: OWASP ZAP, SSLyze, Lighthouse, Static Analyzer (Semgrep/ESLint), Manual Review, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: high

### 📘 Rule S014 – Only use TLS 1.2 or 1.3

- **Objective**: Protect network communication from attacks exploiting older TLS versions like BEAST, POODLE, Heartbleed, or downgrade attacks by only allowing TLS 1.2 or 1.3.
- **Details**:
  - TLS 1.0 and 1.1 are no longer supported by modern browsers and systems.
  - Older versions are vulnerable to downgrade attacks or encryption vulnerability exploitation.
  - Need explicit configuration on backend applications, reverse proxy (NGINX, Apache), and any SSL clients.
  - Prefer default configuration as TLS 1.3 if supported.
- **Applies to**: All languages
- **Tools**: SSLyze, testssl.sh, OWASP ZAP, Manual Review, Configuration Scanner
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S015 – Only accept trusted TLS certificates and eliminate weak ciphers

- **Objective**: Protect client-server connections from MITM attacks by only trusting valid TLS certificates signed by trusted CAs, and rejecting all unknown certificates or weak ciphers.
- **Details**:
  - Always use valid TLS certificates from trusted CAs. If using internal or self-signed certificates, only trust certificates that are explicitly configured (pinning, specific trust store).
  - Absolutely do not accept any certificate automatically (e.g., `rejectUnauthorized: false`, `InsecureSkipVerify: true`).
  - Disable outdated cipher suites like RC4, 3DES, NULL cipher, or those using MD5, SHA1.
  - Prefer modern ciphers like AES-GCM, ChaCha20-Poly1305, TLS_ECDHE.
  - Use tools like [SSL Labs](https://www.ssllabs.com/ssltest/), `testssl.sh`, or `nmap --script ssl-enum-ciphers` to scan TLS configuration.
- **Applies to**: All languages
- **Tools**: SSL Labs, testssl.sh, nmap ssl-enum-ciphers, Manual Review
- **Principles**: SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S016 – Do not pass sensitive data via query string

- **Objective**: Prevent leakage of sensitive data through URLs by not passing sensitive information via query string, instead using HTTP body or headers in authenticated requests or private operations.
- **Details**:
  - Do not put authentication tokens, passwords, OTP codes, personal information (PII) in query strings like `GET /api/reset?token=...`
  - Data in query strings is easily:
    - Logged by server/proxy/load balancer
    - Cached in browser, proxy
    - Saved in history, bookmarks
  - Only pass sensitive data through:
    - HTTP body (POST/PUT)
    - Custom headers (Authorization, X-Token, ...)
- **Applies to**: All languages
- **Tools**: Semgrep (hardcoded query pattern), Manual Review, Proxy log scanner, SonarQube (custom rule)
- **Principles**: SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: high

### 📘 Rule S017 – Always use parameterized queries

- **Objective**: Prevent various forms of injection (SQL Injection, HQL Injection, NoSQL Injection) by always using parameterized queries or ORM when accessing data.
- **Details**:
  - Must not directly concatenate user input values into queries.
  - Use secure query mechanisms such as:
    - `?`, `$1`, `:param`, or binding in ORM
    - Entity Framework, JPA, Sequelize, GORM, etc.
  - For NoSQL (MongoDB, Firebase...), should not use raw JS queries based on string input.
  - If absolutely unavoidable to use parameters, must properly escape input according to the engine being used (not recommended).
- **Applies to**: All languages
- **Tools**: SonarQube (S2077, S3649), Semgrep (injection rules), CodeQL, Manual Review
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: critical

### 📘 Rule S018 – Prefer Allow List for Input Validation

- **Objective**: Ensure all user inputs or external data sources are strictly validated by accepting only known good values. This reduces the risk of attacks like XSS, SQL Injection, and other security issues caused by malformed or unexpected input.
- **Details**:
    - Always use an **Allow List** to validate input values – only allow explicitly defined values or types (e.g., positive integers, valid emails, safe strings).
    - Avoid using **Deny Lists** because:
        - They are prone to bypass due to incomplete coverage.
        - They fail to catch new or unknown attack patterns.
    - Apply to all input sources, including:
        - HTML form fields
        - URL parameters, HTTP headers, cookies
        - REST API payloads or batch file inputs
        - RSS feeds, webhook payloads, etc.
    - Validation should be based on:
        - Data types (number, string, boolean)
        - Specific patterns (regex, enums)
        - Range or length restrictions
    - Use standard validation libraries where available:
        - `class-validator`, `joi`, `yup`, `express-validator` (JavaScript)
        - `javax.validation` (Java), `FluentValidation` (C#), `Cerberus` (Python), etc.
- **Applies to**: All languages
- **Tools**: Static Analysis (Semgrep, SonarQube), Manual Review, Input Validation Libraries
- **Principles**: SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S019 – Sanitize input before sending emails to prevent SMTP Injection
- **Objective**: Prevent SMTP/IMAP injection by removing control characters and ensuring proper formatting of user input used in email sending.
- **Details**:
    - SMTP Injection occurs when input contains `\r`, `\n` which can inject new lines or alter email content.
    - Risks: hidden email sending, modified content, header spoofing, or spam.
    - Prevention:
        - Strip or reject control characters (`\n`, `\r`) in `to`, `subject`, `cc`, `bcc`, `reply-to`.
        - Validate email format strictly before use.
        - Prefer using secure email APIs like SendGrid, Amazon SES, Mailgun instead of direct SMTP protocol.
- **Applies to**: All languages
- **Tools**: Semgrep (regex match), Manual Review, Static Analysis, SonarQube (custom rule)
- **Principles**: SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S020 – Avoid using `eval()` or executing dynamic code
- **Objective**: Prevent Remote Code Execution (RCE) by disallowing use of dynamic code execution functions like `eval()`, `Function()`, `exec()`, `Runtime.exec()` with user-controlled input.
- **Details**:
    - Functions like `eval()`, `exec()`, `new Function()`, or `setTimeout(..., string)` allow arbitrary code execution, dangerous with untrusted input.
    - Attackers can execute system commands, read files, or manipulate databases remotely.
    - Alternatives to `eval`:
        - Object mapping or switch-case for dynamic logic
        - JSON parsing for data structures
        - `safe-eval` library (only within a sandboxed scope)
- **Applies to**: All languages
- **Tools**: Semgrep (eval-detection rules), ESLint (`no-eval`), SonarQube (S1523), Static Analyzer
- **Principles**: SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: high

### 📘 Rule S021 – Sanitize user-generated Markdown, CSS, and XSL content
- **Objective**: Prevent script injection via user-generated content in Markdown, CSS, or XSL.
- **Details**:
    - Attackers may abuse Markdown parser or render engine to inject JS, malicious links, or dangerous attributes (`onload`, `style=...`).
    - For XSL, non-sandboxed processing or external entity access can lead to XXE or XSLT injection.
    - Prevention:
        - Use libraries like `marked.js`, `markdown-it` with `sanitize: true` or XSS filter plugins.
        - Avoid rendering tags like `style`, `script`, `iframe`, or `javascript:` URLs.
        - For CSS/XSL, use sandboxed rendering engines and escape output before rendering.
- **Applies to**: All languages
- **Tools**: DOMPurify, sanitize-html, markdown-it, Bandit (Python), Manual Review, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S022 – Escape data properly based on output context
- **Objective**: Prevent XSS, Header Injection, Email Injection by escaping output data according to context (HTML, JS, URL, Header, Email, etc).
- **Details**:
    - Use the correct escaping strategy for each context:
        - **HTML content**: escape `&`, `<`, `>`, `"`, `'`
        - **HTML attributes**: escape `"` and `'` values
        - **JavaScript inline**: escape strings to avoid arbitrary execution
        - **URL params**: use `encodeURIComponent()`
        - **HTTP headers**: strip `\r`, `\n` to prevent injection
        - **SMTP email**: filter control characters like `\r`, `\n`, `bcc:` from content
    - Avoid using a single escape function for all cases.
- **Applies to**: All languages
- **Tools**: ESLint (`no-script-url`, `react/no-danger`), Bandit, SonarQube (S2076), DOMPurify, Manual Review
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S023 – Prevent JSON Injection and JSON eval attacks
- **Objective**: Prevent JavaScript execution via unsafe JSON handling or injection attacks.
- **Details**:
    - Never use `eval()` to process JSON from users.
    - Use proper JSON parsers:
        - JavaScript: `JSON.parse()`
        - Python: `json.loads()`
        - Java: `Gson`, `Jackson`, `ObjectMapper`
    - When rendering raw JSON into HTML, escape dangerous sequences like `</script>` or `</`.
    - Validate data before embedding JSON into `<script>` tags.
- **Applies to**: All languages
- **Tools**: ESLint (`no-eval`), Semgrep (`eval-dynamic`, `json-injection`), Bandit, SonarQube (S1523), Manual Review
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: high

### 📘 Rule S024 – Protect against XPath Injection and XML External Entity (XXE)
- **Objective**: Prevent XPath injection and XXE vulnerabilities that can expose files, trigger SSRF, or run malicious code.
- **Details**:
    - **XPath Injection**:
        - Never inject user data directly into XPath queries.
        - Use parameterized APIs or safe XPath binding mechanisms.
    - **XXE**:
        - Disable external entity processing in XML parsers to prevent local file access or SSRF.
        - Disable general and parameter entity processing in DOM/SAX/lxml parsers.
- **Applies to**: All languages
- **Tools**: Semgrep (xpath injection), Bandit (Python), SonarQube (S2755), Manual Config Review
- **Principles**: SECURITY
- **Version**:
- **Status**: draft
- **Severity**: medium

### 📘 Rule S025 – Always validate client-side data on the server
- **Objective**: Ensure all data from clients is validated server-side to prevent attacks from forged or malicious input.
- **Details**:
    - Client-side validation is only for UX – it can be bypassed.
    - Server-side validation is the last defense before DB writes or API calls.
    - Benefits:
        - Blocks SQLi, XSS, Buffer Overflow, SSRF
        - Preserves data integrity (valid enums, length limits, etc.)
        - Testable via unit tests
    - Recommended libraries:
        - Java: Hibernate Validator, Spring `@Valid`
        - Node.js: Joi, express-validator
        - Python: pydantic, marshmallow
- **Applies to**: All languages
- **Tools**: SonarQube (S5334), ESLint (`require-validate`), Bandit (Python), Static Analysis
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S026 – Apply JSON Schema Validation to input data
- **Objective**: Ensure all incoming JSON is fully validated by schema (structure, types, constraints) before processing.
- **Details**:
    - JSON schema can enforce:
        - Required fields, type enforcement
        - Constraints like length, min/max values, format checks
        - Reduce injection risk or logic bugs from malformed JSON
    - Language-specific tools:
        - Java: use Jackson + Hibernate Validator (`@Valid`, `@Email`, `@Min`)
        - JavaScript: use `ajv`, `joi`
        - Python: use `jsonschema`, `pydantic`
        - Go: use `gojsonschema`
        - C#: use `NJsonSchema`
- **Applies to**: All languages
- **Tools**: AJV, jsonschema, Joi, Pydantic, Hibernate Validator, SonarQube (custom rule), Manual Review
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S027 – Never expose secrets in source code or Git
- **Objective**: Prevent leakage of credentials, API keys, tokens, or sensitive config via source code or version control.
- **Details**:
    - Common leak sources:
        - `.env`, `config.yaml`, `secrets.json`, or hardcoded values like `API_KEY`, `JWT_SECRET`
    - Mitigation:
        - Use `.gitignore` to exclude secret files
        - Scan commits with GitLeaks, TruffleHog, detect-secrets
        - Add pre-commit hooks to block secret-containing files
        - If leaked, rotate keys, revoke tokens, and clean Git history
    - Additional notes:
        - Avoid plaintext secrets in CI/CD pipelines
        - Store secrets in a secure vault (e.g., AWS Secrets Manager, HashiCorp Vault)
- **Applies to**: All languages
- **Tools**: GitLeaks, TruffleHog, detect-secrets, git-secrets, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S028 – Limit upload file size and number of files per user
- **Objective**: Prevent resource abuse and protect against DoS attacks by limiting file size, number of files, and user storage usage.
- **Details**:
    - Must enforce limits on:
        - **Maximum file size** (e.g., ≤ 10MB)
        - **Total number of files** per user or per upload
        - **Total storage quota per user** (if applicable)
    - Limits should be:
        - Enforced on both client-side and server-side (server is mandatory)
        - Handled via HTTP layer or upload middleware
        - Logged when violations occur for abuse tracking
    - Technology examples:
        - Node.js: `multer` (`limits.fileSize`, `fileFilter`)
        - Python: `Flask-Limiter`, request body size limit
        - Java: Spring's `multipart.maxFileSize`, `maxRequestSize`
        - Nginx/nginx-ingress: `client_max_body_size`
- **Applies to**: All languages
- **Tools**: Manual Review, Static Analysis, API Gateway Limit, Nginx Config, WAF, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S029 – Apply CSRF protection for authentication-related features
- **Objective**: Prevent Cross-Site Request Forgery (CSRF) attacks where an attacker triggers unauthorized actions using the victim's authenticated session.
- **Details**:
    - CSRF occurs when:
        - Victim is logged in (cookies exist)
        - Browser automatically sends cookies with attacker-forged requests
    - **Protection mechanisms**:
        - **CSRF Token**: Generate a unique token (per session/request), attach it in form or header, and validate server-side
        - **SameSite Cookie**:
            - `SameSite=Lax`: suitable for most form-based POST requests
            - `SameSite=Strict`: most secure, may affect UX
            - `SameSite=None; Secure`: required for cross-domain cookies (must use HTTPS)
        - **2FA or re-authentication** for critical actions like changing email/password or performing transactions
    - For API or SPA:
        - Avoid storing access tokens in cookies
        - Prefer using `Authorization: Bearer <token>` to eliminate CSRF risk
- **Applies to**: All languages
- **Tools**: Spring Security CSRF, Express `csurf`, Django CSRF middleware, Helmet.js, Manual Review, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: high

### 📘 Rule S030 – Disable directory browsing and protect sensitive metadata files
- **Objective**: Prevent unauthorized access to file listings or metadata files such as `.git`, `.env`, `.DS_Store`, which can reveal sensitive system or source code information.
- **Details**:
    - Directory browsing occurs if no `index.html` exists or misconfigured server
    - Sensitive files may be exposed if not explicitly blocked, e.g.:
        - `.git/config` → contains repo URL or credentials
        - `.env` → secrets
        - `.DS_Store`, `Thumbs.db`, `.svn` → folder structure leaks
    - **Mitigation**:
        - Disable `autoindex` or `Indexes` on the web server (Apache/Nginx)
        - Deny access to metadata or dotfiles (`.git`, `.env`, etc.)
        - Review default config of frameworks (Express, Spring, Django, etc.)
        - Use `.gitignore` to exclude sensitive files from version control
- **Applies to**: All languages
- **Tools**: Static Analysis, Manual Review, Burp Suite, Nikto, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S031 – Set the Secure flag on session cookies for HTTPS protection
- **Objective**: Prevent attackers from stealing session cookies via unencrypted HTTP, especially on public or monitored networks (MITM).
- **Details**:
    - If a **cookie lacks the `Secure` flag**, it may be sent over plain HTTP
    - Attackers on public Wi-Fi or LAN may intercept session tokens
    - Sensitive cookies should always include:
        - `Secure`: only send via HTTPS
        - `HttpOnly`: prevent JS access
        - `SameSite`: control CSRF exposure
    - **Best practices**:
        - Use HTTPS in all environments (dev, staging, prod)
        - Ensure web server enforces HTTP → HTTPS redirects
- **Applies to**: All languages
- **Tools**: OWASP ZAP, Burp Suite, Static Analysis, Manual Review, SonarQube (custom rule)
- **Principles**: SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: high

### 📘 Rule S032 – Enable HttpOnly attribute for Session Cookies to prevent JavaScript access

- **Objective**: Prevent JavaScript (including malicious code during XSS attacks) from accessing session cookies, thereby limiting the risk of theft and session hijacking.
- **Details**:
  - **Without `HttpOnly`** → JavaScript can call `document.cookie` and read all session data.
  - XSS attacks exploit this vulnerability to steal tokens or cookies.
  - `HttpOnly` is one of the most important security flags along with:
    - `Secure`: only send cookies over HTTPS.
    - `SameSite`: limit CSRF attacks.
  - Combine with:
    - Comprehensive XSS protection (escaping, CSP).
    - Cookie validation in all environments (QA, prod, staging).
- **Applies to**: All languages
- **Tools**: Static Analysis, OWASP ZAP, Burp Suite, Manual Review, SonarQube (custom rule)
- **Principles**: SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S033 – Set SameSite attribute for Session Cookies to reduce CSRF risk

- **Objective**: Limit the browser's ability to automatically send cookies in cross-origin requests, thereby minimizing the risk of Cross-Site Request Forgery (CSRF) attacks.
- **Details**:
  - **Without `SameSite`**, cookies will be sent in all requests – even when coming from malicious sites → easily exploited in CSRF attacks.
  - `SameSite` values:
    - `Strict`: highest security, does not send cookies when redirected from other sites (best CSRF prevention).
    - `Lax`: common default, allows cookies to be sent with GET navigation requests (form submissions...).
    - `None`: **must be used with `Secure`** if the application needs to work cross-domain (e.g., SPA frontend calling API from different domain).
  - `SameSite` needs to work together with:
    - `HttpOnly` to prevent JS access.
    - `Secure` to only send over HTTPS.
  - Always check that the backend application has set `SameSite` correctly, and cookies are not reset incorrectly due to duplicate headers.
- **Applies to**: All languages
- **Tools**: OWASP ZAP, Postman, Static Analysis, Manual Review, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S034 – Use `__Host-` prefix for Session Cookies to prevent subdomain sharing

- **Objective**: Prevent cookie theft between subdomains (e.g., `api.example.com` accessing cookies from `admin.example.com`) by using cookies prefixed with `__Host-`, which enforce strict security tied to the root domain.
- **Details**:
    - The `__Host-` prefix enforces:
        - Must include `Secure`
        - Must not specify `Domain` (defaults to root domain)
        - `Path` must be `/`
    - Advantages:
        - Cookie exists only on the root domain (e.g., `example.com`), cannot be overridden by subdomains.
        - Prevents scenarios like:
            - A malicious app on `sub1.example.com` sets a fake `sessionId`, which is then reused on `example.com`.
    - Commonly used for:
        - Session cookies
        - CSRF tokens
        - Auth tokens
    - Limitation:
        - Only applicable over **HTTPS** and must be set from the root domain.
- **Applies to**: All languages
- **Tools**: Manual Review, Static Analysis, Chrome DevTools Audit, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S035 – Set the `Path` attribute for Session Cookies to limit access scope

- **Objective**: Reduce the risk of session cookie leaks or abuse across multiple apps under the same domain (e.g., `/app1` and `/app2`) by limiting cookie scope via the `Path` attribute.
- **Details**:
    - Cookies with `Path=/` are sent with **all requests under the same domain**, including unrelated applications.
    - Risk in shared domain environments:
        - Example: `example.com/app1`, `example.com/app2`
        - Cookie from `app1` will also be sent to `app2` unless `Path` is restricted.
    - Best practices:
        - Set specific `Path` (e.g., `/app1/`) so cookies only work within that path.
        - Avoid empty `Path` (`""`) – which defaults to `/`.
- **Applies to**: All languages
- **Tools**: Static Analysis, Manual Review, Chrome DevTools, Postman, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S036 – Prevent LFI and RFI using path validation and allow-lists

- **Objective**: Block Local File Inclusion (LFI) and Remote File Inclusion (RFI) attacks where attackers access sensitive files (e.g., `/etc/passwd`, `C:\Windows\system32`) or execute code from external URLs.
- **Details**:
    - **LFI**: The app accepts unchecked file paths → attacker reads internal files.
    - **RFI**: The app includes external files from user input → leads to Remote Code Execution.
    - Preventive measures:
        - Use **Allow List** of valid filenames/paths.
        - Never include/load user input directly.
        - Disallow URL usage in include/require/load/open.
        - Disable remote includes (e.g., `allow_url_include=Off` in PHP).
        - Normalize paths to remove `../` (path traversal).
        - Restrict permissions via sandboxing.
- **Applies to**: All languages
- **Tools**: Static Analysis, OWASP ZAP, Burp Suite, Manual Review, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: high

### 📘 Rule S037 – Set anti-cache headers to prevent sensitive data leakage

- **Objective**: Prevent browsers from caching sensitive data such as tokens, personal information, or financial content which could leak when users share devices or use back/forward navigation.
- **Details**:
    - Modern browsers may cache:
        - Filled-in forms
        - Login results
        - Rendered tokens or confidential data
    - Recommended headers:
        - `Cache-Control: no-store, no-cache, must-revalidate`
        - `Pragma: no-cache`
        - `Expires: 0`
    - Use for:
        - Profile lookups, dashboard pages
        - Tokens, session content
    - Check using DevTools → Network → Headers.
- **Applies to**: All languages
- **Tools**: Static Analysis, Postman, Chrome DevTools, Manual Review, SonarQube (custom rule)
- **Principles**: SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S038 – Hide system version information in HTTP Headers

- **Objective**: Prevent attackers from discovering backend technologies (e.g., server, framework, OS) via HTTP response headers that can be used to target known vulnerabilities.
- **Details**:
    - Common leak examples:
        - `Server: nginx/1.23.0`
        - `X-Powered-By: Express`
        - `X-AspNet-Version`, `X-Runtime`, etc.
    - Preventive steps:
        - Disable or override these headers.
        - Use middleware or reverse proxy to strip response headers.
        - Verify using DevTools, curl, or Postman.
- **Applies to**: All languages
- **Tools**: Static Analysis, curl, Postman, Chrome DevTools, Burp Suite, SonarQube (custom rule)
- **Principles**: SECURITY
- **Version**:
- **Status**: draft
- **Severity**: medium

### 📘 Rule S039 – Never transmit Session Tokens via URL parameters

- **Objective**: Prevent session hijacking by ensuring session tokens are not stored in browser history, server logs, proxy logs, or leaked via Referrer headers.
- **Details**:
    - Risks of token in URL (e.g., `https://example.com/dashboard?sessionId=abc123`):
        - Saved in browser history
        - Logged on server/load balancer
        - Leaked via `Referer` to third parties
    - Best practices:
        - Use `Secure`, `HttpOnly` cookies for token storage
        - Use headers or body for API auth – never query string
- **Applies to**: All languages
- **Tools**: Static Analysis, Manual Review, Burp Suite, Postman, SonarQube (custom rule)
- **Principles**: SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: high

### 📘 Rule S040 – Regenerate Session Token after login to prevent Session Fixation

- **Objective**: Prevent attackers from setting a session ID before login and taking over the session post-login if the ID remains unchanged.
- **Details**:
    - Attack scenario:
        - Attacker sets known session ID before login
        - Victim logs in without regenerating session
        - Attacker reuses the same ID for access
    - Preventive actions:
        - Invalidate old session after login and create a new one
        - For JWT: issue new token on login
        - For cookies: delete old session and set a new cookie
- **Applies to**: All languages
- **Tools**: Static Analysis, Manual Review, OWASP ZAP, Burp Suite, SonarQube (custom rule)
- **Principles**: SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: high

### 📘 Rule S041 – Session Tokens must be invalidated after logout or expiration

- **Objective**: Prevent users from reusing old session tokens after logout or timeout, which could lead to session hijacking.
- **Details**:
    - Actions required:
        - **Backend**:
            - Remove session from memory (e.g., Redis)
            - Revoke token or blacklist old JWT
        - **Frontend**:
            - Delete cookie (`document.cookie = ...`, `res.clearCookie(...)`)
            - Remove tokens from localStorage
            - Redirect/reload after logout
    - Add `Cache-Control: no-store` to prevent old content reuse
- **Applies to**: All languages
- **Tools**: Static Analysis, Manual Review, Postman, DevTools, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S042 – Require re-authentication for long-lived sessions or sensitive actions

- **Objective**: Reduce the risk of session hijacking or privilege misuse by forcing re-authentication after long idle periods or before critical actions.
- **Details**:
    - When using persistent login or "Remember Me":
        - Require re-login after X hours (e.g., 12h, 24h)
        - Re-authenticate after inactivity (e.g., 30 mins)
        - Require password or 2FA for sensitive actions (password change, payments)
    - For JWT:
        - Use short-lived tokens with secure refresh logic
- **Applies to**: All languages
- **Tools**: Manual Review, Static Analysis (JWT expiry, session policy), Security Test, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S043 – Password changes must invalidate all other login sessions

- **Objective**: Ensure attackers cannot continue using old session tokens after a password change. Enforce correct access control after sensitive updates.
- **Details**:
    - On password change:
        - Invalidate all other active sessions (except current if necessary)
        - Clear all session tokens from DB, Redis, or memory
        - For JWT: use token versioning or timestamp to revoke old tokens
        - Require re-login across all devices
- **Applies to**: All languages
- **Tools**: Manual Review, Static Analysis (Token Revocation Logic), SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S044 – Require re-authentication before modifying critical information

- **Objective**: Prevent unauthorized changes to critical information when the session is not fully authenticated. Protect users in half-open session states.
- **Details**:
    - When updating sensitive information (password, email, payment method, access permissions, etc.):
        - Require password re-entry or two-factor authentication (2FA)
        - Do not store any information in session unless fully authenticated
        - If the session is in a temporary state (e.g., OTP not completed or social login not finished), **block access to sensitive resources**
    - On the frontend: redirect the user to the re-authentication screen
- **Applies to**: All languages
- **Tools**: Manual Review, Static Analysis (flow check), Security Test, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S045 – Implement brute-force protection for login

- **Objective**: Prevent brute-force and credential stuffing attacks by limiting failed login attempts and introducing friction for suspicious behavior.
- **Details**:
    - Implement one or more of the following:
        - Limit failed login attempts by IP or account (Rate Limiting)
        - Soft lockout: temporarily lock account (e.g., 15 minutes after 5 failed attempts)
        - Trigger CAPTCHA or 2FA after multiple failed attempts
        - Check passwords against breached password lists (e.g., HaveIBeenPwned, zxcvbn)
    - Log all failed login attempts for monitoring and alerting
- **Applies to**: All languages
- **Tools**: Manual Review, Static Analysis, OWASP ZAP, Custom Logging, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S046 – Notify users of critical account changes

- **Objective**: Alert users to sensitive actions to detect potential compromise and allow timely intervention.
- **Details**:
    - Notify users when performing actions such as:
        - Password reset
        - Changing email or phone number
        - Login from new devices or suspicious IPs
    - Notification channels: Email, Push Notification, or SMS
    - **Do not include sensitive info** (e.g., password, token, unencrypted links)
    - Log these events for security audit purposes
- **Applies to**: All languages
- **Tools**: Manual Review, Security Test, Notification Audit, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S047 – Secure temporary passwords and activation codes

- **Objective**: Ensure that temporary passwords and activation codes are secure, unpredictable, single-use, and time-limited.
- **Details**:
    - Temporary credentials must:
        - Be randomly generated using CSPRNG
        - Be at least **6 characters long**, and contain **letters and numbers**
        - Have short validity: **15 minutes to 24 hours**
        - Be **one-time use only**
        - **Must not** be used as permanent passwords
    - Additional protection:
        - Store only hashed values if persisted
        - Invalidate after use
        - Disallow regeneration until expired
- **Applies to**: All languages
- **Tools**: Manual Review, Static Analysis, Audit Flow, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: high

### 📘 Rule S048 – Do not expose current password during reset flow

- **Objective**: Ensure the current user password is never revealed or sent in any step of the password reset process.
- **Details**:
    - **Never display or send the current password** to the user
    - Do not email or SMS old passwords
    - During password reset:
        - Only ask for new password (with confirmation)
        - Use OTP/email/token for verification, not current password
    - If changing password while logged in, require **manual entry of current password**, never show it
- **Applies to**: All languages
- **Tools**: Manual Review, Penetration Test, SonarQube (custom rule)
- **Principles**: SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S049 – Authentication codes must expire quickly

- **Objective**: Ensure that OTPs, reset tokens, and activation links expire quickly to reduce risk of interception or reuse.
- **Details**:
    - Authentication codes must:
        - Expire quickly (⏱ recommended: **5–10 minutes**)
        - Be **automatically invalidated** after expiration
        - Be **one-time use only**
    - Do not accept expired or reused codes
    - For critical actions (reset password, email verification), require re-authentication after code validation
- **Applies to**: All languages
- **Tools**: Manual Review, Static Analysis, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S050 – Session tokens must have minimum 64-bit entropy and use secure algorithms

- **Objective**: Prevent attackers from predicting or forging session tokens by ensuring sufficient length, entropy, and cryptographic safety.
- **Details**:
    - Session tokens must have at least **64-bit entropy**, recommended: **128-bit or 256-bit**
    - Use approved cryptographic algorithms:
        - HMAC-SHA-256
        - AES-256
        - ChaCha20
    - Avoid weak algorithms:
        - MD5
        - SHA-1
    - Do not generate tokens using `Math.random()` or short guessable strings
    - Always use CSPRNG for token generation
- **Applies to**: All languages
- **Tools**: Manual Review, Static Analysis, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S051 – Support 12–64 character passwords; reject >128 characters

- **Objective**: Allow users to use strong passphrases while preventing resource abuse from excessively long inputs.
- **Details**:
    - Accept passwords with **12–64 characters**
        - Support strong passphrases (e.g. `correct horse battery staple`)
    - **Reject passwords >128 characters** to:
        - Prevent DoS from large inputs
        - Avoid poor hash performance on long strings
    - Optionally warn users if password <12 characters
- **Applies to**: All languages
- **Tools**: Manual Review, Static Analysis, Unit Test, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S052 – OTPs must have at least 20-bit entropy

- **Objective**: Ensure OTPs are strong enough to resist brute-force or statistical guessing attacks.
- **Details**:
    - OTP must have minimum **20-bit entropy**, equivalent to **6-digit random numbers** (`000000–999999`)
    - Generate OTPs using **CSPRNG**
    - Avoid using `Math.random()` or insecure generators
    - Alphanumeric or longer OTPs increase entropy and are preferred
- **Applies to**: All languages
- **Tools**: Manual Review, Unit Test, Static Analysis, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S053 – Only use secure OTP algorithms like HOTP/TOTP

- **Objective**: Ensure OTPs are secure against spoofing and replay attacks by using safe, standard algorithms.
- **Details**:
    - **Do not use**:
        - Weak algorithms like `MD5`, `SHA-1` (deprecated)
        - OTPs without expiration or usage limits
    - **Use standards**:
        - `HOTP` (HMAC-based OTP – [RFC 4226](https://datatracker.ietf.org/doc/html/rfc4226))
        - `TOTP` (Time-based OTP – [RFC 6238](https://datatracker.ietf.org/doc/html/rfc6238))
    - Recommended libraries:
        - `PyOTP` (Python), `otplib` (Node.js), `Google Authenticator` SDK (Java)
    - Always enforce time-based expiration (typically 30–300 seconds)
- **Applies to**: All languages
- **Tools**: Manual Review, Unit Test, Static Analysis, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S054 – Avoid using default accounts like "admin", "root", "sa"

- **Objective**: Prevent brute-force attacks and ensure traceability and accountability in auditing. Avoid predictable, shared accounts lacking identity association.
- **Details**:
    - Do not use default or common account names (e.g., admin, root, sa, test, guest, etc.).
    - Each user must have a separate account with role-based access control.
    - Force password change on first use or system initialization.
    - The system must log all login attempts and resource access per specific user.
- **Applies to**: All languages
- **Tools**: Manual Review, CI Security Audit, IAM Policy Scan, SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: high

### 📘 Rule S055 – Validate input Content-Type in REST services

- **Objective**: Prevent attacks via malformed or improperly handled data by validating incoming data format (e.g., JSON, XML).
- **Details**:
    - REST services must check the Content-Type HTTP header to ensure data format matches expectations (e.g., `application/json`, `application/xml`).
    - Reject requests with incorrect or unsupported Content-Type.
    - Avoid processing `text/plain`, `multipart/form-data` unless explicitly required.
    - Log rejected requests due to invalid Content-Type to detect attacks or client issues early.
- **Applies to**: All languages
- **Tools**: Manual Review, API Gateway Config, Static Code Analysis (Semgrep), SonarQube (custom rule)
- **Principles**: SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S056 – Protect against Log Injection attacks

- **Objective**: Prevent attackers from injecting fake log entries that distort tracking or exploit log analysis systems.
- **Details**:
    - Do not log user input directly without sanitization.
    - Escape special characters like: `\n`, `\r`, `%`, `\t`, `"`, `'`, `[`, `]`, etc.
    - Use structured logging (e.g., JSON) to detect anomalies more easily.
    - Avoid `string concatenation` when writing log entries with user input.
- **Applies to**: All languages
- **Tools**: SonarQube, Semgrep, Manual Review
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: high

### 📘 Rule S057 – Use synchronized time and UTC in logs

- **Objective**: Ensure consistent, accurate log timestamps to support auditing, investigation, and cross-system comparison.
- **Details**:
    - Always use UTC timezone for logging to avoid issues with local offsets or daylight saving.
    - Configure system time sync via NTP (Network Time Protocol).
    - Verify all backends, logging middleware, and log collectors use standard formats (`ISO 8601`, `UTC`, `RFC3339`, etc.).
    - Helps unify log data across services and regions.
- **Applies to**: All languages
- **Tools**: Manual Review, Audit Logging Middleware, Centralized Logging Tools (ELK, Fluentd, Datadog), SonarQube (custom rule)
- **Principles**: CODE_QUALITY, SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S058 – Protect applications from SSRF attacks

- **Objective**: Prevent Server-Side Request Forgery (SSRF) and protect internal networks or cloud metadata services from unauthorized access via untrusted input.
- **Details**:
    - Always validate URLs or network addresses from client input or HTTP metadata.
    - Apply allow lists for:
        - Valid protocols: only allow `https`, `http`
        - Specific domains or trusted internal IP ranges
        - Allowed ports (avoid sensitive ones like 22, 3306, 6379, etc.)
    - Block access to:
        - `127.0.0.1`, `::1` (localhost)
        - `169.254.169.254` (AWS metadata)
        - `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16` if not needed
    - Limit timeouts and disallow redirects unless required.
- **Applies to**: All languages
- **Tools**: SonarQube, Manual Review, Burp Suite Test
- **Principles**: SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: medium

### 📘 Rule S059 – Configure Allow List for server-side outbound requests

- **Objective**: Reduce risks from the server making outbound requests to untrusted systems (SSRF, malicious downloads, data leaks).
- **Details**:
    - All outbound connections (HTTP, FTP, DNS, etc.) must be restricted via allow lists.
    - Do not let the server freely access the internet or unknown domains by default.
    - Example restrictions:
        - Only allow sending files to trusted storage or domains like `https://trusted.example.com`.
        - Containers may only use DNS for allowed addresses.
    - In cloud/serverless environments, restrict outbound traffic using IAM policies, security groups, or network ACLs.
- **Applies to**: All languages
- **Tools**: Manual Config Review, Firewall/Proxy Logs, CloudTrail, Burp Suite Test, SonarQube (custom rule)
- **Principles**: SECURITY
- **Version**: 1.0
- **Status**: activated
- **Severity**: low
