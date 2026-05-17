// ════════════════════════════════════════════════════════════════
// SLTQ — Spatial-Linguistic-Temporal-Quantum coordinate space
// 4D orthogonal positioning for perception signals
// Each axis is independent — no destructive mixing
// ════════════════════════════════════════════════════════════════

use crate::{Mark, Signal, Voice, PHI};

// ── SLTQ Coordinates ────────────────────────────────────────────
// Four orthogonal axes forming a crystalline perception space.
//
//   S (Spatial)    — physical/geometric density [0.0, 1.0]
//   L (Linguistic) — semantic/symbolic weight   [0.0, 1.0]
//   T (Temporal)   — time-decay from mark       [0.0, 1.0]
//   Q (Quantum)    — coherence/entanglement     [0.0, 1.0]

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coord {
    pub s: f64,
    pub l: f64,
    pub t: f64,
    pub q: f64,
}

impl Coord {
    pub fn new(s: f64, l: f64, t: f64, q: f64) -> Self {
        Self {
            s: s.clamp(0.0, 1.0),
            l: l.clamp(0.0, 1.0),
            t: t.clamp(0.0, 1.0),
            q: q.clamp(0.0, 1.0),
        }
    }

    pub fn origin() -> Self {
        Self { s: 0.0, l: 0.0, t: 0.0, q: 0.0 }
    }

    pub fn magnitude(&self) -> f64 {
        (self.s * self.s + self.l * self.l + self.t * self.t + self.q * self.q).sqrt()
    }

    pub fn distance(&self, other: &Coord) -> f64 {
        let ds = self.s - other.s;
        let dl = self.l - other.l;
        let dt = self.t - other.t;
        let dq = self.q - other.q;
        (ds * ds + dl * dl + dt * dt + dq * dq).sqrt()
    }

    pub fn scale(&self, factor: f64) -> Self {
        Self::new(
            self.s * factor,
            self.l * factor,
            self.t * factor,
            self.q * factor,
        )
    }

    pub fn phi_compress(&self) -> Self {
        self.scale(1.0 / PHI)
    }

    pub fn phi_expand(&self) -> Self {
        self.scale(PHI)
    }

    pub fn dot(&self, other: &Coord) -> f64 {
        self.s * other.s + self.l * other.l + self.t * other.t + self.q * other.q
    }
}

// ── Positioned Voice ────────────────────────────────────────────
// A Voice placed in SLTQ space — ready for vortex reduction.

#[derive(Debug, Clone, Copy)]
pub struct Positioned {
    pub voice: Voice,
    pub coord: Coord,
}

// ── Projection Engine ───────────────────────────────────────────
// Maps raw signals into SLTQ coordinates based on channel properties.

pub struct Projector {
    current_pulse: u64,
    max_age_pulses: u64,
}

impl Projector {
    pub fn new(max_age_pulses: u64) -> Self {
        Self {
            current_pulse: 0,
            max_age_pulses,
        }
    }

    pub fn set_pulse(&mut self, pulse: u64) {
        self.current_pulse = pulse;
    }

    pub fn project(&self, voice: &Voice) -> Positioned {
        let coord = match voice.signal {
            Signal::Visual { density, .. } => {
                let s = (density as f64 / 1000.0).clamp(0.0, 1.0);
                let t = self.temporal_decay(voice.mark);
                Coord::new(s, 0.0, t, 0.3)
            }
            Signal::Audio { frequency_peak, amplitude } => {
                let s = (amplitude as f64).clamp(0.0, 1.0);
                let q = (frequency_peak as f64 / 1000.0).clamp(0.0, 1.0);
                let t = self.temporal_decay(voice.mark);
                Coord::new(s, 0.0, t, q)
            }
            Signal::Biometric { coherence, pulse_rate } => {
                let q = coherence as f64;
                let s = (pulse_rate as f64 / 200.0).clamp(0.0, 1.0);
                let t = self.temporal_decay(voice.mark);
                Coord::new(s, 0.0, t, q.clamp(0.0, 1.0))
            }
            Signal::Text { token_count, .. } => {
                let l = (token_count as f64 / 100.0).clamp(0.0, 1.0);
                let t = self.temporal_decay(voice.mark);
                Coord::new(0.0, l, t, 0.5)
            }
        };

        Positioned { voice: *voice, coord }
    }

    fn temporal_decay(&self, mark: Mark) -> f64 {
        if self.max_age_pulses == 0 {
            return 1.0;
        }
        let age = self.current_pulse.saturating_sub(mark.pulse);
        let ratio = age as f64 / self.max_age_pulses as f64;
        (1.0 - ratio).clamp(0.0, 1.0)
    }
}

// ── Vortex Reduction ────────────────────────────────────────────
// Compresses positioned signals using PHI-ratio spiral convergence.
// Signals closer to the vortex center have higher salience.

pub struct Vortex {
    pub center: Coord,
    pub compression_steps: u32,
}

impl Vortex {
    pub fn new(center: Coord) -> Self {
        Self {
            center,
            compression_steps: 0,
        }
    }

    pub fn at_origin() -> Self {
        Self::new(Coord::new(0.5, 0.5, 0.5, 0.5))
    }

    pub fn compress(&mut self, points: &[Positioned]) -> Vec<Positioned> {
        self.compression_steps += 1;
        let phi_inv = 1.0 / PHI;

        points.iter().map(|p| {
            let dx = p.coord.s - self.center.s;
            let dy = p.coord.l - self.center.l;
            let dz = p.coord.t - self.center.t;
            let dw = p.coord.q - self.center.q;

            let new_coord = Coord::new(
                self.center.s + dx * phi_inv,
                self.center.l + dy * phi_inv,
                self.center.t + dz * phi_inv,
                self.center.q + dw * phi_inv,
            );

            Positioned {
                voice: p.voice,
                coord: new_coord,
            }
        }).collect()
    }

    pub fn salience(&self, point: &Positioned) -> f64 {
        let dist = self.center.distance(&point.coord);
        1.0 / (1.0 + dist * PHI)
    }

    pub fn top_n(&self, points: &[Positioned], n: usize) -> Vec<Positioned> {
        let mut scored: Vec<(f64, &Positioned)> = points
            .iter()
            .map(|p| (self.salience(p), p))
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().take(n).map(|(_, p)| *p).collect()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Signal;

    #[test]
    fn coord_basic() {
        let c = Coord::new(0.5, 0.5, 0.5, 0.5);
        assert!((c.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn coord_distance() {
        let a = Coord::origin();
        let b = Coord::new(1.0, 0.0, 0.0, 0.0);
        assert!((a.distance(&b) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn coord_phi_compress() {
        let c = Coord::new(1.0, 1.0, 1.0, 1.0);
        let compressed = c.phi_compress();
        assert!((compressed.s - 1.0 / PHI).abs() < 1e-10);
    }

    #[test]
    fn projector_text() {
        let proj = Projector::new(100);
        let voice = Voice {
            mark: Mark { pulse: 0 },
            signal: Signal::Text { token_count: 50, checksum: 0 },
        };
        let pos = proj.project(&voice);
        assert!((pos.coord.l - 0.5).abs() < 1e-10);
        assert_eq!(pos.coord.s, 0.0);
    }

    #[test]
    fn projector_temporal_decay() {
        let mut proj = Projector::new(100);
        proj.set_pulse(50);
        let voice = Voice {
            mark: Mark { pulse: 0 },
            signal: Signal::Text { token_count: 10, checksum: 0 },
        };
        let pos = proj.project(&voice);
        assert!((pos.coord.t - 0.5).abs() < 1e-10);
    }

    #[test]
    fn vortex_compress_converges() {
        let mut vortex = Vortex::at_origin();
        let voice = Voice {
            mark: Mark { pulse: 0 },
            signal: Signal::Audio { frequency_peak: 639.0, amplitude: 0.8 },
        };
        let points = vec![Positioned {
            voice,
            coord: Coord::new(1.0, 0.0, 1.0, 0.639),
        }];

        let step1 = vortex.compress(&points);
        let step2 = vortex.compress(&step1);
        let step3 = vortex.compress(&step2);

        // Each step moves closer to center
        let d1 = vortex.center.distance(&step1[0].coord);
        let d2 = vortex.center.distance(&step2[0].coord);
        let d3 = vortex.center.distance(&step3[0].coord);
        assert!(d2 < d1);
        assert!(d3 < d2);
    }

    #[test]
    fn vortex_salience_closer_is_higher() {
        let vortex = Vortex::at_origin();
        let voice = Voice {
            mark: Mark { pulse: 0 },
            signal: Signal::Text { token_count: 1, checksum: 0 },
        };

        let near = Positioned { voice, coord: Coord::new(0.5, 0.5, 0.5, 0.5) };
        let far = Positioned { voice, coord: Coord::new(1.0, 1.0, 1.0, 1.0) };

        assert!(vortex.salience(&near) > vortex.salience(&far));
    }

    #[test]
    fn vortex_top_n() {
        let vortex = Vortex::at_origin();
        let voice = Voice {
            mark: Mark { pulse: 0 },
            signal: Signal::Text { token_count: 1, checksum: 0 },
        };

        let points = vec![
            Positioned { voice, coord: Coord::new(1.0, 1.0, 1.0, 1.0) },
            Positioned { voice, coord: Coord::new(0.5, 0.5, 0.5, 0.5) },
            Positioned { voice, coord: Coord::new(0.1, 0.1, 0.1, 0.1) },
        ];

        let top = vortex.top_n(&points, 1);
        // Closest to center (0.5,0.5,0.5,0.5) should be the one at (0.5,0.5,0.5,0.5) — distance 0
        assert!((top[0].coord.s - 0.5).abs() < 1e-10);
    }
}
