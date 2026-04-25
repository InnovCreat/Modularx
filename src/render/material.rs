use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use crate::sacred_math::frequencies::SacredFrequencies;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SacredMaterial {
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(1)]
    pub pulse_phase: f32,
    #[uniform(2)]
    pub fresnel_power: f32,
}

impl Material for SacredMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sacred_pulse.wgsl".into()
    }
}

impl Default for SacredMaterial {
    fn default() -> Self {
        Self {
            base_color:   LinearRgba::new(0.4, 0.8, 1.0, 1.0),
            pulse_phase:  0.0,
            fresnel_power: 3.0,
        }
    }
}

pub fn update_pulse(
    time: Res<Time>,
    freqs: Res<SacredFrequencies>,
    mut materials: ResMut<Assets<SacredMaterial>>,
) {
    let phase = time.elapsed_secs() * freqs.central / 1000.0;
    for (_, mat) in materials.iter_mut() {
        mat.pulse_phase = phase;
    }
}
