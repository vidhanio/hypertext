# AGENTS.md

Guidance for agentic coding assistants working in this repository.

## Repository Snapshot

- Language: Rust (edition 2024).
- Workspace: `crates/*` with two crates:
  - `hypertext` (library, `#![no_std]`, optional `alloc` feature).
  - `hypertext-macros` (proc-macro crate).
- Primary focus: compile-time validated HTML/RSX/Maud macros and rendering.
- CI treats warnings as errors (`RUSTFLAGS=-D warnings`, `RUSTDOCFLAGS=-D warnings`).

## Directory Layout

- `Cargo.toml`: workspace config, shared lints, shared metadata.
- `crates/hypertext/src`: runtime types, rendering, validation, framework adapters.
- `crates/hypertext-macros/src`: parser/generator and proc macros.
- `crates/hypertext/tests`: integration-style tests (`attributes`, `components`, etc.).
- `.github/workflows/ci-cd.yaml`: source of truth for CI checks.
- `rustfmt.toml`, `clippy.toml`, `deny.toml`: formatting/lint/security policy.

## Required Commands

Run all commands from repo root: `/home/vidhanio/Projects/hypertext`.

### Build and Check

- Quick workspace check:
  - `cargo check --workspace --all-targets`
- Check without default features (important for `no_std` path):
  - `cargo check --workspace --all-targets --no-default-features`
- Check with all features:
  - `cargo check --workspace --all-targets --all-features`

### Tests

- Run main integration test suite similar to CI matrix:
  - `cargo test --tests --no-default-features`
  - `cargo test --tests --no-default-features --features default`
  - `cargo test --tests --no-default-features --all-features`
- Run all tests (local convenience):
  - `cargo test --workspace --all-targets --all-features`
- Run docs tests (CI behavior):
  - `cargo test --doc --all-features`

### Running a Single Test (Important)

- Single test function in one integration test file:
  - `cargo test -p hypertext --test components renderable_custom_name --features default -- --exact`
- Single integration test file:
  - `cargo test -p hypertext --test components --features default`
- Filter tests by substring in a file:
  - `cargo test -p hypertext --test integration blog_post --features default`
- List available tests before choosing one:
  - `cargo test -p hypertext --test components --features default -- --list`

Notes:
- Most tests in `crates/hypertext/tests` are behind `#![cfg(feature = "alloc")]`.
- Use `--features default` (or `--all-features`) when running those tests.

### Linting

- Clippy (workspace, strict warnings):
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- CI-compatible clippy matrix flavor:
  - `cargo clippy --no-default-features`
  - `cargo clippy --no-default-features --features default`
  - `cargo clippy --no-default-features --all-features`

### Formatting

- Check formatting (CI):
  - `cargo +nightly fmt --all --check`
- Apply formatting:
  - `cargo +nightly fmt --all`

Why nightly? `rustfmt.toml` uses unstable options (`unstable_features = true`).

### Additional Quality/Security Checks

- Miri (as in CI, nightly toolchain):
  - `cargo +nightly miri setup`
  - `cargo +nightly miri test --tests --no-default-features --all-features`
- Dependency/license audit:
  - `cargo deny check`
- Docs.rs compatibility check (if installed):
  - `cargo docs-rs -p hypertext`

## Code Style and Conventions

### Formatting and Imports

- Always run rustfmt using the repository config.
- Imports are grouped by rustfmt with `group_imports = "StdExternalCrate"`.
- Imports are granular at crate level (`imports_granularity = "Crate"`).
- Preserve existing import ordering; avoid manual style churn.

### Naming

- Types/traits/enums: `PascalCase`.
- Functions/modules/variables/tests: `snake_case`.
- Constants/statics: `SCREAMING_SNAKE_CASE`.
- Keep test names descriptive and behavior-focused.

### Types and API Design

- Prefer explicit, strongly typed APIs; avoid type erasure unless necessary.
- Maintain `no_std` compatibility in `hypertext`; gate allocation-dependent code with `#[cfg(feature = "alloc")]`.
- Preserve existing context-typed rendering model (`Context`, `Node`, `AttributeValue`).
- Prefer trait-based extensibility (`Renderable`, `RenderableExt`) over ad-hoc helpers.

### Error Handling

- Do not add `unwrap`, `expect`, or `panic!` in library/proc-macro logic.
- In proc-macro code, return `syn::Result<_>` and convert errors with `to_compile_error()`.
- Prefer compile-time diagnostics over runtime failure whenever possible.

### Unsafe and Security-Sensitive Code

- Unsafe blocks must carry clear `// SAFETY:` justification comments.
- Raw HTML/string entry points use `dangerously_*` naming; keep this explicit.
- When writing directly to buffers or constructing raw HTML, include `// XSS SAFETY:` comments explaining trust boundaries.
- Never weaken escaping guarantees for node or attribute contexts.

### Docs and Lints

- Public APIs should remain documented; workspace enables `missing_docs` warnings.
- Keep doctests valid and warning-free.
- Use targeted `#[expect(...)]` only when justified; prefer fixing lint findings.

### Testing Expectations

- Add or update tests with behavior changes.
- Prefer parity tests across Maud and RSX when adding syntax/semantics.
- Assert exact HTML output when practical; use `starts_with`/`contains` only when exact output is intentionally flexible.

### Feature Flag Discipline

- Validate changes under:
  - no default features,
  - `--features default`,
  - `--all-features`.
- Web framework adapters live behind optional features; avoid cross-feature breakage.

## Practical Workflow for Agents

1. Read affected modules and tests first.
2. Make minimal, focused edits.
3. Run targeted single-test command(s).
4. Run formatting and clippy.
5. Run broader test matrix if behavior or feature gates changed.
6. Ensure no warnings remain before finishing.
