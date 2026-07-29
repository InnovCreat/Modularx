// ─────────────────────────────────────────────
// VERITAS HORTUS · sacred_math · frequencies.rs
// Architecte  : Isabel Sigouin (InnovCreat)
// Registre    : Mathématique
// Fonction    : Observer · Mémoriser
// Centre grav.: Forme-lourd
// Chaîne      : ─
// Covenant    : Jamais guerre · Jamais cupidité
//               Toujours connaissance · Toujours amour
// Constitution v1.0 — Article IV · Langage v1.3 — §III
// ─────────────────────────────────────────────

use bevy::prelude::*;

pub const CENTRAL_HZ: f32 = 639.0;

pub const SOLID_FREQUENCIES: [(f32, &str); 5] = [
    (720.0,  "Tetrahedron"),
    (1440.0, "Cube"),
    (2160.0, "Octahedron"),
    (3600.0, "Dodecahedron"),
    (6480.0, "Icosahedron"),
];

#[derive(Resource)]
pub struct SacredFrequencies {
    pub central: f32,
    pub active_solid_hz: f32,
    pub elapsed: f32,
}

impl Default for SacredFrequencies {
    fn default() -> Self {
        Self {
            central: CENTRAL_HZ,
            active_solid_hz: SOLID_FREQUENCIES[0].0,
            elapsed: 0.0,
        }
    }
}

impl SacredFrequencies {
    /// Composite pulsation amplitude: 639 Hz base + active solid harmonic
    pub fn pulse(&self, t: f32) -> f32 {
        use std::f32::consts::TAU;
        let base     = (TAU * self.central      * t).sin();
        let harmonic = (TAU * self.active_solid_hz * t).sin() * 0.3;
        (base + harmonic).clamp(-1.0, 1.0)
    }
}
