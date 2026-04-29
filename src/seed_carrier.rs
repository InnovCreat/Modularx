// VERITAS HORTUS · C0 · seed_carrier.rs
// Seed Carrier — immutable identity pattern traversing the entire system
//
// Every module carries a Seed. The Seed is immutable at the identity level:
// its id and identity bytes never change. State can evolve, identity cannot.

use crate::sha256::{sha256, to_hex};

/// The Seed Carrier — core identity and determinism pattern.
///
/// This is the C0 root carrier. The full Seed type in src/seed/types.rs builds
/// on this with spatial, temporal, and scaling fields. This struct is the
/// irreducible minimum: id + identity + immutability.
#[derive(Debug, Clone)]
pub struct SeedCarrier {
    /// Unique module identifier / Identifiant unique du module
    pub id:        String,
    /// Immutable identity bytes — the pattern that defines this entity
    /// Octets d'identité immuables — le pattern qui définit cette entité
    pub identity:  Vec<u8>,
    /// Immutability lock — if false, the carrier is corrupted
    pub immutable: bool,
    /// SHA-256 fingerprint of the identity bytes / Empreinte SHA-256 des octets d'identité
    fingerprint: [u8; 32],
}

impl SeedCarrier {
    pub fn new(id: impl Into<String>, identity: Vec<u8>) -> Self {
        let fingerprint = sha256(&identity);
        Self {
            id: id.into(),
            identity,
            immutable: true,
            fingerprint,
        }
    }

    pub fn is_immutable(&self) -> bool {
        self.immutable
    }

    /// Verify that the identity bytes still match the original fingerprint.
    /// Vérifie que les octets d'identité correspondent toujours à l'empreinte originale.
    pub fn verify_integrity(&self) -> bool {
        !self.identity.is_empty()
            && self.immutable
            && sha256(&self.identity) == self.fingerprint
    }

    /// Hex fingerprint of the identity / Empreinte hexadécimale de l'identité.
    pub fn fingerprint_hex(&self) -> String {
        to_hex(&self.fingerprint)
    }
}

impl Default for SeedCarrier {
    fn default() -> Self {
        // Identity: the system sigil bytes (𝕀⟡₆₃₉ in UTF-8)
        Self::new(
            "seed-carrier-genesis",
            "𝕀⟡₆₃₉".as_bytes().to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_carrier_immutable_by_default() {
        let sc = SeedCarrier::default();
        assert!(sc.is_immutable());
    }

    #[test]
    fn test_seed_carrier_integrity() {
        let sc = SeedCarrier::default();
        assert!(sc.verify_integrity());
    }

    #[test]
    fn test_fingerprint_hex_is_64_chars() {
        let sc = SeedCarrier::default();
        assert_eq!(sc.fingerprint_hex().len(), 64);
    }

    #[test]
    fn test_tampered_identity_fails_integrity() {
        let mut sc = SeedCarrier::default();
        sc.identity = b"tampered".to_vec();
        assert!(!sc.verify_integrity());
    }

    #[test]
    fn test_custom_seed_carrier() {
        let sc = SeedCarrier::new("nucleus-001", b"nucleus-identity".to_vec());
        assert_eq!(sc.id, "nucleus-001");
        assert!(sc.verify_integrity());
    }

    #[test]
    fn test_fingerprint_deterministic() {
        let sc1 = SeedCarrier::default();
        let sc2 = SeedCarrier::default();
        assert_eq!(sc1.fingerprint_hex(), sc2.fingerprint_hex());
    }
}
