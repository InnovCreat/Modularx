#[derive(Clone, Debug)]
pub struct Chronos {
    pub tempo_global: f64,
    pub actif: bool,
    pub cycle: u64,
}

impl Chronos {
    pub fn new() -> Self {
        Self {
            tempo_global: 1.0,
            actif: true,
            cycle: 0,
        }
    }

    pub fn tick(&mut self) {
        if self.actif {
            self.cycle = self.cycle.wrapping_add(1);
        }
    }

    pub fn pause(&mut self) {
        self.actif = false;
    }

    pub fn resume(&mut self) {
        self.actif = true;
    }

    pub fn toggle(&mut self) {
        self.actif = !self.actif;
    }

    pub fn reset(&mut self) {
        self.tempo_global = 1.0;
        self.actif = true;
        self.cycle = 0;
    }
}
