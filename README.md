# AstryxChain

![CI Status](https://github.com/MeridianAlgo/AstryxChain/actions/workflows/ci.yml/badge.svg)

AstryxChain is an **experimental hashing research repository** focused on Astryx/GAQWH-style diffusion.
It currently includes:

- a **Python prototype** hash (`astryx.gaqwh`) used for rapid experimentation, and
- a **Rust crate** (`astryx`) with a pure-Rust walk/compression pipeline and fixed output APIs.

> ⚠️ **Security notice:** this project is unaudited and not standardized. It is not a replacement for SHA-2/SHA-3/BLAKE3 in production critical systems without substantial external cryptanalysis.

---

## Table of contents

1. [Project goals](#project-goals)
2. [Current status](#current-status)
3. [Repository structure](#repository-structure)
4. [Install and quick start (Python)](#install-and-quick-start-python)
5. [Rust crate usage](#rust-crate-usage)
6. [Deterministic example outputs](#deterministic-example-outputs)
7. [CLI usage](#cli-usage)
8. [How CI works](#how-ci-works)
9. [Testing locally](#testing-locally)
10. [Blockchain-oriented hardening guidance](#blockchain-oriented-hardening-guidance)
11. [Threat model and limitations](#threat-model-and-limitations)
12. [Roadmap](#roadmap)
13. [License](#license)

---

## Project goals

AstryxChain aims to provide a reproducible sandbox for evaluating a quantum-walk-inspired hash design with:

- deterministic behavior,
- variable output lengths,
- strong diffusion/avalanche behavior,
- practical APIs for blockchain-adjacent components (block IDs, tx IDs, Merkle leaves, PoW experiments),
- cross-language implementation experiments (Python reference/prototype + Rust implementation).

This repo is intentionally explicit about what it is **not**: a fully audited cryptographic suite.

## Current status

- **Maturity:** experimental research code.
- **Primary language stack:** Python + NumPy prototype, plus Rust crate.
- **Stability target:** deterministic outputs and regression-test coverage.
- **Security state:** no formal proof, no external third-party audit, no standardization-track review.

If your use case is a production blockchain, this repository is best treated as a testbed and should be paired with established primitives and a formal security process.

## Repository structure

```text
astryx/
  __init__.py                 # Python package exports
  core.py                     # compatibility re-exports
  gaqwh.py                    # Python prototype hash

src/
  lib.rs                      # Rust public API: astryx/astryx_256/astryx_512
  walk.rs                     # Rust walk simulation and quantization
  compress.rs                 # Rust compression/mixing layer
  tests.rs                    # Rust tests

.github/workflows/ci.yml      # Multi-job CI pipeline (Python + Rust + docs sanity)

tests/
  test_hash.py                # Python unit tests + CLI checks

scripts/
  benchmark_hashes.py         # benchmark helper
  produce_results.py          # deterministic output helper

cli.py                        # Python CLI entrypoint
Cargo.toml                    # Rust crate manifest
README.md
```

## Install and quick start (Python)

```bash
git clone https://github.com/MeridianAlgo/AstryxChain
cd AstryxChain
python -m pip install --upgrade pip
pip install -r requirements.txt
pip install .
```

### Python API

```python
from astryx import gaqwh

msg = "block_header_height_124"
h256 = gaqwh(msg, output_bits=256)
h512 = gaqwh(msg, output_bits=512)

print(h256)
print(h512)
```

`gaqwh(data, output_bits)` in Python:

- accepts `str | bytes`,
- returns lowercase hex string,
- requires `output_bits > 0` and multiple of 64.

## Rust crate usage

The Rust crate is named `astryx` and exposes:

- `astryx(input: &[u8], output_bits: usize) -> Vec<u8>` (`256` or `512`),
- `astryx_256(input: &[u8]) -> [u8; 32]`,
- `astryx_512(input: &[u8]) -> [u8; 64]`,
- compatibility aliases: `gaqwh`, `gaqwh_256`, `gaqwh_512`.

Example:

```rust
use astryx::{astryx_256, astryx_512};

let h256 = astryx_256(b"Astryx");
let h512 = astryx_512(b"Astryx");

assert_eq!(h256.len(), 32);
assert_eq!(h512.len(), 64);
```

## Deterministic example outputs

### Python prototype (`astryx.gaqwh`)

Message: `"Astryx"`

- 256-bit:
  `50c20f902e5d0995f654d0665ff05b1e5b7fba21cd637442a8d7690eff7c2466`
- 512-bit:
  `50c20f902e5d0995f654d0665ff05b1e5b7fba21cd637442a8d7690eff7c2466145519be2cb4a990c69ba3e4e5624ec18fa5b06855fd0fb5f22f31a39512ae5e`

Message: `"block_header_height_124"`

- 256-bit:
  `ee236a94851f098ecec9a5f4222f17794d0800c3113f565b0791b7522475a6d4`

### Rust crate (`astryx_256` / `astryx_512`)

Message: `b"Astryx"`

- 256-bit:
  `b0902b30549ac346d16ddf5c6756f6dd5efd8e77640a356172b85c8ce391545b`
- 512-bit:
  `b0902b30549ac346d16ddf5c6756f6dd5efd8e77640a356172b85c8ce391545b4abe83fe50bd98f5853995b7e3737668ad9ae5777e21c04bf1681c0c7a4cfe15`

> Note: Python prototype and Rust crate are both deterministic but currently separate implementation tracks and are **not byte-for-byte equivalent**.

## CLI usage

```bash
python cli.py "Astryx"
echo "Astryx" | python cli.py
python cli.py -b 256 "block_header_height_124"
python cli.py -b 512 "block_header_height_124"
```

If no positional input is provided and stdin is empty, the CLI returns a guidance message.

## How CI works

The GitHub Actions workflow (`.github/workflows/ci.yml`) has three jobs:

1. **python-tests**
   - matrix on Python `3.10/3.11/3.12`,
   - installs package + requirements,
   - runs unittest discovery,
   - runs a CLI smoke test.

2. **rust-tests**
   - installs stable Rust toolchain,
   - checks formatting (`cargo fmt --check`),
   - runs `cargo test --all-targets --all-features`,
   - uses Cargo cache for faster reruns.

3. **docs-consistency**
   - basic sanity checks that core docs and key files exist.

The workflow also includes:

- `workflow_dispatch` for manual runs,
- concurrency cancellation to avoid duplicated CI load on force-pushes.

## Testing locally

### Python tests

```bash
python -m unittest discover -s tests -q
```

### Rust tests

```bash
cargo fmt --all -- --check
cargo test --all-targets --all-features
```

### Benchmark script

```bash
python scripts/benchmark_hashes.py --size 1024 --iterations 2000
```

Optional benchmark comparators:

- `blake3` via `pip install blake3`
- `kangaroo12` via `pip install pycryptodome`

## Blockchain-oriented hardening guidance

To move toward blockchain-grade operational safety, use this checklist:

1. **Cryptanalysis phase**
   - commission independent external review,
   - run structured preimage/collision differential campaigns,
   - include reduced-round distinguishers and trail analyses.

2. **Protocol integration hygiene**
   - domain-separate all hash contexts (block header vs tx ID vs Merkle node),
   - use versioned prefixes (e.g., `b"ASTRYX-BLOCK-V1" || payload`),
   - avoid cross-protocol digest reuse without tags.

3. **Consensus safety**
   - lock exact algorithm + test vectors in spec,
   - cross-check outputs on multiple architectures/compilers,
   - introduce mandatory canonical serialization before hashing.

4. **Implementation hardening**
   - fuzz parsers and hash API boundaries,
   - add property tests (determinism, output-length invariants, avalanche distributions),
   - track performance regression thresholds in CI.

5. **Operational readiness**
   - define migration plan/versioning if hash changes,
   - document replay/rollback strategy,
   - stage activation via testnet and shadow-mainnet verification.

## Threat model and limitations

- No post-quantum security proof is provided.
- “Quantum-walk-inspired” does not imply standardized post-quantum assurances.
- No side-channel certification.
- No guarantee of equivalence between independent implementation families unless explicitly tested.

Use this repo for experimentation, education, and prototyping unless/until independent review says otherwise.

## Roadmap

- Publish a formal algorithm specification with normative test vectors.
- Add cross-language conformance harness (Python ↔ Rust) or explicitly version divergent variants.
- Add nightly benchmark trend reporting.
- Expand docs in `docs/` with protocol integration examples and domain-separation recipes.
- Add dedicated fuzzing and long-run statistical test automation.

## License

MIT.
