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
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlay ];
        };
        rust = ((pkgs.buildPackages.rustChannelOf { date = "2022-01-13"; channel = "nightly"; }).default.override {
          targets = [ "x86_64-pc-windows-gnu" ];
          extensions = [ "rust-src" ];
        });
        pkgs-mingw = import nixpkgs {
          inherit system;
          crossSystem = { config = "x86_64-w64-mingw32"; };
        };
      in
      rec {
        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs;[
              rust
              cargo-edit
              hidapi
              udev
              libusb
              pkg-config
              wine64

              # druid build
              cmake
              pkg-config
              fontconfig

              # slint runtime
              xorg.libxcb
              xorg.libX11
              xorg.libXcursor
              xorg.libXrandr
              xorg.libXi

              cairo
            ];
          };
          mingw = pkgs-mingw.mkShell {
            nativeBuildInputs = [ rust ];
            depsBuildBuild = with pkgs; [ wine64 ];
            buildInputs = with pkgs-mingw; [ windows.pthreads ];
          };
        };
      }
    );
}
