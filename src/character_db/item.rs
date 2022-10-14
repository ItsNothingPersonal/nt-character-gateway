use serde::{Deserialize, Serialize};

/// Item Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub trait_1: String,
    pub trait_1_description: String,
    pub trait_2: String,
    pub trait_2_description: String,
    pub additional_trait: String,
    pub additional_trait_description: String,
}
