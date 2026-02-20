# AstryxChain

![CI Status](https://github.com/MeridianAlgo/AstryxChain/actions/workflows/ci.yml/badge.svg)

AstryxChain is a Python package that currently ships one primitive: `gaqwh`, a custom deterministic hash function.

This repository does **not** implement a signature scheme (such as ML-DSA / Dilithium), key exchange, encryption, or consensus logic. It is focused on hashing only.

---

## Table of contents

1. [Project status](#project-status)
2. [What AstryxChain includes](#what-astryxchain-includes)
3. [What AstryxChain does not include](#what-astryxchain-does-not-include)
4. [Repository layout](#repository-layout)
5. [Installation](#installation)
6. [API reference](#api-reference)
7. [Command-line interface](#command-line-interface)
8. [Benchmarking](#benchmarking)
9. [Testing](#testing)
10. [Security and production-use warning](#security-and-production-use-warning)
11. [Roadmap](#roadmap)
12. [License](#license)

---

## Project status

- **Current maturity:** experimental.
- **Implementation language:** Python + NumPy.
- **Primary goal right now:** provide a reproducible hashing prototype and tooling around it.

If your end goal is a production blockchain, treat this repository as a research/prototyping component and pair it with standardized cryptographic primitives for signatures and key management.

## What AstryxChain includes

- A callable hash function: `gaqwh(data, output_bits=256)`.
- A simple class wrapper (`Astryx`) that powers the hash routine.
- Output length selection in multiples of 64 bits.
- A CLI (`cli.py`) for hashing text input or piped input.
- Unit tests under `tests/`.
- A benchmark script (`scripts/benchmark_hashes.py`) to compare throughput with common reference hashes.

## What AstryxChain does not include

- No digital signature scheme.
- No wallet/key derivation standards.
- No consensus implementation.
- No formal cryptographic proof.
- No third-party cryptanalysis report.

Because of those gaps, the project should not be marketed as a replacement for mature standardized cryptographic systems.

## Repository layout

```text
astryx/
  __init__.py          # public package exports
  core.py              # compatibility re-exports
  gaqwh.py             # full hash implementation

scripts/
  benchmark_hashes.py  # local benchmark utility
  produce_results.py   # sample digest printer

tests/
  test_hash.py         # unit tests + CLI tests

cli.py                 # command line entry point
README.md
requirements.txt
setup.py
```

## Installation

### 1) Clone repository

```bash
git clone https://github.com/MeridianAlgo/AstryxChain
cd AstryxChain
```

### 2) Install dependencies

```bash
pip install -r requirements.txt
```

### 3) Install package

```bash
pip install .
```

## API reference

### `gaqwh(data, output_bits=256) -> str`

Hashes a string or bytes input and returns a lowercase hex digest.

#### Parameters

- `data` (`str | bytes`): message to hash.
- `output_bits` (`int`, default `256`): output size in bits.

#### Constraints

- `output_bits` must be:
  - greater than 0
  - a multiple of 64

Invalid values raise `ValueError`.

#### Return value

- Hex string with length `output_bits / 4` characters.

#### Example

```python
from astryx import gaqwh

message = "block_header_height_124"
digest_256 = gaqwh(message)
digest_512 = gaqwh(message, output_bits=512)

print(digest_256)
print(digest_512)
```

### Backward-compatible imports

Both import styles are supported:

```python
from astryx import gaqwh
# or
from astryx.core import gaqwh
```

## Command-line interface

### Hash a positional argument

```bash
python cli.py "Astryx"
```

### Hash piped input

```bash
echo "Astryx" | python cli.py
```

### Choose output size

```bash
python cli.py -b 128 "Astryx"
python cli.py -b 256 "Astryx"
python cli.py -b 512 "Astryx"
```

### Empty input behavior

If no argument is provided and stdin is empty, CLI prints a guidance message and exits without generating a digest.

## Benchmarking

A local benchmark script is available:

```bash
python scripts/benchmark_hashes.py --size 1024 --iterations 2000
```

### Always measured

- `astryx_gaqwh`
- `sha3_256`

### Optional algorithms

- `blake3` (install: `pip install blake3`)
- `kangaroo12` (install: `pip install pycryptodome`)

### Notes on benchmark interpretation

- Results vary by CPU model, power state, and Python version.
- GAQWH is implemented in Python/NumPy and is expected to be slower than highly optimized production hashes.
- Run multiple times and report median values for fair comparison.

## Testing

Run the full test suite:

```bash
python -m unittest discover -s tests -q
```

What tests currently check:

- Determinism.
- String/bytes parity.
- Output format and length.
- Basic avalanche-style bit-difference check.
- Small collision smoke test over a tiny corpus.
- Large-input handling.
- CLI behavior.

## Security and production-use warning

This repository is experimental software.

Before considering production use, you should complete at least:

1. External cryptanalysis by qualified reviewers.
2. Side-channel evaluation.
3. Cross-platform reproducibility verification.
4. Strict threat-model documentation.
5. Performance profiling under real deployment conditions.

Until those activities are completed, do not treat this package as a substitute for standardized audited primitives in critical systems.

## Roadmap

Short-term improvements that would make this repository more useful:

1. Publish formal algorithm specification (step-by-step and test vectors).
2. Expand statistical test corpus and reporting artifacts.
3. Add repeatable benchmark matrix in CI for selected environments.
4. Add packaging/docs polish (versioned docs, changelog, release notes).
5. If targeting production, implement performance-critical path in Rust/C and preserve deterministic compatibility tests.

## License

MIT License.
