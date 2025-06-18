use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
    pub region: String,
    pub reason: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct KnowledgeBase {
    pub incidents: HashMap<String, Incident>, // key: region
}

impl KnowledgeBase {
    pub fn new() -> Self {
        Self { incidents: HashMap::new() }
    }

    pub fn remember_failure(&mut self, region: &str, reason: &str, cooldown_secs: i64) {
        let incident = Incident {
            region: region.to_string(),
            reason: reason.to_string(),
            expires_at: Utc::now() + Duration::seconds(cooldown_secs),
        };
        self.incidents.insert(region.to_string(), incident);
    }

    pub fn is_region_avoided(&self, region: &str) -> bool {
        if let Some(incident) = self.incidents.get(region) {
            if incident.expires_at > Utc::now() {
                return true;
            }
        }
        false
    }

    pub fn clean_expired(&mut self) {
        let now = Utc::now();
        self.incidents.retain(|_, incident| incident.expires_at > now);
    }

    pub fn snapshot(&self) -> Vec<Incident> {
        self.incidents.values().cloned().collect()
    }
}
