pub mod frequencies;
pub mod geometry;
pub mod platonic;
pub mod sri_yantra;

use bevy::prelude::*;
use frequencies::SacredFrequencies;
use platonic::PlatonicRegistry;

pub struct SacredMathPlugin;

impl Plugin for SacredMathPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SacredFrequencies::default())
            .insert_resource(PlatonicRegistry::default());
    }
}
