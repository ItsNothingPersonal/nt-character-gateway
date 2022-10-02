use super::attribute::Attribute;
use serde::Serialize;

/// Attributes Struct
#[derive(Serialize)]
pub struct Attributes {
    pub physical: Attribute,
    pub social: Attribute,
    pub mental: Attribute,
}
