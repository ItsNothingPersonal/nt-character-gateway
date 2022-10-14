use serde::{Deserialize, Serialize};

/// Flaw Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Flaw {
    pub name: String,
    pub value: i8,
    pub flaw_type: String,
}

/// Input for updating a flaw
#[derive(Serialize, Deserialize, Debug)]
pub struct FlawUpdateInput {
    pub name: String,
    pub flaw_type: String,
}
