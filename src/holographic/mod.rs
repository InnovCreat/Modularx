use bevy::prelude::*;

/// Holographic & VR layer (branche en développement)
/// - Shaders: Fresnel + Scanlines + Aberration chromatique
/// - OpenXR: rendu stéréo + head tracking
/// - Audio binaural synchronisé à 639 Hz
pub struct HolographicPlugin;

impl Plugin for HolographicPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: bevy_oxr / bevy_openxr for stereo rendering
        // TODO: binaural audio crate synced to SacredFrequencies
    }
}
