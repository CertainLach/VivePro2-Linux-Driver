{
  description = "VIVE Pro 2 support for linux";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            # (import ./nix/oldGlibc.nix)
            (
              final: prev: let
                rust =
                  (final.buildPackages.rustChannelOf {
                    date = "2023-11-05";
                    channel = "nightly";
                  })
                  .default
                  .override {
                    extensions = ["rust-src" "rust-analyzer"];
                    targets = ["x86_64-unknown-linux-musl"];
                  };
              in {
                rustDev = rust;
                rustPlatform = prev.makeRustPlatform {
                  rustc = rust;
                  cargo = rust;
                };
              }
            )
          ];
        };
        pkgs-mingw = import nixpkgs {
          inherit system;
          config.replaceStdenv = import ./nix/oldGlibcStdenv.nix;
          overlays = [
            rust-overlay.overlays.default
            (final: prev: {
              # inherit rustPlatform;
              # https://github.com/NixOS/nixpkgs/issues/149593
              openssh = prev.openssh.overrideAttrs (prev: {
                doCheck = false;
              });
              rustPlatform = let
                rust =
                  (final.buildPackages.rustChannelOf {
                    date = "2022-09-03";
                    channel = "nightly";
                  })
                  .default
                  .override {
                    targets = ["x86_64-pc-windows-gnu"];
                  };
              in
                prev.makeRustPlatform {
                  rustc = rust;
                  cargo = rust;
                };
            })
          ];
          crossSystem = {
            config = "x86_64-w64-mingw32";
            arch = "x86_64";
            libc = "msvcrt";
            platform = {};
            openssl.system = "mingw64";
          };
          config.allowUnsupportedSystem = true;
        };
      in rec {
        kernelPatches = [
          {
            name = "drm-edid-non-desktop";
            patch = ./kernel-patches/0001-drm-edid-non-desktop.patch;
          }
          {
            name = "drm-edid-type-7-timings";
            patch = ./kernel-patches/0002-drm-edid-type-7-timings.patch;
          }
          {
            name = "drm-edid-dsc-bpp-parse";
            patch = ./kernel-patches/0003-drm-edid-dsc-bpp-parse.patch;
          }
          {
            name = "drm-amd-dsc-bpp-apply";
            patch = ./kernel-patches/0004-drm-amd-dsc-bpp-apply.patch;
          }
        ];
        packages = let
          version = "0.1.0";
          src = builtins.path {
            path = ./.;
            filter = path: type: baseNameOf path != "flake.nix";
          };
          cargoLock.lockFile = ./Cargo.lock;
        in {
          driver-proxy = with pkgs;
            rustPlatform.buildRustPackage {
              inherit version src cargoLock;
              pname = "vivepro2-driver-proxy";
              nativeBuildInputs = [pkg-config];
              buildInputs = [udev dbus.dev];
            };
          sewer = with pkgs.pkgsStatic;
            rustPlatform.buildRustPackage {
              name = "sewer";
              src = fetchFromGitHub {
                owner = "CertainLach";
                repo = "sewer";
                rev = "fb0d054e53e2afd4c64232318495e5351b446330";
                hash = "sha256-2S2JXKLbRQsrQmt25djj/x284NXqPSGJjybDe9Uw7ZM=";
              };
              cargoHash = "sha256-LZTAWRZbJktp5cDTkPWcBSPJwnG5fYDDRGkrVIVdWyU=";
              target = "x86_64-unknown-linux-musl";
              doCheck = false;
            };
          lens-server = with pkgs-mingw;
            rustPlatform.buildRustPackage {
              inherit version src cargoLock;
              pname = "vivepro2-lens-server";
              buildAndTestSubdir = "bin/lens-server";
            };

          driver-proxy-release = with pkgs;
            stdenv.mkDerivation {
              inherit version src;
              pname = "vivepro2-driver-proxy-release";
              installPhase = ''
                cp -r $src/dist-proxy/ $out/
                chmod u+w -R $out
                mkdir $out/bin/
                cp ${packages.sewer}/bin/sewer $out/bin/
                cp ${packages.lens-server}/bin/lens-server.exe $out/lens-server/
                cp ${packages.driver-proxy}/lib/libdriver_proxy.so $out/driver_lighthouse.so
              '';
              patchPhase = "true";
              fixupPhase = "true";
            };
          driver-proxy-release-tar-zstd = with pkgs;
            stdenv.mkDerivation {
              inherit (packages.driver-proxy-release) version pname;
              unpackPhase = "true";
              patchPhase = "true";
              fixupPhase = "true";
              installPhase = ''
                mkdir $out/
                cd ${packages.driver-proxy-release}
                tar -cv * | ${pkgs.zstd}/bin/zstd -9 > $out/driver.tar.zst
              '';
            };
        };
        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              rustDev
              cargo-edit
              pkg-config
              lld
              dbus.dev
              udev
            ];
            LD_LIBRARY_PATH = "${pkgs.dbus.lib}/lib";
          };
        };
        devShell = devShells.default;
      }
    );
}
