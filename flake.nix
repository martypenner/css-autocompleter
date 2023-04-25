{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , rust-overlay
    }:

    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [
        (import rust-overlay)
        (self: super: {
          rustToolchain =
            let
              rust = super.rust-bin;
            in
            if builtins.pathExists ./rust-toolchain.toml then
              rust.fromRustupToolchainFile ./rust-toolchain.toml
            else if builtins.pathExists ./rust-toolchain then
              rust.fromRustupToolchainFile ./rust-toolchain
            else
              rust.stable.latest.default;
        })
      ];

      pkgs = import nixpkgs { inherit system overlays; };
    in
    {
      formatter.x86_64-linux = nixpkgs.legacyPackages.x86_64-linux.nixpkgs-fmt;

      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          rustToolchain
          gnumake
          openssl
          pkg-config
          cargo-deny
          cargo-edit
          cargo-watch
          rust-analyzer
					wasm-pack
					wasm-bindgen-cli

          # nodejs 18
          nodejs

          exa
          fd
          fzf
        ];

        shellHook = ''
          alias ls="exa"
          alias l="ls -lh"
          alias find="fd"
          ${pkgs.rustToolchain}/bin/cargo --version
        '';
      };
    });
}
