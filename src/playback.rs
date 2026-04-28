use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Recording,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Playback {
    state: PlaybackState,
    current_row: usize,
    current_pattern: usize,
    bpm: u32,
    tick_counter: u64,
    total_ticks: u64,
}

impl Playback {
    pub fn new() -> Self {
        Playback {
            state: PlaybackState::Stopped,
            current_row: 0,
            current_pattern: 0,
            bpm: 125,
            tick_counter: 0,
            total_ticks: 0,
        }
    }

    /// Play from current position
    pub fn play(&mut self) {
        self.state = PlaybackState::Playing;
        self.tick_counter = 0;
    }

    /// Stop and reset to beginning
    pub fn stop(&mut self) {
        self.state = PlaybackState::Stopped;
        self.current_row = 0;
        self.current_pattern = 0;
        self.tick_counter = 0;
        self.total_ticks = 0;
    }

    /// Toggle play/stop
    pub fn toggle_play(&mut self) {
        match self.state {
            PlaybackState::Stopped => self.play(),
            PlaybackState::Playing | PlaybackState::Recording => self.stop(),
        }
    }

    /// Enable recording mode
    pub fn record(&mut self) {
        match self.state {
            PlaybackState::Playing => {
                self.state = PlaybackState::Recording;
            }
            PlaybackState::Recording => {
                self.state = PlaybackState::Playing;
            }
            PlaybackState::Stopped => {
                self.play();
                self.state = PlaybackState::Recording;
            }
        }
    }

    /// Set tempo
    pub fn set_bpm(&mut self, bpm: u32) {
        self.bpm = bpm.max(40).min(300); // Clamp to valid range
    }

    /// Get current BPM
    pub fn bpm(&self) -> u32 {
        self.bpm
    }

    /// Get current row (0-63)
    pub fn current_row(&self) -> usize {
        self.current_row
    }

    /// Get current pattern
    pub fn current_pattern(&self) -> usize {
        self.current_pattern
    }

    /// Check if currently playing
    pub fn is_playing(&self) -> bool {
        self.state == PlaybackState::Playing || self.state == PlaybackState::Recording
    }

    /// Check if recording
    pub fn is_recording(&self) -> bool {
        self.state == PlaybackState::Recording
    }

    /// Get playback state
    pub fn state(&self) -> PlaybackState {
        self.state
    }

    /// Advance playback by one tick (16th note)
    /// Returns true if a new row started (for scheduling notes)
    pub fn tick(&mut self) -> bool {
        if !self.is_playing() {
            return false;
        }

        // Each row = 4 ticks (at 16th note resolution)
        // BPM = quarter notes per minute
        // 1 quarter note = 60,000 ms / BPM
        // 1 16th note = 60,000 ms / BPM / 4

        self.tick_counter += 1;
        self.total_ticks += 1;

        // Check if we've moved to a new row
        if self.tick_counter >= 4 {
            self.tick_counter = 0;
            self.current_row = (self.current_row + 1) % 64;

            if self.current_row == 0 {
                // Pattern complete, move to next
                self.current_pattern += 1;
            }

            return true; // Row changed, schedule notes
        }

        false
    }

    /// Get tick duration in milliseconds at current BPM
    pub fn tick_duration_ms(&self) -> f32 {
        (60000.0 / self.bpm as f32) / 4.0
    }

    /// Get row duration in milliseconds
    pub fn row_duration_ms(&self) -> f32 {
        60000.0 / self.bpm as f32
    }

    /// Reset to beginning of pattern
    pub fn reset_pattern(&mut self) {
        self.current_row = 0;
        self.tick_counter = 0;
    }

    /// Jump to specific row
    pub fn goto_row(&mut self, row: usize) {
        self.current_row = row.min(63);
        self.tick_counter = 0;
    }

    /// Jump to specific pattern
    pub fn goto_pattern(&mut self, pattern: usize) {
        self.current_pattern = pattern;
        self.current_row = 0;
        self.tick_counter = 0;
    }

    /// Get progress as percentage (0-100)
    pub fn progress_percent(&self) -> f32 {
        ((self.current_row as f32) / 64.0) * 100.0
    }
}

impl Default for Playback {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playback_new() {
        let pb = Playback::new();
        assert_eq!(pb.state, PlaybackState::Stopped);
        assert_eq!(pb.current_row, 0);
        assert_eq!(pb.bpm, 125);
    }

    #[test]
    fn test_play_stop() {
        let mut pb = Playback::new();
        pb.play();
        assert!(pb.is_playing());

        pb.stop();
        assert!(!pb.is_playing());
        assert_eq!(pb.current_row, 0);
    }

    #[test]
    fn test_tick_advance() {
        let mut pb = Playback::new();
        pb.play();

        // 3 ticks shouldn't advance row
        for _ in 0..3 {
            assert!(!pb.tick());
        }
        assert_eq!(pb.current_row, 0);

        // 4th tick advances row
        assert!(pb.tick());
        assert_eq!(pb.current_row, 1);
    }

    #[test]
    fn test_bpm_range() {
        let mut pb = Playback::new();

        pb.set_bpm(500); // Too high
        assert_eq!(pb.bpm, 300); // Clamped to max

        pb.set_bpm(20); // Too low
        assert_eq!(pb.bpm, 40); // Clamped to min
    }
}
