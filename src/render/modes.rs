use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::*;
use crate::archive::LivingArchive;
use super::{
    ActiveSolid,
    material::{SacredMaterial, MODE_SHADED, MODE_XRAY, MODE_REALISTIC, MODE_SACRED_PULSE},
};

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq)]
pub enum RenderMode {
    Wireframe,
    HiddenLine,
    Shaded,
    XRay,
    Realistic,
    #[default]
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

    pub fn material_mode(self) -> u32 {
        match self {
            Self::Wireframe | Self::HiddenLine => MODE_SHADED,
            Self::Shaded                       => MODE_SHADED,
            Self::XRay                         => MODE_XRAY,
            Self::Realistic                    => MODE_REALISTIC,
            Self::SacredPulse                  => MODE_SACRED_PULSE,
        }
    }

    pub fn uses_wireframe(self) -> bool {
        matches!(self, Self::Wireframe | Self::HiddenLine)
    }
}

pub fn cycle_render_mode(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut mode: ResMut<RenderMode>,
    mut archive: ResMut<LivingArchive>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        *mode = mode.next();
        let msg = format!("Render mode → {:?}", *mode);
        info!("{}", msg);
        archive.seal(time.elapsed_secs_f64(), msg);
    }
}

/// Applies the current RenderMode to the active solid: toggles the Wireframe
/// component and updates the material mode uniform.
pub fn apply_render_mode(
    mode: Res<RenderMode>,
    mut commands: Commands,
    mut materials: ResMut<Assets<SacredMaterial>>,
    solids: Query<(Entity, &MeshMaterial3d<SacredMaterial>), With<ActiveSolid>>,
) {
    if !mode.is_changed() {
        return;
    }
    for (entity, mat_handle) in &solids {
        if mode.uses_wireframe() {
            commands.entity(entity).insert(Wireframe);
        } else {
            commands.entity(entity).remove::<Wireframe>();
        }
        if let Some(mat) = materials.get_mut(&mat_handle.0) {
            mat.mode = mode.material_mode();
        }
    }
}
