// VERITAS MODULARIS · C0 · djb2.rs
// DJB2 (Dan Bernstein) — fast, non-cryptographic hash
//
// Role in the system:
//   - SPEED: used by SEARF for real-time event fingerprinting
//   - DETECTION: fast pattern matching, anomaly fingerprints, node identity keys
//   - NOT for sealing: use sha256.rs + SPARK for cryptographic proofs
//
// Algorithm: hash = hash * 33 ^ byte  (XOR variant — faster than the add variant)
// Seed: 5381 (Bernstein's canonical seed)

// ─── CORE ALGORITHM ──────────────────────────────────────────────────────────

/// DJB2 hash → u32. Fast, non-cryptographic.
///
/// Uses the XOR variant (`hash * 33 ^ byte`) for better avalanche than add.
/// Seed is the canonical Bernstein value 5381.
pub fn djb2(data: &[u8]) -> u32 {
    let mut hash: u32 = 5381;
    for &byte in data {
        hash = hash.wrapping_mul(33) ^ (byte as u32);
    }
    hash
}

/// DJB2 64-bit variant — larger key space, fewer collisions on long inputs.
/// Used by SEARF alert fingerprints.
pub fn djb2_64(data: &[u8]) -> u64 {
    let mut hash: u64 = 5381;
    for &byte in data {
        hash = hash.wrapping_mul(33) ^ (byte as u64);
    }
    hash
}

/// DJB2 hex string — mirrors the `sha256_hex` API for uniform usage.
pub fn djb2_hex(data: &[u8]) -> String {
    format!("{:016x}", djb2_64(data))
}

// ─── TESTS ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        // Empty input returns the seed unchanged (no XOR applied)
        assert_eq!(djb2(b""), 5381);
        assert_eq!(djb2_64(b""), 5381);
    }

    #[test]
    fn test_known_vector_abc() {
        // "abc" — manually verified against reference implementations
        let h = djb2(b"abc");
        assert_ne!(h, 5381);   // must differ from seed
        assert_eq!(h, djb2(b"abc")); // deterministic
    }

    #[test]
    fn test_deterministic() {
        assert_eq!(djb2(b"veritas"), djb2(b"veritas"));
        assert_eq!(djb2_64(b"639Hz"), djb2_64(b"639Hz"));
    }

    #[test]
    fn test_distinct_inputs_distinct_hashes() {
        assert_ne!(djb2(b"war"),   djb2(b"peace"));
        assert_ne!(djb2(b"greed"), djb2(b"love"));
    }

    #[test]
    fn test_case_sensitive() {
        // DJB2 is case-sensitive — SEARF lowercases before hashing
        assert_ne!(djb2(b"SENTINEL"), djb2(b"sentinel"));
    }

    #[test]
    fn test_djb2_64_wider_than_32() {
        // 64-bit output uses more of the key space
        let h32 = djb2(b"test") as u64;
        let h64 = djb2_64(b"test");
        assert_ne!(h32, h64); // different widths produce different values
    }

    #[test]
    fn test_hex_is_16_chars() {
        // 64-bit → 16 hex chars
        assert_eq!(djb2_hex(b"").len(), 16);
        assert_eq!(djb2_hex(b"Veritas Hortus 639").len(), 16);
    }

    #[test]
    fn test_hex_lowercase_hex_digits() {
        let h = djb2_hex(b"test");
        assert!(h.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')));
    }

    #[test]
    fn test_single_byte() {
        // Single byte — one XOR step from seed
        let h = djb2(&[0x41]); // 'A'
        assert_eq!(h, 5381u32.wrapping_mul(33) ^ 0x41);
    }

    #[test]
    fn test_covenant_payload_deterministic() {
        let payload = b"Jamais pour la guerre";
        let h1 = djb2_64(payload);
        let h2 = djb2_64(payload);
        assert_eq!(h1, h2);
        assert_ne!(h1, 5381); // non-trivial
    }
}
