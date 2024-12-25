{
  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay/stable";
    nixpkgs.follows = "rust-overlay/nixpkgs";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    ...
  }: let
    forAllSystems = f:
      nixpkgs.lib.genAttrs [
        "aarch64-linux"
        "x86_64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ] (
        system:
          f {
            pkgs = import nixpkgs {
              inherit system;
              overlays = [(import rust-overlay)];
            };
          }
      );
  in {
    devShells = forAllSystems ({pkgs}: {
      default = with pkgs;
        mkShell {
          packages = [
            (rust-bin.stable.latest.default.override {
              extensions = [
                "rust-src"
                "rust-analyzer"
              ];
            })
          ];

          shellHook = ''
            # Avoid polluting home dir with local project stuff.
            if command -v git  &> /dev/null; then
              CARGO_HOME="$(git rev-parse --show-toplevel)/.cargo"
              export CARGO_HOME
              export PATH=$PATH:$CARGO_HOME/bin
            fi
          '';
        };
    });
  };
}
