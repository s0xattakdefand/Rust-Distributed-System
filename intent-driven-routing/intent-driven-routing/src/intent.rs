use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)] // <-- Add Serialize here
pub struct Intent {
    pub mission: String,
    pub urgency: Option<u8>, // 1 (low) to 10 (high)
}
