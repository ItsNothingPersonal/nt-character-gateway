use serde::{Deserialize, Serialize};

/// Attribut Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute {
    pub value: u8,
    pub foci: Vec<String>,
}

/// the struct for updating a single attribute
#[derive(Serialize, Deserialize, Debug)]
pub struct AttributeUpdateInput {
    pub value: Option<u8>,
    pub foci: Option<Vec<String>>,
}
