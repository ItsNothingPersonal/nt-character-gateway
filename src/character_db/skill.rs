use serde::{Deserialize, Serialize};

/// Skill Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
    pub value: u8,
    pub foci: Option<Vec<String>>,
}

/// Struct for updating a skill
#[derive(Serialize, Deserialize, Debug)]
pub struct SkillUpdateInput {
    pub value: Option<u8>,
    pub foci: Option<Vec<String>>,
}
