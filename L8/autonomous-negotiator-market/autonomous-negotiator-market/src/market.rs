use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub id: String,
    pub name: String,
    pub location: String,
    pub available_cpu: u32,
    pub price_per_cpu: f64,
    pub green: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadBid {
    pub id: String,
    pub requester: String,
    pub cpu_needed: u32,
    pub max_price: f64,
    pub preferred_location: Option<String>,
    pub assigned_provider: Option<String>,
    pub final_price: Option<f64>,
}

pub struct Market {
    pub providers: HashMap<String, Provider>,
    pub bids: HashMap<String, WorkloadBid>,
}

impl Market {
    pub fn new() -> Self {
        let mut providers = HashMap::new();
        providers.insert("aws-1".to_string(), Provider {
            id: "aws-1".into(),
            name: "AWS West".into(),
            location: "us-west".into(),
            available_cpu: 32,
            price_per_cpu: 1.2,
            green: false,
        });
        providers.insert("gcp-1".to_string(), Provider {
            id: "gcp-1".into(),
            name: "GCP Central".into(),
            location: "eu-central".into(),
            available_cpu: 24,
            price_per_cpu: 1.0,
            green: true,
        });
        providers.insert("azure-1".to_string(), Provider {
            id: "azure-1".into(),
            name: "Azure Asia".into(),
            location: "asia-east".into(),
            available_cpu: 64,
            price_per_cpu: 0.9,
            green: false,
        });
        Market {
            providers,
            bids: HashMap::new(),
        }
    }

    // Submit a new workload and try to assign to a provider
    pub fn submit_bid(&mut self, mut bid: WorkloadBid) -> WorkloadBid {
        // Find best provider (first, only immutable)
        let mut best: Option<(String, f64)> = None;
        for (pid, p) in &self.providers {
            if p.available_cpu >= bid.cpu_needed && p.price_per_cpu * (bid.cpu_needed as f64) <= bid.max_price {
                if let Some(ref loc) = bid.preferred_location {
                    if &p.location != loc {
                        continue;
                    }
                }
                let total_price = p.price_per_cpu * (bid.cpu_needed as f64);
                if best.is_none() || total_price < best.as_ref().unwrap().1 {
                    best = Some((pid.clone(), total_price));
                }
            }
        }
        // Now, mutate OUTSIDE the loop!
        if let Some((pid, total_price)) = best {
            bid.assigned_provider = Some(pid.clone());
            bid.final_price = Some(total_price);
            if let Some(provider) = self.providers.get_mut(&pid) {
                provider.available_cpu -= bid.cpu_needed;
            }
        }
        self.bids.insert(bid.id.clone(), bid.clone());
        bid
    }

    pub fn get_providers(&self) -> Vec<Provider> {
        self.providers.values().cloned().collect()
    }

    pub fn get_bids(&self) -> Vec<WorkloadBid> {
        self.bids.values().cloned().collect()
    }
}
