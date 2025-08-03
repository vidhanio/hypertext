# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.11.0](https://github.com/vidhanio/hypertext/compare/hypertext-v0.10.0...hypertext-v0.11.0) - 2025-08-03

### Added

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

- correct `*_static!` mention
- clean up lint rules
- add mathml/web components info
- re-add syn features

## [0.10.0](https://github.com/vidhanio/hypertext/compare/hypertext-v0.9.0...hypertext-v0.10.0) - 2025-07-28

### Fixed

- [**breaking**] add check for quotes
