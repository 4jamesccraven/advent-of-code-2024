{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { flake-utils, nixpkgs, self }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.clean-dirs = pkgs.writeShellScriptBin "clean-dirs" ''
          #!${pkgs.bash}
          find ./solutions/ -name 'Cargo.toml' -execdir cargo clean \;
        '';

        packages.benchmark = pkgs.writeShellScriptBin "benchmark" ''
          #!${pkgs.bash}
          echo "day,time"
          for dir in $(find ./solutions/ -name 'Cargo.toml' -exec dirname {} \; | sort); do
            day=$(echo "$dir" | sed 's|\(.*\)/day-\([0-9]\+\)|\2|')
            echo -n "$day"

            real_time=$(cd "$dir" && { time cargo run --release > /dev/null 2>&1; } 2>&1 | grep real| awk '{print $2}')

            seconds=$(echo "$real_time" | awk -F'm' '{print $2}' | sed 's/s//')

            echo ",$seconds"
          done
        '';

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            self.packages.x86_64-linux.clean-dirs
            self.packages.x86_64-linux.benchmark

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
