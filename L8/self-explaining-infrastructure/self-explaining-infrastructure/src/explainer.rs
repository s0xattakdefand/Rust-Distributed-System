use serde::{Serialize, Deserialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployRequest {
    pub user: String,
    pub region: String,
    pub cost_limit: f32,
    pub required: Vec<String>, // e.g., ["gpu", "ssd"]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployDecision {
    pub approved: bool,
    pub reason: String,
    pub suggestion: Option<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationLog {
    pub user: String,
    pub region: String,
    pub approved: bool,
    pub reason: String,
    pub suggestion: Option<String>,
}

pub struct SelfExplainingInfra {
    // Circular log, last N explanations
    pub history: VecDeque<ExplanationLog>,
    pub max_log: usize,
}

impl SelfExplainingInfra {
    pub fn new(max_log: usize) -> Self {
        Self {
            history: VecDeque::with_capacity(max_log),
            max_log,
        }
    }

    pub fn deploy(&mut self, req: DeployRequest) -> DeployDecision {
        // For demo: pretend we have these regions and quotas
        let allowed_regions = vec!["us-west", "eu-central", "asia-south"];
        let region_costs = vec![
            ("us-west", 1.1),
            ("eu-central", 1.5),
            ("asia-south", 0.9),
        ];

        // Simulate resource availability
        let region_gpu = vec!["us-west"];
        let region_ssd = vec!["us-west", "eu-central"];

        // Evaluate
        let mut approved = true;
        let mut reason = String::new();
        let mut suggestion = None;

        // 1. Region not allowed
        if !allowed_regions.contains(&req.region.as_str()) {
            approved = false;
            reason = format!("Region '{}' is not available.", req.region);
            suggestion = Some("Try 'us-west', 'eu-central', or 'asia-south'.".to_string());
        }

        // 2. Cost too high
        let cost = region_costs.iter().find(|(r, _)| *r == req.region)
            .map(|(_, c)| *c)
            .unwrap_or(f32::INFINITY);
        if approved && cost > req.cost_limit {
            approved = false;
            reason = format!("Requested region '{}' costs ${}/hr, over your limit of ${}/hr.", req.region, cost, req.cost_limit);
            suggestion = Some(format!("Increase your cost limit or try a cheaper region (e.g., 'asia-south')."));
        }

        // 3. Resource required but not available
        if approved && req.required.contains(&"gpu".to_string()) && !region_gpu.contains(&req.region.as_str()) {
            approved = false;
            reason = format!("Region '{}' does not support GPU.", req.region);
            suggestion = Some("Try 'us-west' for GPU workloads.".to_string());
        }
        if approved && req.required.contains(&"ssd".to_string()) && !region_ssd.contains(&req.region.as_str()) {
            approved = false;
            reason = format!("Region '{}' does not support SSD.", req.region);
            suggestion = Some("Try 'us-west' or 'eu-central' for SSD.".to_string());
        }

        if approved {
            reason = "Deployment approved.".to_string();
            suggestion = None;
        }

        let log = ExplanationLog {
            user: req.user.clone(),
            region: req.region.clone(),
            approved,
            reason: reason.clone(),
            suggestion: suggestion.clone(),
        };

        // Maintain circular log
        if self.history.len() == self.max_log {
            self.history.pop_front();
        }
        self.history.push_back(log);

        DeployDecision {
            approved,
            reason,
            suggestion,
            region: req.region,
        }
    }

    pub fn history(&self) -> Vec<ExplanationLog> {
        self.history.iter().cloned().collect()
    }
}
