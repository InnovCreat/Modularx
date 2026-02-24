// ============================================================
// quantum_state.rs — Minimal stub for node_causality tests
// ============================================================

pub const BASE_TENSION: f64 = 1.0;

#[derive(Debug, Clone, PartialEq)]
pub enum SourceKind {
    RawPerturbation,
    Tension,
    LogX,
    Custom(String),
}

// ── Nested causality state ───────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct MutualInfoState {
    pub mi: f64,
    pub normalized_mi: f64,
}

#[derive(Debug, Clone, Default)]
pub struct TransferEntropyState {
    pub te: f64,
    /// te_xy - te_yx  (positive = X drives Y, negative = Y drives X)
    pub net_flow: f64,
    /// 1 = XtoY dominant, -1 = YtoX dominant, 0 = Symmetric or Negligible
    pub dominant: i8,
}

#[derive(Debug, Clone, Default)]
pub struct GrangerState {
    pub f_stat: f64,
    pub p_value: f64,
    pub is_causal: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CausalityState {
    pub mutual_info: MutualInfoState,
    pub transfer_entropy: TransferEntropyState,
    pub granger: GrangerState,
}

// ── Main QuantumState ────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct QuantumState {
    pub base_perturbation: Vec<f64>,
    pub tension: Vec<f64>,
    pub log_x: Vec<f64>,
    pub dirty: bool,
    pub causality: CausalityState,
    amplitude: f64,
}

impl QuantumState {
    /// Create a new state seeded with a simple deterministic signal.
    pub fn new(_seed: u64, n: usize, amplitude: f64) -> Self {
        let base_perturbation: Vec<f64> = (0..n)
            .map(|i| ((i as f64) * 0.2).sin() * amplitude)
            .collect();
        let tension: Vec<f64> = base_perturbation
            .iter()
            .map(|&v| v * amplitude + BASE_TENSION)
            .collect();
        let log_x: Vec<f64> = base_perturbation
            .iter()
            .map(|&v| (v.abs() + 1.0).ln())
            .collect();
        QuantumState {
            base_perturbation,
            tension,
            log_x,
            dirty: false,
            causality: CausalityState::default(),
            amplitude,
        }
    }

    /// Recompute derived fields from base_perturbation.
    pub fn refresh(&mut self) {
        let n = self.base_perturbation.len();
        self.tension = self
            .base_perturbation
            .iter()
            .map(|&v| v * self.amplitude + BASE_TENSION)
            .collect();
        self.log_x = self
            .base_perturbation
            .iter()
            .map(|&v| (v.abs() + 1.0).ln())
            .collect();
        // Ensure consistent length
        self.tension.resize(n, BASE_TENSION);
        self.log_x.resize(n, 0.0);
        self.dirty = false;
    }
}
