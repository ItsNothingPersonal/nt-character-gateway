use super::discipline::{Discipline, DisciplineUpdateInput};
use serde::{Deserialize, Serialize};

/// Powers Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Powers {
    pub in_clan_disciplines: Vec<Discipline>,
    pub out_of_clan_disciplines: Vec<Discipline>,
    pub techniques: Vec<String>,
    pub in_clan_elder_powers: Vec<String>,
    pub out_of_clan_elder_powers: Vec<String>,
}

// Struct for Updating the Powers field
#[derive(Serialize, Deserialize, Debug)]
pub struct PowersUpdateInput {
    pub in_clan_disciplines: Option<Vec<DisciplineUpdateInput>>,
    pub out_of_clan_disciplines: Option<Vec<DisciplineUpdateInput>>,
    pub techniques: Option<Vec<String>>,
    pub in_clan_elder_powers: Option<Vec<String>>,
    pub out_of_clan_elder_powers: Option<Vec<String>>,
}
