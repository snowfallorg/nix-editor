<div align="center">

Nix Editor
===
[![crates.io][crates badge]][crate]
[![Coverage Status][coveralls badge]][coveralls]
[![Built with Nix][builtwithnix badge]][builtwithnix]
[![License: MIT][MIT badge]][MIT]

A command line utility for modifying NixOS configuration values.

</div>

## Installation
### nix-env
```
git clone https://github.com/vlinkz/nix-editor
nix-env -f nix-editor -i nix-editor
```
### nix profile
```
nix profile install github:vlinkz/nix-editor
```

## Run with Nix Flakes
```
nix run github:vlinkz/nix-editor -- --help
```

```
Usage: nix-editor [OPTIONS] <FILE> <ATTRIBUTE>

Arguments:
  <FILE>       Configuration file to read
  <ATTRIBUTE>  Nix configuration option arribute

Options:
  -v, --val <VAL>        Value to write
  -a, --arr <ARR>        Element to add
  -d, --deref            Dereference the value of the attribute
  -i, --inplace          Edit the file in-place
  -o, --output <OUTPUT>  Output file for modified config or read value
  -r, --raw              Prints console output without newlines or trimmed output
  -f, --format           Formats output using nixpkgs-fmt. Helps when writing new values
  -h, --help             Print help
  -V, --version          Print version
```
[coveralls badge]: https://img.shields.io/coveralls/github/vlinkz/nix-editor?style=for-the-badge
[coveralls]: https://coveralls.io/github/vlinkz/nix-editor
[crates badge]: https://img.shields.io/crates/v/nix-editor.svg?style=for-the-badge
[crate]: https://crates.io/crates/nix-editor
[builtwithnix badge]: https://img.shields.io/badge/Built%20With-Nix-41439A?style=for-the-badge&logo=nixos&logoColor=white
[builtwithnix]: https://builtwithnix.org/
[MIT badge]: https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge
[MIT]: https://opensource.org/licenses/MIT