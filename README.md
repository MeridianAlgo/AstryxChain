# Astryx (GAQWH)

**Astryx** is a next-generation, high-performance, and quantum-resistant hashing algorithm designed for industrial-scale blockchain architectures. It implements the **Adaptive Quantum Walk Hash (GAQWH)**, a sophisticated cryptographic engine that combines the chaotic evolution of non-linear quantum dynamics with robust classical bit-diffusion.

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
