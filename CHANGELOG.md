# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.1] - 2026-04-30

### Changed
- Migrate `WparseReason` to `derive(OrionError)`, replacing manual
  `ErrorCode`/`DomainReason` impls and `thiserror::Error`
- Fix `ToStructError` import path for orion-error 0.7.2

[Unreleased]: https://github.com/wp-labs/wp-parse-api/compare/v0.8.1...HEAD
[0.8.1]: https://github.com/wp-labs/wp-parse-api/releases/tag/v0.8.1
