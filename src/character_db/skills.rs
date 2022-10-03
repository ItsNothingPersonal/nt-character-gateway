use super::skill::Skill;
use serde::{Deserialize, Serialize};

/// Skills Struct
#[derive(Serialize, Deserialize)]
pub struct Skills {
    pub academics: Skill,
    pub athletics: Skill,
    pub animal_ken: Skill,
    pub awareness: Skill,
    pub brawl: Skill,
    pub computer: Skill,
    pub craft_a: Skill,
    pub craft_b: Skill,
    pub dodge: Skill,
    pub drive: Skill,
    pub empathy: Skill,
    pub firearms: Skill,
    pub intimidation: Skill,
    pub investigation: Skill,
    pub leadership: Skill,
    pub linguistics: Skill,
    pub lore: Skill,
    pub medicine: Skill,
    pub melee: Skill,
    pub occult: Skill,
    pub performance_a: Skill,
    pub performance_b: Skill,
    pub security: Skill,
    pub science_a: Skill,
    pub science_b: Skill,
    pub stealth: Skill,
    pub streetwise: Skill,
    pub subterfuge: Skill,
    pub survival: Skill,
}
