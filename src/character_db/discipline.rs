use serde::Serialize;

/// Discipline Struct
#[derive(Serialize, Debug)]
pub struct Discipline {
    pub name: String,
    pub value: u8,
}
