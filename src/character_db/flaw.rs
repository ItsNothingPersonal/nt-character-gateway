use serde::Serialize;

/// Flaw Struct
#[derive(Serialize)]
pub struct Flaw {
    pub name: String,
    pub value: i8,
    pub flaw_type: String,
}
