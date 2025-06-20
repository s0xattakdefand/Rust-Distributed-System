use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub name: String,
    pub cost: f32,            // Lower = cheaper
    pub availability: f32,    // Higher = better
    pub is_eu: bool,          // For legal/GDPR use case
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrateIntent {
    pub goal: String,             // "min_cost", "max_availability", "eu_only"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationDecision {
    pub chosen_region: String,
    pub reason: String,
}

pub fn pick_region(regions: &[Region], intent: &OrchestrateIntent) -> OrchestrationDecision {
    match intent.goal.as_str() {
        "min_cost" => {
            let region = regions.iter().min_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap()).unwrap();
            OrchestrationDecision {
                chosen_region: region.name.clone(),
                reason: format!("Lowest cost: ${:.2}/hr", region.cost),
            }
        }
        "max_availability" => {
            let region = regions.iter().max_by(|a, b| a.availability.partial_cmp(&b.availability).unwrap()).unwrap();
            OrchestrationDecision {
                chosen_region: region.name.clone(),
                reason: format!("Highest availability: {:.2}%", region.availability * 100.0),
            }
        }
        "eu_only" => {
            let region = regions
                .iter()
                .filter(|r| r.is_eu)
                .min_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap());
            match region {
                Some(region) => OrchestrationDecision {
                    chosen_region: region.name.clone(),
                    reason: format!("Cheapest EU region: ${:.2}/hr", region.cost),
                },
                None => OrchestrationDecision {
                    chosen_region: "none".to_string(),
                    reason: "No EU region available".to_string(),
                },
            }
        }
        _ => OrchestrationDecision {
            chosen_region: "unknown".to_string(),
            reason: "Unknown intent".to_string(),
        },
    }
}
