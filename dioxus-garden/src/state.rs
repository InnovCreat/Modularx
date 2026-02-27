// dioxus-garden/src/state.rs
// Mirrors the JSON fields exported by SYNAPS VM (main.rs → state.json)

use dioxus::prelude::*;
use serde::Deserialize;

// URL from which the Dioxus app fetches the VM state.
// With the [[web.proxy]] in Dioxus.toml, /state.json is proxied
// from the Python http.server running on port 8090.
const STATE_JSON_URL: &str = "/state.json";

// ── Structs (must match exactly the JSON keys in export_state()) ──────

#[derive(Deserialize, Clone, Default, PartialEq)]
pub struct VmState {
    pub frame:       u64,
    pub pc:          usize,
    pub state:       String,   // "running" | "paused" | "stasis" | "halted"
    pub ternary:     String,   // "NEG" | "NEU" | "POS"
    pub threshold:   f64,
    pub rewires:     u64,
    pub youden:      f64,      // range [-1, 1] → normalized to [0, 1] for bars
    pub auc:         f64,
    pub sensitivity: f64,
    pub specificity: f64,
    pub urgency:     f64,      // 1.0 if Youden < 0.3, else 0.0
    pub stack_depth: usize,
    pub archive:     Vec<ArchiveEntry>,
}

#[derive(Deserialize, Clone, Default, PartialEq)]
pub struct ArchiveEntry {
    pub moment: String,    // "HH:MM:SS"
    pub trace:  String,    // human-readable event description
    pub voix:   String,    // "SYNAPS_VM"
}

// ── Global signals (read by Dashboard, Canvas, Son639) ────────────────

/// Current VM state — None while waiting for synaps_vm to start.
pub static VM_STATE: GlobalSignal<Option<VmState>> = Signal::global(|| None);

/// Last rewire count — used to detect new rewires for flash effect.
pub static LAST_REWIRES: GlobalSignal<u64> = Signal::global(|| 0);

// ── HTTP fetch (called from App coroutine every 100ms) ─────────────────

pub async fn fetch_state() -> Option<VmState> {
    // Cache-busting: append ?t=timestamp to avoid browser caching
    let ts = js_sys::Date::now() as u64;
    let url = format!("{}?t={}", STATE_JSON_URL, ts);

    let resp = gloo_net::http::Request::get(&url)
        .send()
        .await
        .ok()?;

    if !resp.ok() {
        return None;
    }

    resp.json::<VmState>().await.ok()
}
