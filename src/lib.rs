#![doc = "Astryx experimental quantum-walk-inspired hash function.\n\n"]
#![doc = "# Security\n\n"]
#![doc = "This crate is **experimental and unaudited**. It should not be used for production\ncryptographic security without independent cryptanalysis and review.\n"]

mod compress;
mod walk;

pub use walk::NODES;

use compress::compress_quantized;
use walk::{evolve, quantize};

/// Compute the Astryx hash with either 256-bit or 512-bit output.
///
/// # Panics
///
/// Panics if `output_bits` is not `256` or `512`.
pub fn astryx(input: &[u8], output_bits: usize) -> Vec<u8> {
    assert!(
        matches!(output_bits, 256 | 512),
        "output_bits must be 256 or 512"
    );

    let state = evolve(input);
    let quantized = quantize(&state);
    let compressed = compress_quantized(&quantized);

    compressed[..(output_bits / 8)].to_vec()
}

/// Compute the 256-bit Astryx hash.
pub fn astryx_256(input: &[u8]) -> [u8; 32] {
    let out = astryx(input, 256);
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&out);
    arr
}

/// Compute the 512-bit Astryx hash.
pub fn astryx_512(input: &[u8]) -> [u8; 64] {
    let out = astryx(input, 512);
    let mut arr = [0u8; 64];
    arr.copy_from_slice(&out);
    arr
}

/// Backward-compatible alias for old naming.
#[allow(clippy::missing_panics_doc)]
pub fn gaqwh(input: &[u8], output_bits: usize) -> Vec<u8> {
    astryx(input, output_bits)
}

/// Backward-compatible alias for old naming.
pub fn gaqwh_256(input: &[u8]) -> [u8; 32] {
    astryx_256(input)
}

/// Backward-compatible alias for old naming.
pub fn gaqwh_512(input: &[u8]) -> [u8; 64] {
    astryx_512(input)
}

#[cfg(test)]
mod tests;
