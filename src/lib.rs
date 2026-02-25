mod luna;
mod orbital;
mod spiral;

use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, MouseEvent};

use luna::{Nucleus, Sprites, Chronos, Couleur, Archive, Rituel, Incantation};
use luna::nucleus::EtatNucleus;
use orbital::OrbitalRenderer;
use spiral::SpiralRenderer;

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    Orbital,
    Spiral,
}

#[component]
fn App() -> impl IntoView {
    // Luna state
    let (nucleus, set_nucleus) = create_signal(Nucleus::new(250.0, 250.0));
    let (sprites, set_sprites) = create_signal(Sprites::default_sprites());
    let (chronos, set_chronos) = create_signal(Chronos::new());
    let (couleur, set_couleur) = create_signal(Couleur::new());
    let (archive, set_archive) = create_signal(Archive::new());

    // UI state
    let (view_mode, set_view_mode) = create_signal(ViewMode::Spiral);
    let (rotation, set_rotation) = create_signal(0.0_f64);
    let (zoom, set_zoom) = create_signal(1.0_f64);
    let (frequency, set_frequency) = create_signal(639.0_f64);
    let (auto_rotate, set_auto_rotate) = create_signal(true);
    let (show_labels, set_show_labels) = create_signal(true);
    let (show_particles, set_show_particles) = create_signal(true);
    let (selected_ring, set_selected_ring) = create_signal(Option::<usize>::None);
    let (ritual_input, set_ritual_input) = create_signal(String::new());
    let (ritual_message, set_ritual_message) = create_signal(Option::<String>::None);

    let canvas_ref = create_node_ref::<leptos::html::Canvas>();

    // Spiral renderer (stored in a Cell for mutation in animation loop)
    let spiral_renderer = store_value(std::cell::RefCell::new(SpiralRenderer::new()));

    // Animation loop
    create_effect(move |_| {
        let _ = rotation.get();
        let _ = zoom.get();
        let _ = frequency.get();
        let _ = selected_ring.get();
        let _ = show_labels.get();
        let _ = show_particles.get();
        let _ = view_mode.get();
        let _ = nucleus.get();
        let _ = sprites.get();

        if let Some(canvas) = canvas_ref.get() {
            let canvas_el: &HtmlCanvasElement = &canvas;
            if let Ok(Some(context)) = canvas_el.get_context("2d") {
                let ctx: CanvasRenderingContext2d = context.unchecked_into();
                let w = canvas_el.width() as f64;
                let h = canvas_el.height() as f64;
                let time = js_sys::Date::now() / 1000.0;

                match view_mode.get_untracked() {
                    ViewMode::Orbital => {
                        let nuc = nucleus.get_untracked();
                        let spr = sprites.get_untracked();
                        OrbitalRenderer::draw(&ctx, w, h, &nuc, &spr, time);
                    }
                    ViewMode::Spiral => {
                        spiral_renderer.with_value(|sr| {
                            let mut renderer = sr.borrow_mut();
                            renderer.draw(
                                &ctx, w, h,
                                rotation.get_untracked(),
                                zoom.get_untracked(),
                                frequency.get_untracked(),
                                selected_ring.get_untracked(),
                                show_labels.get_untracked(),
                                show_particles.get_untracked(),
                                time,
                            );
                        });
                    }
                }
            }
        }
    });

    // Animation tick
    let tick_handle = store_value(None::<i32>);
    create_effect(move |_| {
        let active = auto_rotate.get();
        let chr = chronos.get();

        // Clear previous
        if let Some(id) = tick_handle.get_value() {
            let window = web_sys::window().unwrap();
            window.clear_interval_with_handle(id);
        }

        let cb = Closure::<dyn Fn()>::new(move || {
            if active && chr.actif {
                set_rotation.update(|r| *r += 0.003);
                set_sprites.update(|s| s.update_all(0.033, chr.tempo_global));
                set_chronos.update(|c| c.tick());
            }
        });

        let window = web_sys::window().unwrap();
        if let Ok(id) = window.set_interval_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            33,
        ) {
            tick_handle.set_value(Some(id));
        }
        cb.forget();
    });

    // Ritual invocation handler
    let invoke_ritual = move || {
        let input = ritual_input.get_untracked();
        if let Some(inc) = Incantation::from_str(&input) {
            let msg = {
                let mut nuc = nucleus.get_untracked();
                let mut spr = sprites.get_untracked();
                let mut chr = chronos.get_untracked();
                let mut cou = couleur.get_untracked();
                let mut arc = archive.get_untracked();
                let msg = Rituel::invoke(&inc, &mut nuc, &mut spr, &mut chr, &mut cou, &mut arc);
                set_nucleus.set(nuc);
                set_sprites.set(spr);
                set_chronos.set(chr);
                set_couleur.set(cou);
                set_archive.set(arc);
                msg
            };
            set_ritual_message.set(Some(msg));
            set_ritual_input.set(String::new());
        } else if !input.is_empty() {
            set_ritual_message.set(Some(format!("Incantation inconnue: {}", input)));
        }
    };

    let invoke_ritual_clone = invoke_ritual.clone();

    // Canvas click handler for spiral ring selection
    let on_canvas_click = move |ev: MouseEvent| {
        if view_mode.get_untracked() != ViewMode::Spiral { return; }
        if let Some(canvas) = canvas_ref.get_untracked() {
            let canvas_el: &HtmlCanvasElement = &canvas;
            let rect = canvas_el.get_bounding_client_rect();
            let x = ev.client_x() as f64 - rect.left();
            let y = ev.client_y() as f64 - rect.top();
            let center_x = rect.width() / 2.0;
            let center_y = rect.height() / 2.0;
            let dist = ((x - center_x).powi(2) + (y - center_y).powi(2)).sqrt();
            let z = zoom.get_untracked();
            let ring_idx = ((dist / z - 30.0) / 50.0).floor() as i32;
            if ring_idx >= 0 && ring_idx < 7 {
                let idx = ring_idx as usize;
                let cur = selected_ring.get_untracked();
                set_selected_ring.set(if cur == Some(idx) { None } else { Some(idx) });
            } else {
                set_selected_ring.set(None);
            }
        }
    };

    let spiral_data = spiral::SpiralData::default_data();
    let spiral_data_overlay = spiral_data.clone();
    let spiral_data_legend = spiral_data.clone();

    view! {
        <div style="width:100%;height:100vh;background:linear-gradient(135deg,#111827,#000,#1e1b4b);color:white;display:flex;flex-direction:column;font-family:monospace;">

            // Header
            <div style="background:rgba(0,0,0,0.5);border-bottom:1px solid rgba(0,255,255,0.3);padding:12px 16px;display:flex;align-items:center;justify-content:space-between;">
                <div>
                    <h1 style="margin:0;font-size:20px;color:#22d3ee;">"Modularx — Luna Orbital System"</h1>
                    <p style="margin:2px 0 0;font-size:12px;color:#6b7280;">"Pure Rust/WASM · Leptos · Zero JavaScript"</p>
                </div>
                <div style="display:flex;gap:8px;align-items:center;">
                    <button
                        style=move || format!(
                            "padding:6px 14px;border:1px solid {};background:{};color:{};border-radius:4px;cursor:pointer;font-family:monospace;font-size:12px;",
                            if view_mode.get() == ViewMode::Orbital { "#22d3ee" } else { "#374151" },
                            if view_mode.get() == ViewMode::Orbital { "#22d3ee" } else { "transparent" },
                            if view_mode.get() == ViewMode::Orbital { "#000" } else { "#9ca3af" },
                        )
                        on:click=move |_| set_view_mode.set(ViewMode::Orbital)
                    >
                        "Orbital"
                    </button>
                    <button
                        style=move || format!(
                            "padding:6px 14px;border:1px solid {};background:{};color:{};border-radius:4px;cursor:pointer;font-family:monospace;font-size:12px;",
                            if view_mode.get() == ViewMode::Spiral { "#22d3ee" } else { "#374151" },
                            if view_mode.get() == ViewMode::Spiral { "#22d3ee" } else { "transparent" },
                            if view_mode.get() == ViewMode::Spiral { "#000" } else { "#9ca3af" },
                        )
                        on:click=move |_| set_view_mode.set(ViewMode::Spiral)
                    >
                        "Spirale 3D"
                    </button>
                    <span style="color:#22d3ee;font-size:11px;">
                        {move || format!("{:.0} Hz", frequency.get())}
                    </span>
                </div>
            </div>

            // Canvas
            <div style="flex:1;position:relative;overflow:hidden;">
                <canvas
                    node_ref=canvas_ref
                    width="1200"
                    height="800"
                    style="width:100%;height:100%;cursor:pointer;"
                    on:click=on_canvas_click
                />

                // Spiral ring info overlay
                {move || {
                    selected_ring.get().map(|idx| {
                        let ring = &spiral_data_overlay.rings[idx];
                        view! {
                            <div style=format!(
                                "position:absolute;top:12px;right:12px;background:rgba(0,0,0,0.9);border:2px solid {};border-radius:8px;padding:12px;max-width:240px;",
                                ring.color
                            )>
                                <div style="display:flex;align-items:center;gap:8px;margin-bottom:8px;">
                                    <div style=format!("width:14px;height:14px;border-radius:50%;background:{};", ring.color)></div>
                                    <span style=format!("font-weight:bold;font-size:15px;color:{};", ring.color)>
                                        {ring.theme}
                                    </span>
                                </div>
                                <p style="color:#9ca3af;font-size:12px;margin:0 0 8px;">
                                    {format!("Niveau {}", ring.level)}
                                </p>
                                <div style="display:flex;flex-wrap:wrap;gap:4px;">
                                    {ring.concepts.iter().map(|c| view! {
                                        <span style="padding:2px 8px;background:#1f2937;font-size:11px;border-radius:4px;color:#d1d5db;">
                                            {*c}
                                        </span>
                                    }).collect_view()}
                                </div>
                            </div>
                        }
                    })
                }}

                // Ritual floating message
                {move || {
                    ritual_message.get().map(|msg| {
                        view! {
                            <div style="position:absolute;top:12px;left:50%;transform:translateX(-50%);background:rgba(0,0,0,0.85);border:1px solid #22d3ee;border-radius:6px;padding:8px 20px;font-size:13px;color:#22d3ee;">
                                {msg}
                            </div>
                        }
                    })
                }}
            </div>

            // Controls panel
            <div style="background:rgba(17,24,39,0.5);border-top:1px solid rgba(0,255,255,0.3);padding:12px 16px;">

                // Ritual input
                <div style="display:flex;gap:8px;margin-bottom:10px;">
                    <input
                        type="text"
                        placeholder="Incantation (RÉVEIL, CHAOS, HARMONIE...)"
                        style="flex:1;background:#1f2937;border:1px solid #374151;color:white;padding:6px 10px;border-radius:4px;font-family:monospace;font-size:12px;"
                        prop:value=move || ritual_input.get()
                        on:input=move |ev| set_ritual_input.set(event_target_value(&ev))
                        on:keydown=move |ev: web_sys::KeyboardEvent| {
                            if ev.key() == "Enter" { invoke_ritual_clone(); }
                        }
                    />
                    <button
                        style="padding:6px 16px;background:#7c3aed;color:white;border:none;border-radius:4px;cursor:pointer;font-family:monospace;font-size:12px;"
                        on:click=move |_| invoke_ritual()
                    >
                        "Invoquer"
                    </button>
                </div>

                // Sliders row
                <div style="display:grid;grid-template-columns:1fr 1fr 1fr;gap:12px;margin-bottom:10px;">
                    // Zoom
                    <div>
                        <div style="display:flex;justify-content:space-between;margin-bottom:4px;">
                            <span style="color:#22d3ee;font-size:11px;">"Zoom"</span>
                            <span style="color:#22d3ee;font-size:11px;">
                                {move || format!("{:.1}x", zoom.get())}
                            </span>
                        </div>
                        <input
                            type="range" min="0.5" max="2.5" step="0.1"
                            style="width:100%;"
                            prop:value=move || zoom.get().to_string()
                            on:input=move |ev| {
                                if let Ok(v) = event_target_value(&ev).parse::<f64>() { set_zoom.set(v); }
                            }
                        />
                    </div>
                    // Frequency
                    <div>
                        <div style="display:flex;justify-content:space-between;margin-bottom:4px;">
                            <span style="color:#22d3ee;font-size:11px;">"Fréquence"</span>
                            <span style="color:#22d3ee;font-size:11px;">
                                {move || format!("{:.0} Hz", frequency.get())}
                            </span>
                        </div>
                        <input
                            type="range" min="100" max="1000" step="50"
                            style="width:100%;"
                            prop:value=move || frequency.get().to_string()
                            on:input=move |ev| {
                                if let Ok(v) = event_target_value(&ev).parse::<f64>() { set_frequency.set(v); }
                            }
                        />
                    </div>
                    // Tempo
                    <div>
                        <div style="display:flex;justify-content:space-between;margin-bottom:4px;">
                            <span style="color:#22d3ee;font-size:11px;">"Tempo"</span>
                            <span style="color:#22d3ee;font-size:11px;">
                                {move || format!("{:.1}x", chronos.get().tempo_global)}
                            </span>
                        </div>
                        <input
                            type="range" min="0.1" max="3.0" step="0.1"
                            style="width:100%;"
                            prop:value=move || chronos.get().tempo_global.to_string()
                            on:input=move |ev| {
                                if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                                    set_chronos.update(|c| c.tempo_global = v);
                                }
                            }
                        />
                    </div>
                </div>

                // Buttons row
                <div style="display:flex;gap:6px;flex-wrap:wrap;margin-bottom:10px;">
                    // Auto-rotate / Pause
                    <button
                        style=move || format!(
                            "padding:6px 12px;border-radius:4px;border:none;cursor:pointer;font-family:monospace;font-size:11px;{}",
                            if auto_rotate.get() { "background:#06b6d4;color:#000;" } else { "background:#374151;color:#9ca3af;" }
                        )
                        on:click=move |_| set_auto_rotate.update(|v| *v = !*v)
                    >
                        {move || if auto_rotate.get() { "⏸ Pause" } else { "▶ Auto-rotation" }}
                    </button>

                    // Labels toggle
                    <button
                        style=move || format!(
                            "padding:6px 12px;border-radius:4px;border:none;cursor:pointer;font-family:monospace;font-size:11px;{}",
                            if show_labels.get() { "background:#7c3aed;color:white;" } else { "background:#374151;color:#9ca3af;" }
                        )
                        on:click=move |_| set_show_labels.update(|v| *v = !*v)
                    >
                        {move || if show_labels.get() { "Labels ON" } else { "Labels OFF" }}
                    </button>

                    // Particles toggle
                    <button
                        style=move || format!(
                            "padding:6px 12px;border-radius:4px;border:none;cursor:pointer;font-family:monospace;font-size:11px;{}",
                            if show_particles.get() { "background:#db2777;color:white;" } else { "background:#374151;color:#9ca3af;" }
                        )
                        on:click=move |_| set_show_particles.update(|v| *v = !*v)
                    >
                        {move || if show_particles.get() { "Particules ON" } else { "Particules OFF" }}
                    </button>

                    // Nucleus état buttons
                    <button
                        style=move || format!(
                            "padding:6px 12px;border-radius:4px;border:none;cursor:pointer;font-family:monospace;font-size:11px;{}",
                            if nucleus.get().etat == EtatNucleus::Actif { "background:#A855F7;color:white;" } else { "background:#374151;color:#9ca3af;" }
                        )
                        on:click=move |_| set_nucleus.update(|n| n.etat = EtatNucleus::Actif)
                    >
                        "Actif"
                    </button>
                    <button
                        style=move || format!(
                            "padding:6px 12px;border-radius:4px;border:none;cursor:pointer;font-family:monospace;font-size:11px;{}",
                            if nucleus.get().etat == EtatNucleus::Latent { "background:#6366F1;color:white;" } else { "background:#374151;color:#9ca3af;" }
                        )
                        on:click=move |_| set_nucleus.update(|n| n.etat = EtatNucleus::Latent)
                    >
                        "Latent"
                    </button>
                    <button
                        style=move || format!(
                            "padding:6px 12px;border-radius:4px;border:none;cursor:pointer;font-family:monospace;font-size:11px;{}",
                            if nucleus.get().etat == EtatNucleus::Instable { "background:#EF4444;color:white;" } else { "background:#374151;color:#9ca3af;" }
                        )
                        on:click=move |_| set_nucleus.update(|n| n.etat = EtatNucleus::Instable)
                    >
                        "Instable"
                    </button>

                    // Reset
                    <button
                        style="padding:6px 12px;border-radius:4px;border:none;cursor:pointer;font-family:monospace;font-size:11px;background:#374151;color:white;"
                        on:click=move |_| {
                            set_selected_ring.set(None);
                            set_ritual_message.set(None);
                        }
                    >
                        "Reset"
                    </button>
                </div>

                // Spiral ring legend + Archive
                <div style="display:flex;gap:12px;">
                    // Ring legend
                    <div style="display:flex;gap:4px;flex-wrap:wrap;flex:1;">
                        {spiral_data_legend.rings.iter().enumerate().map(|(i, ring)| {
                            let color = ring.color.to_string();
                            let theme = ring.theme.to_string();
                            let color_clone = color.clone();
                            view! {
                                <button
                                    style=move || format!(
                                        "padding:3px 10px;border-radius:4px;border:none;cursor:pointer;font-family:monospace;font-size:10px;display:flex;align-items:center;gap:4px;{}",
                                        if selected_ring.get() == Some(i) { "background:white;color:black;" } else { "background:#1f2937;color:#9ca3af;" }
                                    )
                                    on:click=move |_| set_selected_ring.set(Some(i))
                                >
                                    <span style=format!("display:inline-block;width:8px;height:8px;border-radius:50%;background:{};", color_clone)></span>
                                    {theme.clone()}
                                </button>
                            }
                        }).collect_view()}
                    </div>

                    // Archive (last 3 entries)
                    <div style="min-width:260px;max-height:60px;overflow-y:auto;background:#0f172a;border-radius:4px;padding:4px 8px;">
                        <div style="font-size:9px;color:#6b7280;margin-bottom:2px;">"Archive"</div>
                        {move || {
                            let arc = archive.get();
                            if arc.entries.is_empty() {
                                view! { <div style="font-size:10px;color:#4b5563;">"Aucune trace..."</div> }.into_view()
                            } else {
                                arc.entries.iter().take(3).map(|e| {
                                    view! {
                                        <div style="font-size:10px;color:#9ca3af;">
                                            <span style="color:#22d3ee;">{e.moment.clone()}</span>
                                            " — "
                                            {e.trace.clone()}
                                        </div>
                                    }
                                }).collect_view().into_view()
                            }
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    mount_to_body(App);
}
