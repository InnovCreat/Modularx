pub mod camera;

use bevy::prelude::*;
use camera::OrbitalCameraPlugin;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OrbitalCameraPlugin);
    }
}
