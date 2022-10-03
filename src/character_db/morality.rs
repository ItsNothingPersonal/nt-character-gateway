use serde::{Deserialize, Serialize};

/// Morality Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Morality {
    pub name: String,
    pub value: u8,
}
