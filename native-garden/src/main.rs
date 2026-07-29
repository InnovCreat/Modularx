// native-garden/src/main.rs
// VERITAS HORTUS — Native Garden (Pure Rust, native binary)
// Fréquence : 639 Hz · Always for Love, Never for War, Never for Money
//
// Architecture :
//   - veritas_garden  ← ce binaire (egui/eframe, rodio)
//   - synaps_vm       ← SYNAPS VM (rustc main.rs -O -o synaps_vm)
//   - state.json      ← pont entre les deux (écrit par synaps_vm, lu ici)
//
// Build  : cd native-garden && cargo build --release
// Run    : ./target/release/veritas_garden
// (lancer synaps_vm dans le répertoire parent avant)

#![allow(non_snake_case)]

use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};
use std::f32::consts::TAU;
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc, Mutex,
};
use std::time::{Duration, Instant};
use serde::Deserialize;

// ── Dimensions (mirrors garden.html) ─────────────────────────────────

const W:  f32 = 500.0;
const H:  f32 = 500.0;
const CX: f32 = 250.0;
const CY: f32 = 250.0;

const PARTICLE_COUNT: usize = 50;
const BRANCH_COUNT:   usize = 8;

// ── State from state.json ─────────────────────────────────────────────

#[derive(Deserialize, Clone, Default)]
struct VmState {
    frame:       u64,
    pc:          usize,
    state:       String,    // "running" | "paused" | "stasis" | "halted"
    ternary:     String,    // "NEG" | "NEU" | "POS"
    threshold:   f64,
    rewires:     u64,
    youden:      f64,       // [-1, 1]
    auc:         f64,
    sensitivity: f64,
    specificity: f64,
    urgency:     f64,       // 1.0 = stressed
    stack_depth: usize,
    archive:     Vec<ArchiveEntry>,
}

#[derive(Deserialize, Clone, Default)]
struct ArchiveEntry {
    moment: String,
    trace:  String,
    #[allow(dead_code)]
    voix:   String,
}

// ── Particle ──────────────────────────────────────────────────────────

struct Particle {
    x: f32, y: f32,
    vx: f32, vy: f32,
    r: f32,
    hue: f32,
    alpha: f32,
}

impl Particle {
    fn new(rng: &mut Lcg) -> Self {
        Self {
            x:     rng.f32() * W,
            y:     rng.f32() * H,
            vx:    (rng.f32() - 0.5) * 0.4,
            vy:    (rng.f32() - 0.5) * 0.4,
            r:     rng.f32() * 2.0 + 1.0,
            hue:   190.0 + rng.f32() * 20.0,
            alpha: rng.f32() * 0.5 + 0.2,
        }
    }

    fn tick(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        if self.x < 0.0 { self.x = W; }
        if self.x > W   { self.x = 0.0; }
        if self.y < 0.0 { self.y = H; }
        if self.y > H   { self.y = 0.0; }
    }
}

// ── Branch ────────────────────────────────────────────────────────────

struct Branch {
    angle:      f32,
    len:        f32,
    target_len: f32,
    hue:        f32,
}

impl Branch {
    fn new(i: usize) -> Self {
        Self {
            angle:      (i as f32 / BRANCH_COUNT as f32) * TAU,
            len:        0.0,
            target_len: 60.0,
            hue:        260.0 + i as f32 * 12.0,
        }
    }

    fn tick(&mut self, youden_norm: f32) {
        self.target_len = 40.0 + youden_norm * 140.0;
        self.len += (self.target_len - self.len) * 0.05;
    }
}

// ── Minimal LCG RNG (zero deps) ───────────────────────────────────────
// Knuth multiplicative hash, seeded from system time.

struct Lcg(u64);

impl Lcg {
    fn new() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        Self(seed ^ 0xcafe_babe_dead_beef)
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0
    }
    fn f32(&mut self) -> f32 {
        (self.next() >> 33) as f32 / (u32::MAX as f32)
    }
}

// ── HSL → Color32 ────────────────────────────────────────────────────
// h: 0-360, s: 0-1, l: 0-1, a: 0-1

fn hsla(h: f32, s: f32, l: f32, a: f32) -> Color32 {
    let h = h / 360.0;
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = if h < 1.0/6.0      { (c, x, 0.0) }
        else if h < 2.0/6.0             { (x, c, 0.0) }
        else if h < 3.0/6.0             { (0.0, c, x) }
        else if h < 4.0/6.0             { (0.0, x, c) }
        else if h < 5.0/6.0             { (x, 0.0, c) }
        else                             { (c, 0.0, x) };
    Color32::from_rgba_premultiplied(
        ((r + m) * a * 255.0) as u8,
        ((g + m) * a * 255.0) as u8,
        ((b + m) * a * 255.0) as u8,
        (a * 255.0) as u8,
    )
}

// ── Son639 — adjustable-frequency sine wave source ────────────────────
// Implements rodio::Source so it can be appended to a Sink directly.
// Frequency (in mHz) and gain (×10000) are controlled via atomics,
// allowing the UI thread to morph pitch without stopping the audio.

struct AdjustableSine {
    freq_mhz: Arc<AtomicU32>, // Hz × 1000
    gain_ten: Arc<AtomicU32>, // gain × 10000  (400 = 0.04)
    rate:     u32,
    phase:    f64,
}

impl Iterator for AdjustableSine {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        let freq = self.freq_mhz.load(Ordering::Relaxed) as f64 / 1000.0;
        let gain = self.gain_ten.load(Ordering::Relaxed) as f64 / 10_000.0;
        let sample = (self.phase * 2.0 * std::f64::consts::PI).sin() * gain;
        self.phase = (self.phase + freq / self.rate as f64).fract();
        Some(sample as f32)
    }
}

impl rodio::Source for AdjustableSine {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self)                -> u16     { 1 }
    fn sample_rate(&self)             -> u32     { self.rate }
    fn total_duration(&self)          -> Option<Duration> { None }
}

// ── AudioState ────────────────────────────────────────────────────────

struct AudioState {
    _stream: rodio::OutputStream,
    _sink:   rodio::Sink,
    freq_mhz: Arc<AtomicU32>,
    gain_ten: Arc<AtomicU32>,
    muted:    bool,
}

impl AudioState {
    fn try_init() -> Option<Self> {
        let (stream, handle) = rodio::OutputStream::try_default().ok()?;
        let sink = rodio::Sink::try_new(&handle).ok()?;
        let freq_mhz = Arc::new(AtomicU32::new(639_000));
        let gain_ten = Arc::new(AtomicU32::new(400)); // 0.04
        sink.append(AdjustableSine {
            freq_mhz: Arc::clone(&freq_mhz),
            gain_ten: Arc::clone(&gain_ten),
            rate: 44_100,
            phase: 0.0,
        });
        sink.play();
        Some(AudioState { _stream: stream, _sink: sink, freq_mhz, gain_ten, muted: false })
    }

    fn morph_to(&self, hz: f32) {
        self.freq_mhz.store((hz * 1000.0) as u32, Ordering::Relaxed);
    }

    fn set_urgency(&self, on: bool, elapsed: f32) {
        if self.muted { return; }
        let base = self.freq_mhz.load(Ordering::Relaxed) as f32 / 1000.0;
        let target = if on {
            let wobble = (elapsed * 5.0).sin() * 8.0; // ±8 Hz vibrato
            base + wobble
        } else {
            base
        };
        self.freq_mhz.store((target * 1000.0) as u32, Ordering::Relaxed);
    }

    fn toggle_mute(&mut self) {
        self.muted = !self.muted;
        let target = if self.muted { 0 } else { 400 };
        self.gain_ten.store(target, Ordering::Relaxed);
    }

    fn fade_out(&self) {
        // Simple: zero gain immediately (native has no linearRamp — just cut)
        self.gain_ten.store(0, Ordering::Relaxed);
    }
}

// ── GardenApp ─────────────────────────────────────────────────────────

struct GardenApp {
    vm_state:     Arc<Mutex<Option<VmState>>>,
    particles:    Vec<Particle>,
    branches:     Vec<Branch>,
    start:        Instant,
    audio:        Option<AudioState>,
    last_ternary: String,
}

impl GardenApp {
    fn new(vm_state: Arc<Mutex<Option<VmState>>>) -> Self {
        let mut rng = Lcg::new();
        Self {
            vm_state,
            particles: (0..PARTICLE_COUNT).map(|_| Particle::new(&mut rng)).collect(),
            branches:  (0..BRANCH_COUNT).map(Branch::new).collect(),
            start:     Instant::now(),
            audio:     None,
            last_ternary: "NEU".to_string(),
        }
    }
}

impl eframe::App for GardenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ── Read current VM state ──────────────────────────────────────
        let snapshot = self.vm_state.lock().unwrap().clone();

        let (youden, ternary, halted) = snapshot.as_ref()
            .map(|s| (s.youden as f32, s.ternary.clone(), s.state == "halted"))
            .unwrap_or((0.0, "NEU".to_string(), false));

        let youden_norm = (youden + 1.0) / 2.0;
        let elapsed     = self.start.elapsed().as_secs_f32();

        // ── Audio triggers (mirrors main.rs coroutine logic) ──────────
        if let Some(audio) = &self.audio {
            if ternary != self.last_ternary {
                let hz = match ternary.as_str() {
                    "POS" => 528.0,
                    "NEG" => 396.0,
                    _     => 639.0,
                };
                audio.morph_to(hz);
            }
            if let Some(s) = &snapshot {
                audio.set_urgency(s.urgency > 0.0, elapsed);
            }
            if halted { audio.fade_out(); }
        }
        self.last_ternary = ternary.clone();

        // ── Animate particles + branches ──────────────────────────────
        for p in &mut self.particles { p.tick(); }
        for b in &mut self.branches  { b.tick(youden_norm); }

        let pulse      = elapsed.sin() * 3.0;
        let nucleus_r  = 14.0 + pulse;

        let nucleus_color = match ternary.as_str() {
            "POS" => Color32::from_rgb(0x44, 0xff, 0x88),
            "NEG" => Color32::from_rgb(0xff, 0x44, 0x44),
            _     => Color32::from_rgb(0xff, 0xcc, 0x44),
        };

        // ── Left panel — canvas ────────────────────────────────────────
        egui::SidePanel::left("canvas_panel")
            .resizable(false)
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                let (resp, painter) = ui.allocate_painter(
                    Vec2::new(W, H),
                    egui::Sense::hover(),
                );
                let o = resp.rect.min; // origin

                // Background
                painter.rect_filled(resp.rect, 0.0, Color32::from_rgb(6, 4, 14));

                // Fake radial gradient: 3 concentric rects darkening outward
                let mid = Color32::from_rgba_premultiplied(14, 8, 32, 80);
                painter.circle_filled(Pos2::new(o.x + CX, o.y + CY), 160.0, mid);

                // Particles
                for p in &self.particles {
                    let c = hsla(p.hue, 0.70, 0.70, p.alpha);
                    painter.circle_filled(Pos2::new(o.x + p.x, o.y + p.y), p.r, c);
                }

                // Branches
                let cx = o.x + CX;
                let cy = o.y + CY;
                for b in &self.branches {
                    let x2 = cx + b.angle.cos() * b.len;
                    let y2 = cy + b.angle.sin() * b.len;
                    let stroke_c = hsla(b.hue, 0.60, 0.60, 0.45);
                    let tip_c    = hsla(b.hue, 0.70, 0.70, 0.60);
                    painter.line_segment(
                        [Pos2::new(cx, cy), Pos2::new(x2, y2)],
                        Stroke::new(1.5, stroke_c),
                    );
                    painter.circle_filled(Pos2::new(x2, y2), 2.0, tip_c);
                }

                // Nucleus glow ring
                let glow = Color32::from_rgba_premultiplied(
                    nucleus_color.r(), nucleus_color.g(), nucleus_color.b(), 0x44,
                );
                painter.circle_stroke(
                    Pos2::new(cx, cy), nucleus_r + 6.0, Stroke::new(2.0, glow),
                );

                // Nucleus filled
                painter.circle_filled(Pos2::new(cx, cy), nucleus_r, nucleus_color);
            });

        // ── Right panel — dashboard ────────────────────────────────────
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::from_rgb(10, 10, 18)))
            .show(ctx, |ui| {
                ui.add_space(4.0);

                // Title
                ui.label(egui::RichText::new("VERITAS HORTUS")
                    .color(Color32::from_rgb(0xb8, 0x8a, 0xff))
                    .size(14.0).monospace());
                ui.label(egui::RichText::new("SYNAPS VM — 639 Hz")
                    .color(Color32::from_rgb(0x5a, 0x4a, 0x7a))
                    .size(10.0).monospace());
                ui.add_space(10.0);

                // Son639 toggle button
                let audio_on  = self.audio.is_some();
                let audio_mut = self.audio.as_ref().map(|a| a.muted).unwrap_or(false);
                let btn_label = if audio_on && !audio_mut { "♪  639 Hz — ON" }
                    else if audio_on                       { "♩  639 Hz — OFF" }
                    else                                   { "   ACTIVER 639 Hz" };

                let btn_color = if audio_on && !audio_mut {
                    Color32::from_rgb(0x44, 0xff, 0x88)
                } else {
                    Color32::from_rgb(0x7a, 0x6a, 0xaa)
                };

                if ui.add(egui::Button::new(
                    egui::RichText::new(btn_label).color(btn_color).monospace().size(11.0)
                ).min_size(Vec2::new(200.0, 24.0))).clicked() {
                    if self.audio.is_none() {
                        self.audio = AudioState::try_init();
                    } else if let Some(a) = &mut self.audio {
                        a.toggle_mute();
                    }
                }
                ui.add_space(8.0);

                // Ternary badge
                let badge_color = match ternary.as_str() {
                    "POS" => Color32::from_rgb(0x44, 0xff, 0x88),
                    "NEG" => Color32::from_rgb(0xff, 0x44, 0x44),
                    _     => Color32::from_rgb(0xff, 0xcc, 0x44),
                };
                ui.label(egui::RichText::new(format!("[ {} ]", ternary))
                    .color(badge_color).size(13.0).monospace().strong());
                ui.add_space(10.0);

                if let Some(s) = snapshot.as_ref() {
                    info_row(ui, "frame",     &s.frame.to_string());
                    info_row(ui, "pc",        &s.pc.to_string());
                    info_row(ui, "state",     &s.state);
                    info_row(ui, "threshold", &format!("{:.3}", s.threshold));
                    info_row(ui, "rewires",   &s.rewires.to_string());
                    info_row(ui, "stack",     &s.stack_depth.to_string());
                    ui.add_space(10.0);

                    metric_bar(ui, "Youden",      s.youden);
                    metric_bar(ui, "AUC",         s.auc);
                    metric_bar(ui, "Sensitivity", s.sensitivity);
                    metric_bar(ui, "Specificity", s.specificity);

                    if s.urgency > 0.0 {
                        ui.add_space(4.0);
                        ui.label(egui::RichText::new("! URGENCY")
                            .color(Color32::from_rgb(0xff, 0x88, 0x44))
                            .monospace().size(11.0));
                    }

                    // Archive
                    if !s.archive.is_empty() {
                        ui.add_space(12.0);
                        ui.label(egui::RichText::new("ARCHIVE")
                            .color(Color32::from_rgb(0x5a, 0x4a, 0x7a))
                            .monospace().size(10.0));
                        ui.separator();
                        for entry in s.archive.iter().take(5) {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(&entry.moment)
                                    .color(Color32::from_rgb(0x3a, 0x2a, 0x5a))
                                    .monospace().size(10.0));
                                ui.label(egui::RichText::new(&entry.trace)
                                    .color(Color32::from_rgb(0x7a, 0x6a, 0x9a))
                                    .monospace().size(10.0));
                            });
                        }
                    }

                } else {
                    ui.add_space(30.0);
                    ui.label(egui::RichText::new("En attente de SYNAPS VM…")
                        .color(Color32::from_rgb(0x4a, 0x3a, 0x6a))
                        .monospace().size(11.0));
                }
            });

        // 60 fps continuous repaint
        ctx.request_repaint_after(Duration::from_millis(16));
    }
}

// ── UI helpers ────────────────────────────────────────────────────────

fn info_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label)
            .color(Color32::from_rgb(0x66, 0x55, 0xaa))
            .monospace().size(11.0));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(value)
                .color(Color32::from_rgb(0xaa, 0x99, 0xcc))
                .monospace().size(11.0));
        });
    });
}

fn metric_bar(ui: &mut egui::Ui, label: &str, value: f64) {
    let norm = value.clamp(0.0, 1.0) as f32;
    let bar_color = if norm > 0.7 { Color32::from_rgb(0x44, 0xff, 0x88) }
        else if norm > 0.4        { Color32::from_rgb(0xff, 0xcc, 0x44) }
        else                       { Color32::from_rgb(0xff, 0x44, 0x44) };

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label)
            .color(Color32::from_rgb(0x88, 0x77, 0xaa))
            .monospace().size(11.0));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(format!("{:.3}", value))
                .color(Color32::from_rgb(0xd0, 0xc0, 0xf0))
                .monospace().size(11.0));
        });
    });

    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 8.0), egui::Sense::hover());
    let p = ui.painter();
    p.rect_filled(rect, 4.0, Color32::from_rgb(0x1a, 0x12, 0x28));
    let fill = Rect::from_min_size(rect.min, Vec2::new(rect.width() * norm, rect.height()));
    p.rect_filled(fill, 4.0, bar_color);
    ui.add_space(6.0);
}

// ── main ──────────────────────────────────────────────────────────────

fn main() {
    // Shared state between polling thread and UI thread
    let vm_state: Arc<Mutex<Option<VmState>>> = Arc::new(Mutex::new(None));
    let vm_state_bg = Arc::clone(&vm_state);

    // Background thread — reads state.json every 100ms
    // Looks for state.json in the current directory (run from repo root)
    std::thread::spawn(move || {
        let path = std::env::current_dir()
            .unwrap_or_default()
            .join("state.json");

        loop {
            let parsed = std::fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str::<VmState>(&s).ok());
            *vm_state_bg.lock().unwrap() = parsed;
            std::thread::sleep(Duration::from_millis(100));
        }
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("SYNAPS VM — Veritas Hortus Garden")
            .with_inner_size([W + 280.0, H + 40.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "veritas-garden",
        options,
        Box::new(move |_cc| Box::new(GardenApp::new(vm_state))),
    )
    .expect("Garden failed to start");
}
