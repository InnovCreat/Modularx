// dioxus-garden/src/components/dashboard.rs
// Right-side panel — metrics, ternary badge, archive.
// Mirrors the #panel div of garden.html.

#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::state::VM_STATE;
use super::son639::Son639;

// ── Dashboard (root panel component) ──────────────────────────────────

#[component]
pub fn Dashboard() -> Element {
    let state = VM_STATE.read();

    // Derive display values from the current state (or defaults if None)
    let ternary   = state.as_ref().map(|s| s.ternary.as_str()).unwrap_or("NEU");
    let vm_state  = state.as_ref().map(|s| s.state.as_str()).unwrap_or("—");
    let frame     = state.as_ref().map(|s| s.frame.to_string()).unwrap_or_else(|| "—".into());
    let pc        = state.as_ref().map(|s| s.pc.to_string()).unwrap_or_else(|| "—".into());
    let threshold = state.as_ref()
        .map(|s| format!("{:.3}", s.threshold))
        .unwrap_or_else(|| "—".into());
    let rewires   = state.as_ref().map(|s| s.rewires.to_string()).unwrap_or_else(|| "0".into());

    let youden      = state.as_ref().map(|s| s.youden).unwrap_or(0.0);
    let auc         = state.as_ref().map(|s| s.auc).unwrap_or(0.0);
    let sensitivity = state.as_ref().map(|s| s.sensitivity).unwrap_or(0.0);
    let specificity = state.as_ref().map(|s| s.specificity).unwrap_or(0.0);

    let archive     = state.as_ref()
        .map(|s| s.archive.clone())
        .unwrap_or_default();

    let show_waiting = state.is_none();

    // Badge CSS class
    let badge_class = match ternary {
        "POS" => "badge badge-pos",
        "NEG" => "badge badge-neg",
        _     => "badge badge-neu",
    };

    rsx! {
        div { class: "panel",

            h1 { "SYNAPS VM" }
            div { class: "subtitle", "639 Hz · Always for Love" }

            // Audio toggle button
            Son639 {}

            // Ternary badge (NEG / NEU / POS)
            div { class: "{badge_class}", "{ternary}" }

            // VM info rows
            div { class: "info-row", "State"     span { "{vm_state}" } }
            div { class: "info-row", "Frame"     span { "{frame}" } }
            div { class: "info-row", "PC"        span { "{pc}" } }
            div { class: "info-row", "Threshold" span { "{threshold}" } }
            div { class: "info-row", "Rewires"   span { "{rewires}" } }

            // Metric bars
            // Youden is in [-1, 1] — normalize to [0, 1] for the bar width
            MetricBar { label: "Youden Index", value: youden, norm: (youden + 1.0) / 2.0 }
            MetricBar { label: "AUC",          value: auc,         norm: auc }
            MetricBar { label: "Sensitivity",  value: sensitivity, norm: sensitivity }
            MetricBar { label: "Specificity",  value: specificity, norm: specificity }

            // Archive — last 6 entries from SYNAPS VM
            div { class: "archive",
                div { class: "archive-title", "ARCHIVE" }
                for entry in archive.iter().take(6) {
                    div { class: "archive-entry",
                        span { class: "archive-ts",  "{entry.moment}" }
                        span { class: "archive-msg", " {entry.trace}" }
                    }
                }
            }

            // Waiting message — shown when state.json not yet available
            if show_waiting {
                div { class: "waiting",
                    "Waiting for VM…"
                    br {}
                    "Run: "
                    code { "./synaps_vm" }
                }
            }
        }
    }
}

// ── MetricBar — coloured progress bar ─────────────────────────────────

#[component]
fn MetricBar(label: String, value: f64, norm: f64) -> Element {
    let pct = (norm.clamp(0.0, 1.0) * 100.0) as u32;

    // Green > 70%, Yellow 40–70%, Red < 40% — matches garden.html barColor()
    let color = if norm > 0.7      { "#44ff88" }
                else if norm > 0.4 { "#ffcc44" }
                else               { "#ff4444" };

    let val_str = format!("{:.4}", value);

    rsx! {
        div { class: "metric",
            div { class: "metric-label",
                "{label}"
                span { "{val_str}" }
            }
            div { class: "bar-track",
                div {
                    class: "bar-fill",
                    style: "width: {pct}%; background: {color};",
                }
            }
        }
    }
}
