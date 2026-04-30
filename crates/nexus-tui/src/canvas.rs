#![allow(dead_code)]
// ════════════════════════════════════════════════════════════════
// NEXUS TUI - canvas.rs
// Double-buffered rendering with dirty-cell tracking
// Optimized ANSI output - only redraws changed cells
// ════════════════════════════════════════════════════════════════

use std::io::{self, Write};

#[derive(Clone, Copy, PartialEq)]
pub struct Cell {
    pub ch: char,
    pub fg: u8,
    pub bg: u8,
    pub bold: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: 15,  // White
            bg: 0,   // Black
            bold: false,
        }
    }
}

pub struct Canvas {
    width: u16,
    height: u16,
    current: Vec<Cell>,
    previous: Vec<Cell>,
    force_redraw: bool,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        // Initialize previous with a sentinel that differs from default
        // so first render draws everything
        let sentinel = Cell { ch: '\0', fg: 0, bg: 0, bold: false };
        Self {
            width,
            height,
            current: vec![Cell::default(); size],
            previous: vec![sentinel; size],
            force_redraw: true,
        }
    }

    /// Write a character at position
    pub fn put_char(&mut self, x: u16, y: u16, ch: char, fg: u8, bg: u8, bold: bool) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = (y as usize) * (self.width as usize) + (x as usize);
        self.current[idx] = Cell { ch, fg, bg, bold };
    }

    /// Write a string at position
    pub fn put_str(&mut self, x: u16, y: u16, s: &str, fg: u8, bg: u8, bold: bool) {
        for (i, ch) in s.chars().enumerate() {
            let px = x as usize + i;
            if px >= self.width as usize {
                break;
            }
            self.put_char(px as u16, y, ch, fg, bg, bold);
        }
    }

    /// Clear the canvas (fill with default cells)
    pub fn clear(&mut self) {
        self.current.fill(Cell::default());
    }

    /// Render only changed cells to the terminal (differential update)
    pub fn render(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();
        let mut last_fg: u8 = 255;
        let mut last_bg: u8 = 255;
        let mut last_bold: bool = false;
        let mut cursor_row: u16 = u16::MAX;
        let mut cursor_col: u16 = u16::MAX;

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y as usize) * (self.width as usize) + (x as usize);
                let cell = self.current[idx];

                if !self.force_redraw && cell == self.previous[idx] {
                    continue;
                }

                // Move cursor if not contiguous
                if cursor_row != y || cursor_col != x {
                    write!(stdout, "\x1B[{};{}H", y + 1, x + 1)?;
                }

                // Update styles only when changed
                if cell.bold != last_bold {
                    if cell.bold {
                        write!(stdout, "\x1B[1m")?;
                    } else {
                        write!(stdout, "\x1B[22m")?;
                    }
                    last_bold = cell.bold;
                }
                if cell.fg != last_fg {
                    write!(stdout, "\x1B[38;5;{}m", cell.fg)?;
                    last_fg = cell.fg;
                }
                if cell.bg != last_bg {
                    write!(stdout, "\x1B[48;5;{}m", cell.bg)?;
                    last_bg = cell.bg;
                }

                write!(stdout, "{}", cell.ch)?;

                cursor_row = y;
                cursor_col = x + 1;
            }
        }

        write!(stdout, "\x1B[0m")?;
        stdout.flush()?;

        // Swap buffers
        self.previous.copy_from_slice(&self.current);
        self.force_redraw = false;

        Ok(())
    }

    /// Force a full redraw on next render
    pub fn invalidate(&mut self) {
        self.force_redraw = true;
    }

    /// Resize canvas (discards content)
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        let size = (width as usize) * (height as usize);
        self.current = vec![Cell::default(); size];
        let sentinel = Cell { ch: '\0', fg: 0, bg: 0, bold: false };
        self.previous = vec![sentinel; size];
        self.force_redraw = true;
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }
}
