Nix-Editor
===
[![crates.io][crates badge]][crate]
[![Coverage Status][coveralls badge]][coveralls]
[![Built with Nix][builtwithnix badge]][builtwithnix]
[![License: MIT][MIT badge]][MIT]

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
    -h, --help               Print help information
    -o, --output <OUTPUT>    Output file for modified config or read value
    -v, --val <VAL>          Value to write
    -V, --version            Print version information
```
[coveralls badge]: https://img.shields.io/coveralls/github/vlinkz/nix-editor?style=flat-square
[coveralls]: https://coveralls.io/github/vlinkz/nix-editor
[crates badge]: https://img.shields.io/crates/v/nix-editor.svg?style=flat-square
[crate]: https://crates.io/crates/nix-editor
[builtwithnix badge]: https://img.shields.io/badge/Built%20With-Nix-41439A?style=flat-square&logo=nixos&logoColor=white
[builtwithnix]: https://builtwithnix.org/
[MIT badge]: https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square
[MIT]: https://opensource.org/licenses/MIT