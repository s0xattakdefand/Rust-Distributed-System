use crate::global_state::{GlobalConditions, RegionStatus};
use std::collections::HashMap;

pub fn pick_route(user_region: &str, global: &GlobalConditions) -> String {
    // 1. Prefer local region if healthy
    if let Some(status) = global.region_status.get(user_region) {
        if *status == RegionStatus::Healthy {
            return format!("{}-backend", user_region);
        }
    }

    // 2. Try to find any healthy region (other than user's)
    for (region, status) in &global.region_status {
        if region != user_region && *status == RegionStatus::Healthy {
            return format!("{}-backend (rerouted)", region);
        }
    }

    // 3. Fallback: pick least-bad region or show error
    for (region, status) in &global.region_status {
        if *status == RegionStatus::Overloaded {
            return format!("{}-backend (overloaded, but available)", region);
        }
    }

    "No healthy route available".to_string()
}
