# Release Process for Krik

This document describes the automated release process for Krik, which creates
cross-platform binaries and GitHub releases.

## Overview

Krik uses an automated release system that:

- Builds binaries for 5 platforms (Linux x64/ARM64, macOS x64/ARM64, Windows
  x64)
- Creates GitHub releases with changelog integration
- Generates checksums for security verification
- Updates the `latest` tag for easy access
- Optimizes deployment workflows by using pre-built binaries

## Release Workflow

### Automatic Releases

**Triggered by**: Pushing version tags (e.g., `v0.1.24`, `v0.2.0`)

```bash
# Create and push a version tag
git tag v0.1.24
git push origin v0.1.24
```

This will automatically:

1. Create a GitHub release with changelog notes
2. Build binaries for all supported platforms
3. Upload binaries and checksums to the release
4. Update the `latest` tag for deployment workflows

### Manual Releases

**Triggered by**: Manual workflow dispatch in GitHub Actions

1. Go to the Actions tab in GitHub
2. Select "Release Krik Binaries" workflow
3. Click "Run workflow"
4. Enter the desired tag (e.g., `v0.1.24`)
5. Click "Run workflow"

## Supported Platforms

The release workflow builds binaries for:

| Platform            | Target                      | Binary Name             |
| ------------------- | --------------------------- | ----------------------- |
| Linux x64           | `x86_64-unknown-linux-gnu`  | `kk-linux-x86_64`       |
| Linux ARM64         | `aarch64-unknown-linux-gnu` | `kk-linux-aarch64`      |
| macOS Intel         | `x86_64-apple-darwin`       | `kk-macos-x86_64`       |
| macOS Apple Silicon | `aarch64-apple-darwin`      | `kk-macos-aarch64`      |
| Windows x64         | `x86_64-pc-windows-msvc`    | `kk-windows-x86_64.exe` |

## Changelog Integration

The release process automatically extracts changelog notes from `CHANGELOG.md`:

- **For tagged releases**: Extracts the section matching the version (e.g.,
  `## [0.1.24]`)
- **For manual releases**: Falls back to the `[Unreleased]` section if version
  not found

**Format requirements**:

```markdown
## [0.1.24] - 2025-01-30

### Added

- New feature descriptions

### Fixed

- Bug fix descriptions
```

## Deployment Integration

The site deployment workflow (`build-and-deploy.yml`) automatically uses
pre-built binaries:

### Performance Benefits

- **Build time**: ~8-10 minutes → ~30 seconds
- **Resource usage**: No Rust compilation in deployment
- **Reliability**: Pre-tested, verified binaries

### Fallback Strategy

If pre-built binaries are unavailable:

1. Downloads fail → Compile from source
2. Binary verification fails → Compile from source
3. Source compilation fails → Workflow fails with clear error

## Binary Verification

Each binary includes:

- **SHA256 checksums** for integrity verification
- **Version verification** during deployment
- **Cross-platform testing** during release

## Security

### Checksum Verification

```bash
# Verify binary integrity
shasum -a 256 -c kk-linux-x86_64.sha256
```

### Access Control

- Releases require push access to main repository
- Manual releases require Actions workflow permissions
- All release assets are publicly downloadable

## Troubleshooting

### Common Issues

**1. Release workflow fails on cross-compilation**

- Check if target architecture is supported
- Verify cross-compilation tools are installed
- Review the cross-rs/cross configuration

**2. Deployment uses source compilation instead of binary**

- Check if release exists and contains the expected binary
- Verify binary download URL is accessible
- Review binary verification logs in deployment workflow

**3. Changelog notes are empty or incorrect**

- Ensure `CHANGELOG.md` follows the expected format
- Check that version section exists (e.g., `## [0.1.24]`)
- Verify markdown formatting is correct

### Manual Intervention

If automated release fails:

1. Check workflow logs in GitHub Actions
2. Fix issues in the repository
3. Delete the failed release and tag if needed
4. Re-run the release process

```bash
# Delete failed release artifacts
git tag -d v0.1.24
git push origin :refs/tags/v0.1.24
```

## Version Management

### Semantic Versioning

- **MAJOR**: Breaking changes (`v1.0.0` → `v2.0.0`)
- **MINOR**: New features, backward compatible (`v0.1.0` → `v0.2.0`)
- **PATCH**: Bug fixes, backward compatible (`v0.1.23` → `v0.1.24`)

### Tag Format

- Always use `v` prefix: `v0.1.24` (not `0.1.24`)
- Use semantic versioning: `MAJOR.MINOR.PATCH`
- Avoid pre-release suffixes for main releases

### Release Cadence

- **Patch releases**: Bug fixes, weekly/bi-weekly
- **Minor releases**: New features, monthly/bi-monthly
- **Major releases**: Breaking changes, as needed

## Future Enhancements

Planned improvements to the release system:

- **Container images**: Docker images for containerized deployments
- **Package managers**: Homebrew, Chocolatey, APT packages
- **Release notes automation**: AI-generated summaries
- **Binary signing**: GPG signatures for enhanced security
- **Performance metrics**: Size and performance tracking across releases
