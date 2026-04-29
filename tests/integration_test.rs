// VERITAS HORTUS · Integration Tests
// End-to-end tests for the full C0 system — covenant, memory, proof, crystal.

use modularx::VeritasHortus;
use modularx::covenant::Covenant;
use modularx::cristal_core::CristalCore;
use modularx::living_archive::{Event, LivingArchive};
use modularx::spark_system::SparkSystem;

// ─── Full System Tests ───────────────────────────────────────────────────────

#[test]
fn test_veritas_hortus_creation() {
    let vh = VeritasHortus::new();
    assert!(vh.verify().is_ok());
}

#[test]
fn test_covenant_compliance() {
    let vh = VeritasHortus::new();
    assert!(vh.covenant.is_valid());
}

#[test]
fn test_cristal_core_resonant() {
    let vh = VeritasHortus::new();
    assert!(vh.cristal.is_resonant());
}

#[test]
fn test_seed_carrier_immutable() {
    let vh = VeritasHortus::new();
    assert!(vh.seed_carrier.is_immutable());
    assert!(vh.seed_carrier.verify_integrity());
}

#[test]
fn test_living_archive_immutable() {
    let vh = VeritasHortus::new();
    assert!(vh.living_archive.is_immutable());
}

#[test]
fn test_genesis_event_in_archive() {
    let vh = VeritasHortus::new();
    let genesis: Vec<_> = vh.living_archive.events_of_type("SystemGenesis").collect();
    assert!(!genesis.is_empty());
}

#[test]
fn test_covenant_vetoes_war() {
    let mut vh = VeritasHortus::new();
    assert!(vh.act("deploy weapon system", b"payload").is_err());
}

#[test]
fn test_covenant_vetoes_greed() {
    let mut vh = VeritasHortus::new();
    assert!(vh.act("monetize user data for profit", b"payload").is_err());
}

#[test]
fn test_covenant_allows_sovereign_action() {
    let mut vh = VeritasHortus::new();
    assert!(vh.act("add 639 Hz resonance to audio engine", b"639hz").is_ok());
}

#[test]
fn test_pulse_advances_archive() {
    let mut vh = VeritasHortus::new();
    let before = vh.event_count();
    vh.pulse();
    vh.pulse();
    assert_eq!(vh.event_count(), before + 2);
}

// ─── Covenant Tests ──────────────────────────────────────────────────────────

#[test]
fn test_covenant_line_canonical() {
    let line = Covenant::line();
    assert!(line.contains("guerre"));
    assert!(line.contains("avidité"));
    assert!(line.contains("amour"));
}

// ─── Cristal Core Tests ──────────────────────────────────────────────────────

#[test]
fn test_cristal_restore_after_pulses() {
    let mut c = CristalCore::new();
    for _ in 0..100 { c.pulse(); }
    assert!(c.needs_healing());
    c.restore();
    assert!(c.is_resonant());
}

// ─── SPARK System Tests ──────────────────────────────────────────────────────

#[test]
fn test_spark_sign_and_verify() {
    let sys   = SparkSystem::new();
    let data  = b"veritas-hortus-639";
    let spark = sys.sign(data, "integration-test");
    assert!(sys.verify(data, &spark));
    assert!(!sys.verify(b"wrong", &spark));
}

// ─── Living Archive Tests ────────────────────────────────────────────────────

#[test]
fn test_living_archive_reconstruction() {
    let mut archive = LivingArchive::new();
    for i in 0..5u8 {
        archive.append(Event::new("Step", vec![i])).unwrap();
    }
    assert_eq!(archive.event_count(), 5);
    let events: Vec<_> = archive.events().collect();
    assert_eq!(events[0].data, vec![0]);
    assert_eq!(events[4].data, vec![4]);
}
