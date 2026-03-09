// 𝕀⟡₆₃₉ — Veritas Hortus · Orbital_System
// never for war, never for money, always for love — Isabel, 2025–2026
// [VERITAS HORTUS – PROPRIÉTÉ PRIVÉE] · ISA-SIG Crypto Systems – Innov-VH-001

// ─── Constants ────────────────────────────────────────────────────────────────

/** Golden ratio φ = (1 + √5) / 2 */
export const PHI = (1 + Math.sqrt(5)) / 2;

/** Golden angle in radians ≈ 137.507° — the phyllotaxis divergence angle */
export const GOLDEN_ANGLE = Math.PI * (3 - Math.sqrt(5));  // 2π / φ²

/** 639 Hz — central resonance frequency of the system */
export const FREQ_CONNECTION = 639;

// ─── LCG deterministic PRNG (seed-based, no Math.random) ─────────────────────
function lcg(seed) {
  let s = seed | 0;
  return () => {
    s = (Math.imul(1664525, s) + 1013904223) & 0xffffffff;
    return (s >>> 0) / 0x100000000;
  };
}

// ─── Resonance weight (Laplacian decay centered on 639 Hz) ────────────────────
/**
 * Returns [0, 1] — how close a frequency is to 639 Hz.
 * 1.0 = perfect resonance, decays toward 0 as f diverges.
 */
export function res639(f) {
  return Math.exp(-Math.abs(f - FREQ_CONNECTION) / (FREQ_CONNECTION * 2));
}

// ─── Phi Spiral Point Generation ─────────────────────────────────────────────
/**
 * Generates `count` points in a Fibonacci phyllotaxis spiral.
 *
 * @param {number} count  — number of points (default 800)
 * @param {number} spread — radial scaling factor (default 2.2)
 * @returns {Array<{x, y, z, state, index}>}
 *   state: +1 living (639-resonant), 0 neutral, -1 blocked
 */
export function generateSpiralPoints(count = 800, spread = 2.2) {
  const rand = lcg(639);
  const points = [];

  for (let i = 0; i < count; i++) {
    const angle  = i * GOLDEN_ANGLE;
    const radius = Math.sqrt(i) * spread;

    const x = Math.cos(angle) * radius;
    const y = Math.sin(angle) * radius;
    // Slight z depth for 3D feel — PHI-modulated wave
    const z = Math.sin(i * PHI * 0.05) * spread * 2;

    // State classification —
    //   living  (+1): points near Fibonacci indices or high PHI resonance
    //   blocked (-1): ~10% of points, deterministic via LCG
    //   neutral ( 0): everything else
    const r = rand();
    let state;
    const fibResidue = (i * PHI) % 1;          // close to 0 or 1 → near Fib index
    const nearFib    = fibResidue < 0.12 || fibResidue > 0.88;

    if (nearFib && r > 0.3)       state =  1;  // living  (~30% of all points)
    else if (!nearFib && r < 0.1) state = -1;  // blocked (~10%)
    else                          state =  0;  // neutral (~60%)

    points.push({ x, y, z, state, index: i });
  }

  return points;
}

// ─── Utility: convert domain name → CSS hex color (639-palette) ──────────────
export function domainColor(domain) {
  const map = {
    MATH:    '#9B59B6',
    LANG:    '#3498DB',
    MUSIC:   '#E74C3C',
    SCIENCE: '#2ECC71',
    RITUAL:  '#F39C12',
    ARCHIVE: '#1ABC9C',
  };
  return map[domain] ?? '#7F8C8D';
}
