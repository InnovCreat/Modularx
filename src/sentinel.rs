// VERITAS MODULARIS · C1 · sentinel.rs
// Phoenix Sentinel — internal system guardian
//
// Branch 2 of the SLTE tree (see branches.rs: BranchId::PhoenixSentinel)
// Frequency: 741 Hz — intuition, the frequency that senses what is wrong
// Authority: FREEZE · PANIC · KILL — only Sentinel can halt the system
//
// Receives alerts from SEARF. Tracks anomaly count.
// At threshold 5: automatic FREEZE — all mutations blocked.
// Guardian must call restore() to resume.

use crate::searf::{Alert, ThreatClass};

// ─── SENTINEL STATE ───────────────────────────────────────────────────────────

/// Current operational state of the Phoenix Sentinel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SentinelState {
    /// Normal — monitoring, no anomalies above threshold
    Watching,
    /// Anomaly detected — heightened monitoring, below threshold
    Alert,
    /// 5-anomaly threshold reached — all system mutations are BLOCKED
    Frozen,
    /// Critical threat — emergency lockdown, requires guardian intervention
    Panic,
}

impl SentinelState {
    pub fn label(self) -> &'static str {
        match self {
            SentinelState::Watching => "Watching",
            SentinelState::Alert    => "Alert",
            SentinelState::Frozen   => "Frozen",
            SentinelState::Panic    => "Panic",
        }
    }
}

// ─── SENTINEL ─────────────────────────────────────────────────────────────────

/// Phoenix Sentinel — Branch 2, internal system guardian at 741 Hz.
///
/// Sole authority for FREEZE / PANIC / KILL operations.
/// Does not detect threats — that is SEARF's role.
/// Receives classified Alerts from SEARF and decides whether to escalate.
#[derive(Debug)]
pub struct Sentinel {
    pub state:             SentinelState,
    /// Intuition frequency — 741 Hz, unchanged at runtime
    pub frequency_hz:      u32,
    /// Running count of anomalies since last restore
    pub anomaly_count:     u32,
    /// Threshold that triggers auto-freeze (default: 5)
    pub anomaly_threshold: u32,
    /// Audit log of significant events
    pub log:               Vec<String>,
}

impl Sentinel {
    pub fn new() -> Self {
        Self {
            state:             SentinelState::Watching,
            frequency_hz:      741,
            anomaly_count:     0,
            anomaly_threshold: 5,
            log:               Vec::new(),
        }
    }

    /// Process an incoming alert from SEARF.
    ///
    /// - Increments anomaly count
    /// - Severity 5 or War/Betrayal → immediate freeze
    /// - Count ≥ threshold → freeze
    /// - Otherwise → Alert state
    pub fn receive(&mut self, alert: &Alert) {
        self.anomaly_count += 1;
        self.log.push(format!(
            "[{}] threat={} severity={} fp={:016x}",
            self.anomaly_count, alert.threat.label(), alert.severity, alert.fingerprint
        ));

        // Immediate freeze conditions
        if alert.severity >= 5
            || alert.threat == ThreatClass::War
            || alert.threat == ThreatClass::Betrayal
        {
            self.freeze();
            return;
        }

        // Threshold-based freeze
        if self.anomaly_count >= self.anomaly_threshold {
            self.freeze();
            return;
        }

        // Otherwise escalate to Alert if not already blocking
        if self.state == SentinelState::Watching {
            self.state = SentinelState::Alert;
        }
    }

    /// Force the system into FROZEN state — all mutations blocked.
    pub fn freeze(&mut self) {
        self.state = SentinelState::Frozen;
        self.log.push("FREEZE: all mutations blocked".to_string());
    }

    /// Force PANIC mode — critical lockdown, guardian must intervene manually.
    pub fn panic_mode(&mut self) {
        self.state = SentinelState::Panic;
        self.log.push("PANIC: emergency lockdown — guardian intervention required".to_string());
    }

    /// Restore to Watching state — guardian-authorised only.
    /// Clears anomaly count and log.
    pub fn restore(&mut self) {
        self.state         = SentinelState::Watching;
        self.anomaly_count = 0;
        self.log.push("RESTORE: Sentinel reset by guardian".to_string());
    }

    /// True when the Sentinel is blocking all mutations (Frozen or Panic).
    pub fn is_blocking(&self) -> bool {
        matches!(self.state, SentinelState::Frozen | SentinelState::Panic)
    }

    pub fn status(&self) -> &'static str {
        self.state.label()
    }
}

impl Default for Sentinel {
    fn default() -> Self {
        Self::new()
    }
}

// ─── TESTS ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::searf::{Alert, ThreatClass};

    fn war_alert() -> Alert {
        Alert {
            threat:      ThreatClass::War,
            fingerprint: 0xdeadbeef,
            timestamp:   0,
            payload:     "military attack".to_string(),
            severity:    5,
        }
    }

    fn greed_alert() -> Alert {
        Alert {
            threat:      ThreatClass::Greed,
            fingerprint: 0xcafe,
            timestamp:   0,
            payload:     "monetize platform".to_string(),
            severity:    4,
        }
    }

    #[test]
    fn test_new_sentinel_is_watching() {
        let s = Sentinel::new();
        assert_eq!(s.state, SentinelState::Watching);
        assert_eq!(s.frequency_hz, 741);
        assert_eq!(s.anomaly_threshold, 5);
        assert_eq!(s.anomaly_count, 0);
    }

    #[test]
    fn test_single_war_alert_triggers_freeze() {
        let mut s = Sentinel::new();
        s.receive(&war_alert());
        assert_eq!(s.state, SentinelState::Frozen);
        assert!(s.is_blocking());
    }

    #[test]
    fn test_four_greed_alerts_stay_alert() {
        let mut s = Sentinel::new();
        for _ in 0..4 {
            s.receive(&greed_alert());
        }
        assert_eq!(s.state, SentinelState::Alert);
        assert!(!s.is_blocking());
    }

    #[test]
    fn test_five_greed_alerts_freeze() {
        let mut s = Sentinel::new();
        for _ in 0..5 {
            s.receive(&greed_alert());
        }
        assert_eq!(s.state, SentinelState::Frozen);
        assert!(s.is_blocking());
    }

    #[test]
    fn test_is_blocking_true_when_frozen() {
        let mut s = Sentinel::new();
        s.freeze();
        assert!(s.is_blocking());
    }

    #[test]
    fn test_is_blocking_true_when_panic() {
        let mut s = Sentinel::new();
        s.panic_mode();
        assert!(s.is_blocking());
        assert_eq!(s.state, SentinelState::Panic);
    }

    #[test]
    fn test_is_blocking_false_when_watching() {
        let s = Sentinel::new();
        assert!(!s.is_blocking());
    }

    #[test]
    fn test_restore_resets_to_watching() {
        let mut s = Sentinel::new();
        s.receive(&war_alert());
        assert!(s.is_blocking());
        s.restore();
        assert_eq!(s.state, SentinelState::Watching);
        assert_eq!(s.anomaly_count, 0);
    }

    #[test]
    fn test_log_records_events() {
        let mut s = Sentinel::new();
        s.receive(&greed_alert());
        assert!(!s.log.is_empty());
    }

    #[test]
    fn test_frequency_is_741() {
        let s = Sentinel::new();
        assert_eq!(s.frequency_hz, 741);
    }

    #[test]
    fn test_status_labels() {
        let mut s = Sentinel::new();
        assert_eq!(s.status(), "Watching");
        s.receive(&greed_alert());
        assert_eq!(s.status(), "Alert");
        s.freeze();
        assert_eq!(s.status(), "Frozen");
    }
}
