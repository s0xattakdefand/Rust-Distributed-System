use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegionStatus {
    Healthy,
    UnderAttack,
    Overloaded,
    Down,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConditions {
    pub region_status: HashMap<String, RegionStatus>,
}

impl GlobalConditions {
    pub fn default() -> Self {
        let mut region_status = HashMap::new();
        region_status.insert("asia".to_string(), RegionStatus::Healthy);
        region_status.insert("us".to_string(), RegionStatus::Healthy);
        region_status.insert("europe".to_string(), RegionStatus::Healthy);
        Self { region_status }
    }
}
