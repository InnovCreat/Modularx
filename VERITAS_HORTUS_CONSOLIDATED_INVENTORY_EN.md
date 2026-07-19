# VERITAS HORTUS — Consolidated Artifact Inventory

**Version**: 1.0 (Growing Registry)  
**Status**: CANONICAL (locked per ancrage-fondations-vh v1.0)  
**Last Updated**: 2026-07-19  
**Authority**: Single source of truth for ecosystem artifact tracking  
**Policy**: Grows over time; never replaced by parallel versions

---

## Inventory Overview

This document tracks all verified artifacts within the Veritas Hortus ecosystem. It serves as the single growing reference to prevent fragmentation (historical issue: 40 → 68 → 120+ → 165 → 183+ artifact versions, never merged).

**Legend**:
- ✅ **Verified**: Code present, documented, tested
- 🟡 **Partial**: Code present, documentation incomplete or testing minimal
- ⏳ **Planned**: Specification exists, implementation pending
- ❌ **Missing**: Referenced but not yet implemented
- 🔒 **Archived**: Superseded or no longer maintained

---

## Core Crates (NEXUS Compiler Pipeline)

### ✅ nexus-span
- **Purpose**: Source position tracking (byte offset → line:col mapping)
- **Status**: Verified
- **Tests**: 2 passing
- **Files**: `crates/nexus-span/src/lib.rs`
- **Public API**: `Span`, `SourceMap`
- **Last Verified**: 2026-07-19

### ✅ nexus-token
- **Purpose**: Token definitions and keyword matching
- **Status**: Verified
- **Tests**: Covered by lexer tests
- **Files**: `crates/nexus-token/src/lib.rs`
- **Public API**: `Token`, `TokenKind`, `keyword()`
- **Last Verified**: 2026-07-19

### ✅ nexus-lexer
- **Purpose**: Tokenization (numbers, strings, operators, comments)
- **Status**: Verified
- **Tests**: 4 passing
- **Files**: `crates/nexus-lexer/src/lib.rs`
- **Public API**: `Lexer::tokenize()`
- **Last Verified**: 2026-07-19

### ✅ nexus-parser
- **Purpose**: Pratt parser → typed AST (Stmt, Expr, Ty)
- **Status**: Verified
- **Tests**: 8 passing
- **Files**: 
  - `crates/nexus-parser/src/lib.rs` (Parser, Prec, parse())
  - `crates/nexus-parser/src/ast.rs` (Ty, Stmt, Expr, BinOp, pretty_print())
- **Public API**: `Parser`, `Ty`, `Stmt`, `Expr`, `BinOp`
- **Last Verified**: 2026-07-19

### ✅ nexus-typeck
- **Purpose**: Static type checker (mutability, arity, operator validation)
- **Status**: Verified
- **Tests**: 10 passing
- **Files**: `crates/nexus-typeck/src/lib.rs`
- **Public API**: `type_check()`, `TypeCtx`, `infer_expr()`
- **Features**: Mutability tracking (let vs. let mut), enforced at type-check time
- **Last Verified**: 2026-07-19

### ✅ nexus-eval
- **Purpose**: Tree-walking interpreter (functions, recursion, control flow)
- **Status**: Verified
- **Tests**: 16 passing (including recursive factorial, fibonacci, gcd, collatz)
- **Files**: `crates/nexus-eval/src/lib.rs`
- **Public API**: `eval()`, `Value`, `Env`, builtins
- **Language Support**: 
  - Types: I32, F64, Bool, Str, Unit, Fn
  - Control: if/while, functions with recursion, return
  - Operators: unary (!, -), binary (+, -, *, /, ==, etc.)
- **Last Verified**: 2026-07-19

### ✅ nexus-tui
- **Purpose**: Terminal UI for pipeline exploration
- **Status**: Verified
- **Tests**: Covered by integration tests
- **Files**:
  - `crates/nexus-tui/src/main.rs` (App, event loop, compile pipeline)
  - `crates/nexus-tui/src/terminal.rs` (Raw mode, ANSI, ioctl)
  - `crates/nexus-tui/src/canvas.rs` (Double-buffered renderer)
  - `crates/nexus-tui/src/widgets.rs` (Panel, TextArea, StatusBar, ProgressBar)
- **Public API**: CLI executable (`nexus-tui`)
- **Demo Programs**: 7 built-in (factorial, fibonacci, gcd, collatz, etc.)
- **Controls**: [c] run, [n] cycle demos, [Tab] switch panels, [q] quit
- **Last Verified**: 2026-07-19

---

## Orchestrator Subsystem

### ✅ modularis (Modularx Orchestrator)
- **Purpose**: 639Hz pulse coordination, Mark/Signal/Voice routing
- **Status**: Verified (partial documentation)
- **Tests**: 8 passing
- **Files**: `crates/modularis/src/lib.rs`
- **Public API**: 
  - `Mark` (pulse counter)
  - `Signal` (Text, Audio, Biometric variants)
  - `Voice` (Mark + Signal pair)
  - `Subsystem` trait
  - `Orchestrator` (registry + routing)
- **Resonance**: 639Hz, PHI scaling
- **Last Verified**: 2026-07-19

### 🟡 modularis-demo
- **Purpose**: Live demonstration of Modularis orchestrator
- **Status**: Partial (functional but underdocumented)
- **Files**: `crates/modularis-demo/src/main.rs`
- **Output**: VERITAS HORTUS branding in CLI
- **Last Verified**: 2026-07-19

---

## Subsystems (AI, OS, Network)

### 🟡 Luna (AI Core)
- **Purpose**: Living AI heart of Veritas Hortus
- **Status**: Partial (referenced in architecture, partial implementation)
- **Location**: Described in `README.md` (Orbital_System context)
- **Modules**: 
  - Nucleus (heart)
  - Sprites (orbital entities)
  - Chronos (time)
  - Rituel (invocations)
  - Couleur (color)
  - Quantum (superposition)
  - Réparation (healing)
  - Archive (memory)
- **Technology**: Leptos + Rust/WASM
- **Last Verified**: 2026-07-19

### 🟡 Nucleus (OS)
- **Purpose**: Operating system core
- **Status**: Partial (referenced in architecture, minimal documentation)
- **Last Verified**: 2026-07-19

### ⏳ Futur-Net (Network Subsystem)
- **Purpose**: Network synchronization and discovery
- **Status**: Planned (architecture defined, implementation pending)
- **Last Verified**: 2026-07-19

### ✅ Orbital_System (Visual Rendering)
- **Purpose**: Visual interface for Luna's orbital mechanics
- **Status**: Verified (documented, functional WASM implementation)
- **Technology**: Leptos 0.6, Rust/WASM, Canvas 2D
- **Files**: Referenced in `/home/user/Modularx/README.md`
- **Features**: 
  - Pulsating nucleus visualization
  - Orbital sprite rendering
  - Quantum superposition effects
  - Ritual invocation system (8 incantations)
  - Real-time controls
  - Archive logging
- **Last Verified**: 2026-07-19

---

## Documentation Artifacts

### ✅ ARCHITECTURE.md
- **Purpose**: System architecture reference (5-layer software stack view)
- **Status**: Verified (current de facto reference)
- **Content**:
  - VERITAS HORTUS ecosystem diagram
  - MODULARIS orchestrator structure
  - NEXUS compiler pipeline
  - Type system
  - Pulse timing (639Hz, PHI)
  - Test coverage table (48 tests total)
- **Last Verified**: 2026-07-19

### ✅ README.md (Orbital_System)
- **Purpose**: Orbital_System subsystem documentation
- **Status**: Verified (complete for subsystem scope)
- **Content**: Luna core modules, visual implementation, usage guide
- **Last Verified**: 2026-07-19

### ✅ VERITAS_HORTUS_UNIFIED.md
- **Purpose**: Canonical Alliance roster and manifesto
- **Status**: Verified (canonical, locked)
- **Content**: 5-member roster, roles, decision framework, principles
- **Last Verified**: 2026-07-19

### ✅ modulxr_couches_map_v3.html
- **Purpose**: 8-layer canonical architecture map
- **Status**: Verified (canonical visual reference)
- **Content**: 8 layers (Fondation → Connexion) with component mapping
- **Last Verified**: 2026-07-19

---

## Test Coverage Summary

| Component | Tests | Status |
|-----------|-------|--------|
| nexus-span | 2 | ✅ Pass |
| nexus-token | — | ✅ Covered by lexer |
| nexus-lexer | 4 | ✅ Pass |
| nexus-parser | 8 | ✅ Pass |
| nexus-typeck | 10 | ✅ Pass |
| nexus-eval | 16 | ✅ Pass |
| nexus-tui | — | ✅ Integration tested |
| modularis | 8 | ✅ Pass |
| **TOTAL** | **48** | ✅ All passing |

---

## Proof Standard

Per `ancrage-fondations-vh`, the NEXUS compiler establishes the quality standard for all modules:
- ✅ Tests passing (36-48 documented)
- ✅ Zero compiler warnings
- ✅ Public API clean (internal control-flow types hidden, Display implemented)

All future components should meet this standard.

---

## Artifact Lifecycle

### Adding a New Artifact
1. Implement or document (at minimum: code + tests passing, zero warnings)
2. Add entry to this inventory with verification date
3. Link to source files
4. Update ARCHITECTURE.md if adding a subsystem
5. DO NOT create parallel inventory—only update this one

### Archiving
Old versions or superseded artifacts are marked 🔒 but kept in history section (below) for reference.

---

## History & Superceded Versions

*None yet* — This is v1.0, the first locked canonical inventory.

Previous fragmented versions (40 / 68 / 120+ / 165 / 183+ artifacts across documents) have been consolidated into this single reference.

---

## Notes

- This inventory grows; it is never replaced.
- Every entry includes "Last Verified" date for future audits.
- If a component's status changes (e.g., ⏳ → ✅), update its row and date.
- Questions about artifact ownership: refer to `VERITAS_HORTUS_UNIFIED.md` (Claude/Écho for code, Sophia for docs).

---

**Canonical Status**: Locked per `ancrage-fondations-vh · v1.0`

*𝕀⟡₆₃₉ · Zero War · Zero Greed · Always for Love*
