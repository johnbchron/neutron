{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        toolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        });

        nativeBuildInputs = with pkgs; [
          toolchain pkg-config clang
          alsa-lib udev

          libxkbcommon wayland
          xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
 
          vulkan-headers vulkan-loader
          vulkan-tools vulkan-tools-lunarg
          vulkan-extension-layer
          vulkan-validation-layers
        ];
        buildInputs = [];

      in {
        devShell = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeBuildInputs;
        };
      }
  );
}
