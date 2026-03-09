// 𝕀⟡₆₃₉ — Veritas Hortus · Orbital_System
// never for war, never for money, always for love — Isabel, 2025–2026
// [VERITAS HORTUS – PROPRIÉTÉ PRIVÉE] · ISA-SIG Crypto Systems – Innov-VH-001

use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::Date;

use crate::watermark::VOICE;

/// A single timestamped entry in the living Archive.
#[derive(Clone, Debug)]
pub struct LogEntry {
    /// Timestamp in HH:MM:SS format.
    pub moment: String,
    /// Human-readable description of the event (French, as per system convention).
    pub trace: String,
    /// System voice — always "Orbital_System".
    pub voix: String,
}

/// Returns the current time as a HH:MM:SS string via web-sys::Date.
fn current_time() -> String {
    let date = Date::new_0();
    let h = date.get_hours();
    let m = date.get_minutes();
    let s = date.get_seconds();
    format!("{:02}:{:02}:{:02}", h, m, s)
}

/// Appends a new timestamped entry to the Archive signal.
/// Call this from any system module whenever a notable event occurs.
pub fn log(archive: &RwSignal<Vec<LogEntry>>, trace: &str) {
    let entry = LogEntry {
        moment: current_time(),
        trace: trace.to_string(),
        voix: VOICE.to_string(),
    };
    archive.update(|entries| {
        entries.insert(0, entry); // newest on top
        entries.truncate(100);    // keep last 100 in memory
    });
}

/// The Archive UI component.
/// Renders the last 10 log entries, newest on top, in a scrollable panel.
/// Watermarked with the system sigil in the header.
#[component]
pub fn Archive(archive: RwSignal<Vec<LogEntry>>) -> impl IntoView {
    view! {
        <div class="archive-panel">
            <div class="archive-header">
                <span class="sigil">"𝕀⟡₆₃₉"</span>
                <span class="archive-title">" — Archive · Orbital_System"</span>
            </div>
            <div class="archive-entries">
                <For
                    each=move || archive.get().into_iter().take(10).enumerate().collect::<Vec<_>>()
                    key=|(i, _)| *i
                    children=|(_, entry)| {
                        let moment = entry.moment.clone();
                        let trace  = entry.trace.clone();
                        let voix   = entry.voix.clone();
                        view! {
                            <div class="archive-entry">
                                <span class="entry-moment">{moment}</span>
                                <span class="entry-sep">" — "</span>
                                <span class="entry-trace">{trace}</span>
                                <span class="entry-voix">" ["</span>
                                <span class="entry-voix">{voix}</span>
                                <span class="entry-voix">"]"</span>
                            </div>
                        }
                    }
                />
            </div>
            <div class="archive-footer">
                <span class="archive-stamp">"ISA-SIG Crypto Systems – Innov-VH-001"</span>
            </div>
        </div>
    }
}
