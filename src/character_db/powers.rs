use super::discipline::Discipline;
use serde::{Deserialize, Serialize};

/// Powers Struct
#[derive(Serialize, Deserialize)]
pub struct Powers {
    pub in_clan_disciplines: Vec<Discipline>,
    pub out_of_clan_disciplines: Vec<Discipline>,
    pub techniques: Vec<String>,
    pub in_clan_elder_powers: Vec<String>,
    pub out_of_clan_elder_powers: Vec<String>,
}
