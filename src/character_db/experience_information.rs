use serde::{Deserialize, Serialize};

/// ExperienceInformation Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct ExperienceInformation {
    pub start_value: u8,
    pub spent_total: u16,
    pub available: i16,
    pub received_total: u8,
}

/// Input for updating the experience section
#[derive(Serialize, Deserialize, Debug)]
pub struct ExperienceInformationUpdateInput {
    pub start_value: u8,
}
