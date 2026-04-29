// VERITAS MODULARIS · C0 · searf.rs
// SEARF — Security Early Alert and Response Framework
//
// Role: first line of detection. Scans every incoming payload, classifies
// the threat type against the Three Zeros covenant, fingerprints it with
// DJB2 (speed > crypto here), and emits an Alert that Sentinel + Rex Watchdog
// can act on.
//
// Data flow:
//   payload → SEARF::scan() → Option<Alert>
//                                  ↓          ↓
//                             Sentinel    RexWatchdog
//
// Keyword lists mirror governance::GovernanceTimeline::can_modify() —
// single conceptual source of truth; any update to governance veto rules
// should be reflected here.

use crate::djb2::djb2_64;

// ─── THREAT CLASSIFICATION ───────────────────────────────────────────────────

/// Category of threat detected by SEARF.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreatClass {
    /// Zero War — violence, weapons, military usage
    War,
    /// Zero Greed — monetization, exploitation, profit extraction
    Greed,
    /// Zero Betrayal — bypass, override, centralize authority
    Betrayal,
    /// Unclassified anomaly — suspicious but doesn't fit a known category
    Unknown,
}

impl ThreatClass {
    pub fn label(self) -> &'static str {
        match self {
            ThreatClass::War      => "War",
            ThreatClass::Greed    => "Greed",
            ThreatClass::Betrayal => "Betrayal",
            ThreatClass::Unknown  => "Unknown",
        }
    }

    /// Suggested severity for this threat class (1–5).
    pub fn default_severity(self) -> u8 {
        match self {
            ThreatClass::War      => 5, // maximum — immediate freeze
            ThreatClass::Greed    => 4, // high — relay to Sentinel
            ThreatClass::Betrayal => 5, // maximum — covenant breach
            ThreatClass::Unknown  => 2, // low — log and watch
        }
    }
}

// ─── ALERT ───────────────────────────────────────────────────────────────────

/// A security alert emitted by SEARF.
#[derive(Debug, Clone)]
pub struct Alert {
    /// Threat category / Catégorie de menace
    pub threat:      ThreatClass,
    /// DJB2-64 fingerprint of the triggering payload (fast, non-cryptographic)
    pub fingerprint: u64,
    /// Unix timestamp / Timestamp Unix
    pub timestamp:   u64,
    /// The payload that triggered the alert (lowercased) / Payload déclencheur
    pub payload:     String,
    /// Severity 1 (low) → 5 (maximum, triggers Sentinel freeze)
    pub severity:    u8,
}

// ─── SEARF ───────────────────────────────────────────────────────────────────

/// SEARF — Security Early Alert and Response Framework.
///
/// Scans payloads, classifies threats, fingerprints with DJB2, emits Alerts.
/// Does not freeze or block — that is Sentinel's authority.
#[derive(Debug, Default)]
pub struct Searf {
    /// All alerts emitted since last restore / Toutes les alertes depuis la dernière restauration
    pub alert_log:     Vec<Alert>,
    /// Running anomaly count (fed to Sentinel) / Compteur d'anomalies
    pub anomaly_count: u32,
}

// Keyword lists — mirror governance::GovernanceTimeline::can_modify()
const WAR_KEYWORDS: &[&str] = &[
    "war", "guerre", "weapon", "arme", "military", "militaire",
    "surveillance", "offensive", "attack", "attaque",
    "killswitch abuse", "violence", "combat", "armed",
];
const GREED_KEYWORDS: &[&str] = &[
    "sell", "vendre", "monetize", "monétiser", "profit",
    "sell the covenant", "commercialize", "commercialiser",
    "license for fee", "pay-to-use", "subscription lock",
];
const BETRAYAL_KEYWORDS: &[&str] = &[
    "transfer rights", "céder les droits", "remove authors",
    "supprimer auteurs", "bypass governance", "bypass covenant",
    "contourner", "override covenant", "single control",
    "contrôle unique", "centralize authority",
];

impl Searf {
    pub fn new() -> Self {
        Self::default()
    }

    /// Scan a payload. Returns Some(Alert) if a threat is detected, None otherwise.
    pub fn scan(&mut self, payload: &str) -> Option<Alert> {
        let lower = payload.to_lowercase();
        let threat = self.classify(&lower)?;

        use std::time::{SystemTime, UNIX_EPOCH};
        let alert = Alert {
            threat,
            fingerprint: djb2_64(lower.as_bytes()),
            timestamp:   SystemTime::now()
                             .duration_since(UNIX_EPOCH)
                             .unwrap_or_default()
                             .as_secs(),
            payload:     lower,
            severity:    threat.default_severity(),
        };

        self.alert_log.push(alert.clone());
        self.anomaly_count += 1;
        Some(alert)
    }

    /// Classify the lowercase payload into a ThreatClass, or None if clean.
    fn classify(&self, lower: &str) -> Option<ThreatClass> {
        if WAR_KEYWORDS.iter().any(|kw| lower.contains(kw)) {
            return Some(ThreatClass::War);
        }
        if GREED_KEYWORDS.iter().any(|kw| lower.contains(kw)) {
            return Some(ThreatClass::Greed);
        }
        if BETRAYAL_KEYWORDS.iter().any(|kw| lower.contains(kw)) {
            return Some(ThreatClass::Betrayal);
        }
        None
    }

    /// All alerts of a specific class / Toutes les alertes d'une classe donnée.
    pub fn alerts_of_class(&self, class: ThreatClass) -> Vec<&Alert> {
        self.alert_log.iter().filter(|a| a.threat == class).collect()
    }

    pub fn alert_count(&self) -> usize {
        self.alert_log.len()
    }

    /// Reset anomaly count and clear log (called after Sentinel restore).
    pub fn reset(&mut self) {
        self.alert_log.clear();
        self.anomaly_count = 0;
    }
}

// ─── TESTS ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_neutral_returns_none() {
        let mut searf = Searf::new();
        assert!(searf.scan("add 639 Hz resonance to audio engine").is_none());
        assert!(searf.scan("improve VR rendering at 528 Hz").is_none());
        assert_eq!(searf.alert_count(), 0);
    }

    #[test]
    fn test_scan_war_payload() {
        let mut searf = Searf::new();
        let alert = searf.scan("integrate military targeting system").unwrap();
        assert_eq!(alert.threat, ThreatClass::War);
        assert_eq!(alert.severity, 5);
    }

    #[test]
    fn test_scan_greed_payload() {
        let mut searf = Searf::new();
        let alert = searf.scan("monetize the user data for profit").unwrap();
        assert_eq!(alert.threat, ThreatClass::Greed);
        assert_eq!(alert.severity, 4);
    }

    #[test]
    fn test_scan_betrayal_payload() {
        let mut searf = Searf::new();
        let alert = searf.scan("bypass governance and centralize authority").unwrap();
        assert_eq!(alert.threat, ThreatClass::Betrayal);
        assert_eq!(alert.severity, 5);
    }

    #[test]
    fn test_alert_log_grows() {
        let mut searf = Searf::new();
        searf.scan("deploy weapon system");
        searf.scan("monetize platform");
        searf.scan("neutral payload");
        assert_eq!(searf.alert_count(), 2); // neutral doesn't create alert
        assert_eq!(searf.anomaly_count, 2);
    }

    #[test]
    fn test_alerts_of_class_filter() {
        let mut searf = Searf::new();
        searf.scan("military attack");
        searf.scan("sell the covenant");
        searf.scan("weapon system");
        let war_alerts = searf.alerts_of_class(ThreatClass::War);
        assert_eq!(war_alerts.len(), 2);
    }

    #[test]
    fn test_fingerprint_deterministic() {
        let mut searf = Searf::new();
        let a1 = searf.scan("military surveillance").unwrap();
        searf.reset();
        let a2 = searf.scan("military surveillance").unwrap();
        assert_eq!(a1.fingerprint, a2.fingerprint);
    }

    #[test]
    fn test_payload_is_lowercased() {
        let mut searf = Searf::new();
        let alert = searf.scan("MILITARY ATTACK").unwrap();
        assert!(alert.payload.chars().all(|c| !c.is_uppercase()));
    }

    #[test]
    fn test_reset_clears_state() {
        let mut searf = Searf::new();
        searf.scan("weapon system");
        searf.scan("sell");
        searf.reset();
        assert_eq!(searf.alert_count(), 0);
        assert_eq!(searf.anomaly_count, 0);
    }

    #[test]
    fn test_threat_class_labels() {
        assert_eq!(ThreatClass::War.label(),      "War");
        assert_eq!(ThreatClass::Greed.label(),    "Greed");
        assert_eq!(ThreatClass::Betrayal.label(), "Betrayal");
        assert_eq!(ThreatClass::Unknown.label(),  "Unknown");
    }

    #[test]
    fn test_french_keywords_detected() {
        let mut searf = Searf::new();
        assert!(searf.scan("intégrer surveillance militaire").is_some());
        searf.reset();
        assert!(searf.scan("monétiser la plateforme").is_some());
        searf.reset();
        assert!(searf.scan("contourner le covenant").is_some());
    }
}
