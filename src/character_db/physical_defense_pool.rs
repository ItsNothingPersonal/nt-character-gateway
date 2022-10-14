use serde::{Deserialize, Serialize};

/// PhysicalDefensePool Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct PhysicalDefensePool {
    pub base_value: u8,
    pub base_value_with_celerity: u8,
    pub frenzy_modifier: i8,
    pub on_the_ground_closer_than_three_meters_modifier: i8,
    pub on_the_ground_further_than_three_meters_modifier: u8,
    pub special: i8,
}
