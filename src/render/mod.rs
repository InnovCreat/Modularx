pub mod material;
pub mod modes;
pub mod pulse;

use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::prelude::*;
use material::SacredMaterial;
use modes::RenderMode;
use crate::archive::LivingArchive;
use crate::sacred_math::platonic::{PlatonicRegistry, PlatonicSolid};
use crate::sacred_math::frequencies::SacredFrequencies;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<SacredMaterial>::default())
            .add_plugins(WireframePlugin)
            .insert_resource(RenderMode::default())
            .add_plugins(pulse::PulsationPlugin)
            .add_systems(Startup, setup_scene)
            .add_systems(Update, (
                modes::cycle_render_mode,
                modes::apply_render_mode.after(modes::cycle_render_mode),
                material::update_pulse,
                select_solid,
                swap_solid.after(select_solid),
            ));
    }
}

#[derive(Component)]
pub struct ActiveSolid;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SacredMaterial>>,
    registry: Res<PlatonicRegistry>,
    mode: Res<RenderMode>,
    mut archive: ResMut<LivingArchive>,
    time: Res<Time>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.5, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight { illuminance: 6000.0, shadows_enabled: true, ..default() },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.6, 0.8, 0.0)),
    ));

    commands.spawn((
        DirectionalLight { illuminance: 1500.0, ..default() },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.6, -0.8, 0.0)),
    ));

    spawn_solid(&mut commands, &mut meshes, &mut materials, registry.active, *mode);
    archive.seal(
        time.elapsed_secs_f64(),
        format!("Session started — {} ({} Hz)", registry.active.name(), registry.active.frequency()),
    );
}

fn spawn_solid(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<SacredMaterial>,
    solid: PlatonicSolid,
    mode: RenderMode,
) {
    let mut mat = SacredMaterial::for_solid(solid);
    mat.mode = mode.material_mode();

    let mut entity = commands.spawn((
        Mesh3d(meshes.add(solid.build_mesh())),
        MeshMaterial3d(materials.add(mat)),
        Transform::default(),
        pulse::Pulsating { base_scale: 1.0, amplitude: 0.06 },
        ActiveSolid,
    ));

    if mode.uses_wireframe() {
        entity.insert(Wireframe);
    }
}

fn select_solid(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut registry: ResMut<PlatonicRegistry>,
    mut freqs: ResMut<SacredFrequencies>,
    mut archive: ResMut<LivingArchive>,
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
            let msg = format!("Solid → {} ({} Hz)", solid.name(), solid.frequency());
            info!("{}", msg);
            archive.seal(time.elapsed_secs_f64(), msg);
        }
    }
}

fn swap_solid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SacredMaterial>>,
    registry: Res<PlatonicRegistry>,
    mode: Res<RenderMode>,
    current: Query<Entity, With<ActiveSolid>>,
) {
    if !registry.is_changed() {
        return;
    }
    for entity in &current {
        commands.entity(entity).despawn();
    }
    spawn_solid(&mut commands, &mut meshes, &mut materials, registry.active, *mode);
}
