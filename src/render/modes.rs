// ─────────────────────────────────────────────
// VERITAS HORTUS · render · modes.rs
// Architecte  : Isabel Sigouin (InnovCreat)
// Registre    : Runtime
// Fonction    : Décider · Communiquer
// Gravité     : Temps
// Révolution  : externe:input utilisateur
// Souveraineté: grande
// Chaîne      : trigger
// Covenant    : Jamais guerre · Jamais cupidité
//               Toujours connaissance · Toujours amour
// Constitution v1.0 — Article IV · Langage v1.4 — Deux Centres
// ─────────────────────────────────────────────

use bevy::prelude::*;

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq)]
pub enum RenderMode {
    Wireframe,
    HiddenLine,
    #[default]
    Shaded,
    XRay,
    Realistic,
    SacredPulse,
}

impl RenderMode {
    pub fn next(self) -> Self {
        match self {
            Self::Wireframe   => Self::HiddenLine,
            Self::HiddenLine  => Self::Shaded,
            Self::Shaded      => Self::XRay,
            Self::XRay        => Self::Realistic,
            Self::Realistic   => Self::SacredPulse,
            Self::SacredPulse => Self::Wireframe,
        }
    }
}

/// Press R to cycle through the 6 render modes
pub fn cycle_render_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut mode: ResMut<RenderMode>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        *mode = mode.next();
        info!("Render mode → {:?}", *mode);
    }
}
