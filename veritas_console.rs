// veritas_console.rs — Veritas Modularis · REPL Console
// jamais pour la guerre · jamais pour l'argent · toujours pour l'amour
// ============================================================

use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{self, BufRead, Write};

// === Constants & Enums ==============================================

pub const BASE_TENSION: f64 = 1.0;

// std::f64::consts::PHI does not exist — define it manually
pub const PHI: f64 = 1.618_033_988_749_895_f64;

// Amplitude below which the system is calm ("breathing slow") — no cure, no reset
pub const CALM_THRESHOLD: f64 = 0.8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntryKind { Violation, Info, Harmony, Cure }

// AmplitudeOverflow added — it is used in Watchdog::check
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViolationType {
    SigMismatch,
    HighChaos,
    NaNDetected,
    IntegrityFail,
    WatchdogKill,
    AmplitudeOverflow,
}

impl std::fmt::Display for ViolationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CureAction { NoCureNeeded, Damping, ForceClamp, Rollback, EmergencyHalt }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChaosLabel { Stable, Resonant, Chaotic, Saturated }

// === Core types =====================================================

#[derive(Debug, Clone)]
pub struct QuantumState {
    pub amplitude:         f64,
    pub num_points:        usize,
    pub base_perturbation: Vec<f64>,
    pub tension:           Vec<f64>,
    pub is_alive:          bool,   // true = amplitude >= CALM_THRESHOLD (active/watched)
    pub sig:               u64,
    pub chaos_label:       ChaosLabel,
}

impl QuantumState {
    pub fn new(n: usize, amp: f64) -> Self {
        let mut s = Self {
            amplitude:         amp,
            num_points:        n,
            base_perturbation: vec![0.0; n],
            tension:           vec![],
            is_alive:          amp >= CALM_THRESHOLD,
            sig:               0,
            chaos_label:       ChaosLabel::Stable,
        };
        s.update_sig();
        s
    }

    pub fn mark_alive(&mut self) {
        self.is_alive = self.amplitude >= CALM_THRESHOLD;
    }

    pub fn update_sig(&mut self) {
        let mut h: u64 = 5381;
        h = h.wrapping_mul(6364136223846793005).wrapping_add(self.amplitude.to_bits());
        h = h.wrapping_mul(6364136223846793005).wrapping_add(self.num_points as u64);
        for &v in &self.base_perturbation {
            h = h.wrapping_mul(6364136223846793005).wrapping_add(v.to_bits());
        }
        self.sig = h;
    }

    pub fn verify_sig(&self) -> bool {
        let mut h: u64 = 5381;
        h = h.wrapping_mul(6364136223846793005).wrapping_add(self.amplitude.to_bits());
        h = h.wrapping_mul(6364136223846793005).wrapping_add(self.num_points as u64);
        for &v in &self.base_perturbation {
            h = h.wrapping_mul(6364136223846793005).wrapping_add(v.to_bits());
        }
        h == self.sig
    }

    pub fn refresh(&mut self) {
        self.is_alive = self.amplitude >= CALM_THRESHOLD;
        if self.is_alive {
            self.tension = self.base_perturbation.iter()
                .map(|&bp| (BASE_TENSION + bp * self.amplitude).max(0.0))
                .collect();
        }
        self.update_sig(); // always refresh sig so verify_sig() stays valid
    }
}

// === Metrics ========================================================

pub fn node_spread(s: &mut QuantumState) -> f64 {
    s.refresh();
    if s.base_perturbation.is_empty() { return 0.0; }
    let sum_sq: f64 = s.base_perturbation.iter()
        .map(|&bp| (bp * s.amplitude).powi(2))
        .sum();
    (sum_sq / s.num_points as f64).sqrt() / 3.0
}

// === MycBook & Monitor ==============================================

pub fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[derive(Debug, Clone)]
pub struct MycEntry {
    pub ts:   u64,
    pub kind: EntryKind,
    pub msg:  String,
}

pub struct MycBook {
    pub entries: Vec<MycEntry>,
}

impl MycBook {
    pub fn append(&mut self, kind: EntryKind, msg: &str) {
        self.entries.push(MycEntry {
            ts: unix_now(),
            kind,
            msg: msg.to_string(),
        });
        // NOTE: the original println! is intentionally omitted here.
        // The console renders journal entries via render_box() instead.
    }
}

pub struct NodeMonitor {
    pub mycbook:    MycBook,
    pub violations: u64,
}

impl NodeMonitor {
    pub fn new() -> Self {
        Self {
            mycbook:    MycBook { entries: vec![] },
            violations: 0,
        }
    }

    pub fn alert_violation(&mut self, v: ViolationType, detail: &str) {
        self.violations += 1;
        self.mycbook.append(EntryKind::Violation, &format!("{} — {}", v, detail));
    }

    pub fn alert_harmony(&mut self, msg: &str) {
        self.mycbook.append(EntryKind::Harmony, msg);
    }

    pub fn alert_cure(&mut self, msg: &str) {
        self.mycbook.append(EntryKind::Cure, msg);
    }
}

// === Cure with phi thresholds =========================================

#[derive(Debug, Clone)]
pub struct CureThresholds {
    pub phi:                  f64,
    pub phi_inv:              f64,
    pub spread_trigger_ratio: f64,
    pub damping_factor:       f64,
    pub amplitude_hard_max:   f64,
    pub amplitude_target:     f64,
    pub max_iterations:       usize,
}

impl Default for CureThresholds {
    fn default() -> Self {
        Self {
            phi:                  PHI,
            phi_inv:              1.0 / PHI,
            spread_trigger_ratio: PHI,
            damping_factor:       1.0 / PHI,
            amplitude_hard_max:   5.0,
            amplitude_target:     1.0,
            max_iterations:       8,
        }
    }
}

pub struct NodeCure {
    pub thresholds:  CureThresholds,
    pub total_cures: u64,
}

impl NodeCure {
    pub fn with_defaults() -> Self {
        Self {
            thresholds:  CureThresholds::default(),
            total_cures: 0,
        }
    }

    pub fn needs_cure(&self, s: &mut QuantumState, monitor: &mut NodeMonitor) -> (bool, CureAction) {
        if !s.is_alive {
            monitor.mycbook.append(EntryKind::Harmony, "Breathing slow — calm below 0.8");
            return (false, CureAction::NoCureNeeded);
        }
        if s.tension.iter().any(|&v| v.is_nan() || v.is_infinite()) {
            monitor.alert_violation(ViolationType::NaNDetected, "NaN/Inf in tension");
            return (true, CureAction::EmergencyHalt);
        }
        if s.amplitude > self.thresholds.amplitude_hard_max {
            return (true, CureAction::ForceClamp);
        }
        let spread = s.base_perturbation.iter()
            .map(|&bp| bp * s.amplitude)
            .sum::<f64>()
            / s.num_points as f64;
        if spread > self.thresholds.spread_trigger_ratio {
            return (true, CureAction::Damping);
        }
        (false, CureAction::NoCureNeeded)
    }

    pub fn apply(&mut self, s: &mut QuantumState, monitor: &mut NodeMonitor) -> CureResult {
        let amp_before    = s.amplitude;
        let spread_before = s.base_perturbation.iter()
            .map(|&bp| bp * s.amplitude)
            .sum::<f64>()
            / s.num_points as f64;

        self.total_cures += 1;
        s.amplitude *= self.thresholds.damping_factor;
        s.mark_alive();
        s.refresh();

        let spread_after = s.base_perturbation.iter()
            .map(|&bp| bp * s.amplitude)
            .sum::<f64>()
            / s.num_points as f64;

        let msg = format!("Damping phi -> {:.3} -> {:.3}", amp_before, s.amplitude);
        monitor.alert_cure(&msg);

        CureResult {
            applied:          true,
            action:           CureAction::Damping,
            iterations:       1,
            amplitude_before: amp_before,
            amplitude_after:  s.amplitude,
            spread_before,
            spread_after,
            message:          msg,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CureResult {
    pub applied:          bool,
    pub action:           CureAction,
    pub iterations:       usize,
    pub amplitude_before: f64,
    pub amplitude_after:  f64,
    pub spread_before:    f64,
    pub spread_after:     f64,
    pub message:          String,
}

// === Watchdog =======================================================

pub struct Watchdog {
    pub kill_switch: bool,
    pub max_amp:     f64,
    pub max_spread:  f64,
}

impl Watchdog {
    pub fn new() -> Self {
        Self { kill_switch: false, max_amp: 10.0, max_spread: 2.5 }
    }

    pub fn check(&mut self, s: &QuantumState, monitor: &mut NodeMonitor) -> bool {
        if s.amplitude > self.max_amp {
            monitor.alert_violation(ViolationType::AmplitudeOverflow, "Amp too high");
            self.kill_switch = true;
            return false;
        }
        let spread = s.base_perturbation.iter()
            .map(|&bp| bp * s.amplitude)
            .sum::<f64>()
            / s.num_points as f64;
        if spread > self.max_spread {
            monitor.alert_violation(ViolationType::HighChaos, "Spread too high");
            self.kill_switch = true;
            return false;
        }
        true
    }
}

// === Main cycle =====================================================

pub fn guard_and_cure_cycle(
    state:    &mut QuantumState,
    monitor:  &mut NodeMonitor,
    cure:     &mut NodeCure,
    watchdog: &mut Watchdog,
) -> Option<CureResult> {
    if !state.verify_sig() {
        monitor.alert_violation(ViolationType::SigMismatch, "State signature invalid");
        return None;
    }

    if !watchdog.check(state, monitor) {
        // Fallback: attempt one damping cure before halting
        let fallback = cure.apply(state, monitor);
        monitor.mycbook.append(
            EntryKind::Info,
            &format!("Watchdog fallback cure -> amp={:.3}", state.amplitude),
        );
        if state.amplitude <= watchdog.max_amp {
            monitor.alert_harmony("Fallback cure succeeded — watchdog cleared");
        } else {
            monitor.alert_violation(ViolationType::WatchdogKill, "Fallback cure insufficient");
        }
        return Some(fallback);
    }

    let (needs, _) = cure.needs_cure(state, monitor);
    if needs {
        Some(cure.apply(state, monitor))
    } else {
        monitor.mycbook.append(EntryKind::Harmony, "Stable — 639 Hz");
        None
    }
}

// === Console additions ==============================================

struct ConsoleState {
    state:    QuantumState,
    monitor:  NodeMonitor,
    cure:     NodeCure,
    watchdog: Watchdog,
    cycle:    u64,
    show_log: bool,
}

impl ConsoleState {
    fn new() -> Self {
        let mut state = QuantumState::new(64, 0.4);
        state.base_perturbation = vec![0.15; 64];
        state.refresh();
        ConsoleState {
            state,
            monitor:  NodeMonitor::new(),
            cure:     NodeCure::with_defaults(),
            watchdog: Watchdog::new(),
            cycle:    0,
            show_log: true,
        }
    }
}

fn compute_chaos_label(spread: f64, amplitude: f64) -> ChaosLabel {
    if amplitude > 5.0 || spread > 2.0 {
        ChaosLabel::Saturated
    } else if spread > 1.0 || amplitude > 3.0 {
        ChaosLabel::Chaotic
    } else if (amplitude - 1.5).abs() < 0.3 && spread < 0.3 {
        ChaosLabel::Resonant
    } else {
        ChaosLabel::Stable
    }
}

fn spread_bar(spread: f64) -> String {
    // Visual scale 0.0–0.5 for the bar only; watchdog threshold (2.5) is separate
    let bar_max = 0.5_f64;
    let filled  = ((spread / bar_max) * 20.0).round().clamp(0.0, 20.0) as usize;
    let empty   = 20 - filled;
    format!("[{}{}]", "\u{2588}".repeat(filled), "\u{2591}".repeat(empty))
}

fn chaos_label_color(label: ChaosLabel) -> &'static str {
    match label {
        ChaosLabel::Stable    => "vert",
        ChaosLabel::Resonant  => "cyan",
        ChaosLabel::Chaotic   => "orange",
        ChaosLabel::Saturated => "rouge",
    }
}

fn mycbook_prefix(kind: EntryKind) -> &'static str {
    match kind {
        EntryKind::Harmony   => "\u{1F33F}",  // leaf emoji
        EntryKind::Violation => "\u{26A0}",   // warning sign
        EntryKind::Cure      => "\u{21BA}",   // clockwise open circle arrow
        EntryKind::Info      => "\u{00B7}",   // middle dot
    }
}

// Approximate terminal display width: emoji are 2 columns, others 1
fn display_width(s: &str) -> usize {
    s.chars().map(|c| {
        let cp = c as u32;
        // Emoji ranges and some wide symbols
        if (0x1F000..=0x1FFFF).contains(&cp)
            || (0x2600..=0x27FF).contains(&cp)
            || cp == 0x26A0  // warning sign
        {
            2
        } else {
            1
        }
    }).sum()
}

fn render_box(cs: &ConsoleState) {
    let mut state_clone = cs.state.clone();
    let spread = node_spread(&mut state_clone);
    let label  = compute_chaos_label(spread, cs.state.amplitude);
    let bar    = spread_bar(spread);
    let color  = chaos_label_color(label);
    let freq_hz: u64 = 639;

    const W: usize = 54; // inner content width

    // Pad a string to exactly W display columns using display_width for emoji awareness
    let pad = |s: &str| -> String {
        let dw = display_width(s);
        if dw < W {
            format!("{}{}", s, " ".repeat(W - dw))
        } else {
            // Truncate by chars until display width fits
            let mut out = String::new();
            let mut w = 0;
            for c in s.chars() {
                let cw = if (c as u32) > 0x2000 { 2 } else { 1 };
                if w + cw > W { break; }
                out.push(c);
                w += cw;
            }
            out
        }
    };

    println!("\u{2554}{}\u{2557}", "\u{2550}".repeat(W));
    println!("\u{2551}{}\u{2551}", pad("         \u{2B21}  VERITAS MODULARIS  \u{00B7}  LIVE MONITOR  \u{2B21}    "));
    println!("\u{2560}{}\u{2563}", "\u{2550}".repeat(W));
    println!("\u{2551}{}\u{2551}", pad(&format!(" Cycle               #{}", cs.cycle)));
    println!("\u{2551}{}\u{2551}", pad(&format!(" Freq                {} Hz  \u{00B7}  jamais pour la guerre", freq_hz)));
    println!("\u{2560}{}\u{2563}", "\u{2550}".repeat(W));
    println!("\u{2551}{}\u{2551}", pad(" QUANTUM STATE"));
    println!("\u{255F}{}\u{2562}", "\u{2500}".repeat(W));
    println!("\u{2551}{}\u{2551}", pad(&format!(" Amplitude           {:.4}", cs.state.amplitude)));
    println!("\u{2551}{}\u{2551}", pad(&format!(" Spread              {:.4}  {}", spread, bar)));
    println!("\u{2551}{}\u{2551}", pad(&format!(" Chaos Label         {:?}  \u{2190}  {}", label, color)));
    println!("\u{255F}{}\u{2562}", "\u{2500}".repeat(W));

    if cs.show_log {
        println!("\u{2551}{}\u{2551}", pad(" MYCBOOK \u{2014} Journal"));
        let entries = &cs.monitor.mycbook.entries;
        if entries.is_empty() {
            println!("\u{2551}{}\u{2551}", pad("  \u{00B7} (no entries yet)"));
        } else {
            let last_n = entries.len().saturating_sub(3);
            for entry in &entries[last_n..] {
                let prefix = mycbook_prefix(entry.kind);
                let line   = format!("  {} {}", prefix, entry.msg);
                println!("\u{2551}{}\u{2551}", pad(&line));
            }
        }
    }

    println!("\u{255A}{}\u{255D}", "\u{2550}".repeat(W));
}

// === Command parser ==================================================

enum Command {
    Cycle,
    Stable,
    Chaos,
    Resonant,
    Amp(f64),
    Points(usize),
    Log,
    Reset,
    Quit,
    Unknown(String),
}

fn parse_command(line: &str) -> Command {
    let mut parts = line.trim().split_whitespace();
    match parts.next().unwrap_or("") {
        "cycle"    => Command::Cycle,
        "stable"   => Command::Stable,
        "chaos"    => Command::Chaos,
        "resonant" => Command::Resonant,
        "amp" => {
            match parts.next().and_then(|v| v.parse::<f64>().ok()) {
                Some(v) => Command::Amp(v),
                None    => Command::Unknown("amp requires a float value".to_string()),
            }
        }
        "points" => {
            match parts.next().and_then(|v| v.parse::<usize>().ok()) {
                Some(n) => Command::Points(n),
                None    => Command::Unknown("points requires an integer value".to_string()),
            }
        }
        "log"   => Command::Log,
        "reset" => Command::Reset,
        "q"     => Command::Quit,
        other   => Command::Unknown(other.to_string()),
    }
}

// Returns false only on Quit
fn execute_command(cs: &mut ConsoleState, cmd: Command) -> bool {
    match cmd {
        Command::Cycle => {
            cs.cycle += 1;
            guard_and_cure_cycle(
                &mut cs.state,
                &mut cs.monitor,
                &mut cs.cure,
                &mut cs.watchdog,
            );
        }

        Command::Stable => {
            cs.state.amplitude         = 0.3;
            cs.state.base_perturbation = vec![0.05; cs.state.num_points];
            cs.state.mark_alive();
            cs.state.refresh();
            cs.monitor.alert_harmony("Preset: stable — sin doux 0.3");
        }

        Command::Chaos => {
            let n = cs.state.num_points;
            // bp=2.0 with amp=7.0 → spread=14.0 triggers watchdog; fallback cure then runs.
            // This is intentional: demonstrates the watchdog+fallback cure path.
            cs.state.amplitude         = 7.0;
            cs.state.base_perturbation = vec![2.0; n];
            cs.state.mark_alive();
            cs.state.refresh();
            cs.monitor.alert_violation(
                ViolationType::HighChaos,
                "Preset: chaos — amp=7.0 bp=2.0",
            );
            cs.cycle += 1;
            guard_and_cure_cycle(
                &mut cs.state,
                &mut cs.monitor,
                &mut cs.cure,
                &mut cs.watchdog,
            );
        }

        Command::Resonant => {
            cs.state.amplitude         = 1.5;
            cs.state.base_perturbation = vec![0.1; cs.state.num_points];
            cs.state.mark_alive();
            cs.state.refresh();
            cs.monitor.alert_harmony("639 Hz resonance activated");
        }

        Command::Amp(v) => {
            cs.state.amplitude = v;
            cs.state.mark_alive();
            cs.state.refresh();
            cs.monitor.mycbook.append(
                EntryKind::Info,
                &format!("Amplitude set to {:.4}", v),
            );
        }

        Command::Points(n) => {
            if n == 0 {
                cs.monitor.alert_violation(
                    ViolationType::IntegrityFail,
                    "points must be > 0",
                );
            } else {
                cs.state.num_points        = n;
                cs.state.base_perturbation = vec![0.1; n];
                cs.state.mark_alive();
                cs.state.refresh();
                cs.monitor.mycbook.append(
                    EntryKind::Info,
                    &format!("Resized to {} points", n),
                );
            }
        }

        Command::Log => {
            cs.show_log = !cs.show_log;
            println!(
                "  [log {}]",
                if cs.show_log { "ON" } else { "OFF" }
            );
        }

        Command::Reset => {
            *cs = ConsoleState::new();
            cs.monitor.alert_harmony("System reinitialized");
        }

        Command::Quit => {
            println!("  Au revoir \u{00B7} jamais pour la guerre\n");
            return false;
        }

        Command::Unknown(s) => {
            if !s.is_empty() {
                println!(
                    "  Unknown command: '{}' — try: cycle stable chaos resonant amp <v> points <n> log reset q",
                    s
                );
            }
        }
    }

    // After any state-mutating command, update chaos_label on the live state
    {
        let mut clone = cs.state.clone();
        let spread    = node_spread(&mut clone);
        cs.state.chaos_label = compute_chaos_label(spread, cs.state.amplitude);
    }

    true
}

// === REPL main ======================================================

fn main() {
    let mut cs = ConsoleState::new();
    cs.monitor.alert_harmony("639 Hz resonance activated");

    render_box(&cs);

    // IMPORTANT: stdin must be stored in a named binding so that StdinLock
    // borrows from it rather than from a temporary that would be dropped.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        // flush is required — stdout may be line-buffered
        print!("\u{25B6}  ");
        io::stdout().flush().unwrap();

        let raw = match lines.next() {
            Some(Ok(l))  => l,
            Some(Err(e)) => { eprintln!("stdin error: {}", e); break; }
            None         => break,  // EOF (Ctrl-D)
        };

        let cmd = parse_command(&raw);
        let should_continue = execute_command(&mut cs, cmd);

        if !should_continue {
            break;
        }

        render_box(&cs);
    }
}
