# File Finder (ff)

A simple CLI tool for finding files much like you would on github.

## Installatino

Use cargo to install. You can install cargo via rustup (https://rustup.rs/)

```
cargo install -f
```

## Usage

There are only a few options, it searches from your cwd:

```
File finder

USAGE:
    ff [FLAGS] <NEEDLE>

FLAGS:
    -c, --color      whether to display colored matches
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <NEEDLE>    The value to search for
```
