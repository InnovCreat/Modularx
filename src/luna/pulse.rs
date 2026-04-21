// Template 067 — 639 Hz Pulse
// Template 029 — Geometric Memory (PHI, SYMMETRY_THRESHOLD)

pub const RESONANCE_FREQ:       f64 = 639.0;
pub const COHERENCE_THRESHOLD:  f64 = 0.7;
pub const SYMMETRY_THRESHOLD:   f64 = 0.7;   // même seuil — intentionnel
pub const PHI:                  f64 = 1.618_033_988_749_895;

/// CanResonate — cohérence suffisante pour tenir la fréquence
pub fn can_resonate(coherence: f64) -> bool {
    coherence > COHERENCE_THRESHOLD
}

/// CanDetectError — symétrie brisée = erreur visible avant lecture
pub fn can_detect_error(symmetry: f64) -> bool {
    symmetry < SYMMETRY_THRESHOLD
}

/// Valeur du sinus à 639 Hz au timestamp donné (ms)
pub fn pulse_at_639(timestamp_ms: f64) -> f64 {
    (2.0 * std::f64::consts::PI * RESONANCE_FREQ * timestamp_ms / 1000.0).sin()
}

/// Angle orbital (0–360°) → pan stéréo (-1.0 à +1.0)
/// 90° = droite · 270° = gauche
pub fn angle_to_pan(angle_deg: f64) -> f64 {
    angle_deg.to_radians().sin()
}
