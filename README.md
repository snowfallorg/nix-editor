# Nix-Editor
A command line utility for modifying NixOS configuration values.

## NixOS Installation

```
git clone https://github.com/vlinkz/nix-editor
nix-env -f nix-editor -i nix-editor
```

## Usage with Nix Flakes
```
nix run github:vlinkz/nix-editor -- --help
```

```
USAGE:
    nix-editor [OPTIONS] <FILE> <ATTRIBUTE>

ARGS:
    <FILE>         Configuration file to read
    <ATTRIBUTE>    Nix configuration option arribute

OPTIONS:
    -a, --arr <ARR>          Element to add
    -d, --deref              Dereference the value of the attribute
    -e, --eval               Show the evaluated value of the attribute
    -h, --help               Print help information
    -o, --output <OUTPUT>    Output file for modified config or read value
    -v, --val <VAL>          Value to write
    -V, --version            Print version information
```