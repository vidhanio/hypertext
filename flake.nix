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
            craneLib = inputs.crane.mkLib pkgs;
            nightlyCraneLib = craneLib.overrideToolchain (p: p.rust-bin.nightly.latest.default);

            src = craneLib.cleanCargoSource ./.;
            commonArgs = {
              inherit src;
              strictDeps = true;
            };

            cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          in
          {
            _module.args.pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [ inputs.rust-overlay.overlays.default ];
            };

            checks = {
              clippy = nightlyCraneLib.cargoClippy (
                commonArgs
                // {
                  inherit cargoArtifacts;
                  cargoClippyExtraArgs = "--all-targets -- --deny warnings";
                }
              );

              doc = nightlyCraneLib.cargoDoc (
                commonArgs
                // {
                  inherit cargoArtifacts;
                  env.RUSTDOCFLAGS = "--deny warnings";
                }
              );

              fmt = nightlyCraneLib.cargoFmt {
                inherit src;
              };

              deny = craneLib.cargoDeny {
                inherit src;
              };

              nextest = craneLib.cargoNextest (
                commonArgs
                // {
                  inherit cargoArtifacts;
                  partitions = 1;
                  partitionType = "count";
                  cargoNextestPartitionsExtraArgs = "--no-tests=pass";
                }
              );
            };

            devShells.default = craneLib.devShell {
              inherit (self') checks;

              packages = [
                pkgs.nil
                pkgs.rust-analyzer
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
