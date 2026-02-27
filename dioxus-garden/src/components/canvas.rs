// dioxus-garden/src/components/canvas.rs
// Canvas 2D rendering — Rust/web-sys port of the garden.html draw loop.
//
// Architecture:
//   - GardenCanvas component renders a <canvas> element
//   - use_hook creates a gloo_timers::Interval at 16ms (~60fps)
//   - draw_frame() is a standalone fn that reads thread_local state
//     (updated by the main.rs coroutine from VM_STATE)
//   - thread_local avoids any Dioxus signal context issues in timer callbacks

use dioxus::prelude::*;
use std::cell::RefCell;
use wasm_bindgen::{JsCast, JsValue};

const W: f64 = 500.0;
const H: f64 = 500.0;
const CX: f64 = 250.0;
const CY: f64 = 250.0;

// ── Particle & Branch structs (mirrors garden.html) ────────────────────

struct Particle {
    x: f64, y: f64,
    vx: f64, vy: f64,
    r: f64,
    hue: f64,
    alpha: f64,
}

struct Branch {
    angle: f64,
    len: f64,
    target_len: f64,
    hue: f64,
}

// ── Thread-local state (safe on single-threaded WASM) ─────────────────

thread_local! {
    static PARTICLES: RefCell<Vec<Particle>> = RefCell::new(init_particles());
    static BRANCHES:  RefCell<Vec<Branch>>   = RefCell::new(init_branches());

    // Canvas state snapshot — written by the coroutine, read by draw_frame
    static YOUDEN:   RefCell<f64>    = RefCell::new(0.0);
    static TERNARY:  RefCell<String> = RefCell::new("NEU".to_string());
}

fn rand() -> f64 {
    js_sys::Math::random()
}

fn init_particles() -> Vec<Particle> {
    (0..50).map(|_| Particle {
        x:     rand() * W,
        y:     rand() * H,
        vx:    (rand() - 0.5) * 0.4,
        vy:    (rand() - 0.5) * 0.4,
        r:     rand() * 2.0 + 1.0,
        hue:   190.0 + rand() * 20.0,
        alpha: rand() * 0.5 + 0.2,
    }).collect()
}

fn init_branches() -> Vec<Branch> {
    (0..8).map(|i| Branch {
        angle:      (i as f64 / 8.0) * std::f64::consts::TAU,
        len:        0.0,
        target_len: 60.0,
        hue:        260.0 + i as f64 * 12.0,
    }).collect()
}

// ── Public: update snapshot from the main coroutine ───────────────────

pub fn update_canvas_state(youden: f64, ternary: String) {
    YOUDEN.with(|y| *y.borrow_mut() = youden);
    TERNARY.with(|t| *t.borrow_mut() = ternary);
}

// ── draw_frame — called at ~60fps by Interval ─────────────────────────

fn draw_frame() {
    // ── Obtain canvas context ──────────────────────────────────────
    let window = match web_sys::window() { Some(w) => w, None => return };
    let doc    = match window.document() { Some(d) => d, None => return };
    let el     = match doc.get_element_by_id("garden-canvas") { Some(e) => e, None => return };
    let canvas = match el.dyn_into::<web_sys::HtmlCanvasElement>() { Ok(c) => c, Err(_) => return };
    let obj    = match canvas.get_context("2d").ok().flatten() { Some(o) => o, None => return };
    let ctx    = match obj.dyn_into::<web_sys::CanvasRenderingContext2d>() { Ok(c) => c, Err(_) => return };

    // ── Read snapshot ──────────────────────────────────────────────
    let youden     = YOUDEN.with(|y| *y.borrow());
    let ternary    = TERNARY.with(|t| t.borrow().clone());
    let youden_norm = (youden + 1.0) / 2.0; // remap [-1,1] → [0,1]

    // Nucleus color driven by ternary state
    let nucleus_color = match ternary.as_str() {
        "POS" => "#44ff88",
        "NEG" => "#ff4444",
        _     => "#ffcc44",
    };

    // ── Clear ──────────────────────────────────────────────────────
    ctx.clear_rect(0.0, 0.0, W, H);

    // ── Background radial gradient ─────────────────────────────────
    if let Ok(bg) = ctx.create_radial_gradient(CX, CY, 0.0, CX, CY, 260.0) {
        let _ = bg.add_color_stop(0.0, "#0e0820");
        let _ = bg.add_color_stop(1.0, "#06040e");
        ctx.set_fill_style(&JsValue::from(bg));
    } else {
        ctx.set_fill_style(&JsValue::from_str("#06040e"));
    }
    ctx.fill_rect(0.0, 0.0, W, H);

    // ── Particles ──────────────────────────────────────────────────
    PARTICLES.with(|p| {
        let mut particles = p.borrow_mut();
        for p in particles.iter_mut() {
            p.x += p.vx;
            p.y += p.vy;
            // Wrap around edges
            if p.x < 0.0 { p.x = W; }
            if p.x > W   { p.x = 0.0; }
            if p.y < 0.0 { p.y = H; }
            if p.y > H   { p.y = 0.0; }

            ctx.begin_path();
            let _ = ctx.arc(p.x, p.y, p.r, 0.0, std::f64::consts::TAU);
            let fill = format!("hsla({:.0}, 70%, 70%, {:.2})", p.hue, p.alpha);
            ctx.set_fill_style(&JsValue::from_str(&fill));
            ctx.fill();
        }
    });

    // ── Branches (8, growing from centre, length ∝ Youden) ────────
    BRANCHES.with(|b| {
        let mut branches = b.borrow_mut();
        for b in branches.iter_mut() {
            // Target length: 40px (min) to 180px (max), driven by youden_norm
            b.target_len = 40.0 + youden_norm * 140.0;
            // Lerp towards target for smooth animation
            b.len += (b.target_len - b.len) * 0.05;

            let x2 = CX + b.angle.cos() * b.len;
            let y2 = CY + b.angle.sin() * b.len;

            ctx.begin_path();
            ctx.move_to(CX, CY);
            ctx.line_to(x2, y2);
            let stroke = format!("hsla({:.0}, 60%, 60%, 0.45)", b.hue);
            ctx.set_stroke_style(&JsValue::from_str(&stroke));
            ctx.set_line_width(1.5);
            ctx.stroke();

            // Small dot at branch tip
            ctx.begin_path();
            let _ = ctx.arc(x2, y2, 2.0, 0.0, std::f64::consts::TAU);
            let tip = format!("hsla({:.0}, 70%, 70%, 0.6)", b.hue);
            ctx.set_fill_style(&JsValue::from_str(&tip));
            ctx.fill();
        }
    });

    // ── Nucleus (pulsing circle, colour = ternary) ─────────────────
    let now_secs = js_sys::Date::now() * 0.001;
    let pulse    = now_secs.sin() * 3.0; // ±3px radius oscillation
    let radius   = 14.0 + pulse;

    // Solid inner circle
    ctx.begin_path();
    let _ = ctx.arc(CX, CY, radius, 0.0, std::f64::consts::TAU);
    ctx.set_fill_style(&JsValue::from_str(nucleus_color));
    ctx.fill();

    // Outer glow ring (27% opacity)
    ctx.begin_path();
    let _ = ctx.arc(CX, CY, radius + 6.0, 0.0, std::f64::consts::TAU);
    let ring = format!("{}44", nucleus_color); // append hex alpha "44" ≈ 27%
    ctx.set_stroke_style(&JsValue::from_str(&ring));
    ctx.set_line_width(2.0);
    ctx.stroke();
}

// ── GardenCanvas component ─────────────────────────────────────────────

#[component]
pub fn GardenCanvas() -> Element {
    // use_hook persists the Interval for the lifetime of the component.
    // The Interval fires draw_frame every 16ms (~60fps) until dropped.
    // Since GardenCanvas lives as long as the app, the interval runs forever.
    let _interval = use_hook(|| {
        gloo_timers::callback::Interval::new(16, draw_frame)
    });

    rsx! {
        div {
            style: "position: relative; flex-shrink: 0;",
            canvas {
                id: "garden-canvas",
                width: "500",
                height: "500",
            }
        }
    }
}
