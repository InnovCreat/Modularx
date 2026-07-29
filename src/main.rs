// ─────────────────────────────────────────────
// VERITAS HORTUS · main · main.rs
// Architecte  : Isabel Sigouin (InnovCreat)
// Registre    : Runtime
// Fonction    : Agir · Communiquer
// Gravité     : Lien
// Révolution  : interne
// Souveraineté: nulle
// Chaîne      : ─
// Covenant    : Jamais guerre · Jamais cupidité
//               Toujours connaissance · Toujours amour
// Constitution v1.0 — Article IV · Langage v1.4 — Deux Centres
// ─────────────────────────────────────────────

mod archive;
mod gabarit;
mod holographic;
mod interaction;
mod render;
mod sacred_math;

use bevy::prelude::*;
use archive::ArchivePlugin;
use interaction::InteractionPlugin;
use render::RenderPlugin;
use sacred_math::SacredMathPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GHZ 639 CORE — 639 Hz".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((SacredMathPlugin, RenderPlugin, InteractionPlugin, ArchivePlugin))
        .run();
}
