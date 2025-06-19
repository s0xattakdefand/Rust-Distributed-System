use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseInput {
    pub version: String,
    pub features: Vec<String>,
    pub metric_score: f32, // Simulate test coverage, performance, etc (0-1)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseRecord {
    pub version: String,
    pub features: Vec<String>,
    pub metric_score: f32,
    pub outcome: String, // "success" or "fail"
    pub risk_score: f32,
    pub approved: bool,
}

#[derive(Debug)]
pub struct ReleaseManager {
    pub releases: Vec<ReleaseRecord>,
}

impl ReleaseManager {
    pub fn new() -> Self {
        Self { releases: Vec::new() }
    }

    pub fn prior_failures(&self) -> Vec<&ReleaseRecord> {
        self.releases.iter().filter(|r| r.outcome == "fail").collect()
    }

    pub fn predict_risk(&self, features: &[String], metric_score: f32) -> f32 {
        // Simulate ML: If prior failures had similar features/metrics, risk is high
        let failures = self.prior_failures();
        let mut risk: f32 = 0.2; // base risk
        for f in failures {
            let overlap = f.features.iter().filter(|feat| features.contains(feat)).count();
            if overlap > 0 {
                risk += 0.3;
            }
            if (f.metric_score - metric_score).abs() < 0.1 {
                risk += 0.3;
            }
        }
        risk.clamp(0.0, 1.0)
    }

    pub fn propose_release(&mut self, input: ReleaseInput) -> ReleaseRecord {
        let risk = self.predict_risk(&input.features, input.metric_score);
        let approved = risk < 0.6 && input.metric_score >= 0.7;
        let outcome = if approved { "success" } else { "fail" };
        let record = ReleaseRecord {
            version: input.version,
            features: input.features,
            metric_score: input.metric_score,
            outcome: outcome.to_string(),
            risk_score: risk,
            approved,
        };
        self.releases.push(record.clone());
        record
    }
}
