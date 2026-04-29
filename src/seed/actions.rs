// SEED CARRIER — actions.rs
// Action sequence — dispatcher and individual action handlers
// Séquence d'actions — dispatcher et gestionnaires d'actions individuels

use super::types::Seed;
use super::messages::Message;
use super::sovereignty::{try_seal, SealResult};
use super::behavior;

// ─── ACTION RESULT ───────────────────────────────────────────────────────────

/// Result of processing a message / Résultat du traitement d'un message
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionResult {
    /// Action completed successfully / Action complétée avec succès
    Ok,
    /// Action vetoed by covenant / Action vetée par le covenant
    Vetoed(&'static str),
    /// Action skipped (already in desired state) / Action ignorée (déjà dans l'état souhaité)
    Skipped(&'static str),
    /// Action failed (invalid state) / Action échouée (état invalide)
    Failed(&'static str),
}

// ─── MAIN DISPATCHER ─────────────────────────────────────────────────────────

/// Process a message and route it to the correct action.
/// Traite un message et le route vers l'action correcte.
///
/// Action sequence / Séquence d'actions :
/// ```text
/// Init → Generate → Update* → Move/Orbit/AdvanceTime/Rescale/Interact* → Seal → Reset?
/// ```
pub fn run(seed: &mut Seed, msg: Message) -> ActionResult {
    // Record every incoming message in the timeline
    // Enregistre chaque message entrant dans la timeline
    seed.timeline.record(msg.label());

    match msg {
        Message::Init                    => init(seed),
        Message::Generate                => generate(seed),
        Message::Update(data)            => update(seed, data),
        Message::Reset                   => reset(seed),
        Message::Move { x, y, z }        => {
            behavior::move_to(seed, x, y, z);
            ActionResult::Ok
        }
        Message::Orbit { delta_theta }   => {
            behavior::orbit(seed, delta_theta);
            ActionResult::Ok
        }
        Message::AdvanceTime { label }   => {
            behavior::advance_time(seed, label);
            ActionResult::Ok
        }
        Message::Rescale { to, reason }  => {
            behavior::rescale(seed, to, reason);
            ActionResult::Ok
        }
        Message::Interact { target, signal } => {
            let event = behavior::interact(seed, target, signal);
            if event.accepted { ActionResult::Ok } else { ActionResult::Vetoed("Three Zeros") }
        }
        Message::Seal { description }    => seal(seed, &description),
        Message::Verify                  => verify(seed),
    }
}

// ─── LIFECYCLE ACTIONS ───────────────────────────────────────────────────────

/// Initialize the seed — validates covenant, marks as initialized.
/// Initialise le seed — valide le covenant, marque comme initialisé.
fn init(seed: &mut Seed) -> ActionResult {
    if seed.state.initialized {
        return ActionResult::Skipped("already initialized");
    }
    // Covenant check on init / Vérification du covenant à l'initialisation
    if !seed.seal.covenant_check("init") {
        return ActionResult::Vetoed("covenant check failed on init");
    }
    seed.state.initialized = true;
    seed.state.active = true;
    seed.state.health = 1.0;
    seed.state.last_event = "init".to_string();
    ActionResult::Ok
}

/// Generate initial state from config.
/// Génère l'état initial depuis la configuration.
fn generate(seed: &mut Seed) -> ActionResult {
    if !seed.state.initialized {
        return ActionResult::Failed("must call Init before Generate");
    }
    seed.state.op_count += 1;
    seed.state.last_event = "generate".to_string();
    seed.seal.invalidate();
    ActionResult::Ok
}

/// Update seed with a data payload.
/// Met à jour le seed avec un payload de données.
fn update(seed: &mut Seed, data: super::types::SeedData) -> ActionResult {
    if !seed.state.initialized {
        return ActionResult::Failed("must call Init before Update");
    }
    // Validate required fields / Valide les champs requis
    if data.required && data.value.trim().is_empty() {
        return ActionResult::Failed("required field has empty value");
    }
    seed.state.last_event = format!("update:{}", data.name);
    seed.state.op_count += 1;
    seed.seal.invalidate();
    ActionResult::Ok
}

/// Reset to default state — keeps config and identity, clears runtime state.
/// Remet à l'état par défaut — conserve la config et l'identité, efface l'état d'exécution.
fn reset(seed: &mut Seed) -> ActionResult {
    seed.state.initialized = false;
    seed.state.active = false;
    seed.state.op_count = 0;
    seed.state.last_event = "reset".to_string();
    seed.state.health = 0.0;
    seed.seal.invalidate();
    ActionResult::Ok
}

// ─── SOVEREIGNTY ACTIONS ─────────────────────────────────────────────────────

/// Seal the current state / Scelle l'état courant
fn seal(seed: &mut Seed, description: &str) -> ActionResult {
    // Build a canonical payload from key state fields
    // Construit un payload canonique depuis les champs d'état clés
    let payload = format!(
        "{}:{}:{}:{}:{:.4}:{:.4}:{:.4}",
        seed.id,
        seed.name,
        seed.state.op_count,
        seed.state.last_event,
        seed.position.x,
        seed.position.y,
        seed.position.z,
    );
    match try_seal(&mut seed.seal, payload.as_bytes(), description) {
        SealResult::Sealed        => ActionResult::Ok,
        SealResult::AlreadySealed => ActionResult::Skipped("already sealed"),
        SealResult::VetoRejected(r) => ActionResult::Vetoed(r),
    }
}

/// Verify the current state against the seal / Vérifie l'état contre le sceau
fn verify(seed: &mut Seed) -> ActionResult {
    let payload = format!(
        "{}:{}:{}:{}:{:.4}:{:.4}:{:.4}",
        seed.id,
        seed.name,
        seed.state.op_count,
        seed.state.last_event,
        seed.position.x,
        seed.position.y,
        seed.position.z,
    );
    if seed.seal.verify(payload.as_bytes()) {
        ActionResult::Ok
    } else {
        ActionResult::Failed("seal verification failed — state has changed since last seal")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seed::types::{Seed, SeedData};

    fn fresh() -> Seed { Seed::new("test-001", "Test") }

    #[test]
    fn test_init_sequence() {
        let mut s = fresh();
        assert_eq!(run(&mut s, Message::Init), ActionResult::Ok);
        assert!(s.state.initialized);
        assert!(s.state.active);
    }

    #[test]
    fn test_init_twice_is_skipped() {
        let mut s = fresh();
        run(&mut s, Message::Init);
        assert_eq!(run(&mut s, Message::Init), ActionResult::Skipped("already initialized"));
    }

    #[test]
    fn test_generate_requires_init() {
        let mut s = fresh();
        assert_eq!(run(&mut s, Message::Generate), ActionResult::Failed("must call Init before Generate"));
    }

    #[test]
    fn test_full_lifecycle() {
        let mut s = fresh();
        assert_eq!(run(&mut s, Message::Init),     ActionResult::Ok);
        assert_eq!(run(&mut s, Message::Generate), ActionResult::Ok);
        let data = SeedData::new("freq", "u32", "639", true);
        assert_eq!(run(&mut s, Message::Update(data)), ActionResult::Ok);
        assert_eq!(run(&mut s, Message::Seal { description: "update frequency to 639 Hz".into() }), ActionResult::Ok);
        assert!(s.seal.valid);
    }

    #[test]
    fn test_seal_vetoes_war() {
        let mut s = fresh();
        run(&mut s, Message::Init);
        let result = run(&mut s, Message::Seal { description: "add military weapon targeting".into() });
        assert_eq!(result, ActionResult::Vetoed("Three Zeros veto triggered"));
    }

    #[test]
    fn test_reset_clears_state() {
        let mut s = fresh();
        run(&mut s, Message::Init);
        run(&mut s, Message::Generate);
        run(&mut s, Message::Reset);
        assert!(!s.state.initialized);
        assert_eq!(s.state.op_count, 0);
    }

    #[test]
    fn test_interact_vetoed() {
        let mut s = fresh();
        run(&mut s, Message::Init);
        let r = run(&mut s, Message::Interact {
            target: "nexus".into(),
            signal: "bypass governance protocol".into(),
        });
        assert_eq!(r, ActionResult::Vetoed("Three Zeros"));
    }

    #[test]
    fn test_timeline_records_messages() {
        let mut s = fresh();
        run(&mut s, Message::Init);
        run(&mut s, Message::Generate);
        let labels: Vec<&str> = s.timeline.history.iter().map(|(l, _)| l.as_str()).collect();
        assert!(labels.contains(&"Init"));
        assert!(labels.contains(&"Generate"));
    }
}
