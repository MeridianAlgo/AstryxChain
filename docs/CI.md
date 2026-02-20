# CI Pipeline Guide

This repository uses `.github/workflows/ci.yml` as a multi-language CI pipeline.

## Jobs

- **python-tests**: matrix over Python 3.10, 3.11, 3.12; installs deps; runs unit tests and CLI smoke test.
- **rust-tests**: stable Rust toolchain; runs `cargo fmt --check` and `cargo test`.
- **docs-consistency**: verifies expected top-level docs/config files exist.

## Reliability settings

- `concurrency` cancels in-progress duplicate runs for the same branch/PR.
- `workflow_dispatch` enables manual CI execution.
- Caching is enabled for pip and cargo to reduce run time.

## Extending CI

Suggested additions:

1. Add `clippy` and doc linting (`RUSTDOCFLAGS="-D warnings" cargo doc`).
2. Add benchmark thresholds for regression alerts.
3. Publish deterministic output vectors as workflow artifacts.
4. Add scheduled nightly security/statistical test runs.
