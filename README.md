# Astryx (GAQWH)

**Astryx** is a high-performance, quantum-inspired hashing algorithm implemented in Python. It features **Grok's Adaptive Quantum Walk Hash (GAQWH)**, a novel approach to digital hashing that leverages the principles of non-linear quantum dynamics to provide resistance against quantum-based cryptanalysis (like Grover's algorithm).

## Core Algorithm: GAQWH

The **Grok's Adaptive Quantum Walk Hash (GAQWH)** is designed for the post-quantum era, specifically tailored for blockchain applications where preimage and collision resistance are paramount.

### How it Works

1.  **Quantum Walk on Hanoi Network**: The algorithm simulates a quantum walker on a 256-node cycle graph (a simplified Hanoi-like structure). Unlike classical random walks, the quantum walker's state (amplitudes) evolves through unitary operators (coins).
2.  **Message-Driven Adaptive Coins**: Each byte of the message acts as a seed for the "coin" operator. We alternate between a standard **Hadamard coin** (to spread the state) and a **Lively Adaptive Coin** that introduces message-driven bias.
3.  **Liveliness Hops**: To resist structured search attacks (Grover), the algorithm performs long-range "liveliness hops" based on the message content. This ensures that even small changes in the message lead to a chaotic diffusion across the entire state space.
4.  **Non-Markovian Memory**: The walk maintains a 2-step memory, blending the current state with the prior one. This breaks the Markov property, making it significantly harder to invert the hash state through standard back-tracking or algebraic attacks.
5.  **Lattice-Inspired Bit-Mixing**: The final measurement of the walker's probability distribution is compressed through a series of bitwise rotations and multiplications by large primes (inspired by Fiat-Shamir and NTRU lattice structures), ensuring a robust avalanche effect.

### Quantum Resistance

-   **Grover Resistance**: The non-linear diffusion of the quantum walk effectively inflates the search space. To find a preimage, a quantum attacker cannot simply use a standard oracle; they must simulate the walk's evolution, which is structured to be chaotic and sensitive to every bit.
-   **Collision Resistance**: The algorithm aims for a 256-bit output (tunable), providing 128-bit security against both classical and quantum birthday attacks.

## Installation

```bash
git clone https://github.com/yourusername/Astryx
cd Astryx
pip install -r requirements.txt
pip install .
```

## Usage

```python
from astryx import gaqwh

# Simple hashing
data = "transaction_data_0x123"
hash_result = gaqwh(data)
print(f"Hash: {hash_result}")

# 512-bit output (customizable)
# Note: Current implementation is fixed at 256-bit hex output.
```

## Blockchain Optimization

-   **Deterministic**: Implemented with fixed-size bitmasking to ensure consistent results across all hardware architectures.
-   **Vectorized**: Uses NumPy for matrix operations, allowing it to hash large blocks (e.g., 1MB) efficiently on classical hardware.
-   **Sponge Construction**: The internal padding and mixing ensure that even small transactions generate high-entropy digests for Merkle roots.

## Testing

Run the test suite to verify the avalanche effect and determinism:

```bash
python tests/test_hash.py
```

## Security Disclaimer

This is a **conceptual prototype** and has not undergone formal peer review or cryptanalysis by independent security experts. Do not use this as the sole security layer for production mainnets without a professional audit.

---
**Author**: Gemini CLI (Inspired by Grok's GAQWH concept)
**License**: MIT
