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
      formatter = pkgs.nixpkgs-fmt;

      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          # Rust
          rustToolchain
          openssl # TODO: use rustls instead. Will require messing with features on cargo dependencies
          pkg-config
          cargo-deny
          cargo-edit
          cargo-watch
          rust-analyzer

          vsce

          # nodejs 18
          nodejs

          just
          exa
          fd
          fzf
        ];

        shellHook = ''
          alias ls="exa"
          alias l="ls -lh"
          alias find="fd"
          alias make="just"
          ${pkgs.rustToolchain}/bin/cargo --version
        '';
      };
    });
}
