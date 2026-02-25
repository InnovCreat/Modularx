use super::nucleus::{Nucleus, EtatNucleus};
use super::sprites::Sprites;
use super::chronos::Chronos;
use super::couleur::Couleur;
use super::quantum::EtatQuantique;
use super::archive::Archive;

#[derive(Clone, Debug, PartialEq)]
pub enum Incantation {
    Reveil,
    Sommeil,
    Chaos,
    Harmonie,
    Attraction,
    Repulsion,
    Aurore,
    Reparation,
}

impl Incantation {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_uppercase().as_str() {
            "RÉVEIL" | "REVEIL" => Some(Incantation::Reveil),
            "SOMMEIL" => Some(Incantation::Sommeil),
            "CHAOS" => Some(Incantation::Chaos),
            "HARMONIE" => Some(Incantation::Harmonie),
            "ATTRACTION" => Some(Incantation::Attraction),
            "RÉPULSION" | "REPULSION" => Some(Incantation::Repulsion),
            "AURORE" => Some(Incantation::Aurore),
            "RÉPARATION" | "REPARATION" => Some(Incantation::Reparation),
            _ => None,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Incantation::Reveil => "RÉVEIL",
            Incantation::Sommeil => "SOMMEIL",
            Incantation::Chaos => "CHAOS",
            Incantation::Harmonie => "HARMONIE",
            Incantation::Attraction => "ATTRACTION",
            Incantation::Repulsion => "RÉPULSION",
            Incantation::Aurore => "AURORE",
            Incantation::Reparation => "RÉPARATION",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Incantation::Reveil => "Nucleus s'éveille avec énergie",
            Incantation::Sommeil => "Nucleus entre en sommeil profond",
            Incantation::Chaos => "Les sprites entrent en superposition",
            Incantation::Harmonie => "Stabilisation complète du système",
            Incantation::Attraction => "La gravité augmente, les sprites se rapprochent",
            Incantation::Repulsion => "Gravité inversée, les sprites s'éloignent",
            Incantation::Aurore => "Transformation chromatique",
            Incantation::Reparation => "Restauration complète du système",
        }
    }

    pub fn all() -> Vec<Incantation> {
        vec![
            Incantation::Reveil,
            Incantation::Sommeil,
            Incantation::Chaos,
            Incantation::Harmonie,
            Incantation::Attraction,
            Incantation::Repulsion,
            Incantation::Aurore,
            Incantation::Reparation,
        ]
    }
}

pub struct Rituel;

impl Rituel {
    pub fn invoke(
        incantation: &Incantation,
        nucleus: &mut Nucleus,
        sprites: &mut Sprites,
        chronos: &mut Chronos,
        couleur: &mut Couleur,
        archive: &mut Archive,
    ) -> String {
        let message = match incantation {
            Incantation::Reveil => {
                nucleus.etat = EtatNucleus::Actif;
                nucleus.pulsation = 80.0;
                format!("✦ {} — {}", incantation.label(), incantation.description())
            }
            Incantation::Sommeil => {
                nucleus.etat = EtatNucleus::Latent;
                nucleus.pulsation = 20.0;
                format!("🌙 {} — {}", incantation.label(), incantation.description())
            }
            Incantation::Chaos => {
                nucleus.etat = EtatNucleus::Instable;
                for sprite in &mut sprites.list {
                    sprite.etat_quantique = EtatQuantique::Superpose;
                    let factor = 1.0 + (sprite.angle.sin().abs() * 3.0);
                    sprite.vitesse_rotation *= factor;
                    sprite.opacite = 0.5 + 0.5 * sprite.angle.cos().abs();
                }
                format!("🌀 {} — {}", incantation.label(), incantation.description())
            }
            Incantation::Harmonie => {
                nucleus.etat = EtatNucleus::Actif;
                nucleus.pulsation = 60.0;
                sprites.stabilize_all();
                for sprite in &mut sprites.list {
                    sprite.vitesse_rotation = 30.0_f64.copysign(sprite.vitesse_rotation);
                }
                chronos.tempo_global = 1.0;
                format!("🕊 {} — {}", incantation.label(), incantation.description())
            }
            Incantation::Attraction => {
                nucleus.gravite = 2.0;
                for sprite in &mut sprites.list {
                    sprite.distance_centre = (sprite.distance_centre - 30.0).max(50.0);
                }
                format!("⊕ {} — {}", incantation.label(), incantation.description())
            }
            Incantation::Repulsion => {
                nucleus.gravite = -1.0;
                for sprite in &mut sprites.list {
                    sprite.distance_centre = (sprite.distance_centre + 30.0).min(200.0);
                }
                format!("⊖ {} — {}", incantation.label(), incantation.description())
            }
            Incantation::Aurore => {
                couleur.apply_aurore(&mut sprites.list);
                format!("🌅 {} — {}", incantation.label(), incantation.description())
            }
            Incantation::Reparation => {
                super::reparation::Reparation::heal(nucleus, sprites);
                format!("❤ {} — {}", incantation.label(), incantation.description())
            }
        };

        archive.log(&format!("Rituel {} déclenché — {}", incantation.label(), incantation.description()));
        message
    }
}
