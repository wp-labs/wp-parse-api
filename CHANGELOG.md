# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.10.0] - 2026-05-03

### Changed
- Bump orion-error from 0.7 to 0.8; rename `UvsReason` → `UnifiedReason`
- Replace `WparseReason::from_data()` with delegate constructor `data_error()`
- Fix `derive_more` v2.x feature gate

### Removed
- Remove `UvsFrom` import (removed in 0.8)

## [0.9.0] - 2026-04-30

### Changed
- Migrate `WparseReason` to `derive(OrionError)`, replacing manual
  `ErrorCode`/`DomainReason` impls and `thiserror::Error`
- Fix `ToStructError` import path for orion-error 0.7.2
- Bump orion-error dependency from 0.6 to 0.7

[Unreleased]: https://github.com/wp-labs/wp-parse-api/compare/v0.10.0...HEAD
[0.10.0]: https://github.com/wp-labs/wp-parse-api/releases/tag/v0.10.0
[0.9.0]: https://github.com/wp-labs/wp-parse-api/releases/tag/v0.9.0
