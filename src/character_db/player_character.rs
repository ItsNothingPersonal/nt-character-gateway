use super::{
    attributes::Attributes, background::Background, flaw::Flaw, merit::Merit, morality::Morality,
    powers::Powers, skills::Skills,
};
use either::Either;
use serde::{Deserialize, Serialize};

/// the output to the character_data handler
#[derive(Serialize, Deserialize)]
pub struct PlayerCharacter {
    pub character_name: String,
    pub player_name: String,
    pub version_sheet: String,
    pub archetype: String,
    pub generation: Either<u8, String>,
    pub clan: String,
    pub blood_per_turn: u8,
    pub blood_pool: u8,
    pub attributes: Attributes,
    pub skills: Skills,
    pub powers: Powers,
    pub morality: Morality,
    pub faction: String,
    pub merits: Vec<Merit>,
    pub flaws: Vec<Flaw>,
    pub backgrounds: Vec<Background>,
}
