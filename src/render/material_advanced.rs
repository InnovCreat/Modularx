// ─────────────────────────────────────────────
// VERITAS HORTUS · render · material_advanced.rs
// Architecte  : Isabel Sigouin (InnovCreat)
// Registre    : Runtime
// Fonction    : Communiquer
// Gravité     : Forme
// Révolution  : interne
// Souveraineté: nulle
// Chaîne      : concret
// Covenant    : Jamais guerre · Jamais cupidité
//               Toujours connaissance · Toujours amour
// Constitution v1.0 — Article IV · Langage v1.4 — Deux Centres
// ─────────────────────────────────────────────

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use crate::sacred_math::{frequencies::SacredFrequencies, platonic::PlatonicSolid};

/// ✅ ADVANCED Sacred Material with 13 parameters
/// Drives the sacred_pulse_advanced.wgsl shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SacredMaterialAdvanced {
    // Color & appearance
    #[uniform(0)]
    pub base_color: LinearRgba,
    
    // Pulsation
    #[uniform(1)]
    pub pulse_phase: f32,
    #[uniform(2)]
    pub harmonic_hz: f32,
    
    // Fresnel & holographic
    #[uniform(3)]
    pub fresnel_power: f32,
    #[uniform(4)]
    pub fresnel_intensity: f32,
    
    // Glow & emission
    #[uniform(5)]
    pub glow_intensity: f32,
    #[uniform(6)]
    pub emission_strength: f32,
    
    // Distortion & effects
    #[uniform(7)]
    pub distortion_amount: f32,
    #[uniform(8)]
    pub chromatic_aberration: f32,
    
    // Quantum effects
    #[uniform(9)]
    pub superposition_blend: f32,
    #[uniform(10)]
    pub holographic_intensity: f32,
    
    // Crystalline structure
    #[uniform(11)]
    pub crystalline_facets: f32,
    #[uniform(12)]
    pub crystalline_intensity: f32,
}

impl Material for SacredMaterialAdvanced {
    fn fragment_shader() -> ShaderRef {
        "shaders/sacred_pulse_advanced.wgsl".into()
    }
}

impl Default for SacredMaterialAdvanced {
    fn default() -> Self {
        Self {
            base_color: LinearRgba::new(0.4, 0.8, 1.0, 1.0),
            pulse_phase: 0.0,
            harmonic_hz: 639.0,
            fresnel_power: 3.0,
            fresnel_intensity: 1.0,
            glow_intensity: 0.8,
            emission_strength: 0.15,
            distortion_amount: 0.05,
            chromatic_aberration: 0.1,
            superposition_blend: 0.3,
            holographic_intensity: 0.4,
            crystalline_facets: 8.0,
            crystalline_intensity: 0.5,
        }
    }
}

impl SacredMaterialAdvanced {
    /// Create material for a specific Platonic solid
    /// Each solid gets unique parameters reflecting its frequency
    pub fn for_solid(solid: PlatonicSolid) -> Self {
        let (r, g, b, hz, params) = match solid {
            PlatonicSolid::Tetrahedron => {
                // 🔥 Fire - 720 Hz
                (1.0, 0.3, 0.2, 720.0, (4.5, 1.2, 0.12, 0.2, 0.25, 0.6, 0.8))
            }
            PlatonicSolid::Cube => {
                // 🌍 Earth - 1440 Hz
                (0.2, 0.6, 1.0, 1440.0, (3.5, 1.0, 0.08, 0.15, 0.2, 0.5, 0.6))
            }
            PlatonicSolid::Octahedron => {
                // 💨 Air - 2160 Hz
                (0.3, 1.0, 0.5, 2160.0, (3.8, 1.1, 0.10, 0.18, 0.22, 0.7, 0.7))
            }
            PlatonicSolid::Dodecahedron => {
                // ✨ Ether - 3600 Hz
                (0.8, 0.3, 1.0, 3600.0, (4.2, 1.3, 0.14, 0.22, 0.3, 0.8, 0.9))
            }
            PlatonicSolid::Icosahedron => {
                // 💧 Water - 6480 Hz (highest, most harmonically complex)
                (0.2, 0.9, 1.0, 6480.0, (5.0, 1.5, 0.16, 0.25, 0.35, 0.9, 1.0))
            }
        };

        let (fresnel_power, fresnel_int, dist_amt, chroma_abr, superpos, holo_int, cryst_int) =
            params;

        Self {
            base_color: LinearRgba::new(r, g, b, 1.0),
            pulse_phase: 0.0,
            harmonic_hz: hz,
            fresnel_power,
            fresnel_intensity: fresnel_int,
            glow_intensity: 0.8,
            emission_strength: 0.15,
            distortion_amount: dist_amt,
            chromatic_aberration: chroma_abr,
            superposition_blend: superpos,
            holographic_intensity: holo_int,
            crystalline_facets: 6.0,
            crystalline_intensity: cryst_int,
        }
    }

    /// Get human-readable name + element for display
    pub fn name_and_element(solid: PlatonicSolid) -> (&'static str, &'static str, f32) {
        match solid {
            PlatonicSolid::Tetrahedron => ("Tetrahedron", "Fire 🔥", 720.0),
            PlatonicSolid::Cube => ("Cube", "Earth 🌍", 1440.0),
            PlatonicSolid::Octahedron => ("Octahedron", "Air 💨", 2160.0),
            PlatonicSolid::Dodecahedron => ("Dodecahedron", "Ether ✨", 3600.0),
            PlatonicSolid::Icosahedron => ("Icosahedron", "Water 💧", 6480.0),
        }
    }

    /// Update phase from global time
    /// This drives all the pulsation effects
    pub fn set_phase(&mut self, elapsed_secs: f32) {
        self.pulse_phase = elapsed_secs;
    }

    /// Dynamically control holographic intensity based on camera distance
    /// Closer = more holographic artifact
    pub fn set_holographic_intensity_for_distance(&mut self, distance: f32) {
        // Distance 1-40, map to holographic intensity
        self.holographic_intensity = (40.0 - distance) / 40.0 * 0.8; // [0, 0.8]
    }

    /// React to audio input (frequency amplitude)
    /// Makes material respond to music/sound
    pub fn apply_audio_reactivity(&mut self, frequency_amplitude: f32) {
        // Frequency amplitude [0, 1] controls glow and superposition
        self.glow_intensity = 0.5 + frequency_amplitude * 1.0;
        self.superposition_blend = 0.2 + frequency_amplitude * 0.3;
    }
}

/// Update all SacredMaterialAdvanced with current time phase
pub fn update_advanced_pulse(
    time: Res<Time>,
    _freqs: Res<SacredFrequencies>,
    mut materials: ResMut<Assets<SacredMaterialAdvanced>>,
) {
    let elapsed = time.elapsed_secs();

    for (_, mat) in materials.iter_mut() {
        mat.set_phase(elapsed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_frequencies() {
        let tetra = SacredMaterialAdvanced::for_solid(PlatonicSolid::Tetrahedron);
        assert!((tetra.harmonic_hz - 720.0).abs() < 0.1);

        let ico = SacredMaterialAdvanced::for_solid(PlatonicSolid::Icosahedron);
        assert!((ico.harmonic_hz - 6480.0).abs() < 0.1);
    }

    #[test]
    fn test_material_names() {
        let (name, elem, hz) = SacredMaterialAdvanced::name_and_element(PlatonicSolid::Tetrahedron);
        assert_eq!(name, "Tetrahedron");
        assert_eq!(hz, 720.0);
    }

    #[test]
    fn test_holographic_intensity_scaling() {
        let mut mat = SacredMaterialAdvanced::default();
        
        // At distance 1 (very close)
        mat.set_holographic_intensity_for_distance(1.0);
        assert!(mat.holographic_intensity > 0.7);
        
        // At distance 40 (far)
        mat.set_holographic_intensity_for_distance(40.0);
        assert!(mat.holographic_intensity < 0.05);
    }

    #[test]
    fn test_audio_reactivity() {
        let mut mat = SacredMaterialAdvanced::default();
        
        let orig_glow = mat.glow_intensity;
        mat.apply_audio_reactivity(1.0);
        
        assert!(mat.glow_intensity > orig_glow);
        assert!(mat.superposition_blend > 0.2);
    }
}
