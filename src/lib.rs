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

#[wasm_bindgen]
pub struct VHTracker {
    pattern: Arc<Mutex<Pattern>>,
    playback: Arc<Mutex<Playback>>,
}

#[wasm_bindgen]
impl VHTracker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> VHTracker {
        let pattern = Pattern::new();
        let playback = Playback::new();

        VHTracker {
            pattern: Arc::new(Mutex::new(pattern)),
            playback: Arc::new(Mutex::new(playback)),
        }
    }

    /// Set a note at grid position
    pub fn set_note(&self, row: usize, channel: usize, note_str: &str) -> Result<(), JsValue> {
        if let Ok(mut pattern) = self.pattern.lock() {
            match NoteFrequency::parse(note_str) {
                Ok(note) => {
                    pattern.set_cell(row, channel, Some(note));
                    Ok(())
                }
                Err(e) => Err(JsValue::from_str(&format!("Invalid note: {}", e))),
            }
        } else {
            Err(JsValue::from_str("Pattern lock failed"))
        }
    }

    /// Clear a cell
    pub fn clear_cell(&self, row: usize, channel: usize) -> Result<(), JsValue> {
        if let Ok(mut pattern) = self.pattern.lock() {
            pattern.set_cell(row, channel, None);
            Ok(())
        } else {
            Err(JsValue::from_str("Pattern lock failed"))
        }
    }

    /// Get cell contents as JSON
    pub fn get_cell(&self, row: usize, channel: usize) -> String {
        if let Ok(pattern) = self.pattern.lock() {
            if let Some(note) = pattern.get_cell(row, channel) {
                serde_json::to_string(&note).unwrap_or_default()
            } else {
                "null".to_string()
            }
        } else {
            "null".to_string()
        }
    }

    /// Get entire pattern as JSON
    pub fn pattern_to_json(&self) -> String {
        if let Ok(pattern) = self.pattern.lock() {
            serde_json::to_string(&*pattern).unwrap_or_default()
        } else {
            "{}".to_string()
        }
    }

    /// Load pattern from JSON
    pub fn pattern_from_json(&self, json: &str) -> Result<(), JsValue> {
        match serde_json::from_str::<Pattern>(json) {
            Ok(loaded_pattern) => {
                if let Ok(mut pattern) = self.pattern.lock() {
                    *pattern = loaded_pattern;
                    Ok(())
                } else {
                    Err(JsValue::from_str("Pattern lock failed"))
                }
            }
            Err(e) => Err(JsValue::from_str(&format!("JSON parse error: {}", e))),
        }
    }

    /// Start playback
    pub fn play(&self) -> Result<(), JsValue> {
        if let Ok(mut pb) = self.playback.lock() {
            pb.play();
            Ok(())
        } else {
            Err(JsValue::from_str("Playback lock failed"))
        }
    }

    /// Stop playback
    pub fn stop(&self) -> Result<(), JsValue> {
        if let Ok(mut pb) = self.playback.lock() {
            pb.stop();
            Ok(())
        } else {
            Err(JsValue::from_str("Playback lock failed"))
        }
    }

    /// Toggle playback
    pub fn toggle_play(&self) -> Result<bool, JsValue> {
        if let Ok(mut pb) = self.playback.lock() {
            pb.toggle_play();
            Ok(pb.is_playing())
        } else {
            Err(JsValue::from_str("Playback lock failed"))
        }
    }

    /// Set BPM
    pub fn set_bpm(&self, bpm: u32) -> Result<(), JsValue> {
        if let Ok(mut pb) = self.playback.lock() {
            pb.set_bpm(bpm);
            Ok(())
        } else {
            Err(JsValue::from_str("Playback lock failed"))
        }
    }

    /// Get current row
    pub fn current_row(&self) -> usize {
        if let Ok(pb) = self.playback.lock() {
            pb.current_row()
        } else {
            0
        }
    }

    /// Get current state as JSON
    pub fn state_to_json(&self) -> String {
        if let Ok(pb) = self.playback.lock() {
            serde_json::json!({
                "row": pb.current_row(),
                "playing": pb.is_playing(),
                "bpm": pb.bpm(),
            }).to_string()
        } else {
            "{}".to_string()
        }
    }

    /// Advance one tick manually (for testing)
    pub fn tick(&self) {
        if let Ok(mut pb) = self.playback.lock() {
            pb.tick();
        }
    }

    /// Get notes that should play at current row
    pub fn get_row_events(&self) -> String {
        if let Ok(pb) = self.playback.lock() {
            if let Ok(pattern) = self.pattern.lock() {
                let row = pb.current_row();
                let mut events = Vec::new();

                for ch in 0..pattern.channels {
                    if let Some(note) = pattern.get_cell(row, ch) {
                        events.push(serde_json::json!({
                            "channel": ch,
                            "note": note.note_name,
                            "frequency": note.frequency,
                            "octave": note.octave,
                        }));
                    }
                }

                serde_json::to_string(&events).unwrap_or_default()
            } else {
                "[]".to_string()
            }
        } else {
            "[]".to_string()
        }
    }
}

#[wasm_bindgen(start)]
pub fn init() {
    // Initialization hook for WASM
}
