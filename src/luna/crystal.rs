// Template 012 — Crystal State

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CrystalState {
    Divergent,
    Resonant,
    Convergent,
}

#[derive(Debug, Clone)]
pub struct Crystal {
    pub state:     CrystalState,
    pub coherence: f64,   // 0.0–1.0
    pub symmetry:  f64,   // 0.0–1.0
    pub emergence: f64,   // 0.0–1.0
    pub frequency: f64,   // Hz
}

impl Default for Crystal {
    fn default() -> Self {
        Self {
            state:     CrystalState::Resonant,
            coherence: 1.0,
            symmetry:  1.0,
            emergence: 0.0,
            frequency: crate::luna::pulse::RESONANCE_FREQ,
        }
    }
}

impl CrystalState {
    pub fn frequency(&self) -> f64 {
        match self {
            CrystalState::Resonant   => 639.0,
            CrystalState::Divergent  => 741.0,
            CrystalState::Convergent => 528.0,
        }
    }
}
