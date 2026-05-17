# MODULARX — System Architecture

```
                        ╔══════════════════════════════════════════╗
                        ║        VERITAS HORTUS ECOSYSTEM         ║
                        ║         Resonance: 639Hz | PHI          ║
                        ╚══════════════════════════════════════════╝

    ┌─────────────────────────────────────────────────────────────────────────┐
    │                         MODULARIS ORCHESTRATOR                           │
    │                    (639Hz Pulse Coordination Layer)                      │
    │                                                                         │
    │   ┌─────────┐    ┌─────────────────────────────────────────┐            │
    │   │  Mark   │───▶│              Voice Bus                   │            │
    │   │ (pulse) │    │  [Voice] ─── [Voice] ─── [Voice] ───▶   │            │
    │   └─────────┘    └──────────────────┬──────────────────────┘            │
    │                                     │                                   │
    │        ┌────────────────────────────┼────────────────────────┐          │
    │        │            ROUTING         │   (by Channel)         │          │
    │        ▼            ▼               ▼              ▼         │          │
    │   ┌─────────┐ ┌─────────┐    ┌─────────┐    ┌─────────┐    │          │
    │   │ Visual  │ │  Audio  │    │Biometric│    │  Text   │    │          │
    │   │ Channel │ │ Channel │    │ Channel │    │ Channel │    │          │
    │   └────┬────┘ └────┬────┘    └────┬────┘    └────┬────┘    │          │
    │        │            │              │              │         │          │
    └────────┼────────────┼──────────────┼──────────────┼─────────┘          │
             │            │              │              │                     │
    ═════════╪════════════╪══════════════╪══════════════╪═════════════════════╪═
             │            │              │              │    SUBSYSTEM LAYER  │
             ▼            ▼              ▼              ▼                     │
    ┌─────────────┐ ┌──────────┐ ┌────────────┐ ┌──────────────────┐        │
    │    LUNA     │ │ NUCLEUS  │ │  FUTUR-NET │ │      NEXUS       │        │
    │  (AI Core) │ │   (OS)   │ │  (Network) │ │   (Compiler)     │        │
    │            │ │          │ │            │ │                  │        │
    │  Emotion   │ │  Memory  │ │  Protocol  │ │ ┌──────────────┐│        │
    │  Reasoning │ │  Process │ │  Sync      │ │ │    Source    ││        │
    │  Learning  │ │  I/O     │ │  Discovery │ │ └──────┬───────┘│        │
    │            │ │          │ │            │ │        ▼        │        │
    └─────────────┘ └──────────┘ └────────────┘ │ ┌──────────────┐│        │
                                                │ │    Lexer     ││        │
                                                │ │  (nexus-lexer)│        │
                                                │ └──────┬───────┘│        │
                                                │        ▼        │        │
                                                │ ┌──────────────┐│        │
                                                │ │    Parser    ││        │
                                                │ │(nexus-parser)││        │
                                                │ └──────┬───────┘│        │
                                                │        ▼        │        │
                                                │ ┌──────────────┐│        │
                                                │ │  TypeCheck   ││        │
                                                │ │(nexus-typeck)││        │
                                                │ └──────┬───────┘│        │
                                                │        ▼        │        │
                                                │ ┌──────────────┐│        │
                                                │ │  Evaluator   ││        │
                                                │ │ (nexus-eval) ││        │
                                                │ └──────┬───────┘│        │
                                                │        ▼        │        │
                                                │ ┌──────────────┐│        │
                                                │ │     TUI      ││        │
                                                │ │ (nexus-tui)  ││        │
                                                │ └──────────────┘│        │
                                                └──────────────────┘        │
                                                                            │
    ════════════════════════════════════════════════════════════════════════════
```

## Workspace Crates

```
Modularx/
├── Cargo.toml                    # Workspace root
├── ARCHITECTURE.md               # This file
│
└── crates/
    ├── modularis/                 # Orchestrator — 639Hz pulse system
    │   └── src/lib.rs            #   Mark, Signal, Voice, Subsystem trait, Orchestrator
    │
    ├── nexus-span/               # Byte-offset spans & source mapping
    │   └── src/lib.rs            #   Span, SourceMap
    │
    ├── nexus-token/              # Token definitions
    │   └── src/lib.rs            #   Token, TokenKind, keyword()
    │
    ├── nexus-lexer/              # Lexical analysis
    │   └── src/lib.rs            #   Lexer::tokenize()
    │
    ├── nexus-parser/             # Pratt parser + AST
    │   └── src/
    │       ├── lib.rs            #   Parser, Prec, parse()
    │       └── ast.rs            #   Ty, Stmt, Expr, BinOp, pretty_print()
    │
    ├── nexus-typeck/             # Static type checker
    │   └── src/lib.rs            #   type_check(), TypeCtx, infer_expr()
    │
    ├── nexus-eval/               # Tree-walking interpreter
    │   └── src/lib.rs            #   eval(), Value, Env, builtins
    │
    └── nexus-tui/                # Terminal interface
        └── src/
            ├── main.rs           #   App, event loop, compile pipeline
            ├── terminal.rs       #   Raw mode, ANSI, ioctl
            ├── canvas.rs         #   Double-buffered renderer
            └── widgets.rs        #   Panel, TextArea, StatusBar, ProgressBar
```

## Data Flow

```
                    MODULARIS (639Hz)
                         │
                    inject(Signal::Text)
                         │
                         ▼
┌──────────────────────────────────────────────────────┐
│                   NEXUS PIPELINE                      │
│                                                      │
│  "let arr = [1,2,3];"                               │
│         │                                            │
│         ▼                                            │
│  ┌────────────┐  tokens   ┌────────────┐            │
│  │   LEXER    │──────────▶│   PARSER   │            │
│  │ (8 tokens) │           │ (Pratt)    │            │
│  └────────────┘           └─────┬──────┘            │
│                                 │ AST                │
│                                 ▼                    │
│  ┌────────────┐           ┌────────────┐            │
│  │  TYPECK    │◀──────────│    AST     │            │
│  │ (validate) │           │  Stmt::Let │            │
│  └─────┬──────┘           └────────────┘            │
│        │ OK                                         │
│        ▼                                            │
│  ┌────────────┐  output   ┌────────────┐            │
│  │   EVAL     │──────────▶│    TUI     │            │
│  │ (execute)  │           │ (display)  │            │
│  └────────────┘           └────────────┘            │
│                                                      │
└──────────────────────────────────────────────────────┘
```

## Type System

```
Ty
├── I32          # 32-bit integer (stored as f64 at runtime)
├── F64          # 64-bit float
├── Bool         # true / false
├── Str          # String
├── Array(Ty)    # [i32], [bool], etc.
├── Unit         # ()
├── Never        # ! (diverging)
├── Unknown      # ? (inference placeholder)
├── Error        # <error> (recovery)
└── Fn(Vec, Ty)  # fn(i32, i32) -> bool
```

## Pulse Timing

```
639 Hz = 1,564 µs per pulse

│←── 1 pulse ───▶│←── 1 pulse ───▶│←── 1 pulse ───▶│
├────────────────┼────────────────┼────────────────┤
0              1564             3128             4692  (µs)
pulse 0          pulse 1          pulse 2          pulse 3

1 second = 639 pulses
1 minute = 38,340 pulses
PHI ratio between layers: 1.618033988749895
```

## Test Coverage

| Crate | Tests | Status |
|-------|-------|--------|
| nexus-span | 2 | Pass |
| nexus-token | 0 | — |
| nexus-lexer | 4 | Pass |
| nexus-parser | 8 | Pass |
| nexus-typeck | 10 | Pass |
| nexus-eval | 16 | Pass |
| modularis | 8 | Pass |
| **Total** | **48** | **All pass** |
