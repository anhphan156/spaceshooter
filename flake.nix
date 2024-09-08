{
  description = "hello";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [
          (import rust-overlay)
          (self: super: {
            rust-toolchain = self.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          })
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        clang_lib = with pkgs; [
          libclang
        ];

        x11_lib = with pkgs; [
          libGL
          wayland
          wayland-protocols
          libxkbcommon
          xorg.libX11
          xorg.libXrandr
          xorg.libXinerama
          xorg.libXcursor
          xorg.libXi
          xorg.libXext
        ];
      in
        with pkgs; {
          devShells.default = mkShell {
            RUST_BACKTRACE = 1;
            LIBCLANG_PATH = lib.makeLibraryPath clang_lib;
            LD_LIBRARY_PATH = lib.makeLibraryPath x11_lib;
            packages = [
              pkg-config
              cmake
              rust-toolchain
              glfw
              glfw-wayland
              glxinfo
              clang
            ];
            buildInputs = clang_lib ++ x11_lib;
          };
        }
    );
}
