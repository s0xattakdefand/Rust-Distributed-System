use serde::{Serialize, Deserialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalSignal {
    pub signal: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SystemStatus {
    pub active_signals: HashSet<String>,
    pub maintenance_mode: bool,
    pub throttled: bool,
    pub reason: Option<String>,
}

pub struct SocietyContext {
    pub active_signals: HashSet<String>,
    pub maintenance_mode: bool,
    pub throttled: bool,
    pub reason: Option<String>,
}

impl SocietyContext {
    pub fn new() -> Self {
        Self {
            active_signals: HashSet::new(),
            maintenance_mode: false,
            throttled: false,
            reason: None,
        }
    }

    pub fn process_signal(&mut self, sig: ExternalSignal) {
        self.active_signals.insert(sig.signal.clone());

        // Logic for demo: certain signals trigger modes
        match sig.signal.as_str() {
            "grid_overload" => {
                self.throttled = true;
                self.reason = Some("API throttled due to grid overload.".to_string());
            }
            "law_passed" => {
                self.maintenance_mode = true;
                self.reason = Some("API paused due to new regulation.".to_string());
            }
            "air_quality_alert" => {
                self.throttled = true;
                self.reason = Some("API throttled for community environment warning.".to_string());
            }
            "clear" => {
                self.active_signals.clear();
                self.maintenance_mode = false;
                self.throttled = false;
                self.reason = None;
            }
            _ => {}
        }
    }

    pub fn get_status(&self) -> SystemStatus {
        SystemStatus {
            active_signals: self.active_signals.clone(),
            maintenance_mode: self.maintenance_mode,
            throttled: self.throttled,
            reason: self.reason.clone(),
        }
    }
}
