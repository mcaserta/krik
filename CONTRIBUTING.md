# Contributing to Krik

Thank you for your interest in contributing to Krik! This document provides
guidelines for contributing to the project.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/your-username/krik.git
   cd krik
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. Run tests to ensure everything works:
   ```bash
   cargo test
   ```

## Development Environment

### Prerequisites

- Rust 1.70 or later
- Git
- For PDF generation: pandoc and typst-cli

### Building

```bash
cargo build --release
./target/release/kk --help
```

### Running Tests

```bash
cargo test                    # Run all tests
cargo test -- --nocapture    # Run tests with output
cargo test <test_name>        # Run specific test
```

## Code Standards

### Rust Style Guidelines

- Follow the official Rust style guide
- Use `cargo fmt` to format code before committing
- Run `cargo clippy` to catch common mistakes and improve code quality
- Use `cargo check` to verify compilation without building

### Code Organization

- **Module Structure**: Follow the existing module hierarchy:
  - `cli/` - Command-line interface and argument parsing
  - `generator/` - Core site generation logic
  - `server/` - Development server with live reload
  - `theme/` - Theme management
  - `i18n/` - Internationalization support
  - `error/` - Error handling and recovery

- **File Naming**: Use snake_case for file names and module names
- **Function Naming**: Use snake_case for functions and variables
- **Constants**: Use SCREAMING_SNAKE_CASE for constants

### Error Handling

- Use `thiserror` for custom error types
- Implement proper error recovery where possible (see `error/recovery.rs`)
- Provide meaningful error messages for users
- Use `Result<T, E>` for functions that can fail

### Documentation

- Add rustdoc comments for public APIs
- Include examples in documentation where helpful
- Update CHANGELOG.md for user-facing changes
- Keep README.md up to date

### Dependencies

- Minimize external dependencies
- Prefer well-maintained crates with good documentation
- Check security advisories before adding new dependencies
- Update Cargo.toml with appropriate version constraints

## Testing Strategy

### Test Categories

1. **Unit Tests**: Test individual functions and modules
   - Located alongside source code using `#[cfg(test)]`
   - Focus on pure functions and isolated components

2. **Integration Tests**: Test complete workflows and feature interactions
   - Located in `tests/` directory
   - Test CLI commands, file generation, and server functionality
   - Examples: `tests/content.rs`, `tests/markdown.rs`, `tests/feeds.rs`

3. **Smoke Tests**: Basic functionality verification
   - Quick tests to verify major features work
   - Example: `tests/pdf_smoke.rs`

### Writing Tests

- Use descriptive test names that explain what is being tested
- Create temporary directories for tests that create files
- Clean up test artifacts (temporary files/directories)
- Test both success and failure cases
- Use `assert!`, `assert_eq!`, and `assert_ne!` appropriately

Example test structure:

```rust
#[test]
fn test_feature_name() {
    // Setup
    let temp_dir = std::env::temp_dir().join("krik_test_unique_id");
    std::fs::create_dir_all(&temp_dir).unwrap();

    // Execute
    let result = your_function(&temp_dir);

    // Verify
    assert!(result.is_ok());
    assert!(expected_file.exists());

    // Cleanup (optional, temp_dir will be cleaned by OS)
}
```

### Test Requirements

- All new features must include tests
- Bug fixes should include regression tests
- Tests should be fast and independent
- Use `cargo test` locally before submitting PRs

## Release Process

### Version Management

Krik uses semantic versioning (SemVer):

- **Major** (X.y.z): Breaking changes to public API
- **Minor** (x.Y.z): New features, backwards compatible
- **Patch** (x.y.Z): Bug fixes, backwards compatible

### Release Steps

1. **Prepare Release**

   ```bash
   # Update version in Cargo.toml
   # Update CHANGELOG.md with new version and changes
   # Run full test suite
   cargo test
   cargo clippy
   cargo fmt --check
   ```

2. **Create Release Commit**

   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "Release version X.Y.Z"
   ```

3. **Tag Release**

   ```bash
   git tag -a vX.Y.Z -m "Release version X.Y.Z"
   git push origin main --tags
   ```

4. **Publish to Crates.io**

   ```bash
   cargo publish --dry-run  # Verify package contents
   cargo publish
   ```

5. **GitHub Release**
   - Create GitHub release from tag
   - Include changelog entries
   - Attach binary releases if applicable

### Pre-release Checklist

- [ ] All tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] Version is bumped in Cargo.toml
- [ ] Features work end-to-end

### Continuous Integration

The project uses GitHub Actions for CI/CD:

- **Build**: Compile for multiple targets
- **Test**: Run test suite
- **Deploy**: Automatically deploy documentation site
- **Release**: Automated releases on tag push

## Pull Request Guidelines

### Before Submitting

1. Ensure your code follows the style guidelines
2. Add or update tests for your changes
3. Update documentation if needed
4. Run the full test suite locally
5. Check that CI passes

### PR Description

- Clearly describe what your PR does
- Reference any related issues
- Include testing instructions if applicable
- Note any breaking changes

### Review Process

- All PRs require review before merging
- Address reviewer feedback promptly
- Keep PRs focused and reasonably sized
- Squash commits if requested

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and contribute
- Follow GitHub's community guidelines

## Getting Help

- Open an issue for bugs or feature requests
- Use discussions for questions
- Check existing issues before creating new ones
- Provide minimal reproducible examples for bugs

## License

By contributing to Krik, you agree that your contributions will be licensed
under the MIT License.
