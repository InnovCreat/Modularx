mod audio;
mod pattern;
mod synth;
mod playback;
mod persist;

use wasm_bindgen::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;

pub use pattern::{Pattern, Note};
pub use synth::{NoteFrequency, SOLFEGE_INSTRUMENTS};
pub use playback::{Playback, PlaybackState};
pub use persist::{save_to_json, load_from_json};
pub use audio::{Adsr, WaveShape};

#[wasm_bindgen]
pub struct VHTracker {
    pattern:  Arc<Mutex<Pattern>>,
    playback: Arc<Mutex<Playback>>,
}

#[wasm_bindgen]
impl VHTracker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> VHTracker {
        VHTracker {
            pattern:  Arc::new(Mutex::new(Pattern::new())),
            playback: Arc::new(Mutex::new(Playback::new())),
        }
    }

    // ── Grid ────────────────────────────────────────────────────────────────

    pub fn set_note(&self, row: usize, channel: usize, note_str: &str) -> Result<(), JsValue> {
        match NoteFrequency::parse(note_str) {
            Ok(note) => {
                self.pattern.lock().unwrap().set_cell(row, channel, Some(note));
                Ok(())
            }
            Err(e) => Err(JsValue::from_str(&format!("Invalid note: {}", e))),
        }
    }

    pub fn clear_cell(&self, row: usize, channel: usize) {
        self.pattern.lock().unwrap().set_cell(row, channel, None);
    }

    pub fn get_cell(&self, row: usize, channel: usize) -> String {
        match self.pattern.lock().unwrap().get_cell(row, channel) {
            Some(note) => serde_json::to_string(&note).unwrap_or_default(),
            None => "null".to_string(),
        }
    }

    pub fn pattern_to_json(&self) -> String {
        serde_json::to_string(&*self.pattern.lock().unwrap()).unwrap_or_default()
    }

    pub fn pattern_from_json(&self, json: &str) -> Result<(), JsValue> {
        match serde_json::from_str::<Pattern>(json) {
            Ok(loaded) => { *self.pattern.lock().unwrap() = loaded; Ok(()) }
            Err(e) => Err(JsValue::from_str(&format!("JSON parse error: {}", e))),
        }
    }

    // ── Transport ────────────────────────────────────────────────────────────

    pub fn play(&self) {
        self.playback.lock().unwrap().play();
    }

    pub fn stop(&self) {
        self.playback.lock().unwrap().stop();
    }

    pub fn toggle_play(&self) -> bool {
        let mut pb = self.playback.lock().unwrap();
        pb.toggle_play();
        pb.is_playing()
    }

    pub fn set_bpm(&self, bpm: u32) {
        self.playback.lock().unwrap().set_bpm(bpm);
    }

    pub fn current_row(&self) -> usize {
        self.playback.lock().unwrap().current_row()
    }

    pub fn state_to_json(&self) -> String {
        let pb = self.playback.lock().unwrap();
        serde_json::json!({
            "row":     pb.current_row(),
            "playing": pb.is_playing(),
            "bpm":     pb.bpm(),
        }).to_string()
    }

    /// Advance one tick and return whether the row changed.
    /// Call this from a JavaScript `setInterval`/`requestAnimationFrame` loop.
    pub fn tick(&self) -> bool {
        self.playback.lock().unwrap().tick()
    }

    // ── Audio ────────────────────────────────────────────────────────────────

    /// Play all notes in the current row using the ADSR engine.
    /// Call this after every `tick()` that returns true.
    pub fn play_current_row(&self) -> Result<(), JsValue> {
        let pb  = self.playback.lock().unwrap();
        let pat = self.pattern.lock().unwrap();

        if !pb.is_playing() { return Ok(()); }

        let row      = pb.current_row();
        let duration = pb.row_duration_ms() / 1000.0; // ms → seconds

        for ch in 0..pat.channels {
            if let Some(note) = pat.get_cell(row, ch) {
                let adsr = Adsr::from_instrument(note.instrument);
                audio::play_tone(note.frequency, duration, adsr, WaveShape::Sine)?;
            }
        }
        Ok(())
    }

    /// Play a single tone immediately — used for piano keyboard preview.
    ///
    /// `freq`       — Hz
    /// `duration`   — seconds (use 0.5 for key preview)
    /// `instrument` — 0–8, selects per-instrument ADSR defaults
    pub fn preview_note(&self, freq: f32, duration: f32, instrument: u8) -> Result<(), JsValue> {
        audio::play_note_with_instrument(freq, duration, instrument, WaveShape::Sine)
    }

    /// Must be called from a user-gesture handler (click/keydown) to satisfy
    /// browser autoplay policy before any audio can play.
    pub fn resume_audio(&self) -> Result<(), JsValue> {
        audio::resume_audio()
    }

    /// Get notes that will fire at the current row (JSON array for the UI).
    pub fn get_row_events(&self) -> String {
        let pb  = self.playback.lock().unwrap();
        let pat = self.pattern.lock().unwrap();
        let row = pb.current_row();

        let events: Vec<_> = (0..pat.channels)
            .filter_map(|ch| pat.get_cell(row, ch).map(|n| serde_json::json!({
                "channel":   ch,
                "note":      n.note_name,
                "frequency": n.frequency,
                "octave":    n.octave,
                "instrument": n.instrument,
            })))
            .collect();

        serde_json::to_string(&events).unwrap_or_default()
    }
}

#[wasm_bindgen(start)]
pub fn init() {}
