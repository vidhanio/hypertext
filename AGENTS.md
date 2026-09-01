# Repository Guidelines

## Repository specifics

- This is a Rust 2024 workspace under `crates/*`. `hypertext` is `#![no_std]`; allocation and web-framework adapters are feature-gated.
- Use the Nix dev shell for reproducible tooling. It provides nightly Rust, rustfmt, clippy, Miri, cargo-edit, and treefmt; the ordinary `PATH` may not contain a Rust toolchain.
- `rustfmt.toml` uses unstable options, so formatting requires nightly rustfmt.
- Flake builds use the tracked flake source. Stage new files before invoking `nix build` or a Nix check.
- CI treats warnings as errors, runs Clippy on nightly, and checks no-default, default, and all-feature configurations. `cargo deny` audits all features.
- Integration tests and their fixtures live under `crates/hypertext/tests`; many are enabled only with `alloc`.
- Use `#[expect(...)]` for intentional lint exceptions. Reserve `#[allow(...)]` for macro code whose expansion may legitimately omit the lint.

## Lifecycle

Work from the repository root. The normal local gate is:

```sh
nix develop -c treefmt --fail-on-change
nix develop -c cargo test --workspace --all-targets --all-features
nix develop -c cargo clippy --workspace --all-targets --all-features -- -D warnings
nix develop -c cargo deny --all-features check
```

Run the no-default-feature check and documentation tests when changing feature
gates or public APIs. Commit only a coherent, validated change using a
lowercase Conventional Commit subject.
