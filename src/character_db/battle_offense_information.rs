use serde::{Deserialize, Serialize};

use super::name_value::NameValue;

/// BattleOffenseInformation Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct BattleOffenseInformation {
    pub skill: NameValue,
    pub attribute: NameValue,
    pub wildcard: NameValue,
    pub pool: u8,
    pub description: String,
}
