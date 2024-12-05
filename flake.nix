{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            libgcc
            cargo
            rustc
            python312Packages.autopep8
          ];

          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          RUST_BACKTRACE = "full";

          shellHook = ''
            clear; zsh; exit
          '';
        };
      }
    );
}
