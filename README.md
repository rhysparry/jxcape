# About `jxcape`

`jxcape` is a tool for creating JSON values from the command line.

## Strings

To convert a value into a JSON string, use the `string` subcommand:

```bash
$ jxcape string "Hello, world!"
"Hello, world!"
```

It can also take input from stdin:

```bash
$ echo "Hello, world!" | jxcape string --from-stdin 
"Hello, world!"
```

## Arrays

To convert a value into a JSON array, use the `array` subcommand:

```bash
$ jxcape array 1 2 3
["1","2","3"]
```

By default, `jxscape` will treat all input as strings. To treat input as native JSON, use the `--auto` flag:

```bash
$ jxcape array --auto 1 2 3 foo
[1,2,3,"foo"]
```

Any value that cannot be parsed as JSON will be treated as a string.

It can also take input from stdin:
 
```bash
$ seq 3 | jxcape array --from-stdin
["1","2","3"]
```

If for some reason you need an empty array, you can use the `--empty` flag:

```bash
$ jxcape array --empty
[]
```

This is included mostly for completeness.

## Objects

To convert a value into a JSON object, use the `object` subcommand:

```bash
$ jxcape object foo=1 bar=2
{"foo":"1","bar":"2"}
```

By default, `jxscape` will treat all input as strings. To treat input as native JSON, use the `--auto` flag:

```bash
$ jxcape object --auto foo=1 bar=2 baz=3
{"foo":1,"bar":2,"baz":3}
```

Any value that cannot be parsed as JSON will be treated as a string.

It can also take input from stdin:
 
```bash
$ env | jxcape object --from-stdin
{"TERM":"xterm-256color","SHELL":"/bin/bash",...} # truncated for brevity
```

You can specify a custom separator using the `--separator` flag:

```bash
$ jxcape object --separator=: foo:1 bar:2 baz:3
{"foo":"1","bar":"2","baz":"3"}
```

Separators can be multiple characters:
```bash
$ jxcape object --separator=:: foo::1 bar::2 baz::3
{"foo":"1","bar":"2","baz":"3"}
```

If a key is specified multiple times, the last value to appear will be used:

```bash
$ jxcape object foo=1 foo=2 foo=3
{"foo":"3"}
```

If for some reason you need an empty object, you can use the `--empty` flag:

```bash
$ jxcape object --empty
{}
```

This is included mostly for completeness.

## Pretty Printing

To pretty print a JSON value, use the `--pretty` flag before the subcommand:

```bash
$ jxcape --pretty array 1 2 3
[
  "1",
  "2",
  "3"
]
```

## Installation

### Pre-built Binaries

Download the latest release for your platform from the [releases page](https://github.com/rhysparry/jxcape/releases).

Available platforms:
- Linux (x86_64): `jxcape-linux-x86_64.tar.gz`
- macOS (x86_64): `jxcape-macos-x86_64.tar.gz`
- macOS (ARM64): `jxcape-macos-aarch64.tar.gz`
- Windows (x86_64): `jxcape-windows-x86_64.zip`

Extract the archive and add the binary to your PATH.

### From Crates.io

```bash
$ cargo install jxcape
```

### From Source

```bash
$ git clone https://github.com/rhysparry/jxcape.git
$ cd jxcape
$ cargo install --path .
```

## Releasing

This project uses an automated release process. To create a new release:

1. **Update the version** in `Cargo.toml`
2. **Update the changelog** by running `just changelog` (requires [git-cliff](https://git-cliff.org/))
3. **Commit the changes**: `git commit -am "chore: bump version to X.Y.Z"`
4. **Create and push a tag**: `git tag vX.Y.Z && git push origin vX.Y.Z`

The release workflow will automatically:
- Build binaries for multiple platforms
- Create a GitHub release with changelog content
- Upload compressed binary assets
- Publish the crate to [crates.io](https://crates.io/)

### Prerequisites for Releases

- The `CRATES_IO_TOKEN` secret must be configured in the repository settings
- The version in `Cargo.toml` should match the tag (without the 'v' prefix)
- All quality checks (tests, clippy) must pass