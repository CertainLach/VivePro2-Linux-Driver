{
  description = "VIVE Pro 2 support for linux";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-26.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
          ];
        };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        sewer =
          with pkgs.pkgsStatic;
          rustPlatform.buildRustPackage {
            pname = "sewer";
            version = "0.1.0";
            src = fetchFromGitHub {
              owner = "CertainLach";
              repo = "sewer";
              rev = "fb0d054e53e2afd4c64232318495e5351b446330";
              hash = "sha256-2S2JXKLbRQsrQmt25djj/x284NXqPSGJjybDe9Uw7ZM=";
            };
            cargoHash = "sha256-D76DjJW77RY1vsi5QAjm5LGmS8rzYZefMDWm90qaa28=";
            doCheck = false;
          };
      in
      rec {
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
        packages =
          let
            version = "0.1.0";
            src = builtins.path {
              path = ./.;
              filter = path: type: baseNameOf path != "flake.nix";
            };
            cargoLock = {
              lockFile = ./Cargo.lock;
              outputHashes = {
                "pelite-0.10.0" = "sha256-Pzm8OKWuU8/9xEMvVsVARCrJ2qljHwECB410HoUb3MA=";
                "champagne-0.1.1" = "sha256-8TTt3eYE7ikCQNoB3fRQh3ZGICsd4tolkFPJa9Oweh8=";
              };
            };
          in
          {
            driver-vivevr =
              with pkgs;
              rustPlatform.buildRustPackage {
                inherit version src cargoLock;
                pname = "vivepro2-driver-vivevr";
                nativeBuildInputs = [ pkg-config ];
                buildInputs = [
                  udev
                  dbus.dev
                ];
              };

            driver-vivevr-release =
              with pkgs;
              stdenv.mkDerivation {
                inherit version src;
                pname = "vivepro2-driver-vivevr-release";
                installPhase = ''
                  cp -r $src/dist/ $out/
                  chmod u+w -R $out
                  cp ${packages.driver-vivevr}/lib/libdriver_vivevr.so $out/bin/linux64/driver_viveVR.so
                  mkdir $out/tools
                  cp ${sewer}/bin/sewer $out/tools/
                '';
                patchPhase = "true";
                fixupPhase = "true";
              };
            driver-vivevr-release-tar-zstd =
              with pkgs;
              stdenv.mkDerivation {
                inherit (packages.driver-vivevr-release) version pname;
                unpackPhase = "true";
                patchPhase = "true";
                fixupPhase = "true";
                installPhase = ''
                  mkdir $out/
                  cd ${packages.driver-vivevr-release}
                  tar -cv * | ${pkgs.zstd}/bin/zstd -9 > $out/driver.tar.zst
                '';
              };
          };
        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              rust
              cargo-edit
              pkg-config
              lld
              dbus.dev
              udev
              sewer
            ];
            LD_LIBRARY_PATH = "${pkgs.dbus.lib}/lib";
          };
        };
        devShell = devShells.default;
      }
    );
}
