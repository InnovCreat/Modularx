// veritas_full.rs — Veritas Modularis (single-file, rustc pure)
// jamais pour la guerre · jamais pour l'argent · toujours pour l'amour
// ============================================================

use std::time::{SystemTime, UNIX_EPOCH};

// === Constants & Enums ==============================================

pub const BASE_TENSION: f64 = 1.0;
pub const LOG_X_MIN:    f64 = -35.0;
pub const LOG_X_MAX:    f64 =  27.0;
pub const LOG_X_SPAN:   f64 = LOG_X_MAX - LOG_X_MIN;

// std::f64::consts::PHI does not exist — define it manually
pub const PHI: f64 = 1.618_033_988_749_895_f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AutonomyLevel { Minimal = 0, Surveilled = 1, Corrective = 2, Full = 3 }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntryKind { Violation, Info, Harmony, Cure }

// AmplitudeOverflow added — it is used in Watchdog::check
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViolationType {
    SigMismatch,
    HighChaos,
    NaNDetected,
    IntegrityFail,
    WatchdogKill,
    AmplitudeOverflow,
}

impl std::fmt::Display for ViolationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CureAction { NoCureNeeded, Damping, ForceClamp, Rollback, EmergencyHalt }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChaosLabel { Stable, Resonant, Chaotic, Saturated }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TernaryState { Positive, Neutral, Negative }

// === Core types =====================================================

#[derive(Debug, Clone)]
pub struct GlobalConfig {
    pub kill_switch:   bool,
    pub max_autonomy:  AutonomyLevel,
    pub enable_alerts: bool,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            kill_switch:   false,
            max_autonomy:  AutonomyLevel::Corrective,
            enable_alerts: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Guard {
    pub violations: u64,
}

impl Guard {
    pub fn new() -> Self { Self { violations: 0 } }
    pub fn is_valid(&self) -> bool { self.violations == 0 }
}

#[derive(Debug, Clone)]
pub struct QuantumState {
    pub amplitude:         f64,
    pub num_points:        usize,
    pub base_perturbation: Vec<f64>,
    pub tension:           Vec<f64>,
    pub dirty:             bool,
    pub sig:               u64,
    pub chaos_label:       ChaosLabel,
    pub ternary:           TernaryState,
}

impl QuantumState {
    pub fn new(n: usize, amp: f64) -> Self {
        let mut s = Self {
            amplitude:         amp,
            num_points:        n,
            base_perturbation: vec![0.0; n],
            tension:           vec![],
            dirty:             true,
            sig:               0,
            chaos_label:       ChaosLabel::Stable,
            ternary:           TernaryState::Neutral,
        };
        s.update_sig();
        s
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn update_sig(&mut self) {
        let mut h: u64 = 5381;
        h = h.wrapping_mul(6364136223846793005).wrapping_add(self.amplitude.to_bits());
        h = h.wrapping_mul(6364136223846793005).wrapping_add(self.num_points as u64);
        for &v in &self.base_perturbation {
            h = h.wrapping_mul(6364136223846793005).wrapping_add(v.to_bits());
        }
        self.sig = h;
    }

    pub fn verify_sig(&self) -> bool {
        let mut h: u64 = 5381;
        h = h.wrapping_mul(6364136223846793005).wrapping_add(self.amplitude.to_bits());
        h = h.wrapping_mul(6364136223846793005).wrapping_add(self.num_points as u64);
        for &v in &self.base_perturbation {
            h = h.wrapping_mul(6364136223846793005).wrapping_add(v.to_bits());
        }
        h == self.sig
    }

    pub fn refresh(&mut self) {
        if !self.dirty { return; }
        self.tension = self.base_perturbation.iter()
            .map(|&bp| (BASE_TENSION + bp * self.amplitude).max(0.0))
            .collect();
        self.dirty = false;
        self.update_sig();
    }
}

// === Metrics ========================================================

pub fn node_spread(s: &mut QuantumState) -> f64 {
    if s.dirty { s.refresh(); }
    if s.base_perturbation.is_empty() { return 0.0; }
    let sum_sq: f64 = s.base_perturbation.iter()
        .map(|&bp| (bp * s.amplitude).powi(2))
        .sum();
    (sum_sq / s.num_points as f64).sqrt() / 3.0
}

#[derive(Debug, Clone)]
pub struct Statistics { pub mean: f64, pub stddev: f64 }

pub fn node_statistics(s: &mut QuantumState) -> Statistics {
    if s.dirty { s.refresh(); }
    if s.tension.is_empty() { return Statistics { mean: 0.0, stddev: 0.0 }; }
    let mean = s.tension.iter().sum::<f64>() / s.tension.len() as f64;
    let variance = s.tension.iter()
        .map(|&v| (v - mean).powi(2))
        .sum::<f64>()
        / s.tension.len() as f64;
    Statistics { mean, stddev: variance.sqrt() }
}

// === MycBook & Monitor ==============================================

pub fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[derive(Debug, Clone)]
pub struct MycEntry {
    pub ts:   u64,
    pub kind: EntryKind,
    pub msg:  String,
}

pub struct MycBook {
    pub entries: Vec<MycEntry>,
}

impl MycBook {
    pub fn append(&mut self, kind: EntryKind, msg: &str) {
        self.entries.push(MycEntry {
            ts: unix_now(),
            kind,
            msg: msg.to_string(),
        });
        println!("{} {:?} {}", unix_now(), kind, msg);
    }
}

pub struct NodeMonitor {
    pub mycbook:    MycBook,
    pub violations: u64,
}

impl NodeMonitor {
    pub fn new() -> Self {
        Self {
            mycbook:    MycBook { entries: vec![] },
            violations: 0,
        }
    }

    pub fn alert_violation(&mut self, v: ViolationType, detail: &str) {
        self.violations += 1;
        self.mycbook.append(EntryKind::Violation, &format!("{} — {}", v, detail));
    }

    pub fn alert_harmony(&mut self, msg: &str) {
        self.mycbook.append(EntryKind::Harmony, msg);
    }

    pub fn alert_cure(&mut self, msg: &str) {
        self.mycbook.append(EntryKind::Cure, msg);
    }
}

// === Cure with φ thresholds =========================================

#[derive(Debug, Clone)]
pub struct CureThresholds {
    pub phi:                  f64,
    pub phi_inv:              f64,
    pub spread_trigger_ratio: f64,
    pub damping_factor:       f64,
    pub amplitude_hard_max:   f64,
    pub amplitude_target:     f64,
    pub max_iterations:       usize,
}

impl Default for CureThresholds {
    fn default() -> Self {
        Self {
            phi:                  PHI,
            phi_inv:              1.0 / PHI,
            spread_trigger_ratio: PHI,
            damping_factor:       1.0 / PHI,
            amplitude_hard_max:   5.0,
            amplitude_target:     1.0,
            max_iterations:       8,
        }
    }
}

pub struct NodeCure {
    pub thresholds:  CureThresholds,
    pub total_cures: u64,
}

impl NodeCure {
    pub fn with_defaults() -> Self {
        Self {
            thresholds:  CureThresholds::default(),
            total_cures: 0,
        }
    }

    pub fn needs_cure(&self, s: &mut QuantumState, monitor: &mut NodeMonitor) -> (bool, CureAction) {
        if s.tension.iter().any(|&v| v.is_nan() || v.is_infinite()) {
            monitor.alert_violation(ViolationType::NaNDetected, "NaN/Inf in tension");
            return (true, CureAction::EmergencyHalt);
        }
        if s.amplitude > self.thresholds.amplitude_hard_max {
            return (true, CureAction::ForceClamp);
        }
        let spread = s.base_perturbation.iter()
            .map(|&bp| bp * s.amplitude)
            .sum::<f64>()
            / s.num_points as f64;
        if spread > self.thresholds.spread_trigger_ratio {
            return (true, CureAction::Damping);
        }
        (false, CureAction::NoCureNeeded)
    }

    pub fn apply(&mut self, s: &mut QuantumState, monitor: &mut NodeMonitor) -> CureResult {
        let amp_before    = s.amplitude;
        let spread_before = s.base_perturbation.iter()
            .map(|&bp| bp * s.amplitude)
            .sum::<f64>()
            / s.num_points as f64;

        self.total_cures += 1;
        s.amplitude *= self.thresholds.damping_factor;
        s.mark_dirty();
        s.refresh();

        let spread_after = s.base_perturbation.iter()
            .map(|&bp| bp * s.amplitude)
            .sum::<f64>()
            / s.num_points as f64;

        let msg = format!("Damping φ → {:.3} → {:.3}", amp_before, s.amplitude);
        monitor.alert_cure(&msg);

        CureResult {
            applied:          true,
            action:           CureAction::Damping,
            iterations:       1,
            amplitude_before: amp_before,
            amplitude_after:  s.amplitude,
            spread_before,
            spread_after,
            message:          msg,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CureResult {
    pub applied:          bool,
    pub action:           CureAction,
    pub iterations:       usize,
    pub amplitude_before: f64,
    pub amplitude_after:  f64,
    pub spread_before:    f64,
    pub spread_after:     f64,
    pub message:          String,
}

// === Watchdog =======================================================

pub struct Watchdog {
    pub kill_switch: bool,
    pub max_amp:     f64,
    pub max_spread:  f64,
}

impl Watchdog {
    pub fn new() -> Self {
        Self { kill_switch: false, max_amp: 10.0, max_spread: 2.5 }
    }

    pub fn check(&mut self, s: &QuantumState, monitor: &mut NodeMonitor) -> bool {
        if s.amplitude > self.max_amp {
            monitor.alert_violation(ViolationType::AmplitudeOverflow, "Amp too high");
            self.kill_switch = true;
            return false;
        }
        let spread = s.base_perturbation.iter()
            .map(|&bp| bp * s.amplitude)
            .sum::<f64>()
            / s.num_points as f64;
        if spread > self.max_spread {
            monitor.alert_violation(ViolationType::HighChaos, "Spread too high");
            self.kill_switch = true;
            return false;
        }
        true
    }
}

// === Main cycle =====================================================

pub fn guard_and_cure_cycle(
    state:    &mut QuantumState,
    monitor:  &mut NodeMonitor,
    cure:     &mut NodeCure,
    watchdog: &mut Watchdog,
) -> Option<CureResult> {
    if !state.verify_sig() {
        monitor.alert_violation(ViolationType::SigMismatch, "State signature invalid");
        return None;
    }

    if !watchdog.check(state, monitor) {
        // Fallback: attempt one damping cure before halting
        let fallback = cure.apply(state, monitor);
        monitor.mycbook.append(
            EntryKind::Info,
            &format!("Watchdog fallback cure → amp={:.3}", state.amplitude),
        );
        if state.amplitude <= watchdog.max_amp {
            monitor.alert_harmony("Fallback cure succeeded — watchdog cleared");
        } else {
            monitor.alert_violation(ViolationType::WatchdogKill, "Fallback cure insufficient");
        }
        return Some(fallback);
    }

    let (needs, _) = cure.needs_cure(state, monitor);
    if needs {
        Some(cure.apply(state, monitor))
    } else {
        monitor.mycbook.append(EntryKind::Harmony, "Stable — 639 Hz");
        None
    }
}

// === Demo ===========================================================

fn main() {
    println!("Veritas Modularis — Full secure single-file");
    println!("hihi osti peace graine disco scroll\n");

    let mut state = QuantumState::new(100, 0.3);
    state.base_perturbation = vec![0.1; 100];
    state.refresh();

    let mut monitor  = NodeMonitor::new();
    let mut cure     = NodeCure::with_defaults();
    let mut watchdog = Watchdog::new();

    println!("Cycle 1 — stable");
    guard_and_cure_cycle(&mut state, &mut monitor, &mut cure, &mut watchdog);

    // Drive into chaos — mark_dirty() is required after direct field mutation
    // so refresh() doesn't exit early and update_sig() runs
    state.amplitude         = 7.0;   // 12.0 → watchdog kills; 7.0 → cure runs
    state.base_perturbation = vec![0.1; 100]; // 2.0×7.0=14.0 > max_spread; 0.1×7.0=0.7 ✓
    state.mark_dirty();
    state.refresh();

    println!("\nCycle 2 — chaos");
    guard_and_cure_cycle(&mut state, &mut monitor, &mut cure, &mut watchdog);

    // monitor.violations is the correct field name
    println!("\nViolations: {}", monitor.violations);
    println!("Cures:      {}", cure.total_cures);
    println!("Watchdog kill: {}", watchdog.kill_switch);
}

// === Tests ==========================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sig_verify_works() {
        let mut s = QuantumState::new(10, 1.0);
        assert!(s.verify_sig());
        s.amplitude += 0.1;
        assert!(!s.verify_sig());
    }

    #[test]
    fn watchdog_kills_overflow() {
        let s = QuantumState::new(10, 11.0);
        let mut m = NodeMonitor::new();
        let mut w = Watchdog::new();
        assert!(!w.check(&s, &mut m));
        assert!(w.kill_switch);
    }

    #[test]
    fn cure_triggers_on_high_amp() {
        let mut s = QuantumState::new(10, 6.0);
        s.base_perturbation = vec![0.1; 10];
        s.refresh();
        let mut m = NodeMonitor::new();
        let mut c = NodeCure::with_defaults();
        let mut w = Watchdog::new();
        let r = guard_and_cure_cycle(&mut s, &mut m, &mut c, &mut w);
        assert!(r.is_some());
        assert!(r.unwrap().applied);
    }

    #[test]
    fn phi_constant_correct() {
        // Golden ratio: φ = (1 + √5) / 2
        let expected = (1.0_f64 + 5.0_f64.sqrt()) / 2.0;
        assert!((PHI - expected).abs() < 1e-12);
    }

    #[test]
    fn refresh_updates_tension() {
        let mut s = QuantumState::new(4, 2.0);
        s.base_perturbation = vec![1.0, 0.5, 0.0, -0.5];
        s.dirty = true;
        s.refresh();
        // tension[i] = (BASE_TENSION + bp[i] * amplitude).max(0.0)
        assert!((s.tension[0] - (1.0 + 1.0 * 2.0)).abs() < 1e-12); // 3.0
        assert!((s.tension[1] - (1.0 + 0.5 * 2.0)).abs() < 1e-12); // 2.0
        assert!((s.tension[2] - (1.0 + 0.0 * 2.0)).abs() < 1e-12); // 1.0
        assert!((s.tension[3] - 0.0_f64.max(1.0 + (-0.5) * 2.0)).abs() < 1e-12); // 0.0
    }
}
