use serde::{Deserialize, Serialize};

/// NameValue Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct NameValue {
    pub name: String,
    pub value: u8,
}
