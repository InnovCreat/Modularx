// SEED CARRIER — sovereignty.rs
// Sovereignty Seal — final verification layer for every Seed mutation
// Sceau de souveraineté — couche de vérification finale pour toute mutation du Seed
//
// Connects Layer 1 (Seed Carrier) → Layer 0 (sha256.rs, governance.rs, sigil.rs)
// Connecte Layer 1 (Seed Carrier) → Layer 0 (sha256.rs, governance.rs, sigil.rs)

use crate::sha256::sha256;

// ─── SOVEREIGNTY SEAL ─────────────────────────────────────────────────────────

/// The Sovereignty Seal — cryptographic proof that a Seed state is covenant-valid.
/// Le Sceau de Souveraineté — preuve cryptographique qu'un état Seed est conforme au covenant.
///
/// Every mutation of a Seed must pass through the seal before being committed.
/// Toute mutation d'un Seed doit passer par le sceau avant d'être validée.
///
/// The seal connects to:
/// - `sha256.rs`    → hashes the state payload
/// - `governance.rs`→ validates against the Three Zeros veto
/// - `sigil.rs`     → carries the covenant line and ETERNAL_ANCHOR
#[derive(Debug, Clone)]
pub struct SovereigntySeal {
    /// SHA-256 hash of the last sealed state / Hash SHA-256 du dernier état scellé
    pub hash:      [u8; 32],
    /// Guardian who sealed this state / Gardien ayant scellé cet état
    pub guardian:  &'static str,
    /// Unix timestamp of last seal / Timestamp Unix du dernier sceau
    pub sealed_at: u64,
    /// Covenant reference / Référence au covenant
    pub covenant:  &'static str,
    /// Whether the current state is sealed and valid / Si l'état courant est scellé et valide
    pub valid:     bool,
    /// Number of seal operations / Nombre d'opérations de scellement
    pub seal_count: u64,
}

impl Default for SovereigntySeal {
    fn default() -> Self {
        SovereigntySeal {
            hash:       [0u8; 32],
            guardian:   "Isabel Sigouin + Thierry",
            sealed_at:  0,
            covenant:   "Jamais pour la guerre · Jamais par avidité · Toujours pour l'amour",
            valid:      false,
            seal_count: 0,
        }
    }
}

impl SovereigntySeal {
    /// Seal a payload — hashes it and marks the seal as valid.
    /// Scelle un payload — le hache et marque le sceau comme valide.
    ///
    /// The payload should be a canonical serialization of the Seed state.
    /// Le payload doit être une sérialisation canonique de l'état du Seed.
    pub fn seal(&mut self, payload: &[u8]) {
        use std::time::{SystemTime, UNIX_EPOCH};
        self.hash = sha256(payload);
        self.sealed_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.valid = true;
        self.seal_count += 1;
    }

    /// Verify that a payload matches the stored seal hash.
    /// Vérifie qu'un payload correspond au hash scellé stocké.
    pub fn verify(&self, payload: &[u8]) -> bool {
        if !self.valid { return false; }
        sha256(payload) == self.hash
    }

    /// Invalidate the seal — must re-seal after any mutation.
    /// Invalide le sceau — doit re-sceller après toute mutation.
    pub fn invalidate(&mut self) {
        self.valid = false;
    }

    /// Hex string of the current seal hash / Chaîne hex du hash courant
    pub fn hash_hex(&self) -> String {
        crate::sha256::to_hex(&self.hash)
    }

    /// Check seal against Three Zeros — returns false if payload contains veto keywords.
    /// Vérifie le sceau contre les Trois Zéros — retourne false si le payload contient des mots-clés de veto.
    pub fn covenant_check(&self, proposed: &str) -> bool {
        let lower = proposed.to_lowercase();
        const VETO: &[&str] = &[
            "war", "weapon", "military", "kill", "attack",  // zero_war
            "sell", "monetize", "profit", "greedy", "exploit", // zero_greedy
            "bypass", "override", "centralize", "transfer rights", // zero_love
        ];
        !VETO.iter().any(|kw| lower.contains(kw))
    }
}

// ─── SEAL RESULT ─────────────────────────────────────────────────────────────

/// Result of a seal attempt / Résultat d'une tentative de scellement
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SealResult {
    /// Sealed successfully / Scellé avec succès
    Sealed,
    /// Rejected by covenant veto / Rejeté par le veto du covenant
    VetoRejected(&'static str),
    /// Already sealed — no changes detected / Déjà scellé — aucun changement détecté
    AlreadySealed,
}

/// Attempt to seal a payload with covenant validation.
/// Tente de sceller un payload avec validation du covenant.
pub fn try_seal(seal: &mut SovereigntySeal, payload: &[u8], description: &str) -> SealResult {
    if !seal.covenant_check(description) {
        return SealResult::VetoRejected("Three Zeros veto triggered");
    }
    if seal.valid && seal.verify(payload) {
        return SealResult::AlreadySealed;
    }
    seal.seal(payload);
    SealResult::Sealed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seal_and_verify() {
        let mut seal = SovereigntySeal::default();
        let data = b"seed-state-639Hz";
        seal.seal(data);
        assert!(seal.valid);
        assert!(seal.verify(data));
        assert!(!seal.verify(b"different-data"));
    }

    #[test]
    fn test_invalidate() {
        let mut seal = SovereigntySeal::default();
        seal.seal(b"payload");
        seal.invalidate();
        assert!(!seal.valid);
        assert!(!seal.verify(b"payload"));
    }

    #[test]
    fn test_seal_count_increments() {
        let mut seal = SovereigntySeal::default();
        seal.seal(b"a");
        seal.seal(b"b");
        assert_eq!(seal.seal_count, 2);
    }

    #[test]
    fn test_covenant_check_rejects_war() {
        let seal = SovereigntySeal::default();
        assert!(!seal.covenant_check("integrate military surveillance"));
        assert!(!seal.covenant_check("add weapon targeting"));
        assert!(!seal.covenant_check("monetize the system"));
        assert!(!seal.covenant_check("bypass governance"));
    }

    #[test]
    fn test_covenant_check_allows_neutral() {
        let seal = SovereigntySeal::default();
        assert!(seal.covenant_check("add 639 Hz audio synthesis"));
        assert!(seal.covenant_check("improve VR rendering"));
        assert!(seal.covenant_check("update documentation"));
    }

    #[test]
    fn test_try_seal_veto() {
        let mut seal = SovereigntySeal::default();
        let result = try_seal(&mut seal, b"data", "sell the covenant for profit");
        assert_eq!(result, SealResult::VetoRejected("Three Zeros veto triggered"));
        assert!(!seal.valid);
    }

    #[test]
    fn test_try_seal_success() {
        let mut seal = SovereigntySeal::default();
        let result = try_seal(&mut seal, b"state-data", "update audio frequency to 639 Hz");
        assert_eq!(result, SealResult::Sealed);
        assert!(seal.valid);
    }

    #[test]
    fn test_hash_hex_is_64_chars() {
        let mut seal = SovereigntySeal::default();
        seal.seal(b"test");
        assert_eq!(seal.hash_hex().len(), 64);
    }
}
