use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentIntent {
    pub app: String,
    pub business_priority: String, // "cost", "latency", "co2", "balanced"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateRegion {
    pub name: String,
    pub cost: f64,
    pub latency_ms: f64,
    pub co2_kg: f64,
    pub law_ok: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployDecision {
    pub selected: String,
    pub score: f64,
    pub reason: String,
    pub candidates: Vec<CandidateRegion>,
}

pub fn get_candidates() -> Vec<CandidateRegion> {
    vec![
        CandidateRegion {
            name: "us-west".into(),
            cost: 50.0,
            latency_ms: 100.0,
            co2_kg: 30.0,
            law_ok: true,
        },
        CandidateRegion {
            name: "eu-central".into(),
            cost: 60.0,
            latency_ms: 80.0,
            co2_kg: 10.0,
            law_ok: true,
        },
        CandidateRegion {
            name: "asia-east".into(),
            cost: 40.0,
            latency_ms: 150.0,
            co2_kg: 25.0,
            law_ok: false,
        },
        CandidateRegion {
            name: "us-gov".into(),
            cost: 120.0,
            latency_ms: 70.0,
            co2_kg: 15.0,
            law_ok: true,
        },
    ]
}

pub fn multi_objective_score(region: &CandidateRegion, intent: &DeploymentIntent) -> f64 {
    // Lower is better!
    match intent.business_priority.as_str() {
        "cost" => region.cost + region.latency_ms * 0.1 + region.co2_kg,
        "latency" => region.latency_ms + region.cost * 0.5 + region.co2_kg * 2.0,
        "co2" => region.co2_kg * 5.0 + region.cost * 0.2 + region.latency_ms * 0.1,
        _ => region.cost * 0.4 + region.latency_ms * 0.3 + region.co2_kg * 2.0,
    }
}

pub fn optimize(intent: &DeploymentIntent) -> DeployDecision {
    let mut best_score = f64::MAX;
    let mut best_region = None;
    let candidates: Vec<_> = get_candidates().into_iter().filter(|r| r.law_ok).collect();
    for region in &candidates {
        let score = multi_objective_score(region, intent);
        if score < best_score {
            best_score = score;
            best_region = Some(region.name.clone());
        }
    }
    let reason = match intent.business_priority.as_str() {
        "cost" => "Optimized for lowest cost, then latency, then CO₂.",
        "latency" => "Optimized for lowest latency, then cost, then CO₂.",
        "co2" => "Optimized for lowest CO₂ emissions.",
        _ => "Balanced between cost, latency, and CO₂.",
    };
    DeployDecision {
        selected: best_region.unwrap_or("none".to_string()),
        score: best_score,
        reason: reason.into(),
        candidates,
    }
}
