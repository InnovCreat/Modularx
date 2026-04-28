use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorType};
use std::cell::RefCell;

// Shared AudioContext — created once per page load, reused for all notes.
// Creating a new one per note exhausts browser limits quickly.
thread_local! {
    static AUDIO_CTX: RefCell<Option<AudioContext>> = RefCell::new(None);
}

fn get_or_init_ctx() -> Result<AudioContext, JsValue> {
    AUDIO_CTX.with(|cell| {
        let mut borrow = cell.borrow_mut();
        if borrow.is_none() {
            *borrow = Some(AudioContext::new()?);
        }
        Ok(borrow.as_ref().unwrap().clone())
    })
}

/// ADSR envelope parameters (times in seconds, levels 0.0–1.0)
#[derive(Clone, Copy, Debug)]
pub struct Adsr {
    pub attack:  f32,
    pub decay:   f32,
    pub sustain: f32,
    pub release: f32,
    pub volume:  f32,
}

impl Default for Adsr {
    fn default() -> Self {
        Adsr { attack: 0.01, decay: 0.1, sustain: 0.6, release: 0.3, volume: 0.8 }
    }
}

impl Adsr {
    pub fn from_instrument(inst: u8) -> Self {
        // Per-instrument defaults tuned to the Solfège table
        match inst {
            0 => Adsr { attack: 0.01, decay: 0.15, sustain: 0.5, release: 0.4, volume: 0.75 }, // UT
            1 => Adsr { attack: 0.02, decay: 0.12, sustain: 0.55, release: 0.35, volume: 0.75 }, // RE
            2 => Adsr { attack: 0.01, decay: 0.10, sustain: 0.65, release: 0.30, volume: 0.80 }, // MI
            3 => Adsr { attack: 0.01, decay: 0.08, sustain: 0.70, release: 0.25, volume: 0.85 }, // FA 639Hz
            4 => Adsr { attack: 0.02, decay: 0.12, sustain: 0.60, release: 0.30, volume: 0.78 }, // SOL
            5 => Adsr { attack: 0.03, decay: 0.14, sustain: 0.58, release: 0.40, volume: 0.75 }, // LA
            6 => Adsr { attack: 0.01, decay: 0.10, sustain: 0.62, release: 0.35, volume: 0.80 }, // SI
            7 => Adsr { attack: 0.05, decay: 0.20, sustain: 0.50, release: 0.50, volume: 0.70 }, // ISA
            8 => Adsr { attack: 0.10, decay: 0.30, sustain: 0.40, release: 0.80, volume: 0.65 }, // THI (sub)
            _ => Adsr::default(),
        }
    }
}

/// Oscillator waveform type
#[derive(Clone, Copy, Debug)]
pub enum WaveShape {
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

impl WaveShape {
    pub fn to_osc_type(self) -> OscillatorType {
        match self {
            WaveShape::Sine     => OscillatorType::Sine,
            WaveShape::Square   => OscillatorType::Square,
            WaveShape::Sawtooth => OscillatorType::Sawtooth,
            WaveShape::Triangle => OscillatorType::Triangle,
        }
    }
}

/// Play a single note with full ADSR envelope.
///
/// `freq`     — frequency in Hz
/// `duration` — note duration in seconds (= row length at current BPM)
/// `adsr`     — envelope parameters
/// `wave`     — oscillator waveform
pub fn play_tone(
    freq: f32,
    duration: f32,
    adsr: Adsr,
    wave: WaveShape,
) -> Result<(), JsValue> {
    let ctx = get_or_init_ctx()?;
    let now = ctx.current_time() as f32;

    let osc  = ctx.create_oscillator()?;
    let gain = ctx.create_gain()?;

    // Wire: osc → gain → output
    osc.connect_with_audio_node(&gain)?;
    gain.connect_with_audio_node(&ctx.destination())?;

    // Waveform
    osc.set_type(wave.to_osc_type());
    osc.frequency().set_value(freq);

    // ── ADSR envelope ──────────────────────────────────────────────────────
    // Start at 0 (prevents click on note-on)
    gain.gain().set_value_at_time(0.0, now as f64)?;

    // Attack: ramp up to peak volume
    let t_peak = now + adsr.attack;
    gain.gain().linear_ramp_to_value_at_time(adsr.volume, t_peak as f64)?;

    // Decay: ramp down to sustain level
    let t_sustain = t_peak + adsr.decay;
    let sustain_level = adsr.volume * adsr.sustain;
    gain.gain().linear_ramp_to_value_at_time(sustain_level, t_sustain as f64)?;

    // Sustain: hold until release begins
    // Release starts at (duration - release), or immediately if note is very short
    let t_release_start = f32::max(t_sustain, now + duration - adsr.release);
    gain.gain().set_value_at_time(sustain_level, t_release_start as f64)?;

    // Release: ramp to zero
    let t_end = t_release_start + adsr.release;
    gain.gain().linear_ramp_to_value_at_time(0.0, t_end as f64)?;

    // Schedule oscillator lifetime
    osc.start()?;
    osc.stop_with_when((t_end + 0.01) as f64)?; // +10ms buffer to avoid click

    Ok(())
}

/// Convenience: play using instrument defaults
pub fn play_note_with_instrument(
    freq: f32,
    duration: f32,
    instrument: u8,
    wave: WaveShape,
) -> Result<(), JsValue> {
    let adsr = Adsr::from_instrument(instrument);
    play_tone(freq, duration, adsr, wave)
}

/// Resume AudioContext if suspended (required by some browsers after user gesture)
pub fn resume_audio() -> Result<(), JsValue> {
    AUDIO_CTX.with(|cell| {
        if let Some(ctx) = cell.borrow().as_ref() {
            let _ = ctx.resume()?;
        }
        Ok(())
    })
}

/// Get current audio time (used for precise scheduling)
#[allow(dead_code)]
pub fn audio_now() -> f64 {
    AUDIO_CTX.with(|cell| {
        cell.borrow()
            .as_ref()
            .map(|ctx| ctx.current_time())
            .unwrap_or(0.0)
    })
}
