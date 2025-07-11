{
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    fenix.url = "github:nix-community/fenix"; # We don't need it updated every day
    fenix.inputs.nixpkgs.follows = "nixpkgs";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    esp-dev = {
      url = "github:hsel-netsys/nixpkgs-esp-dev-rust";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.devenv.flakeModule
      ];
      perSystem =
        {
          pkgs,
          system,
          ...
        }:
        let
          fnx = inputs.fenix.packages.${pkgs.stdenv.system};
          rustToolchain = fnx.combine [
            fnx.complete.cargo
            fnx.complete.clippy
            fnx.complete.rust-src
            fnx.complete.rustc
            fnx.complete.rustfmt
            fnx.targets.thumbv7em-none-eabi.latest.rust-std
          ];
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.esp-dev.overlays.default
            ];
            config = { };
          };

          devenv.shells.default =
            let
            in
            {
              packages = [
                pkgs.cargo-generate
                pkgs.cmake
                pkgs.ninja
                pkgs.clang
                pkgs.ldproxy
                pkgs.probe-rs
                pkgs.rust-analyzer
                rustToolchain
                pkgs.trunk
              ];

              env = {
                LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
                CRATE_CC_NO_DEFAULTS = 1;
              };

              processes = {
              };

              languages.rust = {
                enable = false;
                mold.enable = true;
                toolchain = rustToolchain;
              };
            };
        };
    };
}
