{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # For more packages/package search go to https://search.nixos.org/
            rust-bin.stable.latest.default
            sccache
            rust-analyzer
            cargo-nextest
            cargo-watch
            cargo-release
            sqlx-cli
            openssl
            pkg-config
            docker
          ];

          shellHook = ''
            export RUSTC_WRAPPER=$(which sccache)
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [ pkgs.openssl ]}
          '';
        };
      });
}
