// 𝕀⟡₆₃₉ — Veritas Hortus · Orbital_System
// never for war, never for money, always for love — Isabel, 2025–2026
// [VERITAS HORTUS – PROPRIÉTÉ PRIVÉE] · ISA-SIG Crypto Systems – Innov-VH-001

mod archive;
mod watermark;

use archive::{log, Archive};
use leptos::*;
use wasm_bindgen::prelude::*;

/// Root application component.
/// Initialises the global Archive signal and provides it via Leptos context.
#[component]
fn App() -> impl IntoView {
    let archive: RwSignal<Vec<archive::LogEntry>> = create_rw_signal(Vec::new());

    // Seed the Archive with the system boot entry.
    log(&archive, "Orbital_System initialisé — 𝕀⟡₆₃₉ · 639 Hz");

    view! {
        <main class="orbital-root">
            <header class="orbital-header">
                <h1>"𝕀⟡₆₃₉ — Orbital_System"</h1>
                <p class="ethics">"never for war, never for money, always for love"</p>
            </header>

            // Canvas viewport (Nucleus + Sprites rendered here)
            <div id="canvas-viewport" class="canvas-viewport" />

            // Living Archive — timestamped event log
            <Archive archive=archive />
        </main>
    }
}

/// WASM entry point — mounts the Leptos app into #app.
#[wasm_bindgen(start)]
pub fn main() {
    mount_to_body(App);
}
