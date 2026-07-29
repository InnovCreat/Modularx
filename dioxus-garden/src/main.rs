// dioxus-garden/src/main.rs
// SYNAPS VM — Veritas Hortus Garden (Dioxus 0.7 / web)
//
// Build:
//   cd dioxus-garden
//   cargo install dioxus-cli          # install dx CLI (once)
//   dx serve --platform web           # → http://127.0.0.1:8080
//
// Setup (3 terminals, from repo root):
//   Terminal 1: rustc main.rs -O -o synaps_vm && ./synaps_vm
//   Terminal 2: python3 -m http.server 8090   (serves state.json)
//   Terminal 3: cd dioxus-garden && dx serve  (proxy → 8090 for /state.json)

#![allow(non_snake_case)]

use dioxus::prelude::*;

mod state;
mod components;

use state::{VmState, VM_STATE, LAST_REWIRES, fetch_state};
use components::canvas::{GardenCanvas, update_canvas_state};
use components::dashboard::Dashboard;
use components::son639;

fn main() {
    dioxus::launch(App);
}

// ── App — root component ───────────────────────────────────────────────

#[component]
fn App() -> Element {
    // ── Polling coroutine — runs for the full page lifetime ──────────
    //
    // Every 100ms:
    //   1. Fetch state.json from the dev proxy (/state.json → :8090)
    //   2. Update VM_STATE GlobalSignal → Dashboard re-renders reactively
    //   3. Update canvas thread_local → draw_frame picks it up at next tick
    //   4. Trigger audio transitions (morph, urgency, halt)
    //   5. Update rewire counter for visual flash detection

    use_coroutine(|_: UnboundedReceiver<()>| async move {
        let mut last_ternary = "NEU".to_string();

        loop {
            if let Some(s) = fetch_state().await {
                // ── Canvas snapshot ──────────────────────────────────
                update_canvas_state(s.youden, s.ternary.clone());

                // ── Audio triggers ───────────────────────────────────
                if son639::is_running() {
                    // 1. Ternary change → frequency morph (linearRamp 1.5s)
                    //    NEU=639 Hz · POS=528 Hz · NEG=396 Hz
                    if s.ternary != last_ternary {
                        let hz: f32 = match s.ternary.as_str() {
                            "POS" => 528.0,
                            "NEG" => 396.0,
                            _     => 639.0,
                        };
                        son639::morph_to(hz, 1.5);
                    }

                    // 2. Urgency → subtle vibrato ±8 Hz when VM is stressed
                    son639::set_urgency(s.urgency > 0.0);

                    // 3. VM halt → fade out audio over 2s
                    if s.state == "halted" {
                        son639::fade_out();
                    }
                }

                last_ternary = s.ternary.clone();

                // ── Rewire counter ───────────────────────────────────
                // Components reading LAST_REWIRES can detect new rewires
                // and trigger their own visual effects (e.g., flash lines)
                *LAST_REWIRES.write() = s.rewires;

                // ── Update global state (triggers Dashboard re-render) ──
                *VM_STATE.write() = Some(s);

            } else {
                // state.json not available → show "Waiting for VM" message
                *VM_STATE.write() = None;
            }

            // Wait 100ms before next fetch — same cadence as garden.html
            gloo_timers::future::TimeoutFuture::new(100).await;
        }
    });

    rsx! {
        style { {CSS} }
        div { class: "layout",
            GardenCanvas {}
            Dashboard {}
        }
    }
}

// ── Inline CSS — mirrors garden.html aesthetic exactly ────────────────

const CSS: &str = r#"
* { margin: 0; padding: 0; box-sizing: border-box; }

body {
  background: #0a0a12;
  color: #c8b8e8;
  font-family: 'Courier New', monospace;
}

.layout {
  display: flex;
  align-items: flex-start;
  justify-content: center;
  min-height: 100vh;
  padding: 24px;
  gap: 32px;
}

#garden-canvas {
  display: block;
  border: 1px solid #2a1a4a;
  border-radius: 4px;
}

.panel {
  width: 280px;
  padding-top: 4px;
  font-size: 11px;
}

h1 {
  font-size: 14px;
  color: #b88aff;
  letter-spacing: 2px;
  margin-bottom: 4px;
}

.subtitle {
  font-size: 10px;
  color: #5a4a7a;
  margin-bottom: 16px;
}

/* Son639 button */
.audio-btn {
  display: block;
  width: 100%;
  background: rgba(180,100,255,0.06);
  border: 1px solid rgba(180,100,255,0.2);
  color: #7a6aaa;
  font-family: inherit;
  font-size: 11px;
  letter-spacing: 2px;
  padding: 6px;
  cursor: pointer;
  margin-bottom: 16px;
  transition: background 0.2s, color 0.2s;
}
.audio-btn:hover { background: rgba(180,100,255,0.14); color: #b88aff; }
.audio-btn.active { border-color: rgba(100,255,180,0.35); color: #44ff88; }

/* Ternary badge */
.badge {
  display: inline-block;
  padding: 4px 14px;
  border-radius: 20px;
  font-size: 13px;
  font-weight: bold;
  letter-spacing: 3px;
  margin-bottom: 16px;
  transition: background 0.4s, color 0.4s;
}
.badge-neg { background: #2a0a0a; color: #ff4444; border: 1px solid #ff3333; }
.badge-neu { background: #1a180a; color: #ffcc44; border: 1px solid #ffbb22; }
.badge-pos { background: #0a1a0a; color: #44ff88; border: 1px solid #33ff77; }

/* Info rows */
.info-row {
  display: flex;
  justify-content: space-between;
  color: #6655aa;
  margin-bottom: 6px;
}
.info-row span { color: #aa99cc; }

/* Metric bars */
.metric { margin-bottom: 14px; }
.metric-label {
  display: flex;
  justify-content: space-between;
  color: #8877aa;
  margin-bottom: 3px;
}
.metric-label span { color: #d0c0f0; }
.bar-track {
  height: 8px;
  background: #1a1228;
  border-radius: 4px;
  overflow: hidden;
}
.bar-fill {
  height: 100%;
  border-radius: 4px;
  transition: width 0.2s ease, background 0.3s ease;
}

/* Archive */
.archive {
  margin-top: 20px;
  border-top: 1px solid #1a1228;
  padding-top: 12px;
}
.archive-title {
  font-size: 10px;
  color: #5a4a7a;
  letter-spacing: 2px;
  margin-bottom: 8px;
}
.archive-entry {
  font-size: 10px;
  color: #5a4a7a;
  margin-bottom: 5px;
  line-height: 1.4;
}
.archive-ts  { color: #3a2a5a; margin-right: 6px; }
.archive-msg { color: #7a6a9a; }

/* Waiting message */
.waiting {
  font-size: 11px;
  color: #4a3a6a;
  margin-top: 30px;
  text-align: center;
}
"#;
