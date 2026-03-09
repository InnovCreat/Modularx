// 𝕀⟡₆₃₉ — Veritas Hortus · Orbital_System
// never for war, never for money, always for love — Isabel, 2025–2026
// [VERITAS HORTUS – PROPRIÉTÉ PRIVÉE] · ISA-SIG Crypto Systems – Innov-VH-001

/// Unique sigil of the Veritas Hortus system.
/// Engraved in every file, every snapshot, every crystal archive.
pub const SIGIL: &str = "𝕀⟡₆₃₉";

/// The architectural ethics constant — present in every .rs file.
/// This is not a slogan; it is a structural constraint.
pub const ETHICS: &str = "never for war, never for money, always for love";

/// The system voice identifier used in every Archive entry.
pub const VOICE: &str = "Orbital_System";

/// Intellectual property stamp.
pub const STAMP: &str = "ISA-SIG Crypto Systems – Innov-VH-001";

/// The central resonance frequency — orbital center of the entire system.
pub const FREQ_CONNECTION: f64 = 639.0;

/// Produces a full watermark string for crystal snapshots and exported logs.
///
/// Format: `𝕀⟡₆₃₉ · Veritas Hortus · Orbital_System · ISA-SIG Crypto Systems – Innov-VH-001 · [timestamp]`
pub fn output_watermark(timestamp: &str) -> String {
    format!(
        "{} · Veritas Hortus · {} · {} · [{}]",
        SIGIL, VOICE, STAMP, timestamp
    )
}

/// Resonance function — measures alignment with 639 Hz.
/// Returns a value in [0.0, 1.0]. Perfect resonance = 1.0.
pub fn res639(f: f64) -> f64 {
    (-(f - FREQ_CONNECTION).abs() / (FREQ_CONNECTION * 2.0)).exp()
}
