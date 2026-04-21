// Template 021 — Orbital Mechanics

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuantumState {
    Stable,
    Superpose,
}

#[derive(Debug, Clone)]
pub struct Sprite {
    pub nom:              String,
    pub couleur:          String,
    pub vitesse_rotation: f64,   // deg/s (négatif = sens anti-horaire)
    pub distance_centre:  f64,   // px
    pub angle:            f64,   // 0–360°
    pub taille:           f64,   // px radius
    pub opacite:          f64,   // 0.0–1.0
    pub etat_quantique:   QuantumState,
}

impl Sprite {
    /// Met à jour l'angle selon le delta temps et la vitesse de rotation
    pub fn update(&mut self, delta_s: f64, tempo: f64) {
        self.angle = (self.angle + self.vitesse_rotation * delta_s * tempo).rem_euclid(360.0);
    }

    /// Position cartésienne sur le canvas (centre = 250,250)
    pub fn position(&self) -> (f64, f64) {
        let rad = self.angle.to_radians();
        (250.0 + self.distance_centre * rad.cos(),
         250.0 + self.distance_centre * rad.sin())
    }
}

pub fn default_sprites() -> Vec<Sprite> {
    vec![
        Sprite { nom: "Aether".into(),  couleur: "#FF6B9D".into(), vitesse_rotation:  30.0, distance_centre: 100.0, angle:   0.0, taille: 12.0, opacite: 1.0, etat_quantique: QuantumState::Stable },
        Sprite { nom: "Lumina".into(),  couleur: "#4ECDC4".into(), vitesse_rotation: -45.0, distance_centre: 150.0, angle: 120.0, taille: 10.0, opacite: 1.0, etat_quantique: QuantumState::Stable },
        Sprite { nom: "Solaris".into(), couleur: "#FFE66D".into(), vitesse_rotation:  20.0, distance_centre:  80.0, angle: 240.0, taille:  8.0, opacite: 1.0, etat_quantique: QuantumState::Stable },
    ]
}
