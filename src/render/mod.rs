pub mod material;
pub mod modes;
pub mod pulse;

use bevy::prelude::*;
use material::SacredMaterial;
use modes::RenderMode;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<SacredMaterial>::default())
            .insert_resource(RenderMode::default())
            .add_plugins(pulse::PulsationPlugin)
            .add_systems(Startup, setup_scene)
            .add_systems(Update, (modes::cycle_render_mode, material::update_pulse));
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SacredMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Directional light
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
    ));

    // Central icosahedron as first visual anchor
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0).mesh().ico(3).unwrap())),
        MeshMaterial3d(materials.add(SacredMaterial::default())),
        Transform::default(),
        pulse::Pulsating { base_scale: 1.0, amplitude: 0.08 },
    ));
}
