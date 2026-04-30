#![allow(dead_code)]
// ════════════════════════════════════════════════════════════════
// NEXUS TUI - widgets.rs
// UI components: Border, Panel, TextArea, StatusBar, ProgressBar
// ════════════════════════════════════════════════════════════════

use crate::canvas::Canvas;

// ── Border ───────────────────────────────────────────────────

pub struct Border;

impl Border {
    /// Draw a single-line border
    pub fn draw_single(canvas: &mut Canvas, x: u16, y: u16, w: u16, h: u16, fg: u8) {
        if w < 2 || h < 2 {
            return;
        }

        // Corners
        canvas.put_char(x, y, '┌', fg, 0, false);
        canvas.put_char(x + w - 1, y, '┐', fg, 0, false);
        canvas.put_char(x, y + h - 1, '└', fg, 0, false);
        canvas.put_char(x + w - 1, y + h - 1, '┘', fg, 0, false);

        // Horizontal edges
        for i in 1..w - 1 {
            canvas.put_char(x + i, y, '─', fg, 0, false);
            canvas.put_char(x + i, y + h - 1, '─', fg, 0, false);
        }

        // Vertical edges
        for i in 1..h - 1 {
            canvas.put_char(x, y + i, '│', fg, 0, false);
            canvas.put_char(x + w - 1, y + i, '│', fg, 0, false);
        }
    }

    /// Draw a double-line border
    pub fn draw_double(canvas: &mut Canvas, x: u16, y: u16, w: u16, h: u16, fg: u8) {
        if w < 2 || h < 2 {
            return;
        }

        canvas.put_char(x, y, '╔', fg, 0, false);
        canvas.put_char(x + w - 1, y, '╗', fg, 0, false);
        canvas.put_char(x, y + h - 1, '╚', fg, 0, false);
        canvas.put_char(x + w - 1, y + h - 1, '╝', fg, 0, false);

        for i in 1..w - 1 {
            canvas.put_char(x + i, y, '═', fg, 0, false);
            canvas.put_char(x + i, y + h - 1, '═', fg, 0, false);
        }

        for i in 1..h - 1 {
            canvas.put_char(x, y + i, '║', fg, 0, false);
            canvas.put_char(x + w - 1, y + i, '║', fg, 0, false);
        }
    }
}

// ── Panel ────────────────────────────────────────────────────

pub struct Panel;

impl Panel {
    /// Draw a panel with background fill, border, and optional title
    pub fn draw(
        canvas: &mut Canvas,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        title: &str,
        fg: u8,
        bg: u8,
    ) {
        // Fill background
        for row in y..y + height {
            for col in x..x + width {
                canvas.put_char(col, row, ' ', fg, bg, false);
            }
        }

        // Border
        Border::draw_single(canvas, x, y, width, height, fg);

        // Title centered in top border
        if !title.is_empty() {
            let label = format!(" {} ", title);
            let label_len = label.chars().count() as u16;
            let title_x = if label_len < width - 2 {
                x + 2
            } else {
                x + 1
            };
            canvas.put_str(title_x, y, &label, fg, bg, true);
        }
    }
}

// ── TextArea ─────────────────────────────────────────────────

pub struct TextArea;

impl TextArea {
    /// Render multi-line text within a bounded area with scroll offset
    pub fn draw(
        canvas: &mut Canvas,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        lines: &[String],
        scroll: u16,
        fg: u8,
        bg: u8,
    ) {
        let start = scroll as usize;
        let visible = height as usize;
        let end = (start + visible).min(lines.len());

        for (i, line) in lines[start..end].iter().enumerate() {
            let display: String = line.chars().take(width as usize).collect();
            canvas.put_str(x, y + i as u16, &display, fg, bg, false);
        }
    }
}

// ── StatusBar ────────────────────────────────────────────────

pub struct StatusBar;

impl StatusBar {
    /// Draw a full-width status bar at a given row
    pub fn draw(canvas: &mut Canvas, y: u16, width: u16, text: &str, fg: u8, bg: u8) {
        // Fill entire line with background
        for x in 0..width {
            canvas.put_char(x, y, ' ', fg, bg, false);
        }
        // Draw text (bold)
        canvas.put_str(0, y, text, fg, bg, true);
    }
}

// ── ProgressBar ──────────────────────────────────────────────

pub struct ProgressBar;

impl ProgressBar {
    /// Draw a progress bar: [████░░░░░░] 40%
    pub fn draw(
        canvas: &mut Canvas,
        x: u16,
        y: u16,
        width: u16,
        progress: f32, // 0.0 .. 1.0
        fg: u8,
        bg: u8,
    ) {
        if width < 5 {
            return;
        }

        let bar_width = (width - 2) as usize; // subtract brackets
        let filled = ((progress.clamp(0.0, 1.0)) * bar_width as f32) as usize;

        canvas.put_char(x, y, '[', fg, bg, false);
        for i in 0..bar_width {
            let ch = if i < filled { '█' } else { '░' };
            canvas.put_char(x + 1 + i as u16, y, ch, fg, bg, false);
        }
        canvas.put_char(x + width - 1, y, ']', fg, bg, false);
    }
}
