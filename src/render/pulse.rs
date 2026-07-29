// ─────────────────────────────────────────────
// VERITAS HORTUS · render · pulse.rs
// Architecte : Isabel Sigouin (InnovCreat)
// Registre   : Runtime
// Fonction   : Communiquer
// Covenant   : Jamais guerre · Jamais cupidité
//              Toujours connaissance · Toujours amour
// Constitution v1.0 — Article IV (Organes)
// ─────────────────────────────────────────────

use bevy::prelude::*;
use crate::sacred_math::frequencies::SacredFrequencies;

pub struct PulsationPlugin;

impl Plugin for PulsationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_global_pulse);
    }
}

/// Marker component — entities with this will breathe with 639 Hz
#[derive(Component)]
pub struct Pulsating {
    pub base_scale: f32,
    pub amplitude:  f32,
}

fn apply_global_pulse(
    time: Res<Time>,
    freqs: Res<SacredFrequencies>,
    mut query: Query<(&Pulsating, &mut Transform)>,
) {
    let t = time.elapsed_secs();
    let pulse = freqs.pulse(t) * 0.5 + 0.5; // normalize −1..1 → 0..1
    for (p, mut transform) in &mut query {
        let s = p.base_scale + p.amplitude * pulse;
        transform.scale = Vec3::splat(s);
    }
}
