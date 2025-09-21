# Security Policy

## Supported Versions

We actively support the following versions with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 1.x.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in Moon Shine, please report it to us as follows:

### Contact
- **Email**: security@moonshine.dev
- **PGP Key**: [Download our PGP key](https://moonshine.dev/security/pgp-key.asc)
- **Response Time**: We will acknowledge your report within 48 hours

### What to Include
Please include the following information in your report:
- A clear description of the vulnerability
- Steps to reproduce the issue
- Potential impact and severity
- Any suggested fixes or mitigations

### Our Process
1. **Acknowledgment**: We'll acknowledge receipt within 48 hours
2. **Investigation**: We'll investigate and validate the vulnerability
3. **Fix Development**: We'll develop and test a fix
4. **Disclosure**: We'll coordinate disclosure with you
5. **Release**: We'll release the fix and security advisory

## Security Best Practices

### For Contributors
- Run `cargo audit` before committing
- Use `cargo deny` to check dependencies
- Follow the principle of least privilege
- Validate all inputs and outputs

### For Users
- Keep dependencies updated
- Use HTTPS for all communications
- Validate checksums of downloaded artifacts
- Report suspicious behavior immediately

## Known Security Considerations

### WASM Execution
Moon Shine executes WebAssembly code. While WASM provides sandboxing, users should:
- Only run trusted WASM modules
- Keep the WASM runtime updated
- Monitor resource usage

### AI Integration
When using AI enhancement features:
- API keys are handled securely
- Communications are encrypted
- No code is sent to AI services without explicit consent

## Responsible Disclosure

We kindly ask that you:
- Give us reasonable time to fix issues before public disclosure
- Avoid accessing user data or disrupting services
- Act in good faith to improve security

Thank you for helping keep Moon Shine and its users secure! 🛡️