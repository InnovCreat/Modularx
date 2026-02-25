#[derive(Clone, Debug, PartialEq)]
pub enum CouleurMode {
    Fixed,
    Cyclic,
    Reactive,
}

#[derive(Clone, Debug)]
pub struct Couleur {
    pub palette: Vec<String>,
    pub mode: CouleurMode,
    pub cycle_index: usize,
}

impl Couleur {
    pub fn new() -> Self {
        Self {
            palette: vec![
                "#FF6B9D".into(),
                "#4ECDC4".into(),
                "#FFE66D".into(),
                "#95E1D3".into(),
                "#F38181".into(),
            ],
            mode: CouleurMode::Fixed,
            cycle_index: 0,
        }
    }

    pub fn cycle_next(&mut self) {
        self.cycle_index = (self.cycle_index + 1) % self.palette.len();
    }

    pub fn current(&self) -> &str {
        &self.palette[self.cycle_index]
    }

    pub fn apply_aurore(&mut self, sprites: &mut [super::sprites::Sprite]) {
        self.mode = CouleurMode::Cyclic;
        for (i, sprite) in sprites.iter_mut().enumerate() {
            let idx = (self.cycle_index + i) % self.palette.len();
            sprite.couleur = self.palette[idx].clone();
        }
        self.cycle_next();
    }
}
