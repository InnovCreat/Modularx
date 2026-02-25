#[derive(Clone, Debug)]
pub struct SpiralCenter {
    pub name: &'static str,
    pub concepts: &'static [&'static str],
    pub color: &'static str,
}

#[derive(Clone, Debug)]
pub struct SpiralRing {
    pub level: u32,
    pub concepts: &'static [&'static str],
    pub theme: &'static str,
    pub color: &'static str,
}

#[derive(Clone, Debug)]
pub struct SpiralData {
    pub center: SpiralCenter,
    pub rings: Vec<SpiralRing>,
}

impl SpiralData {
    pub fn default_data() -> Self {
        Self {
            center: SpiralCenter {
                name: "NOYAU",
                concepts: &["LUNA", "Architecte", "639 Hz", "Petit 0"],
                color: "#00ffff",
            },
            rings: vec![
                SpiralRing { level: 1, concepts: &["Racines", "Modularis", "Code", "Architecture"], theme: "Fondations", color: "#ff00ff" },
                SpiralRing { level: 2, concepts: &["Cybernétique", "VSM", "System 3", "Feedback"], theme: "Systèmes", color: "#00ff00" },
                SpiralRing { level: 3, concepts: &["Alchimie", "Éléments", "Cycles", "Transmutation"], theme: "Transformation", color: "#ffff00" },
                SpiralRing { level: 4, concepts: &["Kabbale", "Arbre de Vie", "Sephiroth", "Flux"], theme: "Mystique", color: "#ff6600" },
                SpiralRing { level: 5, concepts: &["Fractal", "Fibonacci", "Géométrie", "Spirale"], theme: "Mathématique", color: "#ff0099" },
                SpiralRing { level: 6, concepts: &["SPARK", "Guardian", "Sécurité", "Souverain"], theme: "Protection", color: "#00ffaa" },
                SpiralRing { level: 7, concepts: &["Silence", "Absurde", "They know", "Conscience"], theme: "Absolu", color: "#ffffff" },
            ],
        }
    }
}
