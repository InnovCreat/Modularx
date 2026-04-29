// SIGIL GENESIS v4.2.1 · Layer 0 · sha256.rs
// Pure Rust SHA-256 — FIPS 180-4
// SHA-256 Rust pur — FIPS 180-4
//
// No external crates. No unsafe. Zero dependencies.
// Aucun crate externe. Pas d'unsafe. Zéro dépendances.

// ─── FIPS 180-4 CONSTANTS ────────────────────────────────────────────────────

/// Round constants — first 32 bits of fractional parts of cube roots of first 64 primes.
/// Constantes de rondes — 32 premiers bits des parties fractionnaires des racines cubiques des 64 premiers premiers.
#[rustfmt::skip]
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
    0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
    0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
    0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
    0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// Initial hash values — first 32 bits of fractional parts of square roots of first 8 primes.
/// Valeurs de hachage initiales — 32 premiers bits des parties fractionnaires des racines carrées des 8 premiers premiers.
const H0: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
    0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

// ─── PUBLIC API ───────────────────────────────────────────────────────────────

/// Compute SHA-256 hash of `data`. Returns a 32-byte digest.
/// Calcule le hash SHA-256 de `data`. Retourne un résumé de 32 octets.
///
/// # Examples
///
/// ```
/// let digest = modularx::sha256::sha256(b"");
/// assert_eq!(modularx::sha256::to_hex(&digest),
///   "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
/// ```
///
/// ```
/// let digest = modularx::sha256::sha256(b"abc");
/// assert_eq!(modularx::sha256::to_hex(&digest),
///   "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
/// ```
///
/// ```
/// let digest = modularx::sha256::sha256(b"639");
/// assert_eq!(digest.len(), 32);
/// ```
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut state = H0;
    let padded = pad(data);

    for block in padded.chunks(64) {
        compress(&mut state, block);
    }

    let mut out = [0u8; 32];
    for (i, word) in state.iter().enumerate() {
        out[i * 4..(i + 1) * 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

/// Encode a 32-byte digest as a 64-character lowercase hex string.
/// Encode un résumé de 32 octets en une chaîne hexadécimale minuscule de 64 caractères.
pub fn to_hex(hash: &[u8; 32]) -> String {
    hash.iter().fold(String::with_capacity(64), |mut s, b| {
        use std::fmt::Write;
        let _ = write!(s, "{b:02x}");
        s
    })
}

// ─── PADDING — FIPS 180-4 §5.1.1 ─────────────────────────────────────────────

fn pad(data: &[u8]) -> Vec<u8> {
    let bit_len = (data.len() as u64).wrapping_mul(8);
    let mut padded = data.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0x00);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());
    padded
}

// ─── COMPRESSION — FIPS 180-4 §6.2.2 ────────────────────────────────────────

fn compress(state: &mut [u32; 8], block: &[u8]) {
    let mut w = [0u32; 64];
    for i in 0..16 {
        w[i] = u32::from_be_bytes(block[i * 4..i * 4 + 4].try_into().unwrap());
    }
    for i in 16..64 {
        let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
        let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
        w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
    }

    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;

    for i in 0..64 {
        let s1   = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch   = (e & f) ^ (!e & g);
        let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
        let s0   = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj  = (a & b) ^ (a & c) ^ (b & c);
        let temp2 = s0.wrapping_add(maj);

        h = g; g = f; f = e;
        e = d.wrapping_add(temp1);
        d = c; c = b; b = a;
        a = temp1.wrapping_add(temp2);
    }

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

// ─── TESTS ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(data: &[u8]) -> String {
        to_hex(&sha256(data))
    }

    // ── FIPS 180-4 test vectors ──────────────────────────────────────────────

    #[test]
    fn test_empty() {
        assert_eq!(
            hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_abc() {
        assert_eq!(
            hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn test_longer_message() {
        // FIPS 180-4 §B.1 example 2
        assert_eq!(
            hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
    }

    #[test]
    fn test_one_block_boundary() {
        // 55 bytes — fits in a single block without extra padding block
        let data = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        assert_eq!(data.len(), 55);
        let h = sha256(data);
        assert_ne!(h, [0u8; 32]);
    }

    #[test]
    fn test_56_bytes_crosses_block() {
        // 56 bytes forces an extra padding block
        let data = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        assert_eq!(data.len(), 56);
        let h = sha256(data);
        assert_ne!(h, [0u8; 32]);
    }

    #[test]
    fn test_64_bytes_full_block() {
        let data = [0x61u8; 64];
        let h = sha256(&data);
        assert_ne!(h, [0u8; 32]);
    }

    #[test]
    fn test_single_byte() {
        let h = sha256(b"\x00");
        assert_eq!(
            to_hex(&h),
            "6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d"
        );
    }

    #[test]
    fn test_639_hz_seed() {
        // Sovereign frequency — deterministic, non-zero
        let h = sha256(b"639");
        assert_ne!(h, [0u8; 32]);
        assert_eq!(h.len(), 32);
    }

    #[test]
    fn test_covenant_payload() {
        let payload = b"Jamais pour la guerre \xc2\xb7 Jamais par avidi\xc3\xa9 \xc2\xb7 Toujours pour l'amour";
        let h = sha256(payload);
        assert_eq!(h.len(), 32);
        assert_ne!(h, [0u8; 32]);
    }

    #[test]
    fn test_deterministic() {
        assert_eq!(sha256(b"sigil"), sha256(b"sigil"));
    }

    #[test]
    fn test_distinct_inputs_distinct_hashes() {
        assert_ne!(sha256(b"war"), sha256(b"peace"));
    }

    // ── to_hex ───────────────────────────────────────────────────────────────

    #[test]
    fn test_to_hex_length() {
        let h = sha256(b"test");
        assert_eq!(to_hex(&h).len(), 64);
    }

    #[test]
    fn test_to_hex_lowercase() {
        let h = sha256(b"test");
        let s = to_hex(&h);
        assert!(s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }

    #[test]
    fn test_zeros_hash() {
        let zeros = [0u8; 32];
        assert_eq!(to_hex(&zeros), "0000000000000000000000000000000000000000000000000000000000000000");
    }
}
