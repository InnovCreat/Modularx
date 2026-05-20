pub mod frequencies;
pub mod geometry;
pub mod platonic;
pub mod sri_yantra;

use bevy::prelude::*;
use frequencies::SacredFrequencies;
use platonic::PlatonicRegistry;
use crate::module::ModuleRegistry;

pub struct SacredMathPlugin;

impl Plugin for SacredMathPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SacredFrequencies::default())
            .insert_resource(PlatonicRegistry::default())
            .add_systems(Update, tick_frequencies);

        if let Some(mut reg) = app.world_mut().get_resource_mut::<ModuleRegistry>() {
            reg.register("SacredMath", 639.0);
        }
    }
}

fn tick_frequencies(time: Res<Time>, mut freqs: ResMut<SacredFrequencies>) {
    freqs.elapsed += time.delta_secs();
}
