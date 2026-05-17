// ════════════════════════════════════════════════════════════════
// NEXUS TUI - Terminal User Interface Souverain
// Real compiler pipeline: Lexer → Parser → TypeCheck → Eval
// Modal editor: [i] Insert / [Esc] Normal — live compile on exit
// Zero external dependencies — pure ANSI escape codes
// ════════════════════════════════════════════════════════════════

mod terminal;
mod canvas;
mod widgets;
mod editor;

use std::io;
use std::time::{Duration, Instant};
use std::thread;

use terminal::Terminal;
use canvas::Canvas;
use widgets::{Panel, TextArea, EditableTextArea, StatusBar, ProgressBar};
use editor::{Editor, EditorMode};

use nexus_lexer::Lexer;
use nexus_parser::Parser;
use nexus_parser::ast::pretty_print;
use nexus_typeck::type_check;
use nexus_eval::eval;

// ── Input Events ─────────────────────────────────────────────

#[derive(Debug, PartialEq)]
enum Event {
    Key(u8),
    Escape,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    Delete,
    None,
}

fn read_event() -> io::Result<Event> {
    match Terminal::read_byte()? {
        None => Ok(Event::None),
        Some(0x1B) => {
            match Terminal::read_byte()? {
                Some(b'[') => match Terminal::read_byte()? {
                    Some(b'A') => Ok(Event::ArrowUp),
                    Some(b'B') => Ok(Event::ArrowDown),
                    Some(b'C') => Ok(Event::ArrowRight),
                    Some(b'D') => Ok(Event::ArrowLeft),
                    Some(b'H') => Ok(Event::Home),
                    Some(b'F') => Ok(Event::End),
                    Some(b'3') => {
                        match Terminal::read_byte()? {
                            Some(b'~') => Ok(Event::Delete),
                            _ => Ok(Event::Escape),
                        }
                    }
                    Some(b'1') => {
                        match Terminal::read_byte()? {
                            Some(b'~') => Ok(Event::Home),
                            _ => Ok(Event::Escape),
                        }
                    }
                    Some(b'4') => {
                        match Terminal::read_byte()? {
                            Some(b'~') => Ok(Event::End),
                            _ => Ok(Event::Escape),
                        }
                    }
                    _ => Ok(Event::Escape),
                },
                _ => Ok(Event::Escape),
            }
        }
        Some(b) => Ok(Event::Key(b)),
    }
}

// ── Application State ────────────────────────────────────────

struct App {
    running: bool,
    active_panel: usize, // 0=source, 1=AST, 2=log
    ast_scroll: u16,
    log_scroll: u16,
    compiled: bool,
    editor: Editor,
    ast_lines: Vec<String>,
    log_lines: Vec<String>,
    tick: u64,
    demo_index: usize,
}

const DEMO_PROGRAMS: &[(&str, &str)] = &[
    ("Factorial (recursion)", r#"fn factorial(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

print(factorial(5));
print(factorial(10));
"#),
    ("Fibonacci (recursion)", r#"fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

print(fib(10));
print(fib(15));
"#),
    ("GCD (Euclid)", r#"fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    return gcd(b, a - (a / b) * b);
}

print(gcd(48, 18));
print(gcd(100, 75));
"#),
    ("While loop", r#"let mut sum = 0;
let mut i = 1;
while i <= 10 {
    sum = sum + i;
    i = i + 1;
}
print(sum);
"#),
    ("Arithmetic & booleans", r#"let x = 2 + 3 * 4;
let y = (10 - 2) / 4;
print(x);
print(y);
print(x > y);
print(!false && true);
"#),
    ("String operations", r#"let greeting = "Hello";
let target = " NEXUS";
print(greeting + target);
print(greeting == "Hello");
"#),
    ("Arrays", r#"let arr = [10, 20, 30, 40, 50];
print(arr[0]);
print(arr[4]);
print(len(arr));

let mut nums = [1, 2, 3];
push(nums, 4);
push(nums, 5);
print(len(nums));
print(nums[3]);
print(nums[4]);
"#),
    ("Collatz conjecture", r#"fn collatz(n: i32) -> i32 {
    let mut steps = 0;
    let mut x = n;
    while x != 1 {
        if x - (x / 2) * 2 == 0 {
            x = x / 2;
        } else {
            x = x * 3 + 1;
        }
        steps = steps + 1;
    }
    return steps;
}

print(collatz(27));
"#),
];

impl App {
    fn new() -> Self {
        let (_, source) = DEMO_PROGRAMS[0];

        Self {
            running: true,
            active_panel: 0,
            ast_scroll: 0,
            log_scroll: 0,
            compiled: false,
            editor: Editor::new(source),
            ast_lines: vec!["Press [c] to compile, [i] to edit".into()],
            log_lines: vec!["NEXUS Compiler ready.".into()],
            tick: 0,
            demo_index: 0,
        }
    }

    fn handle_event(&mut self, event: Event) {
        // Insert mode: route most keys to the editor
        if self.active_panel == 0 && self.editor.mode == EditorMode::Insert {
            match &event {
                Event::Escape => {
                    self.editor.mode = EditorMode::Normal;
                    if self.editor.dirty {
                        self.compile();
                        self.editor.dirty = false;
                    }
                    return;
                }
                Event::ArrowUp => { self.editor.move_up(); return; }
                Event::ArrowDown => { self.editor.move_down(); return; }
                Event::ArrowLeft => { self.editor.move_left(); return; }
                Event::ArrowRight => { self.editor.move_right(); return; }
                Event::Home => { self.editor.move_home(); return; }
                Event::End => { self.editor.move_end(); return; }
                Event::Delete => { self.editor.delete_char(); return; }
                Event::Key(b) => {
                    match *b {
                        0x03 => { self.running = false; return; }
                        0x1A => { self.editor.undo(); return; }
                        0x7F | 0x08 => { self.editor.backspace(); return; }
                        0x0D | 0x0A => { self.editor.insert_newline(); return; }
                        0x09 => { self.editor.insert_tab(); return; }
                        0x20..=0x7E => {
                            self.editor.insert_char(*b as char);
                            return;
                        }
                        _ => { return; }
                    }
                }
                Event::None => { return; }
            }
        }

        // Normal mode / other panels
        match event {
            Event::Key(b'q') | Event::Key(b'Q') | Event::Escape => {
                self.running = false;
            }
            Event::Key(0x03) => {
                self.running = false;
            }
            Event::Key(b'c') | Event::Key(b'C') => {
                self.compile();
            }
            Event::Key(b'n') | Event::Key(b'N') => {
                self.next_demo();
            }
            Event::Key(b'i') | Event::Key(b'I') => {
                if self.active_panel == 0 {
                    self.editor.mode = EditorMode::Insert;
                }
            }
            Event::Key(b'\t') => {
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
            0 => &mut self.editor.scroll,
            1 => &mut self.ast_scroll,
            _ => &mut self.log_scroll,
        }
    }

    fn next_demo(&mut self) {
        self.demo_index = (self.demo_index + 1) % DEMO_PROGRAMS.len();
        let (name, source) = DEMO_PROGRAMS[self.demo_index];
        self.editor.load(source);
        self.ast_lines = vec!["Press [c] to compile".into()];
        self.log_lines = vec![format!("Loaded: {}", name)];
        self.compiled = false;
        self.ast_scroll = 0;
        self.log_scroll = 0;
    }

    fn compile(&mut self) {
        let source = self.editor.content();
        self.log_lines.clear();
        self.ast_lines.clear();

        let start = Instant::now();

        // Phase 1: Lexical analysis
        let (tokens, lex_diags) = Lexer::new(&source).tokenize();
        let token_count = tokens.len().saturating_sub(1);
        self.log_lines.push(format!("Phase 1  Lexer: {} tokens", token_count));
        for d in &lex_diags {
            self.log_lines.push(format!("  lex: {}", d));
        }

        // Phase 2: Parsing
        let (program, parse_diags) = Parser::new(tokens).parse();
        let node_count = program.stmts.len();
        self.log_lines.push(format!("Phase 2  Parser: {} top-level statements", node_count));
        for d in &parse_diags {
            self.log_lines.push(format!("  parse: {}", d));
        }

        // AST pretty print
        let ast_str = pretty_print(&program);
        self.ast_lines = ast_str.lines().map(String::from).collect();
        if self.ast_lines.is_empty() {
            self.ast_lines.push("<empty AST>".into());
        }

        // Phase 3: Type checking
        let type_diags = type_check(&program);
        if type_diags.is_empty() {
            self.log_lines.push("Phase 3  TypeCheck: OK".into());
        } else {
            self.log_lines.push(format!("Phase 3  TypeCheck: {} errors", type_diags.len()));
            for d in &type_diags {
                self.log_lines.push(format!("  type: {}", d));
            }
        }

        let has_errors = !lex_diags.is_empty() || !parse_diags.is_empty() || !type_diags.is_empty();

        // Phase 4: Evaluation (only if no errors)
        if has_errors {
            self.log_lines.push("Skipping evaluation due to errors.".into());
        } else {
            match eval(&program) {
                Ok(result) => {
                    self.log_lines.push("Phase 4  Eval: OK".into());
                    if !result.output.is_empty() {
                        self.log_lines.push("── Output ──────────────".into());
                        for line in &result.output {
                            self.log_lines.push(format!("  {}", line));
                        }
                    }
                    let val_str = result.value.display();
                    if val_str != "()" {
                        self.log_lines.push(format!("Result: {}", val_str));
                    }
                }
                Err(e) => {
                    self.log_lines.push(format!("Phase 4  Eval ERROR: {}", e));
                }
            }
        }

        let elapsed = start.elapsed();
        self.log_lines.push(format!(
            "Compiled in {:.2}ms",
            elapsed.as_secs_f64() * 1000.0
        ));

        self.compiled = true;
    }

    fn update(&mut self) {
        self.tick += 1;
    }

    fn draw(&self, canvas: &mut Canvas) {
        let w = canvas.width();
        let h = canvas.height();

        canvas.clear();

        // ── Header ───────────────────────────────────────
        let (demo_name, _) = DEMO_PROGRAMS[self.demo_index];
        let mode_str = match self.editor.mode {
            EditorMode::Insert => "INSERT",
            EditorMode::Normal => "NORMAL",
        };
        let title = format!(
            " NEXUS v0.1.0 — {} │ {} │ {}",
            demo_name,
            mode_str,
            if self.compiled { "compiled" } else { "ready" }
        );
        StatusBar::draw(canvas, 0, w, &title, 51, 17);

        // ── Layout ───────────────────────────────────────
        let src_width = w / 2;
        let ast_width = w - src_width;
        let top_height = if h > 12 { h / 2 - 1 } else { h.saturating_sub(4) };
        let msg_y = top_height + 1;
        let msg_height = h.saturating_sub(msg_y + 2);

        // ── Source panel (top-left) ──────────────────────
        let src_fg = if self.active_panel == 0 { 51 } else { 246 };
        let src_title = if self.editor.mode == EditorMode::Insert {
            "SOURCE [EDIT]"
        } else {
            "SOURCE"
        };
        Panel::draw(canvas, 0, 1, src_width, top_height, src_title, src_fg, 234);

        let editor_height = top_height.saturating_sub(2);
        EditableTextArea::draw(
            canvas, 1, 2,
            src_width.saturating_sub(2), editor_height,
            &self.editor.lines, self.editor.scroll,
            self.editor.cursor_row, self.editor.cursor_col,
            self.editor.mode == EditorMode::Insert && self.active_panel == 0,
            15, 234,
        );

        // ── AST panel (top-right) ────────────────────────
        let ast_fg = if self.active_panel == 1 { 226 } else { 246 };
        Panel::draw(canvas, src_width, 1, ast_width, top_height, "AST", ast_fg, 234);
        TextArea::draw(
            canvas, src_width + 1, 2,
            ast_width.saturating_sub(2), top_height.saturating_sub(2),
            &self.ast_lines, self.ast_scroll,
            15, 234,
        );

        // ── Log panel (bottom) ──────────────────────────
        if msg_height >= 3 {
            let log_fg = if self.active_panel == 2 { 82 } else { 246 };
            Panel::draw(canvas, 0, msg_y, w, msg_height, "COMPILER LOG", log_fg, 234);

            if self.compiled {
                let bar_width = w.saturating_sub(4);
                if bar_width > 10 {
                    ProgressBar::draw(canvas, 2, msg_y + 1, bar_width, 1.0, 82, 234);
                }
            }

            let text_y = if self.compiled { msg_y + 2 } else { msg_y + 1 };
            let text_h = msg_height.saturating_sub(if self.compiled { 3 } else { 2 });

            let log_scroll = if self.log_lines.len() as u16 > text_h {
                self.log_lines.len() as u16 - text_h
            } else {
                0
            };

            TextArea::draw(
                canvas, 1, text_y,
                w.saturating_sub(2), text_h,
                &self.log_lines, log_scroll,
                82, 234,
            );
        }

        // ── Footer ───────────────────────────────────────
        let footer = if self.active_panel == 0 && self.editor.mode == EditorMode::Insert {
            format!(
                " [Esc] Normal  [Ctrl+Z] Undo  │  Ln {} Col {}  │  EDITING",
                self.editor.cursor_row + 1,
                self.editor.cursor_col + 1,
            )
        } else {
            " [q] Quit  [c] Compile  [n] Next Demo  [i] Edit  [Tab] Panel  │  Veritas Hortus".into()
        };
        StatusBar::draw(canvas, h - 1, w, &footer, 15, 22);
    }
}

// ── Main ─────────────────────────────────────────────────────

fn main() -> io::Result<()> {
    Terminal::enable_raw_mode()?;
    Terminal::enter_alternate()?;
    Terminal::clear()?;
    Terminal::hide_cursor()?;

    let (rows, cols) = Terminal::size();
    let mut canvas = Canvas::new(cols, rows);
    let mut app = App::new();

    let frame_duration = Duration::from_millis(33);

    while app.running {
        let frame_start = Instant::now();

        let event = read_event()?;
        app.handle_event(event);
        app.update();

        let (new_rows, new_cols) = Terminal::size();
        if new_rows != rows || new_cols != cols {
            canvas.resize(new_cols, new_rows);
            Terminal::clear()?;
        }

        let top_height = if canvas.height() > 12 {
            canvas.height() / 2 - 1
        } else {
            canvas.height().saturating_sub(4)
        };
        app.editor.set_viewport_height(top_height.saturating_sub(2));

        app.draw(&mut canvas);
        canvas.render()?;

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }
    }

    Terminal::show_cursor()?;
    Terminal::exit_alternate()?;
    Terminal::disable_raw_mode()?;
    Terminal::reset()?;

    println!("NEXUS TUI exited cleanly.");
    Ok(())
}
