{
  description = "My Rust Project with Nix Flakes";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      flake-utils,
      nixpkgs,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust-toolchain = (
          pkgs.rust-bin.selectLatestNightlyWith (
            toolchain:
            toolchain.default.override {
              extensions = [
                "rust-analyzer"
                "rust-src"
              ];
            }
          )
        );
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            nativeBuildInputs = [
              clang
              git
              jujutsu
              mold
              pkg-config
              rust-toolchain
              gdb
              lldb
            ];
            buildInputs = [
              openssl
            ]
            ++ lib.optionals (lib.strings.hasInfix "linux" system) [
              alsa-lib
              alsa-utils
              vulkan-loader
              vulkan-tools
              libudev-zero
              libx11
              libxcursor
              libxi
              libxrandr
              libxkbcommon
              wayland
            ];
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            LD_LIBRARY_PATH = lib.makeLibraryPath [
              vulkan-loader
              libx11
              libxi
              libxcursor
              libxkbcommon
            ];

            shellHook = "";
          };
      }
    );
}
