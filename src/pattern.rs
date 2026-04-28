use serde::{Deserialize, Serialize};

pub const ROWS: usize = 64;
pub const CHANNELS: usize = 32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Note {
    pub note_name: String,     // e.g., "C", "C#", "D", etc.
    pub octave: u8,            // 1-7
    pub frequency: f32,        // Hz
    pub instrument: u8,        // 0-8 (Solfège)
    pub volume: u8,            // 0-100
}

impl Note {
    pub fn new(note_name: String, octave: u8, frequency: f32) -> Self {
        Note {
            note_name,
            octave,
            frequency,
            instrument: 0,
            volume: 80,
        }
    }

    pub fn with_instrument(mut self, inst: u8) -> Self {
        self.instrument = inst;
        self
    }

    pub fn with_volume(mut self, vol: u8) -> Self {
        self.volume = vol.min(100);
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pattern {
    pub name: String,
    pub rows: usize,
    pub channels: usize,
    grid: Vec<Vec<Option<Note>>>,
}

impl Pattern {
    pub fn new() -> Self {
        let mut grid = Vec::with_capacity(ROWS);
        for _ in 0..ROWS {
            grid.push(vec![None; CHANNELS]);
        }

        Pattern {
            name: "Pattern 00".to_string(),
            rows: ROWS,
            channels: CHANNELS,
            grid,
        }
    }

    pub fn set_cell(&mut self, row: usize, channel: usize, note: Option<Note>) {
        if row < self.rows && channel < self.channels {
            self.grid[row][channel] = note;
        }
    }

    pub fn get_cell(&self, row: usize, channel: usize) -> Option<Note> {
        if row < self.rows && channel < self.channels {
            self.grid[row][channel].clone()
        } else {
            None
        }
    }

    pub fn clear_row(&mut self, row: usize) {
        if row < self.rows {
            for ch in 0..self.channels {
                self.grid[row][ch] = None;
            }
        }
    }

    pub fn clear_channel(&mut self, channel: usize) {
        if channel < self.channels {
            for row in 0..self.rows {
                self.grid[row][channel] = None;
            }
        }
    }

    pub fn clear_all(&mut self) {
        for row in 0..self.rows {
            self.clear_row(row);
        }
    }

    pub fn get_row(&self, row: usize) -> Vec<Option<Note>> {
        if row < self.rows {
            self.grid[row].clone()
        } else {
            vec![None; self.channels]
        }
    }

    /// Count non-empty cells in pattern
    pub fn population(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.is_some())
            .count()
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Self::new()
    }
}
