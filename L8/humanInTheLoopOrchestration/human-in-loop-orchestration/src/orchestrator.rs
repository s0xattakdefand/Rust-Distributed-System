use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::{VecDeque, HashMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployRequest {
    pub user: String,
    pub model: String,
    pub environment: String, // e.g., "dev", "staging", "prod"
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeployStatus {
    Approved,
    PendingHuman,
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployDecision {
    pub id: String,
    pub request: DeployRequest,
    pub status: DeployStatus,
    pub explanation: String,
}

pub struct HumanLoopOrchestrator {
    pub pending: VecDeque<DeployDecision>,
    pub history: VecDeque<DeployDecision>,
    pub lookup: HashMap<String, DeployDecision>,
    pub max_log: usize,
}

impl HumanLoopOrchestrator {
    pub fn new(max_log: usize) -> Self {
        Self {
            pending: VecDeque::new(),
            history: VecDeque::with_capacity(max_log),
            lookup: HashMap::new(),
            max_log,
        }
    }

    pub fn deploy(&mut self, req: DeployRequest) -> DeployDecision {
        let id = Uuid::new_v4().to_string();
        // Edge-case logic: prod or new model triggers human review
        let needs_human = req.environment == "prod" || req.model.contains("beta");
        let (status, explanation) = if needs_human {
            (
                DeployStatus::PendingHuman,
                "Human approval required for production or beta rollout.".to_string(),
            )
        } else {
            (
                DeployStatus::Approved,
                "Auto-approved: safe environment and model.".to_string(),
            )
        };
        let decision = DeployDecision {
            id: id.clone(),
            request: req,
            status: status.clone(),
            explanation: explanation.clone(),
        };

        self.lookup.insert(id.clone(), decision.clone());
        if status == DeployStatus::PendingHuman {
            self.pending.push_back(decision.clone());
        }
        if self.history.len() == self.max_log {
            self.history.pop_front();
        }
        self.history.push_back(decision.clone());
        decision
    }

    pub fn get_pending(&self) -> Vec<DeployDecision> {
        self.pending.iter().cloned().collect()
    }

    pub fn approve(&mut self, id: &str) -> Option<DeployDecision> {
        if let Some(idx) = self.pending.iter().position(|d| d.id == id) {
            let mut decision = self.pending.remove(idx).unwrap();
            decision.status = DeployStatus::Approved;
            decision.explanation = "Approved by human.".to_string();
            self.lookup.insert(id.to_string(), decision.clone());
            if self.history.len() == self.max_log {
                self.history.pop_front();
            }
            self.history.push_back(decision.clone());
            Some(decision)
        } else {
            None
        }
    }

    pub fn history(&self) -> Vec<DeployDecision> {
        self.history.iter().cloned().collect()
    }
}
