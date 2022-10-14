use serde::{Deserialize, Serialize};

/// Morality Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Morality {
    pub name: String,
    pub value: u8,
}

/// Input for updating the morality section
#[derive(Serialize, Deserialize, Debug)]
pub struct MoralityUpdateInput {
    pub name: Option<String>,
    pub value: Option<u8>,
}
