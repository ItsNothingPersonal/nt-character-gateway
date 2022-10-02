use serde::Serialize;

/// Background Struct
#[derive(Serialize, Debug)]
pub struct Background {
    pub name: String,
    pub value: u8,
    pub description: String,
}
