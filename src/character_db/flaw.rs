use serde::{Deserialize, Serialize};

/// Flaw Struct
#[derive(Serialize, Deserialize)]
pub struct Flaw {
    pub name: String,
    pub value: i8,
    pub flaw_type: String,
}
