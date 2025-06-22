use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub actor: String,
    pub action: String,
    pub resource: Option<String>,
    pub outcome: String, // e.g., "success", "fail"
    pub detail: Option<String>,
}

pub struct AuditLog {
    pub entries: Vec<AuditEntry>,
}

impl AuditLog {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn log(&mut self, entry: AuditEntry) {
        self.entries.push(entry);
    }

    pub fn get(&self) -> Vec<AuditEntry> {
        self.entries.clone()
    }

    pub fn as_csv(&self) -> String {
        let mut wtr = csv::Writer::from_writer(vec![]);
        for e in &self.entries {
            let _ = wtr.serialize(e);
        }
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    }
}
