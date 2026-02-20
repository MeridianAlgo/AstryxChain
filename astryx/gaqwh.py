from typing import Union

import numpy as np


class Astryx:
    """Astryx GAQWH: Adaptive Quantum Walk Hashing Engine."""

    def __init__(self, output_bits: int = 256):
        if output_bits <= 0 or output_bits % 64 != 0:
            raise ValueError("output_bits must be a positive multiple of 64")
        self.output_bits = output_bits
        self.N = 512
        self.MASK64 = 0xFFFFFFFFFFFFFFFF

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
        x = (byte + step + 1) % 256
        for _ in range(3):
            x = (3.99 * x * (256 - x) / 64) % 256
        return int(x)

    def hash(self, data: Union[str, bytes]) -> str:
        if isinstance(data, str):
            data = data.encode("utf-8")

        chunks = list(data)
        if len(chunks) < 64:
            pad_seed = sum(chunks) if chunks else 0xDEADBEEF
            for _ in range(64 - len(chunks)):
                pad_seed = (pad_seed * 0x5851F42D4C957F2D + 1) & self.MASK64
                chunks.append(pad_seed % 256)

        psi = np.zeros(self.N, dtype=np.complex128)
        psi[self.N // 2] = 1.0
        memory_buffer = [psi.copy() for _ in range(4)]

        for i, byte in enumerate(chunks):
            chaos_hop = self._quantum_chaos(byte, i)

            q1 = np.roll(psi, 1)
            q2 = np.roll(psi, -1)
            q3 = np.roll(psi, chaos_hop % (self.N // 4))
            q4 = np.roll(psi, -(chaos_hop % (self.N // 4)))

            psi = (
                self.coin_4d[0, 0] * q1
                + self.coin_4d[0, 1] * q2
                + self.coin_4d[0, 2] * q3
                + self.coin_4d[0, 3] * q4
            )

            psi = 0.6 * psi + 0.25 * memory_buffer[0] + 0.15 * memory_buffer[2]

            norm = np.linalg.norm(psi)
            if not np.isfinite(norm) or norm <= 1e-300:
                psi = memory_buffer[-1].copy()
                norm = np.linalg.norm(psi)
            if norm > 1e-15:
                psi /= norm

            memory_buffer.pop(0)
            memory_buffer.append(psi.copy())

            if i % 8 == 0:
                norm = np.linalg.norm(psi)
                if norm > 1e-15:
                    psi /= norm

        probs = np.abs(psi) ** 2
        quantized = (probs * 0xFFFFFFFF).astype(np.uint64)

        state = [0] * 8
        for i, val in enumerate(quantized):
            idx = i % 8
            v = int(val) & self.MASK64
            mixer = ((v << (i % 63)) & self.MASK64) | (
                v >> (64 - (i % 63)) if i % 63 != 0 else 0
            )
            state[idx] ^= (mixer * 0xBF58476D1CE4E5B9) & self.MASK64
            state[idx] = ((state[idx] << 13) & self.MASK64) | (state[idx] >> 51)
            state[idx] = (state[idx] + (i * 0x94D049BB133111EB)) & self.MASK64

        final_bits = []
        for i in range(self.output_bits // 64):
            word = state[i]
            for j in range(8):
                word ^= (state[j] * 0x632BE59BD9B4E019) & self.MASK64
                word = ((word << 21) & self.MASK64) | (word >> 43)
            final_bits.append(word)

        return "".join(format(b, "016x") for b in final_bits)


def gaqwh(data: Union[str, bytes], output_bits: int = 256) -> str:
    engine = Astryx(output_bits)
    return engine.hash(data)
