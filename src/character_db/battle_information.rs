use super::{
    battle_base_information::BattleBaseInformation,
    battle_defense_information::BattleDefenseInformation,
    battle_offense_information::BattleOffenseInformation,
};
use serde::{Deserialize, Serialize};

/// BattleInformation Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct BattleInformation {
    pub base: BattleBaseInformation,
    pub defense: BattleDefenseInformation,
    pub offense: Vec<BattleOffenseInformation>,
}
