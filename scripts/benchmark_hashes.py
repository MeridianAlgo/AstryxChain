"""Micro-benchmark Astryx GAQWH against common hash functions.

Usage:
    python scripts/benchmark_hashes.py --size 1024 --iterations 5000
"""

from __future__ import annotations

import argparse
import hashlib
import os
import statistics
import time
from typing import Callable

import pathlib
import sys

REPO_ROOT = pathlib.Path(__file__).resolve().parents[1]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from astryx import gaqwh


def _maybe_blake3() -> Callable[[bytes], bytes] | None:
    try:
        import blake3  # type: ignore

        return lambda m: blake3.blake3(m).digest()
    except Exception:
        return None


def _maybe_k12() -> Callable[[bytes], bytes] | None:
    try:
        from Crypto.Hash import KangarooTwelve  # type: ignore

        return lambda m: KangarooTwelve.new(data=m).read(32)
    except Exception:
        return None


def bench(name: str, fn: Callable[[bytes], object], msg: bytes, iterations: int) -> dict:
    samples = []
    for _ in range(5):
        start = time.perf_counter()
        for _ in range(iterations):
            fn(msg)
        elapsed = time.perf_counter() - start
        samples.append(elapsed)

    median = statistics.median(samples)
    total_bytes = len(msg) * iterations
    mb_s = (total_bytes / (1024 * 1024)) / median
    return {
        "name": name,
        "median_s": median,
        "mb_s": mb_s,
        "hashes_s": iterations / median,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description="Benchmark Astryx vs SHA3/BLAKE3/K12")
    parser.add_argument("--size", type=int, default=1024, help="Message size in bytes")
    parser.add_argument(
        "--iterations", type=int, default=2000, help="Hashes per timing sample"
    )
    args = parser.parse_args()

    msg = os.urandom(args.size)

    algos: list[tuple[str, Callable[[bytes], object]]] = [
        ("astryx_gaqwh", lambda m: gaqwh(m)),
        ("sha3_256", lambda m: hashlib.sha3_256(m).digest()),
    ]

    blake3_fn = _maybe_blake3()
    if blake3_fn is not None:
        algos.append(("blake3", blake3_fn))

    k12_fn = _maybe_k12()
    if k12_fn is not None:
        algos.append(("kangaroo12", k12_fn))

    print(f"message_size={args.size} bytes | iterations={args.iterations} | samples=5")
    print("name           median(s)   hashes/s      MB/s")
    print("-" * 50)

    for name, fn in algos:
        row = bench(name, fn, msg, args.iterations)
        print(
            f"{row['name']:<14} {row['median_s']:<10.4f} {row['hashes_s']:<12.0f} {row['mb_s']:.2f}"
        )

    if blake3_fn is None or k12_fn is None:
        print("\nOptional dependencies:")
        if blake3_fn is None:
            print("- pip install blake3")
        if k12_fn is None:
            print("- pip install pycryptodome")


if __name__ == "__main__":
    main()
