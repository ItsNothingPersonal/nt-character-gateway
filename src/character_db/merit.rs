use serde::Serialize;

/// Merit Struct
#[derive(Serialize)]
pub struct Merit {
    pub name: String,
    pub value: i8,
    pub merit_type: String,
}
