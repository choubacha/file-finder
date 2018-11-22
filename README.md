# File Finder (ff)

A simple CLI tool for finding files much like you would on github.

## Installation

Use cargo to install. You can install cargo via rustup (https://rustup.rs/)

```
cargo install -f
```

## Usage

There are only a few options, it searches from your cwd:

```
$ ff --help
File finder

USAGE:
    ff [FLAGS] [OPTIONS] <NEEDLE> [PATH]

FLAGS:
        --help              Prints help information
    -h, --include-hidden    include hidden matches
    -c, --no-color          disable colored matches
    -V, --version           Prints version information

OPTIONS:
    -n, --number <number>    the number of matches to return [default: 10]

ARGS:
    <NEEDLE>    The value to search for
    <PATH>      The path to start searching at. Defaults to current working directory
```
