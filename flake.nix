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

    # Tests run by 'nix flake check' and by Hydra
    checks = forAllSystems
        (system:
          with nixpkgsFor.${system};

          {
            inherit (self.packages.${system}) hello;

            # Additional tests, if applicable.
            test = stdenv.mkDerivation {
              name = "hello-test-${version}";

              buildInputs = [ hello ];

              unpackPhase = "true";

              buildPhase = ''
                echo 'running some integration tests'
                [[ $(hello) = 'Hello Nixers!' ]]
              '';

              installPhase = "mkdir -p $out";
            };
          }
  });
}
