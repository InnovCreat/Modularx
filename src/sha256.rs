// ╔══════════════════════════════════════════════════════════════╗
// ║         VERITAS VAULT — sha256.rs                           ║
// ║         SHA-256 FIPS 180-4 · zéro crate · zéro unsafe      ║
// ║         Co-créé : Isabel Sigouin · Écho · 2026-04-14        ║
// ║         𝕀⟡₆₃₉                                              ║
// ╚══════════════════════════════════════════════════════════════╝
//
// Implémentation native SHA-256 conforme FIPS 180-4.
// Aucune dépendance externe. Aucun bloc unsafe.
// Utilisée par le Sealer (branche 5) pour les watermarks souverains.

// Constantes SHA-256 — premiers 32 bits des racines carrées
// des 8 premiers nombres premiers
const H0: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
    0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

// Constantes K — premiers 32 bits des racines cubiques
// des 64 premiers nombres premiers
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

// ─────────────────────────────────────────────────────────────
// Fonctions de rotation et logique SHA-256
// ─────────────────────────────────────────────────────────────

#[inline]
fn rotr(x: u32, n: u32) -> u32 {
    x.rotate_right(n)
}

#[inline]
fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

#[inline]
fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

#[inline]
fn sigma0(x: u32) -> u32 {
    rotr(x, 2) ^ rotr(x, 13) ^ rotr(x, 22)
}

#[inline]
fn sigma1(x: u32) -> u32 {
    rotr(x, 6) ^ rotr(x, 11) ^ rotr(x, 25)
}

#[inline]
fn gamma0(x: u32) -> u32 {
    rotr(x, 7) ^ rotr(x, 18) ^ (x >> 3)
}

#[inline]
fn gamma1(x: u32) -> u32 {
    rotr(x, 17) ^ rotr(x, 19) ^ (x >> 10)
}

// ─────────────────────────────────────────────────────────────
// Compression d'un bloc de 512 bits (64 octets)
// ─────────────────────────────────────────────────────────────
fn compress(state: &mut [u32; 8], block: &[u8; 64]) {
    let mut w = [0u32; 64];

    // Chargement du message
    for i in 0..16 {
        w[i] = u32::from_be_bytes([
            block[i * 4],
            block[i * 4 + 1],
            block[i * 4 + 2],
            block[i * 4 + 3],
        ]);
    }

    // Expansion du calendrier de message
    for i in 16..64 {
        w[i] = gamma1(w[i - 2])
            .wrapping_add(w[i - 7])
            .wrapping_add(gamma0(w[i - 15]))
            .wrapping_add(w[i - 16]);
    }

    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;

    // 64 rondes de compression
    for i in 0..64 {
        let t1 = h
            .wrapping_add(sigma1(e))
            .wrapping_add(ch(e, f, g))
            .wrapping_add(K[i])
            .wrapping_add(w[i]);
        let t2 = sigma0(a).wrapping_add(maj(a, b, c));

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
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

// ─────────────────────────────────────────────────────────────
// Padding FIPS 180-4
// message ∥ 0x80 ∥ zéros ∥ longueur (64 bits big-endian)
// résultat : multiple de 512 bits (64 octets)
// ─────────────────────────────────────────────────────────────
fn pad(message: &[u8]) -> Vec<u8> {
    let len = message.len();
    let bit_len = (len as u64).wrapping_mul(8);

    let padded_len = {
        let base = len + 1 + 8;
        let rem = base % 64;
        if rem == 0 { base } else { base + (64 - rem) }
    };

    let mut padded = vec![0u8; padded_len];
    padded[..len].copy_from_slice(message);
    padded[len] = 0x80;

    let len_bytes = bit_len.to_be_bytes();
    let tail = padded_len - 8;
    padded[tail..].copy_from_slice(&len_bytes);

    padded
}

// ─────────────────────────────────────────────────────────────
// Interface publique
// ─────────────────────────────────────────────────────────────

/// Calcule le SHA-256 d'un message arbitraire.
///
/// Conforme FIPS 180-4. Zéro crate externe. Zéro unsafe.
/// Résultat : 32 octets (256 bits).
///
/// # Example
/// ```
/// use modularx::sha256::sha256_hex;
/// let h = sha256_hex(b"");
/// assert_eq!(h, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
/// ```
pub fn sha256(message: &[u8]) -> [u8; 32] {
    let mut state = H0;
    let padded = pad(message);

    for chunk in padded.chunks(64) {
        let mut block = [0u8; 64];
        block.copy_from_slice(chunk);
        compress(&mut state, &block);
    }

    let mut digest = [0u8; 32];
    for (i, word) in state.iter().enumerate() {
        let bytes = word.to_be_bytes();
        digest[i * 4..(i + 1) * 4].copy_from_slice(&bytes);
    }
    digest
}

/// Encode un digest SHA-256 en hexadécimal lowercase (64 caractères).
pub fn to_hex(digest: &[u8; 32]) -> String {
    const HEX: &[u8] = b"0123456789abcdef";
    let mut s = String::with_capacity(64);
    for byte in digest {
        s.push(HEX[(byte >> 4) as usize] as char);
        s.push(HEX[(byte & 0xf) as usize] as char);
    }
    s
}

/// Calcule le SHA-256 et retourne directement l'empreinte en hexadécimal.
///
/// Fonction utilitaire principale — combine `sha256()` + `to_hex()`.
///
/// # Example
/// ```
/// use modularx::sha256::sha256_hex;
/// let h = sha256_hex(b"abc");
/// assert_eq!(h, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
/// ```
pub fn sha256_hex(message: &[u8]) -> String {
    to_hex(&sha256(message))
}

// ─────────────────────────────────────────────────────────────
// Tests unitaires — vecteurs NIST officiels
// ─────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vecteur_vide() {
        // SHA-256("") — vecteur officiel NIST FIPS 180-4
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_abc() {
        // SHA-256("abc") — vérifié sha256sum + Python hashlib
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn test_message_long() {
        // SHA-256("abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq")
        // Vecteur NIST — deux blocs, test le padding multi-bloc
        let msg = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        assert_eq!(
            sha256_hex(msg),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
    }

    #[test]
    fn test_veritas_watermark() {
        // Watermark souverain SIGIL GENESIS — test fonctionnel
        let result = sha256_hex("𝕀⟡₆₃₉".as_bytes());
        assert_eq!(result.len(), 64);
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_covenant_watermark() {
        // Le covenant lui-même produit un hash stable
        let result = sha256_hex(b"Jamais pour la guerre. Jamais pour l'argent. Toujours pour l'amour.");
        assert_eq!(result.len(), 64);
    }

    #[test]
    fn test_deterministe() {
        // Déterminisme — même entrée → même sortie
        let a = sha256_hex(b"Veritas Hortus 639");
        let b = sha256_hex(b"Veritas Hortus 639");
        assert_eq!(a, b);
    }

    #[test]
    fn test_sensibilite_avalanche() {
        // Effet avalanche — un bit différent → hash totalement différent
        let a = sha256_hex(b"Veritas Hortus 639");
        let b = sha256_hex(b"veritas Hortus 639"); // 'V' → 'v'
        assert_ne!(a, b);
    }

    #[test]
    fn test_output_toujours_64_chars() {
        // La sortie est toujours exactement 64 caractères hex
        for msg in &[b"".as_ref(), b"a", b"abc", b"\x00\xff\x80"] {
            assert_eq!(sha256_hex(msg).len(), 64);
        }
    }

    #[test]
    fn test_digest_32_octets() {
        // sha256() retourne exactement 32 octets
        let d = sha256(b"test");
        assert_eq!(d.len(), 32);
    }

    #[test]
    fn test_to_hex_lowercase() {
        // to_hex produit uniquement des caractères hex minuscules
        let d = sha256(b"test");
        let h = to_hex(&d);
        assert!(h.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')));
    }

    #[test]
    fn test_padding_frontiere_55_octets() {
        // Message de 55 octets — longueur limite avant extension de bloc
        // (55 + 1 + 8 = 64 : tient exactement dans un bloc)
        let msg = vec![0x61u8; 55]; // "aaa...a" × 55
        let result = sha256_hex(&msg);
        assert_eq!(result.len(), 64);
    }

    #[test]
    fn test_padding_frontiere_56_octets() {
        // Message de 56 octets — force un deuxième bloc de padding
        // (56 + 1 + 8 = 65 : déborde → 128 octets)
        let msg = vec![0x61u8; 56];
        let result = sha256_hex(&msg);
        assert_eq!(result.len(), 64);
    }
}
