#[derive(Clone, Debug)]
pub struct ArchiveEntry {
    pub moment: String,
    pub trace: String,
    pub voix: String,
}

#[derive(Clone, Debug)]
pub struct Archive {
    pub entries: Vec<ArchiveEntry>,
    pub max_entries: usize,
}

impl Archive {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 10,
        }
    }

    pub fn log(&mut self, trace: &str) {
        let now = js_sys::Date::new_0();
        let h = now.get_hours();
        let m = now.get_minutes();
        let s = now.get_seconds();
        let moment = format!("{:02}:{:02}:{:02}", h, m, s);

        self.entries.insert(
            0,
            ArchiveEntry {
                moment,
                trace: trace.to_string(),
                voix: "Orbital_System".to_string(),
            },
        );

        if self.entries.len() > self.max_entries {
            self.entries.truncate(self.max_entries);
        }
    }
}
