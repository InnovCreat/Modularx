mod archive;
mod holographic;
mod interaction;
mod render;
mod sacred_math;

use bevy::prelude::*;
use bevy::pbr::wireframe::WireframePlugin;
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
        .add_plugins(WireframePlugin)
        .add_plugins((SacredMathPlugin, RenderPlugin, InteractionPlugin, ArchivePlugin))
        .run();
}
