use crate::intent::Intent;
use crate::services::{process_service_a, process_service_b, process_service_c};

pub fn pick_route(intent: &Intent) -> String {
    match intent.mission.as_str() {
        "save_cost" => process_service_a(&intent.mission),
        "high_availability" => process_service_b(&intent.mission),
        "try_new" => process_service_c(&intent.mission),
        // Smart logic: use urgency as tie-breaker
        _ => {
            match intent.urgency {
                Some(urgency) if urgency > 7 => process_service_b(&intent.mission),
                Some(urgency) if urgency < 4 => process_service_a(&intent.mission),
                _ => process_service_c(&intent.mission),
            }
        }
    }
}
