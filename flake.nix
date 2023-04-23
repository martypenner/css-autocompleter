{
  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixos-unstable;
    flake-utils.url = github:numtide/flake-utils;
    rust-overlay.url = github:oxalica/rust-overlay;
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachSystem ["x86_64-linux"] (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
    in {
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          (rust-bin.beta.latest.default.override {
            targets = [ "wasm32-unknown-unknown" ];
          })
          wasm-pack
          wasm-bindgen-cli
        ];

				buildInputs = with pkgs; [
          openssl
          pkg-config
          exa
          fd
				];

        shellHook = ''
          alias ls="exa"
          alias l="ls -lh"
          alias find="fd"
        '';
      };
    }
  );
}
