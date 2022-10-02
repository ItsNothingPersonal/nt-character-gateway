use serde::Serialize;

/// Skill Struct
#[derive(Serialize)]
pub struct Skill {
    pub value: u8,
    pub foci: Option<Vec<String>>,
}
