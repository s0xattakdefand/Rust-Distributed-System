use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployIntent {
    pub user: String,
    pub region: String,
    pub model_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationDecision {
    pub approved: bool,
    pub reason: String,
    pub constraint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub name: String,
    pub description: String,
    pub affected_regions: Vec<String>,
    pub affected_models: Vec<String>,
}

pub struct EthicsEngine {
    pub constraints: Vec<Constraint>,
}

impl EthicsEngine {
    pub fn new() -> Self {
        Self {
            constraints: vec![
                Constraint {
                    name: "GDPR".into(),
                    description: "EU regions require GDPR-compliant models.".into(),
                    affected_regions: vec!["eu-central".into(), "eu-west".into()],
                    affected_models: vec!["basic".into(), "non-gdpr".into()],
                },
                Constraint {
                    name: "AI Act".into(),
                    description: "AI Act prohibits 'gen-ai' in certain regions.".into(),
                    affected_regions: vec!["eu-central".into()],
                    affected_models: vec!["gen-ai".into()],
                },
                Constraint {
                    name: "US Export Ban".into(),
                    description: "No 'gen-ai' model in embargoed region 'cn-north'.".into(),
                    affected_regions: vec!["cn-north".into()],
                    affected_models: vec!["gen-ai".into()],
                }
            ]
        }
    }

    pub fn check_intent(&self, intent: &DeployIntent) -> OrchestrationDecision {
        for constraint in &self.constraints {
            if constraint.affected_regions.contains(&intent.region)
                && constraint.affected_models.contains(&intent.model_type) {
                return OrchestrationDecision {
                    approved: false,
                    reason: format!(
                        "Deployment blocked by {}: {}",
                        constraint.name, constraint.description
                    ),
                    constraint: Some(constraint.name.clone()),
                };
            }
        }
        OrchestrationDecision {
            approved: true,
            reason: "Deployment approved (no ethics/legal block).".to_string(),
            constraint: None,
        }
    }
}
