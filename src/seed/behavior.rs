// SEED CARRIER — behavior.rs
// Interaction and spatio-temporal behavior logic
// Logique de comportement d'interaction et spatio-temporel

use super::types::{Seed, Position};
use super::scaling::Scale;

// ─── SPATIAL BEHAVIOR ─────────────────────────────────────────────────────────

/// Move a seed to absolute coordinates, clamped to bounds.
/// Déplace un seed vers des coordonnées absolues, limités aux bornes.
pub fn move_to(seed: &mut Seed, x: f64, y: f64, z: f64) {
    let p = &mut seed.position;
    p.x = clamp(x, p.bounds[0].0, p.bounds[0].1);
    p.y = clamp(y, p.bounds[1].0, p.bounds[1].1);
    p.z = clamp(z, p.bounds[2].0, p.bounds[2].1);
    seed.timeline.record("move");
    seed.seal.invalidate();
}

/// Advance orbital angle by delta_theta (radians), update x/y from orbital math.
/// Avance l'angle orbital de delta_theta (radians), met à jour x/y depuis la géométrie orbitale.
///
/// Uses the rationalized Bernoulli lemniscate for orbital path:
/// Utilise la lemniscate de Bernoulli rationalisée pour le chemin orbital :
/// x(t) = r · cos(t)/(1+sin²(t))
/// y(t) = r · sin(t)cos(t)/(1+sin²(t))
pub fn orbit(seed: &mut Seed, delta_theta: f64) {
    let p = &mut seed.position;
    p.theta += delta_theta;
    let t = p.theta;
    let r = p.radius;
    let denom = 1.0 + t.sin() * t.sin();
    p.x = r * t.cos() / denom;
    p.y = r * t.sin() * t.cos() / denom;
    seed.timeline.record("orbit");
    seed.seal.invalidate();
}

/// Distance between two positions / Distance entre deux positions
pub fn distance(a: &Position, b: &Position) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    // integer arithmetic approximation (no sqrt) — sovereign, no float sqrt needed
    // approximation arithmétique entière — souverain, pas de sqrt flottante nécessaire
    (dx * dx + dy * dy + dz * dz).abs().sqrt()
}

// ─── TEMPORAL BEHAVIOR ────────────────────────────────────────────────────────

/// Advance the seed timeline with an event label.
/// Avance la timeline du seed avec une étiquette d'événement.
pub fn advance_time(seed: &mut Seed, label: impl Into<String>) {
    seed.timeline.record(label);
    seed.state.op_count += 1;
}

/// Returns how many seconds have elapsed since seed creation.
/// Retourne le nombre de secondes écoulées depuis la création du seed.
pub fn elapsed_seconds(seed: &Seed) -> u64 {
    seed.timeline.current_time.saturating_sub(seed.timeline.created_at)
}

// ─── SCALE BEHAVIOR ──────────────────────────────────────────────────────────

/// Transition the seed to a different operating scale.
/// Fait passer le seed à une échelle d'opération différente.
pub fn rescale(seed: &mut Seed, to: Scale, reason: impl Into<String>) {
    let current_value = seed.scale.to_unity(1.0);
    seed.scale.transition(to, current_value, reason);
    seed.timeline.record("rescale");
    seed.seal.invalidate();
}

// ─── INTERACTION BEHAVIOR ─────────────────────────────────────────────────────

/// Interaction event between two seeds or a seed and an external target.
/// Événement d'interaction entre deux seeds ou un seed et une cible externe.
#[derive(Debug, Clone)]
pub struct InteractionEvent {
    /// Source seed ID / ID du seed source
    pub from:     String,
    /// Target module or seed ID / ID du module cible ou seed
    pub to:       String,
    /// Signal payload / Payload du signal
    pub signal:   String,
    /// Unix timestamp / Timestamp Unix
    pub time:     u64,
    /// Whether the interaction was accepted / Si l'interaction a été acceptée
    pub accepted: bool,
}

/// Process an outgoing interaction from a seed toward a target.
/// Traite une interaction sortante d'un seed vers une cible.
pub fn interact(seed: &mut Seed, target: impl Into<String>, signal: impl Into<String>) -> InteractionEvent {
    use std::time::{SystemTime, UNIX_EPOCH};
    let signal_str = signal.into();
    let accepted = seed.seal.covenant_check(&signal_str);
    let event = InteractionEvent {
        from:     seed.id.clone(),
        to:       target.into(),
        signal:   signal_str,
        time:     SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        accepted,
    };
    seed.timeline.record(if accepted { "interact:accepted" } else { "interact:vetoed" });
    seed.state.op_count += 1;
    event
}

// ─── HELPER ──────────────────────────────────────────────────────────────────

fn clamp(v: f64, lo: f64, hi: f64) -> f64 {
    if v < lo { lo } else if v > hi { hi } else { v }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seed::types::Seed;

    fn make_seed() -> Seed {
        Seed::new("test-001", "TestSeed")
    }

    #[test]
    fn test_move_to_updates_position() {
        let mut s = make_seed();
        move_to(&mut s, 10.0, 20.0, 30.0);
        assert!((s.position.x - 10.0).abs() < f64::EPSILON);
        assert!((s.position.y - 20.0).abs() < f64::EPSILON);
        assert!((s.position.z - 30.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_move_clamps_to_bounds() {
        let mut s = make_seed();
        move_to(&mut s, 99999.0, 0.0, 0.0);
        assert_eq!(s.position.x, 1000.0); // clamped to bound
    }

    #[test]
    fn test_orbit_updates_xy() {
        let mut s = make_seed();
        orbit(&mut s, std::f64::consts::PI / 4.0);
        // theta changed — x/y should be non-zero
        let total = s.position.x.abs() + s.position.y.abs();
        assert!(total > 0.0);
    }

    #[test]
    fn test_advance_time_increments_op_count() {
        let mut s = make_seed();
        advance_time(&mut s, "test-event");
        assert_eq!(s.state.op_count, 1);
    }

    #[test]
    fn test_interact_vetoed_on_war_signal() {
        let mut s = make_seed();
        let event = interact(&mut s, "nexus", "deploy military weapon");
        assert!(!event.accepted);
    }

    #[test]
    fn test_interact_accepted_on_neutral_signal() {
        let mut s = make_seed();
        let event = interact(&mut s, "luna", "stream 639 Hz frequency");
        assert!(event.accepted);
    }

    #[test]
    fn test_move_invalidates_seal() {
        let mut s = make_seed();
        s.seal.seal(b"state");
        assert!(s.seal.valid);
        move_to(&mut s, 1.0, 2.0, 3.0);
        assert!(!s.seal.valid);
    }
}
