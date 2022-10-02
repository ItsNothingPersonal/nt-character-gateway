use serde::Serialize;

/// Attribut Struct
#[derive(Serialize)]
pub struct Attribute {
    pub value: u8,
    pub foci: Vec<String>,
}
