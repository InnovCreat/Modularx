use crate::pattern::Pattern;
use serde_json;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SaveFile {
    pub version: String,
    pub name: String,
    pub created_at: String,
    pub pattern: Pattern,
}

/// Serialize pattern to JSON string
pub fn save_to_json(pattern: &Pattern) -> Result<String, String> {
    let save_file = SaveFile {
        version: "1.0".to_string(),
        name: pattern.name.clone(),
        created_at: "2026-04-28".to_string(), // Simplified timestamp
        pattern: pattern.clone(),
    };

    serde_json::to_string_pretty(&save_file)
        .map_err(|e| format!("Serialization error: {}", e))
}

/// Deserialize pattern from JSON string
pub fn load_from_json(json: &str) -> Result<Pattern, String> {
    let save_file: SaveFile = serde_json::from_str(json)
        .map_err(|e| format!("Deserialization error: {}", e))?;

    Ok(save_file.pattern)
}

/// Create a compact representation for transmission over network
#[allow(dead_code)]
pub fn to_compact(pattern: &Pattern) -> Result<String, String> {
    #[derive(serde::Serialize)]
    struct Compact {
        name: String,
        data: Vec<(usize, usize, String)>, // (row, channel, note_string)
    }

    let mut data = Vec::new();
    for row in 0..pattern.rows {
        for ch in 0..pattern.channels {
            if let Some(note) = pattern.get_cell(row, ch) {
                let note_str = format!(
                    "{}{} @{}",
                    note.note_name, note.octave, note.instrument
                );
                data.push((row, ch, note_str));
            }
        }
    }

    let compact = Compact {
        name: pattern.name.clone(),
        data,
    };

    serde_json::to_string(&compact)
        .map_err(|e| format!("Compact encoding error: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_load_roundtrip() {
        let original = Pattern::new();
        let json = save_to_json(&original).unwrap();
        let loaded = load_from_json(&json).unwrap();

        assert_eq!(original.name, loaded.name);
        assert_eq!(original.rows, loaded.rows);
        assert_eq!(original.channels, loaded.channels);
    }
}
