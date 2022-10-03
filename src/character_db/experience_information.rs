use serde::{Deserialize, Serialize};

/// ExperienceInformation Struct
#[derive(Serialize, Deserialize)]
pub struct ExperienceInformation {
    pub start_value: u8,
    pub spent_total: u16,
    pub available: i16,
    pub received_total: u8,
}
