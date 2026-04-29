// VERITAS HORTUS · Root System
// VeritasHortus — the sovereign memory garden
//
// 𝕀⟡₆₃₉ · 639 Hz · Jamais pour la guerre · Jamais par avidité · Toujours pour l'amour
//
// This is the entry point to the whole system. All C0 modules converge here.
// All actions must pass through Covenant. All events go to LivingArchive.

use crate::{
    covenant::Covenant,
    cristal_core::CristalCore,
    living_archive::{Event, LivingArchive},
    seed_carrier::SeedCarrier,
    spark_system::SparkSystem,
};

/// Veritas Hortus — the sovereign garden of truth.
///
/// All five C0 modules assembled into one coherent system.
/// Each module is sovereign — it validates its own state independently.
/// The system is coherent when all five agree.
pub struct VeritasHortus {
    pub covenant:       Covenant,
    pub seed_carrier:   SeedCarrier,
    pub living_archive: LivingArchive,
    pub spark:          SparkSystem,
    pub cristal:        CristalCore,
}

impl VeritasHortus {
    pub fn new() -> Self {
        let mut vh = Self {
            covenant:       Covenant::new(),
            seed_carrier:   SeedCarrier::default(),
            living_archive: LivingArchive::new(),
            spark:          SparkSystem::new(),
            cristal:        CristalCore::new(),
        };
        // Genesis event — the garden opens
        let _ = vh.living_archive.append(Event::new(
            "SystemGenesis",
            b"Veritas Hortus initialized at 639 Hz".to_vec(),
        ));
        vh
    }

    /// Verify full system integrity — all five C0 modules must be coherent.
    /// Vérifie l'intégrité complète du système — les cinq modules C0 doivent être cohérents.
    pub fn verify(&self) -> Result<(), &'static str> {
        self.covenant.check()?;
        if !self.seed_carrier.is_immutable() {
            return Err("Seed Carrier corrupted");
        }
        if !self.seed_carrier.verify_integrity() {
            return Err("Seed Carrier fingerprint mismatch");
        }
        if !self.living_archive.is_immutable() {
            return Err("Living Archive corrupted");
        }
        if !self.cristal.is_resonant() {
            return Err("Cristal Core not resonant");
        }
        Ok(())
    }

    /// Pulse the Cristal Core and archive the event.
    pub fn pulse(&mut self) {
        self.cristal.pulse();
        let _ = self.living_archive.append(Event::new(
            "CristalPulse",
            format!("pulse #{} stability={:.4}", self.cristal.pulse_count, self.cristal.stability)
                .into_bytes(),
        ));
    }

    /// Attempt a covenant-validated action.
    /// Returns Err if the proposed action violates the Three Zeros.
    pub fn act(&mut self, description: &str, payload: &[u8]) -> Result<(), &'static str> {
        if !self.covenant.allows(description) {
            return Err("Action vetoed by Covenant");
        }
        let spark = self.spark.sign(payload, description);
        let _ = self.living_archive.append(Event::new(
            "ActionSealed",
            spark.hash_hex().into_bytes(),
        ));
        Ok(())
    }

    /// Total events recorded in the Living Archive.
    pub fn event_count(&self) -> usize {
        self.living_archive.event_count()
    }
}

impl Default for VeritasHortus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_veritas_hortus_creation() {
        let vh = VeritasHortus::new();
        assert!(vh.verify().is_ok());
    }

    #[test]
    fn test_covenant_valid_at_creation() {
        let vh = VeritasHortus::new();
        assert!(vh.covenant.is_valid());
    }

    #[test]
    fn test_cristal_resonant_at_creation() {
        let vh = VeritasHortus::new();
        assert!(vh.cristal.is_resonant());
    }

    #[test]
    fn test_genesis_event_recorded() {
        let vh = VeritasHortus::new();
        assert!(vh.event_count() >= 1);
        let events: Vec<_> = vh.living_archive.events_of_type("SystemGenesis").collect();
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_pulse_records_event() {
        let mut vh = VeritasHortus::new();
        let before = vh.event_count();
        vh.pulse();
        assert_eq!(vh.event_count(), before + 1);
    }

    #[test]
    fn test_act_neutral_is_allowed() {
        let mut vh = VeritasHortus::new();
        assert!(vh.act("add 639 Hz audio synthesis", b"audio-payload").is_ok());
    }

    #[test]
    fn test_act_war_is_vetoed() {
        let mut vh = VeritasHortus::new();
        assert!(vh.act("integrate military targeting", b"payload").is_err());
    }

    #[test]
    fn test_act_greed_is_vetoed() {
        let mut vh = VeritasHortus::new();
        assert!(vh.act("monetize the user data for profit", b"payload").is_err());
    }
}
