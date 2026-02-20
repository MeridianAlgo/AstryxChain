pub fn compress_quantized(quantized: &[u8; 256]) -> [u8; 64] {
    let mut state = [0u8; 64];

    for (chunk_idx, chunk) in quantized.chunks_exact(32).enumerate() {
        for (j, &b) in chunk.iter().enumerate() {
            let rot = ((chunk_idx + j) & 7) as u32;
            let v = b.rotate_left(rot);
            let idx = (j + chunk_idx * 7) & 63;
            state[idx] ^= v;
            let idx2 = (idx + 13) & 63;
            state[idx2] = state[idx2].wrapping_add(v.rotate_left(1));
            state[(idx + 37) & 63] ^= state[idx2].rotate_left(3);
        }
    }

    for round in 0..12usize {
        let mut next = state;
        for i in 0..64 {
            let a = state[i];
            let b = state[(i + 1) & 63];
            let c = state[(i + 23) & 63];
            let d = state[(i + 41) & 63];
            let m1 = a.wrapping_add(b.rotate_left((round & 7) as u32));
            let m2 = c ^ d.rotate_left(((i + round) & 7) as u32);
            next[i] = m1 ^ m2;
        }
        state = next;
    }

    state
}
