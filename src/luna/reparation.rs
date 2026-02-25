use super::nucleus::Nucleus;
use super::sprites::Sprites;

pub struct Reparation;

impl Reparation {
    pub fn heal(nucleus: &mut Nucleus, sprites: &mut Sprites) {
        nucleus.reset();
        sprites.stabilize_all();
        let default = Sprites::default_sprites();
        for (sprite, def) in sprites.list.iter_mut().zip(default.list.iter()) {
            sprite.vitesse_rotation = def.vitesse_rotation;
            sprite.taille = def.taille;
            sprite.distance_centre = def.distance_centre;
            sprite.couleur = def.couleur.clone();
        }
    }
}
