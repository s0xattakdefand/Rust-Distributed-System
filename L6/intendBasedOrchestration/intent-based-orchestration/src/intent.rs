use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Intent {
    pub goal: String,
    pub parameters: Option<serde_json::Value>,
}
