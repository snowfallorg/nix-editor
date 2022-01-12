# Nix-Editor
A command line utility for modifying NixOS configuration values.

## Usage with Nix Flakes
```
nix run github:vlinkz/nix-editor -- --help
```

```
Usage:
  nix-editor [OPTIONS] COMMAND [ARGUMENTS ...]

Reads an option from a config file

Positional arguments:
  command               Command "read" or "write" required
  arguments             Arguments for command

Optional arguments:
  -h,--help             Show this help message and exit
  -v,--verbose          Be verbose
  -f,--file FILE        Config file
  -q,--query QUERY      Option query
```
