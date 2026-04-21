// Template 020 — Nucleus State

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NucleusState {
    Actif,
    Latent,
    Instable,
}

#[derive(Debug, Clone)]
pub struct Nucleus {
    pub state:     NucleusState,
    pub gravite:   f64,   // -2.0 .. +3.0
    pub pulsation: f64,   // 20.0 .. 120.0 bpm
}

impl Default for Nucleus {
    fn default() -> Self {
        Self { state: NucleusState::Actif, gravite: 1.0, pulsation: 60.0 }
    }
}

impl Nucleus {
    pub fn color(&self) -> &'static str {
        match self.state {
            NucleusState::Actif    => "#A855F7",
            NucleusState::Latent   => "#6366F1",
            NucleusState::Instable => "#EF4444",
        }
    }

    pub fn apply_rituel(&mut self, keyword: &str) {
        match keyword {
            "RÉVEIL"  => { self.state = NucleusState::Actif;    self.pulsation = 80.0; }
            "SOMMEIL" => { self.state = NucleusState::Latent;   self.pulsation = 20.0; }
            "CHAOS"   => { self.state = NucleusState::Instable; }
            "HARMONIE" => {
                self.state = NucleusState::Actif;
                self.pulsation = 60.0;
                self.gravite   = 1.0;
            }
            "ATTRACTION" => { self.gravite = 2.0; }
            "RÉPULSION"  => { self.gravite = -1.0; }
            "RÉPARATION" => {
                self.state = NucleusState::Actif;
                self.gravite   = 1.0;
                self.pulsation = 60.0;
            }
            _ => {}
        }
    }
}
