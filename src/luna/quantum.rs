#[derive(Clone, Debug, PartialEq)]
pub enum EtatQuantique {
    Stable,
    Superpose,
}

impl EtatQuantique {
    pub fn label(&self) -> &str {
        match self {
            EtatQuantique::Stable => "stable",
            EtatQuantique::Superpose => "superposé",
        }
    }

    pub fn is_superpose(&self) -> bool {
        matches!(self, EtatQuantique::Superpose)
    }
}
