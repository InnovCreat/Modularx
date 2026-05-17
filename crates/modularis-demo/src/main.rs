// ════════════════════════════════════════════════════════════════
// MODULARIS LIVE DEMO
// Full orchestrator: inject code → compile via NEXUS → project
// into SLTQ 4D space → compress via Vortex → display results
// All at 639Hz pulse cadence
// ════════════════════════════════════════════════════════════════

use std::time::{Duration, Instant};
use std::io::{self, Write};

use modularis::{
    Mark, Signal, Voice,
    FREQUENCY_HZ, PULSE_MICROS, PHI,
    bridge::NexusCompiler,
    sltq::{Projector, Vortex},
};

// ── ANSI helpers ────────────────────────────────────────────────

fn clear() { print!("\x1B[2J\x1B[H"); }
fn bold(s: &str) -> String { format!("\x1B[1m{}\x1B[0m", s) }
fn cyan(s: &str) -> String { format!("\x1B[36m{}\x1B[0m", s) }
fn green(s: &str) -> String { format!("\x1B[32m{}\x1B[0m", s) }
fn yellow(s: &str) -> String { format!("\x1B[33m{}\x1B[0m", s) }
fn magenta(s: &str) -> String { format!("\x1B[35m{}\x1B[0m", s) }
fn dim(s: &str) -> String { format!("\x1B[2m{}\x1B[0m", s) }
fn red(s: &str) -> String { format!("\x1B[31m{}\x1B[0m", s) }

fn bar(value: f64, width: usize) -> String {
    let filled = ((value.clamp(0.0, 1.0)) * width as f64) as usize;
    let empty = width - filled;
    format!("{}{}",
        "█".repeat(filled),
        dim(&"░".repeat(empty)),
    )
}

// ── Demo programs ───────────────────────────────────────────────

const PROGRAMS: &[(&str, &str)] = &[
    ("Fibonacci", r#"fn fib(n: i32) -> i32 {
    if n <= 1 { return n; }
    return fib(n - 1) + fib(n - 2);
}
print(fib(10));"#),

    ("Array Operations", r#"let arr = [10, 20, 30, 40, 50];
print(arr[0]);
print(arr[4]);
print(len(arr));
let mut nums = [1, 2, 3];
push(nums, 4);
print(len(nums));"#),

    ("GCD (Euclid)", r#"fn gcd(a: i32, b: i32) -> i32 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let t = y;
        y = x - (x / y) * y;
        x = t;
    }
    return x;
}
print(gcd(48, 18));
print(gcd(100, 75));"#),

    ("Collatz", r#"fn collatz(n: i32) -> i32 {
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
print(collatz(27));"#),

    ("String + Boolean", r#"let greeting = "Hello";
let target = " NEXUS";
print(greeting + target);
print(1 + 2 * 3 > 5);
print(!false && true);"#),
];

// ── Main ────────────────────────────────────────────────────────

fn main() {
    clear();
    println!("{}", bold(&cyan("╔══════════════════════════════════════════════════════════════╗")));
    println!("{}", bold(&cyan("║          MODULARIS — VERITAS HORTUS LIVE DEMO               ║")));
    println!("{}", bold(&cyan("║     639Hz Pulse • SLTQ 4D Space • PHI Vortex • NEXUS        ║")));
    println!("{}", bold(&cyan("╚══════════════════════════════════════════════════════════════╝")));
    println!();

    let genesis = Instant::now();
    let mut compiler = NexusCompiler::new();
    let mut projector = Projector::new(FREQUENCY_HZ * 5); // 5 second decay window
    let mut vortex = Vortex::at_origin();

    println!("{}", dim("─── System Constants ───────────────────────────────────────────"));
    println!("  Frequency:    {} {}",
        bold(&format!("{}", FREQUENCY_HZ)),
        dim("Hz"));
    println!("  Pulse period: {} {}",
        bold(&format!("{}", PULSE_MICROS)),
        dim("µs"));
    println!("  PHI ratio:    {} {}",
        bold(&format!("{:.15}", PHI)),
        dim("(golden)"));
    println!("  Genesis:      {} {}",
        bold("T+0"),
        dim("(now)"));
    println!();

    // Process each demo program
    for (i, (name, source)) in PROGRAMS.iter().enumerate() {
        let pulse = Mark::from_instant(genesis, Instant::now()).pulse;
        projector.set_pulse(pulse);

        println!("{}", bold(&yellow(&format!(
            "━━━ Program {}/{}: {} ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
            i + 1, PROGRAMS.len(), name
        ))));
        println!();

        // Show source
        println!("  {}", dim("SOURCE:"));
        for line in source.lines() {
            println!("    {}", cyan(line));
        }
        println!();

        // Compile through NEXUS bridge
        let compile_start = Instant::now();
        let result = compiler.submit(source, pulse);
        let compile_time = compile_start.elapsed();

        // Show compilation results
        println!("  {}", dim("COMPILATION:"));
        println!("    Tokens:     {}", bold(&format!("{}", result.token_count)));
        println!("    Time:       {} {}",
            bold(&format!("{:.3}", compile_time.as_secs_f64() * 1000.0)),
            dim("ms"));
        println!("    Pulse:      {} {}",
            bold(&format!("#{}", pulse)),
            dim(&format!("(T+{:.1}ms)", Mark { pulse }.to_duration().as_secs_f64() * 1000.0)));

        if !result.type_errors.is_empty() {
            println!("    TypeCheck:  {}", red(&format!("{} errors", result.type_errors.len())));
            for e in &result.type_errors {
                println!("      {}", red(e));
            }
        } else {
            println!("    TypeCheck:  {}", green("OK"));
        }

        if let Some(ref err) = result.error {
            println!("    Eval:       {}", red(err));
        } else if !result.output.is_empty() {
            println!("    Output:     {}", green(&result.output.join(", ")));
        }
        println!();

        // Project into SLTQ space
        let voice = Voice {
            mark: Mark { pulse },
            signal: Signal::Text {
                token_count: result.token_count as u32,
                checksum: simple_hash(source),
            },
        };

        let positioned = projector.project(&voice);
        let salience = vortex.salience(&positioned);

        println!("  {}", dim("SLTQ PROJECTION:"));
        println!("    S (Spatial):    {} {:.4}", bar(positioned.coord.s, 20), positioned.coord.s);
        println!("    L (Linguistic): {} {:.4}", bar(positioned.coord.l, 20), positioned.coord.l);
        println!("    T (Temporal):   {} {:.4}", bar(positioned.coord.t, 20), positioned.coord.t);
        println!("    Q (Quantum):    {} {:.4}", bar(positioned.coord.q, 20), positioned.coord.q);
        println!("    Magnitude:      {}", bold(&format!("{:.4}", positioned.coord.magnitude())));
        println!("    Salience:       {}", bold(&magenta(&format!("{:.4}", salience))));
        println!();

        // Vortex compression
        let points = vec![positioned];
        let compressed = vortex.compress(&points);
        let new_coord = compressed[0].coord;

        println!("  {}", dim("VORTEX (PHI compression):"));
        println!("    S: {:.4} → {:.4}  {}",
            positioned.coord.s, new_coord.s,
            dim(&format!("(×{:.4})", 1.0/PHI)));
        println!("    L: {:.4} → {:.4}", positioned.coord.l, new_coord.l);
        println!("    T: {:.4} → {:.4}", positioned.coord.t, new_coord.t);
        println!("    Q: {:.4} → {:.4}", positioned.coord.q, new_coord.q);
        println!("    Distance to center: {:.4} → {:.4}",
            vortex.center.distance(&positioned.coord),
            vortex.center.distance(&new_coord));
        println!();

        // Brief pause between programs for readability
        std::thread::sleep(Duration::from_millis(50));
    }

    // Final summary
    let total_pulse = Mark::from_instant(genesis, Instant::now()).pulse;
    let total_time = genesis.elapsed();

    println!("{}", bold(&cyan("━━━ ORCHESTRATOR SUMMARY ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")));
    println!();
    println!("  Programs compiled: {}", bold(&format!("{}", compiler.compile_count())));
    println!("  Total pulses:      {}", bold(&format!("{}", total_pulse)));
    println!("  Total time:        {} {}",
        bold(&format!("{:.2}", total_time.as_secs_f64() * 1000.0)),
        dim("ms"));
    println!("  Avg per compile:   {} {}",
        bold(&format!("{:.3}", total_time.as_secs_f64() * 1000.0 / PROGRAMS.len() as f64)),
        dim("ms"));
    println!("  Vortex steps:      {}", bold(&format!("{}", vortex.compression_steps)));
    println!();
    println!("  {}", dim("\"Jamais pour la guerre, jamais pour l'argent, toujours pour l'amour.\""));
    println!();
    println!("{}", bold(&cyan("═══════════════════════════════════════════════════════════════")));
    println!("  {} {} • {} {} • {} {}",
        dim("Resonance:"), bold("639Hz"),
        dim("Fidelity:"), bold("100%"),
        dim("Entropy:"), bold("1"));
    println!("{}", bold(&cyan("═══════════════════════════════════════════════════════════════")));

    io::stdout().flush().ok();
}

fn simple_hash(s: &str) -> u32 {
    let mut h: u32 = 0;
    for b in s.bytes() {
        h = h.wrapping_mul(31).wrapping_add(b as u32);
    }
    h
}
