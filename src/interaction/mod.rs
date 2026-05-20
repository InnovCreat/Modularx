pub mod camera;

use bevy::prelude::*;
use camera::OrbitalCameraPlugin;
use crate::module::ModuleRegistry;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OrbitalCameraPlugin);

        if let Some(mut reg) = app.world_mut().get_resource_mut::<ModuleRegistry>() {
            reg.register("Interaction", 0.0); // 0 Hz = always active
        }
    }
}
