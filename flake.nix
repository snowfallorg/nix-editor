{
  description = "Nix configuration command line editor";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        name = "nix-editor";
      in
      rec
      {
        packages.${name} = pkgs.callPackage ./default.nix { };

        # `nix build`
        defaultPackage = packages.${name}; # legacy
        packages.default = packages.${name};

        # `nix run`
        apps.${name} = utils.lib.mkApp {
          inherit name;
          drv = packages.${name};
        };
        defaultApp = apps.${name};

        checks = self.packages.${system};
        hydraJobs = self.packages.${system};

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-analyzer
            rustc
            rustfmt
            cargo
            cargo-tarpaulin
            clippy
          ];
        };
      });
}
