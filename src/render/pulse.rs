use bevy::prelude::*;
use crate::sacred_math::frequencies::SacredFrequencies;

pub struct PulsationPlugin;

impl Plugin for PulsationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (apply_global_pulse, apply_rotation));
    }
}

/// Entities with this component breathe (scale) at 639 Hz
#[derive(Component)]
pub struct Pulsating {
    pub base_scale: f32,
    pub amplitude:  f32,
}

/// Entities with this component rotate continuously around the given axis
#[derive(Component)]
pub struct RotatingBody {
    pub axis:         Vec3,
    pub speed_rad_s:  f32,
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

fn apply_rotation(
    time: Res<Time>,
    mut query: Query<(&RotatingBody, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (r, mut t) in &mut query {
        if let Ok(dir) = Dir3::new(r.axis) {
            t.rotate_axis(dir, r.speed_rad_s * dt);
        }
    }
}
