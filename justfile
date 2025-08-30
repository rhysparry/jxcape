set windows-shell := ["nu", "-c"]

# Generate changelog
changelog:
    git cliff --output CHANGELOG.md

# Check if ready for release (run tests, clippy, etc.)
check:
    cargo fmt --check
    cargo clippy -- -D warnings
    cargo test

# Prepare a new release (update changelog and check everything)
prepare-release version:
    @echo "Preparing release {{version}}..."
    @echo "Updating Cargo.toml version..."
    sed -i 's/^version = ".*"/version = "{{version}}"/' Cargo.toml
    @echo "Updating changelog..."
    git cliff --tag v{{version}} --output CHANGELOG.md
    @echo "Running quality checks..."
    just check
    @echo "Release {{version}} is ready!"
    @echo "Next steps:"
    @echo "  1. Review the changes in Cargo.toml and CHANGELOG.md"
    @echo "  2. Commit: git commit -am 'chore: bump version to {{version}}'"
    @echo "  3. Tag: git tag v{{version}}"
    @echo "  4. Push: git push origin main && git push origin v{{version}}"
