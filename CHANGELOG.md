# Changelog

[![Keep a Changelog](https://img.shields.io/badge/Keep%20a%20Changelog-1.0.0-orange.svg)](https://keepachangelog.com/en/1.0.0/)
[![Semantic Versioning](https://img.shields.io/badge/Semantic%20Versioning-2.0.0-blue.svg)](https://semver.org/spec/v2.0.0.html)

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### ⚠️ BREAKING CHANGES

- 依赖 `wp-model-core` 0.9 → 0.10（上游把整数类型正名：`Value::Digit` → `Value::Int`、`DataType::Digit` → `DataType::Int`、serde 名 `"digit"` → `"int"`、公开构造器 `from_digit` → `from_int`；`DataType::Array` 的载荷类型 `String` → `ArraySubtype`）

### Changed

- 许可证由 Elastic License 2.0 改为 Apache License 2.0（`LICENSE` 全文、`Cargo.toml` 的 `license` 字段、README 许可徽章三处一并更新）
- README 补齐常用徽章（crates.io、docs.rs、crates.io 下载量、Rust Edition），CI 徽章改用现代形式并链接到 Actions 工作流
- CHANGELOG 头部补充 Keep a Changelog 与 Semantic Versioning 徽章

### Dependencies

- `wp-model-core`：`0.9` → `0.10`

## [0.11.0] - 2026-08-04

### ⚠️ BREAKING CHANGES

- 依赖 `wp-model-core` 0.8 → 0.9（上游新增 `Value::BigUint` / `DataType::BigInt` 变体）

### Dependencies

- `wp-model-core`：`0.8` → `0.9`

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

[Unreleased]: https://github.com/wp-labs/wp-parse-api/compare/v0.11.0...HEAD
[0.11.0]: https://github.com/wp-labs/wp-parse-api/compare/v0.10.0...v0.11.0
[0.10.0]: https://github.com/wp-labs/wp-parse-api/releases/tag/v0.10.0
[0.9.0]: https://github.com/wp-labs/wp-parse-api/releases/tag/v0.9.0
