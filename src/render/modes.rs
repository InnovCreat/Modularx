use bevy::prelude::*;
use bevy::pbr::wireframe::WireframeConfig;
use super::pulse::Pulsating;

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq)]
pub enum RenderMode {
    Wireframe,
    HiddenLine,
    #[default]
    Shaded,
    XRay,
    Realistic,
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
}

/// Press R to cycle through the 6 render modes
pub fn cycle_render_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut mode: ResMut<RenderMode>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        *mode = mode.next();
        info!("Render mode → {:?}", *mode);
    }
}

/// Apply the active mode to Bevy's wireframe config and pulsation amplitude
pub fn apply_render_mode(
    mode: Res<RenderMode>,
    mut wireframe: ResMut<WireframeConfig>,
    mut pulsating: Query<&mut Pulsating>,
) {
    if !mode.is_changed() { return; }

    // Wireframe and HiddenLine both show edges
    wireframe.global = matches!(*mode, RenderMode::Wireframe | RenderMode::HiddenLine);

    // SacredPulse: triple the breath amplitude for a dramatic effect
    let amplitude = if matches!(*mode, RenderMode::SacredPulse) { 0.18 } else { 0.06 };
    for mut p in &mut pulsating {
        p.amplitude = amplitude;
    }
}
