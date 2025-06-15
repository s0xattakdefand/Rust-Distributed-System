use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountEvent {
    Deposited(i64),
    Withdrawn(i64),
}
