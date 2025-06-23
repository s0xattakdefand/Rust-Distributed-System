use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub proposer: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub open: bool,
    pub votes_for: HashSet<String>,
    pub votes_against: HashSet<String>,
    pub decided: Option<Decision>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub decided_at: DateTime<Utc>,
    pub passed: bool,
    pub total_for: usize,
    pub total_against: usize,
}

#[derive(Default)]
pub struct DAO {
    pub proposals: HashMap<String, Proposal>,
}

impl DAO {
    pub fn new() -> Self {
        Self { proposals: HashMap::new() }
    }

    pub fn propose(&mut self, proposer: String, description: String) -> Proposal {
        let id = Uuid::new_v4().to_string();
        let prop = Proposal {
            id: id.clone(),
            proposer,
            description,
            created_at: Utc::now(),
            open: true,
            votes_for: HashSet::new(),
            votes_against: HashSet::new(),
            decided: None,
        };
        self.proposals.insert(id.clone(), prop.clone());
        prop
    }

    pub fn vote(&mut self, proposal_id: &str, voter: String, approve: bool) -> Option<Proposal> {
        if let Some(prop) = self.proposals.get_mut(proposal_id) {
            if !prop.open {
                return Some(prop.clone());
            }
            prop.votes_for.remove(&voter);
            prop.votes_against.remove(&voter);
            if approve {
                prop.votes_for.insert(voter);
            } else {
                prop.votes_against.insert(voter);
            }
            Some(prop.clone())
        } else {
            None
        }
    }

    pub fn finalize(&mut self, proposal_id: &str) -> Option<Proposal> {
        if let Some(prop) = self.proposals.get_mut(proposal_id) {
            if !prop.open {
                return Some(prop.clone());
            }
            let total_for = prop.votes_for.len();
            let total_against = prop.votes_against.len();
            let passed = total_for > total_against;
            prop.open = false;
            prop.decided = Some(Decision {
                decided_at: Utc::now(),
                passed,
                total_for,
                total_against,
            });
            Some(prop.clone())
        } else {
            None
        }
    }

    pub fn list(&self) -> Vec<Proposal> {
        self.proposals.values().cloned().collect()
    }

    pub fn get(&self, id: &str) -> Option<Proposal> {
        self.proposals.get(id).cloned()
    }
}
