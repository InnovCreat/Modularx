// ─────────────────────────────────────────────
// VERITAS HORTUS · interaction · mod.rs
// Architecte  : Isabel Sigouin (InnovCreat)
// Registre    : Runtime
// Fonction    : Observer · Agir
// Centre grav.: Lien-lourd
// Chaîne      : ─
// Covenant    : Jamais guerre · Jamais cupidité
//               Toujours connaissance · Toujours amour
// Constitution v1.0 — Article IV · Langage v1.3 — §III
// ─────────────────────────────────────────────

pub mod camera;

use bevy::prelude::*;
use camera::OrbitalCameraPlugin;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OrbitalCameraPlugin);
    }
}
