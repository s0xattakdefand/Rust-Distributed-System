use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLog {
    pub ip: String,
    pub endpoint: String,
    pub timestamp: DateTime<Utc>,
    pub risk_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyAudit {
    pub time: DateTime<Utc>,
    pub action: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyState {
    pub blocked_ips: HashSet<String>,
    pub rate_limit_per_ip: usize,
    pub audit_trail: Vec<PolicyAudit>,
}

impl PolicyState {
    pub fn new() -> Self {
        Self {
            blocked_ips: HashSet::new(),
            rate_limit_per_ip: 10,
            audit_trail: Vec::new(),
        }
    }
    pub fn evolve(&mut self, recent_logs: &[RequestLog]) {
        // Very simple ML-ish rule: if >3 risky requests from an IP in last 1min, block it
        let mut risk_counts: HashMap<String, usize> = HashMap::new();
        let now = Utc::now();
        for log in recent_logs {
            if (now - log.timestamp).num_seconds() < 60 && log.risk_score > 0.7 {
                *risk_counts.entry(log.ip.clone()).or_default() += 1;
            }
        }
        for (ip, count) in risk_counts {
            if count > 3 && !self.blocked_ips.contains(&ip) {
                self.blocked_ips.insert(ip.clone());
                self.audit_trail.push(PolicyAudit {
                    time: Utc::now(),
                    action: "block_ip".to_string(),
                    detail: format!("AI-augmented: Blocked {ip} after {count} risky requests in 60s"),
                });
            }
        }
        // If too many requests from ALL IPs, lower global rate limit (simulates learning)
        if recent_logs.len() > 100 {
            self.rate_limit_per_ip = std::cmp::max(2, self.rate_limit_per_ip - 1);
            self.audit_trail.push(PolicyAudit {
                time: Utc::now(),
                action: "lower_rate_limit".to_string(),
                detail: "AI-augmented: Lowered rate limit due to global surge".to_string(),
            });
        }
    }
}
