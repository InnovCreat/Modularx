// ════════════════════════════════════════════════════════════════
// MODULARIS — Veritas Hortus Orchestrator
// 639Hz pulse-based temporal coordination for composable subsystems
// Layers: Core → SLTQ (4D space) → Vortex (PHI reduction) → Bridge
// ════════════════════════════════════════════════════════════════

pub mod sltq;
pub mod bridge;

use std::time::{Duration, Instant};

// ── Constants ───────────────────────────────────────────────────

pub const FREQUENCY_HZ: u64 = 639;
pub const PULSE_MICROS: u64 = 1_000_000 / FREQUENCY_HZ; // ~1564 µs
pub const PHI: f64 = 1.618033988749895;

// ── Mark in Time ────────────────────────────────────────────────
// Immutable temporal coordinate in the crystalline pulse grid.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mark {
    pub pulse: u64,
}

impl Mark {
    pub fn from_instant(genesis: Instant, now: Instant) -> Self {
        let elapsed = now.duration_since(genesis).as_micros() as u64;
        Mark { pulse: elapsed / PULSE_MICROS }
    }

    pub fn to_duration(self) -> Duration {
        Duration::from_micros(self.pulse * PULSE_MICROS)
    }

    pub fn delta(self, other: Mark) -> u64 {
        self.pulse.abs_diff(other.pulse)
    }
}

// ── Perception Signals ──────────────────────────────────────────
// Four independent sensory channels — never mixed before SLTQ.

#[derive(Debug, Clone, Copy)]
pub enum Signal {
    Visual { luma_hash: u64, density: u32 },
    Audio { frequency_peak: f32, amplitude: f32 },
    Biometric { pulse_rate: u8, coherence: f32 },
    Text { token_count: u32, checksum: u32 },
}

impl Signal {
    pub fn channel(&self) -> Channel {
        match self {
            Signal::Visual { .. } => Channel::Visual,
            Signal::Audio { .. } => Channel::Audio,
            Signal::Biometric { .. } => Channel::Biometric,
            Signal::Text { .. } => Channel::Text,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Channel {
    Visual,
    Audio,
    Biometric,
    Text,
}

// ── Polyphonic Voice ────────────────────────────────────────────
// A signal stamped with its temporal mark — the atomic unit of data flow.

#[derive(Debug, Clone, Copy)]
pub struct Voice {
    pub mark: Mark,
    pub signal: Signal,
}

// ── Subsystem Trait ─────────────────────────────────────────────
// Interface for pluggable components (NEXUS, LUNA, NUCLEUS, etc.)

pub trait Subsystem {
    fn name(&self) -> &str;
    fn accepts(&self) -> &[Channel];
    fn process(&mut self, voice: &Voice) -> Option<Signal>;
    fn tick(&mut self, pulse: u64);
}

// ── Orchestrator ────────────────────────────────────────────────
// Routes voices between subsystems at 639Hz cadence.

pub struct Orchestrator {
    genesis: Instant,
    current_pulse: u64,
    subsystems: Vec<Box<dyn Subsystem>>,
    bus: Vec<Voice>,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {
            genesis: Instant::now(),
            current_pulse: 0,
            subsystems: Vec::new(),
            bus: Vec::new(),
        }
    }

    pub fn genesis(&self) -> Instant {
        self.genesis
    }

    pub fn pulse(&self) -> u64 {
        self.current_pulse
    }

    pub fn register(&mut self, subsystem: Box<dyn Subsystem>) {
        self.subsystems.push(subsystem);
    }

    pub fn inject(&mut self, signal: Signal) {
        let mark = Mark::from_instant(self.genesis, Instant::now());
        self.bus.push(Voice { mark, signal });
    }

    pub fn inject_at(&mut self, signal: Signal, time: Instant) {
        let mark = Mark::from_instant(self.genesis, time);
        self.bus.push(Voice { mark, signal });
    }

    pub fn step(&mut self) {
        let now = Instant::now();
        let mark = Mark::from_instant(self.genesis, now);
        self.current_pulse = mark.pulse;

        // Tick all subsystems
        for sys in &mut self.subsystems {
            sys.tick(self.current_pulse);
        }

        // Route pending voices to matching subsystems
        let voices: Vec<Voice> = self.bus.drain(..).collect();
        let mut new_signals: Vec<Signal> = Vec::new();

        for voice in &voices {
            let channel = voice.signal.channel();
            for sys in &mut self.subsystems {
                if sys.accepts().contains(&channel) {
                    if let Some(output) = sys.process(voice) {
                        new_signals.push(output);
                    }
                }
            }
        }

        // Feed outputs back into the bus for next cycle
        for sig in new_signals {
            let m = Mark::from_instant(self.genesis, Instant::now());
            self.bus.push(Voice { mark: m, signal: sig });
        }
    }

    pub fn run_pulses(&mut self, count: u64) {
        let pulse_duration = Duration::from_micros(PULSE_MICROS);
        for _ in 0..count {
            let start = Instant::now();
            self.step();
            let elapsed = start.elapsed();
            if elapsed < pulse_duration {
                std::thread::sleep(pulse_duration - elapsed);
            }
        }
    }

    pub fn bus_len(&self) -> usize {
        self.bus.len()
    }

    pub fn subsystem_count(&self) -> usize {
        self.subsystems.len()
    }
}

impl Default for Orchestrator {
    fn default() -> Self {
        Self::new()
    }
}

// ── NEXUS Subsystem ─────────────────────────────────────────────
// Bridges the NEXUS compiler into the orchestrator.

pub struct NexusSubsystem {
    processed: u64,
}

impl NexusSubsystem {
    pub fn new() -> Self {
        Self { processed: 0 }
    }

    pub fn processed_count(&self) -> u64 {
        self.processed
    }
}

impl Default for NexusSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Subsystem for NexusSubsystem {
    fn name(&self) -> &str {
        "NEXUS"
    }

    fn accepts(&self) -> &[Channel] {
        &[Channel::Text]
    }

    fn process(&mut self, _voice: &Voice) -> Option<Signal> {
        self.processed += 1;
        None
    }

    fn tick(&mut self, _pulse: u64) {}
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn mark_calculation() {
        let genesis = Instant::now();
        thread::sleep(Duration::from_micros(PULSE_MICROS * 3));
        let mark = Mark::from_instant(genesis, Instant::now());
        assert!(mark.pulse >= 2); // at least 2 pulses elapsed (timing tolerance)
    }

    #[test]
    fn mark_to_duration() {
        let mark = Mark { pulse: 639 };
        let dur = mark.to_duration();
        // 639 pulses * 1564µs ≈ 1 second
        assert!((dur.as_secs_f64() - 1.0).abs() < 0.01);
    }

    #[test]
    fn signal_channels() {
        let s = Signal::Text { token_count: 5, checksum: 0 };
        assert_eq!(s.channel(), Channel::Text);

        let s = Signal::Audio { frequency_peak: 639.0, amplitude: 1.0 };
        assert_eq!(s.channel(), Channel::Audio);
    }

    #[test]
    fn orchestrator_inject_and_step() {
        let mut orch = Orchestrator::new();
        orch.register(Box::new(NexusSubsystem::new()));

        orch.inject(Signal::Text { token_count: 42, checksum: 0xABC });
        assert_eq!(orch.bus_len(), 1);

        orch.step();
        // After step, bus is drained and routed
        assert_eq!(orch.bus_len(), 0);
    }

    #[test]
    fn nexus_subsystem_processes_text() {
        let mut nexus = NexusSubsystem::new();
        let voice = Voice {
            mark: Mark { pulse: 1 },
            signal: Signal::Text { token_count: 10, checksum: 0 },
        };
        nexus.process(&voice);
        assert_eq!(nexus.processed_count(), 1);
    }

    #[test]
    fn orchestrator_multi_subsystem() {
        let mut orch = Orchestrator::new();
        orch.register(Box::new(NexusSubsystem::new()));
        orch.register(Box::new(NexusSubsystem::new()));
        assert_eq!(orch.subsystem_count(), 2);

        orch.inject(Signal::Text { token_count: 1, checksum: 0 });
        orch.step();
        // Both subsystems should have processed the text signal
    }

    #[test]
    fn pulse_frequency_constant() {
        // 639Hz → ~1564 µs per pulse
        assert_eq!(PULSE_MICROS, 1564);
        assert_eq!(FREQUENCY_HZ, 639);
    }

    #[test]
    fn phi_constant() {
        assert!((PHI - 1.618033988749895).abs() < 1e-15);
    }
}
