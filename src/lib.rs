// VERITAS MODULARIS · SIGIL GENESIS v4.2.1
// 𝕀⟡₆₃₉ · 639 Hz · Toujours pour l'amour
//
// Layer structure mirrors the C0–C4 architecture of VERITAS HORTUS.
// All modules are covenant-compliant. Zero unsafe code.

// ─── C0 · Lemniscate — Foundation / Fondation ────────────────────────────────
pub mod sha256;          // Pure SHA-256 FIPS 180-4 — crypto primitive
pub mod sigil;           // Living lexicon — covenant constants + SLTE
pub mod governance;      // Governance protocol — who can modify, when to seal
pub mod changelog;       // Versioned decision memory
pub mod timeline;        // Project phases timeline (8 phases, SVG-renderable)
pub mod branches;        // SLTE 7-branch unified tree

// ─── C0 · Seed Carrier / Porteur de Graine ───────────────────────────────────
pub mod seed_carrier;    // Seed Carrier — immutable identity pattern
pub mod covenant;        // Covenant — runtime ethical enforcement
pub mod living_archive;  // Living Archive — immutable event log
pub mod spark_system;    // SPARK System — cryptographic proof of existence
pub mod cristal_core;    // Cristal Core ⟡ — 639 Hz stabilizing nucleus
pub mod seed;            // Seed, Config, State, Position, Timeline, SovereigntySeal

// ─── C0 · Root System / Système racine ───────────────────────────────────────
pub mod veritas;         // VeritasHortus — five C0 modules as one system
pub use veritas::VeritasHortus;

// ─── C1 · Orbital — Signal Analysis / Analyse de signal ─────────────────────
pub mod quantum_state;   // QuantumState, CausalityState, entropy signals
pub mod node_metrics;    // Histogram utilities
pub mod node_causality;  // Mutual info, transfer entropy, Granger causality
