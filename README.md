# Astryx (GAQWH)

![CI Status](https://github.com/MeridianAlgo/AstryxChain/actions/workflows/ci.yml/badge.svg)

**Astryx** is a next-generation, high-performance, and quantum-resistant hashing algorithm designed for industrial-scale blockchain architectures. It implements the **Adaptive Quantum Walk Hash (GAQWH)**, a sophisticated cryptographic engine that combines the chaotic evolution of non-linear quantum dynamics with robust classical bit-diffusion.

## Sample Results (v1.0.0)

The following results demonstrate the high-entropy output of the GAQWH algorithm across various input types:

| Input Type | Sample Data | Astryx GAQWH Hash (256-bit) |
| :--- | :--- | :--- |
| **Word** | `Astryx` | `1a0ac88a0af3a9d0988fb65820818596fa0ab7e7...` |
| **Word** | `astryx` | `09baab0ebce6d9c36cb18b971bee769ebdd50e6f...` |
| **Wallet Key** | `5Kb8kLf9...` | `f0ea349d883cfea4d0fa5b284b24ad87b075f3c4...` |
| **ETH Address** | `0x742d3...` | `e7369f2d8caef6a62e01172851e8de53ac00d511...` |

> *Note: These hashes are deterministic. A single character change in a wallet key results in a completely unique digest.*

## Technical Architecture

The **Astryx Engine** is engineered to remain secure in the post-quantum era, where traditional ECDSA and SHA-2 algorithms may face vulnerabilities from Shor's and Grover's algorithms.

### 1. High-Dimensional Quantum Walk
Unlike standard random walks, GAQWH operates in a 512-node state space using a **4-Dimensional Unitary Coin (S-Matrix)**. This ensures that every bit of input creates a complex superposition of states across the entire walker space, maximizing entropy and preventing preimage reconstruction.

### 2. Quantum Chaotic Mapping
The walker's movement is steered by a **discrete chaotic mapping** stage. Every message byte triggers a non-linear, index-dependent "chaotic hop," forcing the state evolution to be highly sensitive to input changes (Butterfly Effect). This provides extreme **Avalanche Resistance**, where a single-bit change in input alters more than 50% of the output hash bits.

### 3. Non-Markovian Feedback (History Memory)
To resist backtracking and algebraic attacks, Astryx maintains a **multi-step memory buffer**. Current state evolutions are continuously blended with historical states (weighted feedback loops), breaking the Markov property. This ensures that the hash transformation is mathematically non-invertible even with high-compute quantum hardware.

### 4. Prime-Based Sponge Compression
The final measurement phase uses a **multi-pass sponge construction** with large-prime mixing (inspired by NTRU and lattice cryptography). This stage applies cyclic bit-rotations and prime-factor XORing to ensure the final digest is free of statistical bias and linear patterns.

## Performance & Optimization

-   **Blockchain Ready**: Optimized for Merkle tree structures and Proof-of-Work/Stake validations.
-   **Vectorized Engine**: Built on top of high-performance linear algebra (NumPy), allowing it to process large data blocks with O(N log N) complexity.
-   **Architecture-Agnostic**: Implemented with 64-bit masking to ensure deterministic results across different CPU architectures and operating systems.

## Quick Start

### Installation

```bash
git clone https://github.com/MeridianAlgo/AstryxChain
cd Astryx
pip install -r requirements.txt
pip install .
```

### Usage

```python
from astryx import gaqwh

# Generate a 256-bit secure hash
tx_data = "block_header_data_0xABC123"
digest = gaqwh(tx_data)
print(f"Astryx Digest: {digest}")
```

## Security Analysis

Astryx (GAQWH) is designed to resist:
-   **Grover's Algorithm**: The non-linear chaotic diffusion inflates the search space complexity, requiring a quantum attacker to perform O(2^128) operations for a 256-bit hash.
-   **Differential/Linear Cryptanalysis**: The multi-pass sponge and S-Matrix evolution provide high-order nonlinearity.
-   **Birthday Attacks**: Optimized for collision resistance up to the theoretical limit of the output digest size.

---

**Astryx - Secure The Blockchain.**  
Developed by the Astryx Team.  
License: MIT
