import numpy as np
import hashlib
from typing import Union

class Astryx:
    """
    GAQWH: Grok's Adaptive Quantum Walk Hash.
    A quantum-inspired hashing algorithm for blockchain, designed to be
    resistant to Grover's algorithm by using a non-linear, adaptive
    quantum walk on a Hanoi-inspired network.
    """
    
    def __init__(self, output_bits: int = 256):
        self.output_bits = output_bits
        self.N = 256  # Size of the walker space (nodes in the Hanoi-like graph)
        self.q = 2**23 - 2**13 + 1  # Lattice-inspired modulus (NTRU-like)
        
        # Precompute the Hadamard coin
        self.hadamard = np.array([
            [1/np.sqrt(2), 1/np.sqrt(2)], 
            [1/np.sqrt(2), -1/np.sqrt(2)]
        ], dtype=np.complex128)

    def _lively_coin(self, h: int) -> np.ndarray:
        """Message-driven adaptive coin."""
        angle = (h * np.pi) / 4
        return np.array([
            [np.cos(angle), np.sin(angle)],
            [np.sin(angle), -np.cos(angle)]
        ], dtype=np.complex128)

    def hash(self, data: Union[str, bytes]) -> str:
        if isinstance(data, str):
            data = data.encode('utf-8')
        
        # Preprocess: convert to bytes and ensure minimum entropy
        chunks = list(data)
        if len(chunks) < 32:
            # Padding to ensure enough steps for diffusion
            chunks += [i ^ (len(chunks) + 1) for i in range(32 - len(chunks))]
            
        # Initialize quantum state
        psi = np.zeros(self.N, dtype=np.complex128)
        psi[0] = 1.0
        prior_psi = psi.copy()
        
        # Evolution loop
        for i, byte in enumerate(chunks):
            h = byte % 4
            coin = self.hadamard if i % 2 == 0 else self._lively_coin(h)
            
            # Unitary evolution step
            psi_left = coin[0, 0] * psi + coin[0, 1] * np.roll(psi, 1)
            psi_right = coin[1, 0] * psi + coin[1, 1] * np.roll(psi, -1)
            psi = psi_left + psi_right
            
            # Liveliness hop (non-local mixing)
            if h > 0:
                psi = np.roll(psi, -h * (i + 1))  # Index-dependent hop
            
            # Memory blend
            if i % 2 == 0:
                psi = 0.7 * psi + 0.3 * prior_psi
            prior_psi = psi.copy()
            
            # Renormalize
            norm = np.linalg.norm(psi)
            if norm > 1e-10:
                psi /= norm
        
        # Measurement & Multi-pass Compression
        probs = np.abs(psi)**2
        quantized = (probs * 1024).astype(np.uint64)
        
        # Final diffusion stage: bitwise mixing of the state probabilities
        res = 0
        MASK64 = 0xFFFFFFFFFFFFFFFF
        for i, val in enumerate(quantized):
            # Mix the position and the value using a prime-based rotation
            v = int(val) & MASK64
            mixer = ((v << (i % 64)) & MASK64) | (v >> (64 - (i % 64)) if i % 64 != 0 else 0)
            res ^= ((mixer * 0xBF58476D1CE4E5B9) & MASK64) ^ ((i * 0x94D049BB133111EB) & MASK64)
            res = ((res << 13) & MASK64) | (res >> 51)
            
        # Format to desired output bits
        final_hash = 0
        for i in range(4):
            res = ((res * 0x632BE59BD9B4E019) & MASK64) ^ 0x9E3779B97F4A7C15
            final_hash = (final_hash << 64) | (res & MASK64)
            
        return hex(final_hash)[2:].zfill(self.output_bits // 4)

def gaqwh(data: Union[str, bytes], output_bits: int = 256) -> str:
    engine = Astryx(output_bits)
    return engine.hash(data)
