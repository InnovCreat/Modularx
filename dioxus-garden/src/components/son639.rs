// dioxus-garden/src/components/son639.rs
// Web Audio API — Son639 porté de garden.html en Rust/web-sys
//
// Architecture : état audio dans un thread_local! (WASM = single thread).
// Le composant Son639 affiche uniquement le bouton toggle.
// Les triggers (morph, urgency, fadeOut) sont appelés depuis le
// coroutine principal dans main.rs — même logique que le JS d'origine.

use dioxus::prelude::*;
use std::cell::RefCell;
use wasm_bindgen::JsValue;

// ── Audio state ────────────────────────────────────────────────────────

struct AudioCtx {
    ctx:  web_sys::AudioContext,
    osc:  web_sys::OscillatorNode,
    gain: web_sys::GainNode,
}

thread_local! {
    // Holds the AudioContext + oscillator + gain after first init().
    // None until the user clicks the button (browser autoplay policy).
    static AUDIO: RefCell<Option<AudioCtx>> = RefCell::new(None);

    // Separate flag so we can check running state without borrowing AUDIO.
    static RUNNING: RefCell<bool> = RefCell::new(false);
    static MUTED:   RefCell<bool> = RefCell::new(false);
}

// ── Public API (called from main.rs coroutine) ─────────────────────────

/// Returns true if the oscillator is running (user has clicked the button).
pub fn is_running() -> bool {
    RUNNING.with(|r| *r.borrow())
}

/// Glide to a new frequency over `duration` seconds (linearRamp).
/// Equivalent to JS: osc.frequency.linearRampToValueAtTime(hz, ctx.currentTime + duration)
pub fn morph_to(hz: f32, duration: f64) {
    AUDIO.with(|a| {
        if let Some(audio) = a.borrow().as_ref() {
            let end_time = audio.ctx.current_time() + duration;
            // Result ignored — failure is non-fatal (page keeps working)
            let _ = audio.osc.frequency().linear_ramp_to_value_at_time(hz, end_time);
        }
    });
}

/// Apply a small vibrato (±8 Hz, 0.8 Hz modulation rate) when urgency > 0.
/// Uses a sinusoidal offset driven by wall-clock time so it evolves each tick.
pub fn set_urgency(on: bool) {
    // Skip if muted — no point modulating an inaudible signal
    if MUTED.with(|m| *m.borrow()) { return; }

    AUDIO.with(|a| {
        if let Some(audio) = a.borrow().as_ref() {
            // Read the current oscillator frequency to compute offset
            let base = audio.osc.frequency().value() as f64;
            let target = if on {
                // sin(Date.now() * 0.005) → ~0.8 Hz oscillation between -1 and +1
                let wobble = (js_sys::Date::now() * 0.005).sin() * 8.0;
                (base + wobble) as f32
            } else {
                base as f32
            };
            // setTargetAtTime — exponential convergence (smoother than linearRamp here)
            let _ = audio.osc.frequency()
                .set_target_at_time(target, audio.ctx.current_time(), 0.3);
        }
    });
}

/// Fade out the gain over 2 seconds, then close the AudioContext.
/// Called when state.json reports state == "halted".
pub fn fade_out() {
    AUDIO.with(|a| {
        if let Some(audio) = a.borrow().as_ref() {
            let end_time = audio.ctx.current_time() + 2.0;
            let _ = audio.gain.gain().linear_ramp_to_value_at_time(0.0, end_time);
            // Schedule ctx.close() after the ramp finishes (2.2s)
            // We use gloo_timers for the deferred close
            gloo_timers::callback::Timeout::new(2200, || {
                AUDIO.with(|a| {
                    if let Some(audio) = a.borrow().as_ref() {
                        let _ = audio.ctx.close();
                    }
                });
                RUNNING.with(|r| *r.borrow_mut() = false);
            }).forget(); // `.forget()` — timer runs independently, no owner needed
        }
    });
}

// ── init() — called once on first button click ─────────────────────────

fn init() -> Result<(), JsValue> {
    // Create the AudioContext (browser creates audio thread)
    let ctx = web_sys::AudioContext::new()?;

    // OscillatorNode — generates a pure sine wave
    let osc = ctx.create_oscillator()?;
    osc.set_type(web_sys::OscillatorType::Sine);

    // Set initial frequency to 639 Hz (Veritas Hortus base frequency)
    // AudioParam values are f32, timing params are f64
    osc.frequency().set_value(639.0_f32);

    // GainNode — controls volume
    // 0.04 = 4% of max — very subtle, present but not distracting
    let gain = ctx.create_gain()?;
    gain.gain().set_value(0.04_f32);

    // Connect the graph:  osc → gain → destination (speakers)
    // AudioNode::connect_with_audio_node() chains nodes
    osc.connect_with_audio_node(&gain)?;
    gain.connect_with_audio_node(&ctx.destination())?;

    // Start the oscillator — it runs continuously until ctx.close()
    // After start(), the oscillator CANNOT be restarted (Web Audio spec)
    osc.start()?;

    AUDIO.with(|a| {
        *a.borrow_mut() = Some(AudioCtx { ctx, osc, gain });
    });
    RUNNING.with(|r| *r.borrow_mut() = true);
    MUTED.with(|m| *m.borrow_mut() = false);

    Ok(())
}

// ── toggle() — mute / unmute (or init on first call) ──────────────────

/// Called by the button's onclick handler.
/// Returns (is_active, is_muted) for button label update.
fn toggle() -> (bool, bool) {
    if !RUNNING.with(|r| *r.borrow()) {
        // First click — initialize audio
        if init().is_ok() {
            return (true, false);
        } else {
            return (false, false);
        }
    }

    // Toggle mute state
    let new_muted = MUTED.with(|m| {
        let mut muted = m.borrow_mut();
        *muted = !*muted;
        *muted
    });

    AUDIO.with(|a| {
        if let Some(audio) = a.borrow().as_ref() {
            let target_gain = if new_muted { 0.0_f32 } else { 0.04_f32 };
            // setTargetAtTime with timeConstant=0.1s — smooth, no click/pop
            let _ = audio.gain.gain()
                .set_target_at_time(target_gain, audio.ctx.current_time(), 0.1);
        }
    });

    (true, new_muted)
}

// ── Son639 component — just the button ────────────────────────────────

#[component]
pub fn Son639() -> Element {
    // Local signals track button display state (not audio state itself)
    let mut active = use_signal(|| false);
    let mut muted  = use_signal(|| false);

    let onclick = move |_| {
        let (new_active, new_muted) = toggle();
        active.set(new_active);
        muted.set(new_muted);
    };

    let btn_class = if active() && !muted() {
        "audio-btn active"
    } else {
        "audio-btn"
    };

    let btn_label = if !active() {
        "🔇 ACTIVER 639 Hz"
    } else if muted() {
        "🔇 639 Hz — OFF"
    } else {
        "🔊 639 Hz — ON"
    };

    rsx! {
        button {
            class: "{btn_class}",
            onclick: onclick,
            "{btn_label}"
        }
    }
}
