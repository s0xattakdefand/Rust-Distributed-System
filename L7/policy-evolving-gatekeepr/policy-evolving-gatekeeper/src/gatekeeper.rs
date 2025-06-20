use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UserPolicy {
    pub allowed: bool,
    pub rate_limit: u32,
    pub error_rate: f32,
    pub block_reason: Option<String>,
}

#[derive(Debug, Default)]
pub struct Gatekeeper {
    user_stats: HashMap<String, (u32, u32)>, // user_id -> (reqs, errors)
    user_policy: HashMap<String, UserPolicy>,
}

impl Gatekeeper {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn call_api(&mut self, user_id: &str, success: bool) -> UserPolicy {
        let entry = self.user_stats.entry(user_id.to_string()).or_insert((0, 0));
        entry.0 += 1;
        if !success {
            entry.1 += 1;
        }
        let (reqs, errs) = *entry;
        let error_rate = if reqs > 0 { errs as f32 / reqs as f32 } else { 0.0 };

        // Policy evolution logic
        let (allowed, rate_limit, block_reason) = if error_rate > 0.5 && reqs >= 10 {
            (false, 0, Some("Blocked for abuse".to_string()))
        } else if error_rate > 0.2 && reqs >= 5 {
            (true, 1, Some("Rate limited for suspicious behavior".to_string()))
        } else {
            (true, 5, None)
        };

        let policy = UserPolicy {
            allowed,
            rate_limit,
            error_rate,
            block_reason,
        };
        self.user_policy.insert(user_id.to_string(), policy.clone());
        policy
    }

    pub fn get_policy(&self, user_id: &str) -> UserPolicy {
        self.user_policy.get(user_id).cloned().unwrap_or(UserPolicy {
            allowed: true,
            rate_limit: 5,
            error_rate: 0.0,
            block_reason: None,
        })
    }

    pub fn reset(&mut self) {
        self.user_stats.clear();
        self.user_policy.clear();
    }
}
