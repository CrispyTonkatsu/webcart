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

        graphicsPackages = with pkgs; [
          mesa

          wayland
          wayland-protocols

          vulkan-loader
          vulkan-tools

          libGL
          libglvnd

          libxkbcommon
        ];

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
      in
      {
        devShells.default = pkgs.mkShell {
          packages =
            with pkgs;
            [
              rustToolchain
              wasm-pack

              # LSPs
              tombi
              yaml-language-server
              basedpyright
              ruff

              # Math tools
              texlab
              texliveFull

              (python3.withPackages (
                ps: with ps; [
                  sympy
                  numpy
                ]
              ))

            ]
            ++ (if pkgs.stdenv.isDarwin then [ ] else graphicsPackages);

          # Stuff needed for dGPU in cachyOS
          __EGL_VENDOR_LIBRARY_FILENAMES = "${pkgs.libglvnd}/share/glvnd/egl_vendor.d/10_nvidia.json";
          WGPU_BACKEND = "vulkan";
          VK_ICD_FILENAMES = "/usr/share/vulkan/icd.d/nvidia_icd.json";

          LD_LIBRARY_PATH =
            if pkgs.stdenv.hostPlatform.isLinux then
              pkgs.lib.makeLibraryPath graphicsPackages + ":/usr/lib:/usr/lib32"
            else
              [ ];
        };
      }
    );
}
