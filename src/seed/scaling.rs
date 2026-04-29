// SEED CARRIER — scaling.rs
// Scale system, unit measures, value scoring charts
// Système d'échelle, unités de mesure, tableaux de scores

// ─── SCALE ENUM ──────────────────────────────────────────────────────────────

/// Operating scale of a module — from sub-atomic to ecosystem level.
/// Échelle d'opération d'un module — du sous-atomique au niveau écosystème.
///
/// | Scale  | Factor | Indicates                          |
/// |--------|--------|------------------------------------|
/// | Nano   | 10⁻⁹   | Cryptographic / hash operations    |
/// | Micro  | 10⁻⁶   | Internal module operations         |
/// | Milli  | 10⁻³   | Inter-module communication         |
/// | Unity  | 1.0    | Human-perceptible interactions     |
/// | Kilo   | 10³    | Network-level coordination         |
/// | Mega   | 10⁶    | System-wide effects                |
/// | Giga   | 10⁹    | Ecosystem / multi-system reach     |
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scale {
    Nano,   // 10^-9 — hash, crypto, sub-module
    Micro,  // 10^-6 — single module ops
    Milli,  // 10^-3 — inter-module signals
    Unity,  // 1.0   — human scale (base)
    Kilo,   // 10^3  — network ops
    Mega,   // 10^6  — system-wide
    Giga,   // 10^9  — ecosystem
}

impl Scale {
    /// Returns the numeric multiplier for this scale.
    pub fn factor(self) -> f64 {
        match self {
            Scale::Nano  => 1e-9,
            Scale::Micro => 1e-6,
            Scale::Milli => 1e-3,
            Scale::Unity => 1.0,
            Scale::Kilo  => 1e3,
            Scale::Mega  => 1e6,
            Scale::Giga  => 1e9,
        }
    }

    /// Human-readable label / Étiquette lisible
    pub fn label(self) -> &'static str {
        match self {
            Scale::Nano  => "Nano  (10⁻⁹) — crypto/hash",
            Scale::Micro => "Micro (10⁻⁶) — module ops",
            Scale::Milli => "Milli (10⁻³) — inter-module",
            Scale::Unity => "Unity (1.0)  — human scale",
            Scale::Kilo  => "Kilo  (10³)  — network",
            Scale::Mega  => "Mega  (10⁶)  — system",
            Scale::Giga  => "Giga  (10⁹)  — ecosystem",
        }
    }

    /// Unit string for display / Chaîne d'unité pour l'affichage
    pub fn unit(self) -> &'static str {
        match self {
            Scale::Nano  => "nm",
            Scale::Micro => "µm",
            Scale::Milli => "mm",
            Scale::Unity => "m",
            Scale::Kilo  => "km",
            Scale::Mega  => "Mm",
            Scale::Giga  => "Gm",
        }
    }
}

impl Default for Scale {
    fn default() -> Self {
        Scale::Unity
    }
}

// ─── SCALE ENTRY (history / historique) ──────────────────────────────────────

/// A recorded scale change with timestamp and value at that moment.
/// Un changement d'échelle enregistré avec timestamp et valeur au moment T.
#[derive(Debug, Clone)]
pub struct ScaleEntry {
    pub scale:     Scale,
    /// Unix timestamp of the scale change / Timestamp Unix du changement d'échelle
    pub time:      u64,
    /// Observed value at this scale / Valeur observée à cette échelle
    pub value:     f64,
    /// Label describing why scale changed / Étiquette expliquant le changement
    pub reason:    String,
}

// ─── SCALE SYSTEM ────────────────────────────────────────────────────────────

/// Full scaling system — current scale + change history.
/// Système d'échelle complet — échelle courante + historique des changements.
#[derive(Debug, Clone)]
pub struct ScaleSystem {
    pub current: Scale,
    pub history: Vec<ScaleEntry>,
}

impl Default for ScaleSystem {
    fn default() -> Self {
        ScaleSystem {
            current: Scale::Unity,
            history: Vec::new(),
        }
    }
}

impl ScaleSystem {
    /// Transition to a new scale and record the change.
    /// Passe à une nouvelle échelle et enregistre le changement.
    pub fn transition(&mut self, new_scale: Scale, value: f64, reason: impl Into<String>) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.history.push(ScaleEntry {
            scale:  self.current,
            time:   now,
            value,
            reason: reason.into(),
        });
        self.current = new_scale;
    }

    /// Convert a value from current scale to Unity.
    /// Convertit une valeur de l'échelle courante vers Unity.
    pub fn to_unity(&self, value: f64) -> f64 {
        value * self.current.factor()
    }

    /// Convert a Unity value to the current scale.
    /// Convertit une valeur Unity vers l'échelle courante.
    pub fn from_unity(&self, unity_value: f64) -> f64 {
        if self.current.factor() == 0.0 { return 0.0; }
        unity_value / self.current.factor()
    }
}

// ─── VALUE SCORING ────────────────────────────────────────────────────────────

/// Types of metrics tracked in scoring charts.
/// Types de métriques suivies dans les tableaux de scores.
///
/// | Metric      | Indicates                                    |
/// |-------------|----------------------------------------------|
/// | Energy      | Power / activity level of the module         |
/// | Stability   | How consistent and reliable the module is    |
/// | Growth      | Rate of expansion / learning                 |
/// | Coherence   | Alignment with covenant values               |
/// | Interaction | Connection quality with other modules        |
/// | Harmony     | Resonance with 639 Hz core frequency         |
/// | Chaos       | Entropy / unpredictability (not always bad)  |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricType {
    Energy,
    Stability,
    Growth,
    Coherence,
    Interaction,
    Harmony,
    Chaos,
}

impl MetricType {
    pub fn label(self) -> &'static str {
        match self {
            MetricType::Energy      => "Energy      — power / activity level",
            MetricType::Stability   => "Stability   — consistency and reliability",
            MetricType::Growth      => "Growth      — expansion / learning rate",
            MetricType::Coherence   => "Coherence   — alignment with covenant",
            MetricType::Interaction => "Interaction — connection quality",
            MetricType::Harmony     => "Harmony     — resonance with 639 Hz",
            MetricType::Chaos       => "Chaos       — entropy / unpredictability",
        }
    }
}

/// A scored metric entry — normalized 0.0–1.0, with raw value and scale.
/// Entrée de métrique scorée — normalisée 0.0–1.0, avec valeur brute et échelle.
#[derive(Debug, Clone)]
pub struct ValueChart {
    pub metric:         MetricType,
    /// Normalized score 0.0 (none) → 1.0 (max) / Score normalisé
    pub score:          f32,
    /// Raw measured value / Valeur brute mesurée
    pub raw_value:      f64,
    /// Unit of raw value / Unité de la valeur brute (e.g. "Hz", "ms", "ops/s")
    pub unity_measure:  String,
    /// Scale at which this was measured / Échelle à laquelle c'est mesuré
    pub measured_at:    Scale,
}

impl ValueChart {
    pub fn new(metric: MetricType, score: f32, raw_value: f64, unity_measure: impl Into<String>) -> Self {
        ValueChart {
            metric,
            score:         score.clamp(0.0, 1.0),
            raw_value,
            unity_measure: unity_measure.into(),
            measured_at:   Scale::Unity,
        }
    }

    /// A full scoring snapshot for a module (all 7 metrics at default values).
    /// Snapshot complet pour un module (7 métriques avec valeurs par défaut).
    pub fn default_chart() -> Vec<ValueChart> {
        vec![
            ValueChart::new(MetricType::Energy,      0.5, 0.0, "ops/s"),
            ValueChart::new(MetricType::Stability,   1.0, 1.0, "ratio"),
            ValueChart::new(MetricType::Growth,      0.0, 0.0, "delta/s"),
            ValueChart::new(MetricType::Coherence,   1.0, 1.0, "ratio"),
            ValueChart::new(MetricType::Interaction, 0.0, 0.0, "connections"),
            ValueChart::new(MetricType::Harmony,     1.0, 639.0, "Hz"),
            ValueChart::new(MetricType::Chaos,       0.0, 0.0, "entropy"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_ordering() {
        assert!(Scale::Nano < Scale::Unity);
        assert!(Scale::Unity < Scale::Giga);
    }

    #[test]
    fn test_scale_factors() {
        assert!((Scale::Unity.factor() - 1.0).abs() < f64::EPSILON);
        assert!((Scale::Kilo.factor() - 1000.0).abs() < f64::EPSILON);
        assert!((Scale::Milli.factor() - 0.001).abs() < 1e-15);
    }

    #[test]
    fn test_scale_system_to_unity() {
        let mut sys = ScaleSystem::default();
        sys.transition(Scale::Kilo, 1.5, "network op");
        let unity = sys.to_unity(1.0);
        assert!((unity - 1000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_value_chart_clamps_score() {
        let vc = ValueChart::new(MetricType::Energy, 1.5, 100.0, "ops/s");
        assert_eq!(vc.score, 1.0);
    }

    #[test]
    fn test_default_chart_has_7_metrics() {
        assert_eq!(ValueChart::default_chart().len(), 7);
    }

    #[test]
    fn test_harmony_default_is_639hz() {
        let chart = ValueChart::default_chart();
        let harmony = chart.iter().find(|v| v.metric == MetricType::Harmony).unwrap();
        assert!((harmony.raw_value - 639.0).abs() < f64::EPSILON);
        assert_eq!(harmony.unity_measure, "Hz");
    }
}
