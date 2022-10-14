use serde::{Deserialize, Serialize};

/// Ritual Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Ritual {
    pub name: String,
    pub level: u8,
    pub description: String,
    pub ritual_type: String,
}

/// Input for updating a ritual
#[derive(Serialize, Deserialize, Debug)]
pub struct RitualUpdateInput {
    pub name: String,
    pub level: u8,
    pub ritual_type: String,
}
