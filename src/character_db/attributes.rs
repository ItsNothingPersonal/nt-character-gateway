use super::attribute::{Attribute, AttributeUpdateInput};
use serde::{Deserialize, Serialize};

/// Attributes Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    pub physical: Attribute,
    pub social: Attribute,
    pub mental: Attribute,
}

/// the struct for updating attributes
#[derive(Serialize, Deserialize, Debug)]
pub struct AttributesUpdateInput {
    pub physical: Option<AttributeUpdateInput>,
    pub social: Option<AttributeUpdateInput>,
    pub mental: Option<AttributeUpdateInput>,
}
