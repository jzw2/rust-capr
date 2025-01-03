{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          buildInputs = [ yarn nodejs cargo rustc rustfmt pre-commit rustPackages.clippy rust-analyzer cargo-flamegraph # xdot
          wasm-pack
          lld
          nodePackages.eslint
	  iconv

          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;

        shellHook = ''
exec fish

'';
        };
      }
    );
}
