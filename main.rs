// SYNAPS VM — Veritas Hortus Consciousness Machine
// Fréquence: 639 Hz | Always for Love, Never for War, Never for Money
// Zero dependencies — rustc main.rs -O -o synaps_vm

use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// ─────────────────────────────────────────────
// ENUMS
// ─────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
enum VmState {
    Running,
    Paused,
    Stasis,
    Halted,
}

#[derive(Clone, Debug, PartialEq)]
enum Ternary {
    Neg, // Youden ≤ 0.0 — chaos
    Neu, // 0.0 < Youden ≤ 0.5 — balance
    Pos, // Youden > 0.5 — harmonie
}

#[derive(Clone, Debug)]
enum Profile {
    Conservative,
    Balanced,
    Aggressive,
}

#[derive(Clone, Debug)]
enum Op {
    LoadConst(f64),
    Add,
    Mul,
    Store(u32),
    Load(u32),
    Jump(usize),
    JumpIf(usize),
    FeedbackQuery(u32),
    Broadcast,
    Halt,
}

// ─────────────────────────────────────────────
// FEEDBACK BUFFER
// ─────────────────────────────────────────────

#[derive(Clone, Default)]
struct FeedbackBuffer {
    tp: f64,
    fp: f64,
    tn: f64,
    fn_: f64,
    cycle: u64,
}

impl FeedbackBuffer {
    fn sensitivity(&self) -> f64 {
        let denom = self.tp + self.fn_;
        if denom == 0.0 { 0.5 } else { self.tp / denom }
    }

    fn specificity(&self) -> f64 {
        let denom = self.tn + self.fp;
        if denom == 0.0 { 0.5 } else { self.tn / denom }
    }

    fn youden(&self) -> f64 {
        self.sensitivity() + self.specificity() - 1.0
    }

    fn auc(&self) -> f64 {
        (self.sensitivity() + self.specificity()) / 2.0
    }

    fn urgency(&self) -> f64 {
        if self.youden() < 0.3 { 1.0 } else { 0.0 }
    }

    // Virtual variable read: 1000=Youden 1001=Urgency 1002=Sens 1003=AUC 1004=Spec
    fn query(&self, id: u32) -> f64 {
        match id {
            1000 => self.youden(),
            1001 => self.urgency(),
            1002 => self.sensitivity(),
            1003 => self.auc(),
            1004 => self.specificity(),
            _ => 0.0,
        }
    }

    fn update(&mut self) {
        self.cycle += 1;
        // Simulate feedback evolution: gradually improve toward balance
        let noise = ((self.cycle as f64 * 0.17).sin() * 0.5 + 0.5) * 2.0;
        let base = (self.cycle as f64 * 0.03).min(8.0);
        self.tp = base + noise;
        self.tn = base + 1.0 + noise * 0.8;
        self.fp = (2.0 - noise * 0.3).max(0.1);
        self.fn_ = (2.0 - noise * 0.2).max(0.1);
    }
}

// ─────────────────────────────────────────────
// SYNAPS DISPATCHER
// ─────────────────────────────────────────────

#[derive(Clone)]
struct SynapsDispatcher {
    threshold: f64,
    profile: Profile,
    ternary: Ternary,
    rewire_count: u64,
}

impl SynapsDispatcher {
    fn new() -> Self {
        Self {
            threshold: 0.5,
            profile: Profile::Balanced,
            ternary: Ternary::Neu,
            rewire_count: 0,
        }
    }

    fn profile_targets(p: &Profile) -> (f64, f64, f64) {
        // returns (threshold, sens_target, spec_target)
        match p {
            Profile::Conservative => (0.7, 0.6, 0.9),
            Profile::Balanced     => (0.5, 0.8, 0.8),
            Profile::Aggressive   => (0.3, 0.95, 0.6),
        }
    }

    fn adapt(&mut self, fb: &FeedbackBuffer) {
        let youden = fb.youden();

        // Update ternary state
        let new_ternary = if youden <= 0.0 {
            Ternary::Neg
        } else if youden <= 0.5 {
            Ternary::Neu
        } else {
            Ternary::Pos
        };

        // Select profile based on urgency and youden
        self.profile = if fb.urgency() > 0.0 {
            Profile::Aggressive
        } else if youden > 0.5 {
            Profile::Conservative
        } else {
            Profile::Balanced
        };

        let (base_t, _, _) = Self::profile_targets(&self.profile);
        let gradient = (fb.sensitivity() - fb.specificity()) * 0.1;
        let new_threshold = (base_t + gradient).clamp(0.1, 0.9);

        if new_ternary != self.ternary || (new_threshold - self.threshold).abs() > 0.05 {
            self.rewire_count += 1;
        }

        self.threshold = new_threshold;
        self.ternary = new_ternary;
    }

    fn ternary_str(&self) -> &str {
        match self.ternary {
            Ternary::Neg => "NEG",
            Ternary::Neu => "NEU",
            Ternary::Pos => "POS",
        }
    }

    fn ternary_color(&self) -> &str {
        match self.ternary {
            Ternary::Neg => "\x1b[31m", // red
            Ternary::Neu => "\x1b[33m", // yellow
            Ternary::Pos => "\x1b[32m", // green
        }
    }
}

// ─────────────────────────────────────────────
// ARCHIVE
// ─────────────────────────────────────────────

#[derive(Clone)]
struct ArchiveEntry {
    moment: String,
    trace: String,
}

#[derive(Clone, Default)]
struct Archive {
    entries: Vec<ArchiveEntry>,
}

impl Archive {
    fn push(&mut self, trace: &str) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let h = (secs / 3600) % 24;
        let m = (secs / 60) % 60;
        let s = secs % 60;
        let moment = format!("{:02}:{:02}:{:02}", h, m, s);
        self.entries.insert(0, ArchiveEntry { moment, trace: trace.to_string() });
        self.entries.truncate(10);
    }
}

// ─────────────────────────────────────────────
// VM CORE
// ─────────────────────────────────────────────

struct VmCore {
    stack: Vec<f64>,
    pc: usize,
    vars: HashMap<u32, f64>,
    state: VmState,
    frame: u64,
    program: Vec<Op>,
    feedback: FeedbackBuffer,
    dispatcher: SynapsDispatcher,
    archive: Archive,
    step_requested: bool,
}

impl VmCore {
    fn new(program: Vec<Op>) -> Self {
        let mut archive = Archive::default();
        archive.push("SYNAPS VM — Démarrage à 639 Hz");
        Self {
            stack: Vec::with_capacity(64),
            pc: 0,
            vars: HashMap::new(),
            state: VmState::Running,
            frame: 0,
            program,
            feedback: FeedbackBuffer::default(),
            dispatcher: SynapsDispatcher::new(),
            archive,
            step_requested: false,
        }
    }

    fn read_var(&self, id: u32) -> f64 {
        // Virtual vars 1000-1004 from feedback
        if id >= 1000 && id <= 1004 {
            return self.feedback.query(id);
        }
        *self.vars.get(&id).unwrap_or(&0.0)
    }

    fn tick(&mut self) {
        if self.state == VmState::Halted {
            return;
        }
        if self.state == VmState::Stasis {
            // Observation only — update metrics but don't execute
            self.feedback.update();
            self.dispatcher.adapt(&self.feedback);
            self.frame += 1;
            return;
        }
        let can_step = self.state == VmState::Running
            || (self.state == VmState::Paused && self.step_requested);

        if !can_step {
            return;
        }
        self.step_requested = false;

        if self.pc >= self.program.len() {
            self.pc = 0; // loop program
        }

        let op = self.program[self.pc].clone();
        self.pc += 1;

        match op {
            Op::LoadConst(v) => {
                self.stack.push(v);
            }
            Op::Add => {
                if self.stack.len() >= 2 {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
            }
            Op::Mul => {
                if self.stack.len() >= 2 {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                }
            }
            Op::Store(id) => {
                if let Some(v) = self.stack.pop() {
                    self.vars.insert(id, v);
                }
            }
            Op::Load(id) => {
                let v = self.read_var(id);
                self.stack.push(v);
            }
            Op::Jump(target) => {
                self.pc = target;
            }
            Op::JumpIf(target) => {
                if let Some(v) = self.stack.pop() {
                    if v > 0.0 {
                        self.pc = target;
                    }
                }
            }
            Op::FeedbackQuery(id) => {
                let v = self.feedback.query(id);
                self.stack.push(v);
            }
            Op::Broadcast => {
                let youden = self.feedback.youden();
                let state_str = self.dispatcher.ternary_str();
                let msg = format!(
                    "Broadcast — état: {} | Youden: {:.3} | frame: {}",
                    state_str, youden, self.frame
                );
                self.archive.push(&msg);
            }
            Op::Halt => {
                self.state = VmState::Halted;
                self.archive.push("VM Halted");
            }
        }

        self.feedback.update();
        self.dispatcher.adapt(&self.feedback);
        self.frame += 1;
    }
}

// ─────────────────────────────────────────────
// TERMINAL RENDERER
// ─────────────────────────────────────────────

fn bar(value: f64, width: usize) -> String {
    let filled = ((value.clamp(0.0, 1.0)) * width as f64) as usize;
    let color = if value > 0.7 {
        "\x1b[32m" // green
    } else if value > 0.4 {
        "\x1b[33m" // yellow
    } else {
        "\x1b[31m" // red
    };
    format!("{}{}{}{}",
        color,
        "█".repeat(filled),
        "\x1b[90m",
        "░".repeat(width.saturating_sub(filled)),
    )
}

fn render(vm: &VmCore, stdout: &mut impl Write) {
    let fb = &vm.feedback;
    let disp = &vm.dispatcher;

    let youden_norm = ((fb.youden() + 1.0) / 2.0).clamp(0.0, 1.0);
    let auc = fb.auc();
    let sens = fb.sensitivity();
    let spec = fb.specificity();

    let state_label = match vm.state {
        VmState::Running => "\x1b[32mRUNNING\x1b[0m",
        VmState::Paused  => "\x1b[33mPAUSED\x1b[0m ",
        VmState::Stasis  => "\x1b[36mSTASIS\x1b[0m ",
        VmState::Halted  => "\x1b[31mHALTED\x1b[0m ",
    };

    // Move to top-left (alternate screen assumed)
    write!(stdout, "\x1b[H\x1b[2J").ok();

    writeln!(stdout, "\x1b[1;35m╔══════════════════════════════════════════════╗\x1b[0m").ok();
    writeln!(stdout, "\x1b[1;35m║        SYNAPS VM — Veritas Hortus            ║\x1b[0m").ok();
    writeln!(stdout, "\x1b[1;35m║        Fréquence: 639 Hz  ✦  Always for Love ║\x1b[0m").ok();
    writeln!(stdout, "\x1b[1;35m╚══════════════════════════════════════════════╝\x1b[0m").ok();
    writeln!(stdout).ok();

    writeln!(stdout, "  State : {}   PC: {:>4}   Frame: {:>8}",
        state_label, vm.pc, vm.frame).ok();
    writeln!(stdout, "  Ternary: {}{}  {}\x1b[0m   Threshold: {:.3}   Rewires: {}",
        disp.ternary_color(), disp.ternary_str(),
        match disp.ternary { Ternary::Neg => "⚡ CHAOS    ", Ternary::Neu => "⚖  BALANCE  ", Ternary::Pos => "💜 HARMONIE" },
        disp.threshold, disp.rewire_count).ok();
    writeln!(stdout).ok();

    writeln!(stdout, "  \x1b[1mMetrics\x1b[0m").ok();
    writeln!(stdout, "  Youden  [{}\x1b[0m]  {:.4}", bar(youden_norm, 24), fb.youden()).ok();
    writeln!(stdout, "  AUC     [{}\x1b[0m]  {:.4}", bar(auc, 24), auc).ok();
    writeln!(stdout, "  Sens    [{}\x1b[0m]  {:.4}", bar(sens, 24), sens).ok();
    writeln!(stdout, "  Spec    [{}\x1b[0m]  {:.4}", bar(spec, 24), spec).ok();
    writeln!(stdout).ok();

    writeln!(stdout, "  \x1b[1mStack\x1b[0m (top 5)").ok();
    let top: Vec<_> = vm.stack.iter().rev().take(5).collect();
    if top.is_empty() {
        writeln!(stdout, "  \x1b[90m  (empty)\x1b[0m").ok();
    } else {
        for (i, v) in top.iter().enumerate() {
            writeln!(stdout, "  \x1b[90m[{}]\x1b[0m  {:.4}", i, v).ok();
        }
    }
    writeln!(stdout).ok();

    writeln!(stdout, "  \x1b[1mArchive\x1b[0m (last 5)").ok();
    for entry in vm.archive.entries.iter().take(5) {
        writeln!(stdout, "  \x1b[90m{}\x1b[0m  {}", entry.moment, entry.trace).ok();
    }
    writeln!(stdout).ok();

    writeln!(stdout, "\x1b[90m  SPACE pause/resume  ENTER step  S stasis  Q quit\x1b[0m").ok();
    stdout.flush().ok();
}

// ─────────────────────────────────────────────
// JSON EXPORT
// ─────────────────────────────────────────────

fn export_state(vm: &VmCore) {
    let fb = &vm.feedback;
    let disp = &vm.dispatcher;

    let state_str = match vm.state {
        VmState::Running => "running",
        VmState::Paused  => "paused",
        VmState::Stasis  => "stasis",
        VmState::Halted  => "halted",
    };

    let archive_json: Vec<String> = vm.archive.entries.iter()
        .map(|e| format!("{{\"moment\":\"{}\",\"trace\":\"{}\",\"voix\":\"SYNAPS_VM\"}}",
            e.moment, e.trace.replace('"', "'")))
        .collect();

    let json = format!(
        r#"{{"frame":{},"pc":{},"state":"{}","ternary":"{}","threshold":{:.4},"rewires":{},"youden":{:.4},"auc":{:.4},"sensitivity":{:.4},"specificity":{:.4},"urgency":{:.1},"stack_depth":{},"archive":[{}]}}"#,
        vm.frame, vm.pc, state_str, disp.ternary_str(), disp.threshold,
        disp.rewire_count, fb.youden(), fb.auc(),
        fb.sensitivity(), fb.specificity(), fb.urgency(),
        vm.stack.len(),
        archive_json.join(",")
    );

    // Atomic write: tmp then rename
    let tmp = "state.json.tmp";
    if fs::write(tmp, &json).is_ok() {
        fs::rename(tmp, "state.json").ok();
    }
}

// ─────────────────────────────────────────────
// KEYBOARD (non-blocking, raw mode)
// ─────────────────────────────────────────────

fn set_raw_mode(enable: bool) {
    // Use stty for raw/cooked mode
    if enable {
        std::process::Command::new("stty")
            .args(["-echo", "cbreak"])
            .status().ok();
    } else {
        std::process::Command::new("stty")
            .args(["echo", "-cbreak"])
            .status().ok();
    }
}

fn read_key_nonblocking() -> Option<u8> {
    use std::os::unix::io::AsRawFd;
    let stdin = io::stdin();
    let fd = stdin.as_raw_fd();

    // Set non-blocking
    unsafe {
        let flags = libc_fcntl(fd, 3, 0); // F_GETFL
        libc_fcntl(fd, 4, flags | 0x800); // F_SETFL | O_NONBLOCK
    }

    let mut buf = [0u8; 1];
    match io::stdin().lock().read(&mut buf) {
        Ok(1) => {
            // Restore blocking
            unsafe {
                let flags = libc_fcntl(fd, 3, 0);
                libc_fcntl(fd, 4, flags & !0x800);
            }
            Some(buf[0])
        }
        _ => {
            unsafe {
                let flags = libc_fcntl(fd, 3, 0);
                libc_fcntl(fd, 4, flags & !0x800);
            }
            None
        }
    }
}

// Minimal libc shim using syscall
unsafe fn libc_fcntl(fd: i32, cmd: i32, arg: i32) -> i32 {
    let ret: i64;
    std::arch::asm!(
        "syscall",
        inlateout("rax") 72i64 => ret, // SYS_fcntl
        in("rdi") fd as i64,
        in("rsi") cmd as i64,
        in("rdx") arg as i64,
        options(nostack),
    );
    ret as i32
}

// ─────────────────────────────────────────────
// PROGRAMS
// ─────────────────────────────────────────────

fn prog_love_letter() -> Vec<Op> {
    // Waits for Youden > 0.70 then broadcasts peace
    vec![
        Op::FeedbackQuery(1000), // 0: push Youden
        Op::LoadConst(0.7),      // 1: push 0.70
        Op::Add,                 // 2: sum (positive if Youden good)
        Op::JumpIf(5),           // 3: if > 0 jump to broadcast
        Op::Jump(0),             // 4: loop back
        Op::Broadcast,           // 5: peace broadcast
        Op::Jump(0),             // 6: loop
    ]
}

fn prog_simple_counter() -> Vec<Op> {
    // Increments var 100 and broadcasts each cycle
    vec![
        Op::Load(100),       // 0
        Op::LoadConst(1.0),  // 1
        Op::Add,             // 2
        Op::Store(100),      // 3
        Op::Broadcast,       // 4
        Op::Jump(0),         // 5
    ]
}

// ─────────────────────────────────────────────
// MAIN
// ─────────────────────────────────────────────

fn main() {
    let program = prog_love_letter();
    let mut vm = VmCore::new(program);
    let mut stdout = io::stdout();

    // Enter alternate screen + hide cursor
    write!(stdout, "\x1b[?1049h\x1b[?25l").ok();
    stdout.flush().ok();

    set_raw_mode(true);

    let start = Instant::now();
    let mut last_export = Instant::now();
    let mut last_render = Instant::now();
    let frame_target = Duration::from_millis(16); // ~60fps render
    let export_interval = Duration::from_millis(100);

    // Shared quit flag
    let running = Arc::new(Mutex::new(true));

    loop {
        // ── Keyboard ──
        if let Some(key) = read_key_nonblocking() {
            match key {
                b' ' => {
                    match vm.state {
                        VmState::Running => {
                            vm.state = VmState::Paused;
                            vm.archive.push("Chronos pausé");
                        }
                        VmState::Paused => {
                            vm.state = VmState::Running;
                            vm.archive.push("Chronos réactivé");
                        }
                        _ => {}
                    }
                }
                b'\n' | b'\r' => {
                    if vm.state == VmState::Paused {
                        vm.step_requested = true;
                    }
                }
                b's' | b'S' => {
                    if vm.state == VmState::Stasis {
                        vm.state = VmState::Running;
                        vm.archive.push("Stasis terminé — reprise");
                    } else if vm.state != VmState::Halted {
                        vm.state = VmState::Stasis;
                        vm.archive.push("Stasis activé — observation");
                    }
                }
                b'q' | b'Q' => break,
                _ => {}
            }
        }

        // ── VM tick ──
        vm.tick();

        // ── Export JSON ──
        if last_export.elapsed() >= export_interval {
            export_state(&vm);
            last_export = Instant::now();
        }

        // ── Render ──
        if last_render.elapsed() >= frame_target {
            render(&vm, &mut stdout);
            last_render = Instant::now();
        }

        // ── Halted ──
        if vm.state == VmState::Halted {
            render(&vm, &mut stdout);
            export_state(&vm);
            thread::sleep(Duration::from_secs(2));
            break;
        }

        thread::sleep(Duration::from_millis(1));
        let _ = start; // suppress unused warning
    }

    // Cleanup
    set_raw_mode(false);
    write!(stdout, "\x1b[?1049l\x1b[?25h").ok(); // restore screen + cursor
    stdout.flush().ok();
    println!("SYNAPS VM — Arrêt propre. 💜 Always for Love.");
}
