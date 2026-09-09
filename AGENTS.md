# Repository Guidelines

- `rustfmt.toml` uses unstable options, so formatting requires nightly rustfmt.
- CI treats warnings as errors and checks no-default, default, and all-feature configurations. Run no-default-feature and documentation tests when changing feature gates or public APIs; `cargo deny` audits all features.
- Use `#[expect(...)]` for intentional lint exceptions. Reserve `#[allow(...)]` for macro code whose expansion may legitimately omit the lint.

## Verification

Run the standard local gate:

```sh
treefmt --fail-on-change
cargo test --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny --all-features check
```

- Commit only coherent, validated changes with a lowercase Conventional Commit subject.
