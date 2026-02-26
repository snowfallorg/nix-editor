{
  description = "Nix configuration command line editor";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      nixpkgs,
      ...
    }:
    let
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (
          system: f nixpkgs.legacyPackages.${system} system
        );
    in
    {
      packages = forAllSystems (pkgs: _: { default = pkgs.callPackage ./default.nix { }; });
      devShells = forAllSystems (
        pkgs: _: {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              rust-analyzer
              rustc
              rustfmt
              cargo
              cargo-tarpaulin
              clippy
            ];
          };
        }
      );
    };
}
