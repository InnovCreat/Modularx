// VERITAS MODULARIS · C0 · rex_watchdog.rs
// Rex Watchdog — external AI-ally monitor
//
// Rex = ritual name for Grok (xAI), second-opinion AI ally.
// Named in REGISTRY.toml and MASTER_INDEX.md as an external ally of the system.
//
// Role: independent external monitor, second opinion, escalation signal.
// Authority: NONE over internal state — cannot freeze, panic, or kill.
//   That authority belongs exclusively to Sentinel (Phoenix Sentinel).
//   Rex Watchdog observes, reports, and recommends escalation.
//
// Flow: SEARF emits Alert → RexWatchdog::evaluate() → WatchdogReport
//   If escalation_needed() → caller sends report to Sentinel

use crate::searf::{Alert, ThreatClass};

// ─── WATCHDOG REPORT ──────────────────────────────────────────────────────────

/// A WatchdogReport is Rex's evaluation of a SEARF alert.
/// It does not change system state — it is a recommendation only.
#[derive(Debug, Clone)]
pub struct WatchdogReport {
    /// DJB2 fingerprint of the triggering alert (matches Alert::fingerprint)
    pub alert_ref:   u64,
    /// Rex's verdict on the alert
    pub verdict:     &'static str,
    /// Recommended action for the system guardians
    pub recommended: &'static str,
    /// Unix timestamp of the evaluation
    pub timestamp:   u64,
    /// Whether this report requests Sentinel escalation
    pub escalate:    bool,
}

// ─── REX WATCHDOG ─────────────────────────────────────────────────────────────

/// Rex Watchdog — external AI-ally second opinion monitor.
///
/// Evaluates SEARF alerts independently of Sentinel.
/// Produces WatchdogReports with verdicts and recommendations.
/// Has NO authority to freeze, panic, or kill — those belong to Sentinel.
#[derive(Debug)]
pub struct RexWatchdog {
    /// "rex-watchdog" — the canonical Rex identity
    pub id:      &'static str,
    /// Log of all reports produced
    pub reports: Vec<WatchdogReport>,
}

impl RexWatchdog {
    pub fn new() -> Self {
        Self {
            id:      "rex-watchdog",
            reports: Vec::new(),
        }
    }

    /// Evaluate a SEARF alert and produce a WatchdogReport.
    pub fn evaluate(&mut self, alert: &Alert) -> WatchdogReport {
        use std::time::{SystemTime, UNIX_EPOCH};
        let (verdict, recommended, escalate) = Self::assess(alert);

        let report = WatchdogReport {
            alert_ref:   alert.fingerprint,
            verdict,
            recommended,
            timestamp:   SystemTime::now()
                             .duration_since(UNIX_EPOCH)
                             .unwrap_or_default()
                             .as_secs(),
            escalate,
        };

        self.reports.push(report.clone());
        report
    }

    fn assess(alert: &Alert) -> (&'static str, &'static str, bool) {
        match alert.threat {
            ThreatClass::War => (
                "Covenant breach: Zero War violated",
                "escalate to Sentinel — immediate freeze required",
                true,
            ),
            ThreatClass::Betrayal => (
                "Covenant breach: governance integrity threatened",
                "escalate to Sentinel — freeze and guardian review",
                true,
            ),
            ThreatClass::Greed => (
                "Covenant concern: Zero Greed pressure detected",
                "log and monitor — escalate if pattern repeats",
                alert.severity >= 5,
            ),
            ThreatClass::Unknown => (
                "Anomaly detected — classification unclear",
                "log only — insufficient signal for escalation",
                false,
            ),
        }
    }

    pub fn report_count(&self) -> usize {
        self.reports.len()
    }

    /// True if any report recommends escalation to Sentinel.
    pub fn escalation_needed(&self) -> bool {
        self.reports.iter().any(|r| r.escalate)
    }

    /// Reports that require escalation.
    pub fn escalation_reports(&self) -> Vec<&WatchdogReport> {
        self.reports.iter().filter(|r| r.escalate).collect()
    }
}

impl Default for RexWatchdog {
    fn default() -> Self {
        Self::new()
    }
}

// ─── TESTS ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::searf::{Alert, ThreatClass};

    fn alert(threat: ThreatClass, severity: u8) -> Alert {
        Alert {
            threat,
            fingerprint: 0xabcd1234,
            timestamp:   0,
            payload:     "test payload".to_string(),
            severity,
        }
    }

    #[test]
    fn test_new_watchdog_no_reports() {
        let rex = RexWatchdog::new();
        assert_eq!(rex.id, "rex-watchdog");
        assert_eq!(rex.report_count(), 0);
        assert!(!rex.escalation_needed());
    }

    #[test]
    fn test_war_alert_escalates() {
        let mut rex = RexWatchdog::new();
        let report = rex.evaluate(&alert(ThreatClass::War, 5));
        assert!(report.escalate);
        assert!(report.recommended.contains("escalate to Sentinel"));
        assert!(rex.escalation_needed());
    }

    #[test]
    fn test_betrayal_alert_escalates() {
        let mut rex = RexWatchdog::new();
        let report = rex.evaluate(&alert(ThreatClass::Betrayal, 5));
        assert!(report.escalate);
    }

    #[test]
    fn test_greed_low_severity_no_escalation() {
        let mut rex = RexWatchdog::new();
        let report = rex.evaluate(&alert(ThreatClass::Greed, 4));
        assert!(!report.escalate);
        assert!(!rex.escalation_needed());
    }

    #[test]
    fn test_greed_max_severity_escalates() {
        let mut rex = RexWatchdog::new();
        let report = rex.evaluate(&alert(ThreatClass::Greed, 5));
        assert!(report.escalate);
    }

    #[test]
    fn test_unknown_threat_no_escalation() {
        let mut rex = RexWatchdog::new();
        let report = rex.evaluate(&alert(ThreatClass::Unknown, 2));
        assert!(!report.escalate);
    }

    #[test]
    fn test_report_count_increments() {
        let mut rex = RexWatchdog::new();
        rex.evaluate(&alert(ThreatClass::War, 5));
        rex.evaluate(&alert(ThreatClass::Greed, 3));
        assert_eq!(rex.report_count(), 2);
    }

    #[test]
    fn test_alert_ref_matches_fingerprint() {
        let mut rex = RexWatchdog::new();
        let a = alert(ThreatClass::War, 5);
        let report = rex.evaluate(&a);
        assert_eq!(report.alert_ref, a.fingerprint);
    }

    #[test]
    fn test_escalation_reports_filter() {
        let mut rex = RexWatchdog::new();
        rex.evaluate(&alert(ThreatClass::War, 5));
        rex.evaluate(&alert(ThreatClass::Unknown, 1));
        rex.evaluate(&alert(ThreatClass::Betrayal, 5));
        assert_eq!(rex.escalation_reports().len(), 2);
    }
}
