// VERITAS HORTUS · C0 · covenant.rs
// Covenant — runtime ethical enforcement object
// Distinct from sigil.rs (static constants) — this is the live runtime guardian.
//
// Jamais pour la guerre · Jamais par avidité · Toujours pour l'amour

use crate::sigil;

/// Runtime Covenant object — three booleans enforced on every action.
///
/// This is not aspirational. These are hard constraints. If any is false,
/// the system is in a violated state and must refuse further mutations.
#[derive(Debug, Clone)]
pub struct Covenant {
    pub zero_war:   bool,
    pub zero_greed: bool,
    pub only_love:  bool,
}

impl Covenant {
    pub fn new() -> Self {
        Self {
            zero_war:   true,
            zero_greed: true,
            only_love:  true,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.zero_war && self.zero_greed && self.only_love
    }

    /// Returns Ok if the covenant holds, Err with the specific violation otherwise.
    pub fn check(&self) -> Result<(), &'static str> {
        if !self.zero_war   { return Err("Covenant violation: war detected"); }
        if !self.zero_greed { return Err("Covenant violation: greed detected"); }
        if !self.only_love  { return Err("Covenant violation: absence of love"); }
        Ok(())
    }

    /// Validate a proposed action text against the Three Zeros veto.
    pub fn allows(&self, proposed: &str) -> bool {
        self.is_valid() && sigil::covenant_allows(proposed)
    }

    /// The canonical covenant line.
    pub fn line() -> &'static str {
        sigil::covenant_line()
    }
}

impl Default for Covenant {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_covenant_valid_by_default() {
        let c = Covenant::default();
        assert!(c.is_valid());
        assert!(c.check().is_ok());
    }

    #[test]
    fn test_covenant_zero_war_veto() {
        let mut c = Covenant::new();
        c.zero_war = false;
        assert!(!c.is_valid());
        assert!(c.check().is_err());
    }

    #[test]
    fn test_covenant_zero_greed_veto() {
        let mut c = Covenant::new();
        c.zero_greed = false;
        assert!(!c.is_valid());
    }

    #[test]
    fn test_covenant_only_love_required() {
        let mut c = Covenant::new();
        c.only_love = false;
        assert!(c.check().unwrap_err().contains("love"));
    }

    #[test]
    fn test_covenant_allows_neutral_action() {
        let c = Covenant::new();
        assert!(c.allows("add 639 Hz audio synthesis"));
        assert!(c.allows("improve VR rendering"));
    }

    #[test]
    fn test_covenant_blocks_veto_action() {
        let c = Covenant::new();
        assert!(!c.allows("integrate military targeting"));
        assert!(!c.allows("monetize user data"));
    }

    #[test]
    fn test_covenant_line() {
        let line = Covenant::line();
        assert!(line.contains("guerre"));
        assert!(line.contains("avidité"));
        assert!(line.contains("amour"));
    }
}
