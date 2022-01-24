{
  description = "VIVE Pro 2 support for linux";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs
          {
            inherit system;
            overlays = [ rust-overlay.overlay ];
            crossSystem = {
              config = "x86_64-w64-mingw32";
            };
          };
        rust = ((pkgs.buildPackages.rustChannelOf { date = "2022-01-13"; channel = "nightly"; }).default.override {
          extensions = [ "rust-src" ];
        });
      in
      rec {
        devShell = pkgs.callPackage
          ({ mkShell, stdenv, windows, wine64 }: mkShell {
            nativeBuildInputs = [
              rust
            ];

            depsBuildBuild = [ wine64 ];
            buildInputs = [ windows.pthreads ];

            CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";
            CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUNNER = "wine64";
          })
          { };
      }
    );
}
