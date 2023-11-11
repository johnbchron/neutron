{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, flake-utils, naersk, nixpkgs, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };
        
        toolchain = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
          extensions = [ "rust-analyzer" "rust-src" ];
        };
        
        rust_deps = [ toolchain pkgs.lldb pkgs.bacon ];
        bevy_build_deps = with pkgs; [
          pkg-config
          mold clang
          makeWrapper
        ];
        bevy_runtime_deps = with pkgs; [
          # udev alsa-lib vulkan-loader pipewire.lib # bevy deps
          # xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
          # libxkbcommon wayland # To use the wayland feature
          rustPlatform.bindgenHook darwin.apple_sdk.frameworks.Cocoa
        ];
      in {
        defaultPackage = let 
          naersk' = pkgs.callPackage naersk {
            cargo = toolchain;
            rustc = toolchain;
          };
        in naersk'.buildPackage rec {
          pname = "mage_corp";
          src = ./.;

          nativeBuildInputs = bevy_build_deps;
          buildInputs = bevy_runtime_deps;
          
          overrideMain = attrs: {
            fixupPhase = ''
              wrapProgram $out/bin/${pname} \
                --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath bevy_runtime_deps} \
                # --prefix XCURSOR_THEME : "Adwaita" \
                # --prefix ALSA_PLUGIN_DIR : ${"pkgs.pipewire.lib"}/lib/alsa-lib
              mkdir -p $out/bin/assets
              cp -a crates/mage_corp/assets $out/bin
            '';
          };
        };

        # For `nix develop`:
        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs = bevy_build_deps ++ bevy_runtime_deps ++ rust_deps;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeBuildInputs;
          LIBCLANG_PATH = "${pkgs.libclang}/lib";
        };
      }
    );
}
