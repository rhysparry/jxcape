# jxcape Development Instructions

`jxcape` is a command-line tool for creating JSON values from command line arguments. It's written in Rust and provides three main commands: `string`, `array`, and `object` for generating respective JSON types.

**ALWAYS reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.**

## Working Effectively

### Bootstrap and Build
- Install Rust toolchain if not available: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Build the project: `cargo build` -- takes ~15 seconds for initial build with dependencies
- Build release version: `cargo build --release` -- takes ~10 seconds  
- Install from source: `cargo install --path .` -- takes ~12 seconds

### Testing and Validation
- Run all tests: `cargo test` -- takes <1 second, runs 24 unit tests
- Run linting: `cargo clippy` -- takes ~5 seconds  
- Check formatting: `cargo fmt --check` -- takes <1 second
- **NEVER CANCEL these commands** - they complete quickly but always wait for completion

### Running the Application
- Run directly: `cargo run -- <args>`
- After install: `jxcape <args>`
- Get help: `jxcape --help` or `jxcape <command> --help`

## Validation Scenarios

**ALWAYS test these core scenarios after making changes:**

### Basic Functionality Tests
```bash
# Test string command
jxcape string "Hello, world!"
# Expected: "Hello, world!"

# Test array command  
jxcape array 1 2 3
# Expected: ["1","2","3"]

# Test object command
jxcape object foo=1 bar=2
# Expected: {"bar":"2","foo":"1"}
```

### Advanced Feature Tests
```bash
# Test pretty printing
jxcape --pretty array 1 2 3
# Expected: formatted JSON with newlines and indentation

# Test auto JSON parsing
jxcape array --auto 1 2 3 foo
# Expected: [1,2,3,"foo"]

jxcape object --auto foo=1 bar=true baz=null
# Expected: {"bar":true,"baz":null,"foo":1}

# Test stdin input
echo "test" | jxcape string --from-stdin
# Expected: "test"

seq 1 3 | jxcape array --from-stdin  
# Expected: ["1","2","3"]

# Test custom separators
jxcape object --separator=: foo:1 bar:2
# Expected: {"bar":"2","foo":"1"}

# Test empty collections
jxcape array --empty
# Expected: []

jxcape object --empty
# Expected: {}
```

### Integration Test
```bash
# Test environment variables to JSON
env | head -5 | jxcape object --from-stdin
# Expected: Valid JSON object with environment variables
```

## Build Commands and Timing

- `cargo build` -- Debug build, ~15 seconds first time, ~1 second incremental
- `cargo build --release` -- Release build, ~10 seconds
- `cargo test` -- Run tests, <1 second (24 tests)
- `cargo clippy` -- Lint code, ~5 seconds  
- `cargo fmt` -- Format code, <1 second
- `cargo install --path .` -- Install locally, ~12 seconds

**All commands complete quickly - no special timeout handling needed.**

## Code Quality Checks

**ALWAYS run these before committing changes:**
```bash
cargo fmt           # Format code
cargo clippy        # Lint for issues  
cargo test          # Run all tests
cargo build         # Verify builds successfully
```

**CI will fail if any of these fail**, so run them locally first.

## Project Structure

### Key Files and Directories
- `src/main.rs` -- Entry point, calls into lib
- `src/lib.rs` -- Core library with shared utilities
- `src/cli.rs` -- Command-line argument parsing and main CLI logic
- `src/strings.rs` -- String command implementation  
- `src/arrays.rs` -- Array command implementation
- `src/objects.rs` -- Object command implementation
- `src/json.rs` -- JSON formatting (default vs pretty)
- `src/testing.rs` -- Test utilities for stdin simulation
- `Cargo.toml` -- Dependencies: clap (CLI), serde_json (JSON)
- `.github/workflows/rust.yml` -- CI pipeline (build + test)

### Architecture Overview
- Uses `clap` for CLI parsing with derive macros
- Uses `serde_json` for JSON manipulation
- Trait-based design with `JsonValueCommand` and `ValueReader`
- Comprehensive unit test coverage (24 tests)
- Support for both command-line args and stdin input

## Common Development Tasks

### Adding New Features
1. Add command-line arguments to appropriate `Args` struct in relevant module
2. Implement logic in the `JsonValueCommand::get_json_value()` method
3. Add comprehensive unit tests following existing patterns
4. Test manually with validation scenarios above
5. Run quality checks: `cargo fmt && cargo clippy && cargo test`

### Debugging Issues
- Use `cargo run -- <args>` for quick testing
- Add `dbg!()` macros for debugging (remove before commit)
- Run specific tests: `cargo test <test_name>`
- Use `--verbose` flag with cargo commands for detailed output

### Testing New Changes
1. Run `cargo test` to ensure all existing tests pass
2. Run through all validation scenarios above
3. Test edge cases specific to your changes
4. Verify both debug and release builds work
5. Test the installed binary if adding new features

## Dependencies and Tools

### Required (installed with Rust)
- `cargo` -- Build tool and package manager
- `rustc` -- Rust compiler  
- `rustfmt` -- Code formatter
- `clippy` -- Linter

### Optional (not required for development)
- `just` -- Task runner (used for changelog generation)
- `git-cliff` -- Changelog generator

### No External System Dependencies
The project has no external system dependencies beyond the Rust toolchain.

## Common Issues and Solutions

### Build Issues
- If dependencies fail to download: `cargo clean && cargo build`
- If clippy fails: Fix the reported issues, don't suppress warnings
- If tests fail: Fix the failing tests, don't modify passing tests

### Runtime Issues  
- Invalid JSON input with `--auto`: Will be treated as string (expected behavior)
- Missing value with object command: Key will have `null` value
- Empty stdin: Commands will handle gracefully

## Quick Reference Commands

```bash
# Development workflow
cargo build && cargo test && cargo clippy

# Test basic functionality
jxcape string "test" && jxcape array 1 2 && jxcape object a=1

# Test advanced features  
jxcape --pretty object --auto num=42 bool=true str=hello

# Install and test binary
cargo install --path . && jxcape --version
```

## Version Information
- Current version: 0.2.0 (check `Cargo.toml` for latest)
- Rust edition: 2021
- License: MIT OR Apache-2.0