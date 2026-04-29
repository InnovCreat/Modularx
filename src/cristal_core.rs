// VERITAS HORTUS · C0 · cristal_core.rs
// Cristal Core (⟡) — The Nucleus Stabilizer at 639 Hz
//
// The Cristal Core is the convergence point of the entire system.
// It does not move — everything orbits around it.
// It pulses at 639 Hz — the Anahata Chakra, frequency of coherence.
// Its resonance is 0.618 — the golden ratio (φ).
//
// Stability: 1.0 (target). If stability drops below 0.95, the system warns.

/// Pulse color state — cycles Purple → Blue → Red based on system state.
/// État de couleur de pulsation — cycle Violet → Bleu → Rouge selon l'état système.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PulseColor {
    /// Calm, coherent, listening / Calme, cohérent, à l'écoute
    Purple,
    /// Active, processing, transmitting / Actif, traitant, transmettant
    Blue,
    /// Alert, healing needed, high entropy / Alerte, guérison nécessaire, haute entropie
    Red,
}

impl PulseColor {
    pub fn hex(self) -> &'static str {
        match self {
            PulseColor::Purple => "#9b59b6",
            PulseColor::Blue   => "#00d4ff",
            PulseColor::Red    => "#c8a84b",
        }
    }
}

/// CristalCore — the stabilizing resonator of the Veritas Hortus system.
///
/// JSON canonical form mirrors the ⟡_state block in the VERITAS HORTUS spec.
#[derive(Debug, Clone)]
pub struct CristalCore {
    /// Resonance frequency in Hz — must stay at 639.0 / Fréquence de résonance en Hz
    pub frequency:     f64,
    /// System coherence 0.0–1.0 (target: 1.0) / Cohérence système
    pub stability:     f32,
    /// Golden ratio resonance 0.618 (φ) / Résonance en ratio doré
    pub resonance:     f32,
    /// Whether the pulse cycle is active / Si le cycle de pulsation est actif
    pub cycle_enabled: bool,
    /// Memory decay coefficient per pulse (0.95 = slow decay) / Coefficient de décroissance
    pub mae_decay:     f32,
    /// Current pulse color / Couleur de pulsation courante
    pub color:         PulseColor,
    /// Total pulse count since creation / Nombre total de pulsations depuis la création
    pub pulse_count:   u64,
}

impl CristalCore {
    pub fn new() -> Self {
        Self {
            frequency:     639.0,
            stability:     1.0,
            resonance:     0.618,
            cycle_enabled: true,
            mae_decay:     0.95,
            color:         PulseColor::Purple,
            pulse_count:   0,
        }
    }

    /// True when the core is at target resonance and fully stable.
    /// Vrai quand le cœur est à la résonance cible et pleinement stable.
    pub fn is_resonant(&self) -> bool {
        (self.frequency - 639.0).abs() < 0.1
            && self.stability >= 0.99
            && self.cycle_enabled
    }

    /// Execute one pulse cycle — slight entropy decay, color shift.
    /// Exécute un cycle de pulsation — légère décroissance d'entropie, changement de couleur.
    pub fn pulse(&mut self) {
        if !self.cycle_enabled { return; }
        self.pulse_count += 1;
        self.stability = (self.stability * self.mae_decay).clamp(0.0, 1.0);
        self.color = match self.stability {
            s if s >= 0.97 => PulseColor::Purple,
            s if s >= 0.85 => PulseColor::Blue,
            _              => PulseColor::Red,
        };
    }

    /// Restore to full stability — call after healing/repair cycle.
    /// Restaure la pleine stabilité — appeler après un cycle de guérison/réparation.
    pub fn restore(&mut self) {
        self.stability = 1.0;
        self.color     = PulseColor::Purple;
    }

    /// True if the core needs healing (stability below threshold).
    pub fn needs_healing(&self) -> bool {
        self.stability < 0.85
    }

    /// Human-readable status / Statut lisible.
    pub fn status(&self) -> &'static str {
        if !self.cycle_enabled   { return "Dormant"; }
        if self.stability >= 0.99 { return "Resonant"; }
        if self.stability >= 0.85 { return "Active"; }
        "Healing Required"
    }
}

impl Default for CristalCore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cristal_resonant_at_creation() {
        let c = CristalCore::default();
        assert!(c.is_resonant());
    }

    #[test]
    fn test_cristal_frequency_is_639() {
        let c = CristalCore::default();
        assert!((c.frequency - 639.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_cristal_golden_ratio() {
        let c = CristalCore::default();
        assert!((c.resonance - 0.618).abs() < 1e-4);
    }

    #[test]
    fn test_pulse_decrements_stability() {
        let mut c = CristalCore::default();
        c.pulse();
        assert!(c.stability < 1.0);
        assert_eq!(c.pulse_count, 1);
    }

    #[test]
    fn test_restore_returns_to_full_stability() {
        let mut c = CristalCore::default();
        for _ in 0..20 { c.pulse(); }
        c.restore();
        assert_eq!(c.stability, 1.0);
        assert_eq!(c.color, PulseColor::Purple);
    }

    #[test]
    fn test_needs_healing_after_many_pulses() {
        let mut c = CristalCore::default();
        for _ in 0..100 { c.pulse(); }
        assert!(c.needs_healing());
    }

    #[test]
    fn test_pulse_color_hex() {
        assert_eq!(PulseColor::Purple.hex(), "#9b59b6");
        assert_eq!(PulseColor::Blue.hex(),   "#00d4ff");
        assert_eq!(PulseColor::Red.hex(),    "#c8a84b");
    }

    #[test]
    fn test_status_resonant_at_start() {
        let c = CristalCore::default();
        assert_eq!(c.status(), "Resonant");
    }

    #[test]
    fn test_dormant_when_cycle_disabled() {
        let mut c = CristalCore::default();
        c.cycle_enabled = false;
        assert_eq!(c.status(), "Dormant");
    }
}
