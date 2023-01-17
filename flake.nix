{
  description = "Nix configuration command line editor";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, naersk, ... }: 
        systems = [ "x86_64-linux" "aarch64-linux" ];
        
  flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        naersk-lib = naersk.lib."${system}";
      in
  {
    packages.nixeditor = naersk-lib.buildPackage {
      pname = "nix-editor";
      root = ./.;
      };

    defaultPackage = self.packages.${system}.nixeditor;

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
