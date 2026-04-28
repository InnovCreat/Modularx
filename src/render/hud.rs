use bevy::prelude::*;
use crate::sacred_math::platonic::PlatonicRegistry;
use super::modes::RenderMode;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud)
            .add_systems(Update, refresh_hud);
    }
}

#[derive(Component)]
struct HudText;

fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        Text::new(""),
        TextFont   { font_size: 15.0, ..default() },
        TextColor(Color::srgba(0.85, 0.92, 1.0, 0.90)),
        Node {
            position_type: PositionType::Absolute,
            top:  Val::Px(12.0),
            left: Val::Px(14.0),
            ..default()
        },
        HudText,
    ));
}

fn refresh_hud(
    registry: Res<PlatonicRegistry>,
    mode:     Res<RenderMode>,
    mut text: Query<&mut Text, With<HudText>>,
) {
    if !registry.is_changed() && !mode.is_changed() { return; }
    let solid = registry.active;
    for mut t in &mut text {
        t.0 = format!(
            "{} — {} Hz\nMode: {:?}\n\n[1-5] Solid  [R] Cycle mode\n[LMB] Rotate  [Scroll] Zoom  [RMB] Pan",
            solid.name(),
            solid.frequency() as u32,
            *mode,
        );
    }
}
