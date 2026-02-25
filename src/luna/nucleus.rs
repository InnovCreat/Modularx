#[derive(Clone, Debug, PartialEq)]
pub enum EtatNucleus {
    Actif,
    Latent,
    Instable,
}

impl EtatNucleus {
    pub fn color(&self) -> &str {
        match self {
            EtatNucleus::Actif => "#A855F7",
            EtatNucleus::Latent => "#6366F1",
            EtatNucleus::Instable => "#EF4444",
        }
    }

    pub fn label(&self) -> &str {
        match self {
            EtatNucleus::Actif => "actif",
            EtatNucleus::Latent => "latent",
            EtatNucleus::Instable => "instable",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Nucleus {
    pub gravite: f64,
    pub pulsation: f64,
    pub etat: EtatNucleus,
    pub x: f64,
    pub y: f64,
}

impl Nucleus {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            gravite: 1.0,
            pulsation: 60.0,
            etat: EtatNucleus::Actif,
            x,
            y,
        }
    }

    pub fn pulse_radius(&self, time: f64) -> f64 {
        let base = 20.0;
        let amplitude = 8.0;
        let freq = self.pulsation / 60.0;
        base + amplitude * (time * freq * std::f64::consts::PI * 2.0 / 60.0).sin()
    }

    pub fn reset(&mut self) {
        self.gravite = 1.0;
        self.pulsation = 60.0;
        self.etat = EtatNucleus::Actif;
    }
}
