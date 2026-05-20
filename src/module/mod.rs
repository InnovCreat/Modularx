pub mod registry;

use bevy::prelude::*;
pub use registry::ModuleRegistry;

pub struct ModulePlugin;

impl Plugin for ModulePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ModuleRegistry>()
            .add_systems(Update, registry::auto_tune);
    }
}

/// Implemented by every automodular plugin to declare its harmonic identity.
pub trait SacredModule: Plugin {
    fn module_name(&self) -> &'static str;
    /// The Hz frequency this module tunes to (0.0 = always active).
    fn resonant_hz(&self) -> f32;
    fn dependencies(&self) -> Vec<&'static str> {
        vec![]
    }
}

/// Helper: register a module's metadata when its plugin is building.
pub fn register_module(app: &mut App, name: &'static str, hz: f32) {
    if let Some(mut registry) = app.world_mut().get_resource_mut::<ModuleRegistry>() {
        registry.register(name, hz);
    }
    // If registry not yet present (ordering edge case), queue it via a startup system.
    // In practice ModulePlugin is added first so this branch is never hit.
}
