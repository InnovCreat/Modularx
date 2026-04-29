// VERITAS HORTUS · C0 · living_archive.rs
// Living Archive — immutable, append-only event log (event sourcing)
//
// Every state change is an Event. Nothing is deleted, only appended.
// The full current state can always be reconstructed from the event log.
// Le plein état courant peut toujours être reconstruit depuis le journal d'événements.

use std::collections::VecDeque;

// ─── EVENT ───────────────────────────────────────────────────────────────────

/// A single immutable event in the Living Archive.
/// Un événement immuable unique dans l'Archive Vivante.
#[derive(Debug, Clone)]
pub struct Event {
    /// Unix timestamp of the event / Timestamp Unix de l'événement
    pub timestamp:  u64,
    /// Category label / Étiquette de catégorie (e.g. "MemoryCreated", "StateChanged")
    pub event_type: String,
    /// Raw data payload / Charge utile brute
    pub data:       Vec<u8>,
    /// Covenant-validated flag / Indicateur validé par le covenant
    pub covenanted: bool,
}

impl Event {
    pub fn new(event_type: impl Into<String>, data: Vec<u8>) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        Event {
            timestamp:  SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            event_type: event_type.into(),
            data,
            covenanted: true,
        }
    }
}

// ─── LIVING ARCHIVE ──────────────────────────────────────────────────────────

/// Immutable append-only event log.
///
/// Invariant: once appended, events cannot be modified or removed.
/// The archive grows monotonically — this is how truth is preserved.
#[derive(Debug, Default)]
pub struct LivingArchive {
    events:    VecDeque<Event>,
    /// Hard immutability lock — if false, the archive is corrupted
    immutable: bool,
}

impl LivingArchive {
    pub fn new() -> Self {
        Self {
            events:    VecDeque::new(),
            immutable: true,
        }
    }

    /// Append a new event. Returns Err if the archive is corrupted.
    pub fn append(&mut self, event: Event) -> Result<(), &'static str> {
        if !self.immutable {
            return Err("Archive corrupted — append rejected");
        }
        self.events.push_back(event);
        Ok(())
    }

    /// All events in chronological order / Tous les événements en ordre chronologique.
    pub fn events(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Events filtered by type / Événements filtrés par type.
    pub fn events_of_type<'a>(&'a self, event_type: &'a str) -> impl Iterator<Item = &'a Event> {
        self.events.iter().filter(move |e| e.event_type == event_type)
    }

    /// Latest event, if any / Dernier événement, s'il existe.
    pub fn latest(&self) -> Option<&Event> {
        self.events.back()
    }

    pub fn is_immutable(&self) -> bool {
        self.immutable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_and_count() {
        let mut archive = LivingArchive::new();
        let e = Event::new("MemoryCreated", b"seed-001".to_vec());
        assert!(archive.append(e).is_ok());
        assert_eq!(archive.event_count(), 1);
    }

    #[test]
    fn test_events_are_chronological() {
        let mut archive = LivingArchive::new();
        archive.append(Event::new("First",  vec![1])).unwrap();
        archive.append(Event::new("Second", vec![2])).unwrap();
        let events: Vec<_> = archive.events().collect();
        assert_eq!(events[0].event_type, "First");
        assert_eq!(events[1].event_type, "Second");
    }

    #[test]
    fn test_events_of_type_filter() {
        let mut archive = LivingArchive::new();
        archive.append(Event::new("MemoryCreated",  vec![])).unwrap();
        archive.append(Event::new("StateChanged",   vec![])).unwrap();
        archive.append(Event::new("MemoryCreated",  vec![])).unwrap();
        let mem: Vec<_> = archive.events_of_type("MemoryCreated").collect();
        assert_eq!(mem.len(), 2);
    }

    #[test]
    fn test_latest_event() {
        let mut archive = LivingArchive::new();
        archive.append(Event::new("First",  vec![])).unwrap();
        archive.append(Event::new("Latest", vec![])).unwrap();
        assert_eq!(archive.latest().unwrap().event_type, "Latest");
    }

    #[test]
    fn test_archive_immutable_by_default() {
        let archive = LivingArchive::new();
        assert!(archive.is_immutable());
    }

    #[test]
    fn test_event_timestamp_nonzero() {
        let e = Event::new("Test", vec![]);
        assert!(e.timestamp > 0);
    }

    #[test]
    fn test_event_covenanted_by_default() {
        let e = Event::new("Test", vec![]);
        assert!(e.covenanted);
    }
}
