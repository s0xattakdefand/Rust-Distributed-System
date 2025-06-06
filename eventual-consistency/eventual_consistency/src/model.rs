use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
}
