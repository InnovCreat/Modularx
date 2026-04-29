// SEED CARRIER — types.rs
// Core data structures: objects, spatial, temporal, scaling
// SIGIL GENESIS v4.2.1 · Layer 1 · 639 Hz
//
// Structures de données fondamentales : objets, spatial, temporel, échelle

use super::scaling::ScaleSystem;
use super::sovereignty::SovereigntySeal;

// ─── SEED (root object / objet racine) ───────────────────────────────────────

/// The Seed Carrier — immutable identity pattern traversing the entire system.
/// Le Seed Carrier — pattern d'identité immuable traversant tout le système.
///
/// A Seed holds everything needed for a module to exist and connect:
/// config, state, position, timeline, scale, and its sovereignty seal.
#[derive(Debug, Clone)]
pub struct Seed {
    // Identity / Identité
    pub id:          String,
    pub name:        &'static str,

    // Core objects / Objets principaux
    pub config:      Config,
    pub state:       State,

    // Spatio-temporal / Spatio-temporel
    pub position:    Position,
    pub timeline:    Timeline,

    // Scaling / Échelle
    pub scale:       ScaleSystem,

    // Sovereignty / Souveraineté
    pub seal:        SovereigntySeal,
}

impl Seed {
    pub fn new(id: impl Into<String>, name: &'static str) -> Self {
        Seed {
            id:       id.into(),
            name,
            config:   Config::default(),
            state:    State::default(),
            position: Position::default(),
            timeline: Timeline::default(),
            scale:    ScaleSystem::default(),
            seal:     SovereigntySeal::default(),
        }
    }
}

// ─── CONFIG ──────────────────────────────────────────────────────────────────

/// Module configuration — static parameters set at init.
/// Configuration du module — paramètres statiques définis à l'initialisation.
#[derive(Debug, Clone)]
pub struct Config {
    /// Module version / Version du module
    pub version:     &'static str,
    /// Whether this module requires covenant validation before mutations
    /// Si ce module requiert la validation du covenant avant toute mutation
    pub sovereign:   bool,
    /// Resonance frequency in Hz / Fréquence de résonance en Hz
    pub frequency:   u32,
    /// Maximum allowed operations per cycle / Opérations max par cycle
    pub max_ops:     u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            version:   "1.0",
            sovereign: true,
            frequency: 639,
            max_ops:   1000,
        }
    }
}

// ─── STATE ───────────────────────────────────────────────────────────────────

/// Module runtime state — mutable, tracked over time.
/// État d'exécution du module — mutable, suivi dans le temps.
#[derive(Debug, Clone, Default)]
pub struct State {
    /// Active / actif
    pub active:      bool,
    /// Initialized / initialisé
    pub initialized: bool,
    /// Operation count / Compteur d'opérations
    pub op_count:    u64,
    /// Last event label / Étiquette du dernier événement
    pub last_event:  String,
    /// Health score 0.0–1.0 / Score de santé 0.0–1.0
    pub health:      f32,
}

// ─── POSITION (spatial / spatiale) ───────────────────────────────────────────

/// Spatial position in 3D VR space (Veritas Hortus coordinate system).
/// Position spatiale dans l'espace VR 3D (système de coordonnées Veritas Hortus).
#[derive(Debug, Clone)]
pub struct Position {
    pub x:      f64,
    pub y:      f64,
    pub z:      f64,
    /// Orbital radius (distance from Luna core) / Rayon orbital (distance du cœur Luna)
    pub radius: f64,
    /// Orbital angle in radians / Angle orbital en radians
    pub theta:  f64,
    /// Bounds: (min, max) per axis / Limites : (min, max) par axe
    pub bounds: [(f64, f64); 3],
}

impl Default for Position {
    fn default() -> Self {
        Position {
            x:      0.0,
            y:      0.0,
            z:      0.0,
            radius: 1.0,
            theta:  0.0,
            bounds: [(-1000.0, 1000.0); 3],
        }
    }
}

// ─── TIMELINE (temporal / temporelle) ────────────────────────────────────────

/// Temporal tracking — when events happened and how long things lasted.
/// Suivi temporel — quand les événements se sont produits et leur durée.
#[derive(Debug, Clone)]
pub struct Timeline {
    /// Current Unix timestamp / Timestamp Unix courant
    pub current_time: u64,
    /// Creation timestamp / Timestamp de création
    pub created_at:   u64,
    /// Event history (label, timestamp) / Historique d'événements
    pub history:      Vec<(String, u64)>,
    /// Max history entries / Entrées max dans l'historique
    pub max_history:  usize,
}

impl Default for Timeline {
    fn default() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Timeline {
            current_time: now,
            created_at:   now,
            history:      Vec::new(),
            max_history:  512,
        }
    }
}

impl Timeline {
    /// Record an event in the timeline / Enregistre un événement dans la timeline
    pub fn record(&mut self, label: impl Into<String>) {
        use std::time::{SystemTime, UNIX_EPOCH};
        self.current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        if self.history.len() >= self.max_history {
            self.history.remove(0);
        }
        self.history.push((label.into(), self.current_time));
    }
}

// ─── SEED DATA (payload / charge utile) ──────────────────────────────────────

/// Data payload carried by messages — name, type, value, required flag.
/// Charge utile portée par les messages — nom, type, valeur, indicateur requis.
#[derive(Debug, Clone)]
pub struct SeedData {
    /// Data field name / Nom du champ de données
    pub name:      String,
    /// Data type descriptor / Descripteur de type
    pub data_type: String,
    /// Serialized value / Valeur sérialisée
    pub value:     String,
    /// Whether this field is required / Si ce champ est requis
    pub required:  bool,
    /// Optional unit / Unité optionnelle (e.g. "Hz", "m", "s")
    pub unit:      Option<String>,
}

impl SeedData {
    pub fn new(name: impl Into<String>, data_type: impl Into<String>, value: impl Into<String>, required: bool) -> Self {
        SeedData {
            name:      name.into(),
            data_type: data_type.into(),
            value:     value.into(),
            required,
            unit:      None,
        }
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_new() {
        let s = Seed::new("seed-001", "TestSeed");
        assert_eq!(s.name, "TestSeed");
        assert_eq!(s.id, "seed-001");
        assert!(!s.state.initialized);
    }

    #[test]
    fn test_config_defaults_to_639hz() {
        let c = Config::default();
        assert_eq!(c.frequency, 639);
        assert!(c.sovereign);
    }

    #[test]
    fn test_position_default_origin() {
        let p = Position::default();
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
        assert_eq!(p.z, 0.0);
        assert_eq!(p.radius, 1.0);
    }

    #[test]
    fn test_timeline_records_events() {
        let mut t = Timeline::default();
        t.record("init");
        t.record("generate");
        assert_eq!(t.history.len(), 2);
        assert_eq!(t.history[0].0, "init");
    }

    #[test]
    fn test_seed_data_required_flag() {
        let d = SeedData::new("freq", "u32", "639", true).with_unit("Hz");
        assert!(d.required);
        assert_eq!(d.unit, Some("Hz".to_string()));
    }
}
