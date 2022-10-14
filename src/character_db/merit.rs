use serde::{Deserialize, Serialize};

/// Merit Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Merit {
    pub name: String,
    pub value: i8,
    pub merit_type: String,
}

/// Input for updating merits
#[derive(Serialize, Deserialize, Debug)]
pub struct MeritUpdateInput {
    pub name: String,
    pub merit_type: String,
}
