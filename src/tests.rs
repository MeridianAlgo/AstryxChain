use crate::{astryx, astryx_256, astryx_512, gaqwh_256};

fn to_hex(data: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(data.len() * 2);
    for &b in data {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

fn hamming_distance_bytes(a: &[u8], b: &[u8]) -> u32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x ^ y).count_ones())
        .sum()
}

#[test]
fn output_sizes_are_correct() {
    assert_eq!(astryx(b"", 256).len(), 32);
    assert_eq!(astryx(b"", 512).len(), 64);
}

#[test]
fn known_vectors_256() {
    let cases = [
        (
            b"".as_slice(),
            "99cb93b0b8f4eccad50790e387add25970409f86b701d9dda59bbce805ef30be",
        ),
        (
            b"a".as_slice(),
            "92bedff67c4405e095e0da95fb3cc63dee94a00e2f863c0693626fc4a25214be",
        ),
        (
            b"Astryx".as_slice(),
            "b0902b30549ac346d16ddf5c6756f6dd5efd8e77640a356172b85c8ce391545b",
        ),
        (
            b"The quick brown fox jumps over the lazy dog".as_slice(),
            "64d9cb5ef006f23356d63183190a69b89eb1446c27c60d3ee09bde16832094b6",
        ),
    ];

    for (msg, expected_hex) in cases {
        let digest = astryx_256(msg);
        assert_eq!(to_hex(&digest), expected_hex);
    }
}

#[test]
fn known_vectors_512() {
    let digest = astryx_512(b"Astryx");
    assert_eq!(
        to_hex(&digest),
        "b0902b30549ac346d16ddf5c6756f6dd5efd8e77640a356172b85c8ce391545b4abe83fe50bd98f5853995b7e3737668ad9ae5777e21c04bf1681c0c7a4cfe15"
    );
}

#[test]
fn avalanche_single_bit_flip() {
    let mut msg = [0u8; 32];
    let base = astryx_256(&msg);
    msg[0] ^= 0b0000_0001;
    let flipped = astryx_256(&msg);
    let dist = hamming_distance_bytes(&base, &flipped);
    assert!(dist > 90 && dist < 166, "distance={dist}");
}

#[test]
fn frequency_sanity() {
    let mut ones = 0u64;
    let mut total = 0u64;
    let mut seed: u64 = 0x1234_5678_9ABC_DEF0;

    for _ in 0..256 {
        let mut msg = [0u8; 64];
        for b in &mut msg {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            *b = (seed >> 56) as u8;
        }
        let digest = astryx_256(&msg);
        for byte in digest {
            ones += byte.count_ones() as u64;
            total += 8;
        }
    }

    let ratio = ones as f64 / total as f64;
    assert!(ratio > 0.45 && ratio < 0.55, "ratio={ratio}");
}

#[test]
fn empty_single_and_long_inputs() {
    let empty = astryx_256(b"");
    let single = astryx_256(&[0x42]);
    let long = astryx_256(&vec![0xAB; 1_000_000]);

    assert_ne!(empty, single);
    assert_ne!(single, long);
    assert_ne!(empty, long);
}

#[test]
fn backward_compat_alias_matches() {
    assert_eq!(astryx_256(b"compat"), gaqwh_256(b"compat"));
}
