use crate::pattern::Note;
use serde::{Deserialize, Serialize};

/// Solfège instruments (639 Hz system)
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Instrument {
    pub name: &'static str,
    pub base_hz: f32,
    pub index: u8,
}

pub const SOLFEGE_INSTRUMENTS: &[Instrument] = &[
    Instrument { name: "UT", base_hz: 396.0, index: 0 },
    Instrument { name: "RE", base_hz: 417.0, index: 1 },
    Instrument { name: "MI", base_hz: 528.0, index: 2 },
    Instrument { name: "FA", base_hz: 639.0, index: 3 },
    Instrument { name: "SOL", base_hz: 741.0, index: 4 },
    Instrument { name: "LA", base_hz: 852.0, index: 5 },
    Instrument { name: "SI", base_hz: 963.0, index: 6 },
    Instrument { name: "ISA", base_hz: 432.0, index: 7 },
    Instrument { name: "THI", base_hz: 207.0, index: 8 },
];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoteFrequency {
    pub note_name: String,
    pub octave: u8,
    pub frequency: f32,
    pub midi_number: u8,
}

impl NoteFrequency {
    /// Parse note string like "C4", "C#4", "D-5", etc.
    /// Also accepts "C4 @2" syntax for instrument selection
    pub fn parse(input: &str) -> Result<Note, String> {
        let input = input.trim().to_uppercase();

        if input.is_empty() || input == "---" {
            return Err("Empty note".to_string());
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let note_part = parts.get(0).ok_or("No note specified")?;

        // Parse note name and octave: "C4", "C#4", "D-5", etc.
        let (note_name, octave_str) = if note_part.len() >= 2 {
            let note = &note_part[0..note_part.len() - 1];
            let octave = &note_part[note_part.len() - 1..];
            (note, octave)
        } else {
            return Err("Invalid note format".to_string());
        };

        let octave: u8 = octave_str
            .parse()
            .map_err(|_| "Invalid octave number".to_string())?;

        if octave < 1 || octave > 7 {
            return Err("Octave must be 1-7".to_string());
        }

        let semitone = match note_name {
            "C" => 0,
            "C#" | "DB" => 1,
            "D" => 2,
            "D#" | "EB" => 3,
            "E" => 4,
            "F" => 5,
            "F#" | "GB" => 6,
            "G" => 7,
            "G#" | "AB" => 8,
            "A" => 9,
            "A#" | "BB" => 10,
            "B" => 11,
            _ => return Err(format!("Unknown note: {}", note_name)),
        };

        let midi_number = (octave as u8 - 1) * 12 + semitone;
        let frequency = Self::midi_to_frequency(midi_number);

        // Check for instrument selector (@0 through @8)
        let instrument = if let Some(inst_part) = parts.get(1) {
            let inst_str = inst_part.trim_start_matches('@');
            inst_str
                .parse::<u8>()
                .ok()
                .and_then(|i| if i < 9 { Some(i) } else { None })
                .unwrap_or(0)
        } else {
            0
        };

        Ok(Note {
            note_name: note_name.to_string(),
            octave,
            frequency,
            instrument,
            volume: 80,
        })
    }

    /// Convert MIDI note number (0-127) to frequency in Hz
    pub fn midi_to_frequency(midi: u8) -> f32 {
        let semitones_from_a4 = midi as f32 - 69.0;
        440.0 * 2.0_f32.powf(semitones_from_a4 / 12.0)
    }

    /// Convert frequency back to MIDI number (closest match)
    pub fn frequency_to_midi(freq: f32) -> u8 {
        let semitones_from_a4 = 12.0 * (freq / 440.0).log2();
        (69.0 + semitones_from_a4).round() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_note() {
        let note = NoteFrequency::parse("C4").unwrap();
        assert_eq!(note.note_name, "C");
        assert_eq!(note.octave, 4);
        assert!((note.frequency - 261.63).abs() < 0.1);
    }

    #[test]
    fn test_parse_sharp() {
        let note = NoteFrequency::parse("F#4").unwrap();
        assert_eq!(note.note_name, "F#");
        assert!((note.frequency - 369.99).abs() < 0.1);
    }

    #[test]
    fn test_invalid_note() {
        assert!(NoteFrequency::parse("X9").is_err());
        assert!(NoteFrequency::parse("C8").is_err());
    }
}
