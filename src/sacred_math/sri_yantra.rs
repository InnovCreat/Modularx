// ─────────────────────────────────────────────
// VERITAS HORTUS · sacred_math · sri_yantra.rs
// Architecte  : Isabel Sigouin (InnovCreat)
// Registre    : Mathématique
// Fonction    : Comprendre
// Gravité     : Forme
// Révolution  : interne
// Souveraineté: nulle
// Chaîne      : ─
// Covenant    : Jamais guerre · Jamais cupidité
//               Toujours connaissance · Toujours amour
// Constitution v1.0 — Article IV · Langage v1.4 — Deux Centres
// ─────────────────────────────────────────────

use bevy::prelude::Vec2;

/// Sri Yantra — 4 Shiva triangles (↑) + 5 Shakti triangles (↓) + Bindu
pub struct SriYantra {
    pub triangles:     Vec<[Vec2; 3]>,
    pub intersections: Vec<Vec2>,
    pub lotus_petals:  Vec<Vec2>,
    pub bindu:         Vec2,
}

impl SriYantra {
    pub fn generate(radius: f32) -> Self {
        let triangles     = build_triangles(radius);
        let intersections = compute_intersections(&triangles);
        let lotus_petals  = lotus_ring(radius * 1.2, 16);
        Self {
            triangles,
            intersections,
            lotus_petals,
            bindu: Vec2::ZERO,
        }
    }
}

fn build_triangles(r: f32) -> Vec<[Vec2; 3]> {
    let mut out = Vec::with_capacity(9);
    // 4 Shiva (upward ↑)
    for i in 0..4 {
        out.push(upward_triangle(r * (0.95 - i as f32 * 0.12)));
    }
    // 5 Shakti (downward ↓)
    for i in 0..5 {
        out.push(downward_triangle(r * (0.90 - i as f32 * 0.10)));
    }
    out
}

fn upward_triangle(r: f32) -> [Vec2; 3] {
    use std::f32::consts::TAU;
    [
        Vec2::new(0.0, r),
        Vec2::new(-r * (TAU / 3.0).sin(), -r * 0.5),
        Vec2::new( r * (TAU / 3.0).sin(), -r * 0.5),
    ]
}

fn downward_triangle(r: f32) -> [Vec2; 3] {
    let [a, b, c] = upward_triangle(r);
    [-a, -b, -c]
}

fn compute_intersections(triangles: &[[Vec2; 3]]) -> Vec<Vec2> {
    let mut pts = Vec::new();
    for (i, t1) in triangles.iter().enumerate() {
        for t2 in triangles.iter().skip(i + 1) {
            for e1 in edges(t1) {
                for e2 in edges(t2) {
                    if let Some(p) = segment_intersect(e1, e2) {
                        pts.push(p);
                    }
                }
            }
        }
    }
    pts
}

fn edges(tri: &[Vec2; 3]) -> [[Vec2; 2]; 3] {
    [[tri[0], tri[1]], [tri[1], tri[2]], [tri[2], tri[0]]]
}

fn segment_intersect([p1, p2]: [Vec2; 2], [p3, p4]: [Vec2; 2]) -> Option<Vec2> {
    let d1 = p2 - p1;
    let d2 = p4 - p3;
    let cross = d1.x * d2.y - d1.y * d2.x;
    if cross.abs() < 1e-6 { return None; }
    let t = ((p3.x - p1.x) * d2.y - (p3.y - p1.y) * d2.x) / cross;
    let u = ((p3.x - p1.x) * d1.y - (p3.y - p1.y) * d1.x) / cross;
    if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
        Some(p1 + d1 * t)
    } else {
        None
    }
}

fn lotus_ring(r: f32, n: u32) -> Vec<Vec2> {
    (0..n)
        .map(|i| {
            let a = std::f32::consts::TAU * i as f32 / n as f32;
            Vec2::new(r * a.cos(), r * a.sin())
        })
        .collect()
}
