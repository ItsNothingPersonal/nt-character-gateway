use super::attribute::Attribute;
use serde::{Deserialize, Serialize};

/// Attributes Struct
#[derive(Serialize, Deserialize)]
pub struct Attributes {
    pub physical: Attribute,
    pub social: Attribute,
    pub mental: Attribute,
}
