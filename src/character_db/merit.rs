use serde::{Deserialize, Serialize};

/// Merit Struct
#[derive(Serialize, Deserialize)]
pub struct Merit {
    pub name: String,
    pub value: i8,
    pub merit_type: String,
}
