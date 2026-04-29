// SEED CARRIER — messages.rs
// Message enum — all possible inputs to the Seed action dispatcher
// Enum Message — toutes les entrées possibles du dispatcher d'actions du Seed

use super::types::SeedData;

// ─── MESSAGE ENUM ─────────────────────────────────────────────────────────────

/// All messages a Seed module can receive and process.
/// Tous les messages qu'un module Seed peut recevoir et traiter.
///
/// Messages trigger the action sequence in `actions.rs`.
/// Les messages déclenchent la séquence d'actions dans `actions.rs`.
///
/// Action sequence / Séquence d'actions :
/// ```text
/// Init → Generate → Update* → (Move | AdvanceTime | Interact)* → Seal → Reset?
/// ```
#[derive(Debug, Clone)]
pub enum Message {
    // ── Lifecycle / Cycle de vie ─────────────────────────────────────────────
    /// Initialize the seed — set up config, validate covenant
    /// Initialise le seed — configure, valide le covenant
    Init,

    /// Generate initial state from config
    /// Génère l'état initial depuis la configuration
    Generate,

    /// Update with a data payload / Met à jour avec un payload de données
    Update(SeedData),

    /// Reset to default state (keeps config, clears state)
    /// Remet à l'état par défaut (conserve la config, efface l'état)
    Reset,

    // ── Spatial / Spatial ────────────────────────────────────────────────────
    /// Move to a new position in VR space
    /// Déplace vers une nouvelle position dans l'espace VR
    Move { x: f64, y: f64, z: f64 },

    /// Move along orbital path by delta-theta
    /// Déplace le long du chemin orbital par delta-theta
    Orbit { delta_theta: f64 },

    // ── Temporal / Temporel ──────────────────────────────────────────────────
    /// Advance the timeline and record an event
    /// Avance la timeline et enregistre un événement
    AdvanceTime { label: String },

    // ── Scale / Échelle ──────────────────────────────────────────────────────
    /// Transition to a different operating scale
    /// Passe à une échelle d'opération différente
    Rescale { to: crate::seed::scaling::Scale, reason: String },

    // ── Interaction / Interaction ────────────────────────────────────────────
    /// Interact with another module (signal send)
    /// Interagit avec un autre module (envoi de signal)
    Interact { target: String, signal: String },

    // ── Sovereignty / Souveraineté ───────────────────────────────────────────
    /// Seal the current state with the Sovereignty Seal
    /// Scelle l'état courant avec le Sceau de Souveraineté
    Seal { description: String },

    /// Verify the current state against the seal
    /// Vérifie l'état courant contre le sceau
    Verify,
}

impl Message {
    /// Human-readable message label / Étiquette lisible du message
    pub fn label(&self) -> &'static str {
        match self {
            Message::Init             => "Init",
            Message::Generate         => "Generate",
            Message::Update(_)        => "Update",
            Message::Reset            => "Reset",
            Message::Move { .. }      => "Move",
            Message::Orbit { .. }     => "Orbit",
            Message::AdvanceTime { .. }=> "AdvanceTime",
            Message::Rescale { .. }   => "Rescale",
            Message::Interact { .. }  => "Interact",
            Message::Seal { .. }      => "Seal",
            Message::Verify           => "Verify",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_labels() {
        assert_eq!(Message::Init.label(), "Init");
        assert_eq!(Message::Generate.label(), "Generate");
        assert_eq!(Message::Reset.label(), "Reset");
        assert_eq!(Message::Verify.label(), "Verify");
    }

    #[test]
    fn test_update_message_carries_data() {
        let data = SeedData::new("freq", "u32", "639", true);
        let msg = Message::Update(data.clone());
        if let Message::Update(d) = msg {
            assert_eq!(d.name, "freq");
            assert_eq!(d.value, "639");
        } else {
            panic!("Expected Update message");
        }
    }

    #[test]
    fn test_move_message_carries_coords() {
        let msg = Message::Move { x: 1.0, y: 2.0, z: 3.0 };
        if let Message::Move { x, y, z } = msg {
            assert!((x - 1.0).abs() < f64::EPSILON);
            assert!((y - 2.0).abs() < f64::EPSILON);
            assert!((z - 3.0).abs() < f64::EPSILON);
        } else {
            panic!("Expected Move message");
        }
    }

    #[test]
    fn test_seal_message_carries_description() {
        let msg = Message::Seal { description: "update 639 Hz config".to_string() };
        if let Message::Seal { description } = msg {
            assert!(description.contains("639"));
        } else {
            panic!("Expected Seal message");
        }
    }
}
