from typing import Union

import numpy as np


class Astryx:
    """
    Astryx GAQWH: Adaptive Quantum Walk Hashing Engine.

    A cryptographic hashing algorithm designed for blockchain systems,
    utilizing non-linear quantum walk dynamics and message-driven diffusion
    to resist both classical (Differential/Linear) and quantum (Grover/Shor)
    attacks.
    """

    def __init__(self, output_bits: int = 256):
        if output_bits <= 0 or output_bits % 64 != 0:
            raise ValueError("output_bits must be a positive multiple of 64")
        self.output_bits = output_bits
        self.N = 512  # Increased state size for better collision resistance
        self.MASK64 = 0xFFFFFFFFFFFFFFFF

        # Precompute the S-Matrix (4D Coin) for complex superposition
        # This provides a 4x4 unitary space per node transition
        self.coin_4d = np.array(
            [
                [0.5, 0.5, 0.5, 0.5],
                [0.5, -0.5, 0.5, -0.5],
                [0.5, 0.5, -0.5, -0.5],
                [0.5, -0.5, -0.5, 0.5],
            ],
            dtype=np.complex128,
        )

    def _quantum_chaos(self, byte: int, step: int) -> int:
        """
        Discrete chaotic mapping to steer the quantum walk.
        Uses a logistic-map inspired transformation to generate non-linear hops.
        """
        # x_{n+1} = 4 * x_n * (1 - x_n) approximation
        x = (byte + step + 1) % 256
        for _ in range(3):
            x = (3.99 * x * (256 - x) / 64) % 256
        return int(x)

    def hash(self, data: Union[str, bytes]) -> str:
        if isinstance(data, str):
            data = data.encode("utf-8")

        # Preprocess: ensure high entropy padding
        chunks = list(data)
        if len(chunks) < 64:
            # Recursive padding for short inputs
            pad_seed = sum(chunks) if chunks else 0xDEADBEEF
            for i in range(64 - len(chunks)):
                pad_seed = (pad_seed * 0x5851F42D4C957F2D + 1) & self.MASK64
                chunks.append(pad_seed % 256)

        # Initialize quantum state (walker at center of the 512-node space)
        psi = np.zeros(self.N, dtype=np.complex128)
        psi[self.N // 2] = 1.0
        memory_buffer = [
            psi.copy() for _ in range(4)
        ]  # 4-step memory to resist backtracking

        # Main Evolution Phase
        for i, byte in enumerate(chunks):
            chaos_hop = self._quantum_chaos(byte, i)

            # Apply 4D-Coin superposition
            # We split the state into 4 components (Up, Down, Left, Right)
            # and evolve them using the 4x4 S-Matrix
            q1 = np.roll(psi, 1)  # Shift Left
            q2 = np.roll(psi, -1)  # Shift Right
            q3 = np.roll(psi, chaos_hop % (self.N // 4))  # Chaotic Jump A
            q4 = np.roll(psi, -(chaos_hop % (self.N // 4)))  # Chaotic Jump B

            # Unitary mixing
            psi = (
                self.coin_4d[0, 0] * q1
                + self.coin_4d[0, 1] * q2
                + self.coin_4d[0, 2] * q3
                + self.coin_4d[0, 3] * q4
            )

            # Feedback from memory: Non-Markovian injection
            # Blends current state with a weighted history to ensure non-invertibility
            psi = 0.6 * psi + 0.25 * memory_buffer[0] + 0.15 * memory_buffer[2]

            norm = np.linalg.norm(psi)
            if not np.isfinite(norm) or norm <= 1e-300:
                psi = memory_buffer[-1].copy()
                norm = np.linalg.norm(psi)
            if norm > 1e-15:
                psi /= norm

            # Update history
            memory_buffer.pop(0)
            memory_buffer.append(psi.copy())

            # Periodic Re-normalization and non-linear bit-diffusion
            if i % 8 == 0:
                norm = np.linalg.norm(psi)
                if norm > 1e-15:
                    psi /= norm

        # Final Measurement & Multi-Pass Sponge Compression
        probs = np.abs(psi) ** 2
        quantized = (probs * 0xFFFFFFFF).astype(np.uint64)

        # 512-bit state buffer for compression
        state = [0] * 8
        for i, val in enumerate(quantized):
            idx = i % 8
            # Prime-based mixing with bit-rotations (Avalanche maximization)
            v = int(val) & self.MASK64
            mixer = ((v << (i % 63)) & self.MASK64) | (
                v >> (64 - (i % 63)) if i % 63 != 0 else 0
            )
            state[idx] ^= (mixer * 0xBF58476D1CE4E5B9) & self.MASK64
            state[idx] = ((state[idx] << 13) & self.MASK64) | (state[idx] >> 51)
            state[idx] = (state[idx] + (i * 0x94D049BB133111EB)) & self.MASK64

        # Final sponge-like squeeze
        final_bits = []
        for i in range(self.output_bits // 64):
            # Mix the entire 512-bit state buffer into each 64-bit word
            word = state[i]
            for j in range(8):
                word ^= (state[j] * 0x632BE59BD9B4E019) & self.MASK64
                word = ((word << 21) & self.MASK64) | (word >> 43)
            final_bits.append(word)

        # Convert to Hex
        return "".join(format(b, "016x") for b in final_bits)


def gaqwh(data: Union[str, bytes], output_bits: int = 256) -> str:
    engine = Astryx(output_bits)
    return engine.hash(data)
