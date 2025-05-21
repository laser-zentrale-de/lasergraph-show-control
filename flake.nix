{
  description = "lasergraph-show-control";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = builtins.fromTOML (builtins.readFile ./rust-toolchain.toml);
        rust = pkgs.rust-bin.stable.${rustToolchain.toolchain.channel}.default;
        # Preserved for future use in nix build
        # cargo = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      in {
        devShells.default = with pkgs;
          mkShell rec {
            buildInputs = [
              pkg-config
              rust
              xorg.libX11
              xorg.libXcursor
              xorg.libXrandr
              xorg.libXi
              xorg.libxcb
              libxkbcommon
              vulkan-loader
              wayland
              pre-commit
              nodejs
              nodePackages.npm
              watchexec
              just
            ];

            shellHook = ''
              HOOK_PATH=$(git rev-parse --git-path hooks/pre-commit)
              if [ ! -f "$HOOK_PATH" ]; then
                echo "Setting up pre-commit hooks..."
                ${pkgs.pre-commit}/bin/pre-commit install
              fi

              export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
            '';
          };
      }
    );
}
