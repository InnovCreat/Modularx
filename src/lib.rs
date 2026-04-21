// Luna — Entry point
// Environnement déclaré en premier. Triggers only ici.

use leptos::*;
use wasm_bindgen::prelude::*;

mod artifact;
mod luna;
mod orchestrator;

use luna::{
    nucleus::{Nucleus, NucleusState},
    sprite::{default_sprites, Sprite, QuantumState},
    crystal::Crystal,
    pulse::{can_resonate, angle_to_pan},
};

// ─── Entry point ─────────────────────────────────────────────────────────────

#[wasm_bindgen(start)]
pub fn main() {
    mount_to_body(|| view! { <LunaApp /> });
}

// ─── Root component ───────────────────────────────────────────────────────────

#[component]
fn LunaApp() -> impl IntoView {
    let nucleus = create_rw_signal(Nucleus::default());
    let sprites = create_rw_signal(default_sprites());
    let crystal = create_rw_signal(Crystal::default());
    let tempo   = create_rw_signal(1.0_f64);
    let rituel  = create_rw_signal(String::new());
    let log     = create_rw_signal(Vec::<String>::new());

    // Invocation handler — no magic, explicit trigger
    let invoke = move |_| {
        let keyword = rituel.get();
        let kw = keyword.trim().to_uppercase();
        if kw.is_empty() { return; }

        nucleus.update(|n| n.apply_rituel(&kw));
        sprites.update(|ss| apply_rituel_to_sprites(ss, &kw));

        log.update(|l| {
            let entry = format!("{} — Rituel {} déclenché", timestamp_now(), kw);
            l.insert(0, entry);
            l.truncate(10);
        });

        rituel.set(String::new());
    };

    view! {
        <div class="luna-root">
            <NucleusPanel nucleus=nucleus crystal=crystal />
            <SpritesPanel sprites=sprites />
            <ControlsPanel
                tempo=tempo
                rituel=rituel
                on_invoke=invoke
            />
            <ArchivePanel log=log />
        </div>
    }
}

// ─── Nucleus panel ────────────────────────────────────────────────────────────

#[component]
fn NucleusPanel(nucleus: RwSignal<Nucleus>, crystal: RwSignal<Crystal>) -> impl IntoView {
    let color       = move || nucleus.get().color().to_string();
    let state_label = move || match nucleus.get().state {
        NucleusState::Actif    => "actif",
        NucleusState::Latent   => "latent",
        NucleusState::Instable => "instable",
    };
    let pulsation   = move || nucleus.get().pulsation as u32;
    let gravite     = move || nucleus.get().gravite;
    let resonating  = move || can_resonate(crystal.get().coherence);

    view! {
        <div class="nucleus-panel">
            <h2 style=move || format!("color: {}", color())>
                "Nucleus — " {state_label}
            </h2>
            <p>"Pulsation : " {pulsation} " bpm"</p>
            <p>"Gravité : "   {gravite}</p>
            <p>"Résonance : " {move || if resonating() { "✓ 639 Hz" } else { "—" }}</p>
        </div>
    }
}

// ─── Sprites panel ────────────────────────────────────────────────────────────

#[component]
fn SpritesPanel(sprites: RwSignal<Vec<Sprite>>) -> impl IntoView {
    view! {
        <div class="sprites-panel">
            <h2>"Sprites"</h2>
            <For
                each={move || sprites.get()}
                key={|s: &Sprite| s.nom.clone()}
                children={move |s| {
                    let quantum = match s.etat_quantique {
                        QuantumState::Stable    => "stable",
                        QuantumState::Superpose => "superposé",
                    };
                    let pan = angle_to_pan(s.angle);
                    view! {
                        <div class="sprite-row" style=format!("color: {}", s.couleur)>
                            <span>{s.nom.clone()}</span>
                            <span>" · " {s.angle as u32} "°"</span>
                            <span>" · " {quantum}</span>
                            <span>" · pan " {format!("{:.2}", pan)}</span>
                        </div>
                    }
                }}
            />
        </div>
    }
}

// ─── Controls panel ───────────────────────────────────────────────────────────

#[component]
fn ControlsPanel(
    tempo:     RwSignal<f64>,
    rituel:    RwSignal<String>,
    on_invoke: impl Fn(web_sys::MouseEvent) + 'static,
) -> impl IntoView {
    view! {
        <div class="controls-panel">
            <div class="tempo-control">
                <label>"Tempo " {move || format!("{:.1}x", tempo.get())}</label>
                <input
                    type="range" min="0.1" max="3.0" step="0.1"
                    prop:value=move || tempo.get().to_string()
                    on:input=move |e| {
                        if let Ok(v) = event_target_value(&e).parse::<f64>() {
                            tempo.set(v);
                        }
                    }
                />
            </div>
            <div class="rituel-control">
                <input
                    type="text"
                    placeholder="RÉVEIL · CHAOS · RÉPARATION…"
                    prop:value=move || rituel.get()
                    on:input=move |e| rituel.set(event_target_value(&e))
                    on:keydown=move |e| {
                        if e.key() == "Enter" {
                            let btn = e.target()
                                .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok());
                            if let Some(_) = btn {
                                // trigger via click simulation not needed — handled in invoke
                            }
                        }
                    }
                />
                <button on:click=on_invoke>"Invoquer"</button>
            </div>
        </div>
    }
}

// ─── Archive panel ────────────────────────────────────────────────────────────

#[component]
fn ArchivePanel(log: RwSignal<Vec<String>>) -> impl IntoView {
    view! {
        <div class="archive-panel">
            <h2>"Archive"</h2>
            <ul>
                <For
                    each={move || log.get()}
                    key={|entry: &String| entry.clone()}
                    children={move |entry| view! { <li>{entry}</li> }}
                />
            </ul>
        </div>
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn apply_rituel_to_sprites(sprites: &mut Vec<Sprite>, keyword: &str) {
    match keyword {
        "CHAOS" => {
            for s in sprites.iter_mut() {
                s.etat_quantique = QuantumState::Superpose;
                s.opacite = 0.6;
            }
        }
        "HARMONIE" | "RÉPARATION" => {
            for s in sprites.iter_mut() {
                s.etat_quantique  = QuantumState::Stable;
                s.vitesse_rotation = if s.vitesse_rotation < 0.0 { -30.0 } else { 30.0 };
                s.opacite = 1.0;
            }
        }
        "ATTRACTION" => {
            for s in sprites.iter_mut() {
                s.distance_centre = (s.distance_centre - 30.0).max(50.0);
            }
        }
        "RÉPULSION" => {
            for s in sprites.iter_mut() {
                s.distance_centre = (s.distance_centre + 30.0).min(200.0);
            }
        }
        _ => {}
    }
}

fn timestamp_now() -> String {
    // Dans le contexte WASM, Date.now() via js_sys serait utilisé en prod
    // Placeholder propre pour les tests
    "00:00:00".to_string()
}
