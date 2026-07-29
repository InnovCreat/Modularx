// ─────────────────────────────────────────────
// VERITAS HORTUS · sacred_math · geometry.rs
// Architecte : Isabel Sigouin (InnovCreat)
// Registre   : Mathématique
// Fonction   : Comprendre
// Covenant   : Jamais guerre · Jamais cupidité
//              Toujours connaissance · Toujours amour
// Constitution v1.0 — Article IV (Organes)
// ─────────────────────────────────────────────

use bevy::prelude::Vec2;

pub const PHI: f32 = 1.618_033_988;

pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0u32, 1u32);
            for _ in 2..=n {
                (a, b) = (b, a.saturating_add(b));
            }
            b
        }
    }
}

/// Logarithmic spiral based on φ: r = scale * φ^(t / 2π)
pub fn golden_spiral_point(t: f32, scale: f32) -> Vec2 {
    let r = scale * PHI.powf(t / std::f32::consts::TAU);
    Vec2::new(r * t.cos(), r * t.sin())
}

/// Lemniscate of Bernoulli — ternary XYZ𝕋 grid parametric form
pub fn lemniscate_point(t: f32, a: f32) -> Vec2 {
    let denom = 1.0 + t.sin() * t.sin();
    Vec2::new(a * t.cos() / denom, a * t.sin() * t.cos() / denom)
}
