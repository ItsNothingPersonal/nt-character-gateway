use serde::{Deserialize, Serialize};

/// Ritual Struct
#[derive(Serialize, Deserialize)]
pub struct Ritual {
    pub name: String,
    pub level: u8,
    pub description: String,
    pub ritual_type: String,
}
