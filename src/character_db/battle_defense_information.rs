use super::physical_defense_pool::PhysicalDefensePool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// BattleDefenseInformation Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct BattleDefenseInformation {
    pub physical_defense_pool: PhysicalDefensePool,
    pub mental_defense_pool: HashMap<u8, u8>,
    pub social_defense_pool: HashMap<u8, u8>,
}
