// ─────────────────────────────────────────────
// VERITAS HORTUS · archive · mod.rs
// Architecte  : Isabel Sigouin (InnovCreat)
// Registre    : Événements
// Fonction    : Mémoriser · Observer
// Centre grav.: Temps-lourd
// Chaîne      : ─
// Covenant    : Jamais guerre · Jamais cupidité
//               Toujours connaissance · Toujours amour
// Constitution v1.0 — Article IV · Langage v1.3 — §III
// ─────────────────────────────────────────────

use bevy::prelude::*;

/// SPARF Ring — real-time geometric harmony surveillance
/// Arweave Seal — permanent on-chain condensation of the engine
#[derive(Resource, Default)]
pub struct LivingArchive {
    pub entries: Vec<ArchiveEntry>,
}

#[derive(Debug, Clone)]
pub struct ArchiveEntry {
    pub timestamp: f64,
    pub message:   String,
}

impl LivingArchive {
    pub fn seal(&mut self, time: f64, msg: impl Into<String>) {
        self.entries.push(ArchiveEntry { timestamp: time, message: msg.into() });
        // TODO: push entry to Arweave via HTTP when enabled
    }
}

pub struct ArchivePlugin;

impl Plugin for ArchivePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LivingArchive::default());
    }
}
