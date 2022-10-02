use serde::Serialize;

/// Morality Struct
#[derive(Serialize, Debug)]
pub struct Morality {
    pub name: String,
    pub value: u8,
}
