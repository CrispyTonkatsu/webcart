{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { nixpkgs, ... }: {
    devShells = builtins.mapAttrs (system: pkgs: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          rustc
          cargo
          rustfmt
          rustPackages.clippy
          rust-analyzer

          tombi
        ];
      };
    }) nixpkgs.legacyPackages;
  };
}
