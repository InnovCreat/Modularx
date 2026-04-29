// SEED CARRIER — mod.rs
// Module declarations and public re-exports
// Déclarations de modules et ré-exports publics
//
// SIGIL GENESIS v4.2.1 · Layer 1 · 639 Hz
// Pattern: immutable identity traversing the entire system
// Patron : identité immuable traversant tout le système
//
// File structure / Structure de fichiers :
//   types.rs      — Seed, Config, State, Position, Timeline, SeedData
//   messages.rs   — Message enum (all possible inputs)
//   actions.rs    — run() dispatcher + action handlers
//   scaling.rs    — Scale, ScaleSystem, ValueChart, MetricType
//   behavior.rs   — spatial, temporal, interaction logic
//   sovereignty.rs — SovereigntySeal + covenant verification

pub mod types;
pub mod messages;
pub mod actions;
pub mod scaling;
pub mod behavior;
pub mod sovereignty;

// ─── Public re-exports / Ré-exports publics ───────────────────────────────────

pub use types::{Seed, Config, State, Position, Timeline, SeedData};
pub use messages::Message;
pub use actions::{run, ActionResult};
pub use scaling::{Scale, ScaleSystem, ValueChart, MetricType};
pub use sovereignty::{SovereigntySeal, SealResult};
