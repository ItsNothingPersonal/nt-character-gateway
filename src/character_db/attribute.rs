use serde::{Deserialize, Serialize};

/// Attribut Struct
#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub value: u8,
    pub foci: Vec<String>,
}
