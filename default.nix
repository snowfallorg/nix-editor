{ pkgs ? import <nixpkgs> { }
, lib ? import <nixpkgs/lib>
}:
pkgs.rustPlatform.buildRustPackage {
  pname = "nix-editor";
  version = "0.3.0";

  src = [ ./. ];

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
