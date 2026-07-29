// ─────────────────────────────────────────────
// VERITAS HORTUS · sacred_math · mod.rs
// Architecte  : Isabel Sigouin (InnovCreat)
// Registre    : Mathématique
// Fonction    : Comprendre
// Gravité     : Lien
// Révolution  : interne
// Souveraineté: nulle
// Chaîne      : ─
// Covenant    : Jamais guerre · Jamais cupidité
//               Toujours connaissance · Toujours amour
// Constitution v1.0 — Article IV · Langage v1.4 — Deux Centres
// ─────────────────────────────────────────────

pub mod frequencies;
pub mod geometry;
pub mod platonic;
pub mod sri_yantra;

use bevy::prelude::*;
use frequencies::SacredFrequencies;
use platonic::PlatonicRegistry;

pub struct SacredMathPlugin;

impl Plugin for SacredMathPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SacredFrequencies::default())
            .insert_resource(PlatonicRegistry::default())
            .add_systems(Update, tick_frequencies);
    }
}

fn tick_frequencies(time: Res<Time>, mut freqs: ResMut<SacredFrequencies>) {
    freqs.elapsed += time.delta_secs();
}
