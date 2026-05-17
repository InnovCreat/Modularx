// ════════════════════════════════════════════════════════════════
// NEXUS TUI - editor.rs
// Modal text editor: Normal (navigation) / Insert (editing)
// Zero external dependencies — pure buffer + cursor logic
// ════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EditorMode {
    Normal,
    Insert,
}

struct Snapshot {
    lines: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
}

pub struct Editor {
    pub lines: Vec<String>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub scroll: u16,
    pub mode: EditorMode,
    pub dirty: bool,
    undo_stack: Vec<Snapshot>,
    viewport_height: u16,
}

impl Editor {
    pub fn new(content: &str) -> Self {
        let lines: Vec<String> = if content.is_empty() {
            vec![String::new()]
        } else {
            content.lines().map(String::from).collect()
        };
        Self {
            lines,
            cursor_row: 0,
            cursor_col: 0,
            scroll: 0,
            mode: EditorMode::Normal,
            dirty: false,
            undo_stack: Vec::new(),
            viewport_height: 20,
        }
    }

    pub fn load(&mut self, content: &str) {
        self.lines = if content.is_empty() {
            vec![String::new()]
        } else {
            content.lines().map(String::from).collect()
        };
        self.cursor_row = 0;
        self.cursor_col = 0;
        self.scroll = 0;
        self.mode = EditorMode::Normal;
        self.dirty = false;
        self.undo_stack.clear();
    }

    pub fn content(&self) -> String {
        self.lines.join("\n")
    }

    pub fn set_viewport_height(&mut self, h: u16) {
        self.viewport_height = h;
    }

    // ── Undo ────────────────────────────────────────────────────

    fn push_undo(&mut self) {
        if self.undo_stack.len() >= 50 {
            self.undo_stack.remove(0);
        }
        self.undo_stack.push(Snapshot {
            lines: self.lines.clone(),
            cursor_row: self.cursor_row,
            cursor_col: self.cursor_col,
        });
    }

    pub fn undo(&mut self) {
        if let Some(snap) = self.undo_stack.pop() {
            self.lines = snap.lines;
            self.cursor_row = snap.cursor_row;
            self.cursor_col = snap.cursor_col;
            self.ensure_cursor_visible();
            self.dirty = true;
        }
    }

    // ── Cursor Movement ─────────────────────────────────────────

    pub fn move_up(&mut self) {
        if self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.clamp_col();
            self.ensure_cursor_visible();
        }
    }

    pub fn move_down(&mut self) {
        if self.cursor_row + 1 < self.lines.len() {
            self.cursor_row += 1;
            self.clamp_col();
            self.ensure_cursor_visible();
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.cursor_col = self.current_line_len();
            self.ensure_cursor_visible();
        }
    }

    pub fn move_right(&mut self) {
        let len = self.current_line_len();
        if self.cursor_col < len {
            self.cursor_col += 1;
        } else if self.cursor_row + 1 < self.lines.len() {
            self.cursor_row += 1;
            self.cursor_col = 0;
            self.ensure_cursor_visible();
        }
    }

    pub fn move_home(&mut self) {
        self.cursor_col = 0;
    }

    pub fn move_end(&mut self) {
        self.cursor_col = self.current_line_len();
    }

    // ── Editing Operations ──────────────────────────────────────

    pub fn insert_char(&mut self, ch: char) {
        self.push_undo();
        let line = &mut self.lines[self.cursor_row];
        if self.cursor_col >= line.len() {
            line.push(ch);
        } else {
            line.insert(self.cursor_col, ch);
        }
        self.cursor_col += 1;
        self.dirty = true;
    }

    pub fn insert_tab(&mut self) {
        self.push_undo();
        let line = &mut self.lines[self.cursor_row];
        let spaces = "    ";
        if self.cursor_col >= line.len() {
            line.push_str(spaces);
        } else {
            line.insert_str(self.cursor_col, spaces);
        }
        self.cursor_col += 4;
        self.dirty = true;
    }

    pub fn insert_newline(&mut self) {
        self.push_undo();
        let line = self.lines[self.cursor_row].clone();
        let (before, after) = line.split_at(self.cursor_col.min(line.len()));
        self.lines[self.cursor_row] = before.to_string();
        self.lines.insert(self.cursor_row + 1, after.to_string());
        self.cursor_row += 1;
        self.cursor_col = 0;
        self.dirty = true;
        self.ensure_cursor_visible();
    }

    pub fn backspace(&mut self) {
        if self.cursor_col > 0 {
            self.push_undo();
            let line = &mut self.lines[self.cursor_row];
            let remove_at = self.cursor_col - 1;
            if remove_at < line.len() {
                line.remove(remove_at);
            }
            self.cursor_col -= 1;
            self.dirty = true;
        } else if self.cursor_row > 0 {
            self.push_undo();
            let current_line = self.lines.remove(self.cursor_row);
            self.cursor_row -= 1;
            self.cursor_col = self.lines[self.cursor_row].len();
            self.lines[self.cursor_row].push_str(&current_line);
            self.dirty = true;
            self.ensure_cursor_visible();
        }
    }

    pub fn delete_char(&mut self) {
        let len = self.current_line_len();
        if self.cursor_col < len {
            self.push_undo();
            self.lines[self.cursor_row].remove(self.cursor_col);
            self.dirty = true;
        } else if self.cursor_row + 1 < self.lines.len() {
            self.push_undo();
            let next_line = self.lines.remove(self.cursor_row + 1);
            self.lines[self.cursor_row].push_str(&next_line);
            self.dirty = true;
        }
    }

    // ── Viewport ────────────────────────────────────────────────

    pub fn ensure_cursor_visible(&mut self) {
        if self.viewport_height == 0 {
            return;
        }
        let vh = self.viewport_height as usize;
        if self.cursor_row < self.scroll as usize {
            self.scroll = self.cursor_row as u16;
        } else if self.cursor_row >= self.scroll as usize + vh {
            self.scroll = (self.cursor_row - vh + 1) as u16;
        }
    }

    // ── Helpers ─────────────────────────────────────────────────

    fn current_line_len(&self) -> usize {
        self.lines.get(self.cursor_row).map_or(0, |l| l.len())
    }

    fn clamp_col(&mut self) {
        let len = self.current_line_len();
        if self.cursor_col > len {
            self.cursor_col = len;
        }
    }
}
