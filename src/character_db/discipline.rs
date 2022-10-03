use serde::{Deserialize, Serialize};

/// Discipline Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Discipline {
    pub name: String,
    pub value: u8,
}
