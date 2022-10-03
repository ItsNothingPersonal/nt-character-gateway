use serde::{Deserialize, Serialize};

/// Skill Struct
#[derive(Serialize, Deserialize)]
pub struct Skill {
    pub value: u8,
    pub foci: Option<Vec<String>>,
}
