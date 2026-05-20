pub mod material;
pub mod modes;
pub mod pulse;

use bevy::prelude::*;
use material::SacredMaterial;
use modes::RenderMode;
use crate::sacred_math::platonic::{PlatonicRegistry, PlatonicSolid};
use crate::sacred_math::frequencies::SacredFrequencies;
use crate::module::ModuleRegistry;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<SacredMaterial>::default())
            .insert_resource(RenderMode::default())
            .add_plugins(pulse::PulsationPlugin)
            .add_systems(Startup, setup_scene)
            .add_systems(Update, (
                modes::cycle_render_mode,
                material::update_pulse,
                select_solid,
                swap_solid,
            ));

        if let Some(mut reg) = app.world_mut().get_resource_mut::<ModuleRegistry>() {
            // Render module mirrors the active solid's harmonic — 720 Hz (Tetrahedron default)
            reg.register("Render", 720.0);
        }
    }
}

/// Marker for the active Platonic solid entity
#[derive(Component)]
pub struct ActiveSolid;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SacredMaterial>>,
    registry: Res<PlatonicRegistry>,
) {
    // Camera — starts at a good orbital distance
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.5, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Key light
    commands.spawn((
        DirectionalLight { illuminance: 6000.0, shadows_enabled: true, ..default() },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.6, 0.8, 0.0)),
    ));

    // Fill light (dim, opposite side)
    commands.spawn((
        DirectionalLight { illuminance: 1500.0, ..default() },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.6, -0.8, 0.0)),
    ));

    spawn_solid(&mut commands, &mut meshes, &mut materials, registry.active);
}

fn spawn_solid(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<SacredMaterial>,
    solid: PlatonicSolid,
) {
    commands.spawn((
        Mesh3d(meshes.add(solid.build_mesh())),
        MeshMaterial3d(materials.add(SacredMaterial::for_solid(solid))),
        Transform::default(),
        pulse::Pulsating { base_scale: 1.0, amplitude: 0.06 },
        ActiveSolid,
    ));
}

/// Keys 1-5 select the Platonic solid; updates registry frequency too
fn select_solid(
    keys: Res<ButtonInput<KeyCode>>,
    mut registry: ResMut<PlatonicRegistry>,
    mut freqs: ResMut<SacredFrequencies>,
) {
    let mapping = [
        (KeyCode::Digit1, PlatonicSolid::Tetrahedron),
        (KeyCode::Digit2, PlatonicSolid::Cube),
        (KeyCode::Digit3, PlatonicSolid::Octahedron),
        (KeyCode::Digit4, PlatonicSolid::Dodecahedron),
        (KeyCode::Digit5, PlatonicSolid::Icosahedron),
    ];
    for (key, solid) in mapping {
        if keys.just_pressed(key) && registry.active != solid {
            registry.active = solid;
            freqs.active_solid_hz = solid.frequency();
            info!("Solide → {} ({} Hz)", solid.name(), solid.frequency());
        }
    }
}

/// Despawn old solid and spawn the new one when registry changes
fn swap_solid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SacredMaterial>>,
    registry: Res<PlatonicRegistry>,
    current: Query<Entity, With<ActiveSolid>>,
) {
    if !registry.is_changed() { return; }

    // Despawn all current active solids
    for entity in &current {
        commands.entity(entity).despawn();
    }

    spawn_solid(&mut commands, &mut meshes, &mut materials, registry.active);
}
