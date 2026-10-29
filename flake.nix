{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay.url = "github:oxalica/rust-overlay";
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
          overlays = [ (import rust-overlay) ];
        };

        waylandPackages = with pkgs; [
          wayland
          wayland-protocols
          vulkan-loader

          libGL

          libxkbcommon
        ];

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain

            texlab
            texliveFull

            tombi

            wasm-pack
          ];

          LD_LIBRARY_PATH =
            if pkgs.stdenv.hostPlatform.isLinux then pkgs.lib.makeLibraryPath waylandPackages else [ ];
        };
      }
    );
}
