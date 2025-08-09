# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.11.1](https://github.com/vidhanio/hypertext/compare/hypertext-v0.11.0...hypertext-v0.11.1) - 2025-08-09

### Added

- giant rewrite ([#139](https://github.com/vidhanio/hypertext/pull/139))

### Fixed

- use parens instead of brace

### Other

- cleanup

### Added

- [**breaking**] unify all duplicated `Attribute` versions of types/traits with the `Context` trait
- [**breaking**] make it harder to accidentally make your code vulnerable to XSS via `Buffer`, hiding
  constructors, and `dangerously_*` functions

### Changed

- [**breaking**] rename `html_elements` to `hypertext_elements`
- [**breaking**] rename `[void_]elements!` to `define_[void_]elements!`
- [**breaking**] reorganize all validation-related modules into `validation`

## [0.11.0](https://github.com/vidhanio/hypertext/compare/hypertext-v0.10.0...hypertext-v0.11.0) - 2025-08-06

### Added

- [**breaking**] add handler/`role` attributes and re-org attribute traits (fixes #136)
- export `void_elements!` (fixes #132)
- [**breaking**] support `:` in maud class syntax (fixes #129)
- add custom vis support to `#[component]`
- add ntex support
- make struct unit if no args
- reduce syn feature tree

### Fixed

- only run tests in alloc
- suppress errors about unused delims on toggles (fixes #130)

### Other

- Bump warp from 0.3.7 to 0.4.0 ([#137](https://github.com/vidhanio/hypertext/pull/137))
- get rid of extra `http` dep
- simplify features and macros
- fix docs and ci
- correct `*_static!` mention
- clean up lint rules
- add mathml/web components info
- re-add syn features

## [0.10.0](https://github.com/vidhanio/hypertext/compare/hypertext-v0.9.0...hypertext-v0.10.0) - 2025-07-28

### Fixed

- [**breaking**] add check for quotes
