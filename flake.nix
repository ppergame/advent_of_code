{
  description = "Advent Of Codes";

  inputs = {
    nix-deps.url = "git+ssh://git@github.com/xzrq-net/nix-deps";

    nixpkgs = {
      # url = "github:nixos/nixpkgs/nixos-unstable";
      follows = "nix-deps/nixpkgs";
    };

    rust-overlay = {
      # url = "github:oxalica/rust-overlay";
      follows = "nix-deps/rust-overlay";
    };

    crane = {
      # url = "github:ipetkov/crane";
      follows = "nix-deps/crane";
    };

    flake-utils = {
      # url = "github:numtide/flake-utils";
      follows = "nix-deps/flake-utils";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    crane,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      };
      myBuildInputs = with pkgs; [pkg-config openssl rustPlatform.bindgenHook cmake z3 mpfr m4];
      mkPackage = name: let
        craneLib = (crane.mkLib pkgs).overrideToolchain pkgs.rust-bin.stable.latest.default;
      in (craneLib.buildPackage {
        src = let
          htmlFilter = path: builtins.match ".*html$" path != null;
          files = path: type:
            (htmlFilter path) || (craneLib.filterCargoSources path type);
        in
          pkgs.lib.cleanSourceWith {
            src = craneLib.path ./${name}/.;
            filter = files;
          };
        buildInputs = myBuildInputs;
      });
    in {
      packages =
        builtins.listToAttrs (map (name: {
            inherit name;
            value = mkPackage name;
          }) [
            "aoc2015"
            "aoc2016"
            "aoc2017"
            "aoc2018"
            "aoc2019"
            "aoc2020"
            "aoc2021"
            "aoc2022"
            "aoc2023"
            "aoc2024"
            "aoc2025"
          ])
        // {
          default = mkPackage "xaoc";
        };
      devShells.default = pkgs.mkShell {
        buildInputs = [self.packages.${system}.default] ++ myBuildInputs;
      };
    });
}
