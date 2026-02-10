// ════════════════════════════════════════════════════════════════
// NEXUS TUI - Terminal User Interface Souverain
// Zero dependencies (std only) - Pure ANSI escape codes
// Architecture : terminal.rs + canvas.rs + widgets.rs + main.rs
// Philosophie : Veritas Hortus - Souveraineté totale
// ════════════════════════════════════════════════════════════════

mod terminal;
mod canvas;
mod widgets;

use std::io;
use std::time::{Duration, Instant};
use std::thread;

use terminal::Terminal;
use canvas::Canvas;
use widgets::{Panel, TextArea, StatusBar, ProgressBar};

// ── Input Events ─────────────────────────────────────────────

#[derive(Debug, PartialEq)]
enum Event {
    Key(u8),
    Escape,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    None,
}

/// Read a keyboard event (handles escape sequences for arrow keys)
fn read_event() -> io::Result<Event> {
    match Terminal::read_byte()? {
        None => Ok(Event::None),
        Some(0x1B) => {
            // Escape or escape sequence
            match Terminal::read_byte()? {
                Some(b'[') => match Terminal::read_byte()? {
                    Some(b'A') => Ok(Event::ArrowUp),
                    Some(b'B') => Ok(Event::ArrowDown),
                    Some(b'C') => Ok(Event::ArrowRight),
                    Some(b'D') => Ok(Event::ArrowLeft),
                    _ => Ok(Event::Escape),
                },
                None => Ok(Event::Escape),
                _ => Ok(Event::Escape),
            }
        }
        Some(b) => Ok(Event::Key(b)),
    }
}

// ── Application State ────────────────────────────────────────

struct App {
    running: bool,
    active_panel: usize, // 0=source, 1=IR, 2=log
    source_scroll: u16,
    ir_scroll: u16,
    log_scroll: u16,
    compilation_progress: f32,
    compiling: bool,
    compile_stage: usize,
    source_lines: Vec<String>,
    ir_lines: Vec<String>,
    log_lines: Vec<String>,
    tick: u64,
}

impl App {
    fn new() -> Self {
        let source_lines = vec![
            "fn main() {".into(),
            "    let nexus = Compiler::new();".into(),
            "    let source = Source::load(\"main.nx\");".into(),
            "".into(),
            "    // Phase 1: Lexical Analysis".into(),
            "    let tokens = nexus.lex(&source);".into(),
            "    println!(\"Tokens: {}\", tokens.len());".into(),
            "".into(),
            "    // Phase 2: Parsing".into(),
            "    let ast = nexus.parse(&tokens);".into(),
            "".into(),
            "    // Phase 3: Type Checking".into(),
            "    nexus.type_check(&ast);".into(),
            "".into(),
            "    // Phase 4: IR Generation".into(),
            "    let ir = nexus.gen_ir(&ast);".into(),
            "".into(),
            "    // Phase 5: Code Generation".into(),
            "    let binary = nexus.codegen(&ir);".into(),
            "    binary.write(\"output.bin\");".into(),
            "}".into(),
        ];

        let ir_lines = vec![
            "; NEXUS IR - Phase 4 Output".into(),
            "; Target: x86_64-unknown-linux".into(),
            "".into(),
            "define i32 @main() {".into(),
            "entry:".into(),
            "  %nexus = call %Compiler @Compiler::new()".into(),
            "  %src   = call %Source @Source::load(\"main.nx\")".into(),
            "  %toks  = call %Vec<Token> @lex(%nexus, %src)".into(),
            "  call void @println(\"Tokens: {}\", %toks.len)".into(),
            "  %ast   = call %AST @parse(%nexus, %toks)".into(),
            "  call void @type_check(%nexus, %ast)".into(),
            "  %ir    = call %IR @gen_ir(%nexus, %ast)".into(),
            "  %bin   = call %Binary @codegen(%nexus, %ir)".into(),
            "  call void @write(%bin, \"output.bin\")".into(),
            "  ret i32 0".into(),
            "}".into(),
        ];

        Self {
            running: true,
            active_panel: 0,
            source_scroll: 0,
            ir_scroll: 0,
            log_scroll: 0,
            compilation_progress: 0.0,
            compiling: false,
            compile_stage: 0,
            source_lines,
            ir_lines,
            log_lines: Vec::new(),
            tick: 0,
        }
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(b'q') | Event::Key(b'Q') => {
                self.running = false;
            }
            Event::Key(b'c') | Event::Key(b'C') => {
                if !self.compiling {
                    self.compiling = true;
                    self.compilation_progress = 0.0;
                    self.compile_stage = 0;
                    self.log_lines.clear();
                    self.log_lines.push(format!("▶ Compilation started..."));
                }
            }
            Event::Key(b'r') | Event::Key(b'R') => {
                self.log_lines.push("▶ Run: executing output.bin ...".into());
                self.log_lines.push("  Hello from NEXUS!".into());
                self.log_lines.push("  Process exited with code 0".into());
            }
            Event::Key(b'\t') => {
                // Tab cycles panels
                self.active_panel = (self.active_panel + 1) % 3;
            }
            Event::ArrowUp => {
                let scroll = self.active_scroll_mut();
                *scroll = scroll.saturating_sub(1);
            }
            Event::ArrowDown => {
                let scroll = self.active_scroll_mut();
                *scroll = scroll.saturating_add(1);
            }
            _ => {}
        }
    }

    fn active_scroll_mut(&mut self) -> &mut u16 {
        match self.active_panel {
            0 => &mut self.source_scroll,
            1 => &mut self.ir_scroll,
            _ => &mut self.log_scroll,
        }
    }

    fn update(&mut self) {
        self.tick += 1;

        if self.compiling {
            self.compilation_progress += 0.02;

            let stages = [
                (0.15, "✓ Lexer: 47 tokens identified"),
                (0.30, "✓ Parser: AST constructed (21 nodes)"),
                (0.50, "✓ Type Checker: all types resolved"),
                (0.70, "✓ IR Generation: 16 instructions emitted"),
                (0.90, "✓ Code Generation: binary assembled"),
                (1.00, "✓ Compilation complete — output.bin (4.2 KB)"),
            ];

            if self.compile_stage < stages.len() {
                let (threshold, msg) = stages[self.compile_stage];
                if self.compilation_progress >= threshold {
                    self.log_lines.push(msg.into());
                    self.compile_stage += 1;
                }
            }

            if self.compilation_progress >= 1.0 {
                self.compilation_progress = 1.0;
                self.compiling = false;
            }
        }
    }

    fn draw(&self, canvas: &mut Canvas) {
        let w = canvas.width();
        let h = canvas.height();

        canvas.clear();

        // ── Header ───────────────────────────────────────
        let header_bg = 17; // Dark blue
        let title = format!(
            " NEXUS COMPILER v4.2.1 — Sovereign Build │ tick {}",
            self.tick
        );
        StatusBar::draw(canvas, 0, w, &title, 51, header_bg);

        // ── Layout calculations ──────────────────────────
        let src_width = w / 2;
        let ir_width = w - src_width;
        let top_height = if h > 12 { h / 2 - 1 } else { h.saturating_sub(4) };
        let msg_y = top_height + 1;
        let msg_height = h.saturating_sub(msg_y + 2);

        // ── Source panel (top-left) ──────────────────────
        let src_fg = if self.active_panel == 0 { 51 } else { 246 };
        Panel::draw(canvas, 0, 1, src_width, top_height, "SOURCE", src_fg, 234);
        TextArea::draw(
            canvas,
            1,
            2,
            src_width.saturating_sub(2),
            top_height.saturating_sub(2),
            &self.source_lines,
            self.source_scroll,
            15,
            234,
        );

        // ── IR panel (top-right) ─────────────────────────
        let ir_fg = if self.active_panel == 1 { 226 } else { 246 };
        Panel::draw(canvas, src_width, 1, ir_width, top_height, "IR OUTPUT", ir_fg, 234);
        TextArea::draw(
            canvas,
            src_width + 1,
            2,
            ir_width.saturating_sub(2),
            top_height.saturating_sub(2),
            &self.ir_lines,
            self.ir_scroll,
            15,
            234,
        );

        // ── Log panel (bottom) ──────────────────────────
        if msg_height >= 3 {
            let log_fg = if self.active_panel == 2 { 82 } else { 246 };
            Panel::draw(canvas, 0, msg_y, w, msg_height, "COMPILATION LOG", log_fg, 234);

            // Progress bar inside log panel (when compiling or just finished)
            if self.compiling || self.compilation_progress >= 1.0 {
                let bar_width = w.saturating_sub(4);
                if bar_width > 10 {
                    ProgressBar::draw(
                        canvas,
                        2,
                        msg_y + 1,
                        bar_width,
                        self.compilation_progress,
                        if self.compilation_progress >= 1.0 { 82 } else { 226 },
                        234,
                    );
                }
            }

            let text_y = if self.compiling || self.compilation_progress >= 1.0 {
                msg_y + 2
            } else {
                msg_y + 1
            };
            let text_h = msg_height.saturating_sub(if self.compiling || self.compilation_progress >= 1.0 { 3 } else { 2 });

            // Auto-scroll log to bottom
            let log_scroll = if self.log_lines.len() as u16 > text_h {
                self.log_lines.len() as u16 - text_h
            } else {
                0
            };

            TextArea::draw(
                canvas,
                1,
                text_y,
                w.saturating_sub(2),
                text_h,
                &self.log_lines,
                log_scroll,
                82,
                234,
            );
        }

        // ── Footer status bar ────────────────────────────
        let panel_name = match self.active_panel {
            0 => "SOURCE",
            1 => "IR",
            _ => "LOG",
        };
        let footer = format!(
            " [q] Quit  [c] Compile  [r] Run  [Tab] Switch Panel  │  Active: {}  │  Veritas Hortus",
            panel_name
        );
        StatusBar::draw(canvas, h - 1, w, &footer, 15, 22);
    }
}

// ── Main Entry Point ─────────────────────────────────────────

fn main() -> io::Result<()> {
    // Setup terminal
    Terminal::enable_raw_mode()?;
    Terminal::enter_alternate()?;
    Terminal::clear()?;
    Terminal::hide_cursor()?;

    let (rows, cols) = Terminal::size();
    let mut canvas = Canvas::new(cols, rows);
    let mut app = App::new();

    // Target ~30 FPS
    let frame_duration = Duration::from_millis(33);

    // Main event loop
    while app.running {
        let frame_start = Instant::now();

        // Poll input
        let event = read_event()?;
        app.handle_event(event);

        // Update state
        app.update();

        // Detect terminal resize
        let (new_rows, new_cols) = Terminal::size();
        if new_rows != rows || new_cols != cols {
            canvas.resize(new_cols, new_rows);
            Terminal::clear()?;
        }

        // Draw
        app.draw(&mut canvas);
        canvas.render()?;

        // Frame pacing
        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }
    }

    // Cleanup
    Terminal::show_cursor()?;
    Terminal::exit_alternate()?;
    Terminal::disable_raw_mode()?;
    Terminal::reset()?;

    println!("NEXUS TUI exited cleanly.");
    Ok(())
}
