# Contributing to Moon Shine

Thank you for your interest in contributing to Moon Shine! We welcome contributions from everyone. This document provides guidelines and information for contributors.

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Development Workflow](#development-workflow)
- [Submitting Changes](#submitting-changes)
- [Code Style](#code-style)
- [Testing](#testing)
- [Documentation](#documentation)

## Code of Conduct

This project follows our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to abide by its terms.

## Getting Started

### Prerequisites
- **Rust**: 1.70+ (see `rust-toolchain.toml`)
- **Moon**: 1.40+ (managed via proto)
- **Node.js**: 20+ (for tooling)
- **Git**: 2.30+

### Quick Start
```bash
# Clone the repository
git clone https://github.com/moonrepo/moon-shine.git
cd moon-shine

# Install dependencies
proto install moon rust node

# Run setup
moon run deps-fetch
moon run build

# Run tests
moon run test
```

## Development Setup

### Environment Setup
```bash
# Copy environment template
cp .env.example .env

# Edit with your API keys (optional)
# nano .env
```

### IDE Setup
We recommend:
- **VS Code** with rust-analyzer extension
- **CLion** with Rust plugin
- **Vim/Neovim** with rust.vim

## Development Workflow

### Daily Development
```bash
# Start development build (watches for changes)
moon run watch-build

# In another terminal, run tests continuously
moon run watch-test

# Format code
moon run format

# Lint code
moon run lint
```

### Feature Development
```bash
# Create a feature branch
git checkout -b feature/your-feature-name

# Make changes...
moon run build
moon run test

# Commit changes
git add .
git commit -m "feat: add your feature"

# Push and create PR
git push origin feature/your-feature-name
```

## Submitting Changes

### Pull Request Process
1. **Fork** the repository
2. **Create** a feature branch
3. **Make** your changes
4. **Test** thoroughly
5. **Update** documentation if needed
6. **Submit** a pull request

### PR Requirements
- [ ] Tests pass (`moon run test`)
- [ ] Code lints (`moon run lint`)
- [ ] Formatted (`moon run format`)
- [ ] Documentation updated
- [ ] Security audit passes (`moon run audit`)

### Commit Messages
We follow [Conventional Commits](https://conventionalcommits.org/):
```
type(scope): description

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

## Code Style

### Rust Code Style
- Follow `rustfmt` formatting (configured in `rustfmt.toml`)
- Use `clippy` lints (configured in `clippy.toml`)
- Write comprehensive documentation
- Use meaningful variable names

### General Guidelines
- **DRY**: Don't Repeat Yourself
- **SOLID**: Single responsibility, Open-closed, Liskov substitution, Interface segregation, Dependency inversion
- **Zero-cost abstractions** where possible
- **Memory safety** is paramount

## Testing

### Test Categories
- **Unit tests**: `cargo test` (fast, isolated)
- **Integration tests**: `cargo test --test integration`
- **Property tests**: `moon run test-property`
- **Chicago-style tests**: `moon run test-unit-chicago`
- **London-style tests**: `moon run test-unit-london`

### Coverage
```bash
# Generate coverage report
moon run test-with-coverage

# View coverage in browser
open target/coverage/index.html
```

### Performance Testing
```bash
# Run benchmarks
moon run bench

# Profile performance
moon run profile
```

## Documentation

### Code Documentation
- All public APIs must have documentation
- Use `cargo doc` to generate docs
- Examples in documentation should be testable

### User Documentation
- Keep README.md up to date
- Update CHANGELOG.md for releases
- Document breaking changes clearly

## Release Process

### Version Bumping
We use [cargo-release](https://github.com/crate-ci/cargo-release):
```bash
# Patch release
cargo release patch

# Minor release
cargo release minor

# Major release
cargo release major
```

### Pre-release Checklist
- [ ] All tests pass
- [ ] Security audit passes
- [ ] Dependencies updated
- [ ] Documentation updated
- [ ] Changelog updated

## Getting Help

### Communication Channels
- **Issues**: For bugs and feature requests
- **Discussions**: For questions and ideas
- **Discord**: For real-time chat

### Labels
- `good first issue`: Perfect for newcomers
- `help wanted`: Community contribution welcome
- `bug`: Something isn't working
- `enhancement`: New feature request
- `documentation`: Documentation improvements

## Recognition

Contributors are recognized in:
- CHANGELOG.md for significant contributions
- GitHub's contributor insights
- Release notes

Thank you for contributing to Moon Shine! 🚀