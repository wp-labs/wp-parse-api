# Repository Guidelines

## Project Structure & Module Organization
Each crate in `warp-pase-system` (e.g., `wp-parse-api`, `wp-data-model`, `wp-connector-api`) is a standalone Rust library under its own directory. Source lives in `src/`, fixtures and shared assets sit beside the codebase, and unit tests are colocated via `mod tests` in the same files or `tests/` for integration-style coverage. Crates use relative `../../` paths to depend on siblings, so keep module layouts stable when moving files.

## Build, Test, and Development Commands
Run `cargo build` inside a crate to verify compilation, or `cargo build -p wp-parse-api` from the workspace root when touching multiple crates. Execute `cargo test` (or `cargo test -p wp-data-model`) to run unit tests. Generate docs locally with `cargo doc --open`. Before submitting changes, run `cargo fmt --all` and `cargo clippy --all-targets --all-features -D warnings` to enforce formatting and lint cleanliness.

## Coding Style & Naming Conventions
This workspace targets Rust 2021 with default `rustfmt` (4-space indent, standard line width). Use `CamelCase` for types and traits, `snake_case` for functions/modules, and `SCREAMING_SNAKE_CASE` for constants. Favor `Result`-returning helpers over `unwrap`/`expect`, document public APIs with `///`, and add module-level docs using `//!` when needed. Keep comments focused on intent rather than mechanics.

## Testing Guidelines
Write deterministic unit tests next to the code they exercise; name them `test_*` for clarity. Use `#[tokio::test]` for async cases and `cargo test` as the canonical execution path. Prefer lightweight fixtures and ensure new behavior includes coverage. Optional fuzzing exists under `wp-data-model/fuzz/` via `cargo fuzz run <target>` when touching parsers or serialization logic.

## Commit & Pull Request Guidelines
Follow Conventional Commits with crate scopes when helpful, e.g., `feat(wp-parse-api): add JSON tokenizer`. PRs should explain intent, link issues, and include logs or screenshots when behavior shifts. Ensure formatting, clippy, and tests succeed for affected crates before requesting review, and document noteworthy trade-offs in the PR description.

## Security & Configuration Tips
Never commit sample secrets or keys; treat all input as untrusted data. Prefer workspace-managed dependencies and keep cross-crate paths accurate to avoid supply-chain drift. When handling errors, map them into the workspace error type so downstream services receive consistent diagnostics.
