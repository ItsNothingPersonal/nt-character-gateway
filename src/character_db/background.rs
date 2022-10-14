use serde::{Deserialize, Serialize};

/// Background Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Background {
    pub name: String,
    pub value: u8,
    pub description: String,
}

/// Input for updating a background
#[derive(Serialize, Deserialize, Debug)]
pub struct BackgroundUpdateInput {
    pub name: String,
    pub value: u8,
    pub description: Option<String>,
}
