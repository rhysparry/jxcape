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

### From Source

```bash
$ git clone https://github.com/rhysparry/jxcape.git
$ cd jxcape
$ cargo install --path .
```