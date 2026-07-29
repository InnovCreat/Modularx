// ─────────────────────────────────────────────
// VERITAS HORTUS · interaction · camera.rs
// Architecte  : Isabel Sigouin (InnovCreat)
// Registre    : Runtime
// Fonction    : Observer · Agir
// Gravité     : Temps
// Révolution  : externe:input utilisateur
// Souveraineté: grande
// Chaîne      : trigger
// Covenant    : Jamais guerre · Jamais cupidité
//               Toujours connaissance · Toujours amour
// Constitution v1.0 — Article IV · Langage v1.4 — Deux Centres
// ─────────────────────────────────────────────

use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};

pub struct OrbitalCameraPlugin;

impl Plugin for OrbitalCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OrbitalCamera::default())
            .add_systems(Update, (orbit_left_drag, zoom_scroll, pan_right_drag));
    }
}

#[derive(Resource)]
pub struct OrbitalCamera {
    pub yaw:      f32,
    pub pitch:    f32,
    pub distance: f32,
    pub target:   Vec3,
}

impl Default for OrbitalCamera {
    fn default() -> Self {
        Self { yaw: 0.0, pitch: 0.3, distance: 8.0, target: Vec3::ZERO }
    }
}

const PITCH_MIN: f32 = -1.4;
const PITCH_MAX: f32 =  1.4;
const DIST_MIN:  f32 =  1.0;
const DIST_MAX:  f32 = 40.0;

// Left drag → rotate
fn orbit_left_drag(
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion: EventReader<MouseMotion>,
    mut cam: ResMut<OrbitalCamera>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    if !buttons.pressed(MouseButton::Left) { motion.clear(); return; }
    let delta: Vec2 = motion.read().map(|e| e.delta).sum();
    cam.yaw   -= delta.x * 0.005;
    cam.pitch  = (cam.pitch - delta.y * 0.005).clamp(PITCH_MIN, PITCH_MAX);
    apply_camera(&cam, &mut query);
}

// Scroll wheel → zoom
fn zoom_scroll(
    mut wheel: EventReader<MouseWheel>,
    mut cam: ResMut<OrbitalCamera>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    for ev in wheel.read() {
        let delta = match ev.unit {
            MouseScrollUnit::Line  => ev.y * 0.5,
            MouseScrollUnit::Pixel => ev.y * 0.01,
        };
        cam.distance = (cam.distance - delta).clamp(DIST_MIN, DIST_MAX);
    }
    apply_camera(&cam, &mut query);
}

// Right drag → pan
fn pan_right_drag(
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion: EventReader<MouseMotion>,
    mut cam: ResMut<OrbitalCamera>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    if !buttons.pressed(MouseButton::Right) { motion.clear(); return; }
    let delta: Vec2 = motion.read().map(|e| e.delta).sum();
    cam.target += Vec3::new(-delta.x, delta.y, 0.0) * 0.01;
    apply_camera(&cam, &mut query);
}

fn apply_camera(cam: &OrbitalCamera, query: &mut Query<&mut Transform, With<Camera3d>>) {
    let rot    = Quat::from_euler(EulerRot::YXZ, cam.yaw, cam.pitch, 0.0);
    let offset = rot * Vec3::new(0.0, 0.0, cam.distance);
    for mut t in query.iter_mut() {
        t.translation = cam.target + offset;
        t.look_at(cam.target, Vec3::Y);
    }
}
