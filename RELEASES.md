# Release Process

This document describes the automated release process for jxcape.

## Overview

The release process is fully automated through GitHub Actions. When a version tag is pushed, the system will:

1. **Build cross-platform binaries** for Linux, macOS, and Windows
2. **Run quality checks** (tests, clippy) on all platforms
3. **Create compressed release assets** with binaries and documentation
4. **Generate release notes** from the changelog using git-cliff
5. **Create a GitHub release** with all assets attached
6. **Publish to crates.io** automatically

## Supported Platforms

The release pipeline builds binaries for:

- **Linux x86_64**: `jxcape-linux-x86_64.tar.gz`
- **macOS x86_64**: `jxcape-macos-x86_64.tar.gz`
- **macOS ARM64**: `jxcape-macos-aarch64.tar.gz`
- **Windows x86_64**: `jxcape-windows-x86_64.zip`

Each archive contains:
- The `jxcape` binary (or `jxcape.exe` on Windows)
- `LICENSE` file
- `README.md` documentation
- `CHANGELOG.md` with full project history

## How to Release

### Prerequisites

1. **GitHub Secrets**: Ensure `CRATES_IO_TOKEN` is configured in repository settings
2. **Clean working directory**: Commit all changes before releasing
3. **Quality checks**: All tests and clippy checks must pass

### Step-by-Step Process

1. **Update version in Cargo.toml**:
   ```toml
   version = "0.3.0"  # Update to your target version
   ```

2. **Generate updated changelog**:
   ```bash
   just changelog
   ```

3. **Review and commit changes**:
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "chore: bump version to 0.3.0"
   ```

4. **Create and push the version tag**:
   ```bash
   git tag v0.3.0
   git push origin main
   git push origin v0.3.0
   ```

5. **Monitor the release**: Check the Actions tab for the release workflow progress

### Using the Helper Command

You can use the `just` command to automate most of the process:

```bash
# This will update Cargo.toml, generate changelog, and run quality checks
just prepare-release 0.3.0

# Then review changes and follow the final git commands shown
```

## Workflow Details

### Build and Release Job

- Runs on a matrix of operating systems and targets
- Installs Rust toolchain with the specific target
- Builds release binaries with optimizations
- Runs tests to ensure quality
- Creates compressed archives with binaries and docs
- Uploads artifacts for the release job

### Create Release Job

- Downloads all build artifacts
- Generates version-specific changelog content
- Creates a GitHub release with proper tagging
- Attaches all platform binaries as downloadable assets

### Publish to Crates.io Job

- Runs after successful release creation
- Publishes the crate to the official Rust package registry
- Uses the `CRATES_IO_TOKEN` secret for authentication

## Troubleshooting

### Release Fails During Build

- Check that all quality checks pass locally: `just check`
- Ensure Cargo.toml version is valid semver
- Verify all dependencies are properly specified

### Release Fails During Publishing

- Ensure `CRATES_IO_TOKEN` secret is correctly configured
- Check that the crate name isn't already taken (if this is the first release)
- Verify the version number isn't already published on crates.io

### Changelog Issues

- Ensure git-cliff configuration in `cliff.toml` is correct
- Check that commit messages follow conventional commit format
- Verify tag naming follows the `v*` pattern (e.g., `v1.0.0`)

## Emergency Procedures

### Deleting a Failed Release

If a release fails and you need to retry:

1. Delete the GitHub release (if created)
2. Delete the git tag locally and remotely:
   ```bash
   git tag -d vX.Y.Z
   git push origin :refs/tags/vX.Y.Z
   ```
3. Fix the issues and retry the release process

### Rolling Back a Release

If a release has critical issues:

1. Create a new patch version with fixes
2. Follow the normal release process
3. Consider yanking the problematic version from crates.io if necessary

## Security Considerations

- The `CRATES_IO_TOKEN` secret should be regularly rotated
- Only repository administrators should have access to manage releases
- All release artifacts are public and should not contain sensitive information