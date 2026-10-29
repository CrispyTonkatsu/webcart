{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { nixpkgs, ... }: {
    devShells = builtins.mapAttrs (
      system: pkgs:
      let
        waylandPackages = with pkgs; [
          wayland
          wayland-protocols
          vulkan-loader

          libGL

          libxkbcommon
        ];
      in
      {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            cargo
            rustfmt
            rustPackages.clippy
            rust-analyzer

            texlab
            texliveFull

            tombi
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath waylandPackages;
        };

      }
    ) nixpkgs.legacyPackages;
  };
}
