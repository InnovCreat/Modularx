// ─────────────────────────────────────────────
// VERITAS HORTUS · render · material.rs
// Architecte  : Isabel Sigouin (InnovCreat)
// Registre    : Runtime
// Fonction    : Communiquer
// Centre grav.: Forme-lourd
// Chaîne      : concret
// Covenant    : Jamais guerre · Jamais cupidité
//               Toujours connaissance · Toujours amour
// Constitution v1.0 — Article IV · Langage v1.3 — §III
// ─────────────────────────────────────────────

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use crate::sacred_math::{frequencies::SacredFrequencies, platonic::PlatonicSolid};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SacredMaterial {
    #[uniform(0)]
    pub base_color:    LinearRgba,
    #[uniform(1)]
    pub pulse_phase:   f32,
    #[uniform(2)]
    pub fresnel_power: f32,
}

impl Material for SacredMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sacred_pulse.wgsl".into()
    }
}

impl Default for SacredMaterial {
    fn default() -> Self {
        Self {
            base_color:    LinearRgba::new(0.4, 0.8, 1.0, 1.0),
            pulse_phase:   0.0,
            fresnel_power: 3.0,
        }
    }
}

impl SacredMaterial {
    /// Each Platonic solid maps to a distinct sacred color palette
    pub fn for_solid(solid: PlatonicSolid) -> Self {
        let (r, g, b) = match solid {
            PlatonicSolid::Tetrahedron  => (1.0, 0.3, 0.2), // fire red   — 720 Hz
            PlatonicSolid::Cube         => (0.2, 0.6, 1.0), // earth blue — 1440 Hz
            PlatonicSolid::Octahedron   => (0.3, 1.0, 0.5), // air green  — 2160 Hz
            PlatonicSolid::Dodecahedron => (0.8, 0.3, 1.0), // ether violet — 3600 Hz
            PlatonicSolid::Icosahedron  => (0.2, 0.9, 1.0), // water cyan — 6480 Hz
        };
        Self {
            base_color:    LinearRgba::new(r, g, b, 1.0),
            pulse_phase:   0.0,
            fresnel_power: 3.0,
        }
    }
}

/// Drive pulse_phase from wall-clock time × central frequency
pub fn update_pulse(
    time: Res<Time>,
    freqs: Res<SacredFrequencies>,
    mut materials: ResMut<Assets<SacredMaterial>>,
) {
    let phase = time.elapsed_secs() * freqs.central / 1000.0;
    for (_, mat) in materials.iter_mut() {
        mat.pulse_phase = phase;
    }
}
