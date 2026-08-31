{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    crane.url = "github:ipetkov/crane";
    systems.url = "github:nix-systems/default-linux";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } (
      { inputs, ... }:
      {
        imports = [ inputs.treefmt-nix.flakeModule ];

        systems = import inputs.systems;

        perSystem =
          {
            system,
            pkgs,
            self',
            config,
            ...
          }:
          let
            craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (
              p:
              p.rust-bin.nightly.latest.default.override {
                extensions = [
                  "rust-src"
                  "rust-analyzer"
                  "miri"
                ];
              }
            );
            # Keep Crane's Cargo source filter, but retain integration-test
            # fixtures such as `tests/templates/hello.html`.
            src = pkgs.lib.cleanSourceWith {
              src = ./.;
              filter =
                path: type:
                craneLib.filterCargoSources path type
                || pkgs.lib.hasPrefix (toString ./crates/hypertext/tests) (toString path);
            };
            commonArgs = {
              inherit src;
              strictDeps = true;
            };

            cargoArtifacts = craneLib.buildDepsOnly (
              commonArgs
              // {
                cargoExtraArgs = "--all-features";
              }
            );
          in
          {
            _module.args.pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [ inputs.rust-overlay.overlays.default ];
            };

            checks = {
              no-std = craneLib.mkCargoDerivation (
                commonArgs
                // {
                  inherit cargoArtifacts;
                  buildPhaseCargoCommand = "cargo check --workspace --all-targets --no-default-features --locked";
                  installPhaseCommand = "mkdir -p $out";
                  doCheck = false;
                }
              );

              clippy = craneLib.cargoClippy (
                commonArgs
                // {
                  inherit cargoArtifacts;
                  cargoClippyExtraArgs = "--workspace --all-targets --all-features -- --deny warnings";
                }
              );

              doc = craneLib.cargoDoc (
                commonArgs
                // {
                  inherit cargoArtifacts;
                  cargoDocExtraArgs = "--workspace --all-features --no-deps";
                  env.RUSTDOCFLAGS = "--deny warnings";
                }
              );

              fmt = craneLib.cargoFmt {
                inherit src;
              };

              deny = craneLib.cargoDeny {
                inherit src;
              };

              nextest = craneLib.cargoNextest (
                commonArgs
                // {
                  inherit cargoArtifacts;
                  cargoExtraArgs = "--workspace --all-features";
                  partitions = 1;
                  partitionType = "count";
                  cargoNextestPartitionsExtraArgs = "--no-tests=pass";
                }
              );
            };

            devShells.default = craneLib.devShell {
              inherit (self') checks;

              packages = [
                pkgs.cargo-edit
                pkgs.nil
                pkgs.prek
                config.treefmt.build.wrapper
              ];
            };

            treefmt = {
              programs = {
                nixfmt.enable = true;
                statix.enable = true;
                deadnix.enable = true;
                rustfmt = {
                  enable = true;
                  package = pkgs.rust-bin.nightly.latest.rustfmt;
                };
                taplo.enable = true;
              };
            };
          };
      }
    );
}
