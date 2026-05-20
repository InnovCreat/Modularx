use bevy::prelude::*;
use crate::sacred_math::frequencies::SacredFrequencies;

#[derive(Clone, Debug)]
pub struct ModuleInfo {
    pub name: &'static str,
    /// Hz this module resonates with (0.0 = always active)
    pub resonant_hz: f32,
    pub active: bool,
}

#[derive(Resource, Default)]
pub struct ModuleRegistry {
    pub modules: Vec<ModuleInfo>,
}

impl ModuleRegistry {
    pub fn register(&mut self, name: &'static str, hz: f32) {
        // Avoid double-registration if build() is called twice.
        if !self.modules.iter().any(|m| m.name == name) {
            self.modules.push(ModuleInfo {
                name,
                resonant_hz: hz,
                active: true,
            });
            info!("[ModuleRegistry] registered '{}' @ {:.0} Hz", name, hz);
        }
    }

    pub fn active_names(&self) -> Vec<&'static str> {
        self.modules.iter().filter(|m| m.active).map(|m| m.name).collect()
    }
}

/// System: re-evaluate which modules are active whenever the frequency state changes.
pub fn auto_tune(
    freqs: Res<SacredFrequencies>,
    mut registry: ResMut<ModuleRegistry>,
) {
    if !freqs.is_changed() {
        return;
    }
    let current_hz = freqs.active_solid_hz;
    let central_hz = freqs.central;

    for module in &mut registry.modules {
        module.active = module.resonant_hz == 0.0          // always-on
            || (module.resonant_hz - central_hz).abs() < 1.0  // central (639 Hz)
            || (module.resonant_hz - current_hz).abs() < 1.0; // solid harmonic
    }

    let active: Vec<_> = registry.modules.iter()
        .filter(|m| m.active)
        .map(|m| format!("{}({:.0}Hz)", m.name, m.resonant_hz))
        .collect();
    info!("[AutoTune] active modules: {}", active.join(", "));
}
