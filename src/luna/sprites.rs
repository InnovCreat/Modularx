use super::quantum::EtatQuantique;

#[derive(Clone, Debug)]
pub struct Sprite {
    pub nom: String,
    pub taille: f64,
    pub couleur: String,
    pub vitesse_rotation: f64,
    pub distance_centre: f64,
    pub angle: f64,
    pub etat_quantique: EtatQuantique,
    pub opacite: f64,
}

impl Sprite {
    pub fn new(nom: &str, couleur: &str, vitesse: f64, distance: f64, angle_deg: f64) -> Self {
        Self {
            nom: nom.to_string(),
            taille: 12.0,
            couleur: couleur.to_string(),
            vitesse_rotation: vitesse,
            distance_centre: distance,
            angle: angle_deg.to_radians(),
            etat_quantique: EtatQuantique::Stable,
            opacite: 1.0,
        }
    }

    pub fn update(&mut self, dt: f64, tempo: f64) {
        self.angle += (self.vitesse_rotation * tempo * dt).to_radians();
        if let EtatQuantique::Superpose = self.etat_quantique {
            let t = self.angle * 3.0;
            self.opacite = 0.5 + 0.2 * t.sin();
        }
    }

    pub fn position(&self, cx: f64, cy: f64) -> (f64, f64) {
        let x = cx + self.angle.cos() * self.distance_centre;
        let y = cy + self.angle.sin() * self.distance_centre;
        (x, y)
    }

    pub fn ghost_position(&self, cx: f64, cy: f64) -> (f64, f64) {
        let offset = 10.0 * (self.angle * 2.0).sin();
        let x = cx + self.angle.cos() * self.distance_centre + offset;
        let y = cy + self.angle.sin() * self.distance_centre + offset;
        (x, y)
    }

    pub fn stabilize(&mut self) {
        self.etat_quantique = EtatQuantique::Stable;
        self.opacite = 1.0;
    }
}

#[derive(Clone, Debug)]
pub struct Sprites {
    pub list: Vec<Sprite>,
}

impl Sprites {
    pub fn default_sprites() -> Self {
        Self {
            list: vec![
                Sprite::new("Aether", "#FF6B9D", 30.0, 100.0, 0.0),
                Sprite::new("Lumina", "#4ECDC4", -45.0, 150.0, 120.0),
                Sprite::new("Solaris", "#FFE66D", 20.0, 80.0, 240.0),
            ],
        }
    }

    pub fn update_all(&mut self, dt: f64, tempo: f64) {
        for sprite in &mut self.list {
            sprite.update(dt, tempo);
        }
    }

    pub fn stabilize_all(&mut self) {
        for sprite in &mut self.list {
            sprite.stabilize();
        }
    }
}
