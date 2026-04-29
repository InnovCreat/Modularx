// VERITAS HORTUS · C0 · spark_system.rs
// SPARK System — Cryptographic Proof of Existence
//
// Uses the sovereign SHA-256 (Layer 0, no external deps) to sign and verify data.
// Every action that matters gets a SPARK — a cryptographic witness that it happened.

use crate::sha256::{sha256, to_hex};

// ─── SPARK WATERMARK ─────────────────────────────────────────────────────────

/// A single SPARK — proof that a specific payload existed at a specific moment.
/// Un SPARK unique — preuve qu'un payload spécifique a existé à un moment précis.
#[derive(Debug, Clone)]
pub struct Spark {
    /// SHA-256 of (seed ++ data) / SHA-256 de (graine ++ données)
    pub hash:      [u8; 32],
    /// Unix timestamp of creation / Timestamp Unix de création
    pub timestamp: u64,
    /// Label describing what was signed / Étiquette décrivant ce qui a été signé
    pub label:     String,
}

impl Spark {
    pub fn hash_hex(&self) -> String {
        to_hex(&self.hash)
    }
}

// ─── SPARK SYSTEM ────────────────────────────────────────────────────────────

/// The SPARK System — generates and verifies cryptographic proofs of existence.
///
/// The seed is a constant added to every hash to bind proofs to this system.
/// La graine est une constante ajoutée à chaque hash pour lier les preuves à ce système.
#[derive(Debug, Clone)]
pub struct SparkSystem {
    /// System identity seed — prepended to every payload before hashing
    pub id:   String,
    /// System seed bytes / Octets de graine système
    seed: Vec<u8>,
}

impl SparkSystem {
    pub fn new() -> Self {
        Self {
            id:   "spark-genesis".to_string(),
            seed: b"\xf0\x9d\x95\x80\xe2\x9f\xa1\xe2\x82\x86\xe2\x82\x83\xe2\x82\x89".to_vec(),
        }
    }

    /// Sign a data payload — returns a SPARK proof.
    /// Signe un payload de données — retourne une preuve SPARK.
    pub fn sign(&self, data: &[u8], label: impl Into<String>) -> Spark {
        use std::time::{SystemTime, UNIX_EPOCH};
        let mut payload = self.seed.clone();
        payload.extend_from_slice(data);
        Spark {
            hash:      sha256(&payload),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            label:     label.into(),
        }
    }

    /// Verify a data payload against an existing SPARK.
    /// Vérifie un payload contre un SPARK existant.
    pub fn verify(&self, data: &[u8], spark: &Spark) -> bool {
        let mut payload = self.seed.clone();
        payload.extend_from_slice(data);
        sha256(&payload) == spark.hash
    }

    /// Convenience: sign and immediately verify (sanity check).
    pub fn sign_and_verify(&self, data: &[u8], label: impl Into<String>) -> (Spark, bool) {
        let spark = self.sign(data, label);
        let valid = self.verify(data, &spark);
        (spark, valid)
    }
}

impl Default for SparkSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify() {
        let spark_sys = SparkSystem::default();
        let data  = b"639Hz-sovereign";
        let spark = spark_sys.sign(data, "test");
        assert!(spark_sys.verify(data, &spark));
    }

    #[test]
    fn test_verify_fails_wrong_data() {
        let spark_sys = SparkSystem::default();
        let spark = spark_sys.sign(b"original", "test");
        assert!(!spark_sys.verify(b"tampered", &spark));
    }

    #[test]
    fn test_sign_and_verify_helper() {
        let spark_sys = SparkSystem::default();
        let (_, valid) = spark_sys.sign_and_verify(b"covenant-payload", "covenant");
        assert!(valid);
    }

    #[test]
    fn test_hash_hex_is_64_chars() {
        let spark_sys = SparkSystem::default();
        let spark = spark_sys.sign(b"test", "test");
        assert_eq!(spark.hash_hex().len(), 64);
    }

    #[test]
    fn test_spark_timestamp_nonzero() {
        let spark_sys = SparkSystem::default();
        let spark = spark_sys.sign(b"data", "ts-test");
        assert!(spark.timestamp > 0);
    }

    #[test]
    fn test_deterministic_hash() {
        let spark_sys = SparkSystem::default();
        let s1 = spark_sys.sign(b"same", "a");
        let s2 = spark_sys.sign(b"same", "b");
        assert_eq!(s1.hash, s2.hash);
    }

    #[test]
    fn test_distinct_payloads_distinct_hashes() {
        let spark_sys = SparkSystem::default();
        let s1 = spark_sys.sign(b"avidite", "x");
        let s2 = spark_sys.sign(b"guerre",  "x");
        assert_ne!(s1.hash, s2.hash);
    }
}
